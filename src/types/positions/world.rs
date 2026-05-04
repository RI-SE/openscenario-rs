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

impl WorldPosition {
    /// Create a new WorldPosition with required x, y coordinates
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x: Double::literal(x),
            y: Double::literal(y),
            z: None,
            h: None,
            p: None,
            r: None,
        }
    }

    /// Create a new WorldPosition with x, y, z coordinates
    pub fn with_z(x: f64, y: f64, z: f64) -> Self {
        Self {
            x: Double::literal(x),
            y: Double::literal(y),
            z: Some(Double::literal(z)),
            h: None,
            p: None,
            r: None,
        }
    }

    /// Create a new WorldPosition with x, y, z, h coordinates
    pub fn with_orientation(x: f64, y: f64, z: f64, h: f64) -> Self {
        Self {
            x: Double::literal(x),
            y: Double::literal(y),
            z: Some(Double::literal(z)),
            h: Some(Double::literal(h)),
            p: None,
            r: None,
        }
    }

    /// Create a new WorldPosition with all coordinates
    pub fn with_full_orientation(x: f64, y: f64, z: f64, h: f64, p: f64, r: f64) -> Self {
        Self {
            x: Double::literal(x),
            y: Double::literal(y),
            z: Some(Double::literal(z)),
            h: Some(Double::literal(h)),
            p: Some(Double::literal(p)),
            r: Some(Double::literal(r)),
        }
    }
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

/// Geographic position using latitude/longitude coordinates
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "GeographicPosition")]
pub struct GeographicPosition {
    /// Latitude in degrees
    #[serde(rename = "@latitude")]
    pub latitude: Double,

    /// Longitude in degrees
    #[serde(rename = "@longitude")]
    pub longitude: Double,

    /// Height above sea level in meters (optional)
    #[serde(rename = "@height", skip_serializing_if = "Option::is_none")]
    pub height: Option<Double>,

    /// Orientation in geographic coordinate system
    #[serde(rename = "Orientation", skip_serializing_if = "Option::is_none")]
    pub orientation: Option<crate::types::positions::road::Orientation>,
}

impl GeographicPosition {
    /// Create a new geographic position with latitude and longitude
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude: Double::literal(latitude),
            longitude: Double::literal(longitude),
            height: None,
            orientation: None,
        }
    }

    /// Create geographic position with height
    pub fn with_height(latitude: f64, longitude: f64, height: f64) -> Self {
        Self {
            latitude: Double::literal(latitude),
            longitude: Double::literal(longitude),
            height: Some(Double::literal(height)),
            orientation: None,
        }
    }

    /// Add orientation to geographic position
    pub fn with_orientation(
        mut self,
        orientation: crate::types::positions::road::Orientation,
    ) -> Self {
        self.orientation = Some(orientation);
        self
    }

    /// Create position at coordinates with height and orientation
    pub fn at_coordinates(latitude: f64, longitude: f64, height: f64, heading: f64) -> Self {
        use crate::types::positions::road::Orientation;

        let orientation = Orientation {
            h: Some(Double::literal(heading)),
            p: None,
            r: None,
        };

        Self::with_height(latitude, longitude, height).with_orientation(orientation)
    }
}

impl Default for GeographicPosition {
    fn default() -> Self {
        Self {
            latitude: Double::literal(0.0),
            longitude: Double::literal(0.0),
            height: None,
            orientation: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_position_new() {
        let pos = WorldPosition::new(10.0, 20.0);
        assert_eq!(pos.x.as_literal().unwrap(), &10.0);
        assert_eq!(pos.y.as_literal().unwrap(), &20.0);
        assert!(pos.z.is_none());
        assert!(pos.h.is_none());
    }

    #[test]
    fn test_world_position_with_full_orientation() {
        let pos = WorldPosition::with_full_orientation(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        assert_eq!(pos.x.as_literal().unwrap(), &1.0);
        assert_eq!(pos.z.unwrap().as_literal().unwrap(), &3.0);
        assert_eq!(pos.r.unwrap().as_literal().unwrap(), &6.0);
    }

    #[test]
    fn test_world_position_xml_roundtrip() {
        let pos = WorldPosition::new(100.5, -50.3);
        let xml = quick_xml::se::to_string(&pos).unwrap();
        assert!(xml.contains("x=\"100.5\""));
        assert!(xml.contains("y=\"-50.3\""));
        let deserialized: WorldPosition = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(pos, deserialized);
    }

    #[test]
    fn test_world_position_optional_fields_not_serialized() {
        let pos = WorldPosition::default();
        let xml = quick_xml::se::to_string(&pos).unwrap();
        assert!(!xml.contains("z="));
        assert!(!xml.contains("h="));
    }

    #[test]
    fn test_geographic_position_new() {
        let pos = GeographicPosition::new(48.137, 11.576);
        assert_eq!(pos.latitude.as_literal().unwrap(), &48.137);
        assert_eq!(pos.longitude.as_literal().unwrap(), &11.576);
        assert!(pos.height.is_none());
    }

    #[test]
    fn test_geographic_position_at_coordinates() {
        let pos = GeographicPosition::at_coordinates(48.0, 11.0, 500.0, 1.57);
        assert!(pos.height.is_some());
        assert!(pos.orientation.is_some());
    }

    #[test]
    fn test_geographic_position_xml_roundtrip() {
        let pos = GeographicPosition::with_height(48.0, 11.0, 500.0);
        let xml = quick_xml::se::to_string(&pos).unwrap();
        let deserialized: GeographicPosition = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(pos, deserialized);
    }
}
