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

// --- expectations manifest ------------------------------------------------------------------
//
// `conformance/expectations.toml` records the corpus files whose expected outcome is *not*
// "parses, round-trips losslessly, and serializes to schema-valid XML". It exists because the
// alternatives are worse: deleting the files hides upstream's deliberate fixtures, and a silent
// skip is exactly the false green this harness was built to prevent (see the exit-code comment
// in `src/bin/validate.rs`). An exemption here is therefore never a bare opt-out — it carries two
// assertions the gates actually run, so an exemption that has stopped being true fails loudly:
//
//   1. [`ExpectAssert`] — a premise about the *input* file, checked against the XSD.
//   2. "still needed" — the gate the file is exempt from must still genuinely fail for it,
//      checked by each binary at the point it would otherwise have run the gate.

/// One of the three corpus gates a file can be exempted from.
///
/// `Report` covers both `src/bin/report.rs` and the round-trip test `build.rs` generates for the
/// file — they ask the same question of the same file, so exempting one without the other would
/// leave the gate red in `cargo test` and green in the binary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Gate {
    Report,
    Lossy,
    Validate,
}

impl fmt::Display for Gate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Gate::Report => "report",
            Gate::Lossy => "lossy",
            Gate::Validate => "validate",
        })
    }
}

/// The premise an entry asserts about the *input* file. This is the "positive assertion" that
/// distinguishes an expectation from a skip: it says why the file is exempt, and the harness
/// proves it rather than taking it on trust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
pub enum ExpectAssert {
    /// The input must fail XSD validation — the file itself is bad. If upstream fixes it when the
    /// pinned SHA moves, this assertion fails and the entry has to go.
    #[serde(rename = "input-schema-invalid")]
    InputSchemaInvalid,
    /// The input must be XSD *valid* — the file is fine and the crate is at fault. `reason` names
    /// the issue tracking the fix. Deliberately the opposite assertion, so a crate defect can
    /// never be filed away as an input problem.
    #[serde(rename = "crate-defect")]
    CrateDefect,
}

impl fmt::Display for ExpectAssert {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            ExpectAssert::InputSchemaInvalid => "input-schema-invalid",
            ExpectAssert::CrateDefect => "crate-defect",
        })
    }
}

/// One `[[entry]]` of `conformance/expectations.toml`.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Expectation {
    /// Corpus-relative path, so the manifest is machine-independent.
    pub path: String,
    /// The gates this file is exempt from. Empty is rejected at load: an entry that exempts
    /// nothing is noise.
    pub gates: Vec<Gate>,
    /// The premise about the input file, proven by [`Expectations::check_premises`].
    #[serde(rename = "assert")]
    pub assertion: ExpectAssert,
    /// Prose for a human: why, quoted from upstream or naming the tracking issue.
    pub reason: String,
}

#[derive(Debug, Clone, Default, serde::Deserialize)]
struct ExpectationsFile {
    #[serde(default, rename = "entry")]
    entries: Vec<Expectation>,
}

/// The parsed `conformance/expectations.toml`.
#[derive(Debug, Clone, Default)]
pub struct Expectations {
    entries: Vec<Expectation>,
}

/// The manifest's path: `conformance/expectations.toml`.
pub fn expectations_path() -> PathBuf {
    manifest_dir().join("expectations.toml")
}

impl Expectations {
    /// Parses the manifest. Errors on unreadable/invalid TOML, a duplicate `path`, or an entry
    /// that exempts no gate. Does *not* check the paths exist — see [`Expectations::load_for`].
    pub fn load() -> Result<Self, String> {
        let path = expectations_path();
        let text = std::fs::read_to_string(&path)
            .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
        let parsed: ExpectationsFile =
            toml::from_str(&text).map_err(|e| format!("cannot parse {}: {e}", path.display()))?;

        let mut seen: Vec<&str> = Vec::new();
        for entry in &parsed.entries {
            if entry.gates.is_empty() {
                return Err(format!(
                    "{}: entry `{}` exempts no gate; delete it instead",
                    path.display(),
                    entry.path
                ));
            }
            if seen.contains(&entry.path.as_str()) {
                return Err(format!(
                    "{}: duplicate entry for `{}`",
                    path.display(),
                    entry.path
                ));
            }
            seen.push(&entry.path);
        }

        Ok(Self {
            entries: parsed.entries,
        })
    }

    /// [`Expectations::load`], then requires every `path` to be present in `corpus`.
    ///
    /// A stale entry after a SHA bump is a bug, not dead config: it would silently stop protecting
    /// anything while still looking like it did.
    pub fn load_for(corpus: &[PathBuf]) -> Result<Self, String> {
        let expectations = Self::load()?;
        let dir = corpus_dir();
        let present: Vec<String> = corpus
            .iter()
            .map(|p| {
                p.strip_prefix(&dir)
                    .unwrap_or(p)
                    .to_string_lossy()
                    .into_owned()
            })
            .collect();

        let missing: Vec<&str> = expectations
            .entries
            .iter()
            .map(|e| e.path.as_str())
            .filter(|p| !present.iter().any(|c| c == p))
            .collect();

        if !missing.is_empty() {
            return Err(format!(
                "{}: {} stale entr{} — the path is not present in the corpus. \
                 A stale entry is a bug, not dead config:\n{}",
                expectations_path().display(),
                missing.len(),
                if missing.len() == 1 { "y" } else { "ies" },
                missing
                    .iter()
                    .map(|p| format!("  {p}"))
                    .collect::<Vec<_>>()
                    .join("\n"),
            ));
        }

        Ok(expectations)
    }

    /// Every entry, in manifest order.
    pub fn entries(&self) -> &[Expectation] {
        &self.entries
    }

    /// The entry exempting `rel` from `gate`, if any. `rel` is corpus-relative.
    pub fn exemption(&self, rel: &Path, gate: Gate) -> Option<&Expectation> {
        let rel = rel.to_string_lossy();
        self.entries
            .iter()
            .find(|e| e.path == rel && e.gates.contains(&gate))
    }

    /// Whether `rel` is exempt from `gate`.
    pub fn is_exempt(&self, rel: &Path, gate: Gate) -> bool {
        self.exemption(rel, gate).is_some()
    }

    /// Proves every entry's [`ExpectAssert`] against the input file, and returns one message per
    /// entry whose premise no longer holds.
    ///
    /// This is the half of the design that makes an exemption more than a skip. Run it in every
    /// binary that honours the manifest, whether or not that binary excludes anything: the whole
    /// point is that a premise which has stopped being true is noticed by somebody.
    pub fn check_premises(&self, validator: &mut XsdValidator) -> Vec<String> {
        let dir = corpus_dir();
        let mut failures = Vec::new();
        for entry in &self.entries {
            let abs = dir.join(&entry.path);
            let errors = match validator.validate_file(&abs) {
                Ok(errors) => errors,
                Err(e) => {
                    failures.push(format!(
                        "{}: cannot validate the input file to prove `assert = \"{}\"`: {e}",
                        entry.path, entry.assertion
                    ));
                    continue;
                }
            };
            match entry.assertion {
                ExpectAssert::InputSchemaInvalid if errors.is_empty() => failures.push(format!(
                    "{}: `assert = \"input-schema-invalid\"` but the input file is schema-VALID. \
                     Upstream probably fixed it — drop this entry and let the gates run.",
                    entry.path
                )),
                ExpectAssert::CrateDefect if !errors.is_empty() => failures.push(format!(
                    "{}: `assert = \"crate-defect\"` claims the input is fine and the crate is \
                     not, but the input file is schema-INVALID ({} errors, first: {}). \
                     Re-classify it as `input-schema-invalid`.",
                    entry.path,
                    errors.len(),
                    first_error(&errors),
                )),
                _ => {}
            }
        }
        failures
    }
}

fn first_error(errors: &[openscenario_rs::validation::ValidationError]) -> String {
    errors
        .first()
        .map(|e| xml_profile::first_line(&e.to_string()).to_string())
        .unwrap_or_default()
}

/// The message a gate prints when a file it excluded turns out to pass after all.
///
/// The second assertion behind every exemption: an exemption that is no longer needed must fail,
/// not quietly protect nothing. It is how the temporary `crate-defect` entries expire.
pub fn stale_exemption_message(rel: &Path, gate: Gate) -> String {
    format!(
        "{}: excluded from `{gate}` by conformance/expectations.toml, but it PASSES `{gate}` now. \
         The exemption is stale — remove it from the manifest.",
        rel.display()
    )
}

#[cfg(test)]
mod expectation_tests {
    use super::*;

    /// The manifest is well-formed and every entry still names a file in the corpus.
    ///
    /// The three gate binaries check this too, but they are separate processes: without this,
    /// `cargo test` would stay green while a stale entry quietly stopped protecting anything.
    /// Skipped when the corpus is absent — that is the normal state of a fresh checkout, and
    /// `require_corpus()` is what makes an empty corpus fail for the gates themselves.
    #[test]
    fn manifest_is_well_formed_and_current() {
        let corpus = corpus_files();
        if corpus.is_empty() {
            eprintln!("{MISSING_CORPUS_HINT} (skipping manifest check)");
            return;
        }
        if let Err(e) = Expectations::load_for(&corpus) {
            panic!("{e}");
        }
    }

    /// Every entry's `assert` premise about its *input* file still holds.
    #[test]
    fn manifest_premises_hold() {
        let corpus = corpus_files();
        if corpus.is_empty() {
            eprintln!("{MISSING_CORPUS_HINT} (skipping manifest check)");
            return;
        }
        let expectations = match Expectations::load_for(&corpus) {
            Ok(e) => e,
            Err(e) => panic!("{e}"),
        };
        let mut validator = load_schema().expect("Schema/OpenSCENARIO.xsd");
        let failures = expectations.check_premises(&mut validator);
        assert!(failures.is_empty(), "{}", failures.join("\n"));
    }
}
