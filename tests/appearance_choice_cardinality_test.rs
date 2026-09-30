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
    AnimationAction, AnimationFile, AnimationState, AnimationType, AppearanceAction, Color,
    ColorChoice, ColorRgb, ComponentAnimation, ComponentAnimationChoice, LightState,
    LightStateAction, LightType, LightTypeChoice, PedestrianAnimation, SensorReference,
    UserDefinedAnimation, UserDefinedComponent, VehicleComponent, VehicleLight,
};
use openscenario_rs::types::basic::{Double, OSString, Value};
use openscenario_rs::types::entities::vehicle::File;
use openscenario_rs::types::enums::{
    ColorType, LightMode, PedestrianMotionType, VehicleComponentType, VehicleLightType,
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
// The one-branch document is round-tripped byte for byte inside
// `appearance_action_parses_light_state_branch`.

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
// The one-branch document is round-tripped byte for byte inside
// `animation_type_parses_component_animation_branch`.

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

// ---------------------------------------------------------------------------
// Constructors reachable only through the Rust API, not through Deserialize.
//
// Every test above builds a value by parsing XML, so it exercises the
// derived `Deserialize` impl but never the hand-written `impl` blocks
// appearance.rs defines beside each type (`::new`, the choice-branch
// builders). Those constructors are the schema-fabrication guard this crate
// relies on instead of `Default` (see the removed-`Default` comments in
// appearance.rs), so a wrong literal or a swapped field inside one would not
// be caught by any test above. Each row here builds a value with the
// constructor, serializes it, checks the wire element name the branch is
// supposed to produce, and reparses it back to the constructed value.
// ---------------------------------------------------------------------------

#[test]
fn animation_type_and_component_animation_branch_constructors_emit_the_named_wire_element() {
    struct Case {
        kind: &'static str,
        build: fn() -> AnimationType,
        wire_element: &'static str,
    }
    let cases = [
        Case {
            kind: "component/vehicle",
            build: || {
                AnimationType::component(ComponentAnimation::vehicle(VehicleComponent {
                    vehicle_component_type: Value::Literal(VehicleComponentType::DoorFrontLeft),
                }))
            },
            wire_element: "<AnimationType><ComponentAnimation><VehicleComponent",
        },
        Case {
            kind: "component/user_defined",
            // Goes through `ComponentAnimation::new` directly (rather than a
            // named branch helper) to cover that constructor too.
            build: || {
                AnimationType::component(ComponentAnimation::new(
                    ComponentAnimationChoice::UserDefinedComponent(UserDefinedComponent {
                        user_defined_component_type: OSString::literal("custom".to_string()),
                    }),
                ))
            },
            wire_element: "<AnimationType><ComponentAnimation><UserDefinedComponent",
        },
        Case {
            kind: "pedestrian",
            build: || {
                AnimationType::pedestrian(PedestrianAnimation {
                    motion: Some(Value::Literal(PedestrianMotionType::Walking)),
                    user_defined_pedestrian_animation: None,
                    pedestrian_gestures: vec![],
                })
            },
            wire_element: r#"<AnimationType><PedestrianAnimation motion="walking""#,
        },
        Case {
            kind: "file",
            build: || {
                AnimationType::file(AnimationFile {
                    time_offset: None,
                    file: File {
                        filepath: "anim/wave.fbx".to_string(),
                    },
                })
            },
            wire_element: r#"<AnimationType><AnimationFile><File filepath="anim/wave.fbx""#,
        },
        Case {
            kind: "user_defined",
            build: || {
                AnimationType::user_defined(UserDefinedAnimation {
                    user_defined_animation_type: OSString::literal("custom".to_string()),
                })
            },
            wire_element: r#"<AnimationType><UserDefinedAnimation userDefinedAnimationType="custom""#,
        },
    ];

    for case in cases {
        let animation_type = (case.build)();
        let xml = quick_xml::se::to_string(&animation_type).expect("serialize failed");
        assert!(
            xml.starts_with(case.wire_element),
            "{}: expected wire element {:?}, got {xml}",
            case.kind,
            case.wire_element
        );
        let reparsed: AnimationType = de(&xml);
        assert_eq!(
            reparsed, animation_type,
            "{}: reparsed value must equal the constructed value",
            case.kind
        );
    }
}

#[test]
fn light_state_action_built_via_constructors_matches_the_parsed_document() {
    let light_type = LightType::new(LightTypeChoice::VehicleLight(VehicleLight {
        vehicle_light_type: Value::Literal(VehicleLightType::LowBeam),
    }));
    let light_state = LightState::new(LightMode::On);
    assert!(
        light_state.color.is_none(),
        "LightState::new must default the optional Color to None"
    );
    let action = LightStateAction::new(light_type, light_state);
    assert!(
        action.transition_time.is_none(),
        "LightStateAction::new must default the optional transitionTime to None"
    );

    let xml = quick_xml::se::to_string(&action).expect("serialize failed");
    let parsed: LightStateAction = de(LIGHT_STATE_BRANCH);
    assert_eq!(
        action, parsed,
        "the constructed value must equal the value parsed from the XSD-shaped document"
    );
    let reparsed: LightStateAction = de(&xml);
    assert_eq!(reparsed, action);
}

#[test]
fn color_new_pairs_the_required_color_type_with_the_chosen_branch() {
    let color = Color::new(
        ColorType::Red,
        ColorChoice::ColorRgb(ColorRgb {
            red: Double::literal(1.0),
            green: Double::literal(0.0),
            blue: Double::literal(0.0),
        }),
    );
    let xml = quick_xml::se::to_string(&color).expect("serialize failed");
    assert_eq!(
        xml,
        r#"<Color colorType="red"><ColorRgb red="1" green="0" blue="0"/></Color>"#
    );
    let reparsed: Color = de(&xml);
    assert_eq!(reparsed, color);
}

#[test]
fn animation_action_and_leaf_constructors_match_the_parsed_document() {
    let animation_state = AnimationState::new(1.0);
    assert_eq!(animation_state.state.as_literal(), Some(&1.0));

    let mut action = AnimationAction::new(AnimationType::component(ComponentAnimation::vehicle(
        VehicleComponent {
            vehicle_component_type: Value::Literal(VehicleComponentType::DoorFrontLeft),
        },
    )));
    assert!(
        action.r#loop.is_none() && action.animation_duration.is_none(),
        "AnimationAction::new must default the optional loop/duration to None"
    );
    action.r#loop = Some(openscenario_rs::types::basic::Boolean::literal(true));
    action.animation_duration = Some(Double::literal(2.5));
    action.animation_state = Some(animation_state);

    let xml = quick_xml::se::to_string(&action).expect("serialize failed");
    let parsed: AnimationAction = de(
        r#"<AnimationAction loop="true" animationDuration="2.5"><AnimationType><ComponentAnimation><VehicleComponent vehicleComponentType="doorFrontLeft"/></ComponentAnimation></AnimationType><AnimationState state="1"/></AnimationAction>"#,
    );
    assert_eq!(action, parsed);
    let reparsed: AnimationAction = de(&xml);
    assert_eq!(reparsed, action);

    // `SensorReference` belongs to `VisibilityAction`, not the appearance
    // choice groups above; its `::new` has no other exerciser in the crate.
    let sensor = SensorReference::new("lidar");
    assert_eq!(sensor.name.as_literal(), Some(&"lidar".to_string()));
}
