//! Basic test to verify ByEntityCondition works

use openscenario_rs::types::basic::Value;
use openscenario_rs::types::{
    basic::Double,
    conditions::{ByEntityCondition, EntityCondition},
    enums::Rule,
    scenario::triggers::{EntityRef, TriggeringEntities},
};

#[test]
fn test_by_entity_condition_basic() {
    // Test that we can create a basic ByEntityCondition
    let triggering_entities = TriggeringEntities::any(vec![EntityRef::new("Ego")]).unwrap();
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
