//! Validates each corpus file **as it sits on disk** against the OpenSCENARIO XSD.
//!
//! The other three gates all ask a question about something this crate produced: `report`
//! compares `xml1` with `xml2`, `lossy` compares the original file with `xml1`, and `validate`
//! checks `xml1` against the schema. None of them asks whether the *input* was conforming in the
//! first place, and that gap is not academic. A schema-invalid input whose invalid part the crate
//! does not model is parsed, the offending content is dropped, and the output is schema-valid —
//! so `validate` goes green precisely *because* something was lost, and earlier `lossy`
//! could not see the loss either, because it could not see character content at all.
//!
//! This binary closes that hole from the other side: a corpus file that is not itself XSD-valid
//! is flagged as such, rather than being quietly improved into a valid one. It never parses the
//! file with this crate, so its result is a fact about the corpus, independent of the code.
//!
//! **Not every corpus file is XSD-valid** — upstream openpass ships fixtures that are invalid on
//! purpose. Those are named in `conformance/expectations.toml` with `validate-input` among their
//! `gates`, and, exactly as in the other binaries, an exclusion is never a bare opt-out: each
//! excluded file is still validated, must still fail (a stale exemption is an error), must still
//! prove the premise its `assert` states, and is named on its own summary line.

#![deny(unused_must_use)]

use openscenario_roundtrip_harness::xml_profile::{first_line, print_ranked};
use openscenario_roundtrip_harness::{
    corpus_dir, load_schema, require_corpus, stale_exemption_message, Expectations, Gate,
};
use openscenario_rs::validation::ValidationError;
use std::collections::BTreeMap;
use std::env;

fn main() {
    let corpus_dir = corpus_dir();
    let verbose = env::args().any(|a| a == "-v" || a == "--verbose");
    let entries = require_corpus();

    let expectations = match Expectations::load_for(&entries) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2);
        }
    };

    // Parse the 2500-line schema exactly once and reuse the context for every file.
    let mut validator = match load_schema() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("failed to load schema: {e}");
            std::process::exit(2);
        }
    };

    let mut message_totals: BTreeMap<String, i64> = BTreeMap::new();
    let mut type_totals: BTreeMap<String, i64> = BTreeMap::new();

    let mut valid_files = 0usize;
    let mut invalid_files = 0usize;
    let mut skipped = 0usize;
    let mut total_errors = 0usize;
    let mut excluded: Vec<String> = Vec::new();
    let mut broken_expectations: Vec<String> = Vec::new();

    for path in &entries {
        let rel = path.strip_prefix(&corpus_dir).unwrap();

        // `Err` here means libxml could not even read the document (unreadable file, not
        // well-formed XML); `Ok(errors)` is the only outcome that says anything about validity.
        let outcome: Result<Vec<ValidationError>, String> = validator
            .validate_file(path)
            .map_err(|e| first_line(&e.to_string()).to_string());

        if let Some(entry) = expectations.exemption(rel, Gate::ValidateInput) {
            // Excluded, but still run: the exemption has to keep earning its place.
            match &outcome {
                Err(e) => println!(
                    "XFAIL {}  ({e})  (expectations.toml: {})",
                    rel.display(),
                    entry.assertion
                ),
                Ok(errors) if !errors.is_empty() => {
                    println!(
                        "XFAIL {}  ({} errors)  (expectations.toml: {})",
                        rel.display(),
                        errors.len(),
                        entry.assertion
                    );
                    if verbose {
                        for error in errors {
                            println!("        {error}");
                        }
                    }
                }
                Ok(_) => {
                    broken_expectations.push(stale_exemption_message(rel, Gate::ValidateInput))
                }
            }
            excluded.push(rel.display().to_string());
            continue;
        }

        let errors = match outcome {
            Ok(e) => e,
            Err(e) => {
                skipped += 1;
                println!("SKIP  {}  ({e})", rel.display());
                continue;
            }
        };

        if errors.is_empty() {
            valid_files += 1;
            if verbose {
                println!("VALID {}", rel.display());
            }
            continue;
        }

        invalid_files += 1;
        total_errors += errors.len();
        println!("INVALID {}  ({} errors)", rel.display(), errors.len());
        for error in &errors {
            *message_totals
                .entry(error.message.trim().to_string())
                .or_insert(0) += 1;
            *type_totals.entry(error.error_type.clone()).or_insert(0) += 1;
            if verbose {
                println!("        {error}");
            }
        }
    }

    // The other half of what makes an exemption more than a skip: prove each entry's claim about
    // the input file. Run for every entry in the manifest, not just the ones this gate excluded.
    broken_expectations.extend(expectations.check_premises(&mut validator));

    println!("\n--- summary ---");
    println!(
        "{valid_files} input-schema-valid, {invalid_files} input-schema-invalid, {skipped} skipped, \
         {} excluded, {} total",
        excluded.len(),
        entries.len()
    );
    println!("{total_errors} total validation errors");
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

    print_ranked("error messages", &message_totals, 40);
    print_ranked("error types", &type_totals, 40);

    // Same reasoning as the other three: exit non-zero so a caller can gate on `$?` rather than
    // scraping the summary. `skipped` counts as failure too — a file libxml could not read was
    // never actually validated. That rule is unchanged for every file the manifest does not name;
    // an expectation entry is an assertion about one specific file, never a widened filter.
    if invalid_files > 0 || skipped > 0 || !broken_expectations.is_empty() {
        std::process::exit(1);
    }
}
