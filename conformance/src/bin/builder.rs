//! Runs every builder fixture through the round-trip, fidelity, and schema checks.
//!
//! This is the builder's counterpart to `report` + `validate`. The parser binaries get their
//! corpus from `corpus/`; this one gets it from [`builder_fixtures::fixtures`], because the
//! builder's input is code rather than files. The checks themselves are the same three
//! questions: does it survive a round trip, did anything get dropped, and would a conforming
//! consumer accept the result.
//!
//! `--coverage` switches to a different question entirely — not "is the output valid" but "how
//! much of a real scenario can the builder even express". It compares the ALKS 4.1.1 builder
//! program against the real corpus file it reconstructs. That comparison is reported, never
//! gated: the builder reaches perhaps half of the type system, so a non-zero diff is a coverage
//! figure, not a regression.

use openscenario_roundtrip_harness::builder_fixtures;
use openscenario_roundtrip_harness::xml_profile::{diff, first_line, print_ranked, profile};
use openscenario_roundtrip_harness::{
    check_builder_fixture, corpus_dir, corpus_files, load_schema, MISSING_CORPUS_HINT,
};
use openscenario_rs::serialize_to_string;
use std::collections::BTreeMap;
use std::env;

fn main() {
    let verbose = env::args().any(|a| a == "-v" || a == "--verbose");

    if env::args().any(|a| a == "--coverage") {
        coverage_report(verbose);
        return;
    }

    let mut validator = match load_schema() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("failed to load schema: {e}");
            std::process::exit(2);
        }
    };

    let fixtures = builder_fixtures::fixtures();
    let mut stage_totals: BTreeMap<String, i64> = BTreeMap::new();
    let mut passed = 0usize;
    let mut failed = 0usize;

    for fixture in &fixtures {
        let failures = check_builder_fixture(fixture, &mut validator);

        if failures.is_empty() {
            passed += 1;
            if verbose {
                println!("PASS  {}", fixture.name);
            }
            continue;
        }

        failed += 1;
        let stages: Vec<String> = failures.iter().map(|f| f.stage.to_string()).collect();
        println!("FAIL  {}  [{}]", fixture.name, stages.join(", "));
        for failure in &failures {
            *stage_totals.entry(failure.stage.to_string()).or_insert(0) += 1;
            if verbose {
                println!("        {}", failure.message);
            } else {
                println!(
                    "        [{}] {}",
                    failure.stage,
                    first_line(&failure.message)
                );
            }
        }
    }

    println!("\n--- summary ---");
    println!("{passed} passed, {failed} failed, {} total", fixtures.len());
    print_ranked("failing stages", &stage_totals, 40);

    // Same contract as the corpus binaries: a caller gates on `$?` rather than scraping text.
    if failed > 0 {
        std::process::exit(1);
    }
}

/// Compares the ALKS 4.1.1 builder program against the real corpus file it reconstructs.
fn coverage_report(verbose: bool) {
    // The `..._variation.xosc` sibling is a ParameterValueDistribution wrapper, not a scenario;
    // the concrete template is the document the builder program is actually reconstructing.
    // Looked up by file name rather than a fixed relative path, because the fetched corpus nests
    // each upstream repository under its own directory and that prefix is not ours to assume.
    const REFERENCE_NAME: &str = "alks_scenario_4_1_1_free_driving_template.xosc";
    // Only this mode needs the corpus; the default fixture run above is pure code and works in a
    // fresh checkout. Bail with the fetch hint rather than reporting a coverage figure computed
    // against a file that isn't there.
    let Some(reference) = corpus_files()
        .into_iter()
        .find(|p| p.file_name().map(|n| n == REFERENCE_NAME).unwrap_or(false))
    else {
        eprintln!("{MISSING_CORPUS_HINT}");
        eprintln!("(no {REFERENCE_NAME} under {})", corpus_dir().display());
        std::process::exit(1);
    };

    let built = match builder_fixtures::alks_4_1_1_free_driving() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("builder failed: {e}");
            std::process::exit(2);
        }
    };
    let produced = match serialize_to_string(&built) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("serialize failed: {e}");
            std::process::exit(2);
        }
    };
    let original = match std::fs::read_to_string(&reference) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("failed to read {}: {e}", reference.display());
            std::process::exit(2);
        }
    };

    let (before, after) = match (profile(&original), profile(&produced)) {
        (Ok(b), Ok(a)) => (b, a),
        _ => {
            eprintln!("one side was not well-formed XML");
            std::process::exit(2);
        }
    };

    let d = diff(&before, &after);
    println!("--- builder coverage vs {} ---", reference.display());
    println!(
        "{} distinct paths in the reference, {} in builder output",
        before.len(),
        after.len()
    );
    println!(
        "{} not reproduced, {} not present in the reference",
        d.dropped_count(),
        d.invented_count()
    );

    if verbose {
        let dropped: BTreeMap<String, i64> = d.dropped.iter().cloned().collect();
        let invented: BTreeMap<String, i64> = d.invented.iter().cloned().collect();
        print_ranked("paths the builder cannot express", &dropped, 40);
        print_ranked("paths only the builder emits", &invented, 40);
    } else {
        println!("(pass -v to list the paths)");
    }

    // Deliberately no exit(1): this measures breadth, it does not gate.
}
