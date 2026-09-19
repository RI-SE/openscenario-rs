//! Cardinality tests for the appearance choice groups.
//!
//! `AppearanceAction`, `LightType`, `Color`, `AnimationType` and
//! `ComponentAnimation` were each modelled as a struct of parallel `Option`
//! fields, one per XSD choice branch. That shape describes `xsd:all` with
//! optional members, not `xsd:choice`: it accepts zero branches, and it
//! accepts two branches and keeps both, producing a value no schema-valid
//! document can express. Each is now a single field named `$value` holding
//! an externally tagged enum, which takes the branch's element name from the
//! serialized variant. Serde then enforces the choice cardinality
//! structurally: zero branches is `missing field `$value`` and two branches
//! is `duplicate field `$value``.

use openscenario_rs::types::actions::appearance::{
    AnimationType, AppearanceAction, Color, ComponentAnimation, LightType,
};

fn de<T: serde::de::DeserializeOwned>(xml: &str) -> T {
    quick_xml::de::from_str(xml).unwrap_or_else(|e| panic!("deserialize failed for {xml}: {e}"))
}

fn de_err<T: serde::de::DeserializeOwned>(xml: &str) -> String {
    match quick_xml::de::from_str::<T>(xml) {
        Ok(_) => panic!("expected deserialize to fail for {xml}"),
        Err(e) => e.to_string(),
    }
}

fn round_trip<T>(xml: &str)
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    let parsed: T = de(xml);
    let emitted = quick_xml::se::to_string(&parsed).expect("serialize failed");
    assert_eq!(emitted.as_bytes(), xml.as_bytes());
}

// ── AppearanceAction (XSD `AppearanceAction`, :765-770) ──────────────────────

const LIGHT_STATE_BRANCH: &str = concat!(
    r#"<LightStateAction><LightType><VehicleLight vehicleLightType="lowBeam"/></LightType>"#,
    r#"<LightState mode="on"/></LightStateAction>"#
);

const ANIMATION_BRANCH: &str = concat!(
    r#"<AnimationAction><AnimationType><ComponentAnimation>"#,
    r#"<VehicleComponent vehicleComponentType="doorFrontLeft"/></ComponentAnimation>"#,
    r#"</AnimationType></AnimationAction>"#
);

#[test]
fn appearance_action_parses_light_state_branch() {
    let xml = format!("<AppearanceAction>{LIGHT_STATE_BRANCH}</AppearanceAction>");
    round_trip::<AppearanceAction>(&xml);
}

#[test]
fn appearance_action_rejects_zero_branches() {
    let err = de_err::<AppearanceAction>("<AppearanceAction/>");
    assert!(
        err.contains("missing field `$value`"),
        "unexpected error: {err}"
    );
}

#[test]
fn appearance_action_rejects_two_branches() {
    let xml =
        format!("<AppearanceAction>{LIGHT_STATE_BRANCH}{ANIMATION_BRANCH}</AppearanceAction>");
    let err = de_err::<AppearanceAction>(&xml);
    assert!(
        err.contains("duplicate field `$value`"),
        "unexpected error: {err}"
    );
}

// ── LightType (XSD `LightType`, :1418-1423) ──────────────────────────────────

#[test]
fn light_type_parses_vehicle_light_branch() {
    let xml = r#"<LightType><VehicleLight vehicleLightType="lowBeam"/></LightType>"#;
    round_trip::<LightType>(xml);
}

#[test]
fn light_type_rejects_zero_branches() {
    let err = de_err::<LightType>("<LightType/>");
    assert!(
        err.contains("missing field `$value`"),
        "unexpected error: {err}"
    );
}

#[test]
fn light_type_rejects_two_branches() {
    let xml = r#"<LightType><VehicleLight vehicleLightType="lowBeam"/><UserDefinedLight userDefinedLightType="halo"/></LightType>"#;
    let err = de_err::<LightType>(xml);
    assert!(
        err.contains("duplicate field `$value`"),
        "unexpected error: {err}"
    );
}

// ── Color (XSD `Color`, :929-935) ────────────────────────────────────────────

#[test]
fn color_parses_rgb_branch_alongside_required_attribute() {
    let xml = r#"<Color colorType="red"><ColorRgb red="1" green="0" blue="0"/></Color>"#;
    round_trip::<Color>(xml);
}

#[test]
fn color_rejects_zero_branches() {
    // `@colorType` is present; only the choice element is missing.
    let err = de_err::<Color>(r#"<Color colorType="red"/>"#);
    assert!(
        err.contains("missing field `$value`"),
        "unexpected error: {err}"
    );
}

#[test]
fn color_rejects_two_branches() {
    let xml = r#"<Color colorType="red"><ColorRgb red="1" green="0" blue="0"/><ColorCmyk cyan="0" magenta="0" yellow="0" key="1"/></Color>"#;
    let err = de_err::<Color>(xml);
    assert!(
        err.contains("duplicate field `$value`"),
        "unexpected error: {err}"
    );
}

// ── AnimationType (XSD `AnimationType`, :757-763) ────────────────────────────

const COMPONENT_ANIMATION_BRANCH: &str = concat!(
    r#"<ComponentAnimation><VehicleComponent vehicleComponentType="doorFrontLeft"/>"#,
    r#"</ComponentAnimation>"#
);

#[test]
fn animation_type_parses_component_animation_branch() {
    let xml = format!("<AnimationType>{COMPONENT_ANIMATION_BRANCH}</AnimationType>");
    round_trip::<AnimationType>(&xml);
}

#[test]
fn animation_type_rejects_zero_branches() {
    let err = de_err::<AnimationType>("<AnimationType/>");
    assert!(
        err.contains("missing field `$value`"),
        "unexpected error: {err}"
    );
}

#[test]
fn animation_type_rejects_two_branches() {
    let xml = format!(
        "<AnimationType>{COMPONENT_ANIMATION_BRANCH}<UserDefinedAnimation userDefinedAnimationType=\"custom\"/></AnimationType>"
    );
    let err = de_err::<AnimationType>(&xml);
    assert!(
        err.contains("duplicate field `$value`"),
        "unexpected error: {err}"
    );
}

// ── ComponentAnimation (XSD `ComponentAnimation`, :947-952) ──────────────────

#[test]
fn component_animation_parses_vehicle_component_branch() {
    let xml = r#"<ComponentAnimation><VehicleComponent vehicleComponentType="doorFrontLeft"/></ComponentAnimation>"#;
    round_trip::<ComponentAnimation>(xml);
}

#[test]
fn component_animation_rejects_zero_branches() {
    let err = de_err::<ComponentAnimation>("<ComponentAnimation/>");
    assert!(
        err.contains("missing field `$value`"),
        "unexpected error: {err}"
    );
}

#[test]
fn component_animation_rejects_two_branches() {
    let xml = r#"<ComponentAnimation><VehicleComponent vehicleComponentType="doorFrontLeft"/><UserDefinedComponent userDefinedComponentType="custom"/></ComponentAnimation>"#;
    let err = de_err::<ComponentAnimation>(xml);
    assert!(
        err.contains("duplicate field `$value`"),
        "unexpected error: {err}"
    );
}
