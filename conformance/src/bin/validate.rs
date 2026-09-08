//! Validates the crate's *serialized output* against the OpenSCENARIO XSD.
//!
//! The `report` binary checks a round-trip fixed point (`xml1 == xml2`) and `lossy` compares the
//! original file against `xml1` as a multiset of element/attribute paths. Neither can see a
//! schema violation that is stable across both passes: an invented element, a `xsd:sequence`
//! emitted in the wrong order, or an empty choice group all round-trip cleanly while producing
//! output no conforming consumer would accept.
//!
//! This binary parses each corpus file, serializes it to `xml1`, and validates `xml1` against
//! the crate's `Schema/OpenSCENARIO.xsd`. Every corpus file is itself XSD-valid, so any failure here is
//! a crate bug.

use openscenario_roundtrip_harness::xml_profile::{first_line, print_ranked};

use openscenario_roundtrip_harness::{corpus_dir, is_catalog, load_schema, require_corpus};
use openscenario_rs::{
    parse_catalog_from_file, parse_from_file, serialize_catalog_to_string, serialize_to_string,
};
use std::collections::BTreeMap;
use std::env;

fn main() {
    let corpus_dir = corpus_dir();
    let verbose = env::args().any(|a| a == "-v" || a == "--verbose");
    let entries = require_corpus();

    // Parse the 2500-line schema exactly once and reuse the context for every file.
    let mut validator = match load_schema() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("failed to load schema: {e}");
            std::process::exit(2);
        }
    };

    // Corpus-wide tally of how many times each distinct error message occurs, so the biggest
    // single cause of invalid output sorts to the top.
    let mut message_totals: BTreeMap<String, i64> = BTreeMap::new();
    let mut type_totals: BTreeMap<String, i64> = BTreeMap::new();

    let mut valid_files = 0usize;
    let mut invalid_files = 0usize;
    let mut skipped = 0usize;
    let mut total_errors = 0usize;

    for path in &entries {
        let rel = path.strip_prefix(&corpus_dir).unwrap();
        let path_str = path.to_string_lossy().to_string();

        let produced_xml = if is_catalog(rel) {
            parse_catalog_from_file(&path_str)
                .map_err(|e| e.to_string())
                .and_then(|d| serialize_catalog_to_string(&d).map_err(|e| e.to_string()))
        } else {
            parse_from_file(&path_str)
                .map_err(|e| e.to_string())
                .and_then(|d| serialize_to_string(&d).map_err(|e| e.to_string()))
        };

        let produced_xml = match produced_xml {
            Ok(x) => x,
            Err(e) => {
                skipped += 1;
                println!("SKIP  {}  ({})", rel.display(), first_line(&e));
                continue;
            }
        };

        let errors = match validator.validate_str(&produced_xml) {
            Ok(e) => e,
            Err(e) => {
                skipped += 1;
                println!("SKIP  {}  ({})", rel.display(), first_line(&e.to_string()));
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
            *message_totals.entry(normalize(&error.message)).or_insert(0) += 1;
            *type_totals.entry(error.error_type.clone()).or_insert(0) += 1;
            if verbose {
                println!("        {error}");
            }
        }
    }

    println!("\n--- summary ---");
    println!(
        "{valid_files} schema-valid, {invalid_files} schema-invalid, {skipped} skipped, {} total",
        entries.len()
    );
    println!("{total_errors} total validation errors");

    print_ranked("error messages", &message_totals, 40);
    print_ranked("error types", &type_totals, 40);

    // Exit non-zero so this is a gate, not just a report: a caller checking `$?` must be able to
    // tell a clean run from a regression without parsing the summary text. `skipped` counts as
    // failure too — a file that could not be parsed or serialized was never actually validated,
    // and silently passing it would hide exactly the kind of breakage this is here to catch.
    if invalid_files > 0 || skipped > 0 {
        std::process::exit(1);
    }
}

/// Collapses the file-specific parts of a libxml message so equivalent failures group together.
///
/// libxml messages embed the offending element's own name (`Element 'Foo': ...`), which would
/// otherwise scatter one root cause across dozens of near-identical strings. The element name is
/// kept — it is the actionable part — but nothing else varies per file, so this is currently just
/// a trim. Kept as a seam for future normalization.
fn normalize(message: &str) -> String {
    message.trim().to_string()
}
