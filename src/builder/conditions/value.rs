//! Value-based condition builders (time, speed, etc.)
//!
//! This module provides builders for creating value-based conditions that trigger
//! based on simulation time, parameters, variables, and other non-entity values.
//!
//! # Supported Conditions
//!
//! - **SimulationTimeCondition**: Triggers at specific simulation times
//! - **SpeedCondition**: Triggers when entity speed meets criteria
//! - **ParameterCondition**: Triggers based on parameter values
//! - **VariableCondition**: Triggers based on variable state changes
//!
//! # Usage
//!
//! ```rust
//! use openscenario_rs::builder::conditions::TimeConditionBuilder;
//!
//! let condition = TimeConditionBuilder::new()
//!     .at_time(5.0)
//!     .build()?;
//! ```

use crate::builder::{BuilderError, BuilderResult};
use crate::types::{
    basic::{Double, OSString},
    conditions::entity::{
        ByEntityCondition, EntityCondition, SpeedCondition as EntitySpeedCondition,
    },
    conditions::value::{
        ByValueCondition, ParameterCondition, SimulationTimeCondition,
        StoryboardElementStateCondition, VariableCondition,
    },
    enums::{
        ConditionEdge, Rule, StoryboardElementState, StoryboardElementType, TriggeringEntitiesRule,
    },
    scenario::triggers::{Condition, EntityRef, TriggeringEntities},
};

/// Builder for simulation time conditions
///
/// Creates conditions that trigger at specific simulation times or when
/// simulation time meets certain criteria (greater than, less than, etc.).
#[derive(Debug)]
pub struct TimeConditionBuilder {
    time: Option<f64>,
    rule: Rule,
}

impl TimeConditionBuilder {
    /// Create a new time condition builder
    pub fn new() -> Self {
        Self {
            time: None,
            rule: Rule::GreaterThan,
        }
    }

    /// Set target time (triggers when simulation time > target)
    pub fn at_time(mut self, time: f64) -> Self {
        self.time = Some(time);
        self.rule = Rule::GreaterThan;
        self
    }

    /// Set time with custom rule
    pub fn time_rule(mut self, time: f64, rule: Rule) -> Self {
        self.time = Some(time);
        self.rule = rule;
        self
    }

    /// Build the condition
    pub fn build(self) -> BuilderResult<Condition> {
        if self.time.is_none() {
            return Err(BuilderError::validation_error("Time value is required"));
        }

        Ok(Condition {
            name: OSString::literal("TimeCondition".to_string()),
            condition_edge: ConditionEdge::Rising,
            delay: Some(Double::literal(0.0)),
            by_value_condition: Some(ByValueCondition {
                parameter_condition: None,
                time_of_day_condition: None,
                simulation_time_condition: Some(SimulationTimeCondition {
                    value: Double::literal(self.time.unwrap()),
                    rule: self.rule,
                }),
                storyboard_element_state_condition: None,
                user_defined_value_condition: None,
                traffic_signal_condition: None,
                traffic_signal_controller_condition: None,
                variable_condition: None,
            }),
            by_entity_condition: None,
        })
    }
}

/// Builder for speed conditions
///
/// Creates conditions that trigger when an entity's speed meets certain criteria.
/// This is technically an entity condition but is commonly used, so it's included
/// in the value conditions module for convenience.
#[derive(Debug)]
pub struct SpeedConditionBuilder {
    entity_ref: Option<String>,
    speed: Option<f64>,
    rule: Rule,
}

impl Default for SpeedConditionBuilder {
    fn default() -> Self {
        Self {
            entity_ref: None,
            speed: None,
            rule: Rule::GreaterThan,
        }
    }
}

impl SpeedConditionBuilder {
    /// Create a new speed condition builder
    pub fn new() -> Self {
        Self {
            entity_ref: None,
            speed: None,
            rule: Rule::GreaterThan,
        }
    }

    /// Set target entity
    pub fn for_entity(mut self, entity_ref: &str) -> Self {
        self.entity_ref = Some(entity_ref.to_string());
        self
    }

    /// Set speed threshold (triggers when speed > threshold)
    pub fn speed_above(mut self, speed: f64) -> Self {
        self.speed = Some(speed);
        self.rule = Rule::GreaterThan;
        self
    }

    /// Set speed threshold (triggers when speed < threshold)
    pub fn speed_below(mut self, speed: f64) -> Self {
        self.speed = Some(speed);
        self.rule = Rule::LessThan;
        self
    }

    /// Set speed with custom rule
    pub fn speed_rule(mut self, speed: f64, rule: Rule) -> Self {
        self.speed = Some(speed);
        self.rule = rule;
        self
    }

    /// Build the condition
    pub fn build(self) -> BuilderResult<Condition> {
        if self.entity_ref.is_none() {
            return Err(BuilderError::validation_error(
                "Entity reference is required",
            ));
        }
        if self.speed.is_none() {
            return Err(BuilderError::validation_error("Speed value is required"));
        }

        let entity_ref = self.entity_ref.unwrap();

        Ok(Condition {
            name: OSString::literal("SpeedCondition".to_string()),
            condition_edge: ConditionEdge::Rising,
            delay: Some(Double::literal(0.0)),
            by_value_condition: None,
            by_entity_condition: Some(ByEntityCondition {
                triggering_entities: TriggeringEntities {
                    triggering_entities_rule: TriggeringEntitiesRule::Any,
                    entity_refs: vec![EntityRef {
                        entity_ref: OSString::literal(entity_ref.clone()),
                    }],
                },
                entity_condition: EntityCondition::Speed(EntitySpeedCondition {
                    value: Double::literal(self.speed.unwrap()),
                    rule: self.rule,
                    entity_ref: OSString::literal(entity_ref.clone()),
                    direction: None,
                }),
            }),
        })
    }
}

/// Builder for parameter conditions
#[derive(Debug)]
pub struct ParameterConditionBuilder {
    parameter_ref: Option<String>,
    value: Option<f64>,
    rule: Rule,
}

impl Default for ParameterConditionBuilder {
    fn default() -> Self {
        Self {
            parameter_ref: None,
            value: None,
            rule: Rule::EqualTo,
        }
    }
}

impl ParameterConditionBuilder {
    /// Create new parameter condition builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set parameter reference
    pub fn parameter(mut self, parameter_ref: &str) -> Self {
        self.parameter_ref = Some(parameter_ref.to_string());
        self
    }

    /// Set parameter value threshold (above)
    pub fn value_above(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Rule::GreaterThan;
        self
    }

    /// Set parameter value threshold (below)
    pub fn value_below(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Rule::LessThan;
        self
    }

    /// Set exact parameter value
    pub fn value_equals(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Rule::EqualTo;
        self
    }

    /// Build the condition
    pub fn build(self) -> BuilderResult<Condition> {
        if self.parameter_ref.is_none() {
            return Err(BuilderError::validation_error(
                "Parameter reference is required",
            ));
        }
        if self.value.is_none() {
            return Err(BuilderError::validation_error(
                "Parameter value is required",
            ));
        }

        Ok(Condition {
            name: OSString::literal("ParameterCondition".to_string()),
            condition_edge: ConditionEdge::Rising,
            delay: Some(Double::literal(0.0)),
            by_value_condition: Some(ByValueCondition {
                parameter_condition: Some(ParameterCondition {
                    parameter_ref: OSString::literal(self.parameter_ref.unwrap()),
                    value: OSString::literal(self.value.unwrap().to_string()),
                    rule: self.rule,
                }),
                time_of_day_condition: None,
                simulation_time_condition: None,
                storyboard_element_state_condition: None,
                user_defined_value_condition: None,
                traffic_signal_condition: None,
                traffic_signal_controller_condition: None,
                variable_condition: None,
            }),
            by_entity_condition: None,
        })
    }
}

/// Builder for variable conditions
#[derive(Debug)]
pub struct VariableConditionBuilder {
    variable_ref: Option<String>,
    value: Option<f64>,
    rule: Rule,
}

impl Default for VariableConditionBuilder {
    fn default() -> Self {
        Self {
            variable_ref: None,
            value: None,
            rule: Rule::EqualTo,
        }
    }
}

impl VariableConditionBuilder {
    /// Create new variable condition builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set variable reference
    pub fn variable(mut self, variable_ref: &str) -> Self {
        self.variable_ref = Some(variable_ref.to_string());
        self
    }

    /// Set variable value threshold (above)
    pub fn value_above(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Rule::GreaterThan;
        self
    }

    /// Set variable value threshold (below)
    pub fn value_below(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Rule::LessThan;
        self
    }

    /// Set exact variable value
    pub fn value_equals(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Rule::EqualTo;
        self
    }

    /// Build the condition
    pub fn build(self) -> BuilderResult<Condition> {
        if self.variable_ref.is_none() {
            return Err(BuilderError::validation_error(
                "Variable reference is required",
            ));
        }
        if self.value.is_none() {
            return Err(BuilderError::validation_error("Variable value is required"));
        }

        Ok(Condition {
            name: OSString::literal("VariableCondition".to_string()),
            condition_edge: ConditionEdge::Rising,
            delay: Some(Double::literal(0.0)),
            by_value_condition: Some(ByValueCondition {
                parameter_condition: None,
                time_of_day_condition: None,
                simulation_time_condition: None,
                storyboard_element_state_condition: None,
                user_defined_value_condition: None,
                traffic_signal_condition: None,
                traffic_signal_controller_condition: None,
                variable_condition: Some(VariableCondition {
                    variable_ref: OSString::literal(self.variable_ref.unwrap()),
                    value: OSString::literal(self.value.unwrap().to_string()),
                    rule: self.rule,
                }),
            }),
            by_entity_condition: None,
        })
    }
}

/// Builder for storyboard element state conditions
#[derive(Debug, Default)]
pub struct StoryboardElementStateConditionBuilder {
    storyboard_element_type: Option<StoryboardElementType>,
    storyboard_element_ref: Option<String>,
    state: Option<StoryboardElementState>,
}

impl StoryboardElementStateConditionBuilder {
    /// Create new storyboard element state condition builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set story element
    pub fn story(mut self, story_ref: &str) -> Self {
        self.storyboard_element_type = Some(StoryboardElementType::Story);
        self.storyboard_element_ref = Some(story_ref.to_string());
        self
    }

    /// Set act element
    pub fn act(mut self, act_ref: &str) -> Self {
        self.storyboard_element_type = Some(StoryboardElementType::Act);
        self.storyboard_element_ref = Some(act_ref.to_string());
        self
    }

    /// Set event element
    pub fn event(mut self, event_ref: &str) -> Self {
        self.storyboard_element_type = Some(StoryboardElementType::Event);
        self.storyboard_element_ref = Some(event_ref.to_string());
        self
    }

    /// Set state to complete
    pub fn complete(mut self) -> Self {
        self.state = Some(StoryboardElementState::CompleteState);
        self
    }

    /// Set state to running
    pub fn running(mut self) -> Self {
        self.state = Some(StoryboardElementState::RunningState);
        self
    }

    /// Set state to standby
    pub fn standby(mut self) -> Self {
        self.state = Some(StoryboardElementState::StandbyState);
        self
    }

    /// Build the condition
    pub fn build(self) -> BuilderResult<Condition> {
        if self.storyboard_element_type.is_none() {
            return Err(BuilderError::validation_error(
                "Storyboard element type is required",
            ));
        }
        if self.storyboard_element_ref.is_none() {
            return Err(BuilderError::validation_error(
                "Storyboard element reference is required",
            ));
        }
        if self.state.is_none() {
            return Err(BuilderError::validation_error("Element state is required"));
        }

        Ok(Condition {
            name: OSString::literal("StoryboardElementStateCondition".to_string()),
            condition_edge: ConditionEdge::Rising,
            delay: Some(Double::literal(0.0)),
            by_value_condition: Some(ByValueCondition {
                parameter_condition: None,
                time_of_day_condition: None,
                simulation_time_condition: None,
                storyboard_element_state_condition: Some(StoryboardElementStateCondition {
                    storyboard_element_type: self.storyboard_element_type.unwrap(),
                    storyboard_element_ref: OSString::literal(self.storyboard_element_ref.unwrap()),
                    state: self.state.unwrap(),
                }),
                user_defined_value_condition: None,
                traffic_signal_condition: None,
                traffic_signal_controller_condition: None,
                variable_condition: None,
            }),
            by_entity_condition: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::basic::Value;

    #[test]
    fn test_time_condition_builder() {
        let condition = TimeConditionBuilder::new().at_time(5.0).build().unwrap();

        assert!(condition.by_value_condition.is_some());
        let by_value = condition.by_value_condition.unwrap();
        assert!(by_value.simulation_time_condition.is_some());

        let time_condition = by_value.simulation_time_condition.unwrap();
        assert_eq!(time_condition.value.as_literal().unwrap(), &5.0);
        assert_eq!(time_condition.rule, Rule::GreaterThan);
    }

    #[test]
    fn test_time_condition_with_rule() {
        let condition = TimeConditionBuilder::new()
            .time_rule(10.0, Rule::LessThan)
            .build()
            .unwrap();

        let by_value = condition.by_value_condition.unwrap();
        let time_condition = by_value.simulation_time_condition.unwrap();
        assert_eq!(time_condition.value.as_literal().unwrap(), &10.0);
        assert_eq!(time_condition.rule, Rule::LessThan);
    }

    #[test]
    fn test_speed_condition_builder() {
        let condition = SpeedConditionBuilder::new()
            .for_entity("ego")
            .speed_above(30.0)
            .build()
            .unwrap();

        assert!(condition.by_entity_condition.is_some());
        let by_entity = condition.by_entity_condition.unwrap();

        match by_entity.entity_condition {
            EntityCondition::Speed(speed_condition) => {
                assert_eq!(speed_condition.value.as_literal().unwrap(), &30.0);
                assert_eq!(speed_condition.rule, Rule::GreaterThan);
                assert_eq!(speed_condition.entity_ref.as_literal().unwrap(), "ego");
            }
            _ => panic!("Expected Speed condition"),
        }
    }

    #[test]
    fn test_speed_condition_below() {
        let condition = SpeedConditionBuilder::new()
            .for_entity("target")
            .speed_below(15.0)
            .build()
            .unwrap();

        let by_entity = condition.by_entity_condition.unwrap();
        match by_entity.entity_condition {
            EntityCondition::Speed(speed_condition) => {
                assert_eq!(speed_condition.value.as_literal().unwrap(), &15.0);
                assert_eq!(speed_condition.rule, Rule::LessThan);
            }
            _ => panic!("Expected Speed condition"),
        }
    }

    #[test]
    fn test_time_condition_validation() {
        let result = TimeConditionBuilder::new().build();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Time value is required"));
    }

    #[test]
    fn test_speed_condition_validation() {
        // Missing entity reference
        let result = SpeedConditionBuilder::new().speed_above(30.0).build();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Entity reference is required"));

        // Missing speed value
        let result = SpeedConditionBuilder::new().for_entity("ego").build();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Speed value is required"));
    }
}
