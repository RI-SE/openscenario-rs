//! Test suite for Action wrapper types implementation
//!
//! This test verifies that the new Action wrapper types correctly implement
//! the OpenSCENARIO XSD schema structure for actions.

use openscenario_rs::types::actions::movement::{SpeedActionTarget, TransitionDynamics};
use openscenario_rs::types::actions::{wrappers::*, *};
use openscenario_rs::types::basic::*;
use openscenario_rs::types::enums::{DynamicsDimension, DynamicsShape};
use openscenario_rs::types::positions::*;
use serde_json;

#[test]
fn test_core_action_serialization() {
    // Test PrivateAction serialization
    let private_action = PrivateAction::TeleportAction(TeleportAction::new(Position::world(
        WorldPosition::new(1.0, 2.0),
    )));
    let core_action = Action::PrivateAction(private_action);

    let serialized = serde_json::to_string(&core_action).unwrap();
    assert!(serialized.contains("PrivateAction"));

    // Test deserialization
    let deserialized: Action = serde_json::from_str(&serialized).unwrap();
    assert_eq!(core_action, deserialized);
}

#[test]
fn test_global_action_variants() {
    // Test TrafficAction variant
    let traffic_action = TrafficAction {
        traffic_name: Some(OSString::literal("test_traffic".to_string())),
        action: TrafficActionChoice::TrafficStopAction(TrafficStopAction::default()),
    };
    let global_action = GlobalAction::TrafficAction(traffic_action);

    let serialized = serde_json::to_string(&global_action).unwrap();
    assert!(serialized.contains("TrafficAction"));

    // Test EntityAction variant
    let entity_action = EntityAction {
        entity_ref: OSString::literal("test_entity".to_string()),
        action: EntityActionChoice::DeleteEntityAction(DeleteEntityAction::default()),
    };
    let global_action = GlobalAction::EntityAction(entity_action);

    let serialized = serde_json::to_string(&global_action).unwrap();
    assert!(serialized.contains("EntityAction"));
}

#[test]
fn test_entity_action_types() {
    // Test AddEntityAction
    let add_action = AddEntityAction {
        position: Position::world_origin(),
    };
    let entity_action = EntityAction {
        entity_ref: OSString::literal("new_entity".to_string()),
        action: EntityActionChoice::AddEntityAction(add_action),
    };

    let serialized = serde_json::to_string(&entity_action).unwrap();
    assert!(serialized.contains("AddEntityAction"));
    assert!(serialized.contains("new_entity"));

    // Test DeleteEntityAction
    let delete_action = DeleteEntityAction::default();
    let entity_action = EntityAction {
        entity_ref: OSString::literal("old_entity".to_string()),
        action: EntityActionChoice::DeleteEntityAction(delete_action),
    };

    let serialized = serde_json::to_string(&entity_action).unwrap();
    assert!(serialized.contains("DeleteEntityAction"));
    assert!(serialized.contains("old_entity"));
}

fn sample_traffic_definition() -> TrafficDefinition {
    TrafficDefinition::new(
        "TestTrafficDefinition",
        VehicleCategoryDistribution::mixed_traffic(),
        ControllerDistribution::single_controller("TestController".to_string(), 1.0),
    )
}

#[test]
fn test_traffic_action_variants() {
    // Test TrafficSourceAction
    let traffic_action = TrafficAction {
        traffic_name: Some(OSString::literal("source_traffic".to_string())),
        action: TrafficActionChoice::TrafficSourceAction(TrafficSourceAction::new(
            10.0,
            10.0,
            Position::world_origin(),
            sample_traffic_definition(),
        )),
    };

    let serialized = serde_json::to_string(&traffic_action).unwrap();
    assert!(serialized.contains("TrafficSourceAction"));
    assert!(serialized.contains("source_traffic"));

    // Test TrafficSinkAction
    let traffic_action = TrafficAction {
        traffic_name: None,
        action: TrafficActionChoice::TrafficSinkAction(TrafficSinkAction::new(
            10.0,
            50.0,
            Position::world_origin(),
        )),
    };

    let serialized = serde_json::to_string(&traffic_action).unwrap();
    assert!(serialized.contains("TrafficSinkAction"));
    assert!(!serialized.contains("trafficName"));
}

#[test]
fn test_infrastructure_action() {
    let infra_action = InfrastructureAction {
        traffic_signal_action: TrafficSignalAction::state_action(
            "TestSignal".to_string(),
            "green".to_string(),
        ),
    };

    let serialized = serde_json::to_string(&infra_action).unwrap();
    assert!(serialized.contains("TrafficSignalAction"));
}

#[test]
fn test_private_action_variants() {
    // Test LongitudinalAction
    let private_action =
        PrivateAction::LongitudinalAction(LongitudinalAction::speed(SpeedAction::new(
            TransitionDynamics::new(DynamicsDimension::Time, DynamicsShape::Linear, 1.0),
            SpeedActionTarget::absolute(10.0),
        )));
    let serialized = serde_json::to_string(&private_action).unwrap();
    assert!(serialized.contains("LongitudinalAction"));

    // Test LateralAction
    let private_action =
        PrivateAction::LateralAction(LateralAction::lane_change(LaneChangeAction::new(
            TransitionDynamics::new(DynamicsDimension::Time, DynamicsShape::Linear, 1.0),
            LaneChangeTarget::relative("Ego", -1),
        )));
    let serialized = serde_json::to_string(&private_action).unwrap();
    assert!(serialized.contains("LateralAction"));

    // Test VisibilityAction
    let private_action = PrivateAction::VisibilityAction(VisibilityAction::new(true, true, true));
    let serialized = serde_json::to_string(&private_action).unwrap();
    assert!(serialized.contains("VisibilityAction"));

    // Test ControllerAction
    let private_action = PrivateAction::ControllerAction(ControllerAction::empty());
    let serialized = serde_json::to_string(&private_action).unwrap();
    assert!(serialized.contains("ControllerAction"));
}

#[test]
fn test_override_actions() {
    // Test individual override actions exist and can be created
    let brake_action = OverrideBrakeAction {
        active: Boolean::literal(true),
        value: None,
        brake_input: None,
    };
    assert_eq!(brake_action.active, Boolean::literal(true));

    let throttle_action = OverrideThrottleAction {
        active: Boolean::literal(true),
        value: Double::literal(0.5),
        max_rate: Some(Double::literal(1.0)),
    };
    assert_eq!(throttle_action.value, Double::literal(0.5));

    let steering_action = OverrideSteeringWheelAction {
        active: Boolean::literal(true),
        value: Double::literal(0.2),
        max_rate: Some(Double::literal(0.5)),
        max_torque: Some(Double::literal(100.0)),
    };
    assert_eq!(steering_action.value, Double::literal(0.2));

    let gear_action = OverrideGearAction {
        active: Boolean::literal(true),
        number: None,
        gear: None,
    };
    assert_eq!(gear_action.active, Boolean::literal(true));

    let parking_brake_action = OverrideParkingBrakeAction {
        active: Boolean::literal(true),
        value: None,
        brake_input: None,
    };
    assert_eq!(parking_brake_action.active, Boolean::literal(true));

    let clutch_action = OverrideClutchAction {
        active: Boolean::literal(true),
        value: Double::literal(0.8),
        max_rate: Some(Double::literal(2.0)),
    };
    assert_eq!(clutch_action.value, Double::literal(0.8));
}

#[test]
fn test_action_wrapper() {
    let action_wrapper = NamedAction {
        name: OSString::literal("test_action".to_string()),
        action: Action::PrivateAction(PrivateAction::TeleportAction(TeleportAction::new(
            Position::world(WorldPosition::new(1.0, 2.0)),
        ))),
    };

    let serialized = serde_json::to_string(&action_wrapper).unwrap();
    assert!(serialized.contains("test_action"));
    assert!(serialized.contains("PrivateAction"));
}

#[test]
fn test_user_defined_action() {
    let user_action = UserDefinedAction {
        custom_command_action: CustomCommandAction::new("default", ""),
    };

    let core_action = Action::UserDefinedAction(user_action);
    let serialized = serde_json::to_string(&core_action).unwrap();
    assert!(serialized.contains("UserDefinedAction"));
}

#[test]
fn test_new_action_wrapper_types() {
    // Test main NamedAction wrapper
    let action = NamedAction {
        name: OSString::literal("testAction".to_string()),
        action: Action::PrivateAction(PrivateAction::TeleportAction(TeleportAction::new(
            Position::world(WorldPosition::new(1.0, 2.0)),
        ))),
    };

    let serialized = serde_json::to_string(&action).unwrap();
    assert!(serialized.contains("testAction"));
    assert!(serialized.contains("PrivateAction"));

    // Test PrivateAction wrapper (wrapper struct is now just the enum variant)
    let private_action = Action::PrivateAction(PrivateAction::LongitudinalAction(
        LongitudinalAction::speed(SpeedAction::new(
            TransitionDynamics::new(DynamicsDimension::Time, DynamicsShape::Linear, 1.0),
            SpeedActionTarget::absolute(10.0),
        )),
    ));

    let serialized = serde_json::to_string(&private_action).unwrap();
    assert!(serialized.contains("LongitudinalAction"));
}

#[test]
fn test_variable_action_system() {
    // Test VariableSetAction
    let var_set = VariableSetAction {
        value: OSString::literal("42".to_string()),
    };

    let var_action = VariableAction {
        variable_ref: OSString::literal("testVar".to_string()),
        action: VariableActionChoice::VariableSetAction(var_set),
    };

    let serialized = serde_json::to_string(&var_action).unwrap();
    assert!(serialized.contains("testVar"));
    assert!(serialized.contains("42"));

    // Test VariableModifyAction with AddValueRule
    let add_rule = VariableAddValueRule {
        value: Double::literal(10.5),
    };

    let var_modify = VariableModifyAction {
        rule: VariableModifyRule {
            rule: VariableModifyRuleChoice::VariableAddValueRule(add_rule),
        },
    };

    let var_action = VariableAction {
        variable_ref: OSString::literal("modifyVar".to_string()),
        action: VariableActionChoice::VariableModifyAction(var_modify),
    };

    let serialized = serde_json::to_string(&var_action).unwrap();
    assert!(serialized.contains("modifyVar"));
    assert!(serialized.contains("AddValue"));

    // Test VariableMultiplyByValueRule
    let multiply_rule = VariableMultiplyByValueRule {
        value: Double::literal(2.0),
    };

    let var_modify = VariableModifyAction {
        rule: VariableModifyRule {
            rule: VariableModifyRuleChoice::VariableMultiplyByValueRule(multiply_rule),
        },
    };

    let var_action = VariableAction {
        variable_ref: OSString::literal("multiplyVar".to_string()),
        action: VariableActionChoice::VariableModifyAction(var_modify),
    };

    let serialized = serde_json::to_string(&var_action).unwrap();
    assert!(serialized.contains("multiplyVar"));
    assert!(serialized.contains("MultiplyByValue"));
}

#[test]
fn test_parameter_action_system() {
    // Test ParameterSetAction
    let param_set = ParameterSetAction {
        value: OSString::literal("100".to_string()),
    };

    let param_action = ParameterAction {
        parameter_ref: OSString::literal("testParam".to_string()),
        action: ParameterActionChoice::ParameterSetAction(param_set),
    };

    let serialized = serde_json::to_string(&param_action).unwrap();
    assert!(serialized.contains("testParam"));
    assert!(serialized.contains("100"));

    // Test ParameterModifyAction with AddValueRule
    let add_rule = ParameterAddValueRule {
        value: Double::literal(5.0),
    };

    let param_modify = ParameterModifyAction {
        rule: ModifyRule {
            rule: ModifyRuleChoice::ParameterAddValueRule(add_rule),
        },
    };

    let param_action = ParameterAction {
        parameter_ref: OSString::literal("modifyParam".to_string()),
        action: ParameterActionChoice::ParameterModifyAction(param_modify),
    };

    let serialized = serde_json::to_string(&param_action).unwrap();
    assert!(serialized.contains("modifyParam"));
    assert!(serialized.contains("AddValue"));

    // Test ParameterMultiplyByValueRule
    let multiply_rule = ParameterMultiplyByValueRule {
        value: Double::literal(3.0),
    };

    let param_modify = ParameterModifyAction {
        rule: ModifyRule {
            rule: ModifyRuleChoice::ParameterMultiplyByValueRule(multiply_rule),
        },
    };

    let param_action = ParameterAction {
        parameter_ref: OSString::literal("multiplyParam".to_string()),
        action: ParameterActionChoice::ParameterModifyAction(param_modify),
    };

    let serialized = serde_json::to_string(&param_action).unwrap();
    assert!(serialized.contains("multiplyParam"));
    assert!(serialized.contains("MultiplyByValue"));
}

#[test]
fn test_set_monitor_action() {
    // Test SetMonitorAction with monitorRef and value = true
    let monitor_action = SetMonitorAction {
        monitor_ref: OSString::literal("testMonitor".to_string()),
        value: Boolean::literal(true),
    };

    let serialized = serde_json::to_string(&monitor_action).unwrap();
    assert!(serialized.contains("testMonitor"));
    assert!(serialized.contains("true"));

    // Test SetMonitorAction with value = false
    let monitor_action = SetMonitorAction {
        monitor_ref: OSString::literal("otherMonitor".to_string()),
        value: Boolean::literal(false),
    };

    let serialized = serde_json::to_string(&monitor_action).unwrap();
    assert!(serialized.contains("false"));
}

#[test]
fn test_random_route_action() {
    // XSD `RandomRouteAction` (:1813-1814) is an empty complexType.
    let random_route = RandomRouteAction::default();

    let serialized = serde_json::to_string(&random_route).unwrap();
    assert_eq!(serialized, "{}");
}

#[test]
fn test_type_aliases() {
    // (OSR-04, agent D) These types' `Default` impls fabricated content
    // (a name, an enum branch) for XSD `use="required"` attributes/choices
    // and were removed; exercise the named constructors instead.
    let _entity_action: EntityAction = EntityAction::delete("defaultEntity");
    let _infra_action: InfrastructureAction = InfrastructureAction::new(
        TrafficSignalAction::state_action("TestSignal".to_string(), "green".to_string()),
    );
    let _user_action: UserDefinedAction =
        UserDefinedAction::new(CustomCommandAction::new("default", ""));
    let _var_action: VariableAction = VariableAction::new(
        "defaultVariable",
        VariableActionChoice::VariableSetAction(VariableSetAction::new("0")),
    );
    let _param_action: ParameterAction = ParameterAction::new(
        "defaultParameter",
        ParameterActionChoice::ParameterSetAction(ParameterSetAction::new("0")),
    );
    let _monitor_action: SetMonitorAction = SetMonitorAction::new("defaultMonitor", true);
    let _traffic_action: TrafficAction = TrafficAction::new(
        TrafficActionChoice::TrafficStopAction(TrafficStopAction::default()),
    );

    // All should compile without issues
    assert!(true);
}

#[test]
fn test_constructors_and_benign_defaults() {
    // Choice/container structs that default to all-`None` keep their `Default` only when
    // the schema permits the empty form (category 2 in `docs/type_system_guide.md`).
    // `TeleportAction`/`AddEntityAction` do not qualify — XSD `Position` (`:1738-1751`) is a
    // bare `xsd:choice`, so `<Position />` is schema-invalid (category 3, OSR-09). These
    // now name a branch.
    let _global_action = GlobalAction::TrafficAction(TrafficAction::new(
        TrafficActionChoice::TrafficStopAction(TrafficStopAction::default()),
    ));
    let _private_action = PrivateAction::TeleportAction(TeleportAction::new(Position::world(
        WorldPosition::new(1.0, 2.0),
    )));
    let _entity_action = EntityAction::delete("defaultEntity");
    let _traffic_action = TrafficAction::new(TrafficActionChoice::TrafficStopAction(
        TrafficStopAction::default(),
    ));
    let _infra_action = InfrastructureAction::new(TrafficSignalAction::state_action(
        "TestSignal".to_string(),
        "green".to_string(),
    ));
    let _add_entity = AddEntityAction::new(Position::world(WorldPosition::new(3.0, 4.0)));
    let _delete_entity = DeleteEntityAction::default();
    let _user_action = UserDefinedAction::new(CustomCommandAction::new("default", ""));

    // (OSR-04, agent D) `NamedAction` (F13) cannot round-trip its flattened
    // `Action` choice through quick-xml's serializer; it is not exercised
    // for serialization anywhere in the crate, so no constructor is added.
    let _action_wrapper = NamedAction {
        name: OSString::literal("defaultTraffic".to_string()),
        action: Action::PrivateAction(PrivateAction::TeleportAction(TeleportAction::new(
            Position::world(WorldPosition::new(1.0, 2.0)),
        ))),
    };

    // Named per-branch / `::new` constructors for the previously-fabricating types.
    let _action = Action::PrivateAction(PrivateAction::TeleportAction(TeleportAction::new(
        Position::world(WorldPosition::new(1.0, 2.0)),
    )));
    let _private_action_wrapper = PrivateAction::TeleportAction(TeleportAction::new(
        Position::world(WorldPosition::new(1.0, 2.0)),
    ));
    let _monitor_action = SetMonitorAction::new("defaultMonitor", true);
    let _var_action = VariableAction::new(
        "defaultVariable",
        VariableActionChoice::VariableSetAction(VariableSetAction::new("0")),
    );
    let _var_set = VariableSetAction::new("0");
    let _var_modify = VariableModifyAction::new(VariableModifyRule::add_value(0.0));
    let _var_add_rule = VariableAddValueRule::new(0.0);
    let _var_multiply_rule = VariableMultiplyByValueRule::new(1.0);
    let _param_action = ParameterAction::new(
        "defaultParameter",
        ParameterActionChoice::ParameterSetAction(ParameterSetAction::new("0")),
    );
    let _param_set = ParameterSetAction::new("0");
    let _param_modify = ParameterModifyAction::new(ModifyRule::add_value(0.0));
    let _param_add_rule = ParameterAddValueRule::new(0.0);
    let _param_multiply_rule = ParameterMultiplyByValueRule::new(1.0);
    let _random_route = RandomRouteAction::default();

    // All should compile and not panic
    assert!(true);
}
