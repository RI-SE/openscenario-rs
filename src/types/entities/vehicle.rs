//! Vehicle entity definition

use super::axles::Axles;
use crate::types::basic::{Double, OSString};
use crate::types::enums::VehicleCategory;
use crate::types::geometry::BoundingBox;
use serde::{Deserialize, Serialize};

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

/// Vehicle properties container
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
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
            max_acceleration: Double::literal(10.0),
            max_deceleration: Double::literal(10.0),
        }
    }
}

impl Vehicle {
    /// Create a new car with default specifications
    pub fn new_car(name: String) -> Self {
        Self {
            name: crate::types::basic::Value::literal(name),
            vehicle_category: VehicleCategory::Car,
            bounding_box: BoundingBox::default(),
            performance: Performance::default(),
            axles: Axles::car(),
            properties: None,
        }
    }

    /// Create a new truck with default specifications
    pub fn new_truck(name: String) -> Self {
        Self {
            name: crate::types::basic::Value::literal(name),
            vehicle_category: VehicleCategory::Truck,
            bounding_box: BoundingBox {
                center: crate::types::geometry::Center::default(),
                dimensions: crate::types::geometry::Dimensions::truck_default(),
            },
            performance: Performance {
                max_speed: Double::literal(120.0),
                max_acceleration: Double::literal(3.0),
                max_deceleration: Double::literal(8.0),
            },
            axles: Axles::truck(),
            properties: None,
        }
    }

    /// Create a new motorcycle with default specifications
    pub fn new_motorcycle(name: String) -> Self {
        Self {
            name: crate::types::basic::Value::literal(name),
            vehicle_category: VehicleCategory::Motorbike,
            bounding_box: BoundingBox {
                center: crate::types::geometry::Center::default(),
                dimensions: crate::types::geometry::Dimensions::motorcycle(),
            },
            performance: Performance {
                max_speed: Double::literal(180.0),
                max_acceleration: Double::literal(8.0),
                max_deceleration: Double::literal(12.0),
            },
            axles: Axles::motorcycle(),
            properties: None,
        }
    }

    /// Get the wheelbase of this vehicle
    pub fn wheelbase(
        &self,
        params: &std::collections::HashMap<String, String>,
    ) -> crate::error::Result<f64> {
        self.axles.wheelbase(params)
    }

    /// Check if this vehicle is steerable
    pub fn is_steerable(
        &self,
        params: &std::collections::HashMap<String, String>,
    ) -> crate::error::Result<bool> {
        self.axles.is_steerable(params)
    }

    /// Get the total number of axles
    pub fn axle_count(&self) -> usize {
        self.axles.axle_count()
    }

    /// Calculate the vehicle's footprint area
    pub fn footprint_area(
        &self,
        params: &std::collections::HashMap<String, String>,
    ) -> crate::error::Result<f64> {
        self.bounding_box.dimensions.footprint_area(params)
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
        assert_eq!(
            vehicle.bounding_box.dimensions.width.as_literal().unwrap(),
            &2.0
        );
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

    #[test]
    fn test_vehicle_new_car() {
        let car = Vehicle::new_car("TestCar".to_string());

        assert_eq!(car.name.as_literal().unwrap(), "TestCar");
        assert_eq!(car.vehicle_category, VehicleCategory::Car);
        assert_eq!(car.axle_count(), 2);
    }

    #[test]
    fn test_vehicle_new_truck() {
        let truck = Vehicle::new_truck("TestTruck".to_string());

        assert_eq!(truck.name.as_literal().unwrap(), "TestTruck");
        assert_eq!(truck.vehicle_category, VehicleCategory::Truck);
        assert_eq!(truck.axle_count(), 3); // Front + rear + additional
    }

    #[test]
    fn test_vehicle_new_motorcycle() {
        let motorcycle = Vehicle::new_motorcycle("TestBike".to_string());

        assert_eq!(motorcycle.name.as_literal().unwrap(), "TestBike");
        assert_eq!(motorcycle.vehicle_category, VehicleCategory::Motorbike);
        assert_eq!(motorcycle.axle_count(), 2);
    }

    #[test]
    fn test_vehicle_wheelbase() {
        use std::collections::HashMap;

        let car = Vehicle::new_car("TestCar".to_string());
        let params = HashMap::new();

        let wheelbase = car.wheelbase(&params).unwrap();
        assert!(wheelbase > 0.0);
    }

    #[test]
    fn test_vehicle_is_steerable() {
        use std::collections::HashMap;

        let car = Vehicle::new_car("TestCar".to_string());
        let params = HashMap::new();

        assert!(car.is_steerable(&params).unwrap());
    }

    #[test]
    fn test_vehicle_footprint_area() {
        use std::collections::HashMap;

        let car = Vehicle::new_car("TestCar".to_string());
        let params = HashMap::new();

        let area = car.footprint_area(&params).unwrap();
        assert!(area > 0.0);
    }
}
