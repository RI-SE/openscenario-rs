//! Basic test to verify ByEntityCondition works

use openscenario_rs::types::basic::Value;
use openscenario_rs::types::{
    basic::Double,
    conditions::{ByEntityCondition, EntityCondition, SpeedCondition},
    enums::Rule,
    scenario::triggers::{EntityRef, TriggeringEntities},
};

#[test]
fn test_by_entity_condition_basic() {
    // Test that we can create a basic ByEntityCondition
    let triggering_entities = TriggeringEntities::any(vec![EntityRef::new("Ego")]);
    let condition =
        ByEntityCondition::speed(triggering_entities, 10.0, Rule::GreaterThan, "ego_vehicle");

    match condition.entity_condition {
        EntityCondition::Speed(speed) => {
            assert_eq!(speed.value, Double::literal(10.0));
            assert_eq!(speed.rule, Value::Literal(Rule::GreaterThan));
        }
        _ => panic!("Expected Speed condition"),
    }
}

#[test]
fn test_by_entity_condition_variants() {
    // Test that all variants exist in EntityCondition
    let speed_condition = EntityCondition::Speed(SpeedCondition {
        value: Double::literal(25.0),
        rule: Value::Literal(Rule::GreaterThan),
        direction: None,
    });

    match speed_condition {
        EntityCondition::Speed(_) => {
            // This should work
            assert!(true);
        }
        _ => panic!("Expected Speed condition"),
    }

    // Test that other variants exist (even if we can't construct them easily)
    // This will fail to compile if the variants don't exist
    let _test_variants = |condition: EntityCondition| match condition {
        EntityCondition::Speed(_) => {}
        EntityCondition::ReachPosition(_) => {}
        EntityCondition::Distance(_) => {}
        EntityCondition::RelativeDistance(_) => {}
        EntityCondition::Acceleration(_) => {}
        EntityCondition::StandStill(_) => {}
        EntityCondition::Collision(_) => {}
        EntityCondition::Offroad(_) => {}
        EntityCondition::EndOfRoad(_) => {}
        EntityCondition::TimeHeadway(_) => {}
        EntityCondition::TimeToCollision(_) => {}
        EntityCondition::RelativeSpeed(_) => {}
        EntityCondition::TraveledDistance(_) => {}
        EntityCondition::RelativeClearance(_) => {}
        EntityCondition::Angle(_) => {}
        EntityCondition::RelativeAngle(_) => {}
    };
}
