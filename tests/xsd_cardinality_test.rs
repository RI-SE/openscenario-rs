//! Cardinality and required-content rules that the XSD states but serde will
//! not enforce on its own.
//!
//! Each case here parses a document the schema settles one way and asserts the
//! crate agrees. Every assertion is its own test, because a test that checks
//! several documents in sequence stops at the first failure and never measures
//! the later claims.

use openscenario_rs::types::actions::control::{
    OverrideBrakeAction, OverrideGearAction, OverrideParkingBrakeAction,
};
use openscenario_rs::types::catalogs::entities::CatalogManeuver;
use openscenario_rs::types::distributions::UserDefinedDistribution;

// ---------------------------------------------------------------------------
// The optional choice groups under the override actions.
//
// XSD `OverrideBrakeAction`, `OverrideGearAction` and `OverrideParkingBrakeAction`
// each wrap a choice group with `minOccurs="0"`. Zero branches is therefore
// valid and exactly one branch is valid, while two branches is not. Under
// `#[serde(flatten)]` the children were buffered into a map before being
// interpreted, and a map cannot report that a key occurred twice, so the
// two-branch document parsed and the second branch was dropped in silence.
// The one-branch round trips live in `choice_wrappers_roundtrip_test.rs`.
// ---------------------------------------------------------------------------

#[test]
fn override_brake_action_rejects_two_branches() {
    let xml = r#"<OverrideBrakeAction active="true"><BrakePercent value="0.5"/><BrakeForce value="9"/></OverrideBrakeAction>"#;
    let err = quick_xml::de::from_str::<OverrideBrakeAction>(xml)
        .expect_err("a document carrying both branches of the BrakeInput group was accepted");
    assert_eq!(err.to_string(), "duplicate field `$value`");
}

#[test]
fn override_gear_action_rejects_two_branches() {
    let xml = r#"<OverrideGearAction active="true"><ManualGear number="3"/><AutomaticGear gear="n"/></OverrideGearAction>"#;
    let err = quick_xml::de::from_str::<OverrideGearAction>(xml)
        .expect_err("a document carrying both branches of the Gear group was accepted");
    assert_eq!(err.to_string(), "duplicate field `$value`");
}

#[test]
fn override_parking_brake_action_rejects_two_branches() {
    let xml = r#"<OverrideParkingBrakeAction active="true"><BrakePercent value="0.5"/><BrakeForce value="9"/></OverrideParkingBrakeAction>"#;
    let err = quick_xml::de::from_str::<OverrideParkingBrakeAction>(xml)
        .expect_err("a document carrying both branches of the BrakeInput group was accepted");
    assert_eq!(err.to_string(), "duplicate field `$value`");
}

#[test]
fn override_brake_action_accepts_zero_branches() {
    let xml = r#"<OverrideBrakeAction active="true"/>"#;
    let parsed: OverrideBrakeAction =
        quick_xml::de::from_str(xml).expect("minOccurs=\"0\" group: zero branches is valid");
    assert_eq!(parsed.brake_input, None);
}

// ---------------------------------------------------------------------------
// The text body of a simpleContent extension.
//
// XSD `UserDefinedDistribution` extends `xsd:string`, which permits the empty
// string, so an element with no text body is valid.
// ---------------------------------------------------------------------------

#[test]
fn user_defined_distribution_accepts_an_absent_text_body() {
    let xml = r#"<UserDefinedDistribution type="t"/>"#;
    let parsed: UserDefinedDistribution =
        quick_xml::de::from_str(xml).expect("an empty text body is valid xsd:string");
    assert_eq!(parsed.content, "");
}

#[test]
fn user_defined_distribution_keeps_a_present_text_body() {
    let xml = r#"<UserDefinedDistribution type="t">body</UserDefinedDistribution>"#;
    let parsed: UserDefinedDistribution = quick_xml::de::from_str(xml).expect("parse");
    assert_eq!(parsed.content, "body");
}

// ---------------------------------------------------------------------------
// A repeated child that the schema requires at least once.
//
// XSD `Maneuver` declares `<Event>` with `maxOccurs="unbounded"` and no
// `minOccurs`, which defaults to one. A `<Maneuver>` with no `<Event>` is
// therefore invalid, and `#[serde(default)]` on the field would turn that
// document into an empty maneuver instead of an error.
// ---------------------------------------------------------------------------

#[test]
fn catalog_maneuver_rejects_a_document_with_no_event() {
    let xml = r#"<Maneuver name="m"/>"#;
    let err = quick_xml::de::from_str::<CatalogManeuver>(xml)
        .expect_err("a Maneuver carrying no Event was accepted");
    assert_eq!(err.to_string(), "missing field `Event`");
}
