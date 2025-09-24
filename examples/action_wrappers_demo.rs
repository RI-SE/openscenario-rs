//! Demonstration of Action Wrapper Types
//!
//! This example shows how to use the new action wrapper types that match
//! the OpenSCENARIO XSD schema structure.

use openscenario_rs::types::actions::{wrappers::*, *};
use openscenario_rs::types::basic::*;
use openscenario_rs::types::positions::*;

fn main() {
    println!("=== OpenSCENARIO Action Wrapper Types Demo ===\n");

    // Demonstrate CoreAction with PrivateAction
    demonstrate_private_actions();
    
    // Demonstrate CoreAction with GlobalAction
    demonstrate_global_actions();
    
    // Demonstrate individual Override actions
    demonstrate_override_actions();
    
    // Demonstrate Entity actions
    demonstrate_entity_actions();
    
    // Demonstrate Traffic actions
    demonstrate_traffic_actions();
}

fn demonstrate_private_actions() {
    println!("1. Private Actions:");
    
    // Create a TeleportAction wrapped in PrivateAction
    let teleport_action = TeleportAction::default();
    let private_action = CorePrivateAction::TeleportAction(teleport_action);
    let core_action = CoreAction::PrivateAction(private_action);
    
    println!("   - Created TeleportAction wrapped in PrivateAction");
    
    // Create a LongitudinalAction wrapped in PrivateAction
    let longitudinal_action = LongitudinalAction::default();
    let private_action = CorePrivateAction::LongitudinalAction(longitudinal_action);
    let core_action = CoreAction::PrivateAction(private_action);
    
    println!("   - Created LongitudinalAction wrapped in PrivateAction");
    
    // Create a ControllerAction wrapped in PrivateAction
    let controller_action = ControllerAction::default();
    let private_action = CorePrivateAction::ControllerAction(controller_action);
    let core_action = CoreAction::PrivateAction(private_action);
    
    println!("   - Created ControllerAction wrapped in PrivateAction\n");
}

fn demonstrate_global_actions() {
    println!("2. Global Actions:");
    
    // Create a TrafficAction wrapped in GlobalAction
    let traffic_action = CoreTrafficAction {
        traffic_name: Some(OSString::from("highway_traffic")),
        action: CoreTrafficActionChoice::TrafficSourceAction(TrafficSourceAction::default()),
    };
    let global_action = CoreGlobalAction::TrafficAction(traffic_action);
    let core_action = CoreAction::GlobalAction(global_action);
    
    println!("   - Created TrafficSourceAction wrapped in GlobalAction");
    
    // Create an EntityAction wrapped in GlobalAction
    let entity_action = CoreEntityAction {
        entity_ref: OSString::from("vehicle_001"),
        action: CoreEntityActionChoice::DeleteEntityAction(CoreDeleteEntityAction::default()),
    };
    let global_action = CoreGlobalAction::EntityAction(entity_action);
    let core_action = CoreAction::GlobalAction(global_action);
    
    println!("   - Created EntityAction wrapped in GlobalAction");
    
    // Create an InfrastructureAction wrapped in GlobalAction
    let infra_action = CoreInfrastructureAction {
        traffic_signal_action: TrafficSignalAction::default(),
    };
    let global_action = CoreGlobalAction::InfrastructureAction(infra_action);
    let core_action = CoreAction::GlobalAction(global_action);
    
    println!("   - Created InfrastructureAction wrapped in GlobalAction\n");
}

fn demonstrate_override_actions() {
    println!("3. Override Actions (XSD compliant names):");
    
    // Create individual override actions
    let brake_override = OverrideBrakeAction {
        active: Boolean::from(true),
        value: Some(Double::from(0.8)),
        brake_input: None,
    };
    println!("   - Created OverrideBrakeAction (active: {}, value: {:?})", 
             brake_override.active, brake_override.value);
    
    let throttle_override = OverrideThrottleAction {
        active: Boolean::from(true),
        value: Double::from(0.6),
        max_rate: Some(Double::from(2.0)),
    };
    println!("   - Created OverrideThrottleAction (active: {}, value: {}, max_rate: {:?})", 
             throttle_override.active, throttle_override.value, throttle_override.max_rate);
    
    let steering_override = OverrideSteeringWheelAction {
        active: Boolean::from(true),
        value: Double::from(0.3),
        max_rate: Some(Double::from(1.5)),
        max_torque: Some(Double::from(150.0)),
    };
    println!("   - Created OverrideSteeringWheelAction (active: {}, value: {}, max_torque: {:?})", 
             steering_override.active, steering_override.value, steering_override.max_torque);
    
    let gear_override = OverrideGearAction {
        active: Boolean::from(true),
        number: Some(Double::from(3.0)),
        gear: None,
    };
    println!("   - Created OverrideGearAction (active: {}, number: {:?})", 
             gear_override.active, gear_override.number);
    
    let parking_brake_override = OverrideParkingBrakeAction {
        active: Boolean::from(true),
        value: Some(Double::from(1.0)),
        brake_input: None,
    };
    println!("   - Created OverrideParkingBrakeAction (active: {}, value: {:?})", 
             parking_brake_override.active, parking_brake_override.value);
    
    let clutch_override = OverrideClutchAction {
        active: Boolean::from(true),
        value: Double::from(0.9),
        max_rate: Some(Double::from(3.0)),
    };
    println!("   - Created OverrideClutchAction (active: {}, value: {}, max_rate: {:?})\n", 
             clutch_override.active, clutch_override.value, clutch_override.max_rate);
}

fn demonstrate_entity_actions() {
    println!("4. Entity Actions:");
    
    // Create AddEntityAction
    let add_entity = CoreAddEntityAction {
        position: Position::default(),
    };
    let entity_action = CoreEntityAction {
        entity_ref: OSString::from("new_vehicle"),
        action: CoreEntityActionChoice::AddEntityAction(add_entity),
    };
    println!("   - Created AddEntityAction for entity: {}", entity_action.entity_ref);
    
    // Create DeleteEntityAction
    let delete_entity = CoreDeleteEntityAction::default();
    let entity_action = CoreEntityAction {
        entity_ref: OSString::from("old_vehicle"),
        action: CoreEntityActionChoice::DeleteEntityAction(delete_entity),
    };
    println!("   - Created DeleteEntityAction for entity: {}\n", entity_action.entity_ref);
}

fn demonstrate_traffic_actions() {
    println!("5. Traffic Actions:");
    
    // Create TrafficSourceAction
    let traffic_action = CoreTrafficAction {
        traffic_name: Some(OSString::from("city_traffic")),
        action: CoreTrafficActionChoice::TrafficSourceAction(TrafficSourceAction::default()),
    };
    println!("   - Created TrafficSourceAction with name: {:?}", traffic_action.traffic_name);
    
    // Create TrafficSinkAction
    let traffic_action = CoreTrafficAction {
        traffic_name: None,
        action: CoreTrafficActionChoice::TrafficSinkAction(TrafficSinkAction::default()),
    };
    println!("   - Created TrafficSinkAction with no name");
    
    // Create TrafficSwarmAction
    let traffic_action = CoreTrafficAction {
        traffic_name: Some(OSString::from("swarm_traffic")),
        action: CoreTrafficActionChoice::TrafficSwarmAction(TrafficSwarmAction::default()),
    };
    println!("   - Created TrafficSwarmAction with name: {:?}", traffic_action.traffic_name);
    
    // Create TrafficAreaAction
    let traffic_action = CoreTrafficAction {
        traffic_name: Some(OSString::from("area_traffic")),
        action: CoreTrafficActionChoice::TrafficAreaAction(TrafficAreaAction::default()),
    };
    println!("   - Created TrafficAreaAction with name: {:?}", traffic_action.traffic_name);
    
    // Create TrafficStopAction
    let traffic_action = CoreTrafficAction {
        traffic_name: Some(OSString::from("stop_traffic")),
        action: CoreTrafficActionChoice::TrafficStopAction(TrafficStopAction::default()),
    };
    println!("   - Created TrafficStopAction with name: {:?}\n", traffic_action.traffic_name);
}