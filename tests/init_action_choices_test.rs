//! Round-trip tests for the `<Storyboard><Init><Actions>` choice groups.
//!
//! `init::GlobalAction` (XSD `GlobalAction`, :1282-1295) and `init::PrivateAction`
//! (XSD `PrivateAction`, :1777-1791) are both `xsd:choice` groups, each modelled as a
//! single `$value` field holding an externally-tagged enum. These tests pin every branch
//! the file covers: parse a minimal schema-valid snippet, assert the right variant was
//! selected, re-serialize and assert the branch element survives. Cardinality is enforced
//! by the type rather than by a `validate()` call, so the tests that used to construct an
//! all-`None` or two-branch value now assert that the corresponding documents are rejected
//! at parse time.

use openscenario_rs::types::actions::appearance::AppearanceActionChoice;
use openscenario_rs::types::actions::trailer::TrailerActionChoice;
use openscenario_rs::types::actions::wrappers::{
    EntityActionChoice, ParameterActionChoice, TrafficActionChoice, VariableActionChoice,
};
use openscenario_rs::types::scenario::init::{
    GlobalAction, GlobalActionChoice, PrivateAction, PrivateActionChoice,
};

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
    assert!(matches!(
        action.action,
        GlobalActionChoice::EnvironmentAction(_)
    ));
    assert_eq!(action.action_type(), "EnvironmentAction");

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
    let GlobalActionChoice::EntityAction(entity) = &action.action else {
        panic!("expected the EntityAction branch, got {:?}", action.action);
    };
    assert_eq!(entity.entity_ref.to_string(), "npc1");
    assert!(matches!(
        entity.action,
        EntityActionChoice::DeleteEntityAction(_)
    ));
    assert_eq!(action.action_type(), "EntityAction");

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
    assert!(matches!(
        action.action,
        GlobalActionChoice::InfrastructureAction(_)
    ));
    assert_eq!(action.action_type(), "InfrastructureAction");

    let out = ser("GlobalAction", &action);
    assert!(out.contains("<InfrastructureAction"), "got: {out}");
    assert!(out.contains("TrafficSignalAction"), "got: {out}");
}

#[test]
fn global_action_set_monitor_action_round_trip() {
    // XSD SetMonitorAction: required @monitorRef and @value.
    let xml = r#"<GlobalAction><SetMonitorAction monitorRef="speedMonitor" value="true"/></GlobalAction>"#;
    let action: GlobalAction = de(xml);
    let GlobalActionChoice::SetMonitorAction(monitor) = &action.action else {
        panic!(
            "expected the SetMonitorAction branch, got {:?}",
            action.action
        );
    };
    assert_eq!(monitor.monitor_ref.to_string(), "speedMonitor");
    assert_eq!(monitor.value.as_literal().unwrap(), &true);
    assert_eq!(action.action_type(), "SetMonitorAction");

    let out = ser("GlobalAction", &action);
    assert_eq!(
        out, xml,
        "the serialized bytes must equal the source document"
    );
    let reparsed: GlobalAction = de(&out);
    assert_eq!(action, reparsed);
}

#[test]
fn global_action_parameter_action_round_trip() {
    // XSD ParameterAction (:1288) is deprecated but still a valid choice branch.
    let xml = r#"<GlobalAction><ParameterAction parameterRef="p1"><SetAction value="100"/></ParameterAction></GlobalAction>"#;
    let action: GlobalAction = de(xml);
    let GlobalActionChoice::ParameterAction(param) = &action.action else {
        panic!(
            "expected the ParameterAction branch, got {:?}",
            action.action
        );
    };
    assert_eq!(param.parameter_ref.to_string(), "p1");
    match &param.action {
        ParameterActionChoice::ParameterSetAction(s) => assert_eq!(s.value.to_string(), "100"),
        other => panic!("expected ParameterSetAction, got {other:?}"),
    }
    assert_eq!(action.action_type(), "ParameterAction");

    let out = ser("GlobalAction", &action);
    assert!(out.contains("<ParameterAction"), "got: {out}");
    assert!(out.contains("<SetAction"), "got: {out}");
}

#[test]
fn global_action_traffic_action_round_trip() {
    let xml = r#"<GlobalAction><TrafficAction trafficName="t1"><TrafficStopAction/></TrafficAction></GlobalAction>"#;
    let action: GlobalAction = de(xml);
    let GlobalActionChoice::TrafficAction(traffic) = &action.action else {
        panic!("expected the TrafficAction branch, got {:?}", action.action);
    };
    assert_eq!(
        traffic.traffic_name.as_ref().unwrap().to_string(),
        "t1".to_string()
    );
    assert!(matches!(
        traffic.action,
        TrafficActionChoice::TrafficStopAction(_)
    ));
    assert_eq!(action.action_type(), "TrafficAction");

    let out = ser("GlobalAction", &action);
    assert!(out.contains("<TrafficAction"), "got: {out}");
    assert!(out.contains("TrafficStopAction"), "got: {out}");
}

#[test]
fn global_action_variable_action_round_trip() {
    let xml = r#"<GlobalAction><VariableAction variableRef="v1"><SetAction value="42"/></VariableAction></GlobalAction>"#;
    let action: GlobalAction = de(xml);
    let GlobalActionChoice::VariableAction(var) = &action.action else {
        panic!(
            "expected the VariableAction branch, got {:?}",
            action.action
        );
    };
    assert_eq!(var.variable_ref.to_string(), "v1");
    match &var.action {
        VariableActionChoice::VariableSetAction(s) => assert_eq!(s.value.to_string(), "42"),
        other => panic!("expected VariableSetAction, got {other:?}"),
    }
    assert_eq!(action.action_type(), "VariableAction");

    let out = ser("GlobalAction", &action);
    assert!(out.contains("<VariableAction"), "got: {out}");
    assert!(out.contains("<SetAction"), "got: {out}");
}

/// A `<GlobalAction/>` naming no branch used to deserialize to an all-`None` struct and
/// re-serialize unchanged, which violates the XSD choice. The `$value` shape rejects it.
#[test]
fn global_action_with_no_branch_is_rejected() {
    let err = quick_xml::de::from_str::<GlobalAction>("<GlobalAction/>").unwrap_err();
    assert!(
        err.to_string().contains("$value"),
        "expected a missing-$value error, got {err}"
    );
}

/// Asserted separately from the zero-branch case: a single test covering both would stop at
/// the first failure, and this is the case that used to keep both branches.
#[test]
fn global_action_with_two_branches_is_rejected() {
    let xml = r#"<GlobalAction><SetMonitorAction monitorRef="monitor1" value="true"/><TrafficAction><TrafficStopAction/></TrafficAction></GlobalAction>"#;
    let err = quick_xml::de::from_str::<GlobalAction>(xml).unwrap_err();
    assert!(
        err.to_string().contains("$value"),
        "expected a duplicate-$value error, got {err}"
    );
}

// ─── init::PrivateAction branches ───────────────────────────────────────────

#[test]
fn private_action_appearance_action_light_state_round_trip() {
    // XSD AppearanceAction := choice(LightStateAction | AnimationAction)
    let xml = r#"<PrivateAction><AppearanceAction><LightStateAction><LightType><VehicleLight vehicleLightType="brakeLights"/></LightType><LightState mode="on"/></LightStateAction></AppearanceAction></PrivateAction>"#;
    let action: PrivateAction = de(xml);
    let PrivateActionChoice::AppearanceAction(appearance) = &action.action else {
        panic!(
            "expected the AppearanceAction branch, got {:?}",
            action.action
        );
    };
    assert!(matches!(
        appearance.choice,
        AppearanceActionChoice::LightStateAction(_)
    ));
    assert_eq!(action.action_type(), "AppearanceAction");

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
    let PrivateActionChoice::TrailerAction(trailer) = &action.action else {
        panic!("expected the TrailerAction branch, got {:?}", action.action);
    };
    match &trailer.choice {
        TrailerActionChoice::ConnectTrailerAction(c) => {
            assert_eq!(c.trailer_ref.to_string(), "trailer1")
        }
        other => panic!("expected ConnectTrailerAction, got {other:?}"),
    }
    assert_eq!(action.action_type(), "TrailerAction");

    let out = ser("PrivateAction", &action);
    assert_eq!(
        out, xml,
        "the serialized bytes must equal the source document"
    );
    let reparsed: PrivateAction = de(&out);
    assert_eq!(action, reparsed);
}

#[test]
fn private_action_trailer_action_disconnect_round_trip() {
    let xml = r#"<PrivateAction><TrailerAction><DisconnectTrailerAction/></TrailerAction></PrivateAction>"#;
    let action: PrivateAction = de(xml);
    let PrivateActionChoice::TrailerAction(trailer) = &action.action else {
        panic!("expected the TrailerAction branch, got {:?}", action.action);
    };
    assert!(matches!(
        trailer.choice,
        TrailerActionChoice::DisconnectTrailerAction(_)
    ));

    let out = ser("PrivateAction", &action);
    assert!(out.contains("DisconnectTrailerAction"), "got: {out}");
}

/// The appearance and trailer branches were the two this file was written to pin, so the
/// cardinality cases use that pair.
#[test]
fn private_action_with_no_branch_is_rejected() {
    let err = quick_xml::de::from_str::<PrivateAction>("<PrivateAction/>").unwrap_err();
    assert!(
        err.to_string().contains("$value"),
        "expected a missing-$value error, got {err}"
    );
}

#[test]
fn private_action_with_two_branches_is_rejected() {
    let xml = r#"<PrivateAction><AppearanceAction><LightStateAction><LightType><VehicleLight vehicleLightType="lowBeam"/></LightType><LightState mode="on"/></LightStateAction></AppearanceAction><TrailerAction><DisconnectTrailerAction/></TrailerAction></PrivateAction>"#;
    let err = quick_xml::de::from_str::<PrivateAction>(xml).unwrap_err();
    assert!(
        err.to_string().contains("$value"),
        "expected a duplicate-$value error, got {err}"
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
    assert_eq!(
        init.actions.global_actions[0].action_type(),
        "VariableAction"
    );
    assert_eq!(
        init.actions.global_actions[1].action_type(),
        "SetMonitorAction"
    );
    assert_eq!(init.actions.private_actions.len(), 1);
    assert_eq!(
        init.actions.private_actions[0].private_actions[0].action_type(),
        "TrailerAction"
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
