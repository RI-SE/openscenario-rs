//! Regression tests for the XSD lexical spaces of typed `bool` and `dateTime` attributes.
//!
//! `bool::from_str` and `chrono::DateTime<Utc>::from_str` are both narrower than the XSD
//! lexical spaces they stand in for: `xsd:boolean` accepts `0`/`1` beside `true`/`false`
//! (ASAM OpenSCENARIO section 9.2.2), and `xsd:dateTime` makes the timezone offset optional.
//! `ParameterScope` already checks declarations against those wider spaces (section 9.1); these
//! tests probe the typed attribute path that reads a value back out afterward, through the
//! actual XML parse path rather than a hand-built value.

use openscenario_rs::types::basic::{Value, XsdDateTime};
use openscenario_rs::types::conditions::value::TimeOfDayCondition;
use openscenario_rs::types::enums::Rule;
use openscenario_rs::types::environment::TimeOfDay;
use openscenario_rs::types::scope::ParameterScope;

#[test]
fn boolean_attribute_accepts_numeric_literal_one() {
    let time_of_day: TimeOfDay =
        quick_xml::de::from_str(r#"<TimeOfDay animation="1" dateTime="2020-01-01T00:00:00"/>"#)
            .expect("`1` is in xsd:boolean's lexical space (section 9.2.2) and must parse");
    assert_eq!(time_of_day.animation, Value::Literal(true));
}

#[test]
fn boolean_attribute_accepts_numeric_literal_zero() {
    let time_of_day: TimeOfDay =
        quick_xml::de::from_str(r#"<TimeOfDay animation="0" dateTime="2020-01-01T00:00:00"/>"#)
            .expect("`0` is in xsd:boolean's lexical space (section 9.2.2) and must parse");
    assert_eq!(time_of_day.animation, Value::Literal(false));
}

#[test]
fn timezoneless_datetime_parses_and_round_trips() {
    let condition: TimeOfDayCondition = quick_xml::de::from_str(
        r#"<TimeOfDayCondition dateTime="2020-06-16T10:00:00" rule="greaterThan"/>"#,
    )
    .expect("a dateTime with no timezone is schema-valid xsd:dateTime and must parse");
    assert_eq!(condition.rule, Value::Literal(Rule::GreaterThan));
    // Probe through the parse path rather than hand-building the expected value: `XsdDateTime`
    // keeps the text it was parsed from (OSS-63), so a hand-built value carries none and would
    // not compare equal to one read from XML even when it names the same instant.
    let expected: XsdDateTime = "2020-06-16T10:00:00".parse().unwrap();
    assert_eq!(condition.date_time, Value::Literal(expected));

    let serialized = quick_xml::se::to_string_with_root("TimeOfDayCondition", &condition).unwrap();
    assert_eq!(
        serialized, r#"<TimeOfDayCondition dateTime="2020-06-16T10:00:00" rule="greaterThan"/>"#,
        "a timezone-less dateTime must serialize back byte-identical"
    );
}

#[test]
fn declared_numeric_boolean_resolves_into_a_typed_bool_field() {
    // The scenario OSS-58 names directly: `ParameterScope` already accepts a declared `1` as
    // a valid Boolean (section 9.2.2), so a document declaring one must also resolve through to
    // a typed `bool` -- not merely pass the declaration-time check and then fail on read.
    let mut scope = ParameterScope::new();
    scope
        .declare("flag", "boolean", "1")
        .expect("`1` is a valid Boolean literal (section 9.2.2) and must be accepted");

    let value: Value<bool> = Value::Parameter("flag".to_string());
    let resolved = scope
        .resolve(&value)
        .expect("a declared Boolean `1` must resolve, not fail to parse");
    assert!(resolved, "a declared Boolean `1` must resolve to `true`");
}

#[test]
fn timezone_aware_datetime_still_parses_and_round_trips() {
    let condition: TimeOfDayCondition = quick_xml::de::from_str(
        r#"<TimeOfDayCondition dateTime="2024-04-26T09:10:00Z" rule="lessThan"/>"#,
    )
    .expect("an RFC 3339 dateTime must still parse");
    let serialized = quick_xml::se::to_string_with_root("TimeOfDayCondition", &condition).unwrap();
    assert_eq!(
        serialized,
        r#"<TimeOfDayCondition dateTime="2024-04-26T09:10:00Z" rule="lessThan"/>"#
    );
}
