//! Basic test to verify ByEntityCondition works

use openscenario_rs::types::{
    basic::Double, 
    conditions::{ByEntityCondition, EntityCondition, SpeedCondition}, 
    enums::Rule,
    scenario::triggers::TriggeringEntities,
};

#[test]
fn test_by_entity_condition_basic() {
    // Test that we can create a basic ByEntityCondition
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
fn test_by_entity_condition_variants() {
    // Test that all variants exist in EntityCondition
    let speed_condition = EntityCondition::Speed(SpeedCondition {
        value: Double::literal(25.0),
        rule: Rule::GreaterThan,
        entity_ref: "test_vehicle".to_string(),
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
