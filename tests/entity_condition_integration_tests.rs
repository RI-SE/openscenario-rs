//! Integration tests for entity condition system with spatial conditions
//!
//! Tests the integration of spatial conditions with the ByEntityCondition enum
//! and verifies that all condition types work together seamlessly.

use openscenario_rs::types::{
    basic::{Boolean, Double, OSString},
    conditions::{
        ByEntityCondition, DistanceCondition, ReachPositionCondition, RelativeDistanceCondition,
        EntityCondition,
    },
    enums::{RelativeDistanceType, Rule},
    positions::Position,
    scenario::triggers::TriggeringEntities,
};

#[test]
fn test_by_entity_condition_speed() {
    let triggering_entities = TriggeringEntities::default();
    let speed_condition = ByEntityCondition::speed(triggering_entities, 25.0, Rule::GreaterThan, "ego_vehicle");

    match speed_condition.entity_condition {
        EntityCondition::Speed(speed) => {
            assert_eq!(speed.value, Double::literal(25.0));
            assert_eq!(speed.rule, Rule::GreaterThan);
            assert_eq!(speed.entity_ref, "ego_vehicle");
        }
        _ => panic!("Expected Speed condition"),
    }
}

#[test]
fn test_by_entity_condition_reach_position() {
    let triggering_entities = TriggeringEntities::default();
    let position = Position::default();
    let reach_condition = ByEntityCondition::reach_position(triggering_entities, position, 3.0);

    match reach_condition.entity_condition {
        EntityCondition::ReachPosition(reach) => {
            assert_eq!(reach.tolerance, Double::literal(3.0));
            assert!(reach.position.world_position.is_some());
        }
        _ => panic!("Expected ReachPosition condition"),
    }
}

#[test]
fn test_by_entity_condition_distance() {
    let triggering_entities = TriggeringEntities::default();
    let position = Position::default();
    let distance_condition = ByEntityCondition::distance(triggering_entities, position, 40.0, true, Rule::LessThan);

    match distance_condition.entity_condition {
        EntityCondition::Distance(distance) => {
            assert_eq!(distance.value, Double::literal(40.0));
            assert_eq!(distance.freespace, Boolean::literal(true));
            assert_eq!(distance.rule, Rule::LessThan);
        }
        _ => panic!("Expected Distance condition"),
    }
}

#[test]
fn test_by_entity_condition_relative_distance() {
    let triggering_entities = TriggeringEntities::default();
    let relative_condition = ByEntityCondition::relative_distance(
        triggering_entities,
        "target_vehicle",
        15.0,
        false,
        RelativeDistanceType::Longitudinal,
        Rule::GreaterOrEqual,
    );

    match relative_condition.entity_condition {
        EntityCondition::RelativeDistance(relative) => {
            assert_eq!(
                relative.entity_ref,
                OSString::literal("target_vehicle".to_string())
            );
            assert_eq!(relative.value, Double::literal(15.0));
            assert_eq!(relative.freespace, Boolean::literal(false));
            assert_eq!(
                relative.relative_distance_type,
                RelativeDistanceType::Longitudinal
            );
            assert_eq!(relative.rule, Rule::GreaterOrEqual);
        }
        _ => panic!("Expected RelativeDistance condition"),
    }
}

#[test]
fn test_by_entity_condition_default() {
    let default_condition = ByEntityCondition::default();

    match default_condition.entity_condition {
        EntityCondition::Speed(speed) => {
            assert_eq!(speed.value, Double::literal(10.0));
            assert_eq!(speed.rule, Rule::GreaterThan);
            assert_eq!(speed.entity_ref, "DefaultEntity");
        }
        _ => panic!("Expected default to be Speed condition"),
    }
}

#[test]
fn test_spatial_condition_builders() {
    // Test ReachPositionCondition builder
    let reach_pos = ReachPositionCondition::at_world_position(100.0, 200.0, 0.0, 1.57, 2.0);
    assert_eq!(reach_pos.tolerance, Double::literal(2.0));

    // Test DistanceCondition builders
    let distance_less = DistanceCondition::less_than(Position::default(), 50.0, true);
    assert_eq!(distance_less.rule, Rule::LessThan);
    assert_eq!(distance_less.value, Double::literal(50.0));

    let distance_greater = DistanceCondition::greater_than(Position::default(), 30.0, false);
    assert_eq!(distance_greater.rule, Rule::GreaterThan);
    assert_eq!(distance_greater.value, Double::literal(30.0));

    // Test RelativeDistanceCondition builders
    let longitudinal =
        RelativeDistanceCondition::longitudinal("vehicle1", 20.0, true, Rule::GreaterThan);
    assert_eq!(
        longitudinal.relative_distance_type,
        RelativeDistanceType::Longitudinal
    );

    let lateral = RelativeDistanceCondition::lateral("vehicle2", 5.0, false, Rule::LessThan);
    assert_eq!(
        lateral.relative_distance_type,
        RelativeDistanceType::Lateral
    );

    let cartesian = RelativeDistanceCondition::cartesian("vehicle3", 15.0, true, Rule::EqualTo);
    assert_eq!(
        cartesian.relative_distance_type,
        RelativeDistanceType::Cartesian
    );
}

#[test]
fn test_condition_equality() {
    // Test that identical conditions are equal
    let triggering_entities1 = TriggeringEntities::default();
    let triggering_entities2 = TriggeringEntities::default();
    let triggering_entities3 = TriggeringEntities::default();
    let condition1 = ByEntityCondition::speed(triggering_entities1, 25.0, Rule::EqualTo, "vehicle1");
    let condition2 = ByEntityCondition::speed(triggering_entities2, 25.0, Rule::EqualTo, "vehicle1");
    let condition3 = ByEntityCondition::speed(triggering_entities3, 30.0, Rule::EqualTo, "vehicle1");

    assert_eq!(condition1, condition2);
    assert_ne!(condition1, condition3);
}

#[test]
fn test_condition_cloning() {
    let triggering_entities = TriggeringEntities::default();
    let original = ByEntityCondition::relative_distance(
        triggering_entities,
        "test_vehicle",
        12.5,
        true,
        RelativeDistanceType::Cartesian,
        Rule::GreaterThan,
    );

    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_entity_condition_xml_deserialization() {
    // Test XML deserialization with our custom deserializer
    let xml = r#"
    <EntityCondition>
        <RelativeDistanceCondition entityRef="CutInVehicle" relativeDistanceType="longitudinal" 
                                 value="10.0" freespace="true" rule="lessThan" coordinateSystem="entity" />
    </EntityCondition>
    "#;

    let result = quick_xml::de::from_str::<EntityCondition>(xml);
    assert!(result.is_ok(), "Failed to deserialize EntityCondition: {:?}", result.err());
    
    let condition = result.unwrap();
    match condition {
        EntityCondition::RelativeDistance(rel_dist) => {
            assert_eq!(rel_dist.entity_ref, OSString::literal("CutInVehicle".to_string()));
            assert_eq!(rel_dist.value, Double::literal(10.0));
            assert_eq!(rel_dist.freespace, Boolean::literal(true));
            assert_eq!(rel_dist.relative_distance_type, RelativeDistanceType::Longitudinal);
            assert_eq!(rel_dist.rule, Rule::LessThan);
        }
        _ => panic!("Expected RelativeDistanceCondition, got: {:?}", condition),
    }
}

#[test]
fn test_entity_condition_xml_deserialization_speed() {
    // Test XML deserialization with SpeedCondition
    let xml = r#"
    <EntityCondition>
        <SpeedCondition value="25.0" rule="greaterThan" entityRef="ego_vehicle" />
    </EntityCondition>
    "#;

    let result = quick_xml::de::from_str::<EntityCondition>(xml);
    assert!(result.is_ok(), "Failed to deserialize EntityCondition: {:?}", result.err());
    
    let condition = result.unwrap();
    match condition {
        EntityCondition::Speed(speed) => {
            assert_eq!(speed.value, Double::literal(25.0));
            assert_eq!(speed.rule, Rule::GreaterThan);
            assert_eq!(speed.entity_ref, "ego_vehicle");
        }
        _ => panic!("Expected SpeedCondition, got: {:?}", condition),
    }
}

#[test]
fn test_entity_condition_xml_deserialization_error_multiple_conditions() {
    // Test that multiple conditions in one EntityCondition cause an error
    let xml = r#"
    <EntityCondition>
        <SpeedCondition value="25.0" rule="greaterThan" entityRef="ego_vehicle" />
        <RelativeDistanceCondition entityRef="CutInVehicle" relativeDistanceType="longitudinal" 
                                 value="10.0" freespace="true" rule="lessThan" coordinateSystem="entity" />
    </EntityCondition>
    "#;

    let result = quick_xml::de::from_str::<EntityCondition>(xml);
    assert!(result.is_err(), "Expected error for multiple conditions");
    
    let error_msg = result.err().unwrap().to_string();
    assert!(error_msg.contains("can only contain one condition type"), 
            "Error message should mention single condition requirement: {}", error_msg);
}

#[test]
fn test_entity_condition_xml_deserialization_error_unknown_condition() {
    // Test that unknown condition types cause an error
    let xml = r#"
    <EntityCondition>
        <UnknownCondition value="25.0" />
    </EntityCondition>
    "#;

    let result = quick_xml::de::from_str::<EntityCondition>(xml);
    assert!(result.is_err(), "Expected error for unknown condition type");
    
    let error_msg = result.err().unwrap().to_string();
    assert!(error_msg.contains("Unknown EntityCondition type"), 
            "Error message should mention unknown condition type: {}", error_msg);
}
