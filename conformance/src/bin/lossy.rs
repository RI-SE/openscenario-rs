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

use openscenario_roundtrip_harness::xml_profile::{diff, first_line, print_ranked, profile};
use openscenario_roundtrip_harness::{corpus_dir, is_catalog, require_corpus};
use openscenario_rs::{
    parse_catalog_from_file, parse_from_file, serialize_catalog_to_string, serialize_to_string,
};
use std::collections::BTreeMap;
use std::env;

fn main() {
    let corpus_dir = corpus_dir();
    let verbose = env::args().any(|a| a == "-v" || a == "--verbose");
    let entries = require_corpus();

    // How many distinct keys each dropped/invented path accounts for, corpus-wide.
    let mut dropped_totals: BTreeMap<String, i64> = BTreeMap::new();
    let mut invented_totals: BTreeMap<String, i64> = BTreeMap::new();

    let mut clean_files = 0usize;
    let mut lossy_files = 0usize;
    let mut skipped = 0usize;
    let mut total_dropped = 0i64;
    let mut total_invented = 0i64;

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

        let original_xml = match std::fs::read_to_string(path) {
            Ok(x) => x,
            Err(e) => {
                skipped += 1;
                println!("SKIP  {}  ({e})", rel.display());
                continue;
            }
        };

        let (before, after) = match (profile(&original_xml), profile(&produced_xml)) {
            (Ok(b), Ok(a)) => (b, a),
            _ => {
                skipped += 1;
                println!("SKIP  {}  (unparseable as XML)", rel.display());
                continue;
            }
        };

        let d = diff(&before, &after);
        let file_dropped: i64 = d.dropped.iter().map(|(_, n)| n).sum();
        let file_invented: i64 = d.invented.iter().map(|(_, n)| n).sum();

        if file_dropped == 0 && file_invented == 0 {
            clean_files += 1;
            if verbose {
                println!("CLEAN {}", rel.display());
            }
            continue;
        }

        lossy_files += 1;
        total_dropped += file_dropped;
        total_invented += file_invented;
        println!(
            "LOSSY {}  (-{file_dropped} dropped, +{file_invented} invented)",
            rel.display()
        );
        for (key, n) in &d.dropped {
            *dropped_totals.entry(key.clone()).or_insert(0) += n;
            if verbose {
                println!("        - {key} x{n}");
            }
        }
        for (key, n) in &d.invented {
            *invented_totals.entry(key.clone()).or_insert(0) += n;
            if verbose {
                println!("        + {key} x{n}");
            }
        }
    }

    println!("\n--- summary ---");
    println!(
        "{clean_files} lossless, {lossy_files} lossy, {skipped} skipped, {} total",
        entries.len()
    );
    println!("{total_dropped} dropped items, {total_invented} invented items");

    print_ranked("dropped paths", &dropped_totals, 40);
    print_ranked("invented paths", &invented_totals, 40);

    // Same reasoning as `validate.rs`: exit non-zero so a caller can gate on `$?` rather than
    // scraping the summary line. The corpus has been at zero since pass 3, so any non-zero count
    // here is a regression.
    if lossy_files > 0 || skipped > 0 {
        std::process::exit(1);
    }
}
