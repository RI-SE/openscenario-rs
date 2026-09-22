//! Character content the crate drops must be visible to `lossy`.
//!
//! `tests/data/loose_text_in_parameter_declarations.xosc` is a hand-built eighteen-line
//! reproduction of an upstream typo in
//! `corpus/openscenario1-engine/engine/tests/data/Scenarios/trajectory_shape.xosc`: loose text
//! inside `<ParameterDeclarations>`, whose XSD content type is element-only.
//!
//! All three assertions below verify facts about the crate. Initially `profile()` discarded
//! `Event::Text`, so the first two tests would fail against an older comparator. The third is
//! the defect these tests verify: an invalid input is turned into a valid document. That is not
//! a bug to fix here, it is the fact `lossy` and the `validate-input` gate now make visible
//! instead of swallowing.
//!
//! The fixture deliberately does not live in `corpus/`, which is gitignored third-party content
//! fetched on demand; a regression test may not depend on a network fetch.

use openscenario_roundtrip_harness::load_schema;
use openscenario_roundtrip_harness::xml_profile::{diff, profile};
use openscenario_rs::{parse_from_str, serialize_to_string};
use std::path::PathBuf;

/// The key `profile()` emits for the one stray text node in the fixture.
const TEXT_KEY: &str = "OpenSCENARIO/ParameterDeclarations#text";

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/loose_text_in_parameter_declarations.xosc")
}

fn fixture() -> String {
    std::fs::read_to_string(fixture_path()).expect("the regression fixture is committed")
}

fn serialized_fixture() -> String {
    let doc = parse_from_str(&fixture()).expect("the fixture parses; only its stray text is bad");
    serialize_to_string(&doc).expect("a parsed document serializes")
}

/// `profile()` sees character content at all. `Event::Text` now yields a `#text` key instead
/// of falling into a catch-all, so this key exists.
#[test]
fn profile_sees_loose_character_content() {
    let counts = profile(&fixture()).expect("the fixture is well-formed XML");
    assert_eq!(
        counts.get(TEXT_KEY).copied(),
        Some(1),
        "profile() did not record the stray text node under <ParameterDeclarations>; \
         keys it did record under that path: {:?}",
        counts
            .keys()
            .filter(|k| k.starts_with("OpenSCENARIO/ParameterDeclarations"))
            .collect::<Vec<_>>()
    );
}

/// The comparator `lossy` runs reports the dropped text. Before it could see character
/// content, a transformation that discarded content was certified as having discarded
/// nothing.
#[test]
fn the_dropped_text_is_reported_as_dropped() {
    let before = profile(&fixture()).expect("the fixture is well-formed XML");
    let after = profile(&serialized_fixture()).expect("the crate emits well-formed XML");
    let d = diff(&before, &after);

    assert_eq!(
        d.dropped,
        vec![(TEXT_KEY.to_string(), 1)],
        "expected exactly the stray text node to be reported as dropped"
    );
    assert!(
        d.invented.is_empty(),
        "nothing should be invented: {:?}",
        d.invented
    );
    assert_eq!(d.dropped_count(), 1);
}

/// The hole the `validate-input` gate exists for: the input is schema-invalid, the output is
/// schema-valid, and the difference is content that went missing. Asserting both halves keeps the
/// two gates honest about which question each one answers.
#[test]
fn an_invalid_input_becomes_a_valid_output() {
    let mut validator = load_schema().expect("Schema/OpenSCENARIO.xsd");

    let input_errors = validator
        .validate_file(fixture_path())
        .expect("the fixture is well-formed, so validation returns a verdict");
    assert_eq!(
        input_errors.len(),
        1,
        "the fixture must be invalid for exactly one reason: {input_errors:?}"
    );
    assert!(
        input_errors[0]
            .message
            .contains("Character content other than whitespace is not allowed"),
        "the one reason must be the loose text, not an unrelated drift in the fixture: {}",
        input_errors[0].message
    );

    let output_errors = validator
        .validate_str(&serialized_fixture())
        .expect("the crate emits well-formed XML");
    assert!(
        output_errors.is_empty(),
        "the crate is expected to quietly produce a VALID document from this INVALID input — \
         that is the whole point of the fixture. If this now fails, the crate changed and the \
         test expectations need rewriting rather than this assertion loosening: {output_errors:?}"
    );
}
