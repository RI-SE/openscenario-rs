//! Vehicle entity definition

use super::axles::Axles;
use crate::types::basic::{Double, OSString, ParameterDeclarations};
use crate::types::enums::{Role, VehicleCategory};
use crate::types::geometry::BoundingBox;
use crate::types::scenario::story::EntityRef;
use serde::{Deserialize, Serialize};

/// Vehicle performance characteristics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Performance {
    #[serde(rename = "@maxSpeed")]
    pub max_speed: Double,
    #[serde(rename = "@maxAcceleration")]
    pub max_acceleration: Double,
    #[serde(
        rename = "@maxAccelerationRate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_acceleration_rate: Option<Double>,
    #[serde(rename = "@maxDeceleration")]
    pub max_deceleration: Double,
    #[serde(
        rename = "@maxDecelerationRate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_deceleration_rate: Option<Double>,
}

/// Attachment point for a towing vehicle's trailer hitch
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrailerHitch {
    #[serde(rename = "@dx")]
    pub dx: Double,
    #[serde(rename = "@dz", default, skip_serializing_if = "Option::is_none")]
    pub dz: Option<Double>,
}

/// Attachment point for a trailer's coupler
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrailerCoupler {
    #[serde(rename = "@dx")]
    pub dx: Double,
    #[serde(rename = "@dz", default, skip_serializing_if = "Option::is_none")]
    pub dz: Option<Double>,
}

/// Trailer attached to a vehicle: either an inline nested ScenarioObject or a
/// reference to an existing entity acting as the trailer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trailer {
    /// Inline scenario object defining the trailer (boxed to break recursion)
    #[serde(rename = "Trailer", default, skip_serializing_if = "Option::is_none")]
    pub trailer: Option<Box<crate::types::entities::ScenarioObject>>,

    /// Reference to an existing entity acting as the trailer
    #[serde(
        rename = "TrailerRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailer_ref: Option<EntityRef>,
}

/// Vehicle properties container
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Properties {
    #[serde(rename = "Property", default)]
    pub properties: Vec<Property>,
    #[serde(rename = "File", default)]
    pub files: Vec<File>,
    #[serde(rename = "CustomContent", default)]
    pub custom_content: Vec<CustomContent>,
}

/// Arbitrary vendor-specific content — XSD `CustomContent` (`:1016`), `simpleContent`
/// extending `xsd:string` with no attributes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomContent {
    #[serde(rename = "$text", default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
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

    /// Role of the vehicle (e.g. ambulance, police)
    #[serde(rename = "@role", default, skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,

    /// Mass of the vehicle in kg
    #[serde(rename = "@mass", default, skip_serializing_if = "Option::is_none")]
    pub mass: Option<Double>,

    /// Path to an external 3D model
    #[serde(rename = "@model3d", default, skip_serializing_if = "Option::is_none")]
    pub model3d: Option<OSString>,

    /// Parameter declarations
    #[serde(
        rename = "ParameterDeclarations",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_declarations: Option<ParameterDeclarations>,

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

    /// Trailer hitch attachment point
    #[serde(
        rename = "TrailerHitch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailer_hitch: Option<TrailerHitch>,

    /// Trailer coupler attachment point
    #[serde(
        rename = "TrailerCoupler",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailer_coupler: Option<TrailerCoupler>,

    /// Attached trailer (inline definition or reference)
    #[serde(rename = "Trailer", default, skip_serializing_if = "Option::is_none")]
    pub trailer: Option<Trailer>,
}

impl Vehicle {
    /// Create a new car with default specifications
    pub fn new_car(name: String) -> Self {
        Self {
            name: crate::types::basic::Value::literal(name),
            vehicle_category: VehicleCategory::Car,
            role: None,
            mass: None,
            model3d: None,
            parameter_declarations: None,
            bounding_box: BoundingBox::default(),
            performance: Performance {
                max_speed: Double::literal(200.0),
                max_acceleration: Double::literal(10.0),
                max_acceleration_rate: None,
                max_deceleration: Double::literal(10.0),
                max_deceleration_rate: None,
            },
            axles: Axles::car(),
            properties: None,
            trailer_hitch: None,
            trailer_coupler: None,
            trailer: None,
        }
    }

    /// Create a new truck with default specifications
    pub fn new_truck(name: String) -> Self {
        Self {
            name: crate::types::basic::Value::literal(name),
            vehicle_category: VehicleCategory::Truck,
            role: None,
            mass: None,
            model3d: None,
            parameter_declarations: None,
            bounding_box: BoundingBox {
                center: crate::types::geometry::Center::default(),
                dimensions: crate::types::geometry::Dimensions::truck_default(),
            },
            performance: Performance {
                max_speed: Double::literal(120.0),
                max_acceleration: Double::literal(3.0),
                max_acceleration_rate: None,
                max_deceleration: Double::literal(8.0),
                max_deceleration_rate: None,
            },
            axles: Axles::truck(),
            properties: None,
            trailer_hitch: None,
            trailer_coupler: None,
            trailer: None,
        }
    }

    /// Create a new motorcycle with default specifications
    pub fn new_motorcycle(name: String) -> Self {
        Self {
            name: crate::types::basic::Value::literal(name),
            vehicle_category: VehicleCategory::Motorbike,
            role: None,
            mass: None,
            model3d: None,
            parameter_declarations: None,
            bounding_box: BoundingBox {
                center: crate::types::geometry::Center::default(),
                dimensions: crate::types::geometry::Dimensions::motorcycle(),
            },
            performance: Performance {
                max_speed: Double::literal(180.0),
                max_acceleration: Double::literal(8.0),
                max_acceleration_rate: None,
                max_deceleration: Double::literal(12.0),
                max_deceleration_rate: None,
            },
            axles: Axles::motorcycle(),
            properties: None,
            trailer_hitch: None,
            trailer_coupler: None,
            trailer: None,
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
            role: None,
            mass: None,
            model3d: None,
            parameter_declarations: None,
            bounding_box: BoundingBox::default(),
            performance: Performance {
                max_speed: Double::literal(200.0),
                max_acceleration: Double::literal(10.0),
                max_acceleration_rate: None,
                max_deceleration: Double::literal(10.0),
                max_deceleration_rate: None,
            },
            axles: Axles::default(),
            properties: None,
            trailer_hitch: None,
            trailer_coupler: None,
            trailer: None,
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
            role: None,
            mass: None,
            model3d: None,
            parameter_declarations: None,
            bounding_box: BoundingBox::default(),
            performance: Performance {
                max_speed: Double::literal(200.0),
                max_acceleration: Double::literal(10.0),
                max_acceleration_rate: None,
                max_deceleration: Double::literal(10.0),
                max_deceleration_rate: None,
            },
            axles: Axles::default(),
            properties: None,
            trailer_hitch: None,
            trailer_coupler: None,
            trailer: None,
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

    #[test]
    fn test_vehicle_trailer_ref_roundtrip() {
        let mut car = Vehicle::new_car("TowCar".to_string());
        car.trailer = Some(Trailer {
            trailer: None,
            trailer_ref: Some(EntityRef {
                entity_ref: crate::types::basic::Value::literal("trailer1".to_string()),
            }),
        });

        let xml = quick_xml::se::to_string(&car).unwrap();
        assert!(xml.contains("<Trailer>"));
        assert!(xml.contains("TrailerRef"));
        assert!(xml.contains("entityRef=\"trailer1\""));

        let deserialized: Vehicle = quick_xml::de::from_str(&xml).unwrap();
        let trailer = deserialized.trailer.expect("expected trailer");
        assert!(trailer.trailer.is_none());
        let trailer_ref = trailer.trailer_ref.expect("expected trailer ref");
        assert_eq!(trailer_ref.entity_ref.as_literal().unwrap(), "trailer1");
    }

    #[test]
    fn test_vehicle_nested_trailer_roundtrip() {
        let mut towed_car = Vehicle::new_car("TrailerVehicle".to_string());
        towed_car.trailer = None;

        let nested_scenario_object =
            crate::types::entities::ScenarioObject::new_vehicle("Trailer1".to_string(), towed_car);

        let mut tow_car = Vehicle::new_car("TowCar".to_string());
        tow_car.trailer = Some(Trailer {
            trailer: Some(Box::new(nested_scenario_object)),
            trailer_ref: None,
        });

        let xml = quick_xml::se::to_string(&tow_car).unwrap();
        assert!(xml.contains("<Trailer>"));
        assert!(xml.contains("Trailer1"));

        let deserialized: Vehicle = quick_xml::de::from_str(&xml).unwrap();
        let trailer = deserialized.trailer.expect("expected trailer");
        assert!(trailer.trailer_ref.is_none());
        let nested = trailer.trailer.expect("expected nested scenario object");
        assert_eq!(nested.get_name(), Some("Trailer1"));
        assert!(nested.vehicle.is_some());
    }

    #[test]
    fn test_properties_custom_content_round_trip() {
        let xml = r#"<Properties>
    <Property name="test" value="1"/>
    <CustomContent>some vendor content</CustomContent>
</Properties>"#;
        let properties: Properties = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(properties.custom_content.len(), 1);
        assert_eq!(
            properties.custom_content[0].content.as_deref(),
            Some("some vendor content")
        );

        let serialized = quick_xml::se::to_string(&properties).unwrap();
        assert!(
            serialized.contains("<CustomContent>some vendor content</CustomContent>"),
            "serialized: {serialized}"
        );
        let reparsed: Properties = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(properties, reparsed);
    }
}
