//! Cardinality tests for the condition and trigger choice groups converted from parallel
//! `Option` fields to a single `$value` enum: `CollisionCondition`, `TimeToCollisionTarget`,
//! `ByValueCondition`, and `Condition` (in `scenario::triggers`).
//!
//! Each type gets zero-branch and two-branch rejection as separate assertions, since a
//! document accepting either would silently emit output no schema-valid tool accepts.
//! `Condition` also gets a parse test per branch and a byte-exact round trip. The per-branch
//! wire form of the other three lives in the condition wire tables
//! (`entity_conditions_serde_test.rs`, `value_conditions_test.rs`).

use openscenario_rs::types::conditions::entity::{CollisionCondition, TimeToCollisionTarget};
use openscenario_rs::types::conditions::value::ByValueCondition;
use openscenario_rs::types::scenario::triggers::{Condition, ConditionChoice};

// ─── CollisionCondition (XSD:923-928, bare choice) ────────────────────────────

#[test]
fn collision_condition_rejects_zero_branches() {
    let xml = r#"<CollisionCondition></CollisionCondition>"#;
    let result: Result<CollisionCondition, _> = quick_xml::de::from_str(xml);
    assert!(result.is_err(), "zero branches must be rejected");
}

#[test]
fn collision_condition_rejects_two_branches() {
    let xml = r#"<CollisionCondition><EntityRef entityRef="Ego"/><ByType type="vehicle"/></CollisionCondition>"#;
    let result: Result<CollisionCondition, _> = quick_xml::de::from_str(xml);
    assert!(result.is_err(), "two branches must be rejected");
}

// ─── TimeToCollisionTarget (XSD:2193-2198, bare choice) ───────────────────────

#[test]
fn time_to_collision_target_rejects_zero_branches() {
    let xml = r#"<TimeToCollisionTarget></TimeToCollisionTarget>"#;
    let result: Result<TimeToCollisionTarget, _> = quick_xml::de::from_str(xml);
    assert!(result.is_err(), "zero branches must be rejected");
}

#[test]
fn time_to_collision_target_rejects_two_branches() {
    let xml = r#"<TimeToCollisionTarget><EntityRef entityRef="Ego"/><Position><WorldPosition x="0" y="0" z="0" h="0" p="0" r="0"/></Position></TimeToCollisionTarget>"#;
    let result: Result<TimeToCollisionTarget, _> = quick_xml::de::from_str(xml);
    assert!(result.is_err(), "two branches must be rejected");
}

// ─── ByValueCondition (XSD:837-847, bare choice, 8 branches) ──────────────────

#[test]
fn by_value_condition_rejects_zero_branches() {
    let xml = r#"<ByValueCondition></ByValueCondition>"#;
    let result: Result<ByValueCondition, _> = quick_xml::de::from_str(xml);
    assert!(result.is_err(), "zero branches must be rejected");
}

#[test]
fn by_value_condition_rejects_two_branches() {
    let xml = r#"<ByValueCondition><SimulationTimeCondition value="5" rule="greaterThan"/><ParameterCondition parameterRef="p" rule="equalTo" value="1"/></ByValueCondition>"#;
    let result: Result<ByValueCondition, _> = quick_xml::de::from_str(xml);
    assert!(result.is_err(), "two branches must be rejected");
}

// ─── Condition (XSD:953-960, bare choice + three sibling attributes) ──────────

#[test]
fn condition_parses_by_value_branch() {
    let xml = r#"<Condition name="C1" conditionEdge="rising" delay="0"><ByValueCondition><SimulationTimeCondition value="5" rule="greaterThan"/></ByValueCondition></Condition>"#;
    let condition: Condition = quick_xml::de::from_str(xml).unwrap();
    assert!(matches!(
        condition.choice,
        ConditionChoice::ByValueCondition(_)
    ));
}

#[test]
fn condition_parses_by_entity_branch() {
    let xml = r#"<Condition name="C2" conditionEdge="rising" delay="0"><ByEntityCondition><TriggeringEntities triggeringEntitiesRule="any"><EntityRef entityRef="Ego"/></TriggeringEntities><EntityCondition><StandStillCondition duration="1"/></EntityCondition></ByEntityCondition></Condition>"#;
    let condition: Condition = quick_xml::de::from_str(xml).unwrap();
    assert!(matches!(
        condition.choice,
        ConditionChoice::ByEntityCondition(_)
    ));
}

#[test]
fn condition_round_trips_byte_exact() {
    let xml = r#"<Condition name="C1" conditionEdge="rising" delay="0"><ByValueCondition><SimulationTimeCondition value="5" rule="greaterThan"/></ByValueCondition></Condition>"#;
    let condition: Condition = quick_xml::de::from_str(xml).unwrap();
    let serialized = quick_xml::se::to_string(&condition).unwrap();
    assert_eq!(serialized, xml);
}

#[test]
fn condition_rejects_zero_branches() {
    // Before conversion to `$value`, `Condition` accepted a document with neither
    // `ByEntityCondition` nor `ByValueCondition`, silently constructing an all-`None` value.
    // The externally-tagged `$value` enum makes that structurally impossible.
    let xml = r#"<Condition name="C3" conditionEdge="rising" delay="0"></Condition>"#;
    let result: Result<Condition, _> = quick_xml::de::from_str(xml);
    assert!(result.is_err(), "zero branches must be rejected");
}

#[test]
fn condition_rejects_two_branches() {
    // Before conversion to `$value`, `Condition` accepted both branches at once and kept both,
    // re-serializing a document no schema-valid tool would accept.
    let xml = r#"<Condition name="C4" conditionEdge="rising" delay="0"><ByValueCondition><SimulationTimeCondition value="5" rule="greaterThan"/></ByValueCondition><ByEntityCondition><TriggeringEntities triggeringEntitiesRule="any"><EntityRef entityRef="Ego"/></TriggeringEntities><EntityCondition><StandStillCondition duration="1"/></EntityCondition></ByEntityCondition></Condition>"#;
    let result: Result<Condition, _> = quick_xml::de::from_str(xml);
    assert!(result.is_err(), "two branches must be rejected");
}
