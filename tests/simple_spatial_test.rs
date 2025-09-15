//! Simple test to verify spatial conditions compile and work

use openscenario_rs::types::{
    basic::{Double, Boolean, OSString},
    enums::{RelativeDistanceType, Rule},
    positions::Position,
};

#[test]
fn test_spatial_conditions_exist() {
    // This test just verifies that the spatial condition types exist and can be imported
    // We'll use the types directly from the entity module
    
    use openscenario_rs::types::conditions::{
        ReachPositionCondition, DistanceCondition, RelativeDistanceCondition
    };
    
    // Test ReachPositionCondition
    let reach_pos = ReachPositionCondition {
        position: Position::default(),
        tolerance: Double::literal(2.0),
    };
    assert_eq!(reach_pos.tolerance, Double::literal(2.0));
    
    // Test DistanceCondition
    let distance = DistanceCondition {
        position: Position::default(),
        value: Double::literal(50.0),
        freespace: Boolean::literal(true),
        rule: Rule::LessThan,
        along_route: None,
        coordinate_system: None,
        relative_distance_type: None,
        routing_algorithm: None,
    };
    assert_eq!(distance.value, Double::literal(50.0));
    assert_eq!(distance.rule, Rule::LessThan);
    
    // Test RelativeDistanceCondition
    let relative_distance = RelativeDistanceCondition {
        entity_ref: OSString::literal("target_vehicle".to_string()),
        value: Double::literal(15.0),
        freespace: Boolean::literal(false),
        relative_distance_type: RelativeDistanceType::Longitudinal,
        rule: Rule::GreaterThan,
        coordinate_system: None,
        routing_algorithm: None,
    };
    assert_eq!(relative_distance.entity_ref, OSString::literal("target_vehicle".to_string()));
    assert_eq!(relative_distance.relative_distance_type, RelativeDistanceType::Longitudinal);
}

#[test]
fn test_spatial_condition_builders() {
    use openscenario_rs::types::conditions::{
        ReachPositionCondition, DistanceCondition, RelativeDistanceCondition
    };
    
    // Test builder methods
    let reach_pos = ReachPositionCondition::new(Position::default(), 3.0);
    assert_eq!(reach_pos.tolerance, Double::literal(3.0));
    
    let distance = DistanceCondition::new(Position::default(), 25.0, true, Rule::GreaterThan);
    assert_eq!(distance.value, Double::literal(25.0));
    assert_eq!(distance.freespace, Boolean::literal(true));
    assert_eq!(distance.rule, Rule::GreaterThan);
    
    let relative_distance = RelativeDistanceCondition::new(
        OSString::literal("vehicle1".to_string()),
        10.0,
        false,
        RelativeDistanceType::Cartesian,
        Rule::EqualTo,
    );
    assert_eq!(relative_distance.value, Double::literal(10.0));
    assert_eq!(relative_distance.relative_distance_type, RelativeDistanceType::Cartesian);
    assert_eq!(relative_distance.rule, Rule::EqualTo);
}