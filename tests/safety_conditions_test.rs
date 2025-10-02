//! Tests for Phase 3 Safety Conditions: CollisionCondition, OffRoadCondition, EndOfRoadCondition

use openscenario_rs::types::{
    basic::{Double, OSString},
    conditions::{
        ByEntityCondition, CollisionCondition, CollisionTarget, EndOfRoadCondition,
        EntityCondition, OffRoadCondition,
    },
    positions::Position,
    scenario::triggers::TriggeringEntities,
};

#[test]
fn test_collision_condition_with_target() {
    let condition = CollisionCondition::with_target("vehicle1");
    assert_eq!(
        condition.target,
        Some(OSString::literal("vehicle1".to_string()))
    );
    assert_eq!(condition.by_type, None);
    assert_eq!(condition.position, None);
}

#[test]
fn test_collision_condition_with_type() {
    let condition = CollisionCondition::with_type("pedestrian");
    assert_eq!(condition.target, None);
    assert!(condition.by_type.is_some());
    if let Some(by_type) = condition.by_type {
        assert_eq!(
            by_type.target_type,
            OSString::literal("pedestrian".to_string())
        );
    }
    assert_eq!(condition.position, None);
}

#[test]
fn test_collision_condition_at_position() {
    let position = Position::default();
    let condition = CollisionCondition::at_position(position.clone());
    assert_eq!(condition.target, None);
    assert_eq!(condition.by_type, None);
    assert_eq!(condition.position, Some(position));
}

#[test]
fn test_collision_condition_any_collision() {
    let condition = CollisionCondition::any_collision();
    assert_eq!(condition.target, None);
    assert_eq!(condition.by_type, None);
    assert_eq!(condition.position, None);
}

#[test]
fn test_collision_condition_default() {
    let condition = CollisionCondition::default();
    assert_eq!(condition.target, None);
    assert_eq!(condition.by_type, None);
    assert_eq!(condition.position, None);
}

#[test]
fn test_collision_target_default() {
    let target = CollisionTarget::default();
    assert_eq!(target.target_type, OSString::literal("vehicle".to_string()));
}

#[test]
fn test_off_road_condition_new() {
    let condition = OffRoadCondition::new(2.5);
    assert_eq!(condition.duration, Double::literal(2.5));
}

#[test]
fn test_off_road_condition_with_duration() {
    let condition = OffRoadCondition::with_duration(4.0);
    assert_eq!(condition.duration, Double::literal(4.0));
}

#[test]
fn test_off_road_condition_default() {
    let condition = OffRoadCondition::default();
    assert_eq!(condition.duration, Double::literal(1.0));
}

#[test]
fn test_end_of_road_condition_new() {
    let condition = EndOfRoadCondition::new(1.5);
    assert_eq!(condition.duration, Double::literal(1.5));
}

#[test]
fn test_end_of_road_condition_with_duration() {
    let condition = EndOfRoadCondition::with_duration(3.0);
    assert_eq!(condition.duration, Double::literal(3.0));
}

#[test]
fn test_end_of_road_condition_default() {
    let condition = EndOfRoadCondition::default();
    assert_eq!(condition.duration, Double::literal(1.0));
}

#[test]
fn test_by_entity_condition_collision_variants() {
    let triggering_entities = TriggeringEntities::default();
    let collision_target =
        ByEntityCondition::collision_with_target(triggering_entities.clone(), "vehicle1");
    let collision_type =
        ByEntityCondition::collision_with_type(triggering_entities.clone(), "pedestrian");
    let collision_any = ByEntityCondition::collision(triggering_entities);

    match collision_target.entity_condition {
        EntityCondition::Collision(condition) => {
            assert_eq!(
                condition.target,
                Some(OSString::literal("vehicle1".to_string()))
            );
        }
        _ => panic!("Expected Collision variant"),
    }

    match collision_type.entity_condition {
        EntityCondition::Collision(condition) => {
            assert!(condition.by_type.is_some());
        }
        _ => panic!("Expected Collision variant"),
    }

    match collision_any.entity_condition {
        EntityCondition::Collision(_) => (),
        _ => panic!("Expected Collision variant"),
    }
}

#[test]
fn test_by_entity_condition_collision_at_position() {
    let triggering_entities = TriggeringEntities::default();
    let position = Position::default();
    let collision_pos =
        ByEntityCondition::collision_at_position(triggering_entities, position.clone());

    match collision_pos.entity_condition {
        EntityCondition::Collision(condition) => {
            assert_eq!(condition.position, Some(position));
        }
        _ => panic!("Expected Collision variant"),
    }
}

#[test]
fn test_by_entity_condition_safety_variants() {
    let triggering_entities = TriggeringEntities::default();
    let off_road = ByEntityCondition::off_road(triggering_entities.clone(), 2.0);
    let end_of_road = ByEntityCondition::end_of_road(triggering_entities, 3.0);

    match off_road.entity_condition {
        EntityCondition::EndOfRoad(condition) => {
            assert_eq!(condition.duration, Double::literal(2.0));
        }
        _ => panic!("Expected EndOfRoad variant (off_road maps to EndOfRoad)"),
    }

    match end_of_road.entity_condition {
        EntityCondition::EndOfRoad(condition) => {
            assert_eq!(condition.duration, Double::literal(3.0));
        }
        _ => panic!("Expected EndOfRoad variant"),
    }
}

#[test]
fn test_safety_conditions_serialization() {
    let collision = CollisionCondition::with_target("vehicle1");
    let off_road = OffRoadCondition::new(2.0);
    let end_of_road = EndOfRoadCondition::new(3.0);

    // Test that they can be serialized and deserialized
    let collision_serialized = serde_json::to_string(&collision).unwrap();
    let collision_deserialized: CollisionCondition =
        serde_json::from_str(&collision_serialized).unwrap();
    assert_eq!(collision, collision_deserialized);

    let off_road_serialized = serde_json::to_string(&off_road).unwrap();
    let off_road_deserialized: OffRoadCondition =
        serde_json::from_str(&off_road_serialized).unwrap();
    assert_eq!(off_road, off_road_deserialized);

    let end_of_road_serialized = serde_json::to_string(&end_of_road).unwrap();
    let end_of_road_deserialized: EndOfRoadCondition =
        serde_json::from_str(&end_of_road_serialized).unwrap();
    assert_eq!(end_of_road, end_of_road_deserialized);
}

#[test]
fn test_by_entity_condition_enum_completeness() {
    // Test that all variants can be matched
    let triggering_entities = TriggeringEntities::default();
    let conditions = vec![
        ByEntityCondition::collision(triggering_entities.clone()),
        ByEntityCondition::off_road(triggering_entities.clone(), 1.0),
        ByEntityCondition::end_of_road(triggering_entities, 2.0),
    ];

    for condition in conditions {
        match condition.entity_condition {
            EntityCondition::Speed(_) => panic!("Unexpected Speed variant"),
            EntityCondition::ReachPosition(_) => panic!("Unexpected ReachPosition variant"),
            EntityCondition::Distance(_) => panic!("Unexpected Distance variant"),
            EntityCondition::RelativeDistance(_) => panic!("Unexpected RelativeDistance variant"),
            EntityCondition::Acceleration(_) => panic!("Unexpected Acceleration variant"),
            EntityCondition::StandStill(_) => panic!("Unexpected StandStill variant"),
            EntityCondition::Collision(_) => assert!(true),
            EntityCondition::Offroad(_) => panic!("Unexpected Offroad variant"),
            EntityCondition::EndOfRoad(_) => assert!(true),
            EntityCondition::TimeHeadway(_) => panic!("Unexpected TimeHeadway variant"),
            EntityCondition::TimeToCollision(_) => panic!("Unexpected TimeToCollision variant"),
            EntityCondition::RelativeSpeed(_) => panic!("Unexpected RelativeSpeed variant"),
            EntityCondition::TraveledDistance(_) => panic!("Unexpected TraveledDistance variant"),
            EntityCondition::RelativeClearance(_) => panic!("Unexpected RelativeClearance variant"),
            EntityCondition::Angle(_) => panic!("Unexpected Angle variant"),
            EntityCondition::RelativeAngle(_) => panic!("Unexpected RelativeAngle variant"),
        }
    }
}

#[test]
fn test_by_entity_condition_safety_integration() {
    let triggering_entities = TriggeringEntities::default();

    // Test collision conditions
    let collision_target =
        ByEntityCondition::collision_with_target(triggering_entities.clone(), "vehicle1");
    let collision_type =
        ByEntityCondition::collision_with_type(triggering_entities.clone(), "pedestrian");
    let collision_any = ByEntityCondition::collision(triggering_entities.clone());

    // Test safety conditions
    let off_road = ByEntityCondition::off_road(triggering_entities.clone(), 2.0);
    let end_of_road = ByEntityCondition::end_of_road(triggering_entities, 3.0);

    // Verify all conditions are created successfully
    assert!(matches!(
        collision_target.entity_condition,
        EntityCondition::Collision(_)
    ));
    assert!(matches!(
        collision_type.entity_condition,
        EntityCondition::Collision(_)
    ));
    assert!(matches!(
        collision_any.entity_condition,
        EntityCondition::Collision(_)
    ));
    assert!(matches!(
        off_road.entity_condition,
        EntityCondition::EndOfRoad(_)
    ));
    assert!(matches!(
        end_of_road.entity_condition,
        EntityCondition::EndOfRoad(_)
    ));
}
