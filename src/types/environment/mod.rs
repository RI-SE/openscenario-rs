//! Environment type module for weather, lighting, and road conditions
//!
//! This file contains:
//! - Environment container with complete environmental setup
//! - TimeOfDay settings for lighting and time progression
//! - Weather conditions including sun, fog, and precipitation
//! - Road conditions with friction and surface properties
//! - Integration with rendering and physics systems
//!
//! Contributes to project by:
//! - Enabling complete environmental setup for OpenSCENARIO scenarios
//! - Supporting realistic weather and lighting conditions
//! - Providing integration with external environment systems
//! - Facilitating environment-dependent scenario validation

use crate::types::basic::{Boolean, OSString};
use serde::{Deserialize, Serialize};

pub mod road;
pub mod weather;

pub use road::*;
pub use weather::*;

/// Complete Environment container for scenario environmental setup
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Environment {
    #[serde(rename = "@name")]
    pub name: OSString,
    #[serde(rename = "TimeOfDay")]
    pub time_of_day: TimeOfDay,
    #[serde(rename = "Weather")]
    pub weather: Weather,
    #[serde(rename = "RoadCondition")]
    pub road_condition: RoadCondition,
}

/// Time of day settings for lighting and animation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimeOfDay {
    #[serde(rename = "@animation")]
    pub animation: Boolean,
    #[serde(rename = "@dateTime")]
    pub date_time: String, // ISO 8601 datetime format: "2021-12-10T11:00:00"
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            name: crate::types::basic::Value::literal("DefaultEnvironment".to_string()),
            time_of_day: TimeOfDay::default(),
            weather: Weather::default(),
            road_condition: RoadCondition::default(),
        }
    }
}

impl Default for TimeOfDay {
    fn default() -> Self {
        Self {
            animation: crate::types::basic::Value::literal(false),
            date_time: "2021-01-01T12:00:00".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::basic::Value;

    #[test]
    fn test_environment_creation() {
        let environment = Environment {
            name: Value::literal("TestEnvironment".to_string()),
            time_of_day: TimeOfDay {
                animation: Value::literal(false),
                date_time: "2021-12-10T11:00:00".to_string(),
            },
            weather: Weather::default(),
            road_condition: RoadCondition::default(),
        };

        assert_eq!(environment.name.as_literal().unwrap(), "TestEnvironment");
        assert_eq!(environment.time_of_day.date_time, "2021-12-10T11:00:00");
        assert!(!environment.time_of_day.animation.as_literal().unwrap());
    }

    #[test]
    fn test_environment_default() {
        let environment = Environment::default();

        assert_eq!(environment.name.as_literal().unwrap(), "DefaultEnvironment");
        assert_eq!(environment.time_of_day.date_time, "2021-01-01T12:00:00");
        assert!(!environment.time_of_day.animation.as_literal().unwrap());
    }

    #[test]
    fn test_time_of_day_serialization() {
        let time_of_day = TimeOfDay {
            animation: Value::literal(true),
            date_time: "2021-12-10T11:00:00".to_string(),
        };

        let serialized = quick_xml::se::to_string(&time_of_day).unwrap();
        assert!(serialized.contains("animation=\"true\""));
        assert!(serialized.contains("dateTime=\"2021-12-10T11:00:00\""));
    }

    #[test]
    fn test_environment_serialization() {
        let environment = Environment {
            name: Value::literal("TestEnvironment".to_string()),
            time_of_day: TimeOfDay {
                animation: Value::literal(false),
                date_time: "2021-12-10T11:00:00".to_string(),
            },
            weather: Weather::default(),
            road_condition: RoadCondition::default(),
        };

        let serialized = quick_xml::se::to_string(&environment).unwrap();
        assert!(serialized.contains("name=\"TestEnvironment\""));
        assert!(serialized.contains("<TimeOfDay"));
        assert!(serialized.contains("<Weather"));
        assert!(serialized.contains("<RoadCondition"));
    }
}
