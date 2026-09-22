//! Cardinality tests for the condition and trigger choice groups converted from parallel
//! `Option` fields to a single `$value` enum: `CollisionCondition`, `TimeToCollisionTarget`,
//! `ByValueCondition`, and `Condition` (in `scenario::triggers`).
//!
//! Each type gets: a parse test per branch, a byte-exact round trip, and zero-branch and
//! two-branch rejection as separate assertions, since a document accepting either would
//! silently emit output no schema-valid tool accepts.

use openscenario_rs::types::conditions::entity::{
    ByEntityCondition, CollisionCondition, CollisionConditionChoice, EntityCondition,
    TimeToCollisionTarget, TimeToCollisionTargetChoice,
};
use openscenario_rs::types::conditions::value::{ByValueCondition, ByValueConditionChoice};
use openscenario_rs::types::scenario::triggers::{Condition, ConditionChoice};

// `EntityCondition` (XSD:1135-1153) is not a parallel-`Option` struct -- it already carries its
// branch in a hand-written externally tagged enum, serialized as a map. It is out of this
// issue's scope, but `CollisionCondition` is one of its sixteen branches, so this checks that a
// `$value`-wrapped struct nests correctly inside that hand-written map serializer, through the
// `ByEntityCondition` wrapper that gives the map a root tag to serialize under (`EntityCondition`
// has no root tag of its own, so it cannot be serialized standalone; that limit predates this
// issue and is unrelated to the `$value` conversion).
#[test]
fn collision_condition_round_trips_nested_inside_entity_condition() {
    let xml = r#"<ByEntityCondition><TriggeringEntities triggeringEntitiesRule="any"><EntityRef entityRef="Ego"/></TriggeringEntities><EntityCondition><CollisionCondition><EntityRef entityRef="Target"/></CollisionCondition></EntityCondition></ByEntityCondition>"#;
    let condition: ByEntityCondition = quick_xml::de::from_str(xml).unwrap();
    assert!(matches!(
        condition.entity_condition,
        EntityCondition::Collision(_)
    ));
    let serialized = quick_xml::se::to_string(&condition).unwrap();
    assert_eq!(serialized, xml);
}

// ─── CollisionCondition (XSD:923-928, bare choice) ────────────────────────────

#[test]
fn collision_condition_parses_entity_ref_branch() {
    let xml = r#"<CollisionCondition><EntityRef entityRef="Ego"/></CollisionCondition>"#;
    let condition: CollisionCondition = quick_xml::de::from_str(xml).unwrap();
    assert!(matches!(
        condition.choice,
        CollisionConditionChoice::EntityRef(_)
    ));
}

#[test]
fn collision_condition_parses_by_type_branch() {
    let xml = r#"<CollisionCondition><ByType type="vehicle"/></CollisionCondition>"#;
    let condition: CollisionCondition = quick_xml::de::from_str(xml).unwrap();
    assert!(matches!(
        condition.choice,
        CollisionConditionChoice::ByType(_)
    ));
}

#[test]
fn collision_condition_round_trips_byte_exact() {
    let xml = r#"<CollisionCondition><EntityRef entityRef="Ego"/></CollisionCondition>"#;
    let condition: CollisionCondition = quick_xml::de::from_str(xml).unwrap();
    let serialized = quick_xml::se::to_string(&condition).unwrap();
    assert_eq!(serialized, xml);
}

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
fn time_to_collision_target_parses_entity_ref_branch() {
    let xml = r#"<TimeToCollisionTarget><EntityRef entityRef="Ego"/></TimeToCollisionTarget>"#;
    let target: TimeToCollisionTarget = quick_xml::de::from_str(xml).unwrap();
    assert!(matches!(
        target.choice,
        TimeToCollisionTargetChoice::EntityRef(_)
    ));
}

#[test]
fn time_to_collision_target_parses_position_branch() {
    let xml = r#"<TimeToCollisionTarget><Position><WorldPosition x="0" y="0" z="0" h="0" p="0" r="0"/></Position></TimeToCollisionTarget>"#;
    let target: TimeToCollisionTarget = quick_xml::de::from_str(xml).unwrap();
    assert!(matches!(
        target.choice,
        TimeToCollisionTargetChoice::Position(_)
    ));
}

#[test]
fn time_to_collision_target_round_trips_byte_exact() {
    let xml = r#"<TimeToCollisionTarget><EntityRef entityRef="Ego"/></TimeToCollisionTarget>"#;
    let target: TimeToCollisionTarget = quick_xml::de::from_str(xml).unwrap();
    let serialized = quick_xml::se::to_string(&target).unwrap();
    assert_eq!(serialized, xml);
}

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
fn by_value_condition_parses_simulation_time_branch() {
    let xml = r#"<ByValueCondition><SimulationTimeCondition value="5" rule="greaterThan"/></ByValueCondition>"#;
    let condition: ByValueCondition = quick_xml::de::from_str(xml).unwrap();
    assert!(matches!(
        condition.choice,
        ByValueConditionChoice::SimulationTimeCondition(_)
    ));
}

#[test]
fn by_value_condition_parses_parameter_branch() {
    let xml = r#"<ByValueCondition><ParameterCondition parameterRef="p" rule="equalTo" value="1"/></ByValueCondition>"#;
    let condition: ByValueCondition = quick_xml::de::from_str(xml).unwrap();
    assert!(matches!(
        condition.choice,
        ByValueConditionChoice::ParameterCondition(_)
    ));
}

#[test]
fn by_value_condition_round_trips_byte_exact() {
    let xml = r#"<ByValueCondition><SimulationTimeCondition value="5" rule="greaterThan"/></ByValueCondition>"#;
    let condition: ByValueCondition = quick_xml::de::from_str(xml).unwrap();
    let serialized = quick_xml::se::to_string(&condition).unwrap();
    assert_eq!(serialized, xml);
}

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
