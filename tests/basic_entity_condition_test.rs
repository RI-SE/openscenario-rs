//! Basic test to verify ByEntityCondition works

use openscenario_rs::types::{
    conditions::ByEntityCondition,
    basic::Double,
    enums::Rule,
};

#[test]
fn test_by_entity_condition_basic() {
    // Test that we can create a basic ByEntityCondition
    let default_condition = ByEntityCondition::default();
    
    match default_condition {
        ByEntityCondition::Speed(speed) => {
            assert_eq!(speed.value, Double::literal(10.0));
            assert_eq!(speed.rule, Rule::GreaterThan);
            assert_eq!(speed.entity_ref, "DefaultEntity");
        }
        _ => panic!("Expected default to be Speed condition"),
    }
}

#[test]
fn test_by_entity_condition_variants() {
    // Test that all variants exist
    let speed_condition = ByEntityCondition::Speed(
        openscenario_rs::types::conditions::SpeedCondition {
            value: Double::literal(25.0),
            rule: Rule::GreaterThan,
            entity_ref: "test_vehicle".to_string(),
        }
    );
    
    match speed_condition {
        ByEntityCondition::Speed(_) => {
            // This should work
            assert!(true);
        }
        _ => panic!("Expected Speed condition"),
    }
    
    // Test that other variants exist (even if we can't construct them easily)
    // This will fail to compile if the variants don't exist
    let _test_variants = |condition: ByEntityCondition| {
        match condition {
            ByEntityCondition::Speed(_) => {},
            ByEntityCondition::ReachPosition(_) => {},
            ByEntityCondition::Distance(_) => {},
            ByEntityCondition::RelativeDistance(_) => {},
            ByEntityCondition::Acceleration(_) => {},
            ByEntityCondition::StandStill(_) => {},
            ByEntityCondition::Collision(_) => {},
            ByEntityCondition::OffRoad(_) => {},
            ByEntityCondition::EndOfRoad(_) => {},
            ByEntityCondition::TimeHeadway(_) => {},
            ByEntityCondition::TimeToCollision(_) => {},
        }
    };
}