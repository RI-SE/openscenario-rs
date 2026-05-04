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
    pub fn with_orientation(
        mut self,
        orientation: crate::types::positions::road::Orientation,
    ) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relative_position_new() {
        let pos = RelativeObjectPosition::new("ego", 5.0, 3.0);
        assert_eq!(pos.entity_ref.as_literal().unwrap(), "ego");
        assert_eq!(pos.dx.as_literal().unwrap(), &5.0);
        assert_eq!(pos.dy.as_literal().unwrap(), &3.0);
        assert!(pos.dz.is_none());
    }

    #[test]
    fn test_relative_position_behind() {
        let pos = RelativeObjectPosition::behind("ego", 10.0);
        assert_eq!(pos.dx.as_literal().unwrap(), &-10.0);
        assert_eq!(pos.dy.as_literal().unwrap(), &0.0);
    }

    #[test]
    fn test_relative_position_ahead() {
        let pos = RelativeObjectPosition::ahead("ego", 10.0);
        assert_eq!(pos.dx.as_literal().unwrap(), &10.0);
    }

    #[test]
    fn test_relative_position_left_right() {
        let left = RelativeObjectPosition::left_of("ego", 3.5);
        assert_eq!(left.dy.as_literal().unwrap(), &3.5);

        let right = RelativeObjectPosition::right_of("ego", 3.5);
        assert_eq!(right.dy.as_literal().unwrap(), &-3.5);
    }

    #[test]
    fn test_relative_position_xml_roundtrip() {
        let pos = RelativeObjectPosition::with_z("car1", 1.0, 2.0, 3.0);
        let xml = quick_xml::se::to_string(&pos).unwrap();
        assert!(xml.contains("entityRef=\"car1\""));
        let deserialized: RelativeObjectPosition = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(pos, deserialized);
    }
}
