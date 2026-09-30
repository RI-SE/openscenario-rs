//! Regression tests for `XsdDateTime`'s lexical fidelity and `TimeOfDay::date_time`'s type.
//!
//! `XsdDateTime`'s `Display` used to re-derive a lexical form from the parsed value rather
//! than reproducing the text that was read, so `+00:00` became `Z`, `.5Z` became `.500Z`, and
//! `.25` became `.250` -- each a different, still schema-valid, written form of the same
//! instant. A timezone-less form used to be refused outright. These tests parse each row
//! through the actual XML path (`TimeOfDayCondition`) and assert the serialized bytes equal
//! the source, since a fixed point alone does not prove a round trip.

use openscenario_rs::types::conditions::value::TimeOfDayCondition;
use openscenario_rs::types::environment::TimeOfDay;

/// Each row is one written form of an `xsd:dateTime`; each must parse through the XML path
/// and serialize back byte-identical. Every row is measured before the test fails.
#[test]
fn date_time_spellings_round_trip_byte_identical() {
    let rows = [
        // `+00:00` and `Z` name the same offset; each stays as written.
        "2020-06-16T10:00:00+00:00",
        "2020-06-16T10:00:00Z",
        // `.5` and `.25` are not padded to `.500` / `.250`.
        "2020-06-16T10:00:00.5Z",
        "2020-06-16T10:00:00.25",
        // No timezone: schema-valid `xsd:dateTime`, which RFC 3339 (and `chrono`'s
        // `DateTime<Utc>::from_str`) refuses; it must parse, and stay offset-less.
        "2020-06-16T10:00:00",
    ];
    let failures: Vec<String> = rows
        .iter()
        .filter_map(|date_time| {
            let xml = format!(r#"<TimeOfDayCondition dateTime="{date_time}" rule="greaterThan"/>"#);
            let condition: TimeOfDayCondition = match quick_xml::de::from_str(&xml) {
                Ok(condition) => condition,
                Err(e) => return Some(format!("{date_time}: failed to parse: {e}")),
            };
            let serialized =
                quick_xml::se::to_string_with_root("TimeOfDayCondition", &condition).unwrap();
            (serialized != xml).then(|| format!("{date_time}: serialized as {serialized}"))
        })
        .collect();
    assert!(failures.is_empty(), "{failures:#?}");
}

#[test]
fn time_of_day_date_time_rejects_a_non_date() {
    let err = quick_xml::de::from_str::<TimeOfDay>(
        r#"<TimeOfDay animation="true" dateTime="not-a-date"/>"#,
    )
    .expect_err("`not-a-date` is not in xsd:dateTime's lexical space and must be refused");
    assert!(
        err.to_string().starts_with("Failed to parse 'not-a-date':"),
        "the error must name the refused text, got: {err}"
    );
}

#[test]
fn time_of_day_date_time_accepts_a_parameter_reference() {
    let time_of_day: TimeOfDay =
        quick_xml::de::from_str(r#"<TimeOfDay animation="true" dateTime="$myDateTime"/>"#)
            .expect("a `$param` reference must be accepted on a typed dateTime attribute");
    assert_eq!(time_of_day.date_time.as_parameter(), Some("myDateTime"));
}
