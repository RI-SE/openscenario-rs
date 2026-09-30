//! Condition and trigger builders.
//!
//! [`value`] holds the conditions that watch time, parameters and variables;
//! [`spatial`] and [`entity`] hold the ones that watch entities. [`TriggerBuilder`]
//! assembles them into the OR-of-ANDs shape a `Trigger` takes, and the storyboard
//! builders attach the result.

pub mod entity;
pub mod spatial;
pub mod value;

pub use entity::{
    AccelerationConditionBuilder, EndOfRoadConditionBuilder, EnhancedSpeedConditionBuilder,
    ReachPositionConditionBuilder, TraveledDistanceConditionBuilder,
};
pub use spatial::{
    CollisionConditionBuilder, DistanceConditionBuilder, RelativeDistanceConditionBuilder,
};
pub use value::{
    ParameterConditionBuilder, SpeedConditionBuilder,
    SpeedConditionBuilder as ValueSpeedConditionBuilder, StoryboardElementStateConditionBuilder,
    TimeConditionBuilder, VariableConditionBuilder,
};

use crate::builder::{BuilderError, BuilderResult};
use crate::types::{
    scenario::triggers::Condition,
    scenario::triggers::{ConditionGroup, Trigger},
};

/// Builder for event triggers with condition groups
///
/// A TriggerBuilder creates triggers that combine multiple conditions using
/// logical AND/OR operations. Condition groups are combined with OR logic,
/// while conditions within a group use AND logic.
#[derive(Debug, Default)]
pub struct TriggerBuilder {
    condition_groups: Vec<ConditionGroup>,
}

impl TriggerBuilder {
    /// Create a new trigger builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a condition group (OR logic between groups)
    pub fn add_condition_group(self) -> ConditionGroupBuilder {
        ConditionGroupBuilder::new(self)
    }

    /// Add a single condition as its own group (convenience method)
    ///
    /// `ConditionGroup::new` keeps a `Result` for `ConditionGroupBuilder::finish_group`,
    /// which collects conditions into a plain `Vec` that can still be empty. This method
    /// never has that problem: it is handed exactly one condition, which is `MinVec<_,
    /// 1>`'s minimum by construction, so `ConditionGroup::from_min` proves the bound at
    /// the type level and there is nothing left to check.
    pub fn add_condition(mut self, condition: Condition) -> Self {
        self.condition_groups
            .push(ConditionGroup::from_min(condition, Vec::new()));
        self
    }

    /// Build the trigger
    pub fn build(self) -> BuilderResult<Trigger> {
        if self.condition_groups.is_empty() {
            return Err(BuilderError::validation_error(
                "At least one condition is required",
            ));
        }

        Ok(Trigger {
            condition_groups: self.condition_groups,
        })
    }

    /// Internal method to add condition group
    pub(crate) fn add_group(mut self, group: ConditionGroup) -> Self {
        self.condition_groups.push(group);
        self
    }
}

/// Builder for condition groups (AND logic within group)
///
/// A ConditionGroupBuilder creates groups of conditions that must all be true
/// for the group to trigger. Multiple groups in a trigger use OR logic.
pub struct ConditionGroupBuilder {
    parent: TriggerBuilder,
    conditions: Vec<Condition>,
}

impl ConditionGroupBuilder {
    pub fn new(parent: TriggerBuilder) -> Self {
        Self {
            parent,
            conditions: Vec::new(),
        }
    }

    /// Add condition to this group
    pub fn add_condition(mut self, condition: Condition) -> Self {
        self.conditions.push(condition);
        self
    }

    /// Add time condition
    pub fn time_condition(self) -> TimeConditionGroupBuilder {
        TimeConditionGroupBuilder::new(self)
    }

    /// Add speed condition
    pub fn speed_condition(self) -> SpeedConditionGroupBuilder {
        SpeedConditionGroupBuilder::new(self)
    }

    /// Finish this group and return to trigger builder
    ///
    /// A group that collected no condition contributes nothing, as before. The emptiness
    /// test is now `ConditionGroup::new` refusing the short list rather than a separate
    /// `is_empty` check, so the two cannot drift apart.
    pub fn finish_group(self) -> TriggerBuilder {
        match ConditionGroup::new(self.conditions) {
            Ok(group) => self.parent.add_group(group),
            Err(_) => self.parent,
        }
    }

    /// Build the condition group and return to trigger builder (alias for finish_group)
    pub fn build(self) -> TriggerBuilder {
        self.finish_group()
    }
}

/// Helper builder for time conditions within groups
pub struct TimeConditionGroupBuilder {
    parent: ConditionGroupBuilder,
    builder: TimeConditionBuilder,
}

impl TimeConditionGroupBuilder {
    pub fn new(parent: ConditionGroupBuilder) -> Self {
        Self {
            parent,
            builder: TimeConditionBuilder::new(),
        }
    }

    pub fn at_time(mut self, time: f64) -> Self {
        self.builder = self.builder.at_time(time);
        self
    }

    pub fn finish(self) -> BuilderResult<ConditionGroupBuilder> {
        let condition = self.builder.build()?;
        Ok(self.parent.add_condition(condition))
    }
}

/// Helper builder for speed conditions within groups
pub struct SpeedConditionGroupBuilder {
    parent: ConditionGroupBuilder,
    builder: ValueSpeedConditionBuilder,
}

impl SpeedConditionGroupBuilder {
    pub fn new(parent: ConditionGroupBuilder) -> Self {
        Self {
            parent,
            builder: ValueSpeedConditionBuilder::new(),
        }
    }

    pub fn for_entity(mut self, entity_ref: &str) -> Self {
        self.builder = self.builder.for_entity(entity_ref);
        self
    }

    pub fn speed_above(mut self, speed: f64) -> Self {
        self.builder = self.builder.speed_above(speed);
        self
    }

    pub fn finish(self) -> BuilderResult<ConditionGroupBuilder> {
        let condition = self.builder.build()?;
        Ok(self.parent.add_condition(condition))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::types::scenario::triggers::ConditionChoice;

    #[test]
    fn test_trigger_builder_rejects_no_conditions() {
        let error = TriggerBuilder::new().build().unwrap_err();
        assert!(
            error
                .to_string()
                .contains("At least one condition is required"),
            "unexpected error: {error}"
        );
    }

    /// `add_condition` builds each `ConditionGroup` from `ConditionGroup::from_min` with the one
    /// condition it is handed, so nothing here can fail. Three chained calls below take no
    /// `Result` at any step; a change that put the fallible `ConditionGroup::new` back on
    /// this path would force a `?` back into this call site and fail to compile.
    #[test]
    fn test_trigger_builder_three_single_condition_groups() {
        let t1 = TimeConditionBuilder::new().at_time(1.0).build().unwrap();
        let t2 = TimeConditionBuilder::new().at_time(2.0).build().unwrap();
        let t3 = TimeConditionBuilder::new().at_time(3.0).build().unwrap();

        let trigger = TriggerBuilder::new()
            .add_condition(t1)
            .add_condition(t2)
            .add_condition(t3)
            .build()
            .unwrap();

        assert_eq!(trigger.condition_groups.len(), 3);
        for group in &trigger.condition_groups {
            assert_eq!(group.conditions.len(), 1);
        }
    }

    #[test]
    fn test_condition_group_builder() {
        let time_condition = TimeConditionBuilder::new().at_time(5.0).build().unwrap();

        let speed_condition = ValueSpeedConditionBuilder::new()
            .for_entity("ego")
            .speed_above(30.0)
            .build()
            .unwrap();

        let trigger = TriggerBuilder::new()
            .add_condition_group()
            .add_condition(time_condition)
            .add_condition(speed_condition)
            .finish_group()
            .build()
            .unwrap();

        assert_eq!(trigger.condition_groups.len(), 1);
        assert_eq!(trigger.condition_groups[0].conditions.len(), 2);
    }

    #[test]
    fn test_condition_group_builder_inline_condition_builders() {
        let trigger = TriggerBuilder::new()
            .add_condition_group()
            .time_condition()
            .at_time(5.0)
            .finish()
            .unwrap()
            .speed_condition()
            .for_entity("ego")
            .speed_above(25.0)
            .finish()
            .unwrap()
            .finish_group()
            .build()
            .unwrap();

        assert_eq!(trigger.condition_groups.len(), 1);
        let conditions = &trigger.condition_groups[0].conditions;
        assert_eq!(conditions.len(), 2);
        assert!(matches!(
            conditions[0].choice,
            ConditionChoice::ByValueCondition(_)
        ));
        assert!(matches!(
            conditions[1].choice,
            ConditionChoice::ByEntityCondition(_)
        ));
    }
}
