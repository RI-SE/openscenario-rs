//! Value-based condition types for parameter and time-based triggering
//!
//! This file contains:
//! - Parameter conditions for scenario parameter monitoring
//! - Variable conditions for dynamic variable state checking
//! - Time-based conditions (simulation time, time-of-day)
//! - Storyboard element state conditions for execution flow control
//! - Traffic signal conditions for infrastructure interaction
//! - User-defined custom condition support
//!
#[cfg(feature = "chrono")]
use crate::types::basic::DateTime;
use crate::types::basic::{Double, OSString};
use crate::types::enums::{Rule, StoryboardElementState, StoryboardElementType};
use serde::{Deserialize, Serialize};

/// Simulation time-based condition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SimulationTimeCondition {
    #[serde(rename = "@value")]
    pub value: Double,
    #[serde(rename = "@rule")]
    pub rule: Rule,
}

/// Parameter-based condition for monitoring scenario parameters
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParameterCondition {
    #[serde(rename = "@parameterRef")]
    pub parameter_ref: OSString,
    #[serde(rename = "@rule")]
    pub rule: Rule,
    #[serde(rename = "@value")]
    pub value: OSString,
}

/// Time-of-day condition for scheduling based on absolute time
#[cfg(feature = "chrono")]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimeOfDayCondition {
    #[serde(rename = "@dateTime")]
    pub date_time: DateTime,
    #[serde(rename = "@rule")]
    pub rule: Rule,
}

/// Time-of-day condition for scheduling based on absolute time (without chrono feature)
#[cfg(not(feature = "chrono"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimeOfDayCondition {
    #[serde(rename = "@dateTime")]
    pub date_time: OSString, // Fallback to string when chrono is not available
    #[serde(rename = "@rule")]
    pub rule: Rule,
}

/// Storyboard element state condition for execution flow control
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoryboardElementStateCondition {
    #[serde(rename = "@storyboardElementRef")]
    pub storyboard_element_ref: OSString,
    #[serde(rename = "@state")]
    pub state: StoryboardElementState,
    #[serde(rename = "@storyboardElementType")]
    pub storyboard_element_type: StoryboardElementType,
}

/// User-defined custom condition for extensible condition logic
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserDefinedValueCondition {
    #[serde(rename = "@name")]
    pub name: OSString,
    #[serde(rename = "@rule")]
    pub rule: Rule,
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
    pub rule: Rule,
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

// Default implementations
impl Default for SimulationTimeCondition {
    fn default() -> Self {
        Self {
            value: Double::literal(10.0),
            rule: Rule::GreaterThan,
        }
    }
}

impl Default for ParameterCondition {
    fn default() -> Self {
        Self {
            parameter_ref: OSString::literal("defaultParam".to_string()),
            rule: Rule::EqualTo,
            value: OSString::literal("defaultValue".to_string()),
        }
    }
}

#[cfg(feature = "chrono")]
impl Default for TimeOfDayCondition {
    fn default() -> Self {
        Self {
            date_time: DateTime::literal(chrono::Utc::now()),
            rule: Rule::GreaterThan,
        }
    }
}

#[cfg(not(feature = "chrono"))]
impl Default for TimeOfDayCondition {
    fn default() -> Self {
        Self {
            date_time: OSString::literal("2024-01-01T12:00:00Z".to_string()),
            rule: Rule::GreaterThan,
        }
    }
}

impl Default for StoryboardElementStateCondition {
    fn default() -> Self {
        Self {
            storyboard_element_ref: OSString::literal("defaultElement".to_string()),
            state: StoryboardElementState::RunningState,
            storyboard_element_type: StoryboardElementType::Story,
        }
    }
}

impl Default for UserDefinedValueCondition {
    fn default() -> Self {
        Self {
            name: OSString::literal("defaultCondition".to_string()),
            rule: Rule::EqualTo,
            value: OSString::literal("defaultValue".to_string()),
        }
    }
}

impl Default for TrafficSignalCondition {
    fn default() -> Self {
        Self {
            name: OSString::literal("defaultSignal".to_string()),
            state: OSString::literal("green".to_string()),
        }
    }
}

impl Default for TrafficSignalControllerCondition {
    fn default() -> Self {
        Self {
            traffic_signal_controller_ref: OSString::literal("defaultController".to_string()),
            phase: OSString::literal("phase1".to_string()),
        }
    }
}

impl Default for VariableCondition {
    fn default() -> Self {
        Self {
            variable_ref: OSString::literal("defaultVariable".to_string()),
            rule: Rule::EqualTo,
            value: OSString::literal("defaultValue".to_string()),
        }
    }
}

impl Default for ByValueCondition {
    fn default() -> Self {
        Self {
            parameter_condition: None,
            time_of_day_condition: None,
            simulation_time_condition: Some(SimulationTimeCondition::default()),
            storyboard_element_state_condition: None,
            user_defined_value_condition: None,
            traffic_signal_condition: None,
            traffic_signal_controller_condition: None,
            variable_condition: None,
        }
    }
}
