//! Catalog entity types for reference resolution
//!
//! This module contains catalog-specific entity types that can be loaded from
//! catalog files and resolved into scenario entities with parameter substitution.

use crate::error::Result;
use crate::types::basic::{Double, OSString, Value};
use crate::types::controllers::Controller;
use crate::types::entities::{pedestrian, vehicle};
use crate::types::enums::{ControllerType, MiscObjectCategory, PedestrianCategory};
use crate::types::geometry::BoundingBox;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Trait for types that can be loaded from catalog files and resolved into scenario entities
pub trait CatalogEntity: Clone + Send + Sync {
    /// The type this catalog entity resolves to in scenarios
    type ResolvedType;

    /// Convert this catalog entity into a scenario entity with parameter substitution
    fn into_scenario_entity(
        self,
        parameters: HashMap<String, String>,
    ) -> Result<Self::ResolvedType>;

    /// Get the parameter schema for this catalog entity
    fn parameter_schema() -> Vec<ParameterDefinition>;

    /// Get the name of this catalog entity
    fn entity_name(&self) -> &str;
}

/// In-memory description of a parameter accepted by a catalog entity.
///
/// This is *not* an XML type: the wire representation of
/// `<ParameterDeclarations>` is [`crate::types::basic::ParameterDeclarations`].
/// `ParameterDefinition` is only used by [`CatalogEntity::parameter_schema`] and
/// the parameter substitution engine in `crate::catalog::parameters` to describe
/// and validate the parameters an entity understands.
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterDefinition {
    /// Parameter name
    pub name: String,
    /// Parameter type (e.g. "String", "Double", "Boolean")
    pub parameter_type: String,
    /// Default value, if any
    pub default_value: Option<String>,
    /// Human-readable description
    pub description: Option<String>,
}

/// Vehicle entity definition for catalogs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogVehicle {
    /// Name of the vehicle in the catalog
    #[serde(rename = "@name")]
    pub name: String,

    /// Vehicle category (can be parameterized)
    #[serde(rename = "@vehicleCategory")]
    pub vehicle_category: OSString,

    /// Bounding box (can have parameterized dimensions)
    #[serde(rename = "BoundingBox")]
    pub bounding_box: BoundingBox,

    /// Performance characteristics (can be parameterized)
    #[serde(rename = "Performance")]
    pub performance: CatalogPerformance,

    /// Axle definitions (can be parameterized)
    #[serde(rename = "Axles")]
    pub axles: CatalogAxles,

    /// Additional properties
    #[serde(rename = "Properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<vehicle::Properties>,

    /// Parameter declarations for this catalog vehicle
    #[serde(
        rename = "ParameterDeclarations",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_declarations: Option<crate::types::basic::ParameterDeclarations>,
}

/// Performance characteristics with parameter support
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogPerformance {
    #[serde(rename = "@maxSpeed")]
    pub max_speed: Double,
    #[serde(rename = "@maxAcceleration")]
    pub max_acceleration: Double,
    #[serde(rename = "@maxDeceleration")]
    pub max_deceleration: Double,
}

/// Axles with parameter support
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogAxles {
    #[serde(
        rename = "FrontAxle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub front_axle: Option<CatalogFrontAxle>,
    #[serde(rename = "RearAxle")]
    pub rear_axle: CatalogRearAxle,
}

/// Front axle with parameter support
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogFrontAxle {
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

/// Rear axle with parameter support
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogRearAxle {
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

impl CatalogEntity for CatalogVehicle {
    type ResolvedType = vehicle::Vehicle;

    fn into_scenario_entity(
        self,
        parameters: HashMap<String, String>,
    ) -> Result<Self::ResolvedType> {
        // Resolve parameters in the catalog vehicle
        let resolved_vehicle = vehicle::Vehicle {
            name: Value::literal(self.resolve_parameter(&self.name, &parameters)?),
            vehicle_category: self.resolve_vehicle_category(&self.vehicle_category, &parameters)?,
            role: None,
            mass: None,
            model3d: None,
            parameter_declarations: None,
            bounding_box: self.bounding_box.resolve_parameters(&parameters)?,
            performance: vehicle::Performance {
                max_speed: crate::types::basic::Double::literal(
                    self.performance.max_speed.resolve(&parameters)?,
                ),
                max_acceleration: crate::types::basic::Double::literal(
                    self.performance.max_acceleration.resolve(&parameters)?,
                ),
                max_acceleration_rate: None,
                max_deceleration: crate::types::basic::Double::literal(
                    self.performance.max_deceleration.resolve(&parameters)?,
                ),
                max_deceleration_rate: None,
            },
            axles: crate::types::Axles {
                front_axle: match self.axles.front_axle {
                    Some(front_axle) => Some(crate::types::Axle {
                        max_steering: crate::types::basic::Double::literal(
                            front_axle.max_steering.resolve(&parameters)?,
                        ),
                        wheel_diameter: crate::types::basic::Double::literal(
                            front_axle.wheel_diameter.resolve(&parameters)?,
                        ),
                        track_width: crate::types::basic::Double::literal(
                            front_axle.track_width.resolve(&parameters)?,
                        ),
                        position_x: crate::types::basic::Double::literal(
                            front_axle.position_x.resolve(&parameters)?,
                        ),
                        position_z: crate::types::basic::Double::literal(
                            front_axle.position_z.resolve(&parameters)?,
                        ),
                    }),
                    None => None,
                },
                rear_axle: crate::types::Axle {
                    max_steering: crate::types::basic::Double::literal(
                        self.axles.rear_axle.max_steering.resolve(&parameters)?,
                    ),
                    wheel_diameter: crate::types::basic::Double::literal(
                        self.axles.rear_axle.wheel_diameter.resolve(&parameters)?,
                    ),
                    track_width: crate::types::basic::Double::literal(
                        self.axles.rear_axle.track_width.resolve(&parameters)?,
                    ),
                    position_x: crate::types::basic::Double::literal(
                        self.axles.rear_axle.position_x.resolve(&parameters)?,
                    ),
                    position_z: crate::types::basic::Double::literal(
                        self.axles.rear_axle.position_z.resolve(&parameters)?,
                    ),
                },
                additional_axles: Vec::new(),
            },
            properties: self.properties,
            trailer_hitch: None,
            trailer_coupler: None,
            trailer: None,
        };

        Ok(resolved_vehicle)
    }

    fn parameter_schema() -> Vec<ParameterDefinition> {
        vec![
            ParameterDefinition {
                name: "MaxSpeed".to_string(),
                parameter_type: "Double".to_string(),
                default_value: Some("200.0".to_string()),
                description: Some("Maximum speed of the vehicle in m/s".to_string()),
            },
            ParameterDefinition {
                name: "MaxAcceleration".to_string(),
                parameter_type: "Double".to_string(),
                default_value: Some("200.0".to_string()),
                description: Some("Maximum acceleration of the vehicle in m/s²".to_string()),
            },
            ParameterDefinition {
                name: "MaxDeceleration".to_string(),
                parameter_type: "Double".to_string(),
                default_value: Some("10.0".to_string()),
                description: Some("Maximum deceleration of the vehicle in m/s²".to_string()),
            },
            ParameterDefinition {
                name: "VehicleCategory".to_string(),
                parameter_type: "String".to_string(),
                default_value: Some("car".to_string()),
                description: Some("Category of the vehicle (car, truck, bus, etc.)".to_string()),
            },
        ]
    }

    fn entity_name(&self) -> &str {
        &self.name
    }
}

impl CatalogVehicle {
    /// Helper method to resolve a parameter value
    fn resolve_parameter(
        &self,
        value: &str,
        parameters: &HashMap<String, String>,
    ) -> Result<String> {
        // If the value starts with '$', it's a parameter reference
        if let Some(param_name) = value.strip_prefix('$') {
            // Remove '$' prefix
            let available: Vec<String> = parameters.keys().cloned().collect();
            parameters
                .get(param_name).cloned()
                .ok_or_else(|| crate::error::Error::parameter_not_found(param_name, &available))
        } else {
            Ok(value.to_string())
        }
    }

    /// Helper method to resolve vehicle category parameter
    fn resolve_vehicle_category(
        &self,
        category: &OSString,
        parameters: &HashMap<String, String>,
    ) -> Result<crate::types::enums::VehicleCategory> {
        let category_str = category.resolve(parameters)?;
        match category_str.as_str() {
            "car" => Ok(crate::types::enums::VehicleCategory::Car),
            "truck" => Ok(crate::types::enums::VehicleCategory::Truck),
            "bus" => Ok(crate::types::enums::VehicleCategory::Bus),
            "motorbike" => Ok(crate::types::enums::VehicleCategory::Motorbike),
            "bicycle" => Ok(crate::types::enums::VehicleCategory::Bicycle),
            "train" => Ok(crate::types::enums::VehicleCategory::Train),
            "tram" => Ok(crate::types::enums::VehicleCategory::Tram),
            _ => Err(crate::error::Error::invalid_value(
                "vehicle_category",
                &category_str,
                "must be one of: car, truck, bus, motorbike, bicycle, train, tram",
            )),
        }
    }
}

/// Controller entity definition for catalogs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogController {
    /// Name of the controller in the catalog
    #[serde(rename = "@name")]
    pub name: String,

    /// Type of controller (can be parameterized)
    #[serde(rename = "@controllerType", skip_serializing_if = "Option::is_none")]
    pub controller_type: Option<OSString>,

    /// Parameter declarations for this catalog controller
    #[serde(
        rename = "ParameterDeclarations",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_declarations: Option<crate::types::basic::ParameterDeclarations>,

    /// Additional properties
    #[serde(rename = "Properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<vehicle::Properties>,
}

impl CatalogEntity for CatalogController {
    type ResolvedType = Controller;

    fn into_scenario_entity(
        self,
        parameters: HashMap<String, String>,
    ) -> Result<Self::ResolvedType> {
        let resolved_controller = Controller {
            name: Value::literal(self.resolve_parameter(&self.name, &parameters)?),
            controller_type: match &self.controller_type {
                Some(controller_type) => {
                    Some(self.resolve_controller_type(controller_type, &parameters)?)
                }
                None => Some(ControllerType::Movement), // Default to Movement when not specified
            },
            parameter_declarations: None,
            properties: self.properties,
        };

        Ok(resolved_controller)
    }

    fn parameter_schema() -> Vec<ParameterDefinition> {
        vec![ParameterDefinition {
            name: "ControllerType".to_string(),
            parameter_type: "String".to_string(),
            default_value: Some("movement".to_string()),
            description: Some(
                "Type of controller (movement, lateral, longitudinal, etc.)".to_string(),
            ),
        }]
    }

    fn entity_name(&self) -> &str {
        &self.name
    }
}

impl CatalogController {
    /// Helper method to resolve a parameter value
    fn resolve_parameter(
        &self,
        value: &str,
        parameters: &HashMap<String, String>,
    ) -> Result<String> {
        // If the value starts with '$', it's a parameter reference
        if let Some(param_name) = value.strip_prefix('$') {
            // Remove '$' prefix
            parameters
                .get(param_name).cloned()
                .ok_or_else(|| {
                    crate::error::Error::catalog_error(&format!(
                        "Parameter '{}' not found in substitution map",
                        param_name
                    ))
                })
        } else {
            Ok(value.to_string())
        }
    }

    /// Helper method to resolve controller type parameter
    fn resolve_controller_type(
        &self,
        controller_type: &OSString,
        parameters: &HashMap<String, String>,
    ) -> Result<ControllerType> {
        let type_str = controller_type.resolve(parameters)?;
        match type_str.as_str() {
            "movement" => Ok(ControllerType::Movement),
            "lateral" => Ok(ControllerType::Lateral),
            "longitudinal" => Ok(ControllerType::Longitudinal),
            "lighting" => Ok(ControllerType::Lighting),
            "animation" => Ok(ControllerType::Animation),
            "appearance" => Ok(ControllerType::Appearance),
            _ => Err(crate::error::Error::catalog_error(&format!(
                "Invalid controller type: {}",
                type_str
            ))),
        }
    }
}

/// Pedestrian entity definition for catalogs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogPedestrian {
    /// Name of pedestrian in the catalog
    #[serde(rename = "@name")]
    pub name: String,

    /// Category of pedestrian (can be parameterized)
    #[serde(rename = "@pedestrianCategory")]
    pub pedestrian_category: OSString,

    /// Mass in kg (can be parameterized) - REQUIRED by XSD
    #[serde(rename = "@mass")]
    pub mass: OSString,

    /// Role (can be parameterized)
    #[serde(rename = "@role", skip_serializing_if = "Option::is_none")]
    pub role: Option<OSString>,

    /// 3D model path (can be parameterized)
    #[serde(rename = "@model3d", skip_serializing_if = "Option::is_none")]
    pub model3d: Option<String>,

    /// Bounding box (can have parameterized dimensions)
    #[serde(rename = "BoundingBox")]
    pub bounding_box: BoundingBox,

    /// Additional properties (can be parameterized)
    #[serde(rename = "Properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<vehicle::Properties>,

    /// Parameter declarations for this catalog pedestrian
    #[serde(
        rename = "ParameterDeclarations",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_declarations: Option<crate::types::basic::ParameterDeclarations>,
}

impl CatalogEntity for CatalogPedestrian {
    type ResolvedType = pedestrian::Pedestrian;

    fn into_scenario_entity(
        self,
        parameters: HashMap<String, String>,
    ) -> Result<Self::ResolvedType> {
        let resolved_mass = self.mass.resolve(&parameters)?;
        let mass_value = resolved_mass.parse::<f64>().map_err(|e| {
            crate::error::Error::catalog_error(&format!("Failed to parse mass value: {}", e))
        })?;

        let resolved_pedestrian = pedestrian::Pedestrian {
            name: Value::literal(self.resolve_parameter(&self.name, &parameters)?),
            pedestrian_category: self
                .resolve_pedestrian_category(&self.pedestrian_category, &parameters)?,
            mass: crate::types::basic::Double::literal(mass_value),
            role: self
                .role
                .as_ref()
                .map(|r| self.resolve_role(r, &parameters))
                .transpose()?,
            model3d: self.model3d,
            bounding_box: self.bounding_box.resolve_parameters(&parameters)?,
            properties: self.properties,
            parameter_declarations: None,
        };

        Ok(resolved_pedestrian)
    }

    fn parameter_schema() -> Vec<ParameterDefinition> {
        vec![
            ParameterDefinition {
                name: "PedestrianCategory".to_string(),
                parameter_type: "String".to_string(),
                default_value: Some("pedestrian".to_string()),
                description: Some(
                    "Category of pedestrian (pedestrian, wheelchair, animal)".to_string(),
                ),
            },
            ParameterDefinition {
                name: "Mass".to_string(),
                parameter_type: "Double".to_string(),
                default_value: Some("75.0".to_string()),
                description: Some("Mass of pedestrian in kg".to_string()),
            },
            ParameterDefinition {
                name: "Role".to_string(),
                parameter_type: "String".to_string(),
                default_value: Some("none".to_string()),
                description: Some("Role of pedestrian (civil, police, etc.)".to_string()),
            },
        ]
    }

    fn entity_name(&self) -> &str {
        &self.name
    }
}

impl CatalogPedestrian {
    /// Helper method to resolve a parameter value
    fn resolve_parameter(
        &self,
        value: &str,
        parameters: &HashMap<String, String>,
    ) -> Result<String> {
        // If the value starts with '$', it's a parameter reference
        if let Some(param_name) = value.strip_prefix('$') {
            // Remove '$' prefix
            parameters
                .get(param_name).cloned()
                .ok_or_else(|| {
                    crate::error::Error::catalog_error(&format!(
                        "Parameter '{}' not found in substitution map",
                        param_name
                    ))
                })
        } else {
            Ok(value.to_string())
        }
    }

    /// Helper method to resolve pedestrian category parameter
    fn resolve_pedestrian_category(
        &self,
        category: &OSString,
        parameters: &HashMap<String, String>,
    ) -> Result<PedestrianCategory> {
        let category_str = category.resolve(parameters)?;
        match category_str.as_str() {
            "pedestrian" => Ok(PedestrianCategory::Pedestrian),
            "wheelchair" => Ok(PedestrianCategory::Wheelchair),
            "animal" => Ok(PedestrianCategory::Animal),
            _ => Err(crate::error::Error::catalog_error(&format!(
                "Invalid pedestrian category: {}",
                category_str
            ))),
        }
    }

    /// Helper method to resolve role parameter
    fn resolve_role(
        &self,
        role: &OSString,
        parameters: &HashMap<String, String>,
    ) -> Result<crate::types::enums::Role> {
        let role_str = role.resolve(parameters)?;
        match role_str.as_str() {
            "none" => Ok(crate::types::enums::Role::None),
            "ambulance" => Ok(crate::types::enums::Role::Ambulance),
            "civil" => Ok(crate::types::enums::Role::Civil),
            "fire" => Ok(crate::types::enums::Role::Fire),
            "military" => Ok(crate::types::enums::Role::Military),
            "police" => Ok(crate::types::enums::Role::Police),
            "publicTransport" => Ok(crate::types::enums::Role::PublicTransport),
            "roadAssistance" => Ok(crate::types::enums::Role::RoadAssistance),
            _ => Err(crate::error::Error::catalog_error(&format!(
                "Invalid role: {}",
                role_str
            ))),
        }
    }
}

/// Placeholder catalog entities for remaining types
/// These provide basic structure for future implementation

/// Miscellaneous object entity definition for catalogs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogMiscObject {
    #[serde(rename = "@name")]
    pub name: String,

    /// Mass of the object in kg — XSD attribute `mass`, `use="required"`
    #[serde(rename = "@mass")]
    pub mass: Double,

    /// Category of the object — XSD attribute `miscObjectCategory`, `use="required"`
    #[serde(rename = "@miscObjectCategory")]
    pub misc_object_category: MiscObjectCategory,

    /// Optional reference to a 3D model — XSD attribute `model3d`
    #[serde(rename = "@model3d", default, skip_serializing_if = "Option::is_none")]
    pub model3d: Option<OSString>,

    #[serde(rename = "BoundingBox")]
    pub bounding_box: BoundingBox,

    /// Optional additional properties — XSD child element `<Properties>`
    #[serde(rename = "Properties", default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<vehicle::Properties>,

    #[serde(
        rename = "ParameterDeclarations",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_declarations: Option<crate::types::basic::ParameterDeclarations>,
}

/// Maneuver entity definition for catalogs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogManeuver {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(
        rename = "ParameterDeclarations",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_declarations: Option<crate::types::basic::ParameterDeclarations>,

    /// Events making up this maneuver — XSD `<Event>`, `maxOccurs="unbounded"`
    #[serde(rename = "Event", default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<crate::types::scenario::story::Event>,
}

// Placeholder implementations for remaining catalog entities
// These will be expanded when the corresponding entity types are fully implemented

impl CatalogEntity for CatalogMiscObject {
    type ResolvedType = String; // Placeholder - will be MiscObject when implemented

    fn into_scenario_entity(
        self,
        _parameters: HashMap<String, String>,
    ) -> Result<Self::ResolvedType> {
        Ok(format!("MiscObject:{}", self.name)) // Placeholder implementation
    }

    fn parameter_schema() -> Vec<ParameterDefinition> {
        vec![
            ParameterDefinition {
                name: "Width".to_string(),
                parameter_type: "Double".to_string(),
                default_value: Some("1.0".to_string()),
                description: Some("Width of the miscellaneous object in meters".to_string()),
            },
            ParameterDefinition {
                name: "Length".to_string(),
                parameter_type: "Double".to_string(),
                default_value: Some("1.0".to_string()),
                description: Some("Length of the miscellaneous object in meters".to_string()),
            },
            ParameterDefinition {
                name: "Height".to_string(),
                parameter_type: "Double".to_string(),
                default_value: Some("1.0".to_string()),
                description: Some("Height of the miscellaneous object in meters".to_string()),
            },
            ParameterDefinition {
                name: "Mass".to_string(),
                parameter_type: "Double".to_string(),
                default_value: Some("1.0".to_string()),
                description: Some("Mass of the miscellaneous object in kg".to_string()),
            },
            ParameterDefinition {
                name: "MiscObjectCategory".to_string(),
                parameter_type: "String".to_string(),
                default_value: Some("obstacle".to_string()),
                description: Some("Category of the miscellaneous object".to_string()),
            },
        ]
    }

    fn entity_name(&self) -> &str {
        &self.name
    }
}

impl CatalogEntity for CatalogManeuver {
    type ResolvedType = String; // Placeholder - will be Maneuver when implemented

    fn into_scenario_entity(
        self,
        _parameters: HashMap<String, String>,
    ) -> Result<Self::ResolvedType> {
        Ok(format!("Maneuver:{}", self.name)) // Placeholder implementation
    }

    fn parameter_schema() -> Vec<ParameterDefinition> {
        vec![
            ParameterDefinition {
                name: "Duration".to_string(),
                parameter_type: "Double".to_string(),
                default_value: Some("10.0".to_string()),
                description: Some("Duration of the maneuver in seconds".to_string()),
            },
            ParameterDefinition {
                name: "TargetSpeed".to_string(),
                parameter_type: "Double".to_string(),
                default_value: Some("30.0".to_string()),
                description: Some("Target speed for the maneuver in m/s".to_string()),
            },
            ParameterDefinition {
                name: "ManeuverType".to_string(),
                parameter_type: "String".to_string(),
                default_value: Some("lane_change".to_string()),
                description: Some(
                    "Type of maneuver (lane_change, overtake, merge, etc.)".to_string(),
                ),
            },
        ]
    }

    fn entity_name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalog_vehicle_parameter_schema() {
        let schema = CatalogVehicle::parameter_schema();
        assert_eq!(schema.len(), 4);

        let max_speed_param = schema.iter().find(|p| p.name == "MaxSpeed").unwrap();
        assert_eq!(max_speed_param.parameter_type, "Double");
        assert_eq!(max_speed_param.default_value.as_ref().unwrap(), "200.0");
    }

    #[test]
    fn test_parameter_resolution() {
        let mut parameters = HashMap::new();
        parameters.insert("TestParam".to_string(), "42.0".to_string());

        let value: Double = Value::Parameter("TestParam".to_string());
        let resolved = value.resolve(&parameters).unwrap();
        assert_eq!(resolved, 42.0);
    }

    #[test]
    fn test_catalog_vehicle_entity_name() {
        let catalog_vehicle = CatalogVehicle {
            name: "SportsCar".to_string(),
            vehicle_category: Value::Literal("car".to_string()),
            bounding_box: BoundingBox::default(),
            performance: CatalogPerformance {
                max_speed: Value::Literal(250.0),
                max_acceleration: Value::Literal(15.0),
                max_deceleration: Value::Literal(12.0),
            },
            axles: CatalogAxles {
                front_axle: Some(CatalogFrontAxle {
                    max_steering: Value::Literal(0.6),
                    wheel_diameter: Value::Literal(0.65),
                    track_width: Value::Literal(1.8),
                    position_x: Value::Literal(3.0),
                    position_z: Value::Literal(0.3),
                }),
                rear_axle: CatalogRearAxle {
                    max_steering: Value::Literal(0.0),
                    wheel_diameter: Value::Literal(0.65),
                    track_width: Value::Literal(1.8),
                    position_x: Value::Literal(0.0),
                    position_z: Value::Literal(0.3),
                },
            },
            properties: None,
            parameter_declarations: None,
        };

        assert_eq!(catalog_vehicle.entity_name(), "SportsCar");
    }

    #[test]
    fn test_catalog_vehicle_resolution() {
        let catalog_vehicle = CatalogVehicle {
            name: "TestVehicle".to_string(),
            vehicle_category: Value::Literal("car".to_string()),
            bounding_box: BoundingBox::default(),
            performance: CatalogPerformance {
                max_speed: Value::Parameter("MaxSpeedParam".to_string()),
                max_acceleration: Value::Literal(10.0),
                max_deceleration: Value::Literal(8.0),
            },
            axles: CatalogAxles {
                front_axle: Some(CatalogFrontAxle {
                    max_steering: Value::Literal(0.5),
                    wheel_diameter: Value::Literal(0.6),
                    track_width: Value::Literal(1.7),
                    position_x: Value::Literal(2.8),
                    position_z: Value::Literal(0.25),
                }),
                rear_axle: CatalogRearAxle {
                    max_steering: Value::Literal(0.0),
                    wheel_diameter: Value::Literal(0.6),
                    track_width: Value::Literal(1.7),
                    position_x: Value::Literal(0.0),
                    position_z: Value::Literal(0.25),
                },
            },
            properties: None,
            parameter_declarations: None,
        };

        let mut parameters = HashMap::new();
        parameters.insert("MaxSpeedParam".to_string(), "180.0".to_string());

        let resolved = catalog_vehicle.into_scenario_entity(parameters).unwrap();
        assert_eq!(resolved.performance.max_speed.as_literal().unwrap(), &180.0);
        assert_eq!(resolved.name.as_literal().unwrap(), "TestVehicle");
    }

    #[test]
    fn test_catalog_controller_parameter_schema() {
        let schema = CatalogController::parameter_schema();
        assert_eq!(schema.len(), 1);

        let controller_type_param = schema.iter().find(|p| p.name == "ControllerType").unwrap();
        assert_eq!(controller_type_param.parameter_type, "String");
        assert_eq!(
            controller_type_param.default_value.as_ref().unwrap(),
            "movement"
        );
    }

    #[test]
    fn test_catalog_controller_entity_name() {
        let catalog_controller = CatalogController {
            name: "AIDriver".to_string(),
            controller_type: Some(Value::Literal("movement".to_string())),
            parameter_declarations: None,
            properties: None,
        };

        assert_eq!(catalog_controller.entity_name(), "AIDriver");
    }

    #[test]
    fn test_catalog_controller_resolution() {
        let catalog_controller = CatalogController {
            name: "TestController".to_string(),
            controller_type: Some(Value::Parameter("ControllerTypeParam".to_string())),
            parameter_declarations: None,
            properties: None,
        };

        let mut parameters = HashMap::new();
        parameters.insert("ControllerTypeParam".to_string(), "lateral".to_string());

        let resolved = catalog_controller.into_scenario_entity(parameters).unwrap();
        assert_eq!(resolved.controller_type.unwrap(), ControllerType::Lateral);
        assert_eq!(resolved.name.as_literal().unwrap(), "TestController");
    }

    #[test]
    fn test_catalog_controller_type_resolution() {
        let catalog_controller = CatalogController {
            name: "FlexController".to_string(),
            controller_type: Some(Value::Literal("longitudinal".to_string())),
            parameter_declarations: None,
            properties: None,
        };

        let parameters = HashMap::new();
        let resolved = catalog_controller.into_scenario_entity(parameters).unwrap();
        assert_eq!(
            resolved.controller_type.unwrap(),
            ControllerType::Longitudinal
        );
    }

    #[test]
    fn test_catalog_pedestrian_parameter_schema() {
        let schema = CatalogPedestrian::parameter_schema();
        assert_eq!(schema.len(), 3);

        let pedestrian_category_param = schema
            .iter()
            .find(|p| p.name == "PedestrianCategory")
            .unwrap();
        assert_eq!(pedestrian_category_param.parameter_type, "String");
        assert_eq!(
            pedestrian_category_param.default_value.as_ref().unwrap(),
            "pedestrian"
        );

        let mass_param = schema.iter().find(|p| p.name == "Mass").unwrap();
        assert_eq!(mass_param.parameter_type, "Double");
        assert_eq!(mass_param.default_value.as_ref().unwrap(), "75.0");

        let role_param = schema.iter().find(|p| p.name == "Role").unwrap();
        assert_eq!(role_param.parameter_type, "String");
        assert_eq!(role_param.default_value.as_ref().unwrap(), "none");
    }

    #[test]
    fn test_catalog_pedestrian_entity_name() {
        let catalog_pedestrian = CatalogPedestrian {
            name: "WalkingPerson".to_string(),
            pedestrian_category: Value::Literal("pedestrian".to_string()),
            mass: Value::Literal("75.0".to_string()),
            role: Some(Value::Literal("none".to_string())),
            model3d: None,
            bounding_box: BoundingBox::default(),
            properties: None,
            parameter_declarations: None,
        };

        assert_eq!(catalog_pedestrian.entity_name(), "WalkingPerson");
    }

    #[test]
    fn test_catalog_pedestrian_resolution() {
        let catalog_pedestrian = CatalogPedestrian {
            name: "TestPedestrian".to_string(),
            pedestrian_category: Value::Parameter("PedestrianTypeParam".to_string()),
            mass: Value::Literal("75.0".to_string()),
            role: Some(Value::Literal("civil".to_string())),
            model3d: None,
            bounding_box: BoundingBox::default(),
            properties: None,
            parameter_declarations: None,
        };

        let mut parameters = HashMap::new();
        parameters.insert("PedestrianTypeParam".to_string(), "wheelchair".to_string());

        let resolved = catalog_pedestrian.into_scenario_entity(parameters).unwrap();
        assert_eq!(resolved.pedestrian_category, PedestrianCategory::Wheelchair);
        assert_eq!(resolved.name.as_literal().unwrap(), "TestPedestrian");
        assert_eq!(resolved.role.unwrap(), crate::types::enums::Role::Civil);
    }

    #[test]
    fn test_catalog_misc_object_placeholder() {
        let catalog_misc_object = CatalogMiscObject {
            name: "TrafficCone".to_string(),
            mass: Value::Literal(5.0),
            misc_object_category: MiscObjectCategory::Obstacle,
            model3d: None,
            bounding_box: BoundingBox::default(),
            properties: None,
            parameter_declarations: None,
        };

        assert_eq!(catalog_misc_object.entity_name(), "TrafficCone");

        let resolved = catalog_misc_object
            .into_scenario_entity(HashMap::new())
            .unwrap();
        assert_eq!(resolved, "MiscObject:TrafficCone");
    }

    /// Regression: `mass`, `miscObjectCategory` and `<Properties>` are part of
    /// the XSD MiscObject type but were absent from the catalog entry struct,
    /// so a real misc object catalog failed to parse at all.
    #[test]
    fn test_catalog_misc_object_round_trip() {
        let xml = r#"<MiscObject miscObjectCategory="obstacle" mass="70" name="obstacle">
    <BoundingBox>
        <Center x="0.5" y="0.0" z="0.5"/>
        <Dimensions width="1.0" length="1.0" height="1.0"/>
    </BoundingBox>
    <Properties/>
</MiscObject>"#;

        let misc_object: CatalogMiscObject = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(misc_object.name, "obstacle");
        assert_eq!(misc_object.mass.as_literal().unwrap(), &70.0);
        assert_eq!(
            misc_object.misc_object_category,
            MiscObjectCategory::Obstacle
        );
        assert!(misc_object.properties.is_some());

        let serialized = quick_xml::se::to_string(&misc_object).unwrap();
        assert!(
            serialized.contains("mass=\"70\""),
            "mass lost on serialize: {serialized}"
        );
        assert!(
            serialized.contains("miscObjectCategory=\"obstacle\""),
            "category lost on serialize: {serialized}"
        );
    }

    /// Regression: the XSD Maneuver type is ParameterDeclarations? followed by
    /// one or more <Event>. The catalog entry modelled only @name, so every
    /// event in an inline catalog <Maneuver> was silently discarded.
    #[test]
    fn test_catalog_maneuver_parses_events() {
        let xml = r#"<Maneuver name="LogAndSetVariables">
    <ParameterDeclarations>
        <ParameterDeclaration name="collidingEntity" parameterType="string" value="VRU"/>
    </ParameterDeclarations>
    <Event name="AtCollision" priority="parallel" maximumExecutionCount="1">
        <Action name="SetCollisionVariable">
            <GlobalAction>
                <VariableAction variableRef="collisionDetected">
                    <SetAction value="true"/>
                </VariableAction>
            </GlobalAction>
        </Action>
    </Event>
</Maneuver>"#;

        let maneuver: CatalogManeuver = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(maneuver.name, "LogAndSetVariables");
        assert_eq!(maneuver.events.len(), 1);

        let event = &maneuver.events[0];
        assert_eq!(event.name.as_literal().unwrap(), "AtCollision");
        assert_eq!(event.actions.len(), 1);

        let serialized = quick_xml::se::to_string(&maneuver).unwrap();
        assert!(
            serialized.contains("<Event"),
            "Event lost on serialize: {serialized}"
        );
        assert!(
            serialized.contains("SetCollisionVariable"),
            "Action lost on serialize: {serialized}"
        );
    }

    #[test]
    fn test_all_catalog_entity_schemas() {
        // Test that all entity types have valid parameter schemas
        let vehicle_schema = CatalogVehicle::parameter_schema();
        let controller_schema = CatalogController::parameter_schema();
        let pedestrian_schema = CatalogPedestrian::parameter_schema();
        let misc_object_schema = CatalogMiscObject::parameter_schema();

        // Should not panic and should return valid schemas
        assert!(vehicle_schema.len() >= 1);
        assert!(controller_schema.len() >= 1);
        assert!(pedestrian_schema.len() >= 3); // Has PedestrianCategory, Mass, Role
        assert!(misc_object_schema.len() >= 3);
    }

    // ---------------------------------------------------------------------------
    // Regression tests: ParameterDeclarations XML round-trip
    // ---------------------------------------------------------------------------

    /// Verify that a Vehicle with a <ParameterDeclarations> block deserializes
    /// correctly.  This was previously broken because ParameterDefinition lacked
    /// #[serde(rename = "@...")] on its fields, causing quick-xml to look for
    /// child elements instead of XML attributes.
    #[test]
    fn test_catalog_vehicle_with_parameter_declarations_parses() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<OpenSCENARIO>
  <FileHeader revMajor="1" revMinor="3" date="2024-01-01T00:00:00"
              description="regression test" author="test"/>
  <Catalog name="TestCatalog">
    <Vehicle name="TestVehicle" vehicleCategory="car">
      <ParameterDeclarations>
        <ParameterDeclaration name="MaxDeceleration" parameterType="double" value="10.0"/>
        <ParameterDeclaration name="MaxSpeed" parameterType="double" value="50.0"/>
      </ParameterDeclarations>
      <BoundingBox>
        <Center x="0.0" y="0.0" z="0.75"/>
        <Dimensions width="2.0" length="4.5" height="1.5"/>
      </BoundingBox>
      <Performance maxSpeed="$MaxSpeed" maxAcceleration="10.0" maxDeceleration="$MaxDeceleration"/>
      <Axles>
        <FrontAxle maxSteering="0.5" wheelDiameter="0.6" trackWidth="1.7" positionX="2.8" positionZ="0.3"/>
        <RearAxle  maxSteering="0.0" wheelDiameter="0.6" trackWidth="1.7" positionX="0.0" positionZ="0.3"/>
      </Axles>
    </Vehicle>
  </Catalog>
</OpenSCENARIO>"#;

        let catalog = crate::parser::xml::parse_catalog_from_str(xml)
            .expect("catalog with ParameterDeclarations should parse without error");

        assert_eq!(catalog.catalog.vehicles.len(), 1, "expected one vehicle");

        let vehicle = &catalog.catalog.vehicles[0];
        assert_eq!(vehicle.name, "TestVehicle");

        let decls = vehicle
            .parameter_declarations
            .as_ref()
            .expect("vehicle should have a ParameterDeclarations block");

        assert_eq!(
            decls.parameter_declarations.len(),
            2,
            "expected two parameter declarations"
        );

        let max_decel = decls
            .parameter_declarations
            .iter()
            .find(|p| p.name.as_literal().map(|n| n == "MaxDeceleration") == Some(true))
            .expect("MaxDeceleration declaration must be present");
        assert_eq!(
            max_decel.parameter_type,
            crate::types::enums::ParameterType::Double
        );
        assert_eq!(
            max_decel.value.as_literal().map(String::as_str),
            Some("10.0"),
            "value should map to the 'value' XML attribute"
        );

        let max_speed = decls
            .parameter_declarations
            .iter()
            .find(|p| p.name.as_literal().map(|n| n == "MaxSpeed") == Some(true))
            .expect("MaxSpeed declaration must be present");
        assert_eq!(
            max_speed.parameter_type,
            crate::types::enums::ParameterType::Double
        );
        assert_eq!(max_speed.value.as_literal().map(String::as_str), Some("50.0"));
    }

    /// Regression: catalog entities previously used a bespoke
    /// `ParameterDeclarationsBlock` whose declarations had no `<ConstraintGroup>`
    /// field, so constraints were silently dropped on every catalog round-trip.
    #[test]
    fn test_catalog_vehicle_parameter_declarations_constraint_group_round_trip() {
        let xml = r#"<Vehicle name="ConstrainedCar" vehicleCategory="car">
  <ParameterDeclarations>
    <ParameterDeclaration name="x" parameterType="double" value="1.0">
      <ConstraintGroup>
        <ValueConstraint rule="greaterThan" value="0"/>
      </ConstraintGroup>
    </ParameterDeclaration>
  </ParameterDeclarations>
  <BoundingBox>
    <Center x="0.0" y="0.0" z="0.75"/>
    <Dimensions width="2.0" length="4.5" height="1.5"/>
  </BoundingBox>
  <Performance maxSpeed="50.0" maxAcceleration="10.0" maxDeceleration="8.0"/>
  <Axles>
    <FrontAxle maxSteering="0.5" wheelDiameter="0.6" trackWidth="1.7" positionX="2.8" positionZ="0.3"/>
    <RearAxle  maxSteering="0.0" wheelDiameter="0.6" trackWidth="1.7" positionX="0.0" positionZ="0.3"/>
  </Axles>
</Vehicle>"#;

        let check = |vehicle: &CatalogVehicle| {
            let decls = vehicle
                .parameter_declarations
                .as_ref()
                .expect("ParameterDeclarations must be present");
            assert_eq!(decls.parameter_declarations.len(), 1);
            let decl = &decls.parameter_declarations[0];
            assert_eq!(decl.name.as_literal().map(String::as_str), Some("x"));
            assert_eq!(
                decl.parameter_type,
                crate::types::enums::ParameterType::Double
            );
            assert_eq!(decl.value.as_literal().map(String::as_str), Some("1.0"));
            assert_eq!(
                decl.constraint_groups.len(),
                1,
                "ConstraintGroup must survive"
            );
            let constraints = &decl.constraint_groups[0].value_constraints;
            assert_eq!(constraints.len(), 1);
            assert_eq!(constraints[0].rule, crate::types::enums::Rule::GreaterThan);
            assert_eq!(
                constraints[0].value.as_literal().map(String::as_str),
                Some("0")
            );
        };

        let vehicle: CatalogVehicle = quick_xml::de::from_str(xml).unwrap();
        check(&vehicle);

        let serialized = quick_xml::se::to_string(&vehicle).unwrap();
        assert!(
            serialized.contains("<ConstraintGroup>"),
            "ConstraintGroup lost on serialize: {serialized}"
        );

        let reparsed: CatalogVehicle = quick_xml::de::from_str(&serialized).unwrap();
        check(&reparsed);
        assert_eq!(vehicle, reparsed);
    }

    /// Verify that a Vehicle WITHOUT a <ParameterDeclarations> block still
    /// parses correctly (regression guard: the Option must remain None).
    #[test]
    fn test_catalog_vehicle_without_parameter_declarations_parses() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<OpenSCENARIO>
  <FileHeader revMajor="1" revMinor="3" date="2024-01-01T00:00:00"
              description="regression test" author="test"/>
  <Catalog name="TestCatalog">
    <Vehicle name="SimpleCar" vehicleCategory="car">
      <BoundingBox>
        <Center x="0.0" y="0.0" z="0.75"/>
        <Dimensions width="2.0" length="4.5" height="1.5"/>
      </BoundingBox>
      <Performance maxSpeed="50.0" maxAcceleration="10.0" maxDeceleration="8.0"/>
      <Axles>
        <FrontAxle maxSteering="0.5" wheelDiameter="0.6" trackWidth="1.7" positionX="2.8" positionZ="0.3"/>
        <RearAxle  maxSteering="0.0" wheelDiameter="0.6" trackWidth="1.7" positionX="0.0" positionZ="0.3"/>
      </Axles>
    </Vehicle>
  </Catalog>
</OpenSCENARIO>"#;

        let catalog = crate::parser::xml::parse_catalog_from_str(xml)
            .expect("catalog without ParameterDeclarations should parse without error");

        let vehicle = &catalog.catalog.vehicles[0];
        assert_eq!(vehicle.name, "SimpleCar");
        assert!(
            vehicle.parameter_declarations.is_none(),
            "parameter_declarations should be None when element is absent"
        );
    }
}
