//! World coordinate position types for absolute positioning

use crate::types::basic::Double;
use serde::{Deserialize, Serialize};

/// Absolute world position with X, Y, Z coordinates and orientation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldPosition {
    /// X coordinate in meters
    pub x: Double,
    /// Y coordinate in meters
    pub y: Double,
    /// Z coordinate in meters (height)
    pub z: Option<Double>,
    /// Heading angle in radians
    pub h: Option<Double>,
    /// Pitch angle in radians
    pub p: Option<Double>,
    /// Roll angle in radians
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