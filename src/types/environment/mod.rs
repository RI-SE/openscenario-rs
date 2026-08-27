//! Environment type module for weather, lighting, and road conditions
//!
//! This file contains:
//! - Environment container with complete environmental setup
//! - TimeOfDay settings for lighting and time progression
//! - Weather conditions including sun, fog, and precipitation
//! - Road conditions with friction and surface properties
//! - Integration with rendering and physics systems
//!
use crate::types::basic::{Boolean, OSString, ParameterDeclarations};
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
    #[serde(
        rename = "ParameterDeclarations",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_declarations: Option<ParameterDeclarations>,
    #[serde(rename = "TimeOfDay", default, skip_serializing_if = "Option::is_none")]
    pub time_of_day: Option<TimeOfDay>,
    #[serde(rename = "Weather", default, skip_serializing_if = "Option::is_none")]
    pub weather: Option<Weather>,
    #[serde(
        rename = "RoadCondition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub road_condition: Option<RoadCondition>,
}

/// Time of day settings for lighting and animation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimeOfDay {
    #[serde(rename = "@animation")]
    pub animation: Boolean,
    #[serde(rename = "@dateTime")]
    pub date_time: String, // ISO 8601 datetime format: "2021-12-10T11:00:00"
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::basic::Value;

    #[test]
    fn test_environment_creation() {
        let environment = Environment {
            name: Value::literal("TestEnvironment".to_string()),
            parameter_declarations: None,
            time_of_day: Some(TimeOfDay {
                animation: Value::literal(false),
                date_time: "2021-12-10T11:00:00".to_string(),
            }),
            weather: Some(Weather::default()),
            road_condition: Some(RoadCondition {
                friction_scale_factor: crate::types::basic::Double::literal(1.0),
                wetness: None,
                properties: None,
            }),
        };

        assert_eq!(environment.name.as_literal().unwrap(), "TestEnvironment");
        assert_eq!(
            environment.time_of_day.as_ref().unwrap().date_time,
            "2021-12-10T11:00:00"
        );
        assert!(
            !environment
                .time_of_day
                .as_ref()
                .unwrap()
                .animation
                .as_literal()
                .unwrap()
        );
    }

    #[test]
    fn test_environment_minimal() {
        let xml = r#"<Environment name="e"/>"#;
        let environment: Environment = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(environment.name.as_literal().unwrap(), "e");
        assert!(environment.parameter_declarations.is_none());
        assert!(environment.time_of_day.is_none());
        assert!(environment.weather.is_none());
        assert!(environment.road_condition.is_none());
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
            parameter_declarations: None,
            time_of_day: Some(TimeOfDay {
                animation: Value::literal(false),
                date_time: "2021-12-10T11:00:00".to_string(),
            }),
            weather: Some(Weather::default()),
            road_condition: Some(RoadCondition {
                friction_scale_factor: crate::types::basic::Double::literal(1.0),
                wetness: None,
                properties: None,
            }),
        };

        let serialized = quick_xml::se::to_string(&environment).unwrap();
        assert!(serialized.contains("name=\"TestEnvironment\""));
        assert!(serialized.contains("<TimeOfDay"));
        assert!(serialized.contains("<Weather"));
        assert!(serialized.contains("<RoadCondition"));
    }
}
