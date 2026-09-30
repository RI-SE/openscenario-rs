//! Triggers, events, and the conditions that gate them.
//!
//! A `Trigger` is a disjunction of `ConditionGroup`s, each a conjunction of
//! `Condition`s: OR of ANDs, as the XSD defines it. `TriggeringEntities` names which
//! entities a condition applies to, and `conditionEdge` says whether it fires on the
//! rising edge, the falling edge, or both.
use crate::types::basic::{Double, MinVec, OSString, Value};
use crate::types::conditions::{ByEntityCondition, ByValueCondition};
use crate::types::enums::{ConditionEdge, TriggeringEntitiesRule};
use serde::{Deserialize, Serialize};

/// Trigger definition containing condition groups
///
/// A Trigger represents a logical OR of condition groups - the trigger fires
/// when any of its condition groups evaluates to true.
/// Empty triggers (no condition groups) are allowed for optional triggers.
///
/// XSD `Trigger` (`:2395-2399`): `ConditionGroup` has `minOccurs="0"`, so an empty
/// `Vec` is schema-valid, not fabricated — the derived `Default` states nothing and is
/// kept.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Trigger {
    /// Condition groups that make up this trigger (OR logic between groups)
    #[serde(rename = "ConditionGroup", default)]
    pub condition_groups: Vec<ConditionGroup>,
}

/// Condition group containing multiple conditions
///
/// A ConditionGroup represents a logical AND of conditions - the group
/// evaluates to true when all of its conditions are true.
///
/// XSD `ConditionGroup` (`:962-966`): `Condition` has `maxOccurs="unbounded"` and no
/// `minOccurs="0"`, so a schema-valid group needs at least one condition. That bound is
/// stated by `MinVec<Condition, 1>` rather than by a `validate()` the caller has to
/// remember. Hence there is no `Default` and no `empty()`: both produced
/// `<ConditionGroup/>`, which the parser rejects on the way in and which the serializer
/// could nonetheless emit on the way out.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConditionGroup {
    /// Conditions within this group (AND logic between conditions)
    #[serde(rename = "Condition")]
    pub conditions: MinVec<Condition, 1>,
}

/// Individual condition with edge detection and delay
///
/// A Condition defines when a specific state or event should trigger,
/// with support for edge detection and timing delays.
///
/// XSD `Condition` (`:953-960`) is a bare `xsd:choice` between `ByEntityCondition` and
/// `ByValueCondition`, alongside the three required attributes below.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Condition {
    /// Name of the condition for identification
    #[serde(rename = "@name")]
    pub name: OSString,

    /// Edge detection mode (rising, falling, risingOrFalling, none)
    #[serde(rename = "@conditionEdge")]
    pub condition_edge: Value<ConditionEdge>,

    /// Delay before condition fires (required by XSD Condition complexType)
    #[serde(rename = "@delay")]
    pub delay: Double,

    /// The entity- or value-based condition branch
    #[serde(rename = "$value")]
    pub choice: ConditionChoice,
}

/// The two `Condition` branches. XSD:954-956.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConditionChoice {
    /// Entity-based condition (collision, distance, speed, etc.)
    ByEntityCondition(ByEntityCondition),
    /// Value-based condition (time, parameter, variable, etc.)
    ByValueCondition(ByValueCondition),
}

/// Triggering entities specification for entity-based conditions
///
/// Defines which entities can trigger a condition and how multiple
/// triggering entities should be handled (all or any).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TriggeringEntities {
    /// Rule for combining multiple triggering entities (all, any)
    #[serde(rename = "@triggeringEntitiesRule")]
    pub triggering_entities_rule: Value<TriggeringEntitiesRule>,

    /// References to entities that can trigger this condition
    #[serde(rename = "EntityRef")]
    pub entity_refs: MinVec<EntityRef, 1>,
}

/// Reference to an entity for triggering purposes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EntityRef {
    /// Name of the referenced entity
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,
}

// Implementation methods
impl Trigger {
    /// Create a new trigger with a single condition group
    pub fn new(condition_group: ConditionGroup) -> Self {
        Self {
            condition_groups: vec![condition_group],
        }
    }

    /// Add a condition group to this trigger (OR logic)
    pub fn add_condition_group(&mut self, group: ConditionGroup) {
        self.condition_groups.push(group);
    }

    /// Check whether this trigger carries any condition group.
    ///
    /// The per-group emptiness test this used to perform is gone: a `ConditionGroup`
    /// holds `MinVec<Condition, 1>` and therefore always has at least one condition, so
    /// the only way a trigger can be conditionless is to hold no group at all.
    pub fn has_conditions(&self) -> bool {
        !self.condition_groups.is_empty()
    }
}

impl ConditionGroup {
    /// Create a condition group from the conditions it ANDs together.
    ///
    /// XSD `ConditionGroup` (`Schema/OpenSCENARIO.xsd:964`) declares `Condition` with the
    /// default `minOccurs="1"`. The previous API let a group be created with one
    /// condition and then mutated, and offered an `empty()` whose only product was
    /// `<ConditionGroup/>`. Neither the empty group nor the mutation path can state the
    /// minimum, so the conditions are supplied once, here, and checked once.
    pub fn new(conditions: Vec<Condition>) -> crate::Result<Self> {
        Ok(Self {
            conditions: MinVec::new(conditions)?,
        })
    }

    /// Create a `ConditionGroup` from one guaranteed condition plus any further ones.
    ///
    /// [`ConditionGroupBuilder::finish_group`](crate::builder::conditions::ConditionGroupBuilder::finish_group)
    /// collects conditions into a plain `Vec` that can still be empty when the builder
    /// finishes without adding one, so [`Self::new`] keeps its `Result` for it. A caller
    /// that already holds one condition, such as
    /// [`TriggerBuilder::add_condition`](crate::builder::conditions::TriggerBuilder::add_condition),
    /// proves the bound at the type level instead: the first condition is a required
    /// parameter rather than the head of a vector that might be short.
    pub fn from_min(first: Condition, rest: Vec<Condition>) -> Self {
        Self {
            conditions: MinVec::from_min([first], rest),
        }
    }
}

impl Condition {
    /// Create a new condition with default edge detection
    pub fn new(name: impl Into<String>, choice: ConditionChoice) -> Self {
        Self {
            name: OSString::literal(name.into()),
            condition_edge: Value::Literal(ConditionEdge::Rising),
            delay: Double::literal(0.0),
            choice,
        }
    }

    /// Set the condition edge detection mode
    pub fn with_edge(mut self, edge: ConditionEdge) -> Self {
        self.condition_edge = Value::Literal(edge);
        self
    }

    /// Set a delay for this condition
    pub fn with_delay(mut self, delay: Double) -> Self {
        self.delay = delay;
        self
    }
}

impl TriggeringEntities {
    /// Create a new triggering entities specification
    ///
    /// XSD `TriggeringEntities` (`Schema/OpenSCENARIO.xsd:2402`) declares `EntityRef` with
    /// the default `minOccurs="1"`, so an empty list is refused here rather than emitted.
    pub fn new(rule: TriggeringEntitiesRule, entity_refs: Vec<EntityRef>) -> crate::Result<Self> {
        Ok(Self {
            triggering_entities_rule: Value::Literal(rule),
            entity_refs: MinVec::new(entity_refs)?,
        })
    }

    /// Create triggering entities with "any" rule
    pub fn any(entity_refs: Vec<EntityRef>) -> crate::Result<Self> {
        Self::new(TriggeringEntitiesRule::Any, entity_refs)
    }

    /// Create triggering entities with "all" rule
    pub fn all(entity_refs: Vec<EntityRef>) -> crate::Result<Self> {
        Self::new(TriggeringEntitiesRule::All, entity_refs)
    }
}

impl EntityRef {
    /// Create a new entity reference
    pub fn new(entity_name: impl Into<String>) -> Self {
        Self {
            entity_ref: OSString::literal(entity_name.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::basic::Value;
    use crate::types::conditions::value::SimulationTimeCondition;
    use crate::types::enums::{ConditionEdge, Rule};

    /// A stand-in `ConditionChoice` for tests that don't care which branch is used.
    fn test_condition_type() -> ConditionChoice {
        ConditionChoice::ByValueCondition(ByValueCondition::simulation_time(
            SimulationTimeCondition::new(10.0, Rule::GreaterThan),
        ))
    }

    /// `Trigger` is an OR of `ConditionGroup`s, each an AND of `Condition`s (XSD `Trigger`
    /// `:2395`, `ConditionGroup` `:962`, `Condition` `:953`). The constructors keep groups and
    /// conditions in the order given; `Condition::new` states `conditionEdge="rising"` and
    /// `delay="0"`, both required attributes, and `with_edge`/`with_delay` replace them.
    #[test]
    fn trigger_constructors_emit_an_or_of_ands_in_order() {
        let mut trigger = Trigger::new(
            ConditionGroup::new(vec![
                Condition::new("First", test_condition_type()),
                Condition::new("Second", test_condition_type())
                    .with_edge(ConditionEdge::Falling)
                    .with_delay(Value::literal(2.5)),
            ])
            .unwrap(),
        );
        trigger.add_condition_group(
            ConditionGroup::new(vec![Condition::new("Third", test_condition_type())]).unwrap(),
        );

        assert!(trigger.has_conditions());
        assert!(!Trigger::default().has_conditions());

        let time = r#"<ByValueCondition><SimulationTimeCondition value="10" rule="greaterThan"/></ByValueCondition>"#;
        let expected = format!(
            concat!(
                r#"<StartTrigger>"#,
                r#"<ConditionGroup>"#,
                r#"<Condition name="First" conditionEdge="rising" delay="0">{t}</Condition>"#,
                r#"<Condition name="Second" conditionEdge="falling" delay="2.5">{t}</Condition>"#,
                r#"</ConditionGroup>"#,
                r#"<ConditionGroup>"#,
                r#"<Condition name="Third" conditionEdge="rising" delay="0">{t}</Condition>"#,
                r#"</ConditionGroup>"#,
                r#"</StartTrigger>"#,
            ),
            t = time
        );
        let xml = quick_xml::se::to_string_with_root("StartTrigger", &trigger).unwrap();
        assert_eq!(xml, expected);
    }

    #[test]
    fn test_triggering_entities() {
        let entities = vec![EntityRef::new("Ego"), EntityRef::new("Target")];

        let any_entities = TriggeringEntities::any(entities.clone()).unwrap();
        let all_entities = TriggeringEntities::all(entities).unwrap();

        assert_eq!(
            any_entities.triggering_entities_rule,
            Value::Literal(TriggeringEntitiesRule::Any)
        );
        assert_eq!(
            all_entities.triggering_entities_rule,
            Value::Literal(TriggeringEntitiesRule::All)
        );
        assert_eq!(any_entities.entity_refs.len(), 2);
        assert_eq!(
            any_entities.entity_refs[0].entity_ref.as_literal().unwrap(),
            "Ego"
        );
        assert_eq!(
            any_entities.entity_refs[1].entity_ref.as_literal().unwrap(),
            "Target"
        );
    }
}
