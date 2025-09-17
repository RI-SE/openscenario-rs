//! Controller system types for OpenSCENARIO.
//!
//! This module provides comprehensive controller functionality for entity behavior management,
//! including controller definitions, activation actions, and parameter management.

use crate::types::basic::{Boolean, OSString, ParameterDeclarations, Value};
use crate::types::catalogs::references::ControllerCatalogReference;
use crate::types::distributions::ParameterValueDistribution;
use crate::types::entities::vehicle::{File, Properties, Property};
use crate::types::enums::ControllerType;
use serde::{Deserialize, Serialize};

// Placeholder types that will be implemented in future modules
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParameterAssignments {
    pub assignments: Vec<ParameterAssignment>,
}

impl Default for ParameterAssignments {
    fn default() -> Self {
        Self {
            assignments: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParameterAssignment {
    pub parameter_ref: OSString,
    pub value: OSString,
}

impl Default for ParameterAssignment {
    fn default() -> Self {
        Self {
            parameter_ref: Value::Literal("defaultParam".to_string()),
            value: Value::Literal("defaultValue".to_string()),
        }
    }
}

// CatalogReference is now imported from crate::types::catalogs::references

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Directory {
    pub path: OSString,
}

impl Default for Directory {
    fn default() -> Self {
        Self {
            path: Value::Literal("./".to_string()),
        }
    }
}

/// Main controller definition with type information and properties.
///
/// Represents a controller that can be assigned to entities to manage their behavior.
/// Controllers can have parameters, properties, and specific controller types.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Controller {
    /// Name of the controller
    #[serde(rename = "@name")]
    pub name: OSString,

    /// Type of controller (interactive, external, etc.)
    #[serde(rename = "@controllerType", skip_serializing_if = "Option::is_none")]
    pub controller_type: Option<ControllerType>,

    /// Parameter declarations for the controller
    #[serde(
        rename = "ParameterDeclarations",
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_declarations: Option<ParameterDeclarations>,

    /// Additional properties for the controller
    #[serde(rename = "Properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Properties>,
}

impl Default for Controller {
    fn default() -> Self {
        Self {
            name: Value::Literal("DefaultController".to_string()),
            controller_type: Some(ControllerType::Movement),
            parameter_declarations: None,
            properties: None,
        }
    }
}

/// Object controller wrapper that can reference a controller definition or catalog.
///
/// This is the controller structure used in ScenarioObject entities.
/// It can either contain a direct controller definition or reference a controller catalog.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ObjectController {
    /// Direct controller definition
    #[serde(rename = "Controller", skip_serializing_if = "Option::is_none")]
    pub controller: Option<Controller>,

    /// Reference to a controller in a catalog
    #[serde(rename = "CatalogReference", skip_serializing_if = "Option::is_none")]
    pub catalog_reference: Option<ControllerCatalogReference>,
}

impl Default for ObjectController {
    fn default() -> Self {
        Self {
            controller: Some(Controller::default()),
            catalog_reference: None,
        }
    }
}

/// Collection of controller-specific properties.
///
/// Provides a container for controller parameters and configuration options.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ControllerProperties {
    /// List of controller properties
    #[serde(rename = "Property")]
    pub properties: Vec<Property>,
}

impl Default for ControllerProperties {
    fn default() -> Self {
        Self {
            properties: Vec::new(),
        }
    }
}

/// Action to activate a controller for an entity.
///
/// This action enables a controller and optionally sets parameter values.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ActivateControllerAction {
    /// Reference to the controller to activate
    #[serde(rename = "@controllerRef")]
    pub controller_ref: OSString,

    /// Parameter assignments for controller activation
    #[serde(
        rename = "ParameterAssignments",
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_assignments: Option<ParameterAssignments>,
}

impl Default for ActivateControllerAction {
    fn default() -> Self {
        Self {
            controller_ref: Value::Literal("DefaultController".to_string()),
            parameter_assignments: None,
        }
    }
}

/// Action to override controller parameter values.
///
/// This action modifies controller behavior by overriding specific parameter values
/// and can activate or deactivate the override.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OverrideControllerValueAction {
    /// Reference to the controller to override
    #[serde(rename = "@controllerRef")]
    pub controller_ref: OSString,

    /// Parameter assignments for the override
    #[serde(rename = "ParameterAssignments")]
    pub parameter_assignments: ParameterAssignments,

    /// Whether the override is active
    #[serde(rename = "@active")]
    pub active: Boolean,
}

impl Default for OverrideControllerValueAction {
    fn default() -> Self {
        Self {
            controller_ref: Value::Literal("DefaultController".to_string()),
            parameter_assignments: ParameterAssignments::default(),
            active: Value::Literal(true),
        }
    }
}

/// Assignment of a controller to a specific entity.
///
/// Defines the relationship between a controller and the entity it manages.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ControllerAssignment {
    /// Reference to the controller
    #[serde(rename = "@controllerRef")]
    pub controller_ref: OSString,

    /// Target entity for the controller
    #[serde(rename = "@targetEntity")]
    pub target_entity: OSString,
}

impl Default for ControllerAssignment {
    fn default() -> Self {
        Self {
            controller_ref: Value::Literal("DefaultController".to_string()),
            target_entity: Value::Literal("Ego".to_string()),
        }
    }
}

/// Catalog location for controller definitions.
///
/// Specifies where controller catalog files can be found.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ControllerCatalogLocation {
    /// Directory containing controller catalog files
    #[serde(rename = "Directory")]
    pub directory: Directory,
}

impl Default for ControllerCatalogLocation {
    fn default() -> Self {
        Self {
            directory: Directory::default(),
        }
    }
}

/// Distribution configuration for controller parameters.
///
/// Allows for statistical or deterministic variation of controller parameters
/// across multiple scenario runs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ControllerDistribution {
    /// Type of controller this distribution applies to
    #[serde(rename = "@controllerType")]
    pub controller_type: ControllerType,

    /// Parameter distribution specification
    #[serde(rename = "ParameterValueDistribution")]
    pub distribution: ParameterValueDistribution,
}

impl Default for ControllerDistribution {
    fn default() -> Self {
        use crate::types::distributions::deterministic::*;

        // Create a simple deterministic distribution
        let single_param_dist = DeterministicSingleParameterDistribution {
            distribution_type: DeterministicSingleParameterDistributionType::DistributionSet(
                DistributionSet {
                    elements: vec![DistributionSetElement {
                        value: Value::Literal("default".to_string()),
                    }],
                },
            ),
            parameter_name: Value::Literal("controllerParam".to_string()),
        };

        let deterministic = crate::types::distributions::Deterministic {
            single_distributions: vec![single_param_dist],
            multi_distributions: vec![],
        };
        
        Self {
            controller_type: ControllerType::Movement,
            distribution: ParameterValueDistribution::new_deterministic(
                File { filepath: "default.xosc".to_string() }, 
                deterministic
            ),
        }
    }
}

// Helper implementations for common controller operations

impl Controller {
    /// Creates a new controller with the specified name and type.
    pub fn new(name: String, controller_type: ControllerType) -> Self {
        Self {
            name: Value::Literal(name),
            controller_type: Some(controller_type),
            parameter_declarations: None,
            properties: None,
        }
    }

    /// Creates a controller with parameters.
    pub fn with_parameters(
        name: String,
        controller_type: ControllerType,
        parameters: ParameterDeclarations,
    ) -> Self {
        Self {
            name: Value::Literal(name),
            controller_type: Some(controller_type),
            parameter_declarations: Some(parameters),
            properties: None,
        }
    }

    /// Creates a controller with properties.
    pub fn with_properties(
        name: String,
        controller_type: ControllerType,
        properties: Properties,
    ) -> Self {
        Self {
            name: Value::Literal(name),
            controller_type: Some(controller_type),
            parameter_declarations: None,
            properties: Some(properties),
        }
    }
}

impl ObjectController {
    /// Creates an ObjectController with a direct controller definition.
    pub fn with_controller(controller: Controller) -> Self {
        Self {
            controller: Some(controller),
            catalog_reference: None,
        }
    }

    /// Creates an ObjectController with a catalog reference.
    pub fn with_catalog_reference(catalog_reference: ControllerCatalogReference) -> Self {
        Self {
            controller: None,
            catalog_reference: Some(catalog_reference),
        }
    }
}

impl ActivateControllerAction {
    /// Creates an action to activate a controller by name.
    pub fn new(controller_ref: String) -> Self {
        Self {
            controller_ref: Value::Literal(controller_ref),
            parameter_assignments: None,
        }
    }

    /// Creates an action to activate a controller with parameter assignments.
    pub fn with_parameters(
        controller_ref: String,
        parameter_assignments: ParameterAssignments,
    ) -> Self {
        Self {
            controller_ref: Value::Literal(controller_ref),
            parameter_assignments: Some(parameter_assignments),
        }
    }
}

impl OverrideControllerValueAction {
    /// Creates an action to override controller values.
    pub fn new(
        controller_ref: String,
        parameter_assignments: ParameterAssignments,
        active: bool,
    ) -> Self {
        Self {
            controller_ref: Value::Literal(controller_ref),
            parameter_assignments,
            active: Value::Literal(active),
        }
    }
}

impl ControllerAssignment {
    /// Creates a controller assignment.
    pub fn new(controller_ref: String, target_entity: String) -> Self {
        Self {
            controller_ref: Value::Literal(controller_ref),
            target_entity: Value::Literal(target_entity),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::enums::ControllerType;

    #[test]
    fn test_controller_creation() {
        let controller = Controller::new("TestController".to_string(), ControllerType::Movement);

        assert_eq!(controller.name.as_literal().unwrap(), "TestController");
        assert_eq!(controller.controller_type, Some(ControllerType::Movement));
    }

    #[test]
    fn test_object_controller_with_direct_controller() {
        let controller = Controller::new("DirectController".to_string(), ControllerType::Lateral);
        let object_controller = ObjectController::with_controller(controller);

        assert!(object_controller.controller.is_some());
        assert!(object_controller.catalog_reference.is_none());
    }

    #[test]
    fn test_activate_controller_action() {
        let action = ActivateControllerAction::new("MainController".to_string());

        assert_eq!(
            action.controller_ref.as_literal().unwrap(),
            "MainController"
        );
        assert!(action.parameter_assignments.is_none());
    }

    #[test]
    fn test_override_controller_action() {
        let assignments = ParameterAssignments::default();
        let action =
            OverrideControllerValueAction::new("TestController".to_string(), assignments, true);

        assert_eq!(
            action.controller_ref.as_literal().unwrap(),
            "TestController"
        );
        assert_eq!(action.active.as_literal().unwrap(), &true);
    }

    #[test]
    fn test_controller_assignment() {
        let assignment =
            ControllerAssignment::new("AIController".to_string(), "Vehicle1".to_string());

        assert_eq!(
            assignment.controller_ref.as_literal().unwrap(),
            "AIController"
        );
        assert_eq!(assignment.target_entity.as_literal().unwrap(), "Vehicle1");
    }

    #[test]
    fn test_controller_serialization() {
        let controller = Controller::new("SerializationTest".to_string(), ControllerType::Movement);

        // Test XML serialization
        let xml = quick_xml::se::to_string(&controller).unwrap();
        assert!(xml.contains("SerializationTest"));
        assert!(xml.contains("movement"));

        // Test deserialization
        let deserialized: Controller = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(controller, deserialized);
    }

    #[test]
    fn test_controller_distribution() {
        let distribution = ControllerDistribution::default();

        assert_eq!(distribution.controller_type, ControllerType::Movement);
        assert!(matches!(
            distribution.distribution,
            ParameterValueDistribution { .. }
        ));
    }

    #[test]
    fn test_controller_properties() {
        let mut properties = ControllerProperties::default();
        // Create a simple property with correct String types
        let property = Property {
            name: "testProp".to_string(),
            value: "testValue".to_string(),
        };
        properties.properties.push(property);

        assert_eq!(properties.properties.len(), 1);
    }

    #[test]
    fn test_controller_defaults() {
        let controller = Controller::default();
        let object_controller = ObjectController::default();
        let properties = ControllerProperties::default();
        let activate_action = ActivateControllerAction::default();
        let override_action = OverrideControllerValueAction::default();
        let assignment = ControllerAssignment::default();

        // All defaults should be valid
        assert!(controller.name.as_literal().is_some());
        assert!(object_controller.controller.is_some());
        assert!(properties.properties.is_empty());
        assert!(activate_action.controller_ref.as_literal().is_some());
        assert!(override_action.active.as_literal().is_some());
        assert!(assignment.target_entity.as_literal().is_some());
    }
}
