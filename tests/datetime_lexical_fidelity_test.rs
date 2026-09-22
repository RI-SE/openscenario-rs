//! Regression tests for `XsdDateTime`'s lexical fidelity and `TimeOfDay::date_time`'s type.
//!
//! `XsdDateTime`'s `Display` used to re-derive a lexical form from the parsed value rather
//! than reproducing the text that was read, so `+00:00` became `Z`, `.5Z` became `.500Z`, and
//! `.25` became `.250` -- each a different, still schema-valid, written form of the same
//! instant. These tests parse each row through the actual XML path (`TimeOfDayCondition`) and
//! assert the serialized bytes equal the source, since a fixed point alone does not prove a
//! round trip.

use openscenario_rs::types::conditions::value::TimeOfDayCondition;
use openscenario_rs::types::environment::TimeOfDay;

fn round_trips_byte_identical(date_time: &str) {
    let xml = format!(r#"<TimeOfDayCondition dateTime="{date_time}" rule="greaterThan"/>"#);
    let condition: TimeOfDayCondition =
        quick_xml::de::from_str(&xml).unwrap_or_else(|e| panic!("failed to parse {xml}: {e}"));
    let serialized = quick_xml::se::to_string_with_root("TimeOfDayCondition", &condition).unwrap();
    assert_eq!(
        serialized, xml,
        "dateTime={date_time} did not round-trip byte-identical"
    );
}

#[test]
fn offset_plus_zero_survives_as_written() {
    round_trips_byte_identical("2020-06-16T10:00:00+00:00");
}

#[test]
fn fractional_second_with_z_offset_survives_as_written() {
    round_trips_byte_identical("2020-06-16T10:00:00.5Z");
}

#[test]
fn fractional_second_without_offset_survives_as_written() {
    round_trips_byte_identical("2020-06-16T10:00:00.25");
}

#[test]
fn time_of_day_date_time_rejects_a_non_date() {
    let result: Result<TimeOfDay, _> =
        quick_xml::de::from_str(r#"<TimeOfDay animation="true" dateTime="not-a-date"/>"#);
    assert!(
        result.is_err(),
        "`not-a-date` is not in xsd:dateTime's lexical space and must be refused"
    );
}

#[test]
fn time_of_day_date_time_accepts_a_parameter_reference() {
    let time_of_day: TimeOfDay =
        quick_xml::de::from_str(r#"<TimeOfDay animation="true" dateTime="$myDateTime"/>"#)
            .expect("a `$param` reference must be accepted on a typed dateTime attribute");
    assert_eq!(time_of_day.date_time.as_parameter(), Some("myDateTime"));
}
