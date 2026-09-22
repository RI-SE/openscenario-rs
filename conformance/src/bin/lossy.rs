//! Measures how much of each `.xosc` file is *lost* on the first parse.
//!
//! The `report` binary checks a round-trip *fixed point* (`xml1 == xml2`), which cannot see
//! data dropped at the first parse: serde ignores unknown XML attributes and elements, so a
//! field the Rust types don't model is dropped identically on both passes and the comparison
//! still succeeds. This binary instead compares the **original file** against `xml1`.
//!
//! Both sides are reduced to a multiset of `path/to/Element` and `path/to/Element@attr` keys,
//! so attribute order and whitespace don't matter. Anything in the original but not in `xml1`
//! is dropped (silent data loss); anything in `xml1` but not the original is invented
//! (schema-invalid output).
//!
//! Files listed in `conformance/expectations.toml` with `lossy` among their `gates` are excluded
//! from the tally — but never silently. Each is still run, must still fail (a stale exemption is
//! an error), must still prove the premise its `assert` states about the input file, and is named
//! on its own summary line.

#![deny(unused_must_use)]

use openscenario_roundtrip_harness::xml_profile::{
    compare_values, diff, first_line, print_ranked, profile, values, ValueDiff,
};
use openscenario_roundtrip_harness::{
    corpus_dir, is_catalog, load_schema, require_corpus, stale_exemption_message, Expectations,
    Gate,
};
use openscenario_rs::{
    parse_catalog_from_file, parse_from_file, serialize_catalog_to_string, serialize_to_string,
};
use std::collections::BTreeMap;
use std::env;
use std::path::Path;

/// What this gate learned about one file: the dropped and invented keys, the value rewrites
/// found by [`compare_values`], or the reason the file could not be examined at all (which counts
/// as a failure, not a pass — see the exit code below).
type Outcome = Result<(Vec<(String, i64)>, Vec<(String, i64)>, ValueDiff), String>;

fn examine(path: &Path, rel: &Path) -> Outcome {
    let path_str = path.to_string_lossy().to_string();

    let produced_xml = if is_catalog(rel) {
        parse_catalog_from_file(&path_str)
            .map_err(|e| e.to_string())
            .and_then(|d| serialize_catalog_to_string(&d).map_err(|e| e.to_string()))
    } else {
        parse_from_file(&path_str)
            .map_err(|e| e.to_string())
            .and_then(|d| serialize_to_string(&d).map_err(|e| e.to_string()))
    }
    .map_err(|e| first_line(&e).to_string())?;

    let original_xml = std::fs::read_to_string(path).map_err(|e| e.to_string())?;

    let (before, after) = match (profile(&original_xml), profile(&produced_xml)) {
        (Ok(b), Ok(a)) => (b, a),
        _ => return Err("unparseable as XML".to_string()),
    };
    let d = diff(&before, &after);

    let (before_values, after_values) = match (values(&original_xml), values(&produced_xml)) {
        (Ok(b), Ok(a)) => (b, a),
        _ => return Err("unparseable as XML".to_string()),
    };
    let value_diff = compare_values(&before_values, &after_values);

    Ok((d.dropped, d.invented, value_diff))
}

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

    // How many distinct keys each dropped/invented path accounts for, corpus-wide.
    let mut dropped_totals: BTreeMap<String, i64> = BTreeMap::new();
    let mut invented_totals: BTreeMap<String, i64> = BTreeMap::new();

    let mut clean_files = 0usize;
    let mut lossy_files = 0usize;
    let mut skipped = 0usize;
    let mut total_dropped = 0i64;
    let mut total_invented = 0i64;
    let mut total_rewritten = 0usize;
    let mut total_number_format_only = 0usize;
    let mut files_with_number_format_only = 0usize;
    let mut excluded: Vec<String> = Vec::new();
    let mut broken_expectations: Vec<String> = Vec::new();

    for path in &entries {
        let rel = path.strip_prefix(&corpus_dir).unwrap();
        let outcome = examine(path, rel);

        if let Some(entry) = expectations.exemption(rel, Gate::Lossy) {
            // Excluded, but still run: the exemption has to keep earning its place.
            match &outcome {
                Err(e) => println!(
                    "XFAIL {}  ({e})  (expectations.toml: {})",
                    rel.display(),
                    entry.assertion
                ),
                Ok((dropped, invented, value_diff))
                    if !dropped.is_empty() || !invented.is_empty() || !value_diff.is_empty() =>
                {
                    println!(
                        "XFAIL {}  (lossy)  (expectations.toml: {})",
                        rel.display(),
                        entry.assertion
                    )
                }
                Ok(_) => broken_expectations.push(stale_exemption_message(rel, Gate::Lossy)),
            }
            excluded.push(rel.display().to_string());
            continue;
        }

        let (dropped, invented, value_diff) = match outcome {
            Ok(d) => d,
            Err(e) => {
                skipped += 1;
                println!("SKIP  {}  ({e})", rel.display());
                continue;
            }
        };

        let file_dropped: i64 = dropped.iter().map(|(_, n)| n).sum();
        let file_invented: i64 = invented.iter().map(|(_, n)| n).sum();
        let file_rewritten = value_diff.mismatches.len();

        if value_diff.number_format_only > 0 {
            files_with_number_format_only += 1;
            total_number_format_only += value_diff.number_format_only;
        }

        if file_dropped == 0 && file_invented == 0 && file_rewritten == 0 {
            clean_files += 1;
            if verbose {
                println!("CLEAN {}", rel.display());
            }
            continue;
        }

        lossy_files += 1;
        total_dropped += file_dropped;
        total_invented += file_invented;
        total_rewritten += file_rewritten;
        println!(
            "LOSSY {}  (-{file_dropped} dropped, +{file_invented} invented, ~{file_rewritten} rewritten)",
            rel.display()
        );
        for (key, n) in &dropped {
            *dropped_totals.entry(key.clone()).or_insert(0) += n;
            if verbose {
                println!("        - {key} x{n}");
            }
        }
        for (key, n) in &invented {
            *invented_totals.entry(key.clone()).or_insert(0) += n;
            if verbose {
                println!("        + {key} x{n}");
            }
        }
        if verbose {
            for (key, before, after) in &value_diff.mismatches {
                println!("        ~ {key}: \"{before}\" \u{2192} \"{after}\"");
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
        "{clean_files} lossless, {lossy_files} lossy, {skipped} skipped, {} excluded, {} total",
        excluded.len(),
        entries.len()
    );
    println!("{total_dropped} dropped items, {total_invented} invented items, {total_rewritten} rewritten values");
    println!(
        "{total_number_format_only} number-format-only rewrites (e.g. \"1.0\" vs \"1\") across {files_with_number_format_only} files — not a failure, the value space is unchanged"
    );
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

    print_ranked("dropped paths", &dropped_totals, 40);
    print_ranked("invented paths", &invented_totals, 40);

    // Same reasoning as `validate.rs`: exit non-zero so a caller can gate on `$?` rather than
    // scraping the summary line. The corpus has been at zero since pass 3, so any non-zero count
    // here is a regression. `skipped` still counts as failure for every file the manifest does
    // not name — an exemption is an assertion about one file, never a widened filter.
    if lossy_files > 0 || skipped > 0 || !broken_expectations.is_empty() {
        std::process::exit(1);
    }
}
