//! Controller system types for OpenSCENARIO.
//!
//! This module provides comprehensive controller functionality for entity behavior management,
//! including controller definitions, activation actions, and parameter management.

use crate::types::basic::{Directory, OSString, ParameterDeclarations, Value};
use crate::types::catalogs::references::ControllerCatalogReference;
use crate::types::entities::vehicle::{Properties, Property};
use crate::types::enums::ControllerType;
use serde::{Deserialize, Serialize};

pub use crate::types::catalogs::references::{ParameterAssignment, ParameterAssignments};

// CatalogReference is now imported from crate::types::catalogs::references

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

/// Object controller wrapper that can reference a controller definition or catalog.
///
/// This is the controller structure used in ScenarioObject entities.
/// It can either contain a direct controller definition or reference a controller catalog.
/// According to XSD schema, exactly one of Controller or CatalogReference must be present.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ObjectController {
    /// Optional name attribute for the controller
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<OSString>,

    /// Direct controller definition
    #[serde(rename = "Controller", skip_serializing_if = "Option::is_none")]
    pub controller: Option<Controller>,

    /// Reference to a controller in a catalog
    #[serde(rename = "CatalogReference", skip_serializing_if = "Option::is_none")]
    pub catalog_reference: Option<ControllerCatalogReference>,
}

// Custom deserializer to handle XSD choice group validation
impl<'de> Deserialize<'de> for ObjectController {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};
        use std::fmt;

        #[derive(Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "@name")]
            Name,
            #[serde(rename = "Controller")]
            Controller,
            #[serde(rename = "CatalogReference")]
            CatalogReference,
        }

        struct ObjectControllerVisitor;

        impl<'de> Visitor<'de> for ObjectControllerVisitor {
            type Value = ObjectController;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ObjectController")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ObjectController, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut name = None;
                let mut controller = None;
                let mut catalog_reference = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Name => {
                            if name.is_some() {
                                return Err(de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        Field::Controller => {
                            if controller.is_some() {
                                return Err(de::Error::duplicate_field("Controller"));
                            }
                            controller = Some(map.next_value()?);
                        }
                        Field::CatalogReference => {
                            if catalog_reference.is_some() {
                                return Err(de::Error::duplicate_field("CatalogReference"));
                            }
                            catalog_reference = Some(map.next_value()?);
                        }
                    }
                }

                // XSD choice group validation: exactly one of Controller or CatalogReference must be present
                // However, we allow empty ObjectController elements for backward compatibility
                match (controller.is_some(), catalog_reference.is_some()) {
                    (true, false) | (false, true) | (false, false) => {
                        Ok(ObjectController {
                            name,
                            controller,
                            catalog_reference,
                        })
                    }
                    (true, true) => Err(de::Error::custom(
                        "ObjectController must contain exactly one of Controller or CatalogReference, found both"
                    )),
                }
            }
        }

        const FIELDS: &[&str] = &["@name", "Controller", "CatalogReference"];
        deserializer.deserialize_struct("ObjectController", FIELDS, ObjectControllerVisitor)
    }
}

impl Default for ObjectController {
    fn default() -> Self {
        Self {
            name: None,
            controller: None,
            catalog_reference: None,
        }
    }
}

/// Collection of controller-specific properties.
///
/// Provides a container for controller parameters and configuration options.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct ControllerProperties {
    /// List of controller properties
    #[serde(rename = "Property")]
    pub properties: Vec<Property>,
}

/// Catalog location for controller definitions.
///
/// Specifies where controller catalog files can be found.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct ControllerCatalogLocation {
    /// Directory containing controller catalog files
    #[serde(rename = "Directory")]
    pub directory: Directory,
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
            name: None,
            controller: Some(controller),
            catalog_reference: None,
        }
    }

    /// Creates an ObjectController with a catalog reference.
    pub fn with_catalog_reference(catalog_reference: ControllerCatalogReference) -> Self {
        Self {
            name: None,
            controller: None,
            catalog_reference: Some(catalog_reference),
        }
    }

    /// Creates an ObjectController with a name and direct controller definition.
    pub fn with_named_controller(name: String, controller: Controller) -> Self {
        Self {
            name: Some(Value::Literal(name)),
            controller: Some(controller),
            catalog_reference: None,
        }
    }

    /// Creates an ObjectController with a name and catalog reference.
    pub fn with_named_catalog_reference(
        name: String,
        catalog_reference: ControllerCatalogReference,
    ) -> Self {
        Self {
            name: Some(Value::Literal(name)),
            controller: None,
            catalog_reference: Some(catalog_reference),
        }
    }

    /// Validates that at most one of Controller or CatalogReference is present
    /// Empty ObjectController elements are allowed for backward compatibility
    pub fn validate(&self) -> Result<(), String> {
        match (self.controller.is_some(), self.catalog_reference.is_some()) {
            (true, false) | (false, true) | (false, false) => Ok(()),
            (true, true) => Err("ObjectController must contain at most one of Controller or CatalogReference, found both".to_string()),
        }
    }

    /// Validates strict XSD compliance (exactly one of Controller or CatalogReference must be present)
    pub fn validate_strict(&self) -> Result<(), String> {
        match (self.controller.is_some(), self.catalog_reference.is_some()) {
            (true, false) | (false, true) => Ok(()),
            (true, true) => Err("ObjectController must contain exactly one of Controller or CatalogReference, found both".to_string()),
            (false, false) => Err("ObjectController must contain exactly one of Controller or CatalogReference, found neither".to_string()),
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
        let object_controller = ObjectController::default();
        let properties = ControllerProperties::default();

        // All defaults should be valid
        assert!(object_controller.controller.is_none());
        assert!(object_controller.catalog_reference.is_none());
        assert!(properties.properties.is_empty());
    }

    #[test]
    fn test_object_controller_validation() {
        // Test valid controller with direct controller
        let valid_direct = ObjectController {
            name: None,
            controller: Some(Controller::new(
                "TestController".to_string(),
                ControllerType::Movement,
            )),
            catalog_reference: None,
        };
        assert!(valid_direct.validate().is_ok());

        // Test valid controller with catalog reference
        let valid_catalog = ObjectController {
            name: None,
            controller: None,
            catalog_reference: Some(ControllerCatalogReference::new(
                "catalog".to_string(),
                "entry".to_string(),
            )),
        };
        assert!(valid_catalog.validate().is_ok());

        // Test empty controller (allowed for backward compatibility)
        let empty_controller = ObjectController {
            name: None,
            controller: None,
            catalog_reference: None,
        };
        assert!(empty_controller.validate().is_ok());
        // But strict validation should fail
        assert!(empty_controller.validate_strict().is_err());

        // Test invalid controller with both controller and reference
        let invalid_both = ObjectController {
            name: None,
            controller: Some(Controller::new(
                "TestController".to_string(),
                ControllerType::Movement,
            )),
            catalog_reference: Some(ControllerCatalogReference::new(
                "catalog".to_string(),
                "entry".to_string(),
            )),
        };
        assert!(invalid_both.validate().is_err());

        // Test named controller
        let named_controller = ObjectController::with_named_controller(
            "TestController".to_string(),
            Controller::new("TestController".to_string(), ControllerType::Movement),
        );
        assert!(named_controller.validate().is_ok());
        assert_eq!(
            named_controller
                .name
                .as_ref()
                .unwrap()
                .as_literal()
                .unwrap(),
            "TestController"
        );
    }
}
