//! Conditions on values instead of entities: scenario parameters and variables,
//! simulation time and time of day, storyboard element state, traffic signal state,
//! and user-defined conditions.
use crate::types::basic::DateTime;
use crate::types::basic::{Double, OSString, Value};
use crate::types::enums::{Rule, StoryboardElementState, StoryboardElementType};
use serde::{Deserialize, Serialize};

/// Simulation time-based condition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SimulationTimeCondition {
    #[serde(rename = "@value")]
    pub value: Double,
    #[serde(rename = "@rule")]
    pub rule: Value<Rule>,
}

/// Parameter-based condition for monitoring scenario parameters
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParameterCondition {
    #[serde(rename = "@parameterRef")]
    pub parameter_ref: OSString,
    #[serde(rename = "@rule")]
    pub rule: Value<Rule>,
    #[serde(rename = "@value")]
    pub value: OSString,
}

/// Time-of-day condition for scheduling based on absolute time
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimeOfDayCondition {
    #[serde(rename = "@dateTime")]
    pub date_time: DateTime,
    #[serde(rename = "@rule")]
    pub rule: Value<Rule>,
}

/// Storyboard element state condition for execution flow control
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoryboardElementStateCondition {
    #[serde(rename = "@storyboardElementRef")]
    pub storyboard_element_ref: OSString,
    #[serde(rename = "@state")]
    pub state: Value<StoryboardElementState>,
    #[serde(rename = "@storyboardElementType")]
    pub storyboard_element_type: Value<StoryboardElementType>,
}

/// User-defined custom condition for extensible condition logic
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserDefinedValueCondition {
    #[serde(rename = "@name")]
    pub name: OSString,
    #[serde(rename = "@rule")]
    pub rule: Value<Rule>,
    #[serde(rename = "@value")]
    pub value: OSString,
}

/// Traffic signal condition for infrastructure signal monitoring
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficSignalCondition {
    #[serde(rename = "@name")]
    pub name: OSString,
    #[serde(rename = "@state")]
    pub state: OSString,
}

/// Traffic signal controller condition for controller phase monitoring
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficSignalControllerCondition {
    #[serde(rename = "@trafficSignalControllerRef")]
    pub traffic_signal_controller_ref: OSString,
    #[serde(rename = "@phase")]
    pub phase: OSString,
}

/// Variable condition for dynamic variable state checking
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VariableCondition {
    #[serde(rename = "@variableRef")]
    pub variable_ref: OSString,
    #[serde(rename = "@rule")]
    pub rule: Value<Rule>,
    #[serde(rename = "@value")]
    pub value: OSString,
}

/// Value-based condition types - implements all OpenSCENARIO ByValueCondition variants
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ByValueCondition {
    /// Parameter-based condition
    #[serde(rename = "ParameterCondition", skip_serializing_if = "Option::is_none")]
    pub parameter_condition: Option<ParameterCondition>,

    /// Time-of-day condition
    #[serde(rename = "TimeOfDayCondition", skip_serializing_if = "Option::is_none")]
    pub time_of_day_condition: Option<TimeOfDayCondition>,

    /// Simulation time-based condition
    #[serde(
        rename = "SimulationTimeCondition",
        skip_serializing_if = "Option::is_none"
    )]
    pub simulation_time_condition: Option<SimulationTimeCondition>,

    /// Storyboard element state condition
    #[serde(
        rename = "StoryboardElementStateCondition",
        skip_serializing_if = "Option::is_none"
    )]
    pub storyboard_element_state_condition: Option<StoryboardElementStateCondition>,

    /// User-defined condition
    #[serde(
        rename = "UserDefinedValueCondition",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_defined_value_condition: Option<UserDefinedValueCondition>,

    /// Traffic signal condition
    #[serde(
        rename = "TrafficSignalCondition",
        skip_serializing_if = "Option::is_none"
    )]
    pub traffic_signal_condition: Option<TrafficSignalCondition>,

    /// Traffic signal controller condition
    #[serde(
        rename = "TrafficSignalControllerCondition",
        skip_serializing_if = "Option::is_none"
    )]
    pub traffic_signal_controller_condition: Option<TrafficSignalControllerCondition>,

    /// Variable condition
    #[serde(rename = "VariableCondition", skip_serializing_if = "Option::is_none")]
    pub variable_condition: Option<VariableCondition>,
}

// Constructors — see the module-level XSD notes on each type for the required attributes.
impl SimulationTimeCondition {
    /// XSD:2039-2042 `SimulationTimeCondition` — `value` and `rule` are both `use="required"`.
    pub fn new(value: f64, rule: Rule) -> Self {
        Self {
            value: Double::literal(value),
            rule: Value::Literal(rule),
        }
    }
}

impl ParameterCondition {
    /// XSD:1629-1633 `ParameterCondition` — `parameterRef`, `rule`, and `value` are all
    /// `use="required"`.
    pub fn new(parameter_ref: &str, rule: Rule, value: &str) -> Self {
        Self {
            parameter_ref: OSString::literal(parameter_ref.to_string()),
            rule: Value::Literal(rule),
            value: OSString::literal(value.to_string()),
        }
    }
}

impl TimeOfDayCondition {
    /// XSD:2169-2172 `TimeOfDayCondition` — `dateTime` and `rule` are both `use="required"`.
    pub fn new(date_time: chrono::DateTime<chrono::Utc>, rule: Rule) -> Self {
        Self {
            date_time: DateTime::literal(date_time),
            rule: Value::Literal(rule),
        }
    }
}

impl StoryboardElementStateCondition {
    /// XSD:2119-2123 `StoryboardElementStateCondition` — `storyboardElementRef`, `state`, and
    /// `storyboardElementType` are all `use="required"`.
    pub fn new(
        storyboard_element_ref: &str,
        state: StoryboardElementState,
        storyboard_element_type: StoryboardElementType,
    ) -> Self {
        Self {
            storyboard_element_ref: OSString::literal(storyboard_element_ref.to_string()),
            state: Value::Literal(state),
            storyboard_element_type: Value::Literal(storyboard_element_type),
        }
    }
}

impl UserDefinedValueCondition {
    /// XSD:2437-2441 `UserDefinedValueCondition` — `name`, `rule`, and `value` are all
    /// `use="required"`.
    pub fn new(name: &str, rule: Rule, value: &str) -> Self {
        Self {
            name: OSString::literal(name.to_string()),
            rule: Value::Literal(rule),
            value: OSString::literal(value.to_string()),
        }
    }
}

impl TrafficSignalCondition {
    /// XSD:2254-2257 `TrafficSignalCondition` — `name` and `state` are both `use="required"`.
    pub fn new(name: &str, state: &str) -> Self {
        Self {
            name: OSString::literal(name.to_string()),
            state: OSString::literal(state.to_string()),
        }
    }
}

impl TrafficSignalControllerCondition {
    /// XSD:2275-2278 `TrafficSignalControllerCondition` — `trafficSignalControllerRef` and
    /// `phase` are both `use="required"`.
    pub fn new(traffic_signal_controller_ref: &str, phase: &str) -> Self {
        Self {
            traffic_signal_controller_ref: OSString::literal(
                traffic_signal_controller_ref.to_string(),
            ),
            phase: OSString::literal(phase.to_string()),
        }
    }
}

impl VariableCondition {
    /// XSD:2466-2470 `VariableCondition` — `variableRef`, `rule`, and `value` are all
    /// `use="required"`.
    pub fn new(variable_ref: &str, rule: Rule, value: &str) -> Self {
        Self {
            variable_ref: OSString::literal(variable_ref.to_string()),
            rule: Value::Literal(rule),
            value: OSString::literal(value.to_string()),
        }
    }
}

/// Per-branch constructors for the `ByValueCondition` XSD:837-848 choice group. No `Default`:
/// `xsd:choice` requires exactly one child, so any default would silently pick a branch — the
/// worst case for a value that "states nothing". Follows `RouteRef::direct`/`::catalog` and
/// `SpeedActionTarget::absolute`/`::relative`.
impl ByValueCondition {
    fn empty() -> Self {
        Self {
            parameter_condition: None,
            time_of_day_condition: None,
            simulation_time_condition: None,
            storyboard_element_state_condition: None,
            user_defined_value_condition: None,
            traffic_signal_condition: None,
            traffic_signal_controller_condition: None,
            variable_condition: None,
        }
    }

    /// Wrap a `ParameterCondition` branch
    pub fn parameter(condition: ParameterCondition) -> Self {
        Self {
            parameter_condition: Some(condition),
            ..Self::empty()
        }
    }

    /// Wrap a `TimeOfDayCondition` branch
    pub fn time_of_day(condition: TimeOfDayCondition) -> Self {
        Self {
            time_of_day_condition: Some(condition),
            ..Self::empty()
        }
    }

    /// Wrap a `SimulationTimeCondition` branch
    pub fn simulation_time(condition: SimulationTimeCondition) -> Self {
        Self {
            simulation_time_condition: Some(condition),
            ..Self::empty()
        }
    }

    /// Wrap a `StoryboardElementStateCondition` branch
    pub fn storyboard_element_state(condition: StoryboardElementStateCondition) -> Self {
        Self {
            storyboard_element_state_condition: Some(condition),
            ..Self::empty()
        }
    }

    /// Wrap a `UserDefinedValueCondition` branch
    pub fn user_defined_value(condition: UserDefinedValueCondition) -> Self {
        Self {
            user_defined_value_condition: Some(condition),
            ..Self::empty()
        }
    }

    /// Wrap a `TrafficSignalCondition` branch
    pub fn traffic_signal(condition: TrafficSignalCondition) -> Self {
        Self {
            traffic_signal_condition: Some(condition),
            ..Self::empty()
        }
    }

    /// Wrap a `TrafficSignalControllerCondition` branch
    pub fn traffic_signal_controller(condition: TrafficSignalControllerCondition) -> Self {
        Self {
            traffic_signal_controller_condition: Some(condition),
            ..Self::empty()
        }
    }

    /// Wrap a `VariableCondition` branch
    pub fn variable(condition: VariableCondition) -> Self {
        Self {
            variable_condition: Some(condition),
            ..Self::empty()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation_time_condition_new() {
        let cond = SimulationTimeCondition::new(10.0, Rule::GreaterThan);
        assert_eq!(cond.value.as_literal().unwrap(), &10.0);
        assert_eq!(cond.rule, Value::Literal(Rule::GreaterThan));
    }

    #[test]
    fn test_simulation_time_condition_xml_roundtrip() {
        let cond = SimulationTimeCondition {
            value: Double::literal(5.0),
            rule: Value::Literal(Rule::EqualTo),
        };
        let xml = quick_xml::se::to_string(&cond).unwrap();
        let deserialized: SimulationTimeCondition = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(cond, deserialized);
    }

    #[test]
    fn test_parameter_condition_new() {
        let cond = ParameterCondition::new("myParam", Rule::EqualTo, "myValue");
        assert_eq!(cond.parameter_ref.as_literal().unwrap(), "myParam");
        assert_eq!(cond.rule, Value::Literal(Rule::EqualTo));
        assert_eq!(cond.value.as_literal().unwrap(), "myValue");
    }

    #[test]
    fn test_traffic_signal_condition_xml_roundtrip() {
        let cond = TrafficSignalCondition {
            name: OSString::literal("signal1".to_string()),
            state: OSString::literal("red".to_string()),
        };
        let xml = quick_xml::se::to_string(&cond).unwrap();
        let deserialized: TrafficSignalCondition = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(cond, deserialized);
    }

    #[test]
    fn test_by_value_condition_simulation_time_branch() {
        let cond = ByValueCondition::simulation_time(SimulationTimeCondition::new(
            10.0,
            Rule::GreaterThan,
        ));
        assert!(cond.simulation_time_condition.is_some());
        assert!(cond.parameter_condition.is_none());
        assert!(cond.variable_condition.is_none());
    }

    #[test]
    fn test_storyboard_element_state_condition_new() {
        let cond = StoryboardElementStateCondition::new(
            "myElement",
            StoryboardElementState::RunningState,
            StoryboardElementType::Story,
        );
        assert_eq!(
            cond.state,
            Value::Literal(StoryboardElementState::RunningState)
        );
        assert_eq!(
            cond.storyboard_element_type,
            Value::Literal(StoryboardElementType::Story)
        );
    }

    #[test]
    fn test_variable_condition_xml_roundtrip() {
        let cond = VariableCondition {
            variable_ref: OSString::literal("speed".to_string()),
            rule: Value::Literal(Rule::GreaterThan),
            value: OSString::literal("100".to_string()),
        };
        let xml = quick_xml::se::to_string(&cond).unwrap();
        let deserialized: VariableCondition = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(cond, deserialized);
    }
}
