//! Condition builders for simulation time, parameters, variables, and entity speed.
//!
//! ```rust
//! use openscenario_rs::builder::conditions::TimeConditionBuilder;
//!
//! let condition = TimeConditionBuilder::new()
//!     .at_time(5.0)
//!     .build()
//!     .unwrap();
//! ```

use crate::builder::{BuilderError, BuilderResult};
use crate::types::basic::Value;
use crate::types::{
    basic::{Double, OSString},
    conditions::entity::{
        ByEntityCondition, EntityCondition, SpeedCondition as EntitySpeedCondition,
    },
    conditions::value::{
        ByValueCondition, ParameterCondition, SimulationTimeCondition,
        StoryboardElementStateCondition, VariableCondition,
    },
    enums::{ConditionEdge, Rule, StoryboardElementState, StoryboardElementType},
    scenario::triggers::{Condition, ConditionChoice, EntityRef, TriggeringEntities},
};

/// Builder for simulation time conditions
///
/// Creates conditions that trigger at specific simulation times or when
/// simulation time meets certain criteria (greater than, less than, etc.).
#[derive(Debug)]
pub struct TimeConditionBuilder {
    time: Option<f64>,
    rule: Value<Rule>,
}

impl Default for TimeConditionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeConditionBuilder {
    /// Create a new time condition builder
    pub fn new() -> Self {
        Self {
            time: None,
            rule: Value::Literal(Rule::GreaterThan),
        }
    }

    /// Set target time (triggers when simulation time > target)
    pub fn at_time(mut self, time: f64) -> Self {
        self.time = Some(time);
        self.rule = Value::Literal(Rule::GreaterThan);
        self
    }

    /// Set time with custom rule
    pub fn time_rule(mut self, time: f64, rule: Rule) -> Self {
        self.time = Some(time);
        self.rule = Value::Literal(rule);
        self
    }

    /// Set `rule` to a parameter reference (`rule="$name"`) -- the attribute's `xsd:union` admits a `parameter` member alongside the enumeration, so `$name` is schema-valid here; `name` omits the `$`.
    pub fn rule_param(mut self, name: &str) -> Self {
        self.rule = Value::Parameter(name.to_string());
        self
    }

    /// Build the condition
    pub fn build(self) -> BuilderResult<Condition> {
        if self.time.is_none() {
            return Err(BuilderError::validation_error("Time value is required"));
        }

        Ok(Condition {
            name: OSString::literal("TimeCondition".to_string()),
            condition_edge: Value::Literal(ConditionEdge::Rising),
            delay: Double::literal(0.0),
            choice: ConditionChoice::ByValueCondition(ByValueCondition::simulation_time(
                SimulationTimeCondition {
                    value: Double::literal(self.time.unwrap()),
                    rule: self.rule,
                },
            )),
        })
    }
}

/// Builder for speed conditions
///
/// Creates conditions that trigger when an entity's speed meets certain criteria.
/// This is technically an entity condition but is commonly used, so it's included
/// in the value conditions module for convenience.
#[derive(Debug, Default)]
pub struct SpeedConditionBuilder {
    entity_ref: Option<String>,
    speed: Option<f64>,
    rule: Option<Value<Rule>>,
}

impl SpeedConditionBuilder {
    /// Create a new speed condition builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set target entity
    pub fn for_entity(mut self, entity_ref: &str) -> Self {
        self.entity_ref = Some(entity_ref.to_string());
        self
    }

    /// Set speed threshold (triggers when speed > threshold)
    pub fn speed_above(mut self, speed: f64) -> Self {
        self.speed = Some(speed);
        self.rule = Some(Value::Literal(Rule::GreaterThan));
        self
    }

    /// Set speed threshold (triggers when speed < threshold)
    pub fn speed_below(mut self, speed: f64) -> Self {
        self.speed = Some(speed);
        self.rule = Some(Value::Literal(Rule::LessThan));
        self
    }

    /// Set speed with custom rule
    pub fn speed_rule(mut self, speed: f64, rule: Rule) -> Self {
        self.speed = Some(speed);
        self.rule = Some(Value::Literal(rule));
        self
    }

    /// Set `rule` to a parameter reference (`rule="$name"`) -- the attribute's `xsd:union` admits a `parameter` member alongside the enumeration, so `$name` is schema-valid here; `name` omits the `$`.
    pub fn rule_param(mut self, name: &str) -> Self {
        self.rule = Some(Value::Parameter(name.to_string()));
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
        let rule = self
            .rule
            .ok_or_else(|| BuilderError::validation_error("Rule is required"))?;

        let entity_ref = self.entity_ref.unwrap();

        Ok(Condition {
            name: OSString::literal("SpeedCondition".to_string()),
            condition_edge: Value::Literal(ConditionEdge::Rising),
            delay: Double::literal(0.0),
            choice: ConditionChoice::ByEntityCondition(ByEntityCondition {
                triggering_entities: TriggeringEntities::any(vec![EntityRef {
                    entity_ref: OSString::literal(entity_ref.clone()),
                }])?,
                entity_condition: EntityCondition::Speed(EntitySpeedCondition {
                    value: Double::literal(self.speed.unwrap()),
                    rule,
                    direction: None,
                }),
            }),
        })
    }
}

/// Builder for parameter conditions
#[derive(Debug, Default)]
pub struct ParameterConditionBuilder {
    parameter_ref: Option<String>,
    value: Option<f64>,
    rule: Option<Value<Rule>>,
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
        self.rule = Some(Value::Literal(Rule::GreaterThan));
        self
    }

    /// Set parameter value threshold (below)
    pub fn value_below(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Some(Value::Literal(Rule::LessThan));
        self
    }

    /// Set exact parameter value
    pub fn value_equals(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Some(Value::Literal(Rule::EqualTo));
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
        let rule = self
            .rule
            .ok_or_else(|| BuilderError::validation_error("Rule is required"))?;

        Ok(Condition {
            name: OSString::literal("ParameterCondition".to_string()),
            condition_edge: Value::Literal(ConditionEdge::Rising),
            delay: Double::literal(0.0),
            choice: ConditionChoice::ByValueCondition(ByValueCondition::parameter(
                ParameterCondition {
                    parameter_ref: OSString::literal(self.parameter_ref.unwrap()),
                    value: OSString::literal(self.value.unwrap().to_string()),
                    rule,
                },
            )),
        })
    }
}

/// Builder for variable conditions
#[derive(Debug, Default)]
pub struct VariableConditionBuilder {
    variable_ref: Option<String>,
    value: Option<f64>,
    rule: Option<Value<Rule>>,
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
        self.rule = Some(Value::Literal(Rule::GreaterThan));
        self
    }

    /// Set variable value threshold (below)
    pub fn value_below(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Some(Value::Literal(Rule::LessThan));
        self
    }

    /// Set exact variable value
    pub fn value_equals(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Some(Value::Literal(Rule::EqualTo));
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
        let rule = self
            .rule
            .ok_or_else(|| BuilderError::validation_error("Rule is required"))?;

        Ok(Condition {
            name: OSString::literal("VariableCondition".to_string()),
            condition_edge: Value::Literal(ConditionEdge::Rising),
            delay: Double::literal(0.0),
            choice: ConditionChoice::ByValueCondition(ByValueCondition::variable(
                VariableCondition {
                    variable_ref: OSString::literal(self.variable_ref.unwrap()),
                    value: OSString::literal(self.value.unwrap().to_string()),
                    rule,
                },
            )),
        })
    }
}

/// Builder for storyboard element state conditions
#[derive(Debug, Default)]
pub struct StoryboardElementStateConditionBuilder {
    storyboard_element_type: Option<Value<StoryboardElementType>>,
    storyboard_element_ref: Option<String>,
    state: Option<Value<StoryboardElementState>>,
}

impl StoryboardElementStateConditionBuilder {
    /// Create new storyboard element state condition builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set story element
    pub fn story(mut self, story_ref: &str) -> Self {
        self.storyboard_element_type = Some(Value::Literal(StoryboardElementType::Story));
        self.storyboard_element_ref = Some(story_ref.to_string());
        self
    }

    /// Set act element
    pub fn act(mut self, act_ref: &str) -> Self {
        self.storyboard_element_type = Some(Value::Literal(StoryboardElementType::Act));
        self.storyboard_element_ref = Some(act_ref.to_string());
        self
    }

    /// Set event element
    pub fn event(mut self, event_ref: &str) -> Self {
        self.storyboard_element_type = Some(Value::Literal(StoryboardElementType::Event));
        self.storyboard_element_ref = Some(event_ref.to_string());
        self
    }

    /// Set state to complete
    pub fn complete(mut self) -> Self {
        self.state = Some(Value::Literal(StoryboardElementState::CompleteState));
        self
    }

    /// Set state to running
    pub fn running(mut self) -> Self {
        self.state = Some(Value::Literal(StoryboardElementState::RunningState));
        self
    }

    /// Set state to standby
    pub fn standby(mut self) -> Self {
        self.state = Some(Value::Literal(StoryboardElementState::StandbyState));
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
            condition_edge: Value::Literal(ConditionEdge::Rising),
            delay: Double::literal(0.0),
            choice: ConditionChoice::ByValueCondition(ByValueCondition::storyboard_element_state(
                StoryboardElementStateCondition {
                    storyboard_element_type: self.storyboard_element_type.unwrap(),
                    storyboard_element_ref: OSString::literal(self.storyboard_element_ref.unwrap()),
                    state: self.state.unwrap(),
                },
            )),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::basic::Value;
    use crate::types::conditions::value::ByValueConditionChoice;

    #[test]
    fn test_time_condition_builder() {
        let condition = TimeConditionBuilder::new().at_time(5.0).build().unwrap();

        let ConditionChoice::ByValueCondition(by_value) = condition.choice else {
            panic!("Expected ByValueCondition");
        };
        let ByValueConditionChoice::SimulationTimeCondition(time_condition) = by_value.choice
        else {
            panic!("Expected SimulationTimeCondition branch");
        };
        assert_eq!(time_condition.value.as_literal().unwrap(), &5.0);
        assert_eq!(time_condition.rule, Value::Literal(Rule::GreaterThan));
        // XSD:953-960 `Condition@delay` is required; the builder emits 0.
        assert_eq!(condition.delay, Double::literal(0.0));
    }

    #[test]
    fn test_time_condition_with_rule() {
        let condition = TimeConditionBuilder::new()
            .time_rule(10.0, Rule::LessThan)
            .build()
            .unwrap();

        let ConditionChoice::ByValueCondition(by_value) = condition.choice else {
            panic!("Expected ByValueCondition");
        };
        let ByValueConditionChoice::SimulationTimeCondition(time_condition) = by_value.choice
        else {
            panic!("Expected SimulationTimeCondition branch");
        };
        assert_eq!(time_condition.value.as_literal().unwrap(), &10.0);
        assert_eq!(time_condition.rule, Value::Literal(Rule::LessThan));
    }

    #[test]
    fn test_speed_condition_builder() {
        let condition = SpeedConditionBuilder::new()
            .for_entity("ego")
            .speed_above(30.0)
            .build()
            .unwrap();
        assert_eq!(condition.delay, Double::literal(0.0));

        let ConditionChoice::ByEntityCondition(by_entity) = condition.choice else {
            panic!("Expected ByEntityCondition");
        };

        assert_eq!(
            by_entity.triggering_entities.entity_refs[0]
                .entity_ref
                .as_literal()
                .unwrap(),
            "ego"
        );

        match by_entity.entity_condition {
            EntityCondition::Speed(speed_condition) => {
                assert_eq!(speed_condition.value.as_literal().unwrap(), &30.0);
                assert_eq!(speed_condition.rule, Value::Literal(Rule::GreaterThan));
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

        let ConditionChoice::ByEntityCondition(by_entity) = condition.choice else {
            panic!("Expected ByEntityCondition");
        };
        match by_entity.entity_condition {
            EntityCondition::Speed(speed_condition) => {
                assert_eq!(speed_condition.value.as_literal().unwrap(), &15.0);
                assert_eq!(speed_condition.rule, Value::Literal(Rule::LessThan));
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

    fn condition_xml(condition: &Condition) -> String {
        quick_xml::se::to_string_with_root("Condition", condition).expect("serialize Condition")
    }

    /// `ParameterCondition` (XSD:1629-1633) and `VariableCondition` (XSD:2466-2470) under
    /// `ByValueCondition` (XSD:837-847): each threshold setter picks its `rule`, the `f64`
    /// threshold is written as the XSD `String` `value`, and the reference lands on
    /// `parameterRef` / `variableRef`.
    #[test]
    fn parameter_and_variable_condition_builders_emit_xsd_condition() {
        let by_value = |name: &str, inner: &str| {
            format!(
                r#"<Condition name="{name}" conditionEdge="rising" delay="0"><ByValueCondition>{inner}</ByValueCondition></Condition>"#
            )
        };
        let rows = [
            (
                "parameter value_above",
                ParameterConditionBuilder::new()
                    .parameter("max_speed")
                    .value_above(60.0)
                    .build(),
                by_value(
                    "ParameterCondition",
                    r#"<ParameterCondition parameterRef="max_speed" rule="greaterThan" value="60"/>"#,
                ),
            ),
            (
                "parameter value_below",
                ParameterConditionBuilder::new()
                    .parameter("gap")
                    .value_below(2.5)
                    .build(),
                by_value(
                    "ParameterCondition",
                    r#"<ParameterCondition parameterRef="gap" rule="lessThan" value="2.5"/>"#,
                ),
            ),
            (
                "parameter value_equals",
                ParameterConditionBuilder::new()
                    .parameter("lane")
                    .value_equals(-1.0)
                    .build(),
                by_value(
                    "ParameterCondition",
                    r#"<ParameterCondition parameterRef="lane" rule="equalTo" value="-1"/>"#,
                ),
            ),
            (
                "variable value_above",
                VariableConditionBuilder::new()
                    .variable("count")
                    .value_above(3.0)
                    .build(),
                by_value(
                    "VariableCondition",
                    r#"<VariableCondition variableRef="count" rule="greaterThan" value="3"/>"#,
                ),
            ),
            (
                "variable value_below",
                VariableConditionBuilder::new()
                    .variable("traffic_density")
                    .value_below(0.5)
                    .build(),
                by_value(
                    "VariableCondition",
                    r#"<VariableCondition variableRef="traffic_density" rule="lessThan" value="0.5"/>"#,
                ),
            ),
            (
                "variable value_equals",
                VariableConditionBuilder::new()
                    .variable("phase")
                    .value_equals(2.0)
                    .build(),
                by_value(
                    "VariableCondition",
                    r#"<VariableCondition variableRef="phase" rule="equalTo" value="2"/>"#,
                ),
            ),
        ];
        for (row, condition, expected) in rows {
            let condition = condition.unwrap_or_else(|e| panic!("{row}: {e}"));
            assert_eq!(condition_xml(&condition), expected, "{row}");
        }
    }

    /// The reference and the threshold are both XSD-required (`use="required"`, no default).
    #[test]
    fn parameter_and_variable_condition_builders_refuse_missing_fields() {
        let rows = [
            (
                ParameterConditionBuilder::new().value_above(1.0).build(),
                "Parameter reference is required",
            ),
            (
                ParameterConditionBuilder::new().parameter("p").build(),
                "Parameter value is required",
            ),
            (
                VariableConditionBuilder::new().value_below(1.0).build(),
                "Variable reference is required",
            ),
            (
                VariableConditionBuilder::new().variable("v").build(),
                "Variable value is required",
            ),
        ];
        for (result, message) in rows {
            let error = result.expect_err(message).to_string();
            assert!(
                error.contains(message),
                "expected `{message}`, got `{error}`"
            );
        }
    }
}
