//! Test suite for Action wrapper types implementation
//!
//! This test verifies that the new Action wrapper types correctly implement
//! the OpenSCENARIO XSD schema structure for actions.

use openscenario_rs::types::actions::{wrappers::*, *};
use openscenario_rs::types::basic::*;
use openscenario_rs::types::positions::*;
use serde_json;

#[test]
fn test_core_action_serialization() {
    // Test PrivateAction serialization
    let private_action = PrivateAction::TeleportAction(TeleportAction::default());
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
        position: Position::default(),
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

#[test]
fn test_traffic_action_variants() {
    // Test TrafficSourceAction
    let traffic_action = TrafficAction {
        traffic_name: Some(OSString::literal("source_traffic".to_string())),
        action: TrafficActionChoice::TrafficSourceAction(TrafficSourceAction::default()),
    };

    let serialized = serde_json::to_string(&traffic_action).unwrap();
    assert!(serialized.contains("TrafficSourceAction"));
    assert!(serialized.contains("source_traffic"));

    // Test TrafficSinkAction
    let traffic_action = TrafficAction {
        traffic_name: None,
        action: TrafficActionChoice::TrafficSinkAction(TrafficSinkAction::default()),
    };

    let serialized = serde_json::to_string(&traffic_action).unwrap();
    assert!(serialized.contains("TrafficSinkAction"));
    assert!(!serialized.contains("trafficName"));
}

#[test]
fn test_infrastructure_action() {
    let infra_action = InfrastructureAction {
        traffic_signal_action: TrafficSignalAction::default(),
    };

    let serialized = serde_json::to_string(&infra_action).unwrap();
    assert!(serialized.contains("TrafficSignalAction"));
}

#[test]
fn test_private_action_variants() {
    // Test LongitudinalAction
    let private_action = PrivateAction::LongitudinalAction(LongitudinalAction::default());
    let serialized = serde_json::to_string(&private_action).unwrap();
    assert!(serialized.contains("LongitudinalAction"));

    // Test LateralAction
    let private_action = PrivateAction::LateralAction(LateralAction::default());
    let serialized = serde_json::to_string(&private_action).unwrap();
    assert!(serialized.contains("LateralAction"));

    // Test VisibilityAction
    let private_action = PrivateAction::VisibilityAction(VisibilityAction::default());
    let serialized = serde_json::to_string(&private_action).unwrap();
    assert!(serialized.contains("VisibilityAction"));

    // Test ControllerAction
    let private_action = PrivateAction::ControllerAction(ControllerAction::default());
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
        action: Action::PrivateAction(PrivateAction::TeleportAction(TeleportAction::default())),
    };

    let serialized = serde_json::to_string(&action_wrapper).unwrap();
    assert!(serialized.contains("test_action"));
    assert!(serialized.contains("PrivateAction"));
}

#[test]
fn test_user_defined_action() {
    let user_action = UserDefinedAction {
        custom_command_action: CustomCommandAction::default(),
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
        action: Action::PrivateAction(PrivateAction::TeleportAction(TeleportAction::default())),
    };

    let serialized = serde_json::to_string(&action).unwrap();
    assert!(serialized.contains("testAction"));
    assert!(serialized.contains("PrivateAction"));

    // Test PrivateAction wrapper (wrapper struct is now just the enum variant)
    let private_action = Action::PrivateAction(PrivateAction::LongitudinalAction(
        LongitudinalAction::default(),
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
        rule: VariableModifyRule::VariableAddValueRule(add_rule),
    };

    let var_action = VariableAction {
        variable_ref: OSString::literal("modifyVar".to_string()),
        action: VariableActionChoice::VariableModifyAction(var_modify),
    };

    let serialized = serde_json::to_string(&var_action).unwrap();
    assert!(serialized.contains("modifyVar"));
    assert!(serialized.contains("VariableAddValueRule"));

    // Test VariableMultiplyByValueRule
    let multiply_rule = VariableMultiplyByValueRule {
        value: Double::literal(2.0),
    };

    let var_modify = VariableModifyAction {
        rule: VariableModifyRule::VariableMultiplyByValueRule(multiply_rule),
    };

    let var_action = VariableAction {
        variable_ref: OSString::literal("multiplyVar".to_string()),
        action: VariableActionChoice::VariableModifyAction(var_modify),
    };

    let serialized = serde_json::to_string(&var_action).unwrap();
    assert!(serialized.contains("multiplyVar"));
    assert!(serialized.contains("VariableMultiplyByValueRule"));
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
        rule: ModifyRule::ParameterAddValueRule(add_rule),
    };

    let param_action = ParameterAction {
        parameter_ref: OSString::literal("modifyParam".to_string()),
        action: ParameterActionChoice::ParameterModifyAction(param_modify),
    };

    let serialized = serde_json::to_string(&param_action).unwrap();
    assert!(serialized.contains("modifyParam"));
    assert!(serialized.contains("ParameterAddValueRule"));

    // Test ParameterMultiplyByValueRule
    let multiply_rule = ParameterMultiplyByValueRule {
        value: Double::literal(3.0),
    };

    let param_modify = ParameterModifyAction {
        rule: ModifyRule::ParameterMultiplyByValueRule(multiply_rule),
    };

    let param_action = ParameterAction {
        parameter_ref: OSString::literal("multiplyParam".to_string()),
        action: ParameterActionChoice::ParameterModifyAction(param_modify),
    };

    let serialized = serde_json::to_string(&param_action).unwrap();
    assert!(serialized.contains("multiplyParam"));
    assert!(serialized.contains("ParameterMultiplyByValueRule"));
}

#[test]
fn test_set_monitor_action() {
    // Test SetMonitorAction with monitor name
    let monitor_action = SetMonitorAction {
        enable: Boolean::literal(true),
        monitor_name: Some(OSString::literal("testMonitor".to_string())),
    };

    let serialized = serde_json::to_string(&monitor_action).unwrap();
    assert!(serialized.contains("testMonitor"));
    assert!(serialized.contains("true"));

    // Test SetMonitorAction without monitor name
    let monitor_action = SetMonitorAction {
        enable: Boolean::literal(false),
        monitor_name: None,
    };

    let serialized = serde_json::to_string(&monitor_action).unwrap();
    assert!(serialized.contains("false"));
    assert!(!serialized.contains("monitorName"));
}

#[test]
fn test_random_route_action() {
    // Test RandomRouteAction with all fields
    let random_route = RandomRouteAction {
        number_of_routes: Some(UnsignedInt::literal(5)),
        random_seed: Some(UnsignedInt::literal(12345)),
    };

    let serialized = serde_json::to_string(&random_route).unwrap();
    assert!(serialized.contains("5"));
    assert!(serialized.contains("12345"));

    // Test RandomRouteAction with no fields
    let random_route = RandomRouteAction {
        number_of_routes: None,
        random_seed: None,
    };

    let serialized = serde_json::to_string(&random_route).unwrap();
    assert!(!serialized.contains("numberOfRoutes"));
    assert!(!serialized.contains("randomSeed"));
}

#[test]
fn test_type_aliases() {
    // Test that type aliases work correctly
    let _entity_action: EntityAction = EntityAction::default();
    let _infra_action: InfrastructureAction = InfrastructureAction::default();
    let _user_action: UserDefinedAction = UserDefinedAction::default();
    let _var_action: VariableAction = VariableAction::default();
    let _param_action: ParameterAction = ParameterAction::default();
    let _monitor_action: SetMonitorAction = SetMonitorAction::default();
    let _traffic_action: TrafficAction = TrafficAction::default();

    // All should compile without issues
    assert!(true);
}

#[test]
fn test_default_implementations() {
    // Test all Default implementations work
    let _core_action = Action::default();
    let _global_action = GlobalAction::default();
    let _private_action = PrivateAction::default();
    let _entity_action = EntityAction::default();
    let _traffic_action = TrafficAction::default();
    let _infra_action = InfrastructureAction::default();
    let _add_entity = AddEntityAction::default();
    let _delete_entity = DeleteEntityAction::default();
    let _user_action = UserDefinedAction::default();
    let _action_wrapper = NamedAction::default();

    // Test new Default implementations
    let _action = Action::default();
    let _private_action_wrapper = PrivateAction::default();
    let _monitor_action = SetMonitorAction::default();
    let _var_action = VariableAction::default();
    let _var_set = VariableSetAction::default();
    let _var_modify = VariableModifyAction::default();
    let _var_add_rule = VariableAddValueRule::default();
    let _var_multiply_rule = VariableMultiplyByValueRule::default();
    let _param_action = ParameterAction::default();
    let _param_set = ParameterSetAction::default();
    let _param_modify = ParameterModifyAction::default();
    let _param_add_rule = ParameterAddValueRule::default();
    let _param_multiply_rule = ParameterMultiplyByValueRule::default();
    let _random_route = RandomRouteAction::default();

    // All should compile and not panic
    assert!(true);
}
