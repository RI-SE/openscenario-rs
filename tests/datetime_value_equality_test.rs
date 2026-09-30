//! Regression tests for `XsdDateTime`'s equality and hashing.
//!
//! `PartialEq`, `Eq` and `Hash` used to be derived over the whole struct, including the
//! `source` text kept for lexical fidelity (see `datetime_lexical_fidelity_test.rs`). Two
//! spellings of the same instant, or a constructed value and the parsed value it came from,
//! then compared unequal even though they named the same value. These tests assert the value
//! comparison directly; that each spelling still serializes exactly as written is
//! `datetime_lexical_fidelity_test.rs`'s table.

use openscenario_rs::types::basic::XsdDateTime;
use std::collections::HashSet;

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
fn two_spellings_of_one_instant_are_equal_and_hash_alike() {
    let z: XsdDateTime = "2020-06-16T10:00:00Z".parse().unwrap();
    let offset: XsdDateTime = "2020-06-16T10:00:00+00:00".parse().unwrap();
    assert_eq!(
        z, offset,
        "same instant, different spelling, must compare equal"
    );
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
