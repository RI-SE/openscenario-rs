//! Phase 5 Implementation Tests
//!
//! Tests for temporal conditions and coordinate system types:
//! - TimeHeadwayCondition: Time-based following distance measurement
//! - TimeToCollisionCondition: Collision prediction timing condition
//! - RoadCoordinate: Road-based coordinate system positioning
//! - LaneCoordinate: Lane-based coordinate system positioning

use openscenario_rs::types::basic::{Boolean, Double};
use openscenario_rs::types::conditions::entity::{
    ByEntityCondition, TimeHeadwayCondition, TimeToCollisionCondition, TimeToCollisionTarget,
};
use openscenario_rs::types::enums::{
    CoordinateSystem, RelativeDistanceType, RoutingAlgorithm, Rule,
};
use openscenario_rs::types::positions::{LaneCoordinate, Position, RoadCoordinate};

// ========== Temporal Condition Tests ==========

#[test]
fn test_time_headway_condition_new() {
    let condition = TimeHeadwayCondition::new("vehicle1", 2.5, Rule::GreaterThan, true);

    assert_eq!(condition.entity_ref.as_literal().unwrap(), "vehicle1");
    assert_eq!(condition.value, Double::literal(2.5));
    assert_eq!(condition.rule, Rule::GreaterThan);
    assert_eq!(condition.freespace, Boolean::literal(true));
    assert!(condition.coordinate_system.is_none());
    assert!(condition.relative_distance_type.is_none());
    assert!(condition.routing_algorithm.is_none());
}

#[test]
fn test_time_headway_condition_less_than() {
    let condition = TimeHeadwayCondition::less_than("vehicle2", 1.8, false);

    assert_eq!(condition.entity_ref.as_literal().unwrap(), "vehicle2");
    assert_eq!(condition.value, Double::literal(1.8));
    assert_eq!(condition.rule, Rule::LessThan);
    assert_eq!(condition.freespace, Boolean::literal(false));
}

#[test]
fn test_time_headway_condition_greater_than() {
    let condition = TimeHeadwayCondition::greater_than("vehicle3", 3.0, true);

    assert_eq!(condition.entity_ref.as_literal().unwrap(), "vehicle3");
    assert_eq!(condition.value, Double::literal(3.0));
    assert_eq!(condition.rule, Rule::GreaterThan);
    assert_eq!(condition.freespace, Boolean::literal(true));
}

#[test]
fn test_time_headway_condition_with_options() {
    let condition = TimeHeadwayCondition::new("vehicle4", 2.0, Rule::LessThan, true)
        .with_coordinate_system(CoordinateSystem::Road)
        .with_distance_type(RelativeDistanceType::Lateral)
        .with_routing_algorithm(RoutingAlgorithm::Fastest);

    assert_eq!(condition.entity_ref.as_literal().unwrap(), "vehicle4");
    assert_eq!(condition.value, Double::literal(2.0));
    assert_eq!(condition.rule, Rule::LessThan);
    assert_eq!(condition.freespace, Boolean::literal(true));
    assert_eq!(condition.coordinate_system, Some(CoordinateSystem::Road));
    assert_eq!(
        condition.relative_distance_type,
        Some(RelativeDistanceType::Lateral)
    );
    assert_eq!(condition.routing_algorithm, Some(RoutingAlgorithm::Fastest));
}

#[test]
fn test_time_to_collision_condition_entity_target() {
    let condition =
        TimeToCollisionCondition::with_entity_target("vehicle1", 5.0, Rule::LessThan, true);

    assert_eq!(condition.value, Double::literal(5.0));
    assert_eq!(condition.rule, Rule::LessThan);
    assert_eq!(condition.freespace, Boolean::literal(true));
    assert!(condition.target.entity_ref.is_some());
    assert_eq!(
        condition
            .target
            .entity_ref
            .as_ref()
            .unwrap()
            .as_literal()
            .unwrap(),
        "vehicle1"
    );
    assert!(condition.target.position.is_none());
}

#[test]
fn test_time_to_collision_condition_position_target() {
    let position = Position::default();
    let condition = TimeToCollisionCondition::with_position_target(
        position.clone(),
        3.5,
        Rule::GreaterThan,
        false,
    );

    assert_eq!(condition.value, Double::literal(3.5));
    assert_eq!(condition.rule, Rule::GreaterThan);
    assert_eq!(condition.freespace, Boolean::literal(false));
    assert!(condition.target.entity_ref.is_none());
    assert!(condition.target.position.is_some());
}

#[test]
fn test_time_to_collision_condition_entity_less_than() {
    let condition = TimeToCollisionCondition::entity_less_than("obstacle", 2.0, true);

    assert_eq!(condition.value, Double::literal(2.0));
    assert_eq!(condition.rule, Rule::LessThan);
    assert_eq!(condition.freespace, Boolean::literal(true));
    assert_eq!(
        condition
            .target
            .entity_ref
            .as_ref()
            .unwrap()
            .as_literal()
            .unwrap(),
        "obstacle"
    );
}

#[test]
fn test_time_to_collision_condition_entity_greater_than() {
    let condition = TimeToCollisionCondition::entity_greater_than("pedestrian", 4.0, false);

    assert_eq!(condition.value, Double::literal(4.0));
    assert_eq!(condition.rule, Rule::GreaterThan);
    assert_eq!(condition.freespace, Boolean::literal(false));
    assert_eq!(
        condition
            .target
            .entity_ref
            .as_ref()
            .unwrap()
            .as_literal()
            .unwrap(),
        "pedestrian"
    );
}

#[test]
fn test_time_to_collision_condition_position_less_than() {
    let position = Position::default();
    let condition = TimeToCollisionCondition::position_less_than(position.clone(), 1.5, true);

    assert_eq!(condition.value, Double::literal(1.5));
    assert_eq!(condition.rule, Rule::LessThan);
    assert_eq!(condition.freespace, Boolean::literal(true));
    assert!(condition.target.position.is_some());
}

#[test]
fn test_time_to_collision_condition_position_greater_than() {
    let position = Position::default();
    let condition = TimeToCollisionCondition::position_greater_than(position.clone(), 6.0, false);

    assert_eq!(condition.value, Double::literal(6.0));
    assert_eq!(condition.rule, Rule::GreaterThan);
    assert_eq!(condition.freespace, Boolean::literal(false));
    assert!(condition.target.position.is_some());
}

#[test]
fn test_time_to_collision_condition_with_options() {
    let condition = TimeToCollisionCondition::entity_less_than("vehicle", 3.0, true)
        .with_coordinate_system(CoordinateSystem::Lane)
        .with_distance_type(RelativeDistanceType::Longitudinal)
        .with_routing_algorithm(RoutingAlgorithm::Shortest);

    assert_eq!(condition.coordinate_system, Some(CoordinateSystem::Lane));
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
fn test_time_to_collision_target_entity() {
    let target = TimeToCollisionTarget::entity("target_vehicle");

    assert!(target.entity_ref.is_some());
    assert_eq!(
        target.entity_ref.as_ref().unwrap().as_literal().unwrap(),
        "target_vehicle"
    );
    assert!(target.position.is_none());
}

#[test]
fn test_time_to_collision_target_position() {
    let position = Position::default();
    let target = TimeToCollisionTarget::position(position.clone());

    assert!(target.entity_ref.is_none());
    assert!(target.position.is_some());
}

// ========== By Entity Condition Integration Tests ==========

#[test]
fn test_by_entity_condition_time_headway() {
    let condition = ByEntityCondition::time_headway("vehicle1", 2.0, Rule::LessThan, true);

    match condition {
        ByEntityCondition::TimeHeadway(th_condition) => {
            assert_eq!(th_condition.entity_ref.as_literal().unwrap(), "vehicle1");
            assert_eq!(th_condition.value, Double::literal(2.0));
            assert_eq!(th_condition.rule, Rule::LessThan);
            assert_eq!(th_condition.freespace, Boolean::literal(true));
        }
        _ => panic!("Expected TimeHeadway variant"),
    }
}

#[test]
fn test_by_entity_condition_time_to_collision_entity() {
    let condition =
        ByEntityCondition::time_to_collision_entity("obstacle", 3.0, Rule::GreaterThan, false);

    match condition {
        ByEntityCondition::TimeToCollision(ttc_condition) => {
            assert_eq!(ttc_condition.value, Double::literal(3.0));
            assert_eq!(ttc_condition.rule, Rule::GreaterThan);
            assert_eq!(ttc_condition.freespace, Boolean::literal(false));
            assert!(ttc_condition.target.entity_ref.is_some());
            assert_eq!(
                ttc_condition
                    .target
                    .entity_ref
                    .as_ref()
                    .unwrap()
                    .as_literal()
                    .unwrap(),
                "obstacle"
            );
        }
        _ => panic!("Expected TimeToCollision variant"),
    }
}

#[test]
fn test_by_entity_condition_time_to_collision_position() {
    let position = Position::default();
    let condition =
        ByEntityCondition::time_to_collision_position(position.clone(), 4.5, Rule::EqualTo, true);

    match condition {
        ByEntityCondition::TimeToCollision(ttc_condition) => {
            assert_eq!(ttc_condition.value, Double::literal(4.5));
            assert_eq!(ttc_condition.rule, Rule::EqualTo);
            assert_eq!(ttc_condition.freespace, Boolean::literal(true));
            assert!(ttc_condition.target.position.is_some());
        }
        _ => panic!("Expected TimeToCollision variant"),
    }
}

// ========== Coordinate System Tests ==========

#[test]
fn test_road_coordinate_new() {
    let coordinate = RoadCoordinate::new(100.0, -2.5);

    assert_eq!(coordinate.s, Double::literal(100.0));
    assert_eq!(coordinate.t, Double::literal(-2.5));
    assert!(coordinate.h.is_none());
}

#[test]
fn test_road_coordinate_with_height() {
    let coordinate = RoadCoordinate::with_height(50.0, 1.0, 10.0);

    assert_eq!(coordinate.s, Double::literal(50.0));
    assert_eq!(coordinate.t, Double::literal(1.0));
    assert!(coordinate.h.is_some());
    assert_eq!(coordinate.h.unwrap(), Double::literal(10.0));
}

#[test]
fn test_road_coordinate_center_line() {
    let coordinate = RoadCoordinate::center_line(75.0);

    assert_eq!(coordinate.s, Double::literal(75.0));
    assert_eq!(coordinate.t, Double::literal(0.0));
    assert!(coordinate.h.is_none());
}

#[test]
fn test_road_coordinate_with_offset() {
    let coordinate = RoadCoordinate::with_offset(25.0, -3.0);

    assert_eq!(coordinate.s, Double::literal(25.0));
    assert_eq!(coordinate.t, Double::literal(-3.0));
    assert!(coordinate.h.is_none());
}

#[test]
fn test_lane_coordinate_new() {
    let coordinate = LaneCoordinate::new(200.0, 0.5);

    assert_eq!(coordinate.s, Double::literal(200.0));
    assert_eq!(coordinate.offset, Double::literal(0.5));
    assert!(coordinate.h.is_none());
}

#[test]
fn test_lane_coordinate_with_height() {
    let coordinate = LaneCoordinate::with_height(150.0, -1.0, 5.0);

    assert_eq!(coordinate.s, Double::literal(150.0));
    assert_eq!(coordinate.offset, Double::literal(-1.0));
    assert!(coordinate.h.is_some());
    assert_eq!(coordinate.h.unwrap(), Double::literal(5.0));
}

#[test]
fn test_lane_coordinate_center_line() {
    let coordinate = LaneCoordinate::center_line(80.0);

    assert_eq!(coordinate.s, Double::literal(80.0));
    assert_eq!(coordinate.offset, Double::literal(0.0));
    assert!(coordinate.h.is_none());
}

#[test]
fn test_lane_coordinate_with_offset() {
    let coordinate = LaneCoordinate::with_offset(60.0, 2.0);

    assert_eq!(coordinate.s, Double::literal(60.0));
    assert_eq!(coordinate.offset, Double::literal(2.0));
    assert!(coordinate.h.is_none());
}

// ========== Default Implementation Tests ==========

#[test]
fn test_temporal_condition_defaults() {
    let headway_default = TimeHeadwayCondition::default();
    assert_eq!(
        headway_default.entity_ref.as_literal().unwrap(),
        "DefaultEntity"
    );
    assert_eq!(headway_default.value, Double::literal(2.0));
    assert_eq!(headway_default.rule, Rule::LessThan);
    assert_eq!(headway_default.freespace, Boolean::literal(true));

    let ttc_default = TimeToCollisionCondition::default();
    assert_eq!(ttc_default.value, Double::literal(5.0));
    assert_eq!(ttc_default.rule, Rule::LessThan);
    assert_eq!(ttc_default.freespace, Boolean::literal(true));
    assert!(ttc_default.target.entity_ref.is_some());
    assert_eq!(
        ttc_default
            .target
            .entity_ref
            .as_ref()
            .unwrap()
            .as_literal()
            .unwrap(),
        "DefaultEntity"
    );

    let target_default = TimeToCollisionTarget::default();
    assert!(target_default.entity_ref.is_some());
    assert_eq!(
        target_default
            .entity_ref
            .as_ref()
            .unwrap()
            .as_literal()
            .unwrap(),
        "DefaultEntity"
    );
    assert!(target_default.position.is_none());
}

#[test]
fn test_coordinate_defaults() {
    let road_default = RoadCoordinate::default();
    assert_eq!(road_default.s, Double::literal(0.0));
    assert_eq!(road_default.t, Double::literal(0.0));
    assert!(road_default.h.is_none());

    let lane_default = LaneCoordinate::default();
    assert_eq!(lane_default.s, Double::literal(0.0));
    assert_eq!(lane_default.offset, Double::literal(0.0));
    assert!(lane_default.h.is_none());
}

// ========== Serialization Tests ==========

#[test]
fn test_time_headway_condition_serialization() {
    let condition = TimeHeadwayCondition::new("vehicle1", 2.5, Rule::LessThan, true);

    let serialized =
        serde_json::to_string(&condition).expect("Failed to serialize TimeHeadwayCondition");
    let deserialized: TimeHeadwayCondition =
        serde_json::from_str(&serialized).expect("Failed to deserialize TimeHeadwayCondition");

    assert_eq!(condition, deserialized);
    assert_eq!(deserialized.entity_ref.as_literal().unwrap(), "vehicle1");
    assert_eq!(deserialized.value, Double::literal(2.5));
}

#[test]
fn test_time_to_collision_condition_serialization() {
    let condition = TimeToCollisionCondition::entity_less_than("obstacle", 3.0, false);

    let serialized =
        serde_json::to_string(&condition).expect("Failed to serialize TimeToCollisionCondition");
    let deserialized: TimeToCollisionCondition =
        serde_json::from_str(&serialized).expect("Failed to deserialize TimeToCollisionCondition");

    assert_eq!(condition, deserialized);
    assert_eq!(deserialized.value, Double::literal(3.0));
    assert_eq!(deserialized.rule, Rule::LessThan);
}

#[test]
fn test_road_coordinate_serialization() {
    let coordinate = RoadCoordinate::with_height(100.0, -2.0, 5.0);

    let serialized =
        serde_json::to_string(&coordinate).expect("Failed to serialize RoadCoordinate");
    let deserialized: RoadCoordinate =
        serde_json::from_str(&serialized).expect("Failed to deserialize RoadCoordinate");

    assert_eq!(coordinate, deserialized);
    assert_eq!(deserialized.s, Double::literal(100.0));
    assert_eq!(deserialized.t, Double::literal(-2.0));
    assert_eq!(deserialized.h, Some(Double::literal(5.0)));
}

#[test]
fn test_lane_coordinate_serialization() {
    let coordinate = LaneCoordinate::with_height(200.0, 1.5, 8.0);

    let serialized =
        serde_json::to_string(&coordinate).expect("Failed to serialize LaneCoordinate");
    let deserialized: LaneCoordinate =
        serde_json::from_str(&serialized).expect("Failed to deserialize LaneCoordinate");

    assert_eq!(coordinate, deserialized);
    assert_eq!(deserialized.s, Double::literal(200.0));
    assert_eq!(deserialized.offset, Double::literal(1.5));
    assert_eq!(deserialized.h, Some(Double::literal(8.0)));
}
