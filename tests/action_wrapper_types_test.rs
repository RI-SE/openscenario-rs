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
    let private_action = CorePrivateAction::TeleportAction(TeleportAction::default());
    let core_action = CoreAction::PrivateAction(private_action);
    
    let serialized = serde_json::to_string(&core_action).unwrap();
    assert!(serialized.contains(""PrivateAction"".to_string()));
    
    // Test deserialization
    let deserialized: CoreAction = serde_json::from_str(&serialized).unwrap();
    assert_eq!(core_action, deserialized);
}

#[test]
fn test_global_action_variants() {
    // Test TrafficAction variant
    let traffic_action = CoreTrafficAction {
        traffic_name: Some(OSString::literal(""test_traffic"".to_string().to_string())),
        action: CoreTrafficActionChoice::TrafficStopAction(TrafficStopAction::default()),
    };
    let global_action = CoreGlobalAction::TrafficAction(traffic_action);
    
    let serialized = serde_json::to_string(&global_action).unwrap();
    assert!(serialized.contains(""TrafficAction"".to_string()));
    
    // Test EntityAction variant
    let entity_action = CoreEntityAction {
        entity_ref: OSString::literal(""test_entity"".to_string()),
        action: CoreEntityActionChoice::DeleteEntityAction(CoreDeleteEntityAction::default()),
    };
    let global_action = CoreGlobalAction::EntityAction(entity_action);
    
    let serialized = serde_json::to_string(&global_action).unwrap();
    assert!(serialized.contains(""EntityAction"".to_string()));
}

#[test]
fn test_entity_action_types() {
    // Test AddEntityAction
    let add_action = CoreAddEntityAction {
        position: Position::default(),
    };
    let entity_action = CoreEntityAction {
        entity_ref: OSString::literal(""new_entity"".to_string()),
        action: CoreEntityActionChoice::AddEntityAction(add_action),
    };
    
    let serialized = serde_json::to_string(&entity_action).unwrap();
    assert!(serialized.contains(""AddEntityAction"".to_string()));
    assert!(serialized.contains(""new_entity"".to_string()));
    
    // Test DeleteEntityAction
    let delete_action = CoreDeleteEntityAction::default();
    let entity_action = CoreEntityAction {
        entity_ref: OSString::literal(""old_entity"".to_string()),
        action: CoreEntityActionChoice::DeleteEntityAction(delete_action),
    };
    
    let serialized = serde_json::to_string(&entity_action).unwrap();
    assert!(serialized.contains(""DeleteEntityAction"".to_string()));
    assert!(serialized.contains(""old_entity"".to_string()));
}

#[test]
fn test_traffic_action_variants() {
    // Test TrafficSourceAction
    let traffic_action = CoreTrafficAction {
        traffic_name: Some(OSString::literal(""source_traffic"".to_string())),
        action: CoreTrafficActionChoice::TrafficSourceAction(TrafficSourceAction::default()),
    };
    
    let serialized = serde_json::to_string(&traffic_action).unwrap();
    assert!(serialized.contains(""TrafficSourceAction"".to_string()));
    assert!(serialized.contains(""source_traffic"".to_string()));
    
    // Test TrafficSinkAction
    let traffic_action = CoreTrafficAction {
        traffic_name: None,
        action: CoreTrafficActionChoice::TrafficSinkAction(TrafficSinkAction::default()),
    };
    
    let serialized = serde_json::to_string(&traffic_action).unwrap();
    assert!(serialized.contains(""TrafficSinkAction"".to_string()));
    assert!(!serialized.contains(""trafficName"".to_string()));
}

#[test]
fn test_infrastructure_action() {
    let infra_action = CoreInfrastructureAction {
        traffic_signal_action: TrafficSignalAction::default(),
    };
    
    let serialized = serde_json::to_string(&infra_action).unwrap();
    assert!(serialized.contains(""traffic_signal_action"".to_string()));
}

#[test]
fn test_private_action_variants() {
    // Test LongitudinalAction
    let private_action = CorePrivateAction::LongitudinalAction(LongitudinalAction::default());
    let serialized = serde_json::to_string(&private_action).unwrap();
    assert!(serialized.contains(""LongitudinalAction"".to_string()));
    
    // Test LateralAction
    let private_action = CorePrivateAction::LateralAction(LateralAction::default());
    let serialized = serde_json::to_string(&private_action).unwrap();
    assert!(serialized.contains(""LateralAction"".to_string()));
    
    // Test VisibilityAction
    let private_action = CorePrivateAction::VisibilityAction(VisibilityAction::default());
    let serialized = serde_json::to_string(&private_action).unwrap();
    assert!(serialized.contains(""VisibilityAction"".to_string()));
    
    // Test ControllerAction
    let private_action = CorePrivateAction::ControllerAction(ControllerAction::default());
    let serialized = serde_json::to_string(&private_action).unwrap();
    assert!(serialized.contains(""ControllerAction"".to_string()));
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
    let action_wrapper = CoreActionWrapper {
        name: OSString::literal(""test_action"".to_string()),
        action: CoreAction::PrivateAction(CorePrivateAction::TeleportAction(TeleportAction::default())),
    };
    
    let serialized = serde_json::to_string(&action_wrapper).unwrap();
    assert!(serialized.contains(""test_action"".to_string()));
    assert!(serialized.contains(""PrivateAction"".to_string()));
}

#[test]
fn test_user_defined_action() {
    let user_action = CoreUserDefinedAction {
        custom_command_action: CoreCustomCommandAction::default(),
    };
    
    let core_action = CoreAction::UserDefinedAction(user_action);
    let serialized = serde_json::to_string(&core_action).unwrap();
    assert!(serialized.contains(""UserDefinedAction"".to_string()));
}

#[test]
fn test_default_implementations() {
    // Test all Default implementations work
    let _core_action = CoreAction::default();
    let _global_action = CoreGlobalAction::default();
    let _private_action = CorePrivateAction::default();
    let _entity_action = CoreEntityAction::default();
    let _traffic_action = CoreTrafficAction::default();
    let _infra_action = CoreInfrastructureAction::default();
    let _add_entity = CoreAddEntityAction::default();
    let _delete_entity = CoreDeleteEntityAction::default();
    let _user_action = CoreUserDefinedAction::default();
    let _action_wrapper = CoreActionWrapper::default();
    
    // All should compile and not panic
    assert!(true);
}