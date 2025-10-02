//! Integration tests for Phase 2: Motion Conditions
//!
//! Tests AccelerationCondition and StandStillCondition implementations

use openscenario_rs::types::basic::Double;
use openscenario_rs::types::conditions::entity::{
    AccelerationCondition, ByEntityCondition, EntityCondition, StandStillCondition,
};
use openscenario_rs::types::enums::{DirectionalDimension, Rule};
use openscenario_rs::types::scenario::triggers::TriggeringEntities;

#[test]
fn test_acceleration_condition_creation() {
    let condition = AccelerationCondition::new(5.0, Rule::GreaterThan);
    assert_eq!(condition.value, Double::literal(5.0));
    assert_eq!(condition.rule, Rule::GreaterThan);
    assert_eq!(condition.direction, None);
}

#[test]
fn test_acceleration_condition_with_direction() {
    let condition = AccelerationCondition::longitudinal(3.0, Rule::LessThan);
    assert_eq!(condition.value, Double::literal(3.0));
    assert_eq!(condition.rule, Rule::LessThan);
    assert_eq!(
        condition.direction,
        Some(DirectionalDimension::Longitudinal)
    );
}

#[test]
fn test_acceleration_condition_convenience_methods() {
    let greater = AccelerationCondition::greater_than(2.5);
    assert_eq!(greater.rule, Rule::GreaterThan);
    assert_eq!(greater.value, Double::literal(2.5));

    let less = AccelerationCondition::less_than(1.0);
    assert_eq!(less.rule, Rule::LessThan);
    assert_eq!(less.value, Double::literal(1.0));
}

#[test]
fn test_acceleration_condition_directional_methods() {
    let longitudinal = AccelerationCondition::longitudinal(4.0, Rule::EqualTo);
    assert_eq!(
        longitudinal.direction,
        Some(DirectionalDimension::Longitudinal)
    );

    let lateral = AccelerationCondition::lateral(2.0, Rule::GreaterOrEqual);
    assert_eq!(lateral.direction, Some(DirectionalDimension::Lateral));

    let vertical = AccelerationCondition::vertical(1.5, Rule::LessOrEqual);
    assert_eq!(vertical.direction, Some(DirectionalDimension::Vertical));
}

#[test]
fn test_standstill_condition_creation() {
    let condition = StandStillCondition::new(3.0);
    assert_eq!(condition.duration, Double::literal(3.0));
}

#[test]
fn test_standstill_condition_with_duration() {
    let condition = StandStillCondition::with_duration(5.5);
    assert_eq!(condition.duration, Double::literal(5.5));
}

#[test]
fn test_by_entity_condition_acceleration_variants() {
    let triggering_entities = TriggeringEntities::default();
    let simple =
        ByEntityCondition::acceleration(triggering_entities.clone(), 3.0, Rule::GreaterThan);
    match simple.entity_condition {
        EntityCondition::Acceleration(acc) => {
            assert_eq!(acc.value, Double::literal(3.0));
            assert_eq!(acc.rule, Rule::GreaterThan);
            assert_eq!(acc.direction, None);
        }
        _ => panic!("Expected Acceleration variant"),
    }

    let with_direction = ByEntityCondition::acceleration_with_direction(
        triggering_entities,
        2.5,
        Rule::LessThan,
        DirectionalDimension::Lateral,
    );
    match with_direction.entity_condition {
        EntityCondition::Acceleration(acc) => {
            assert_eq!(acc.value, Double::literal(2.5));
            assert_eq!(acc.rule, Rule::LessThan);
            assert_eq!(acc.direction, Some(DirectionalDimension::Lateral));
        }
        _ => panic!("Expected Acceleration variant"),
    }
}

#[test]
fn test_by_entity_condition_standstill_variant() {
    let triggering_entities = TriggeringEntities::default();
    let condition = ByEntityCondition::standstill(triggering_entities, 4.0);
    match condition.entity_condition {
        EntityCondition::StandStill(standstill) => {
            assert_eq!(standstill.duration, Double::literal(4.0));
        }
        _ => panic!("Expected StandStill variant"),
    }
}

#[test]
fn test_default_implementations() {
    let acc_default = AccelerationCondition::default();
    assert_eq!(acc_default.value, Double::literal(2.0));
    assert_eq!(acc_default.rule, Rule::GreaterThan);
    assert_eq!(acc_default.direction, None);

    let standstill_default = StandStillCondition::default();
    assert_eq!(standstill_default.duration, Double::literal(1.0));
}

#[test]
fn test_serialization_deserialization() {
    // Test AccelerationCondition serialization
    let acc_condition = AccelerationCondition::new(2.5, Rule::GreaterThan)
        .with_direction(DirectionalDimension::Longitudinal);

    let serialized = serde_json::to_string(&acc_condition).unwrap();
    let deserialized: AccelerationCondition = serde_json::from_str(&serialized).unwrap();
    assert_eq!(acc_condition, deserialized);

    // Test StandStillCondition serialization
    let standstill_condition = StandStillCondition::new(3.5);

    let serialized = serde_json::to_string(&standstill_condition).unwrap();
    let deserialized: StandStillCondition = serde_json::from_str(&serialized).unwrap();
    assert_eq!(standstill_condition, deserialized);
}

#[test]
fn test_motion_conditions_in_enum() {
    // Test that both new condition types work within the ByEntityCondition enum
    let triggering_entities = TriggeringEntities::default();
    let conditions = vec![
        ByEntityCondition::acceleration(triggering_entities.clone(), 5.0, Rule::GreaterThan),
        ByEntityCondition::standstill(triggering_entities.clone(), 2.0),
        ByEntityCondition::acceleration_with_direction(
            triggering_entities,
            3.0,
            Rule::LessThan,
            DirectionalDimension::Vertical,
        ),
    ];

    assert_eq!(conditions.len(), 3);

    // Verify each condition type
    match &conditions[0].entity_condition {
        EntityCondition::Acceleration(_) => (),
        _ => panic!("Expected Acceleration variant"),
    }

    match &conditions[1].entity_condition {
        EntityCondition::StandStill(_) => (),
        _ => panic!("Expected StandStill variant"),
    }

    match &conditions[2].entity_condition {
        EntityCondition::Acceleration(acc) => {
            assert_eq!(acc.direction, Some(DirectionalDimension::Vertical));
        }
        _ => panic!("Expected Acceleration variant"),
    }
}

#[test]
fn test_acceleration_condition_all_rules() {
    let rules = vec![
        Rule::EqualTo,
        Rule::GreaterThan,
        Rule::LessThan,
        Rule::GreaterOrEqual,
        Rule::LessOrEqual,
        Rule::NotEqualTo,
    ];

    for rule in rules {
        let condition = AccelerationCondition::new(1.0, rule.clone());
        assert_eq!(condition.rule, rule);
    }
}

#[test]
fn test_acceleration_condition_all_directions() {
    let directions = vec![
        DirectionalDimension::Longitudinal,
        DirectionalDimension::Lateral,
        DirectionalDimension::Vertical,
    ];

    for direction in directions {
        let condition =
            AccelerationCondition::new(1.0, Rule::GreaterThan).with_direction(direction.clone());
        assert_eq!(condition.direction, Some(direction));
    }
}
