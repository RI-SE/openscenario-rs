pub mod builder_fixtures;
pub mod xml_profile;

use openscenario_rs::validation::XsdValidator;
use openscenario_rs::{
    parse_catalog_from_file, parse_catalog_from_str, parse_from_file, parse_from_str,
    serialize_catalog_to_string, serialize_to_string,
};
use std::fmt;
use std::path::{Path, PathBuf};

/// Which step of the parse -> serialize -> reparse -> serialize round trip failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    /// A builder fixture's `build()` returned `Err` (builder checks only).
    Build,
    /// The initial `parse_from_file`/`parse_catalog_from_file` call failed.
    Parse,
    /// Serializing the freshly-parsed document back to XML failed.
    Serialize,
    /// Reparsing the serialized XML failed.
    Reparse,
    /// Serializing the reparsed document failed.
    Reserialize,
    /// Both serializations succeeded but produced different XML (not a fixed point).
    FixedPointMismatch,
    /// The document survived the round trip as XML, but the reparsed struct differs from the
    /// one that was built (builder checks only).
    FidelityMismatch,
    /// The produced XML violates `Schema/OpenSCENARIO.xsd` (builder checks only).
    SchemaInvalid,
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Stage::Build => "build",
            Stage::Parse => "parse",
            Stage::Serialize => "serialize",
            Stage::Reparse => "reparse",
            Stage::Reserialize => "reserialize",
            Stage::FixedPointMismatch => "fixed-point mismatch",
            Stage::FidelityMismatch => "fidelity mismatch",
            Stage::SchemaInvalid => "schema-invalid",
        };
        f.write_str(s)
    }
}

/// A round-trip failure: which stage it happened at, and the error/diff message.
#[derive(Debug)]
pub struct RoundtripError {
    pub stage: Stage,
    pub message: String,
}

impl fmt::Display for RoundtripError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.stage, self.message)
    }
}

fn err(stage: Stage, message: impl fmt::Display) -> RoundtripError {
    RoundtripError {
        stage,
        message: message.to_string(),
    }
}

/// Parses `path` as a full OpenSCENARIO document, serializes it back to XML, reparses that
/// XML, and serializes again, checking the two serialized strings are identical (a round-trip
/// fixed point). Returns which stage failed and why, rather than panicking.
pub fn check_roundtrip(path: &str) -> Result<(), RoundtripError> {
    let original = parse_from_file(path).map_err(|e| err(Stage::Parse, e))?;
    let xml1 = serialize_to_string(&original).map_err(|e| err(Stage::Serialize, e))?;
    let reparsed = parse_from_str(&xml1).map_err(|e| err(Stage::Reparse, e))?;
    let xml2 = serialize_to_string(&reparsed).map_err(|e| err(Stage::Reserialize, e))?;
    if xml1 != xml2 {
        return Err(err(
            Stage::FixedPointMismatch,
            format!("first pass:\n{xml1}\nsecond pass:\n{xml2}"),
        ));
    }
    Ok(())
}

/// Same as [`check_roundtrip`], but for standalone catalog documents (files whose root
/// `<Catalog>` element has no enclosing `Storyboard`/`ParameterValueDistribution`), which the
/// crate parses and serializes through a dedicated `CatalogFile` API instead of `OpenScenario`.
pub fn check_catalog_roundtrip(path: &str) -> Result<(), RoundtripError> {
    let original = parse_catalog_from_file(path).map_err(|e| err(Stage::Parse, e))?;
    let xml1 = serialize_catalog_to_string(&original).map_err(|e| err(Stage::Serialize, e))?;
    let reparsed = parse_catalog_from_str(&xml1).map_err(|e| err(Stage::Reparse, e))?;
    let xml2 = serialize_catalog_to_string(&reparsed).map_err(|e| err(Stage::Reserialize, e))?;
    if xml1 != xml2 {
        return Err(err(
            Stage::FixedPointMismatch,
            format!("first pass:\n{xml1}\nsecond pass:\n{xml2}"),
        ));
    }
    Ok(())
}

/// Runs a builder fixture through the same three questions the corpus binaries ask of the
/// parser, and returns *every* failure rather than stopping at the first.
///
/// The parser harness gets to ask its questions of 172 real files; the builder's input is code,
/// so the fixtures in [`builder_fixtures`] are its corpus. The checks, in order:
///
/// 1. **round trip** — `build` -> serialize -> parse -> serialize is a fixed point, mirroring
///    [`check_roundtrip`].
/// 2. **fidelity** — the reparsed document equals the one that was built. This is *stronger*
///    than check 1, which cannot see a field that serialization drops on both passes.
/// 3. **schema** — the produced XML validates against `Schema/OpenSCENARIO.xsd`, mirroring the
///    `validate` binary.
///
/// `validator` is passed in because parsing the 106 KB schema per fixture would dominate the
/// runtime; construct one with [`load_schema`] and reuse it.
pub fn check_builder_fixture(
    fixture: &builder_fixtures::BuilderFixture,
    validator: &mut XsdValidator,
) -> Vec<RoundtripError> {
    let built = match (fixture.build)() {
        Ok(s) => s,
        Err(e) => return vec![err(Stage::Build, e)],
    };

    let xml1 = match serialize_to_string(&built) {
        Ok(x) => x,
        Err(e) => return vec![err(Stage::Serialize, e)],
    };

    let reparsed = match parse_from_str(&xml1) {
        Ok(r) => r,
        Err(e) => return vec![err(Stage::Reparse, e)],
    };

    // Past this point every check is independent, so collect them all: one fixture that is both
    // lossy and schema-invalid should report both, not just whichever runs first.
    let mut failures = Vec::new();

    match serialize_to_string(&reparsed) {
        Ok(xml2) if xml2 != xml1 => failures.push(err(
            Stage::FixedPointMismatch,
            format!("first pass:\n{xml1}\nsecond pass:\n{xml2}"),
        )),
        Ok(_) => {}
        Err(e) => failures.push(err(Stage::Reserialize, e)),
    }

    if reparsed != built {
        failures.push(err(
            Stage::FidelityMismatch,
            "the reparsed document differs from the one the builder produced; \
             a field was set on the builder but did not survive serialization"
                .to_string(),
        ));
    }

    match validator.validate_str(&xml1) {
        Ok(errors) if !errors.is_empty() => {
            let detail: Vec<String> = errors.iter().map(|e| e.to_string()).collect();
            failures.push(err(
                Stage::SchemaInvalid,
                format!("{} schema violations:\n{}", errors.len(), detail.join("\n")),
            ));
        }
        Ok(_) => {}
        Err(e) => failures.push(err(Stage::SchemaInvalid, e)),
    }

    failures
}

/// Loads the crate's `Schema/OpenSCENARIO.xsd` relative to this crate's manifest directory.
pub fn load_schema() -> openscenario_rs::Result<XsdValidator> {
    XsdValidator::from_schema_file(manifest_dir().join("../Schema/OpenSCENARIO.xsd"))
}

/// This crate's manifest directory, falling back to the process CWD outside of Cargo.
fn manifest_dir() -> PathBuf {
    PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string()))
}

/// What to tell the user when `corpus/` is absent. The corpus is fetched on demand rather than
/// vendored, so an empty directory is the normal state of a fresh checkout, not a bug.
pub const MISSING_CORPUS_HINT: &str =
    "conformance corpus not present; run scripts/fetch-corpus.sh to enable the corpus gates";

/// The corpus directory: `conformance/corpus/`, populated on demand and not tracked in git.
pub fn corpus_dir() -> PathBuf {
    manifest_dir().join("corpus")
}

/// Every `.xosc` file under [`corpus_dir`], in a stable order. Empty if the corpus is absent —
/// `WalkDir` on a nonexistent directory yields no entries rather than erroring.
pub fn corpus_files() -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = walkdir::WalkDir::new(corpus_dir())
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.into_path())
        .filter(|p| p.extension().map(|ext| ext == "xosc").unwrap_or(false))
        .collect();
    paths.sort();
    paths
}

/// [`corpus_files`], but exits non-zero with [`MISSING_CORPUS_HINT`] when the corpus is empty.
///
/// The corpus binaries are gates. A gate that reports success because it examined nothing is
/// exactly the false green this harness exists to prevent, so an absent corpus is a hard failure
/// for anything that needs one — never a clean run over zero files.
pub fn require_corpus() -> Vec<PathBuf> {
    let files = corpus_files();
    if files.is_empty() {
        eprintln!("{MISSING_CORPUS_HINT}");
        std::process::exit(1);
    }
    files
}

/// Whether `path` sits under a `catalogs/` directory, and so must round-trip through the
/// standalone-catalog API rather than the `OpenScenario` one.
pub fn is_catalog(rel: &Path) -> bool {
    rel.components()
        .any(|c| c.as_os_str().to_string_lossy().to_lowercase() == "catalogs")
}

/// Test-friendly wrapper around [`check_roundtrip`] that panics with the file name, failing
/// stage, and underlying error/diff on failure.
pub fn assert_roundtrip_fixed_point(path: &str) {
    if let Err(e) = check_roundtrip(path) {
        panic!("{path}\n{e}");
    }
}

/// Test-friendly wrapper around [`check_catalog_roundtrip`] that panics with the file name,
/// failing stage, and underlying error/diff on failure.
pub fn assert_catalog_roundtrip_fixed_point(path: &str) {
    if let Err(e) = check_catalog_roundtrip(path) {
        panic!("{path}\n{e}");
    }
}
