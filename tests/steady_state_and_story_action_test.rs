//! Round-trip tests for the XSD `SteadyState` group and the full story-level
//! `Action` choice.
//!
//! Covers:
//! - `SteadyState` (XSD:2075-2080) on `AbsoluteSpeed` (:672-677) and
//!   `RelativeSpeedToMaster` (:1889-1895), plus the latter's required
//!   `@speedTargetValueType`.
//! - `Action` (XSD:705-712) choice(GlobalAction | UserDefinedAction |
//!   PrivateAction) as modeled by `StoryAction`.
//! - The deprecated-but-emitted `ActivateControllerAction` branch of
//!   `PrivateAction` (XSD:1777-1791).

use openscenario_rs::types::actions::movement::{AbsoluteSpeed, RelativeSpeedToMaster};
use openscenario_rs::types::actions::wrappers::{EntityActionChoice, GlobalAction};
use openscenario_rs::types::enums::SpeedTargetValueType;
use openscenario_rs::types::scenario::story::StoryAction;

fn de<T: serde::de::DeserializeOwned>(xml: &str) -> T {
    quick_xml::de::from_str(xml).unwrap_or_else(|e| panic!("deserialize failed for {xml}: {e}"))
}

fn ser<T: serde::Serialize>(root: &str, v: &T) -> String {
    quick_xml::se::to_string_with_root(root, v).expect("serialize failed")
}

#[test]
fn absolute_speed_target_distance_steady_state_round_trip() {
    let xml = r#"<AbsoluteSpeed value="0"><TargetDistanceSteadyState distance="5"/></AbsoluteSpeed>"#;
    let speed: AbsoluteSpeed = de(xml);
    assert_eq!(speed.value.as_literal().copied(), Some(0.0));
    assert_eq!(
        speed
            .target_distance_steady_state
            .as_ref()
            .and_then(|s| s.distance.as_literal().copied()),
        Some(5.0)
    );
    assert!(speed.target_time_steady_state.is_none());

    let out = ser("AbsoluteSpeed", &speed);
    assert!(out.contains(r#"distance="5""#), "got: {out}");
    assert!(out.contains("TargetDistanceSteadyState"), "got: {out}");
    assert_eq!(de::<AbsoluteSpeed>(&out), speed);
}

#[test]
fn absolute_speed_target_time_steady_state_round_trip() {
    let xml = r#"<AbsoluteSpeed value="12.5"><TargetTimeSteadyState time="2"/></AbsoluteSpeed>"#;
    let speed: AbsoluteSpeed = de(xml);
    assert_eq!(
        speed
            .target_time_steady_state
            .as_ref()
            .and_then(|s| s.time.as_literal().copied()),
        Some(2.0)
    );
    assert!(speed.target_distance_steady_state.is_none());

    let out = ser("AbsoluteSpeed", &speed);
    assert!(out.contains("TargetTimeSteadyState"), "got: {out}");
    assert_eq!(de::<AbsoluteSpeed>(&out), speed);
}

#[test]
fn absolute_speed_without_steady_state_round_trip() {
    let xml = r#"<AbsoluteSpeed value="30"/>"#;
    let speed: AbsoluteSpeed = de(xml);
    assert!(speed.target_distance_steady_state.is_none());
    assert!(speed.target_time_steady_state.is_none());

    let out = ser("AbsoluteSpeed", &speed);
    assert!(!out.contains("SteadyState"), "got: {out}");
}

#[test]
fn relative_speed_to_master_round_trip() {
    let xml = r#"<RelativeSpeedToMaster speedTargetValueType="delta" value="-5"><TargetTimeSteadyState time="1.5"/></RelativeSpeedToMaster>"#;
    let speed: RelativeSpeedToMaster = de(xml);
    assert_eq!(speed.speed_target_value_type, SpeedTargetValueType::Delta);
    assert_eq!(speed.value.as_literal().copied(), Some(-5.0));
    assert_eq!(
        speed
            .target_time_steady_state
            .as_ref()
            .and_then(|s| s.time.as_literal().copied()),
        Some(1.5)
    );

    let out = ser("RelativeSpeedToMaster", &speed);
    assert!(out.contains(r#"speedTargetValueType="delta""#), "got: {out}");
    assert_eq!(de::<RelativeSpeedToMaster>(&out), speed);
}

#[test]
fn story_action_global_action_round_trip() {
    let xml = r#"<Action name="x"><GlobalAction><EntityAction entityRef="e"><DeleteEntityAction/></EntityAction></GlobalAction></Action>"#;
    let action: StoryAction = de(xml);
    assert_eq!(action.name.to_string(), "x");
    assert!(action.private_action.is_none());
    match action.global_action.as_ref().map(|g| &g.action) {
        Some(GlobalAction::EntityAction(e)) => {
            assert_eq!(e.entity_ref.to_string(), "e");
            assert!(matches!(e.action, EntityActionChoice::DeleteEntityAction(_)));
        }
        other => panic!("expected EntityAction, got {other:?}"),
    }

    let out = ser("Action", &action);
    assert!(out.contains("GlobalAction"), "got: {out}");
    assert!(out.contains("DeleteEntityAction"), "got: {out}");
    assert_eq!(de::<StoryAction>(&out), action);
}

#[test]
fn story_action_activate_controller_action_round_trip() {
    let xml = r#"<Action name="act"><PrivateAction><ActivateControllerAction controllerRef="ctrl" lateral="true"/></PrivateAction></Action>"#;
    let action: StoryAction = de(xml);
    let private = action.private_action.as_ref().expect("private action");
    let aca = private
        .activate_controller_action
        .as_ref()
        .expect("activate controller action");
    assert_eq!(
        aca.controller_ref.as_ref().map(|r| r.to_string()),
        Some("ctrl".to_string())
    );

    let out = ser("Action", &action);
    assert!(out.contains("ActivateControllerAction"), "got: {out}");
    assert_eq!(de::<StoryAction>(&out), action);
}
