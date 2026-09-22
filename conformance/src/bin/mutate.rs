//! Asks the half of the conformance question the corpus cannot ask: **does the crate refuse what
//! the schema refuses?**
//!
//! `report`, `lossy`, `validate` and `validate-input` all feed the crate valid documents, so all
//! four stay green over a type that accepts anything. Every defect of that shape — a choice
//! modelled as parallel `Option`s, a required repeated element with no `minOccurs`, a `flatten`ed
//! child — parsed an invalid document without complaint while the gate reported eleven green
//! stages.
//!
//! This binary breaks each corpus file in XSD-guided ways and compares two verdicts on the result:
//!
//! 1. **the oracle** — libxml2 against `Schema/OpenSCENARIO.xsd`, the same validator `validate`
//!    and `validate-input` trust. A mutant it still accepts is discarded, not tested: a mutation
//!    that removed an element which is optional in that context proves nothing.
//! 2. **the crate** — `parse_from_str`, or the catalog entry point for a catalog, exactly as
//!    `lossy` chooses between them.
//!
//! A mutant the schema rejects and the crate accepts is a **hole**. Holes are grouped by mutation
//! kind and XSD type, because that is the granularity at which one is fixed.
//!
//! The converse is a defect too, so it is counted rather than ignored: an unmutated control per
//! file, which the crate must parse, and the schema-valid mutants, which it must also parse.

#![deny(unused_must_use)]

use openscenario_roundtrip_harness::mutate::{fnv1a, mutants, resolve, Donors, Element, Kind, Xsd};
use openscenario_roundtrip_harness::{
    corpus_dir, is_catalog, load_schema, require_corpus, schema_path, stale_hole_message,
    Expectations,
};
use openscenario_rs::validation::XsdValidator;
use openscenario_rs::{
    parse_catalog_from_file, parse_catalog_from_str, parse_from_file, parse_from_str,
};
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::path::Path;

/// Mutants per kind per file.
///
/// Per kind rather than per file so that `bad-enum`, which has few sites, is not crowded out by
/// `drop-required-attr`, which has hundreds. Two rather than one because a single mutant per kind
/// would exercise one type per file and the corpus repeats the same few types at its top levels;
/// two rather than ten because the cost is one libxml2 validation plus one crate parse per mutant,
/// and the gate is meant to be seconds rather than minutes. The trade is coverage of the long
/// tail: a type that appears in one file, at one site, may never be mutated by a kind whose
/// candidates that file has many of.
const CAP: usize = 2;

/// Mixed into the per-file seed. Changing it reshuffles which candidates are chosen corpus-wide,
/// so it is a constant rather than an option: a gate whose selection a caller can vary is a gate
/// whose green result means something different each run.
const SEED: u64 = 0x05CE_4A11_0000_0001;

/// One mutant the crate accepted although the schema rejected it.
#[derive(Debug)]
struct Hole {
    kind: Kind,
    type_name: String,
    element_path: String,
    detail: String,
    file: String,
}

/// Per-kind tallies, printed as the summary table.
#[derive(Debug, Default, Clone, Copy)]
struct Tally {
    generated: usize,
    still_valid: usize,
    schema_rejected: usize,
    crate_rejected: usize,
    holes: usize,
    over_strict: usize,
}

/// Joins the distinct edits of one hole class, keeping the line readable when a class is wide.
fn summarize(details: &BTreeSet<&str>) -> String {
    const SHOWN: usize = 6;
    let head = details
        .iter()
        .take(SHOWN)
        .copied()
        .collect::<Vec<_>>()
        .join("; ");
    if details.len() > SHOWN {
        format!("{head}; and {} more", details.len() - SHOWN)
    } else {
        head
    }
}

/// The crate's verdict on one document. `Ok(())` means it parsed.
fn crate_parses(xml: &str, catalog: bool) -> Result<(), String> {
    let outcome = if catalog {
        parse_catalog_from_str(xml).map(|_| ())
    } else {
        parse_from_str(xml).map(|_| ())
    };
    outcome.map_err(|e| e.to_string())
}

fn crate_parses_file(path: &Path, catalog: bool) -> Result<(), String> {
    let path = path.to_string_lossy().to_string();
    let outcome = if catalog {
        parse_catalog_from_file(&path).map(|_| ())
    } else {
        parse_from_file(&path).map(|_| ())
    };
    outcome.map_err(|e| e.to_string())
}

/// Whether the oracle rejects a document. An error from the validator itself counts as a
/// rejection only when it is a validation error; a failure to run is reported to the caller.
fn schema_rejects(validator: &mut XsdValidator, xml: &str) -> Result<bool, String> {
    validator
        .validate_str(xml)
        .map(|errors| !errors.is_empty())
        .map_err(|e| e.to_string())
}

fn main() {
    let verbose = env::args().any(|a| a == "-v" || a == "--verbose");
    let corpus_dir = corpus_dir();
    let files = require_corpus();

    let expectations = match Expectations::load_for(&files) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2);
        }
    };

    let mut validator = match load_schema() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("cannot load Schema/OpenSCENARIO.xsd: {e}");
            std::process::exit(2);
        }
    };

    let schema_text = match std::fs::read_to_string(schema_path()) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("cannot read {}: {e}", schema_path().display());
            std::process::exit(2);
        }
    };
    let xsd = match Xsd::parse(&schema_text) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("cannot index {}: {e}", schema_path().display());
            std::process::exit(2);
        }
    };

    // --- pass one: choose the sources and harvest donor content -----------------------------
    //
    // A mutation source has to be schema-valid to begin with, or "the schema rejects the mutant"
    // says nothing about the mutation. That is the same question `validate-input` asks, asked here
    // directly rather than read out of the manifest, so the two cannot drift apart.
    let mut sources: Vec<(std::path::PathBuf, String, Element, bool)> = Vec::new();
    let mut donors = Donors::default();
    let mut not_valid_input = 0usize;
    let mut unreadable: Vec<String> = Vec::new();
    let mut controls_refused: Vec<String> = Vec::new();
    let mut unresolved_total = 0usize;

    for path in &files {
        let rel = path.strip_prefix(&corpus_dir).unwrap_or(path);
        let name = rel.display().to_string();
        let text = match std::fs::read_to_string(path) {
            Ok(t) => t,
            Err(e) => {
                unreadable.push(format!("{name}: {e}"));
                continue;
            }
        };
        match schema_rejects(&mut validator, &text) {
            Ok(true) => {
                not_valid_input += 1;
                continue;
            }
            Ok(false) => {}
            Err(e) => {
                unreadable.push(format!("{name}: the oracle could not judge the input: {e}"));
                continue;
            }
        }
        let root = match Element::parse(&text) {
            Ok(r) => r,
            Err(e) => {
                unreadable.push(format!("{name}: {e}"));
                continue;
            }
        };
        let catalog = is_catalog(rel);

        // The control: a schema-valid document the crate must accept. Over-strictness is a defect
        // in the other direction, and a file the crate cannot read at all would make every mutant
        // of it "crate-rejected" for a reason that has nothing to do with the mutation.
        if let Err(e) = crate_parses_file(path, catalog) {
            controls_refused.push(format!("{name}: {e}"));
            continue;
        }

        let (sites, unresolved) = resolve(&xsd, &root);
        unresolved_total += unresolved;
        donors.harvest(&xsd, &root, &sites);
        sources.push((path.clone(), name, root, catalog));
    }

    // --- pass two: mutate, judge, compare ---------------------------------------------------
    let mut tallies: BTreeMap<Kind, Tally> = BTreeMap::new();
    let mut holes: Vec<Hole> = Vec::new();
    let mut over_strict: Vec<String> = Vec::new();
    let mut oracle_failures: Vec<String> = Vec::new();
    // Which XSD types were mutated at all. A hole count is only as wide as this number: a type no
    // mutant ever touched is untested, not sound.
    let mut types_mutated: BTreeSet<String> = BTreeSet::new();

    for (_, name, root, catalog) in &sources {
        let (sites, _) = resolve(&xsd, root);
        let seed = SEED ^ fnv1a(name);
        for mutant in mutants(&xsd, &donors, root, &sites, seed, CAP) {
            let tally = tallies.entry(mutant.kind).or_default();
            tally.generated += 1;
            types_mutated.insert(mutant.type_name.clone());
            let rejected = match schema_rejects(&mut validator, &mutant.xml) {
                Ok(r) => r,
                Err(e) => {
                    oracle_failures.push(format!("{name}: {} {}: {e}", mutant.kind, mutant.detail));
                    continue;
                }
            };
            let parsed = crate_parses(&mutant.xml, *catalog).is_ok();
            if !rejected {
                // The mutation left a valid document: discarded as a test of refusal, but still
                // worth one question, because the crate must accept it.
                tally.still_valid += 1;
                if !parsed {
                    tally.over_strict += 1;
                    over_strict.push(format!(
                        "{name}: {} at {} ({}) stays schema-valid, but the crate refuses it",
                        mutant.kind, mutant.element_path, mutant.detail
                    ));
                }
                continue;
            }
            tally.schema_rejected += 1;
            if parsed {
                tally.holes += 1;
                holes.push(Hole {
                    kind: mutant.kind,
                    type_name: mutant.type_name.clone(),
                    element_path: mutant.element_path.clone(),
                    detail: mutant.detail.clone(),
                    file: name.clone(),
                });
            } else {
                tally.crate_rejected += 1;
                if verbose {
                    println!(
                        "OK    {name}  {} at {} ({})",
                        mutant.kind, mutant.element_path, mutant.detail
                    );
                }
            }
        }
    }

    // --- holes, grouped the way an issue would be written ------------------------------------
    let mut by_class: BTreeMap<(Kind, String), Vec<&Hole>> = BTreeMap::new();
    for hole in &holes {
        by_class
            .entry((hole.kind, hole.type_name.clone()))
            .or_default()
            .push(hole);
    }

    let mut expected_classes = 0usize;
    let mut unexpected: Vec<String> = Vec::new();
    let mut expected_lines: Vec<String> = Vec::new();
    for ((kind, type_name), group) in &by_class {
        let head = group[0];
        // The distinct edits, because "57 holes" reads as 57 problems when it is often one
        // constraint met in 57 files. The count that matters for an issue is this one.
        let details: BTreeSet<&str> = group.iter().map(|h| h.detail.as_str()).collect();
        let files: BTreeSet<&str> = group.iter().map(|h| h.file.as_str()).collect();
        let line = format!(
            "{kind} / {type_name}  x{} in {} file(s), {} distinct edit(s): {}\n          e.g. {} at {}",
            group.len(),
            files.len(),
            details.len(),
            summarize(&details),
            head.file,
            head.element_path
        );
        match expectations.hole(kind.as_str(), type_name) {
            Some(_) => {
                expected_classes += 1;
                expected_lines.push(line);
            }
            None => unexpected.push(line),
        }
    }

    // The manifest's per-file entries belong to the other gates, but their premises are checked
    // by every binary that reads the manifest: a premise that has stopped being true has to be
    // noticed by somebody, and which process notices it is not the point.
    let broken_premises = expectations.check_premises(&mut validator);

    let stale: Vec<String> = expectations
        .holes()
        .iter()
        .filter(|h| {
            !by_class
                .keys()
                .any(|(kind, type_name)| kind.as_str() == h.kind && type_name == &h.type_name)
        })
        .map(stale_hole_message)
        .collect();

    // --- the report --------------------------------------------------------------------------
    let (types, enums) = xsd.sizes();
    println!(
        "schema: {types} complex types, {enums} enumerated simple types; donor content for {} types",
        donors.len()
    );
    println!(
        "sources: {} of {} corpus files ({not_valid_input} are not schema-valid input, {} unreadable, {} controls refused)",
        sources.len(),
        files.len(),
        unreadable.len(),
        controls_refused.len()
    );
    println!("cap: {CAP} mutants per kind per file, seed {SEED:#x}");
    if unresolved_total > 0 {
        println!("{unresolved_total} document elements the schema does not declare in that position (not mutated)");
    }

    println!("\n--- per mutation kind ---");
    println!(
        "{:<20} {:>9} {:>11} {:>15} {:>14} {:>6} {:>11}",
        "kind",
        "generated",
        "still-valid",
        "schema-rejected",
        "crate-rejected",
        "holes",
        "over-strict"
    );
    for kind in Kind::ALL {
        let t = tallies.get(&kind).copied().unwrap_or_default();
        println!(
            "{:<20} {:>9} {:>11} {:>15} {:>14} {:>6} {:>11}",
            kind.as_str(),
            t.generated,
            t.still_valid,
            t.schema_rejected,
            t.crate_rejected,
            t.holes,
            t.over_strict
        );
    }

    println!("\n--- summary ---");
    let generated: usize = tallies.values().map(|t| t.generated).sum();
    let schema_rejected: usize = tallies.values().map(|t| t.schema_rejected).sum();
    let crate_rejected: usize = tallies.values().map(|t| t.crate_rejected).sum();
    println!(
        "{generated} mutants over {} XSD types, {schema_rejected} schema-rejected, \
         {crate_rejected} crate-rejected, {} holes in {} classes ({expected_classes} expected), \
         {} controls refused",
        types_mutated.len(),
        holes.len(),
        by_class.len(),
        controls_refused.len()
    );

    if !expected_lines.is_empty() {
        println!(
            "\n{} hole class(es) recorded in conformance/expectations.toml:",
            expected_lines.len()
        );
        for line in &expected_lines {
            println!("  XHOLE {line}");
        }
    }
    if !unexpected.is_empty() {
        println!("\n{} hole class(es) with no entry:", unexpected.len());
        for line in &unexpected {
            println!("  HOLE  {line}");
        }
    }
    for (label, items) in [
        ("controls refused", &controls_refused),
        ("schema-valid mutants refused", &over_strict),
        ("unreadable sources", &unreadable),
        ("oracle failures", &oracle_failures),
        ("stale hole entries", &stale),
        ("broken expectation(s)", &broken_premises),
    ] {
        if !items.is_empty() {
            println!("\n{} {label}:", items.len());
            for item in items.iter() {
                println!("  {item}");
            }
        }
    }

    // Same exit convention as the other gates: non-zero so a caller can gate on `$?` rather than
    // scrape a summary line. Generating nothing is a failure too — a gate that examined nothing
    // reporting success is the false green this harness exists to prevent.
    let failed = !unexpected.is_empty()
        || !controls_refused.is_empty()
        || !over_strict.is_empty()
        || !unreadable.is_empty()
        || !oracle_failures.is_empty()
        || !stale.is_empty()
        || !broken_premises.is_empty()
        || generated == 0;
    if failed {
        std::process::exit(1);
    }
}
