//! Catalog entity types for reference resolution
//!
//! This module contains catalog-specific entity types that can be loaded from
//! catalog files and resolved into scenario entities with parameter substitution.

use crate::error::Result;
use crate::types::basic::{Double, OSString, Value};
use crate::types::controllers::Controller;
use crate::types::entities::{pedestrian, vehicle};
use crate::types::enums::{ControllerType, PedestrianCategory};
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

/// Parameter definition for catalog entities
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterDefinition {
    /// Parameter name
    pub name: String,
    /// Parameter type (String, Double, Integer, Boolean)
    pub parameter_type: String,
    /// Optional default value
    pub default_value: Option<String>,
    /// Parameter description
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
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_declarations: Option<Vec<ParameterDefinition>>,
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
    #[serde(rename = "FrontAxle")]
    pub front_axle: CatalogFrontAxle,
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
            bounding_box: self.bounding_box.resolve_parameters(&parameters)?,
            performance: vehicle::Performance {
                max_speed: crate::types::basic::Double::literal(
                    self.performance.max_speed.resolve(&parameters)?,
                ),
                max_acceleration: crate::types::basic::Double::literal(
                    self.performance.max_acceleration.resolve(&parameters)?,
                ),
                max_deceleration: crate::types::basic::Double::literal(
                    self.performance.max_deceleration.resolve(&parameters)?,
                ),
            },
            axles: crate::types::Axles {
                front_axle: Some(crate::types::Axle {
                    max_steering: crate::types::basic::Double::literal(
                        self.axles.front_axle.max_steering.resolve(&parameters)?,
                    ),
                    wheel_diameter: crate::types::basic::Double::literal(
                        self.axles.front_axle.wheel_diameter.resolve(&parameters)?,
                    ),
                    track_width: crate::types::basic::Double::literal(
                        self.axles.front_axle.track_width.resolve(&parameters)?,
                    ),
                    position_x: crate::types::basic::Double::literal(
                        self.axles.front_axle.position_x.resolve(&parameters)?,
                    ),
                    position_z: crate::types::basic::Double::literal(
                        self.axles.front_axle.position_z.resolve(&parameters)?,
                    ),
                }),
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
        if value.starts_with('$') {
            let param_name = &value[1..]; // Remove '$' prefix
            parameters
                .get(param_name)
                .map(|v| v.clone())
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
            _ => Err(crate::error::Error::catalog_error(&format!(
                "Invalid vehicle category: {}",
                category_str
            ))),
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
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_declarations: Option<Vec<ParameterDefinition>>,

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
            parameter_declarations: None, // TODO: Add parameter declaration resolution
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
        if value.starts_with('$') {
            let param_name = &value[1..]; // Remove '$' prefix
            parameters
                .get(param_name)
                .map(|v| v.clone())
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
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_declarations: Option<Vec<ParameterDefinition>>,
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
        if value.starts_with('$') {
            let param_name = &value[1..]; // Remove '$' prefix
            parameters
                .get(param_name)
                .map(|v| v.clone())
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
    #[serde(rename = "BoundingBox")]
    pub bounding_box: BoundingBox,
    #[serde(
        rename = "ParameterDeclarations",
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_declarations: Option<Vec<ParameterDefinition>>,
}

/// Environment entity definition for catalogs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogEnvironment {
    #[serde(rename = "@name")]
    pub name: String,
    // TODO: Add environment-specific fields in future phases
    #[serde(
        rename = "ParameterDeclarations",
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_declarations: Option<Vec<ParameterDefinition>>,
}

/// Maneuver entity definition for catalogs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogManeuver {
    #[serde(rename = "@name")]
    pub name: String,
    // TODO: Add maneuver-specific fields in future phases
    #[serde(
        rename = "ParameterDeclarations",
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_declarations: Option<Vec<ParameterDefinition>>,
}

/// Trajectory entity definition for catalogs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogTrajectory {
    #[serde(rename = "@name")]
    pub name: String,
    // TODO: Add trajectory-specific fields in future phases
    #[serde(
        rename = "ParameterDeclarations",
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_declarations: Option<Vec<ParameterDefinition>>,
}

/// Route entity definition for catalogs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogRoute {
    #[serde(rename = "@name")]
    pub name: String,
    // TODO: Add route-specific fields in future phases
    #[serde(
        rename = "ParameterDeclarations",
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_declarations: Option<Vec<ParameterDefinition>>,
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
        ]
    }

    fn entity_name(&self) -> &str {
        &self.name
    }
}

impl CatalogEntity for CatalogEnvironment {
    type ResolvedType = String; // Placeholder - will be Environment when implemented

    fn into_scenario_entity(
        self,
        _parameters: HashMap<String, String>,
    ) -> Result<Self::ResolvedType> {
        Ok(format!("Environment:{}", self.name)) // Placeholder implementation
    }

    fn parameter_schema() -> Vec<ParameterDefinition> {
        vec![
            ParameterDefinition {
                name: "TimeOfDay".to_string(),
                parameter_type: "String".to_string(),
                default_value: Some("12:00:00".to_string()),
                description: Some("Time of day in HH:MM:SS format".to_string()),
            },
            ParameterDefinition {
                name: "WeatherCondition".to_string(),
                parameter_type: "String".to_string(),
                default_value: Some("dry".to_string()),
                description: Some("Weather condition (dry, wet, snow, fog)".to_string()),
            },
            ParameterDefinition {
                name: "RoadCondition".to_string(),
                parameter_type: "String".to_string(),
                default_value: Some("dry".to_string()),
                description: Some("Road surface condition (dry, wet, snow, ice)".to_string()),
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

impl CatalogEntity for CatalogTrajectory {
    type ResolvedType = String; // Placeholder - will be Trajectory when implemented

    fn into_scenario_entity(
        self,
        _parameters: HashMap<String, String>,
    ) -> Result<Self::ResolvedType> {
        Ok(format!("Trajectory:{}", self.name)) // Placeholder implementation
    }

    fn parameter_schema() -> Vec<ParameterDefinition> {
        vec![
            ParameterDefinition {
                name: "StartTime".to_string(),
                parameter_type: "Double".to_string(),
                default_value: Some("0.0".to_string()),
                description: Some("Start time of the trajectory in seconds".to_string()),
            },
            ParameterDefinition {
                name: "Duration".to_string(),
                parameter_type: "Double".to_string(),
                default_value: Some("60.0".to_string()),
                description: Some("Duration of the trajectory in seconds".to_string()),
            },
            ParameterDefinition {
                name: "Closed".to_string(),
                parameter_type: "Boolean".to_string(),
                default_value: Some("false".to_string()),
                description: Some(
                    "Whether the trajectory is closed (loops back to start)".to_string(),
                ),
            },
        ]
    }

    fn entity_name(&self) -> &str {
        &self.name
    }
}

impl CatalogEntity for CatalogRoute {
    type ResolvedType = String; // Placeholder - will be Route when implemented

    fn into_scenario_entity(
        self,
        _parameters: HashMap<String, String>,
    ) -> Result<Self::ResolvedType> {
        Ok(format!("Route:{}", self.name)) // Placeholder implementation
    }

    fn parameter_schema() -> Vec<ParameterDefinition> {
        vec![
            ParameterDefinition {
                name: "StartRoadId".to_string(),
                parameter_type: "String".to_string(),
                default_value: Some("road_1".to_string()),
                description: Some("ID of the starting road".to_string()),
            },
            ParameterDefinition {
                name: "EndRoadId".to_string(),
                parameter_type: "String".to_string(),
                default_value: Some("road_2".to_string()),
                description: Some("ID of the ending road".to_string()),
            },
            ParameterDefinition {
                name: "Closed".to_string(),
                parameter_type: "Boolean".to_string(),
                default_value: Some("false".to_string()),
                description: Some("Whether the route is closed (loops back to start)".to_string()),
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
                front_axle: CatalogFrontAxle {
                    max_steering: Value::Literal(0.6),
                    wheel_diameter: Value::Literal(0.65),
                    track_width: Value::Literal(1.8),
                    position_x: Value::Literal(3.0),
                    position_z: Value::Literal(0.3),
                },
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
                front_axle: CatalogFrontAxle {
                    max_steering: Value::Literal(0.5),
                    wheel_diameter: Value::Literal(0.6),
                    track_width: Value::Literal(1.7),
                    position_x: Value::Literal(2.8),
                    position_z: Value::Literal(0.25),
                },
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
            bounding_box: BoundingBox::default(),
            parameter_declarations: None,
        };

        assert_eq!(catalog_misc_object.entity_name(), "TrafficCone");

        let resolved = catalog_misc_object
            .into_scenario_entity(HashMap::new())
            .unwrap();
        assert_eq!(resolved, "MiscObject:TrafficCone");
    }

    #[test]
    fn test_catalog_environment_placeholder() {
        let catalog_environment = CatalogEnvironment {
            name: "SunnyDay".to_string(),
            parameter_declarations: None,
        };

        assert_eq!(catalog_environment.entity_name(), "SunnyDay");

        let resolved = catalog_environment
            .into_scenario_entity(HashMap::new())
            .unwrap();
        assert_eq!(resolved, "Environment:SunnyDay");
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
}
