//! Cardinality and required-content rules that the XSD states but serde will
//! not enforce on its own.
//!
//! Each case here parses a document the schema settles one way and asserts the
//! crate agrees. Every assertion is its own test, because a test that checks
//! several documents in sequence stops at the first failure and never measures
//! the later claims.

use openscenario_rs::types::actions::control::{
    BrakeInput, Gear, OverrideBrakeAction, OverrideGearAction, OverrideParkingBrakeAction,
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
// ---------------------------------------------------------------------------

const BRAKE_ONE: &str =
    r#"<OverrideBrakeAction active="true"><BrakePercent value="0.5"/></OverrideBrakeAction>"#;
const GEAR_ONE: &str =
    r#"<OverrideGearAction active="true"><ManualGear number="3"/></OverrideGearAction>"#;
const PARKING_ONE: &str = r#"<OverrideParkingBrakeAction active="true"><BrakeForce value="100"/></OverrideParkingBrakeAction>"#;

#[test]
fn override_brake_action_rejects_two_branches() {
    let xml = r#"<OverrideBrakeAction active="true"><BrakePercent value="0.5"/><BrakeForce value="9"/></OverrideBrakeAction>"#;
    let parsed: Result<OverrideBrakeAction, _> = quick_xml::de::from_str(xml);
    assert!(
        parsed.is_err(),
        "a document carrying both branches of the BrakeInput group was accepted: {:?}",
        parsed
    );
}

#[test]
fn override_gear_action_rejects_two_branches() {
    let xml = r#"<OverrideGearAction active="true"><ManualGear number="3"/><AutomaticGear gear="n"/></OverrideGearAction>"#;
    let parsed: Result<OverrideGearAction, _> = quick_xml::de::from_str(xml);
    assert!(
        parsed.is_err(),
        "a document carrying both branches of the Gear group was accepted: {:?}",
        parsed
    );
}

#[test]
fn override_parking_brake_action_rejects_two_branches() {
    let xml = r#"<OverrideParkingBrakeAction active="true"><BrakePercent value="0.5"/><BrakeForce value="9"/></OverrideParkingBrakeAction>"#;
    let parsed: Result<OverrideParkingBrakeAction, _> = quick_xml::de::from_str(xml);
    assert!(
        parsed.is_err(),
        "a document carrying both branches of the BrakeInput group was accepted: {:?}",
        parsed
    );
}

#[test]
fn override_brake_action_accepts_zero_branches() {
    let xml = r#"<OverrideBrakeAction active="true"/>"#;
    let parsed: Result<OverrideBrakeAction, _> = quick_xml::de::from_str(xml);
    assert!(parsed.is_ok(), "minOccurs=\"0\" group: {:?}", parsed);
}

#[test]
fn override_brake_action_keeps_its_branch() {
    let parsed: OverrideBrakeAction = quick_xml::de::from_str(BRAKE_ONE).expect("parse");
    assert!(
        matches!(parsed.brake_input, Some(BrakeInput::BrakePercent(_))),
        "branch lost: {:?}",
        parsed.brake_input
    );
}

#[test]
fn override_gear_action_keeps_its_branch() {
    let parsed: OverrideGearAction = quick_xml::de::from_str(GEAR_ONE).expect("parse");
    assert!(
        matches!(parsed.gear, Some(Gear::ManualGear(_))),
        "branch lost: {:?}",
        parsed.gear
    );
}

#[test]
fn override_parking_brake_action_keeps_its_branch() {
    let parsed: OverrideParkingBrakeAction = quick_xml::de::from_str(PARKING_ONE).expect("parse");
    assert!(
        matches!(parsed.brake_input, Some(BrakeInput::BrakeForce(_))),
        "branch lost: {:?}",
        parsed.brake_input
    );
}

#[test]
fn override_brake_action_serializes_back_to_its_source() {
    let parsed: OverrideBrakeAction = quick_xml::de::from_str(BRAKE_ONE).expect("parse");
    let out =
        quick_xml::se::to_string_with_root("OverrideBrakeAction", &parsed).expect("serialize");
    assert_eq!(out, BRAKE_ONE);
}

#[test]
fn override_gear_action_serializes_back_to_its_source() {
    let parsed: OverrideGearAction = quick_xml::de::from_str(GEAR_ONE).expect("parse");
    let out = quick_xml::se::to_string_with_root("OverrideGearAction", &parsed).expect("serialize");
    assert_eq!(out, GEAR_ONE);
}

#[test]
fn override_parking_brake_action_serializes_back_to_its_source() {
    let parsed: OverrideParkingBrakeAction = quick_xml::de::from_str(PARKING_ONE).expect("parse");
    let out = quick_xml::se::to_string_with_root("OverrideParkingBrakeAction", &parsed)
        .expect("serialize");
    assert_eq!(out, PARKING_ONE);
}

#[test]
fn override_brake_action_keeps_its_sibling_attribute() {
    let parsed: OverrideBrakeAction = quick_xml::de::from_str(BRAKE_ONE).expect("parse");
    assert_eq!(parsed.active.as_literal(), Some(&true));
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
    let parsed: Result<UserDefinedDistribution, _> = quick_xml::de::from_str(xml);
    assert!(parsed.is_ok(), "empty text body rejected: {:?}", parsed);
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
    let parsed: Result<CatalogManeuver, _> = quick_xml::de::from_str(xml);
    assert!(
        parsed.is_err(),
        "a Maneuver carrying no Event was accepted: {:?}",
        parsed
    );
}
