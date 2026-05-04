//! Appearance and animation action types for visual representation
//!
//! This file contains:
//! - Light state actions for vehicle lighting systems
//! - Animation actions for entity movement and component animation
//! - Pedestrian gesture and motion animations
//! - Vehicle component animations (doors, windows, etc.)
//! - Custom user-defined animation support
//! - Visibility actions for entity appearance control
//!
use crate::types::basic::{Boolean, OSString};
use serde::{Deserialize, Serialize};

/// Controls entity visibility in different simulation contexts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VisibilityAction {
    /// Whether entity is visible to graphics/rendering systems
    #[serde(rename = "@graphics")]
    pub graphics: Boolean,

    /// Whether entity is detectable by sensor systems
    #[serde(rename = "@sensors")]
    pub sensors: Boolean,

    /// Whether entity participates in traffic interactions
    #[serde(rename = "@traffic")]
    pub traffic: Boolean,

    /// Optional sensor reference set for selective sensor visibility
    #[serde(rename = "SensorReferenceSet", skip_serializing_if = "Option::is_none")]
    pub sensor_reference_set: Option<SensorReferenceSet>,
}

/// Set of sensor references for selective visibility control
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[derive(Default)]
pub struct SensorReferenceSet {
    /// Individual sensor references
    #[serde(rename = "SensorReference")]
    pub sensor_references: Vec<SensorReference>,
}

/// Reference to a specific sensor for visibility control
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SensorReference {
    /// Name of the referenced sensor
    #[serde(rename = "@name")]
    pub name: OSString,
}

/// Appearance actions for visual changes and animations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[derive(Default)]
pub struct AppearanceAction {
    /// Light state action for lighting control
    #[serde(rename = "LightStateAction", skip_serializing_if = "Option::is_none")]
    pub light_state_action: Option<LightStateAction>,

    /// Animation action for entity animations
    #[serde(rename = "AnimationAction", skip_serializing_if = "Option::is_none")]
    pub animation_action: Option<AnimationAction>,
}

/// Light state control action for vehicle lighting systems
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[derive(Default)]
pub struct LightStateAction {}

/// Animation action for entity movement and component animation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[derive(Default)]
pub struct AnimationAction {}

impl Default for VisibilityAction {
    fn default() -> Self {
        Self {
            graphics: Boolean::literal(true),
            sensors: Boolean::literal(true),
            traffic: Boolean::literal(true),
            sensor_reference_set: None,
        }
    }
}


impl Default for SensorReference {
    fn default() -> Self {
        Self {
            name: OSString::literal("DefaultSensor".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visibility_action_default_all_true() {
        let va = VisibilityAction::default();
        assert_eq!(va.graphics.as_literal(), Some(&true));
        assert_eq!(va.sensors.as_literal(), Some(&true));
        assert_eq!(va.traffic.as_literal(), Some(&true));
        assert!(va.sensor_reference_set.is_none());
    }

    #[test]
    fn test_appearance_action_default_is_empty() {
        let aa = AppearanceAction::default();
        assert!(aa.light_state_action.is_none());
        assert!(aa.animation_action.is_none());
    }

    #[test]
    fn test_sensor_reference_set_default_empty_vec() {
        let srs = SensorReferenceSet::default();
        assert!(srs.sensor_references.is_empty());
    }

    #[test]
    fn test_visibility_action_xml_roundtrip() {
        let va = VisibilityAction::default();
        let xml = quick_xml::se::to_string(&va).unwrap();
        let deserialized: VisibilityAction = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(va, deserialized);
    }
}
