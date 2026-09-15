//! Demonstration of Action Wrapper Types
//!
//! This example shows how to use the new action wrapper types that match
//! the OpenSCENARIO XSD schema structure.

use openscenario_rs::types::actions::movement::{SpeedActionTarget, TransitionDynamics};
use openscenario_rs::types::actions::{wrappers::*, *};
use openscenario_rs::types::basic::*;
use openscenario_rs::types::entities::{EntityDistribution, EntityDistributionEntry};
use openscenario_rs::types::enums::{DynamicsDimension, DynamicsShape};
use openscenario_rs::types::positions::*;

fn sample_traffic_definition() -> TrafficDefinition {
    TrafficDefinition::new(
        "DemoTrafficDefinition",
        VehicleCategoryDistribution::mixed_traffic(),
        ControllerDistribution::single_controller("DemoController".to_string(), 1.0),
    )
}

fn main() {
    println!("=== OpenSCENARIO Action Wrapper Types Demo ===\n");

    // Demonstrate Action with PrivateAction
    demonstrate_private_actions();

    // Demonstrate Action with GlobalAction
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
    let teleport_action = TeleportAction::new(Position::world(WorldPosition::new(100.0, 50.0)));
    let private_action = PrivateAction::TeleportAction(teleport_action);
    let core_action = Action::PrivateAction(private_action);

    println!("   - Created TeleportAction wrapped in PrivateAction");

    // Create a LongitudinalAction wrapped in PrivateAction
    let longitudinal_action = LongitudinalAction::speed(SpeedAction::new(
        TransitionDynamics::new(DynamicsDimension::Time, DynamicsShape::Linear, 1.0),
        SpeedActionTarget::absolute(10.0),
    ));
    let private_action = PrivateAction::LongitudinalAction(longitudinal_action);
    let core_action = Action::PrivateAction(private_action);

    println!("   - Created LongitudinalAction wrapped in PrivateAction");

    // Create a ControllerAction wrapped in PrivateAction
    let controller_action = ControllerAction::empty();
    let private_action = PrivateAction::ControllerAction(controller_action);
    let core_action = Action::PrivateAction(private_action);

    println!("   - Created ControllerAction wrapped in PrivateAction\n");
}

fn demonstrate_global_actions() {
    println!("2. Global Actions:");

    // Create a TrafficAction wrapped in GlobalAction
    let traffic_action = TrafficAction {
        traffic_name: Some(Value::Literal("highway_traffic".to_string())),
        action: TrafficActionChoice::TrafficSourceAction(TrafficSourceAction::new(
            10.0,
            10.0,
            Position::world_origin(),
            sample_traffic_definition(),
        )),
    };
    let global_action = GlobalAction::TrafficAction(traffic_action);
    let action = Action::GlobalAction(global_action);

    println!("   - Created TrafficSourceAction wrapped in GlobalAction");

    // Create an EntityAction wrapped in GlobalAction
    let entity_action = EntityAction {
        entity_ref: Value::Literal("vehicle_001".to_string()),
        action: EntityActionChoice::DeleteEntityAction(DeleteEntityAction::default()),
    };
    let global_action = GlobalAction::EntityAction(entity_action);
    let core_action = Action::GlobalAction(global_action);

    println!("   - Created EntityAction wrapped in GlobalAction");

    // Create an InfrastructureAction wrapped in GlobalAction
    let infra_action = InfrastructureAction {
        traffic_signal_action: TrafficSignalAction::state_action(
            "DemoSignal".to_string(),
            "green".to_string(),
        ),
    };
    let global_action = GlobalAction::InfrastructureAction(infra_action);
    let core_action = Action::GlobalAction(global_action);

    println!("   - Created InfrastructureAction wrapped in GlobalAction\n");
}

fn demonstrate_override_actions() {
    println!("3. Override Actions (XSD compliant names):");

    // Create individual override actions
    let brake_override = OverrideBrakeAction {
        active: Value::Literal(true),
        value: Some(Value::Literal(0.8)),
        brake_input: None,
    };
    println!(
        "   - Created OverrideBrakeAction (active: {}, value: {:?})",
        brake_override.active, brake_override.value
    );

    let throttle_override = OverrideThrottleAction {
        active: Value::Literal(true),
        value: Value::Literal(0.6),
        max_rate: Some(Value::Literal(2.0)),
    };
    println!(
        "   - Created OverrideThrottleAction (active: {}, value: {}, max_rate: {:?})",
        throttle_override.active, throttle_override.value, throttle_override.max_rate
    );

    let steering_override = OverrideSteeringWheelAction {
        active: Value::Literal(true),
        value: Value::Literal(0.3),
        max_rate: Some(Value::Literal(1.5)),
        max_torque: Some(Value::Literal(150.0)),
    };
    println!(
        "   - Created OverrideSteeringWheelAction (active: {}, value: {}, max_torque: {:?})",
        steering_override.active, steering_override.value, steering_override.max_torque
    );

    let gear_override = OverrideGearAction {
        active: Value::Literal(true),
        number: Some(Value::Literal(3.0)),
        gear: None,
    };
    println!(
        "   - Created OverrideGearAction (active: {}, number: {:?})",
        gear_override.active, gear_override.number
    );

    let parking_brake_override = OverrideParkingBrakeAction {
        active: Value::Literal(true),
        value: Some(Value::Literal(1.0)),
        brake_input: None,
    };
    println!(
        "   - Created OverrideParkingBrakeAction (active: {}, value: {:?})",
        parking_brake_override.active, parking_brake_override.value
    );

    let clutch_override = OverrideClutchAction {
        active: Value::Literal(true),
        value: Value::Literal(0.9),
        max_rate: Some(Value::Literal(3.0)),
    };
    println!(
        "   - Created OverrideClutchAction (active: {}, value: {}, max_rate: {:?})\n",
        clutch_override.active, clutch_override.value, clutch_override.max_rate
    );
}

fn demonstrate_entity_actions() {
    println!("4. Entity Actions:");

    // Create AddEntityAction
    let add_entity = AddEntityAction {
        position: Position::world_origin(),
    };
    let entity_action = EntityAction {
        entity_ref: Value::Literal("new_vehicle".to_string()),
        action: EntityActionChoice::AddEntityAction(add_entity),
    };
    println!(
        "   - Created AddEntityAction for entity: {}",
        entity_action.entity_ref
    );

    // Create DeleteEntityAction
    let delete_entity = DeleteEntityAction::default();
    let entity_action = EntityAction {
        entity_ref: Value::Literal("old_vehicle".to_string()),
        action: EntityActionChoice::DeleteEntityAction(delete_entity),
    };
    println!(
        "   - Created DeleteEntityAction for entity: {}\n",
        entity_action.entity_ref
    );
}

fn demonstrate_traffic_actions() {
    println!("5. Traffic Actions:");

    // Create TrafficSourceAction
    let traffic_action = TrafficAction {
        traffic_name: Some(Value::Literal("city_traffic".to_string())),
        action: TrafficActionChoice::TrafficSourceAction(TrafficSourceAction::new(
            10.0,
            10.0,
            Position::world_origin(),
            sample_traffic_definition(),
        )),
    };
    println!(
        "   - Created TrafficSourceAction with name: {:?}",
        traffic_action.traffic_name
    );

    // Create TrafficSinkAction
    let traffic_action = TrafficAction {
        traffic_name: None,
        action: TrafficActionChoice::TrafficSinkAction(TrafficSinkAction::new(
            10.0,
            50.0,
            Position::world_origin(),
        )),
    };
    println!("   - Created TrafficSinkAction with no name");

    // Create TrafficSwarmAction
    let traffic_action = TrafficAction {
        traffic_name: Some(Value::Literal("swarm_traffic".to_string())),
        action: TrafficActionChoice::TrafficSwarmAction(TrafficSwarmAction::new(
            "SwarmCenter",
            100.0,
            50.0,
            20,
        )),
    };
    println!(
        "   - Created TrafficSwarmAction with name: {:?}",
        traffic_action.traffic_name
    );

    // Create TrafficAreaAction
    use openscenario_rs::types::entities::{ScenarioObjectTemplate, Vehicle};
    let entity_distribution = EntityDistribution {
        entries: vec![EntityDistributionEntry::new(
            ScenarioObjectTemplate::new_vehicle(Vehicle::new_car("TestVehicle".to_string())),
            1.0,
        )],
    };
    let traffic_action = TrafficAction {
        traffic_name: Some(Value::Literal("area_traffic".to_string())),
        action: TrafficActionChoice::TrafficAreaAction(TrafficAreaAction::new(
            5,
            true,
            TrafficDistribution::new(vec![TrafficDistributionEntry::new(
                1.0,
                entity_distribution,
            )]),
            TrafficArea::rectangle(0.0, 0.0, 50.0, 50.0),
        )),
    };
    println!(
        "   - Created TrafficAreaAction with name: {:?}",
        traffic_action.traffic_name
    );

    // Create TrafficStopAction
    let traffic_action = TrafficAction {
        traffic_name: Some(Value::Literal("stop_traffic".to_string())),
        action: TrafficActionChoice::TrafficStopAction(TrafficStopAction::default()),
    };
    println!(
        "   - Created TrafficStopAction with name: {:?}\n",
        traffic_action.traffic_name
    );
}
