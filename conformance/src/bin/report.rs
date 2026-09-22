//! Walks `corpus/`, round-trips every `.xosc` file, and prints one line per file
//! (PASS or FAIL + stage + error), followed by a summary grouped by failure stage.
//!
//! Files listed in `conformance/expectations.toml` with `report` among their `gates` are excluded
//! from the pass/fail tally — but never silently. Each is still run, must still fail (a stale
//! exemption is an error), must still prove the premise its `assert` states about the input file,
//! and is named on its own summary line.

#![deny(unused_must_use)]

use openscenario_roundtrip_harness::xml_profile::first_line;
use openscenario_roundtrip_harness::{
    check_catalog_roundtrip, check_roundtrip, corpus_dir, is_catalog, load_schema, require_corpus,
    stale_exemption_message, Expectations, Gate,
};
use std::collections::BTreeMap;

fn main() {
    let corpus_dir = corpus_dir();
    let entries = require_corpus();

    let expectations = match Expectations::load_for(&entries) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2);
        }
    };

    let mut failures_by_stage: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut pass_count = 0usize;
    let mut fail_count = 0usize;
    let mut excluded: Vec<String> = Vec::new();
    let mut broken_expectations: Vec<String> = Vec::new();

    for path in &entries {
        let rel = path.strip_prefix(&corpus_dir).unwrap();
        let path_str = path.to_string_lossy().to_string();

        let result = if is_catalog(rel) {
            check_catalog_roundtrip(&path_str)
        } else {
            check_roundtrip(&path_str)
        };

        if let Some(entry) = expectations.exemption(rel, Gate::Report) {
            // Excluded, but still run: the exemption has to keep earning its place.
            match &result {
                Ok(()) => broken_expectations.push(stale_exemption_message(rel, Gate::Report)),
                Err(e) => println!(
                    "XFAIL {}  [{}]  {}  (expectations.toml: {})",
                    rel.display(),
                    e.stage,
                    first_line(&e.message),
                    entry.assertion
                ),
            }
            excluded.push(rel.display().to_string());
            continue;
        }

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

    // The other half of what makes an exemption more than a skip: prove each entry's claim about
    // the input file. Run for every entry in the manifest, not just the ones this gate excluded.
    match load_schema() {
        Ok(mut validator) => {
            broken_expectations.extend(expectations.check_premises(&mut validator))
        }
        Err(e) => broken_expectations.push(format!(
            "cannot load Schema/OpenSCENARIO.xsd to check expectations.toml assertions: {e}"
        )),
    }

    println!("\n--- summary ---");
    println!(
        "{pass_count} passed, {fail_count} failed, {} excluded, {} total",
        excluded.len(),
        entries.len()
    );
    for (stage, files) in &failures_by_stage {
        println!("\n{} failures at [{stage}]:", files.len());
        for f in files {
            println!("  {f}");
        }
    }
    if !excluded.is_empty() {
        println!(
            "\n{} excluded by conformance/expectations.toml:",
            excluded.len()
        );
        for f in &excluded {
            println!("  {f}");
        }
    }
    if !broken_expectations.is_empty() {
        println!(
            "\n{} broken expectation(s) in conformance/expectations.toml:",
            broken_expectations.len()
        );
        for f in &broken_expectations {
            println!("  {f}");
        }
    }

    if fail_count > 0 || !broken_expectations.is_empty() {
        std::process::exit(1);
    }
}
