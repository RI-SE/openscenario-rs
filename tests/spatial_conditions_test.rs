//! Integration tests for spatial conditions
//!
//! Tests position and distance-based condition functionality including:
//! - ReachPositionCondition with tolerance validation
//! - DistanceCondition with coordinate systems and routing
//! - RelativeDistanceCondition with different measurement types
//! - XML serialization/deserialization round-trips
//! - Builder pattern functionality and ergonomic APIs

use openscenario_rs::types::{
    basic::{Boolean, Double, OSString},
    conditions::{DistanceCondition, ReachPositionCondition, RelativeDistanceCondition},
    enums::{CoordinateSystem, RelativeDistanceType, RoutingAlgorithm, Rule},
    positions::{Position, WorldPosition},
};

#[test]
fn test_reach_position_condition_basic() {
    let world_pos = WorldPosition::with_full_orientation(100.0, 200.0, 0.0, 1.57, 0.0, 0.0);
    let mut position = Position::default();
    position.world_position = Some(world_pos);
    position.relative_world_position = None;
    position.road_position = None;
    position.lane_position = None;

    let condition = ReachPositionCondition::new(position, 2.5);

    assert_eq!(condition.tolerance, Double::literal(2.5));
    assert!(condition.position.world_position.is_some());

    let world_pos = condition.position.world_position.unwrap();
    assert_eq!(world_pos.x, Double::literal(100.0));
    assert_eq!(world_pos.y, Double::literal(200.0));
    assert_eq!(world_pos.h, Some(Double::literal(1.57)));
}

#[test]
fn test_reach_position_condition_builder() {
    let condition = ReachPositionCondition::at_world_position(50.0, 75.0, 1.0, 0.0, 1.5);

    assert_eq!(condition.tolerance, Double::literal(1.5));
    assert!(condition.position.world_position.is_some());

    let world_pos = condition.position.world_position.unwrap();
    assert_eq!(world_pos.x, Double::literal(50.0));
    assert_eq!(world_pos.y, Double::literal(75.0));
    assert_eq!(world_pos.z, Some(Double::literal(1.0)));
    assert_eq!(world_pos.h, Some(Double::literal(0.0)));
}

#[test]
fn test_distance_condition_basic() {
    let position = Position::default();
    let condition = DistanceCondition::new(position, 25.0, true, Rule::LessThan);

    assert_eq!(condition.value, Double::literal(25.0));
    assert_eq!(condition.freespace, Boolean::literal(true));
    assert_eq!(condition.rule, Rule::LessThan);
    assert!(condition.coordinate_system.is_none());
    assert!(condition.relative_distance_type.is_none());
    assert!(condition.routing_algorithm.is_none());
}

#[test]
fn test_distance_condition_builder() {
    let position = Position::default();
    let condition = DistanceCondition::less_than(position, 30.0, false)
        .with_coordinate_system(CoordinateSystem::Road)
        .with_distance_type(RelativeDistanceType::Longitudinal)
        .with_routing_algorithm(RoutingAlgorithm::Shortest);

    assert_eq!(condition.value, Double::literal(30.0));
    assert_eq!(condition.freespace, Boolean::literal(false));
    assert_eq!(condition.rule, Rule::LessThan);
    assert_eq!(condition.coordinate_system, Some(CoordinateSystem::Road));
    assert_eq!(
        condition.relative_distance_type,
        Some(RelativeDistanceType::Longitudinal)
    );
    assert_eq!(
        condition.routing_algorithm,
        Some(RoutingAlgorithm::Shortest)
    );
}

#[test]
fn test_distance_condition_convenience_methods() {
    let position = Position::default();

    let less_than = DistanceCondition::less_than(position.clone(), 15.0, true);
    assert_eq!(less_than.rule, Rule::LessThan);
    assert_eq!(less_than.value, Double::literal(15.0));
    assert_eq!(less_than.freespace, Boolean::literal(true));

    let greater_than = DistanceCondition::greater_than(position, 20.0, false);
    assert_eq!(greater_than.rule, Rule::GreaterThan);
    assert_eq!(greater_than.value, Double::literal(20.0));
    assert_eq!(greater_than.freespace, Boolean::literal(false));
}

#[test]
fn test_relative_distance_condition_basic() {
    let condition = RelativeDistanceCondition::new(
        "target_vehicle",
        12.5,
        true,
        RelativeDistanceType::Cartesian,
        Rule::GreaterThan,
    );

    assert_eq!(
        condition.entity_ref,
        OSString::literal("target_vehicle".to_string())
    );
    assert_eq!(condition.value, Double::literal(12.5));
    assert_eq!(condition.freespace, Boolean::literal(true));
    assert_eq!(
        condition.relative_distance_type,
        RelativeDistanceType::Cartesian
    );
    assert_eq!(condition.rule, Rule::GreaterThan);
    assert!(condition.coordinate_system.is_none());
    assert!(condition.routing_algorithm.is_none());
}

#[test]
fn test_relative_distance_condition_types() {
    let longitudinal =
        RelativeDistanceCondition::longitudinal("vehicle1", 20.0, false, Rule::LessThan);
    assert_eq!(
        longitudinal.relative_distance_type,
        RelativeDistanceType::Longitudinal
    );
    assert_eq!(
        longitudinal.entity_ref,
        OSString::literal("vehicle1".to_string())
    );
    assert_eq!(longitudinal.value, Double::literal(20.0));
    assert_eq!(longitudinal.freespace, Boolean::literal(false));
    assert_eq!(longitudinal.rule, Rule::LessThan);

    let lateral = RelativeDistanceCondition::lateral("vehicle2", 5.0, true, Rule::GreaterThan);
    assert_eq!(
        lateral.relative_distance_type,
        RelativeDistanceType::Lateral
    );
    assert_eq!(
        lateral.entity_ref,
        OSString::literal("vehicle2".to_string())
    );
    assert_eq!(lateral.value, Double::literal(5.0));
    assert_eq!(lateral.freespace, Boolean::literal(true));
    assert_eq!(lateral.rule, Rule::GreaterThan);

    let cartesian = RelativeDistanceCondition::cartesian("vehicle3", 15.0, false, Rule::EqualTo);
    assert_eq!(
        cartesian.relative_distance_type,
        RelativeDistanceType::Cartesian
    );
    assert_eq!(
        cartesian.entity_ref,
        OSString::literal("vehicle3".to_string())
    );
    assert_eq!(cartesian.value, Double::literal(15.0));
    assert_eq!(cartesian.freespace, Boolean::literal(false));
    assert_eq!(cartesian.rule, Rule::EqualTo);
}

#[test]
fn test_relative_distance_condition_with_options() {
    let condition =
        RelativeDistanceCondition::cartesian("ego_vehicle", 25.0, true, Rule::LessOrEqual)
            .with_coordinate_system(CoordinateSystem::Lane)
            .with_routing_algorithm(RoutingAlgorithm::Fastest);

    assert_eq!(condition.coordinate_system, Some(CoordinateSystem::Lane));
    assert_eq!(condition.routing_algorithm, Some(RoutingAlgorithm::Fastest));
    assert_eq!(
        condition.relative_distance_type,
        RelativeDistanceType::Cartesian
    );
    assert_eq!(condition.rule, Rule::LessOrEqual);
}

#[test]
fn test_spatial_condition_defaults() {
    let reach_pos = ReachPositionCondition::default();
    assert_eq!(reach_pos.tolerance, Double::literal(1.0));
    assert!(reach_pos.position.world_position.is_some());

    let distance = DistanceCondition::default();
    assert_eq!(distance.value, Double::literal(10.0));
    assert_eq!(distance.freespace, Boolean::literal(true));
    assert_eq!(distance.rule, Rule::LessThan);

    let relative_distance = RelativeDistanceCondition::default();
    assert_eq!(
        relative_distance.entity_ref,
        OSString::literal("DefaultEntity".to_string())
    );
    assert_eq!(relative_distance.value, Double::literal(10.0));
    assert_eq!(relative_distance.freespace, Boolean::literal(true));
    assert_eq!(
        relative_distance.relative_distance_type,
        RelativeDistanceType::Cartesian
    );
    assert_eq!(relative_distance.rule, Rule::LessThan);
}

#[test]
fn test_xml_serialization_reach_position() {
    let condition = ReachPositionCondition::at_world_position(100.0, 200.0, 0.0, 1.57, 2.0);

    let xml =
        quick_xml::se::to_string(&condition).expect("Failed to serialize ReachPositionCondition");

    // Check that key attributes are present
    assert!(xml.contains("tolerance=\"2\""));
    assert!(xml.contains("Position"));
    assert!(xml.contains("WorldPosition"));
    assert!(xml.contains("x=\"100\""));
    assert!(xml.contains("y=\"200\""));
    assert!(xml.contains("h=\"1.57\""));
}

#[test]
fn test_xml_serialization_distance_condition() {
    let position = Position::default();
    let condition = DistanceCondition::greater_than(position, 50.0, true)
        .with_coordinate_system(CoordinateSystem::Entity)
        .with_distance_type(RelativeDistanceType::Longitudinal);

    let xml = quick_xml::se::to_string(&condition).expect("Failed to serialize DistanceCondition");

    // Check that key attributes are present
    assert!(xml.contains("value=\"50\""));
    assert!(xml.contains("freespace=\"true\""));
    assert!(xml.contains("rule=\"greaterThan\""));
    assert!(xml.contains("coordinateSystem=\"entity\""));
    assert!(xml.contains("relativeDistanceType=\"longitudinal\""));
    assert!(xml.contains("Position"));
}

#[test]
fn test_xml_serialization_relative_distance_condition() {
    let condition =
        RelativeDistanceCondition::lateral("target_vehicle", 8.5, false, Rule::NotEqualTo)
            .with_coordinate_system(CoordinateSystem::Road)
            .with_routing_algorithm(RoutingAlgorithm::LeastIntersections);

    let xml = quick_xml::se::to_string(&condition)
        .expect("Failed to serialize RelativeDistanceCondition");

    // Check that key attributes are present
    assert!(xml.contains("entityRef=\"target_vehicle\""));
    assert!(xml.contains("value=\"8.5\""));
    assert!(xml.contains("freespace=\"false\""));
    assert!(xml.contains("relativeDistanceType=\"lateral\""));
    assert!(xml.contains("rule=\"notEqualTo\""));
    assert!(xml.contains("coordinateSystem=\"road\""));
    assert!(xml.contains("routingAlgorithm=\"leastIntersections\""));
}

#[test]
fn test_xml_round_trip_reach_position() {
    let original = ReachPositionCondition::at_world_position(75.0, 125.0, 2.0, 3.14, 1.5);

    let xml = quick_xml::se::to_string(&original).expect("Failed to serialize");
    let deserialized: ReachPositionCondition =
        quick_xml::de::from_str(&xml).expect("Failed to deserialize ReachPositionCondition");

    assert_eq!(original.tolerance, deserialized.tolerance);
    assert_eq!(
        original.position.world_position.is_some(),
        deserialized.position.world_position.is_some()
    );

    if let (Some(orig_pos), Some(deser_pos)) = (
        original.position.world_position,
        deserialized.position.world_position,
    ) {
        assert_eq!(orig_pos.x, deser_pos.x);
        assert_eq!(orig_pos.y, deser_pos.y);
        assert_eq!(orig_pos.z, deser_pos.z);
        assert_eq!(orig_pos.h, deser_pos.h);
    }
}

#[test]
fn test_xml_round_trip_distance_condition() {
    let position = Position::default();
    let original = DistanceCondition::less_than(position, 35.0, false)
        .with_coordinate_system(CoordinateSystem::Trajectory)
        .with_distance_type(RelativeDistanceType::Cartesian);

    let xml = quick_xml::se::to_string(&original).expect("Failed to serialize");
    let deserialized: DistanceCondition =
        quick_xml::de::from_str(&xml).expect("Failed to deserialize DistanceCondition");

    assert_eq!(original.value, deserialized.value);
    assert_eq!(original.freespace, deserialized.freespace);
    assert_eq!(original.rule, deserialized.rule);
    assert_eq!(original.coordinate_system, deserialized.coordinate_system);
    assert_eq!(
        original.relative_distance_type,
        deserialized.relative_distance_type
    );
}

#[test]
fn test_xml_round_trip_relative_distance_condition() {
    let original = RelativeDistanceCondition::longitudinal(
        "reference_vehicle",
        18.0,
        true,
        Rule::GreaterOrEqual,
    )
    .with_coordinate_system(CoordinateSystem::Lane)
    .with_routing_algorithm(RoutingAlgorithm::AssignedRoute);

    let xml = quick_xml::se::to_string(&original).expect("Failed to serialize");
    let deserialized: RelativeDistanceCondition =
        quick_xml::de::from_str(&xml).expect("Failed to deserialize RelativeDistanceCondition");

    assert_eq!(original.entity_ref, deserialized.entity_ref);
    assert_eq!(original.value, deserialized.value);
    assert_eq!(original.freespace, deserialized.freespace);
    assert_eq!(
        original.relative_distance_type,
        deserialized.relative_distance_type
    );
    assert_eq!(original.rule, deserialized.rule);
    assert_eq!(original.coordinate_system, deserialized.coordinate_system);
    assert_eq!(original.routing_algorithm, deserialized.routing_algorithm);
}

#[test]
fn test_real_world_scenario_examples() {
    // Example 1: Vehicle must reach a specific waypoint within 2 meters
    let waypoint_condition = ReachPositionCondition::at_world_position(
        1250.0, // x coordinate
        850.0,  // y coordinate
        0.0,    // z coordinate (ground level)
        1.57,   // heading (90 degrees)
        2.0,    // tolerance in meters
    );

    assert_eq!(waypoint_condition.tolerance, Double::literal(2.0));

    // Example 2: Trigger when ego vehicle is less than 50m from intersection center
    let intersection_pos = Position::default(); // Would be set to intersection coordinates
    let intersection_condition = DistanceCondition::less_than(intersection_pos, 50.0, true)
        .with_coordinate_system(CoordinateSystem::Road)
        .with_distance_type(RelativeDistanceType::Cartesian);

    assert_eq!(intersection_condition.value, Double::literal(50.0));
    assert_eq!(intersection_condition.rule, Rule::LessThan);

    // Example 3: Maintain longitudinal distance > 20m from lead vehicle
    let following_condition = RelativeDistanceCondition::longitudinal(
        "lead_vehicle",
        20.0,
        true, // freespace distance
        Rule::GreaterThan,
    )
    .with_coordinate_system(CoordinateSystem::Road);

    assert_eq!(
        following_condition.relative_distance_type,
        RelativeDistanceType::Longitudinal
    );
    assert_eq!(following_condition.value, Double::literal(20.0));
    assert_eq!(following_condition.rule, Rule::GreaterThan);

    // Example 4: Lane change safety check - lateral distance > 3m from adjacent vehicle
    let lane_change_condition = RelativeDistanceCondition::lateral(
        "adjacent_vehicle",
        3.0,
        true, // freespace
        Rule::GreaterThan,
    )
    .with_coordinate_system(CoordinateSystem::Lane);

    assert_eq!(
        lane_change_condition.relative_distance_type,
        RelativeDistanceType::Lateral
    );
    assert_eq!(
        lane_change_condition.coordinate_system,
        Some(CoordinateSystem::Lane)
    );
}

#[test]
fn test_parameter_support() {
    // Test that conditions can use parameterized values
    let condition = RelativeDistanceCondition {
        entity_ref: OSString::parameter("target_entity".to_string()),
        value: Double::parameter("safety_distance".to_string()),
        freespace: Boolean::parameter("use_freespace".to_string()),
        relative_distance_type: RelativeDistanceType::Cartesian,
        rule: Rule::GreaterThan,
        coordinate_system: None,
        routing_algorithm: None,
    };

    // Verify parameter values are preserved
    match condition.entity_ref {
        OSString::Parameter(ref param) => assert_eq!(param, "target_entity"),
        _ => panic!("Expected parameter value"),
    }

    match condition.value {
        Double::Parameter(ref param) => assert_eq!(param, "safety_distance"),
        _ => panic!("Expected parameter value"),
    }

    match condition.freespace {
        Boolean::Parameter(ref param) => assert_eq!(param, "use_freespace"),
        _ => panic!("Expected parameter value"),
    }
}
