//! Round-trip tests for every `#[serde(flatten)]`-over-externally-tagged-enum site.
//!
//! These sites model an XSD `xsd:choice` as a struct holding a flattened enum.
//! Externally tagged enums serialize using the *variant* name, so the variant name
//! must match the **XSD element name**, not the XSD type name. Each test below
//! feeds a minimal schema-valid XML snippet in, checks the deserialized variant and
//! its field values, re-serializes, and asserts the XSD element name is emitted.

use openscenario_rs::types::actions::control::{
    BrakeInput, Gear, OverrideBrakeAction, OverrideGearAction, OverrideParkingBrakeAction,
};
use openscenario_rs::types::actions::movement::{
    AssignRouteAction, FinalSpeed, FinalSpeedChoice, LaneChangeTarget, LaneChangeTargetChoice,
    LaneOffsetTarget, LaneOffsetTargetChoice, LateralAction, LateralActionChoice,
    LongitudinalAction, LongitudinalActionChoice,
};
use openscenario_rs::types::actions::traffic::{TrafficSignalAction, TrafficSignalActionChoice};
use openscenario_rs::types::actions::wrappers::{
    Action, EntityAction, EntityActionChoice, GlobalAction, ModifyRuleChoice, NamedAction,
    ParameterAction, ParameterActionChoice, TrafficAction, TrafficActionChoice, VariableAction,
    VariableActionChoice, VariableModifyRuleChoice,
};
use openscenario_rs::types::routing::RouteRef;

fn de<T: serde::de::DeserializeOwned>(xml: &str) -> T {
    quick_xml::de::from_str(xml).unwrap_or_else(|e| panic!("deserialize failed for {xml}: {e}"))
}

fn ser<T: serde::Serialize>(root: &str, v: &T) -> String {
    // Flattened content serializes as a map, so quick-xml needs an explicit root tag.
    quick_xml::se::to_string_with_root(root, v).expect("serialize failed")
}

// ─── movement.rs: AssignRouteAction (RouteRef) ────────────────────────────
// XSD:786-791 AssignRouteAction := choice(Route | CatalogReference)

#[test]
fn assign_route_action_catalog_reference_round_trip() {
    let xml = r#"<AssignRouteAction><CatalogReference catalogName="Routes" entryName="R1"/></AssignRouteAction>"#;
    let action: AssignRouteAction = de(xml);
    match &action.route {
        RouteRef::Catalog(c) => {
            assert_eq!(c.catalog_name.to_string(), "Routes");
            assert_eq!(c.entry_name.to_string(), "R1");
        }
        other => panic!("expected RouteRef::Catalog, got {other:?}"),
    }
    let out = ser("AssignRouteAction", &action);
    assert!(out.contains("CatalogReference"), "got: {out}");
}

// ─── movement.rs: LaneChangeTarget ──────────────────────────────────────────
// XSD:1343-1348 choice(RelativeTargetLane | AbsoluteTargetLane)

#[test]
fn lane_change_target_absolute_round_trip() {
    let xml = r#"<LaneChangeTarget><AbsoluteTargetLane value="2"/></LaneChangeTarget>"#;
    let target: LaneChangeTarget = de(xml);
    match &target.target_choice {
        LaneChangeTargetChoice::AbsoluteTargetLane(a) => assert_eq!(a.value.to_string(), "2"),
        other => panic!("expected AbsoluteTargetLane, got {other:?}"),
    }
    let out = ser("LaneChangeTarget", &target);
    assert!(out.contains("AbsoluteTargetLane"), "got: {out}");
}

// ─── movement.rs: LaneOffsetTarget ──────────────────────────────────────────
// XSD:1360-1365 choice(RelativeTargetLaneOffset | AbsoluteTargetLaneOffset)

#[test]
fn lane_offset_target_absolute_round_trip() {
    let xml = r#"<LaneOffsetTarget><AbsoluteTargetLaneOffset value="1.5"/></LaneOffsetTarget>"#;
    let target: LaneOffsetTarget = de(xml);
    match &target.target_choice {
        LaneOffsetTargetChoice::AbsoluteTargetLaneOffset(a) => {
            assert_eq!(a.value.as_literal().copied(), Some(1.5))
        }
        other => panic!("expected AbsoluteTargetLaneOffset, got {other:?}"),
    }
    let out = ser("LaneOffsetTarget", &target);
    assert!(out.contains("AbsoluteTargetLaneOffset"), "got: {out}");
}

// ─── movement.rs: LateralAction ─────────────────────────────────────────────
// XSD:1375-1381 choice(LaneChangeAction | LaneOffsetAction | LateralDistanceAction)

#[test]
fn lateral_action_lateral_distance_round_trip() {
    let xml = r#"<LateralAction><LateralDistanceAction entityRef="ego" distance="5.0" freespace="true" continuous="false"/></LateralAction>"#;
    let action: LateralAction = de(xml);
    match &action.lateral_choice {
        LateralActionChoice::LateralDistanceAction(a) => {
            assert_eq!(a.entity_ref.to_string(), "ego");
            assert_eq!(
                a.distance.as_ref().and_then(|d| d.as_literal().copied()),
                Some(5.0)
            );
        }
        other => panic!("expected LateralDistanceAction, got {other:?}"),
    }
    let out = ser("LateralAction", &action);
    assert!(out.contains("LateralDistanceAction"), "got: {out}");
}

// ─── movement.rs: LongitudinalAction ────────────────────────────────────────
// XSD:1431-1437 choice(SpeedAction | LongitudinalDistanceAction | SpeedProfileAction)

#[test]
fn longitudinal_action_longitudinal_distance_round_trip() {
    let xml = r#"<LongitudinalAction><LongitudinalDistanceAction entityRef="lead" distance="12.0" freespace="true" continuous="true"/></LongitudinalAction>"#;
    let action: LongitudinalAction = de(xml);
    match &action.longitudinal_action_choice {
        LongitudinalActionChoice::LongitudinalDistanceAction(a) => {
            assert_eq!(a.entity_ref.to_string(), "lead");
            assert_eq!(
                a.distance.as_ref().and_then(|d| d.as_literal().copied()),
                Some(12.0)
            );
        }
        other => panic!("expected LongitudinalDistanceAction, got {other:?}"),
    }
    let out = ser("LongitudinalAction", &action);
    assert!(out.contains("LongitudinalDistanceAction"), "got: {out}");
}

// ─── movement.rs: FinalSpeed ────────────────────────────────────────────────
// XSD:1232-1237 choice(AbsoluteSpeed | RelativeSpeedToMaster)

#[test]
fn final_speed_absolute_round_trip() {
    let xml = r#"<FinalSpeed><AbsoluteSpeed value="27.5"/></FinalSpeed>"#;
    let fs: FinalSpeed = de(xml);
    match &fs.speed_choice {
        FinalSpeedChoice::AbsoluteSpeed(a) => {
            assert_eq!(a.value.as_literal().copied(), Some(27.5))
        }
        other => panic!("expected AbsoluteSpeed, got {other:?}"),
    }
    let out = ser("FinalSpeed", &fs);
    assert!(out.contains("AbsoluteSpeed"), "got: {out}");
}

// ─── wrappers.rs: EntityAction ──────────────────────────────────────────────
// XSD:1128-1134 choice(AddEntityAction | DeleteEntityAction) + @entityRef required

#[test]
fn entity_action_delete_round_trip() {
    let xml = r#"<EntityAction entityRef="npc1"><DeleteEntityAction/></EntityAction>"#;
    let action: EntityAction = de(xml);
    assert_eq!(action.entity_ref.to_string(), "npc1");
    assert!(matches!(
        action.action,
        EntityActionChoice::DeleteEntityAction(_)
    ));
    let out = ser("EntityAction", &action);
    assert!(out.contains("DeleteEntityAction"), "got: {out}");
    assert!(out.contains("entityRef=\"npc1\""), "got: {out}");
}

// ─── wrappers.rs: TrafficAction ─────────────────────────────────────────────
// XSD:2204-2213 choice(TrafficSourceAction | ... | TrafficStopAction) + @trafficName

#[test]
fn traffic_action_stop_round_trip() {
    let xml = r#"<TrafficAction trafficName="t1"><TrafficStopAction/></TrafficAction>"#;
    let action: TrafficAction = de(xml);
    assert_eq!(
        action.traffic_name.as_ref().map(|n| n.to_string()),
        Some("t1".to_string())
    );
    assert!(matches!(
        action.action,
        TrafficActionChoice::TrafficStopAction(_)
    ));
    let out = ser("TrafficAction", &action);
    assert!(out.contains("TrafficStopAction"), "got: {out}");
}

// ─── wrappers.rs: VariableAction ────────────────────────────────────────────
// XSD:2456-2462 choice(SetAction | ModifyAction) + @variableRef required

#[test]
fn variable_action_set_action_round_trip() {
    let xml = r#"<VariableAction variableRef="v1"><SetAction value="42"/></VariableAction>"#;
    let action: VariableAction = de(xml);
    assert_eq!(action.variable_ref.to_string(), "v1");
    match &action.action {
        VariableActionChoice::VariableSetAction(s) => assert_eq!(s.value.to_string(), "42"),
        other => panic!("expected SetAction, got {other:?}"),
    }
    let out = ser("VariableAction", &action);
    assert!(out.contains("<SetAction"), "got: {out}");
    assert!(out.contains("variableRef=\"v1\""), "got: {out}");
    assert!(!out.contains("VariableSetAction"), "got: {out}");
}

// XSD:2481-2485 VariableModifyAction := all(Rule: VariableModifyRule)
// XSD:2486-2491 VariableModifyRule := choice(AddValue | MultiplyByValue)

#[test]
fn variable_action_modify_action_round_trip() {
    let xml = r#"<VariableAction variableRef="v2"><ModifyAction><Rule><AddValue value="10.5"/></Rule></ModifyAction></VariableAction>"#;
    let action: VariableAction = de(xml);
    assert_eq!(action.variable_ref.to_string(), "v2");
    match &action.action {
        VariableActionChoice::VariableModifyAction(m) => match &m.rule.rule {
            VariableModifyRuleChoice::VariableAddValueRule(r) => {
                assert_eq!(r.value.as_literal().copied(), Some(10.5))
            }
            other => panic!("expected AddValue, got {other:?}"),
        },
        other => panic!("expected ModifyAction, got {other:?}"),
    }
    let out = ser("VariableAction", &action);
    assert!(out.contains("<ModifyAction"), "got: {out}");
    assert!(out.contains("<Rule"), "got: {out}");
    assert!(out.contains("<AddValue"), "got: {out}");
}

#[test]
fn variable_modify_action_multiply_round_trip() {
    let xml = r#"<VariableAction variableRef="v3"><ModifyAction><Rule><MultiplyByValue value="2"/></Rule></ModifyAction></VariableAction>"#;
    let action: VariableAction = de(xml);
    match &action.action {
        VariableActionChoice::VariableModifyAction(m) => assert!(matches!(
            m.rule.rule,
            VariableModifyRuleChoice::VariableMultiplyByValueRule(_)
        )),
        other => panic!("expected ModifyAction, got {other:?}"),
    }
    assert!(ser("VariableAction", &action).contains("<MultiplyByValue"));
}

// ─── wrappers.rs: ParameterAction ───────────────────────────────────────────
// XSD:1606-1613 choice(SetAction | ModifyAction) + @parameterRef required

#[test]
fn parameter_action_set_action_round_trip() {
    let xml = r#"<ParameterAction parameterRef="p1"><SetAction value="100"/></ParameterAction>"#;
    let action: ParameterAction = de(xml);
    assert_eq!(action.parameter_ref.to_string(), "p1");
    match &action.action {
        ParameterActionChoice::ParameterSetAction(s) => assert_eq!(s.value.to_string(), "100"),
        other => panic!("expected SetAction, got {other:?}"),
    }
    let out = ser("ParameterAction", &action);
    assert!(out.contains("<SetAction"), "got: {out}");
    assert!(out.contains("parameterRef=\"p1\""), "got: {out}");
    assert!(!out.contains("ParameterSetAction"), "got: {out}");
}

// XSD:1647-1651 ParameterModifyAction := all(Rule: ModifyRule)
// XSD:1490-1496 ModifyRule := choice(AddValue | MultiplyByValue)

#[test]
fn parameter_action_modify_action_round_trip() {
    let xml = r#"<ParameterAction parameterRef="p2"><ModifyAction><Rule><AddValue value="5"/></Rule></ModifyAction></ParameterAction>"#;
    let action: ParameterAction = de(xml);
    match &action.action {
        ParameterActionChoice::ParameterModifyAction(m) => match &m.rule.rule {
            ModifyRuleChoice::ParameterAddValueRule(r) => {
                assert_eq!(r.value.as_literal().copied(), Some(5.0))
            }
            other => panic!("expected AddValue, got {other:?}"),
        },
        other => panic!("expected ModifyAction, got {other:?}"),
    }
    let out = ser("ParameterAction", &action);
    assert!(out.contains("<ModifyAction"), "got: {out}");
    assert!(out.contains("<Rule"), "got: {out}");
    assert!(out.contains("<AddValue"), "got: {out}");
}

#[test]
fn parameter_modify_action_multiply_round_trip() {
    let xml = r#"<ParameterAction parameterRef="p3"><ModifyAction><Rule><MultiplyByValue value="3"/></Rule></ModifyAction></ParameterAction>"#;
    let action: ParameterAction = de(xml);
    match &action.action {
        ParameterActionChoice::ParameterModifyAction(m) => assert!(matches!(
            m.rule.rule,
            ModifyRuleChoice::ParameterMultiplyByValueRule(_)
        )),
        other => panic!("expected ModifyAction, got {other:?}"),
    }
    assert!(ser("ParameterAction", &action).contains("<MultiplyByValue"));
}

// ─── wrappers.rs: NamedAction (Action) ──────────────────────────────────────
// XSD:705-712 choice(GlobalAction | UserDefinedAction | PrivateAction) + @name required

#[test]
fn named_action_user_defined_action_round_trip() {
    let xml = r#"<Action name="a1"><UserDefinedAction><CustomCommandAction type="myCommand"/></UserDefinedAction></Action>"#;
    let action: NamedAction = de(xml);
    assert_eq!(action.name.to_string(), "a1");
    assert!(matches!(action.action, Action::UserDefinedAction(_)));
    let out = ser("Action", &action);
    assert!(out.contains("UserDefinedAction"), "got: {out}");
    assert!(out.contains("name=\"a1\""), "got: {out}");
}

/// KNOWN LIMITATION: `NamedAction` flattens `Action`, whose `GlobalAction` /
/// `PrivateAction` variants carry a *nested* externally-tagged enum. quick-xml
/// cannot serialize an enum newtype variant nested inside flattened map content,
/// so this branch deserializes correctly but cannot be re-serialized. `NamedAction`
/// is not used by the scenario tree (see `StoryAction` in `scenario/story.rs`,
/// which uses parallel `Option` fields instead), so real files are unaffected.
#[test]
fn named_action_global_action_deserializes_but_cannot_serialize() {
    let xml = r#"<Action name="a1"><GlobalAction><SetMonitorAction monitorRef="m1" value="true"/></GlobalAction></Action>"#;
    let action: NamedAction = de(xml);
    assert_eq!(action.name.to_string(), "a1");
    match &action.action {
        Action::GlobalAction(GlobalAction::SetMonitorAction(m)) => {
            assert_eq!(m.monitor_ref.to_string(), "m1")
        }
        other => panic!("expected GlobalAction/SetMonitorAction, got {other:?}"),
    }
    assert!(
        quick_xml::se::to_string_with_root("Action", &action).is_err(),
        "nested enum-in-flatten unexpectedly serialized; update this test if fixed"
    );
}

// ─── traffic.rs: TrafficSignalAction ────────────────────────────────────────
// XSD:2248-2253 choice(TrafficSignalControllerAction | TrafficSignalStateAction)

#[test]
fn traffic_signal_action_controller_round_trip() {
    let xml = r#"<TrafficSignalAction><TrafficSignalControllerAction trafficSignalControllerRef="ctrl" phase="green"/></TrafficSignalAction>"#;
    let action: TrafficSignalAction = de(xml);
    match &action.signal_action_choice {
        TrafficSignalActionChoice::TrafficSignalControllerAction(c) => {
            assert_eq!(c.traffic_signal_controller_ref.to_string(), "ctrl");
            assert_eq!(c.phase_ref.to_string(), "green");
        }
        other => panic!("expected TrafficSignalControllerAction, got {other:?}"),
    }
    let out = ser("TrafficSignalAction", &action);
    assert!(out.contains("TrafficSignalControllerAction"), "got: {out}");
}

// ─── control.rs: OverrideBrakeAction (BrakeInput group) ─────────────────────
// XSD:819-824 BrakeInput := choice(BrakePercent | BrakeForce)

#[test]
fn override_brake_action_brake_percent_round_trip() {
    let xml =
        r#"<OverrideBrakeAction active="true"><BrakePercent value="0.5"/></OverrideBrakeAction>"#;
    let action: OverrideBrakeAction = de(xml);
    match &action.brake_input {
        Some(BrakeInput::BrakePercent(b)) => {
            assert_eq!(b.value.as_literal().copied(), Some(0.5))
        }
        other => panic!("expected BrakePercent, got {other:?}"),
    }
    let out = ser("OverrideBrakeAction", &action);
    assert!(out.contains("BrakePercent"), "got: {out}");
}

// XSD:1584-1590 OverrideParkingBrakeAction := sequence(group BrakeInput minOccurs=0)

#[test]
fn override_parking_brake_action_brake_force_round_trip() {
    let xml = r#"<OverrideParkingBrakeAction active="true"><BrakeForce value="1000"/></OverrideParkingBrakeAction>"#;
    let action: OverrideParkingBrakeAction = de(xml);
    assert!(matches!(
        action.brake_input,
        Some(BrakeInput::BrakeForce(_))
    ));
    let out = ser("OverrideParkingBrakeAction", &action);
    assert!(out.contains("BrakeForce"), "got: {out}");
}

// ─── control.rs: OverrideGearAction (Gear group) ────────────────────────────
// XSD:1258-1263 Gear := choice(ManualGear | AutomaticGear)

#[test]
fn override_gear_action_manual_gear_round_trip() {
    let xml = r#"<OverrideGearAction active="true"><ManualGear number="3"/></OverrideGearAction>"#;
    let action: OverrideGearAction = de(xml);
    match &action.gear {
        Some(Gear::ManualGear(g)) => assert_eq!(g.number.to_string(), "3"),
        other => panic!("expected ManualGear, got {other:?}"),
    }
    let out = ser("OverrideGearAction", &action);
    assert!(out.contains("ManualGear"), "got: {out}");
}
