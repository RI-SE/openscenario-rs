//! Trigger and event types for scenario timing and control
//!
//! This file contains:
//! - Event definitions with actions and trigger conditions
//! - Trigger combinations and condition group logic
//! - Condition evaluation and edge detection logic
//! - TriggeringEntities for entity-based condition evaluation
//! - Event priority and execution order management
//!
//! Contributes to project by:
//! - Enabling event-driven scenario execution and timing control
//! - Supporting complex trigger logic through condition combinations
//! - Providing precise event timing through edge detection
//! - Facilitating entity-aware trigger conditions and responses
//! - Supporting deterministic event ordering and priority handling

use crate::types::basic::{Double, OSString};
use crate::types::conditions::{ByEntityCondition, ByValueCondition};
use crate::types::enums::{ConditionEdge, TriggeringEntitiesRule};
use serde::{Deserialize, Serialize};

/// Trigger definition containing condition groups
///
/// A Trigger represents a logical OR of condition groups - the trigger fires
/// when any of its condition groups evaluates to true.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Trigger {
    /// Condition groups that make up this trigger (OR logic between groups)
    #[serde(rename = "ConditionGroup")]
    pub condition_groups: Vec<ConditionGroup>,
}

/// Condition group containing multiple conditions
///
/// A ConditionGroup represents a logical AND of conditions - the group
/// evaluates to true when all of its conditions are true.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConditionGroup {
    /// Conditions within this group (AND logic between conditions)
    #[serde(rename = "Condition")]
    pub conditions: Vec<Condition>,
}

/// Individual condition with edge detection and delay
///
/// A Condition defines when a specific state or event should trigger,
/// with support for edge detection and timing delays.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Condition {
    /// Name of the condition for identification
    #[serde(rename = "@name")]
    pub name: OSString,

    /// Edge detection mode (rising, falling, risingOrFalling, none)
    #[serde(rename = "@conditionEdge")]
    pub condition_edge: ConditionEdge,

    /// Optional delay before condition fires
    #[serde(rename = "@delay", skip_serializing_if = "Option::is_none")]
    pub delay: Option<Double>,

    /// Value-based condition (time, parameter, variable, etc.)
    #[serde(rename = "ByValueCondition", skip_serializing_if = "Option::is_none")]
    pub by_value_condition: Option<ByValueCondition>,

    /// Entity-based condition (collision, distance, speed, etc.)
    #[serde(rename = "ByEntityCondition", skip_serializing_if = "Option::is_none")]
    pub by_entity_condition: Option<ByEntityCondition>,
}

/// Type of condition - either entity-based or value-based
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ConditionType {
    /// Entity-based condition (collision, distance, speed, etc.)
    ByEntity(ByEntityCondition),
    /// Value-based condition (time, parameter, variable, etc.)
    ByValue(ByValueCondition),
}

/// Triggering entities specification for entity-based conditions
///
/// Defines which entities can trigger a condition and how multiple
/// triggering entities should be handled (all or any).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TriggeringEntities {
    /// Rule for combining multiple triggering entities (all, any)
    #[serde(rename = "@triggeringEntitiesRule")]
    pub triggering_entities_rule: TriggeringEntitiesRule,

    /// References to entities that can trigger this condition
    #[serde(rename = "EntityRef")]
    pub entity_refs: Vec<EntityRef>,
}

/// Reference to an entity for triggering purposes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EntityRef {
    /// Name of the referenced entity
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,
}

// Default implementations
impl Default for Trigger {
    fn default() -> Self {
        Self {
            condition_groups: vec![ConditionGroup::default()],
        }
    }
}

impl Default for ConditionGroup {
    fn default() -> Self {
        Self {
            conditions: vec![Condition::default()],
        }
    }
}

impl Default for Condition {
    fn default() -> Self {
        Self {
            name: OSString::literal("DefaultCondition".to_string()),
            condition_edge: ConditionEdge::Rising,
            delay: None,
            by_value_condition: Some(ByValueCondition::default()),
            by_entity_condition: None,
        }
    }
}

impl Default for ConditionType {
    fn default() -> Self {
        ConditionType::ByValue(ByValueCondition::default())
    }
}

impl Default for TriggeringEntities {
    fn default() -> Self {
        Self {
            triggering_entities_rule: TriggeringEntitiesRule::Any,
            entity_refs: vec![EntityRef::default()],
        }
    }
}

impl Default for EntityRef {
    fn default() -> Self {
        Self {
            entity_ref: OSString::literal("DefaultEntity".to_string()),
        }
    }
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

    /// Check if this trigger has any conditions
    pub fn has_conditions(&self) -> bool {
        self.condition_groups
            .iter()
            .any(|g| !g.conditions.is_empty())
    }
}

impl ConditionGroup {
    /// Create a new condition group with a single condition
    pub fn new(condition: Condition) -> Self {
        Self {
            conditions: vec![condition],
        }
    }

    /// Add a condition to this group (AND logic)
    pub fn add_condition(&mut self, condition: Condition) {
        self.conditions.push(condition);
    }

    /// Create an empty condition group
    pub fn empty() -> Self {
        Self {
            conditions: Vec::new(),
        }
    }
}

impl Condition {
    /// Create a new condition with default edge detection
    pub fn new(name: impl Into<String>, condition_type: ConditionType) -> Self {
        let (by_value_condition, by_entity_condition) = match condition_type {
            ConditionType::ByValue(cond) => (Some(cond), None),
            ConditionType::ByEntity(cond) => (None, Some(cond)),
        };

        Self {
            name: OSString::literal(name.into()),
            condition_edge: ConditionEdge::Rising,
            delay: None,
            by_value_condition,
            by_entity_condition,
        }
    }

    /// Set the condition edge detection mode
    pub fn with_edge(mut self, edge: ConditionEdge) -> Self {
        self.condition_edge = edge;
        self
    }

    /// Set a delay for this condition
    pub fn with_delay(mut self, delay: Double) -> Self {
        self.delay = Some(delay);
        self
    }
}

impl TriggeringEntities {
    /// Create a new triggering entities specification
    pub fn new(rule: TriggeringEntitiesRule, entity_refs: Vec<EntityRef>) -> Self {
        Self {
            triggering_entities_rule: rule,
            entity_refs,
        }
    }

    /// Create triggering entities with "any" rule
    pub fn any(entity_refs: Vec<EntityRef>) -> Self {
        Self::new(TriggeringEntitiesRule::Any, entity_refs)
    }

    /// Create triggering entities with "all" rule
    pub fn all(entity_refs: Vec<EntityRef>) -> Self {
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
    use crate::types::enums::ConditionEdge;

    #[test]
    fn test_trigger_creation() {
        let condition = Condition::new("TestCondition", ConditionType::default());
        let group = ConditionGroup::new(condition);
        let trigger = Trigger::new(group);

        assert_eq!(trigger.condition_groups.len(), 1);
        assert_eq!(trigger.condition_groups[0].conditions.len(), 1);
        assert_eq!(
            trigger.condition_groups[0].conditions[0]
                .name
                .as_literal()
                .unwrap(),
            "TestCondition"
        );
        assert!(trigger.has_conditions());
    }

    #[test]
    fn test_condition_group_and_logic() {
        let mut group = ConditionGroup::empty();

        group.add_condition(Condition::new("Condition1", ConditionType::default()));
        group.add_condition(Condition::new("Condition2", ConditionType::default()));

        assert_eq!(group.conditions.len(), 2);
        assert_eq!(group.conditions[0].name.as_literal().unwrap(), "Condition1");
        assert_eq!(group.conditions[1].name.as_literal().unwrap(), "Condition2");
    }

    #[test]
    fn test_trigger_or_logic() {
        let mut trigger = Trigger::default();

        // Add second condition group (OR logic)
        let condition = Condition::new("SecondCondition", ConditionType::default());
        let group = ConditionGroup::new(condition);
        trigger.add_condition_group(group);

        assert_eq!(trigger.condition_groups.len(), 2);
        assert_eq!(
            trigger.condition_groups[1].conditions[0]
                .name
                .as_literal()
                .unwrap(),
            "SecondCondition"
        );
    }

    #[test]
    fn test_condition_with_edge_and_delay() {
        let condition = Condition::new("TimedCondition", ConditionType::default())
            .with_edge(ConditionEdge::Falling)
            .with_delay(Value::literal(2.5));

        assert_eq!(condition.name.as_literal().unwrap(), "TimedCondition");
        assert_eq!(condition.condition_edge, ConditionEdge::Falling);
        assert_eq!(
            condition.delay.as_ref().unwrap().as_literal().unwrap(),
            &2.5
        );
    }

    #[test]
    fn test_triggering_entities() {
        let entities = vec![EntityRef::new("Ego"), EntityRef::new("Target")];

        let any_entities = TriggeringEntities::any(entities.clone());
        let all_entities = TriggeringEntities::all(entities);

        assert_eq!(
            any_entities.triggering_entities_rule,
            TriggeringEntitiesRule::Any
        );
        assert_eq!(
            all_entities.triggering_entities_rule,
            TriggeringEntitiesRule::All
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

    #[test]
    fn test_entity_ref() {
        let entity_ref = EntityRef::new("TestEntity");
        assert_eq!(entity_ref.entity_ref.as_literal().unwrap(), "TestEntity");
    }

    #[test]
    fn test_trigger_serialization() {
        let trigger = Trigger::default();
        let serialized = quick_xml::se::to_string(&trigger).expect("Serialization should succeed");
        assert!(serialized.contains("DefaultCondition"));
    }

    #[test]
    fn test_complex_trigger_scenario() {
        // Create a complex trigger: (Condition1 AND Condition2) OR (Condition3)
        let mut group1 = ConditionGroup::new(
            Condition::new("SpeedCondition", ConditionType::default())
                .with_edge(ConditionEdge::Rising),
        );
        group1.add_condition(
            Condition::new("TimeCondition", ConditionType::default())
                .with_delay(Value::literal(1.0)),
        );

        let group2 = ConditionGroup::new(
            Condition::new("CollisionCondition", ConditionType::default())
                .with_edge(ConditionEdge::RisingOrFalling),
        );

        let mut trigger = Trigger::new(group1);
        trigger.add_condition_group(group2);

        assert_eq!(trigger.condition_groups.len(), 2);
        assert_eq!(trigger.condition_groups[0].conditions.len(), 2); // AND group
        assert_eq!(trigger.condition_groups[1].conditions.len(), 1); // OR group
        assert!(trigger.has_conditions());
    }
}

