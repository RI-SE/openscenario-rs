//! Regression tests for the XSD lexical spaces of typed `bool` and `dateTime` attributes.
//!
//! `bool::from_str` and `chrono::DateTime<Utc>::from_str` are both narrower than the XSD
//! lexical spaces they stand in for: `xsd:boolean` accepts `0`/`1` beside `true`/`false`
//! (ASAM OpenSCENARIO section 9.2.2), and `xsd:dateTime` makes the timezone offset optional.
//! `ParameterScope` already checks declarations against those wider spaces (section 9.1); these
//! tests probe the typed attribute path that reads a value back out afterward, through the
//! actual XML parse path rather than a hand-built value. The timezone-less `dateTime` row
//! lives in `datetime_lexical_fidelity_test.rs`.

use openscenario_rs::types::basic::Value;
use openscenario_rs::types::environment::TimeOfDay;
use openscenario_rs::types::scope::ParameterScope;

#[test]
fn boolean_attribute_accepts_numeric_literals() {
    for (text, expected) in [("1", true), ("0", false)] {
        let xml = format!(r#"<TimeOfDay animation="{text}" dateTime="2020-01-01T00:00:00"/>"#);
        let time_of_day: TimeOfDay = quick_xml::de::from_str(&xml).unwrap_or_else(|e| {
            panic!("`{text}` is in xsd:boolean's lexical space (section 9.2.2): {e}")
        });
        assert_eq!(time_of_day.animation, Value::Literal(expected), "`{text}`");
    }
}

#[test]
fn declared_numeric_boolean_resolves_into_a_typed_bool_field() {
    // `ParameterScope` accepts a declared `1` as a valid Boolean (section 9.2.2), so a
    // document declaring one must also resolve through to a typed `bool` -- not merely pass
    // the declaration-time check and then fail on read.
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
