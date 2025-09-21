//! Axle system definitions for vehicles
//!
//! This module provides comprehensive axle modeling for OpenSCENARIO vehicles,
//! supporting front, rear, and additional axles with full geometric and steering
//! characteristics as defined in the OpenSCENARIO XSD specification.

use crate::error::Result;
use crate::types::basic::Double;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Complete axle system for a vehicle
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Axles")]
pub struct Axles {
    /// Front axle (optional for some vehicle types)
    #[serde(rename = "FrontAxle", skip_serializing_if = "Option::is_none")]
    pub front_axle: Option<Axle>,

    /// Rear axle (required)
    #[serde(rename = "RearAxle")]
    pub rear_axle: Axle,

    /// Additional axles for multi-axle vehicles (trucks, trailers)
    #[serde(rename = "AdditionalAxle", default)]
    pub additional_axles: Vec<Axle>,
}

/// Individual axle specification with position and wheel characteristics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Axle")]
pub struct Axle {
    /// Maximum steering angle in radians
    #[serde(rename = "@maxSteering")]
    pub max_steering: Double,

    /// Wheel diameter in meters
    #[serde(rename = "@wheelDiameter")]
    pub wheel_diameter: Double,

    /// Track width (distance between wheels) in meters
    #[serde(rename = "@trackWidth")]
    pub track_width: Double,

    /// X position relative to vehicle center in meters
    #[serde(rename = "@positionX")]
    pub position_x: Double,

    /// Z position (height) relative to vehicle center in meters
    #[serde(rename = "@positionZ")]
    pub position_z: Double,
}

impl Axles {
    /// Create a standard car axle configuration
    pub fn car() -> Self {
        Self {
            front_axle: Some(Axle::front_car()),
            rear_axle: Axle::rear_car(),
            additional_axles: vec![],
        }
    }

    /// Create a truck axle configuration with multiple rear axles
    pub fn truck() -> Self {
        Self {
            front_axle: Some(Axle::front_truck()),
            rear_axle: Axle::rear_truck(),
            additional_axles: vec![Axle::additional_truck()],
        }
    }

    /// Create a trailer axle configuration (no front axle)
    pub fn trailer() -> Self {
        Self {
            front_axle: None,
            rear_axle: Axle::rear_truck(),
            additional_axles: vec![],
        }
    }

    /// Create a motorcycle axle configuration
    pub fn motorcycle() -> Self {
        Self {
            front_axle: Some(Axle::front_motorcycle()),
            rear_axle: Axle::rear_motorcycle(),
            additional_axles: vec![],
        }
    }

    /// Get total number of axles
    pub fn axle_count(&self) -> usize {
        let front_count = if self.front_axle.is_some() { 1 } else { 0 };
        1 + front_count + self.additional_axles.len() // rear_axle is always present
    }

    /// Get total wheelbase (distance from front to rear axle)
    pub fn wheelbase(&self, params: &HashMap<String, String>) -> Result<f64> {
        if let Some(front_axle) = &self.front_axle {
            let front_x = front_axle.position_x.resolve(params)?;
            let rear_x = self.rear_axle.position_x.resolve(params)?;
            Ok((front_x - rear_x).abs())
        } else {
            Ok(0.0) // No front axle
        }
    }

    /// Check if this is a steerable vehicle (has front axle with steering capability)
    pub fn is_steerable(&self, params: &HashMap<String, String>) -> Result<bool> {
        if let Some(front_axle) = &self.front_axle {
            let max_steering = front_axle.max_steering.resolve(params)?;
            Ok(max_steering > 0.0)
        } else {
            Ok(false)
        }
    }

    /// Get all axles as a vector for iteration
    pub fn all_axles(&self) -> Vec<&Axle> {
        let mut axles = Vec::new();
        if let Some(front) = &self.front_axle {
            axles.push(front);
        }
        axles.push(&self.rear_axle);
        axles.extend(&self.additional_axles);
        axles
    }
}

impl Axle {
    /// Create a standard car front axle
    pub fn front_car() -> Self {
        Self {
            max_steering: Double::literal(0.5236), // 30 degrees in radians
            wheel_diameter: Double::literal(0.65),
            track_width: Double::literal(1.6),
            position_x: Double::literal(1.4),
            position_z: Double::literal(0.3),
        }
    }

    /// Create a standard car rear axle
    pub fn rear_car() -> Self {
        Self {
            max_steering: Double::literal(0.0), // No steering
            wheel_diameter: Double::literal(0.65),
            track_width: Double::literal(1.6),
            position_x: Double::literal(-1.4),
            position_z: Double::literal(0.3),
        }
    }

    /// Create a truck front axle
    pub fn front_truck() -> Self {
        Self {
            max_steering: Double::literal(0.4363), // 25 degrees in radians
            wheel_diameter: Double::literal(1.0),
            track_width: Double::literal(2.0),
            position_x: Double::literal(3.5),
            position_z: Double::literal(0.5),
        }
    }

    /// Create a truck rear axle
    pub fn rear_truck() -> Self {
        Self {
            max_steering: Double::literal(0.0),
            wheel_diameter: Double::literal(1.0),
            track_width: Double::literal(2.0),
            position_x: Double::literal(-1.5),
            position_z: Double::literal(0.5),
        }
    }

    /// Create an additional truck axle
    pub fn additional_truck() -> Self {
        Self {
            max_steering: Double::literal(0.0),
            wheel_diameter: Double::literal(1.0),
            track_width: Double::literal(2.0),
            position_x: Double::literal(-3.0),
            position_z: Double::literal(0.5),
        }
    }

    /// Create a motorcycle front axle
    pub fn front_motorcycle() -> Self {
        Self {
            max_steering: Double::literal(0.7854), // 45 degrees in radians
            wheel_diameter: Double::literal(0.6),
            track_width: Double::literal(0.0), // Single wheel
            position_x: Double::literal(0.8),
            position_z: Double::literal(0.3),
        }
    }

    /// Create a motorcycle rear axle
    pub fn rear_motorcycle() -> Self {
        Self {
            max_steering: Double::literal(0.0),
            wheel_diameter: Double::literal(0.65),
            track_width: Double::literal(0.0), // Single wheel
            position_x: Double::literal(-0.8),
            position_z: Double::literal(0.3),
        }
    }

    /// Calculate wheel circumference
    pub fn wheel_circumference(&self, params: &HashMap<String, String>) -> Result<f64> {
        let diameter = self.wheel_diameter.resolve(params)?;
        Ok(std::f64::consts::PI * diameter)
    }

    /// Convert steering angle from radians to degrees
    pub fn max_steering_degrees(&self, params: &HashMap<String, String>) -> Result<f64> {
        let radians = self.max_steering.resolve(params)?;
        Ok(radians.to_degrees())
    }

    /// Calculate turning radius for this axle at maximum steering
    pub fn turning_radius(&self, wheelbase: f64, params: &HashMap<String, String>) -> Result<f64> {
        let max_steering = self.max_steering.resolve(params)?;
        if max_steering == 0.0 {
            Ok(f64::INFINITY) // Cannot turn
        } else {
            Ok(wheelbase / max_steering.tan())
        }
    }

    /// Check if this axle has dual wheels (track width > 0)
    pub fn has_dual_wheels(&self, params: &HashMap<String, String>) -> Result<bool> {
        let track_width = self.track_width.resolve(params)?;
        Ok(track_width > 0.0)
    }

    /// Get wheel positions relative to axle center
    pub fn wheel_positions(&self, params: &HashMap<String, String>) -> Result<Vec<(f64, f64)>> {
        let track_width = self.track_width.resolve(params)?;
        let position_x = self.position_x.resolve(params)?;

        if track_width > 0.0 {
            // Dual wheels
            let half_track = track_width / 2.0;
            Ok(vec![
                (position_x, -half_track), // Left wheel
                (position_x, half_track),  // Right wheel
            ])
        } else {
            // Single wheel (motorcycle)
            Ok(vec![(position_x, 0.0)])
        }
    }
}

impl Default for Axles {
    fn default() -> Self {
        Self::car()
    }
}

impl Default for Axle {
    fn default() -> Self {
        Self::rear_car()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_axles_car_configuration() {
        let axles = Axles::car();

        assert!(axles.front_axle.is_some());
        assert_eq!(axles.additional_axles.len(), 0);
        assert_eq!(axles.axle_count(), 2);
    }

    #[test]
    fn test_axles_truck_configuration() {
        let axles = Axles::truck();

        assert!(axles.front_axle.is_some());
        assert_eq!(axles.additional_axles.len(), 1);
        assert_eq!(axles.axle_count(), 3);
    }

    #[test]
    fn test_axles_trailer_configuration() {
        let axles = Axles::trailer();

        assert!(axles.front_axle.is_none());
        assert_eq!(axles.additional_axles.len(), 0);
        assert_eq!(axles.axle_count(), 1);
    }

    #[test]
    fn test_axles_wheelbase() {
        let axles = Axles::car();
        let params = HashMap::new();

        let wheelbase = axles.wheelbase(&params).unwrap();
        assert_eq!(wheelbase, 2.8); // 1.4 - (-1.4)
    }

    #[test]
    fn test_axles_is_steerable() {
        let axles = Axles::car();
        let params = HashMap::new();

        assert!(axles.is_steerable(&params).unwrap());

        let trailer = Axles::trailer();
        assert!(!trailer.is_steerable(&params).unwrap());
    }

    #[test]
    fn test_axle_wheel_circumference() {
        let axle = Axle::front_car();
        let params = HashMap::new();

        let circumference = axle.wheel_circumference(&params).unwrap();
        let expected = std::f64::consts::PI * 0.65;
        assert!((circumference - expected).abs() < 0.001);
    }

    #[test]
    fn test_axle_max_steering_degrees() {
        let axle = Axle::front_car();
        let params = HashMap::new();

        let degrees = axle.max_steering_degrees(&params).unwrap();
        assert!((degrees - 30.0).abs() < 0.1); // Should be approximately 30 degrees
    }

    #[test]
    fn test_axle_turning_radius() {
        let axle = Axle::front_car();
        let params = HashMap::new();
        let wheelbase = 2.8;

        let radius = axle.turning_radius(wheelbase, &params).unwrap();
        assert!(radius > 0.0 && radius < f64::INFINITY);

        // Test non-steerable axle
        let rear_axle = Axle::rear_car();
        let rear_radius = rear_axle.turning_radius(wheelbase, &params).unwrap();
        assert_eq!(rear_radius, f64::INFINITY);
    }

    #[test]
    fn test_axle_has_dual_wheels() {
        let car_axle = Axle::front_car();
        let motorcycle_axle = Axle::front_motorcycle();
        let params = HashMap::new();

        assert!(car_axle.has_dual_wheels(&params).unwrap());
        assert!(!motorcycle_axle.has_dual_wheels(&params).unwrap());
    }

    #[test]
    fn test_axle_wheel_positions() {
        let car_axle = Axle::front_car();
        let motorcycle_axle = Axle::front_motorcycle();
        let params = HashMap::new();

        let car_positions = car_axle.wheel_positions(&params).unwrap();
        assert_eq!(car_positions.len(), 2); // Two wheels
        assert_eq!(car_positions[0], (1.4, -0.8)); // Left wheel
        assert_eq!(car_positions[1], (1.4, 0.8)); // Right wheel

        let motorcycle_positions = motorcycle_axle.wheel_positions(&params).unwrap();
        assert_eq!(motorcycle_positions.len(), 1); // Single wheel
        assert_eq!(motorcycle_positions[0], (0.8, 0.0));
    }

    #[test]
    fn test_axles_all_axles() {
        let truck = Axles::truck();
        let all_axles = truck.all_axles();

        assert_eq!(all_axles.len(), 3); // Front + rear + additional
    }

    #[test]
    fn test_axle_serialization() {
        let axle = Axle::front_car();

        let xml = quick_xml::se::to_string(&axle).unwrap();
        assert!(xml.contains("maxSteering"));
        assert!(xml.contains("wheelDiameter"));
        assert!(xml.contains("trackWidth"));
        assert!(xml.contains("positionX"));
        assert!(xml.contains("positionZ"));
    }

    #[test]
    fn test_axles_serialization() {
        let axles = Axles::car();

        let xml = quick_xml::se::to_string(&axles).unwrap();
        assert!(xml.contains("FrontAxle"));
        assert!(xml.contains("RearAxle"));
    }
}
