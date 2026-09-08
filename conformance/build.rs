use std::env;
use std::fs;
use std::path::Path;

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

    // Rerun when the corpus appears, disappears, or changes shape. Registered before the early
    // return so that fetching a corpus into a fresh checkout regenerates the tests.
    println!("cargo:rerun-if-changed={}", corpus_dir.display());

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
        // checkout. Emit an empty file: `tests/generated.rs` then compiles to zero tests and the
        // rest of the harness (the builder fixtures, which need no corpus) still runs.
        fs::write(&dest_path, "").unwrap();
        println!("cargo:warning=conformance corpus not present; run scripts/fetch-corpus.sh to enable the corpus gates");
        return;
    }

    for entry in entries {
        let path = entry.path();
        let rel = path.strip_prefix(&corpus_dir).unwrap();
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

        println!("cargo:rerun-if-changed={}", path.display());
    }

    fs::write(&dest_path, generated).unwrap();
}
