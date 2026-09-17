use std::env;
use std::fs;
use std::path::Path;

/// A `[[entry]]` of `conformance/expectations.toml`, reduced to what test generation needs.
///
/// A build script cannot depend on its own crate, so this is a deliberately minimal second
/// reader of the same file rather than a call into `Expectations` in `src/lib.rs`. Only `path`
/// and `gates` are read here; `serde(deny_unknown_fields)` is *not* set, so the assertion fields
/// the binaries check are ignored rather than fought over. `src/lib.rs` is the strict reader —
/// it rejects a malformed manifest, and `cargo test` runs it before any generated test matters.
#[derive(serde::Deserialize)]
struct Entry {
    path: String,
    #[serde(default)]
    gates: Vec<String>,
}

#[derive(serde::Deserialize, Default)]
struct Manifest {
    #[serde(default, rename = "entry")]
    entry: Vec<Entry>,
}

/// The prelude every generated file carries, empty corpus or not: what this script actually saw.
///
/// `tests/generated.rs` compares `CORPUS_FILES_AT_BUILD_TIME` against `corpus_files()` at run
/// time. Cargo tracks a `rerun-if-changed` directory by the directory's own mtime and `mv`
/// preserves mtime, so restoring a corpus that was moved aside does not always re-run this
/// script; without that comparison the empty file written while the corpus was absent survives
/// and `cargo test` reports `0 passed` as success (OSP-15).
fn counts_prelude(corpus_files: usize, generated_tests: usize) -> String {
    format!(
        "/// Number of `.xosc` files the build script saw under `conformance/corpus/`.\n\
         const CORPUS_FILES_AT_BUILD_TIME: usize = {corpus_files};\n\n\
         /// Number of generated round-trip test functions below.\n\
         const GENERATED_ROUNDTRIP_TESTS: usize = {generated_tests};\n\n"
    )
}

fn sanitize(path: &str) -> String {
    path.to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let corpus_dir = Path::new(&manifest_dir).join("corpus");
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_roundtrip.rs");

    let manifest_path = Path::new(&manifest_dir).join("expectations.toml");

    // Rerun when the corpus appears, disappears, or changes shape. Registered before the early
    // return so that fetching a corpus into a fresh checkout regenerates the tests.
    println!("cargo:rerun-if-changed={}", corpus_dir.display());
    println!("cargo:rerun-if-changed={}", manifest_path.display());

    let mut generated = String::new();
    // `WalkDir` on a nonexistent directory yields no entries rather than erroring, so an absent
    // corpus and an empty one land in the same place below.
    let mut entries: Vec<_> = walkdir::WalkDir::new(&corpus_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "xosc")
                .unwrap_or(false)
        })
        .collect();
    entries.sort_by_key(|e| e.path().to_path_buf());

    if entries.is_empty() {
        // The corpus is fetched on demand, not vendored, so this is the normal state of a fresh
        // checkout. Emit the counts and nothing else: `tests/generated.rs` then compiles to its
        // own staleness check and zero round-trip tests, and the rest of the harness (the builder
        // fixtures, which need no corpus) still runs. The counts are what stops that empty file
        // from passing vacuously once a corpus is present again — see `counts_prelude`.
        fs::write(&dest_path, counts_prelude(0, 0)).unwrap();
        println!("cargo:warning=conformance corpus not present; run scripts/fetch-corpus.sh to enable the corpus gates");
        return;
    }

    // Files the manifest exempts from the `report` gate get no generated test: the generated test
    // and `src/bin/report.rs` ask the same question of the same file, so exempting one without
    // the other would leave `cargo test` red and the binary green. Every other exemption
    // (`lossy`, `validate`) leaves the round-trip test in place.
    //
    // Read only after the empty-corpus early return, so a fresh checkout with no corpus is never
    // broken by the manifest. `src/lib.rs` does the strict checking (stale paths, duplicates,
    // empty `gates`, the input-file assertions); here a malformed manifest is a hard error too,
    // because generating tests from a manifest that could not be understood would silently gate
    // more or less than intended.
    let manifest: Manifest = match fs::read_to_string(&manifest_path) {
        Ok(text) => toml::from_str(&text).unwrap_or_else(|e| {
            panic!("cannot parse {}: {e}", manifest_path.display());
        }),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Manifest::default(),
        Err(e) => panic!("cannot read {}: {e}", manifest_path.display()),
    };
    let report_exempt: Vec<&str> = manifest
        .entry
        .iter()
        .filter(|e| e.gates.iter().any(|g| g == "report"))
        .map(|e| e.path.as_str())
        .collect();

    let corpus_files = entries.len();
    let mut generated_tests = 0usize;

    for entry in entries {
        let path = entry.path();
        let rel = path.strip_prefix(&corpus_dir).unwrap();
        if report_exempt.contains(&rel.to_string_lossy().as_ref()) {
            // Still rerun if the file changes: an exempted file that upstream fixes must not be
            // stuck on a stale build.
            println!("cargo:rerun-if-changed={}", path.display());
            continue;
        }
        let test_name = format!("roundtrip_{}", sanitize(&rel.to_string_lossy()));
        let abs_path = path.to_string_lossy();
        let is_catalog = rel
            .components()
            .any(|c| c.as_os_str().to_string_lossy().to_lowercase() == "catalogs");
        let helper = if is_catalog {
            "assert_catalog_roundtrip_fixed_point"
        } else {
            "assert_roundtrip_fixed_point"
        };

        generated.push_str(&format!(
            "#[test]\nfn {test_name}() {{\n    openscenario_roundtrip_harness::{helper}(r#\"{abs_path}\"#);\n}}\n\n",
        ));
        generated_tests += 1;

        println!("cargo:rerun-if-changed={}", path.display());
    }

    let mut out = counts_prelude(corpus_files, generated_tests);
    out.push_str(&generated);
    fs::write(&dest_path, out).unwrap();
}
