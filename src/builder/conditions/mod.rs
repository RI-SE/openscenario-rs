//! Condition and trigger builders for event-driven scenario control
//!
//! This module provides builders for creating OpenSCENARIO conditions and triggers
//! that control when events fire in scenarios. It supports both value-based conditions
//! (time, parameters) and entity-based conditions (speed, distance, collision).
//!
//! # Architecture
//!
//! - **Value Conditions**: Time-based, parameter-based, and state-based triggers
//! - **Spatial Conditions**: Distance, position, and collision-based triggers  
//! - **Trigger Builder**: Combines conditions with AND/OR logic
//! - **Integration**: Works with existing storyboard and event builders
//!
//! # Usage
//!
//! ```rust
//! use openscenario_rs::builder::conditions::{TimeConditionBuilder, TriggerBuilder};
//!
//! // Create a time-based trigger
//! let trigger = TriggerBuilder::new()
//!     .add_condition(
//!         TimeConditionBuilder::new()
//!             .at_time(5.0)
//!             .build()?
//!     )
//!     .build()?;
//! ```

pub mod value;
pub mod spatial;
pub mod entity;

pub use value::{TimeConditionBuilder, SpeedConditionBuilder, SpeedConditionBuilder as ValueSpeedConditionBuilder, ParameterConditionBuilder, VariableConditionBuilder, StoryboardElementStateConditionBuilder};
pub use spatial::{DistanceConditionBuilder, RelativeDistanceConditionBuilder, CollisionConditionBuilder};
pub use entity::{AccelerationConditionBuilder, EnhancedSpeedConditionBuilder, TraveledDistanceConditionBuilder, ReachPositionConditionBuilder, EndOfRoadConditionBuilder};

use crate::builder::{BuilderError, BuilderResult};
use crate::types::{
    scenario::triggers::{Trigger, ConditionGroup},
    scenario::triggers::Condition,
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
    pub fn add_condition(mut self, condition: Condition) -> Self {
        let group = ConditionGroup {
            conditions: vec![condition],
        };
        self.condition_groups.push(group);
        self
    }
    
    /// Build the trigger
    pub fn build(self) -> BuilderResult<Trigger> {
        if self.condition_groups.is_empty() {
            return Err(BuilderError::validation_error("At least one condition is required"));
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
    pub fn finish_group(self) -> TriggerBuilder {
        if !self.conditions.is_empty() {
            let group = ConditionGroup {
                conditions: self.conditions,
            };
            self.parent.add_group(group)
        } else {
            self.parent
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
    use crate::types::basic::Value;

    #[test]
    fn test_trigger_builder_basic() {
        let time_condition = TimeConditionBuilder::new()
            .at_time(3.0)
            .build()
            .unwrap();
            
        let trigger = TriggerBuilder::new()
            .add_condition(time_condition)
            .build()
            .unwrap();
            
        assert_eq!(trigger.condition_groups.len(), 1);
        assert_eq!(trigger.condition_groups[0].conditions.len(), 1);
    }

    #[test]
    fn test_condition_group_builder() {
        let time_condition = TimeConditionBuilder::new()
            .at_time(5.0)
            .build()
            .unwrap();

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
}