//! Walks `corpus/`, round-trips every `.xosc` file, and prints one line per file
//! (PASS or FAIL + stage + error), followed by a summary grouped by failure stage.

use openscenario_roundtrip_harness::xml_profile::first_line;
use openscenario_roundtrip_harness::{
    check_catalog_roundtrip, check_roundtrip, corpus_dir, is_catalog, require_corpus,
};
use std::collections::BTreeMap;

fn main() {
    let corpus_dir = corpus_dir();
    let entries = require_corpus();

    let mut failures_by_stage: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut pass_count = 0usize;
    let mut fail_count = 0usize;

    for path in &entries {
        let rel = path.strip_prefix(&corpus_dir).unwrap();
        let path_str = path.to_string_lossy().to_string();

        let result = if is_catalog(rel) {
            check_catalog_roundtrip(&path_str)
        } else {
            check_roundtrip(&path_str)
        };

        match result {
            Ok(()) => {
                pass_count += 1;
                println!("PASS  {}", rel.display());
            }
            Err(e) => {
                fail_count += 1;
                println!(
                    "FAIL  {}  [{}]  {}",
                    rel.display(),
                    e.stage,
                    first_line(&e.message)
                );
                failures_by_stage
                    .entry(e.stage.to_string())
                    .or_default()
                    .push(rel.display().to_string());
            }
        }
    }

    println!("\n--- summary ---");
    println!(
        "{pass_count} passed, {fail_count} failed, {} total",
        entries.len()
    );
    for (stage, files) in &failures_by_stage {
        println!("\n{} failures at [{stage}]:", files.len());
        for f in files {
            println!("  {f}");
        }
    }

    if fail_count > 0 {
        std::process::exit(1);
    }
}
