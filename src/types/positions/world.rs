//! World coordinate position types for absolute positioning

use crate::types::basic::Double;
use serde::{Deserialize, Serialize};

/// Absolute world position with X, Y, Z coordinates and orientation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldPosition {
    /// X coordinate in meters
    #[serde(rename = "@x")]
    pub x: Double,
    /// Y coordinate in meters
    #[serde(rename = "@y")]
    pub y: Double,
    /// Z coordinate in meters (height)
    #[serde(rename = "@z", skip_serializing_if = "Option::is_none")]
    pub z: Option<Double>,
    /// Heading angle in radians
    #[serde(rename = "@h", skip_serializing_if = "Option::is_none")]
    pub h: Option<Double>,
    /// Pitch angle in radians
    #[serde(rename = "@p", skip_serializing_if = "Option::is_none")]
    pub p: Option<Double>,
    /// Roll angle in radians
    #[serde(rename = "@r", skip_serializing_if = "Option::is_none")]
    pub r: Option<Double>,
}

impl Default for WorldPosition {
    fn default() -> Self {
        Self {
            x: Double::literal(0.0),
            y: Double::literal(0.0),
            z: None,
            h: None,
            p: None,
            r: None,
        }
    }
}