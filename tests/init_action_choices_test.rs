//! Round-trip tests for the `<Storyboard><Init><Actions>` choice groups.
//!
//! `init::GlobalAction` (XSD `GlobalAction`, :1282-1296) and `init::PrivateAction`
//! (XSD `PrivateAction`, :1777-1790) are both `xsd:choice` groups modelled as a
//! struct of parallel `Option` fields. Neither uses `deny_unknown_fields`, so an
//! unmodelled branch does not fail to parse — it silently deserializes to an
//! all-`None` struct and re-serializes as an empty element, which violates the
//! choice (exactly one child required). These tests pin every branch: parse a
//! minimal schema-valid snippet, assert the right field is populated, re-serialize
//! and assert the branch element survives.

use openscenario_rs::types::actions::wrappers::{
    EntityActionChoice, ParameterActionChoice, TrafficActionChoice, VariableActionChoice,
};
use openscenario_rs::types::scenario::init::{GlobalAction, PrivateAction};

fn de<T: serde::de::DeserializeOwned>(xml: &str) -> T {
    quick_xml::de::from_str(xml).unwrap_or_else(|e| panic!("deserialize failed for {xml}: {e}"))
}

fn ser<T: serde::Serialize>(root: &str, v: &T) -> String {
    // Flattened content serializes as a map, so quick-xml needs an explicit root tag.
    quick_xml::se::to_string_with_root(root, v).expect("serialize failed")
}

// ─── init::GlobalAction branches ────────────────────────────────────────────

#[test]
fn global_action_environment_action_round_trip() {
    let xml = r#"<GlobalAction><EnvironmentAction><Environment name="Env1"/></EnvironmentAction></GlobalAction>"#;
    let action: GlobalAction = de(xml);
    assert!(action.environment_action.is_some());
    assert_eq!(action.get_action_type(), Some("EnvironmentAction"));
    assert!(action.validate().is_ok());

    let out = ser("GlobalAction", &action);
    assert!(out.contains("<EnvironmentAction"), "got: {out}");
    let reparsed: GlobalAction = de(&out);
    assert_eq!(action, reparsed);
}

#[test]
fn global_action_entity_action_round_trip() {
    // XSD EntityAction (:1128-1134): @entityRef + choice(AddEntityAction | DeleteEntityAction)
    let xml = r#"<GlobalAction><EntityAction entityRef="npc1"><DeleteEntityAction/></EntityAction></GlobalAction>"#;
    let action: GlobalAction = de(xml);
    let entity = action
        .entity_action
        .as_ref()
        .expect("EntityAction branch must be populated, not silently dropped");
    assert_eq!(entity.entity_ref.to_string(), "npc1");
    assert!(matches!(
        entity.action,
        EntityActionChoice::DeleteEntityAction(_)
    ));
    assert_eq!(action.get_action_type(), Some("EntityAction"));
    assert!(action.validate().is_ok());

    let out = ser("GlobalAction", &action);
    assert!(out.contains("<EntityAction"), "got: {out}");
    assert!(out.contains("DeleteEntityAction"), "got: {out}");
    assert!(out.contains(r#"entityRef="npc1""#), "got: {out}");
}

#[test]
fn global_action_infrastructure_action_round_trip() {
    // XSD InfrastructureAction: sequence with a required TrafficSignalAction.
    let xml = r#"<GlobalAction><InfrastructureAction><TrafficSignalAction><TrafficSignalStateAction name="sig1" state="green"/></TrafficSignalAction></InfrastructureAction></GlobalAction>"#;
    let action: GlobalAction = de(xml);
    assert!(
        action.infrastructure_action.is_some(),
        "InfrastructureAction branch must be populated"
    );
    assert_eq!(action.get_action_type(), Some("InfrastructureAction"));
    assert!(action.validate().is_ok());

    let out = ser("GlobalAction", &action);
    assert!(out.contains("<InfrastructureAction"), "got: {out}");
    assert!(out.contains("TrafficSignalAction"), "got: {out}");
}

#[test]
fn global_action_set_monitor_action_round_trip() {
    // XSD SetMonitorAction: required @monitorRef and @value.
    let xml = r#"<GlobalAction><SetMonitorAction monitorRef="speedMonitor" value="true"/></GlobalAction>"#;
    let action: GlobalAction = de(xml);
    let monitor = action
        .set_monitor_action
        .as_ref()
        .expect("SetMonitorAction branch must be populated");
    assert_eq!(monitor.monitor_ref.to_string(), "speedMonitor");
    assert_eq!(monitor.value.as_literal().unwrap(), &true);
    assert_eq!(action.get_action_type(), Some("SetMonitorAction"));
    assert!(action.validate().is_ok());

    let out = ser("GlobalAction", &action);
    assert!(out.contains("<SetMonitorAction"), "got: {out}");
    assert!(out.contains(r#"monitorRef="speedMonitor""#), "got: {out}");
    let reparsed: GlobalAction = de(&out);
    assert_eq!(action, reparsed);
}

#[test]
fn global_action_parameter_action_round_trip() {
    // XSD ParameterAction (:1288) is deprecated but still a valid choice branch.
    let xml = r#"<GlobalAction><ParameterAction parameterRef="p1"><SetAction value="100"/></ParameterAction></GlobalAction>"#;
    let action: GlobalAction = de(xml);
    let param = action
        .parameter_action
        .as_ref()
        .expect("ParameterAction branch must be populated");
    assert_eq!(param.parameter_ref.to_string(), "p1");
    match &param.action {
        ParameterActionChoice::ParameterSetAction(s) => assert_eq!(s.value.to_string(), "100"),
        other => panic!("expected ParameterSetAction, got {other:?}"),
    }
    assert_eq!(action.get_action_type(), Some("ParameterAction"));
    assert!(action.validate().is_ok());

    let out = ser("GlobalAction", &action);
    assert!(out.contains("<ParameterAction"), "got: {out}");
    assert!(out.contains("<SetAction"), "got: {out}");
}

#[test]
fn global_action_traffic_action_round_trip() {
    let xml = r#"<GlobalAction><TrafficAction trafficName="t1"><TrafficStopAction/></TrafficAction></GlobalAction>"#;
    let action: GlobalAction = de(xml);
    let traffic = action
        .traffic_action
        .as_ref()
        .expect("TrafficAction branch must be populated");
    assert_eq!(
        traffic.traffic_name.as_ref().unwrap().to_string(),
        "t1".to_string()
    );
    assert!(matches!(
        traffic.action,
        TrafficActionChoice::TrafficStopAction(_)
    ));
    assert_eq!(action.get_action_type(), Some("TrafficAction"));
    assert!(action.validate().is_ok());

    let out = ser("GlobalAction", &action);
    assert!(out.contains("<TrafficAction"), "got: {out}");
    assert!(out.contains("TrafficStopAction"), "got: {out}");
}

#[test]
fn global_action_variable_action_round_trip() {
    let xml = r#"<GlobalAction><VariableAction variableRef="v1"><SetAction value="42"/></VariableAction></GlobalAction>"#;
    let action: GlobalAction = de(xml);
    let var = action
        .variable_action
        .as_ref()
        .expect("VariableAction branch must be populated");
    assert_eq!(var.variable_ref.to_string(), "v1");
    match &var.action {
        VariableActionChoice::VariableSetAction(s) => assert_eq!(s.value.to_string(), "42"),
        other => panic!("expected VariableSetAction, got {other:?}"),
    }
    assert_eq!(action.get_action_type(), Some("VariableAction"));
    assert!(action.validate().is_ok());

    let out = ser("GlobalAction", &action);
    assert!(out.contains("<VariableAction"), "got: {out}");
    assert!(out.contains("<SetAction"), "got: {out}");
}

#[test]
fn global_action_choice_cardinality_is_validated() {
    // An all-`None` GlobalAction is what an unmodelled branch used to produce:
    // it serializes to `<GlobalAction/>`, which violates the XSD choice.
    let empty = GlobalAction::default();
    assert!(empty.validate().is_err());
    assert_eq!(empty.get_action_type(), None);

    let multiple = GlobalAction {
        set_monitor_action: Some(
            openscenario_rs::types::actions::wrappers::SetMonitorAction::default(),
        ),
        traffic_action: Some(openscenario_rs::types::actions::wrappers::TrafficAction::default()),
        ..Default::default()
    };
    assert!(multiple.validate().is_err());
}

// ─── init::PrivateAction branches ───────────────────────────────────────────

#[test]
fn private_action_appearance_action_light_state_round_trip() {
    // XSD AppearanceAction := choice(LightStateAction | AnimationAction)
    let xml = r#"<PrivateAction><AppearanceAction><LightStateAction><LightType><VehicleLight vehicleLightType="brakeLights"/></LightType><LightState mode="on"/></LightStateAction></AppearanceAction></PrivateAction>"#;
    let action: PrivateAction = de(xml);
    let appearance = action
        .appearance_action
        .as_ref()
        .expect("AppearanceAction branch must be populated, not silently dropped");
    assert!(appearance.light_state_action.is_some());
    assert!(appearance.animation_action.is_none());
    assert_eq!(action.get_action_type(), Some("AppearanceAction"));
    assert!(action.validate().is_ok());

    let out = ser("PrivateAction", &action);
    assert!(out.contains("<AppearanceAction"), "got: {out}");
    assert!(out.contains("LightStateAction"), "got: {out}");
    let reparsed: PrivateAction = de(&out);
    assert_eq!(action, reparsed);
}

#[test]
fn private_action_trailer_action_connect_round_trip() {
    // XSD TrailerAction := choice(ConnectTrailerAction | DisconnectTrailerAction)
    let xml = r#"<PrivateAction><TrailerAction><ConnectTrailerAction trailerRef="trailer1"/></TrailerAction></PrivateAction>"#;
    let action: PrivateAction = de(xml);
    let trailer = action
        .trailer_action
        .as_ref()
        .expect("TrailerAction branch must be populated, not silently dropped");
    assert_eq!(
        trailer
            .connect_trailer_action
            .as_ref()
            .unwrap()
            .trailer_ref
            .to_string(),
        "trailer1"
    );
    assert!(trailer.disconnect_trailer_action.is_none());
    assert_eq!(action.get_action_type(), Some("TrailerAction"));
    assert!(action.validate().is_ok());

    let out = ser("PrivateAction", &action);
    assert!(out.contains("<TrailerAction"), "got: {out}");
    assert!(out.contains(r#"trailerRef="trailer1""#), "got: {out}");
    let reparsed: PrivateAction = de(&out);
    assert_eq!(action, reparsed);
}

#[test]
fn private_action_trailer_action_disconnect_round_trip() {
    let xml = r#"<PrivateAction><TrailerAction><DisconnectTrailerAction/></TrailerAction></PrivateAction>"#;
    let action: PrivateAction = de(xml);
    let trailer = action
        .trailer_action
        .as_ref()
        .expect("TrailerAction branch");
    assert!(trailer.connect_trailer_action.is_none());
    assert!(trailer.disconnect_trailer_action.is_some());
    assert!(action.validate().is_ok());

    let out = ser("PrivateAction", &action);
    assert!(out.contains("DisconnectTrailerAction"), "got: {out}");
}

#[test]
fn private_action_choice_cardinality_covers_new_branches() {
    let multiple = PrivateAction {
        appearance_action: Some(Default::default()),
        trailer_action: Some(Default::default()),
        ..Default::default()
    };
    assert!(
        multiple.validate().is_err(),
        "two branches set must fail the choice check"
    );
}

// ─── whole-Init integration ─────────────────────────────────────────────────

#[test]
fn init_actions_carry_non_environment_global_actions() {
    // The regression this whole file guards: a non-EnvironmentAction GlobalAction
    // inside <Init> used to deserialize to an empty struct and re-serialize as
    // `<GlobalAction/>`, losing the payload and emitting schema-invalid XML.
    let xml = r#"<Init><Actions>
        <GlobalAction><VariableAction variableRef="v1"><SetAction value="7"/></VariableAction></GlobalAction>
        <GlobalAction><SetMonitorAction monitorRef="m1" value="false"/></GlobalAction>
        <Private entityRef="Ego">
            <PrivateAction><TrailerAction><ConnectTrailerAction trailerRef="tr1"/></TrailerAction></PrivateAction>
        </Private>
    </Actions></Init>"#;
    let init: openscenario_rs::types::scenario::init::Init = de(xml);
    assert_eq!(init.actions.global_actions.len(), 2);
    for ga in &init.actions.global_actions {
        assert!(
            ga.validate().is_ok(),
            "each GlobalAction must hold a branch"
        );
    }
    assert_eq!(
        init.actions.global_actions[0].get_action_type(),
        Some("VariableAction")
    );
    assert_eq!(
        init.actions.global_actions[1].get_action_type(),
        Some("SetMonitorAction")
    );
    assert_eq!(init.actions.private_actions.len(), 1);
    assert_eq!(
        init.actions.private_actions[0].private_actions[0].get_action_type(),
        Some("TrailerAction")
    );

    let out = quick_xml::se::to_string_with_root("Init", &init).expect("serialize failed");
    assert!(out.contains("VariableAction"), "got: {out}");
    assert!(out.contains(r#"monitorRef="m1""#), "got: {out}");
    assert!(out.contains(r#"trailerRef="tr1""#), "got: {out}");
    assert!(
        !out.contains("<GlobalAction/>"),
        "empty GlobalAction is schema-invalid: {out}"
    );
}
