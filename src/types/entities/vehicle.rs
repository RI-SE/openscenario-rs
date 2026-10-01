//! Vehicle entity definition

use super::axles::Axles;
use crate::types::basic::{Double, OSString, ParameterDeclarations, Value};
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
///
/// XSD `Trailer` (`:2336-2341`): a bare `xsd:choice` of `Trailer` (type `ScenarioObject`) |
/// `TrailerRef` (type `EntityRef`), no occurrence attributes, so exactly one branch is
/// required. `$value` reads the branch from the live reader by element name; parallel
/// `Option` fields would let both branches populate at once and both re-serialize, which
/// no schema-valid document can express.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trailer {
    #[serde(rename = "$value")]
    pub choice: TrailerChoice,
}

/// The branch selected by a `Trailer`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum TrailerChoice {
    /// Inline scenario object defining the trailer (boxed to break recursion)
    Trailer(Box<crate::types::entities::ScenarioObject>),
    /// Reference to an existing entity acting as the trailer
    TrailerRef(EntityRef),
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

/// Property key-value pair. XSD `Property` (`:1809-1812`) types both `@name` and
/// `@value` as the schema's `String`, which is a union including the parameter member,
/// so either attribute may hold `$paramName` or `${expr}` rather than a literal.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Property {
    #[serde(rename = "@name")]
    pub name: OSString,
    #[serde(rename = "@value")]
    pub value: OSString,
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
    pub vehicle_category: Value<VehicleCategory>,

    /// Role of the vehicle (e.g. ambulance, police)
    #[serde(rename = "@role", default, skip_serializing_if = "Option::is_none")]
    pub role: Option<Value<Role>>,

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
            vehicle_category: Value::Literal(VehicleCategory::Car),
            role: None,
            mass: None,
            model3d: None,
            parameter_declarations: None,
            bounding_box: BoundingBox::new(
                crate::types::geometry::Center::new(0.0, 0.0, 0.0),
                crate::types::geometry::Dimensions::new(2.0, 4.5, 1.5),
            ),
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
            vehicle_category: Value::Literal(VehicleCategory::Truck),
            role: None,
            mass: None,
            model3d: None,
            parameter_declarations: None,
            bounding_box: BoundingBox {
                center: crate::types::geometry::Center::new(0.0, 0.0, 0.0),
                dimensions: crate::types::geometry::Dimensions::truck(),
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
            vehicle_category: Value::Literal(VehicleCategory::Motorbike),
            role: None,
            mass: None,
            model3d: None,
            parameter_declarations: None,
            bounding_box: BoundingBox {
                center: crate::types::geometry::Center::new(0.0, 0.0, 0.0),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn presets_set_category_axles_and_footprint() {
        let params = std::collections::HashMap::new();
        // (preset, category, axle count, wheelbase, footprint = width * length, height)
        let cases = [
            (
                Vehicle::new_car("C".into()),
                VehicleCategory::Car,
                2,
                2.8,
                2.0 * 4.5,
                1.5,
            ),
            (
                Vehicle::new_truck("T".into()),
                VehicleCategory::Truck,
                3,
                5.0,
                2.5 * 12.0,
                3.8,
            ),
            (
                Vehicle::new_motorcycle("M".into()),
                VehicleCategory::Motorbike,
                2,
                1.6,
                0.8 * 2.2,
                1.3,
            ),
        ];
        for (vehicle, category, axles, wheelbase, footprint, height) in cases {
            let name = vehicle.name.as_literal().unwrap().clone();
            assert_eq!(vehicle.vehicle_category, Value::Literal(category), "{name}");
            assert_eq!(vehicle.axle_count(), axles, "{name}: axle_count");
            assert_eq!(vehicle.wheelbase(&params).unwrap(), wheelbase, "{name}");
            assert!(vehicle.is_steerable(&params).unwrap(), "{name}: steerable");
            assert_eq!(
                vehicle.footprint_area(&params).unwrap(),
                footprint,
                "{name}"
            );
            assert_eq!(
                vehicle.bounding_box.dimensions.height.as_literal().unwrap(),
                &height,
                "{name}: height"
            );
        }
    }

    #[test]
    fn test_vehicle_trailer_ref_roundtrip() {
        let mut car = Vehicle::new_car("TowCar".to_string());
        car.trailer = Some(Trailer {
            choice: TrailerChoice::TrailerRef(EntityRef {
                entity_ref: crate::types::basic::Value::literal("trailer1".to_string()),
            }),
        });

        let xml = quick_xml::se::to_string(&car).unwrap();
        assert!(xml.contains("<Trailer>"));
        assert!(xml.contains("TrailerRef"));
        assert!(xml.contains("entityRef=\"trailer1\""));

        let deserialized: Vehicle = quick_xml::de::from_str(&xml).unwrap();
        let trailer = deserialized.trailer.expect("expected trailer");
        let trailer_ref = match trailer.choice {
            TrailerChoice::TrailerRef(r) => r,
            other => panic!("expected TrailerRef, got {other:?}"),
        };
        assert_eq!(trailer_ref.entity_ref.as_literal().unwrap(), "trailer1");
    }

    #[test]
    fn test_vehicle_trailer_zero_branches_rejected() {
        let xml = "<Trailer></Trailer>";
        let err = quick_xml::de::from_str::<Trailer>(xml).unwrap_err();
        assert!(
            err.to_string().contains("missing field `$value`"),
            "got: {err}"
        );
    }

    #[test]
    fn test_vehicle_trailer_two_branches_rejected() {
        let xml = r#"<Trailer><TrailerRef entityRef="e1"/><TrailerRef entityRef="e2"/></Trailer>"#;
        let err = quick_xml::de::from_str::<Trailer>(xml).unwrap_err();
        assert!(
            err.to_string().contains("duplicate field `$value`"),
            "got: {err}"
        );
    }

    #[test]
    fn test_vehicle_nested_trailer_roundtrip() {
        let mut towed_car = Vehicle::new_car("TrailerVehicle".to_string());
        towed_car.trailer = None;

        let nested_scenario_object =
            crate::types::entities::ScenarioObject::new_vehicle("Trailer1".to_string(), towed_car);

        let mut tow_car = Vehicle::new_car("TowCar".to_string());
        tow_car.trailer = Some(Trailer {
            choice: TrailerChoice::Trailer(Box::new(nested_scenario_object)),
        });

        let xml = quick_xml::se::to_string(&tow_car).unwrap();
        assert!(xml.contains("<Trailer>"));
        assert!(xml.contains("Trailer1"));

        let deserialized: Vehicle = quick_xml::de::from_str(&xml).unwrap();
        let trailer = deserialized.trailer.expect("expected trailer");
        let nested = match trailer.choice {
            TrailerChoice::Trailer(nested) => nested,
            other => panic!("expected nested scenario object, got {other:?}"),
        };
        assert_eq!(nested.get_name(), Some("Trailer1"));
        assert!(nested.vehicle().is_some());
    }
}
