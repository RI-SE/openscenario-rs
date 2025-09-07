//! Vehicle entity definition

use serde::{Deserialize, Serialize};
use crate::types::basic::{OSString, Double};
use crate::types::enums::VehicleCategory;
use crate::types::geometry::BoundingBox;

/// Vehicle performance characteristics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Performance {
    #[serde(rename = "@maxSpeed")]
    pub max_speed: Double,
    #[serde(rename = "@maxAcceleration")]
    pub max_acceleration: Double,
    #[serde(rename = "@maxDeceleration")]
    pub max_deceleration: Double,
}

/// Axle definitions for vehicle
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Axles {
    #[serde(rename = "FrontAxle")]
    pub front_axle: FrontAxle,
    #[serde(rename = "RearAxle")]
    pub rear_axle: RearAxle,
}

/// Front axle specification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrontAxle {
    #[serde(rename = "@maxSteering")]
    pub max_steering: Double,
    #[serde(rename = "@wheelDiameter")]
    pub wheel_diameter: Double,
    #[serde(rename = "@trackWidth")]
    pub track_width: Double,
    #[serde(rename = "@positionX")]
    pub position_x: Double,
    #[serde(rename = "@positionZ")]
    pub position_z: Double,
}

/// Rear axle specification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RearAxle {
    #[serde(rename = "@maxSteering")]
    pub max_steering: Double,
    #[serde(rename = "@wheelDiameter")]
    pub wheel_diameter: Double,
    #[serde(rename = "@trackWidth")]
    pub track_width: Double,
    #[serde(rename = "@positionX")]
    pub position_x: Double,
    #[serde(rename = "@positionZ")]
    pub position_z: Double,
}

/// Vehicle properties container
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Properties {
    #[serde(rename = "Property", default)]
    pub properties: Vec<Property>,
    #[serde(rename = "File", default)]
    pub files: Vec<File>,
}

/// Property key-value pair
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Property {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@value")]
    pub value: String,
}

/// File reference
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct File {
    #[serde(rename = "@filepath")]
    pub filepath: String,
}

/// Vehicle entity definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vehicle {
    /// Name of the vehicle
    #[serde(rename = "@name")]
    pub name: OSString,
    
    /// Category of the vehicle (car, truck, bus, etc.)
    #[serde(rename = "@vehicleCategory")]
    pub vehicle_category: VehicleCategory,
    
    /// Bounding box defining the vehicle's spatial extents
    #[serde(rename = "BoundingBox")]
    pub bounding_box: BoundingBox,
    
    /// Vehicle performance characteristics
    #[serde(rename = "Performance")]
    pub performance: Performance,
    
    /// Axle definitions
    #[serde(rename = "Axles")]
    pub axles: Axles,
    
    /// Vehicle properties
    #[serde(rename = "Properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Properties>,
}

impl Default for Performance {
    fn default() -> Self {
        Self {
            max_speed: Double::literal(200.0),
            max_acceleration: Double::literal(200.0),
            max_deceleration: Double::literal(10.0),
        }
    }
}

impl Default for FrontAxle {
    fn default() -> Self {
        Self {
            max_steering: Double::literal(0.5),
            wheel_diameter: Double::literal(0.5),
            track_width: Double::literal(1.75),
            position_x: Double::literal(2.8),
            position_z: Double::literal(0.25),
        }
    }
}

impl Default for RearAxle {
    fn default() -> Self {
        Self {
            max_steering: Double::literal(0.0),
            wheel_diameter: Double::literal(0.5),
            track_width: Double::literal(1.75),
            position_x: Double::literal(0.0),
            position_z: Double::literal(0.25),
        }
    }
}

impl Default for Axles {
    fn default() -> Self {
        Self {
            front_axle: FrontAxle::default(),
            rear_axle: RearAxle::default(),
        }
    }
}

impl Default for Properties {
    fn default() -> Self {
        Self {
            properties: Vec::new(),
            files: Vec::new(),
        }
    }
}

impl Default for Vehicle {
    fn default() -> Self {
        Self {
            name: crate::types::basic::Value::literal("DefaultVehicle".to_string()),
            vehicle_category: VehicleCategory::Car,
            bounding_box: BoundingBox::default(),
            performance: Performance::default(),
            axles: Axles::default(),
            properties: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vehicle_default() {
        let vehicle = Vehicle::default();
        
        assert_eq!(vehicle.name.as_literal().unwrap(), "DefaultVehicle");
        assert_eq!(vehicle.vehicle_category, VehicleCategory::Car);
        
        // Should have default bounding box
        assert_eq!(vehicle.bounding_box.dimensions.width.as_literal().unwrap(), &2.0);
    }

    #[test]
    fn test_vehicle_creation() {
        let vehicle = Vehicle {
            name: crate::types::basic::Value::literal("TestCar".to_string()),
            vehicle_category: VehicleCategory::Car,
            bounding_box: BoundingBox::default(),
            performance: Performance::default(),
            axles: Axles::default(),
            properties: None,
        };
        
        assert_eq!(vehicle.name.as_literal().unwrap(), "TestCar");
        assert_eq!(vehicle.vehicle_category, VehicleCategory::Car);
    }

    #[test]
    fn test_vehicle_serialization() {
        let vehicle = Vehicle::default();
        
        // Test that serialization works
        let xml = quick_xml::se::to_string(&vehicle).unwrap();
        assert!(xml.contains("name=\"DefaultVehicle\""));
        assert!(xml.contains("vehicleCategory=\"car\""));
        assert!(xml.contains("BoundingBox"));
    }
}