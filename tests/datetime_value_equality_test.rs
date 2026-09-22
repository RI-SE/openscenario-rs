//! Regression tests for `XsdDateTime`'s equality and hashing.
//!
//! `PartialEq`, `Eq` and `Hash` used to be derived over the whole struct, including the
//! `source` text kept for lexical fidelity (see `datetime_lexical_fidelity_test.rs`). Two
//! spellings of the same instant, or a constructed value and the parsed value it came from,
//! then compared unequal even though they named the same value. These tests assert the value
//! comparison directly and, since the fix must not regress lexical fidelity, that each
//! spelling still serializes exactly as written.

use openscenario_rs::types::basic::XsdDateTime;
use openscenario_rs::types::conditions::value::TimeOfDayCondition;
use std::collections::HashSet;

#[test]
fn z_and_plus_zero_offset_compare_equal() {
    let z: XsdDateTime = "2020-06-16T10:00:00Z".parse().unwrap();
    let offset: XsdDateTime = "2020-06-16T10:00:00+00:00".parse().unwrap();
    assert_eq!(
        z, offset,
        "same instant, different spelling, must compare equal"
    );
}

#[test]
fn equal_spellings_still_serialize_as_written() {
    // The equality fix must not touch the lexical-fidelity guarantee OSS-63 built: each
    // spelling still round-trips through the real XML path byte-identical, even though the
    // two values now compare equal.
    for date_time in ["2020-06-16T10:00:00Z", "2020-06-16T10:00:00+00:00"] {
        let xml = format!(r#"<TimeOfDayCondition dateTime="{date_time}" rule="greaterThan"/>"#);
        let condition: TimeOfDayCondition =
            quick_xml::de::from_str(&xml).unwrap_or_else(|e| panic!("failed to parse {xml}: {e}"));
        let serialized =
            quick_xml::se::to_string_with_root("TimeOfDayCondition", &condition).unwrap();
        assert_eq!(
            serialized, xml,
            "dateTime={date_time} did not round-trip byte-identical"
        );
    }
}

#[test]
fn constructed_value_equals_the_parsed_value_it_came_from() {
    let parsed: XsdDateTime = "2020-06-16T10:00:00Z".parse().unwrap();
    let constructed = XsdDateTime::aware(parsed.as_aware().unwrap());
    assert_eq!(
        constructed, parsed,
        "constructed value must equal the parsed value it came from"
    );
}

#[test]
fn hash_agrees_with_partial_eq_for_two_spellings() {
    let z: XsdDateTime = "2020-06-16T10:00:00Z".parse().unwrap();
    let offset: XsdDateTime = "2020-06-16T10:00:00+00:00".parse().unwrap();
    let mut set = HashSet::new();
    set.insert(z);
    set.insert(offset);
    assert_eq!(
        set.len(),
        1,
        "a HashSet must hold one entry for the two spellings of one instant"
    );
}

#[test]
fn naive_and_aware_values_are_never_equal() {
    // A naive dateTime carries no offset, so it names no single instant; it is not the same
    // value as an aware one even when the clock reading matches.
    let naive: XsdDateTime = "2020-06-16T10:00:00".parse().unwrap();
    let aware: XsdDateTime = "2020-06-16T10:00:00Z".parse().unwrap();
    assert_ne!(
        naive, aware,
        "a naive dateTime must not compare equal to an aware one with the same clock reading"
    );
}
