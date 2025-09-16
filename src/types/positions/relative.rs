//! Relative position types for object-relative positioning

use crate::types::basic::{Double, OSString};
use serde::{Deserialize, Serialize};

/// Position relative to another object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "RelativeObjectPosition")]
pub struct RelativeObjectPosition {
    /// Reference entity for relative positioning
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,
    
    /// Delta X-coordinate in the object's coordinate system
    #[serde(rename = "@dx")]
    pub dx: Double,
    
    /// Delta Y-coordinate in the object's coordinate system
    #[serde(rename = "@dy")]
    pub dy: Double,
    
    /// Delta Z-coordinate in the object's coordinate system (optional)
    #[serde(rename = "@dz", skip_serializing_if = "Option::is_none")]
    pub dz: Option<Double>,
    
    /// Orientation relative to reference object
    #[serde(rename = "Orientation", skip_serializing_if = "Option::is_none")]
    pub orientation: Option<crate::types::positions::road::Orientation>,
}

impl RelativeObjectPosition {
    /// Create a new relative object position
    pub fn new(entity_ref: &str, dx: f64, dy: f64) -> Self {
        Self {
            entity_ref: OSString::literal(entity_ref.to_string()),
            dx: Double::literal(dx),
            dy: Double::literal(dy),
            dz: None,
            orientation: None,
        }
    }
    
    /// Create relative position with Z offset
    pub fn with_z(entity_ref: &str, dx: f64, dy: f64, dz: f64) -> Self {
        Self {
            entity_ref: OSString::literal(entity_ref.to_string()),
            dx: Double::literal(dx),
            dy: Double::literal(dy),
            dz: Some(Double::literal(dz)),
            orientation: None,
        }
    }
    
    /// Add orientation to relative position
    pub fn with_orientation(mut self, orientation: crate::types::positions::road::Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }
    
    /// Create relative position behind entity
    pub fn behind(entity_ref: &str, distance: f64) -> Self {
        Self::new(entity_ref, -distance, 0.0)
    }
    
    /// Create relative position in front of entity
    pub fn ahead(entity_ref: &str, distance: f64) -> Self {
        Self::new(entity_ref, distance, 0.0)
    }
    
    /// Create relative position to the left of entity
    pub fn left_of(entity_ref: &str, distance: f64) -> Self {
        Self::new(entity_ref, 0.0, distance)
    }
    
    /// Create relative position to the right of entity
    pub fn right_of(entity_ref: &str, distance: f64) -> Self {
        Self::new(entity_ref, 0.0, -distance)
    }
    
    /// Create relative position at offset with full coordinates
    pub fn at_offset(entity_ref: &str, dx: f64, dy: f64, dz: f64) -> Self {
        Self::with_z(entity_ref, dx, dy, dz)
    }
}

impl Default for RelativeObjectPosition {
    fn default() -> Self {
        Self {
            entity_ref: OSString::literal("DefaultEntity".to_string()),
            dx: Double::literal(0.0),
            dy: Double::literal(0.0),
            dz: None,
            orientation: None,
        }
    }
}