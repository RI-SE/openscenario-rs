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
//! Contributes to project by:
//! - Enabling realistic visual representation of scenario entities
//! - Supporting complex animation sequences for pedestrians and vehicles
//! - Providing lighting control for realistic day/night scenarios
//! - Enabling custom animations through extensible user-defined types
//! - Facilitating visual simulation output and rendering integration
//! - Controlling entity visibility for graphics, sensors, and traffic

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
pub struct AppearanceAction {
    /// Light state action for lighting control
    #[serde(rename = "LightStateAction", skip_serializing_if = "Option::is_none")]
    pub light_state_action: Option<LightStateAction>,

    /// Animation action for entity animations
    #[serde(rename = "AnimationAction", skip_serializing_if = "Option::is_none")]
    pub animation_action: Option<AnimationAction>,
}

/// Light state control action (placeholder for future implementation)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LightStateAction {
    // TODO: Implement based on schema definition
}

/// Animation action (placeholder for future implementation)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnimationAction {
    // TODO: Implement based on schema definition
}

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

impl Default for SensorReferenceSet {
    fn default() -> Self {
        Self {
            sensor_references: Vec::new(),
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

impl Default for AppearanceAction {
    fn default() -> Self {
        Self {
            light_state_action: None,
            animation_action: None,
        }
    }
}

impl Default for LightStateAction {
    fn default() -> Self {
        Self {}
    }
}

impl Default for AnimationAction {
    fn default() -> Self {
        Self {}
    }
}
