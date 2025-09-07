//! Vehicle entity definition

use serde::{Deserialize, Serialize};
use crate::types::basic::OSString;
use crate::types::enums::VehicleCategory;
use crate::types::geometry::BoundingBox;

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
    
    // TODO: Add these complex fields in later phases:
    // #[serde(rename = "Performance", skip_serializing_if = "Option::is_none")]
    // pub performance: Option<Performance>,
    // #[serde(rename = "Axles", skip_serializing_if = "Option::is_none")] 
    // pub axles: Option<Axles>,
    // #[serde(rename = "Properties", skip_serializing_if = "Option::is_none")]
    // pub properties: Option<Properties>,
}

impl Default for Vehicle {
    fn default() -> Self {
        Self {
            name: crate::types::basic::Value::literal("DefaultVehicle".to_string()),
            vehicle_category: VehicleCategory::Car,
            bounding_box: BoundingBox::default(),
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