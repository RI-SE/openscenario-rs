//! Controller catalog types for OpenSCENARIO reusable controller definitions
//!
//! This module contains catalog-specific controller types that enable reuse of
//! controller definitions across multiple scenarios with parameter substitution.

use crate::types::basic::{Int, OSString, ParameterDeclarations, Value};
use crate::types::controllers::Controller;
use crate::types::entities::vehicle::{Properties, Property};
use crate::types::enums::ControllerType;
use serde::{Deserialize, Serialize};

/// Controller catalog containing reusable controller definitions
///
/// Represents a collection of controller definitions that can be referenced
/// from scenarios, enabling modular controller design and reuse.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "ControllerCatalog")]
pub struct ControllerCatalog {
    /// Version information for catalog compatibility
    #[serde(rename = "@revMajor")]
    pub rev_major: Int,

    #[serde(rename = "@revMinor")]
    pub rev_minor: Int,

    /// Collection of controller entries in this catalog
    #[serde(rename = "Controller")]
    pub controllers: Vec<CatalogController>,
}

impl Default for ControllerCatalog {
    fn default() -> Self {
        Self {
            rev_major: Int::literal(1),
            rev_minor: Int::literal(0),
            controllers: Vec::new(),
        }
    }
}

/// Controller definition within a catalog
///
/// Extends the base Controller type with catalog-specific functionality
/// including parameter declarations and reusable properties.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Controller")]
pub struct CatalogController {
    /// Unique name for this controller in the catalog
    #[serde(rename = "@name")]
    pub name: String,

    /// Type of controller (interactive, external, etc.)
    #[serde(rename = "@controllerType", skip_serializing_if = "Option::is_none")]
    pub controller_type: Option<ControllerType>,

    /// Parameter declarations for this controller
    #[serde(
        rename = "ParameterDeclarations",
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_declarations: Option<ParameterDeclarations>,

    /// Controller-specific properties
    #[serde(rename = "Properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<ControllerProperties>,
}

impl Default for CatalogController {
    fn default() -> Self {
        Self {
            name: "DefaultCatalogController".to_string(),
            controller_type: Some(ControllerType::Movement),
            parameter_declarations: None,
            properties: None,
        }
    }
}

/// Properties specific to catalog controllers
///
/// Container for controller parameters and configuration options that
/// can be parameterized and overridden when the controller is referenced.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Properties")]
pub struct ControllerProperties {
    /// List of controller properties
    #[serde(rename = "Property")]
    pub properties: Vec<ControllerProperty>,
}

impl Default for ControllerProperties {
    fn default() -> Self {
        Self {
            properties: Vec::new(),
        }
    }
}

/// Individual property for catalog controllers
///
/// Represents a single configuration parameter for a controller that
/// can be parameterized using OpenSCENARIO parameter syntax.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Property")]
pub struct ControllerProperty {
    /// Property name
    #[serde(rename = "@name")]
    pub name: String,

    /// Property value (can be parameterized)
    #[serde(rename = "@value")]
    pub value: OSString,
}

impl Default for ControllerProperty {
    fn default() -> Self {
        Self {
            name: "defaultProperty".to_string(),
            value: Value::Literal("defaultValue".to_string()),
        }
    }
}

// Implementation methods for catalog controllers

impl ControllerCatalog {
    /// Creates a new controller catalog with version information
    pub fn new(rev_major: i32, rev_minor: i32) -> Self {
        Self {
            rev_major: Value::Literal(rev_major),
            rev_minor: Value::Literal(rev_minor),
            controllers: Vec::new(),
        }
    }

    /// Adds a controller to this catalog
    pub fn add_controller(&mut self, controller: CatalogController) {
        self.controllers.push(controller);
    }

    /// Finds a controller by name in this catalog
    pub fn find_controller(&self, name: &str) -> Option<&CatalogController> {
        self.controllers.iter().find(|c| c.name == name)
    }

    /// Gets all controller names in this catalog
    pub fn controller_names(&self) -> Vec<&str> {
        self.controllers.iter().map(|c| c.name.as_str()).collect()
    }
}

impl CatalogController {
    /// Creates a new catalog controller with the specified name and type
    pub fn new(name: String, controller_type: ControllerType) -> Self {
        Self {
            name,
            controller_type: Some(controller_type),
            parameter_declarations: None,
            properties: None,
        }
    }

    /// Creates a catalog controller with parameter declarations
    pub fn with_parameters(
        name: String,
        controller_type: ControllerType,
        parameters: ParameterDeclarations,
    ) -> Self {
        Self {
            name,
            controller_type: Some(controller_type),
            parameter_declarations: Some(parameters),
            properties: None,
        }
    }

    /// Creates a catalog controller with properties
    pub fn with_properties(
        name: String,
        controller_type: ControllerType,
        properties: ControllerProperties,
    ) -> Self {
        Self {
            name,
            controller_type: Some(controller_type),
            parameter_declarations: None,
            properties: Some(properties),
        }
    }

    /// Converts this catalog controller to a scenario controller
    /// with parameter substitution (placeholder for future implementation)
    pub fn to_scenario_controller(&self) -> Controller {
        Controller {
            name: Value::Literal(self.name.clone()),
            controller_type: self.controller_type.clone(),
            parameter_declarations: self.parameter_declarations.clone(),
            properties: self.properties.as_ref().map(|p| p.to_scenario_properties()),
        }
    }
}

impl ControllerProperties {
    /// Creates properties from a list of name-value pairs
    pub fn from_pairs<I>(pairs: I) -> Self
    where
        I: IntoIterator<Item = (String, OSString)>,
    {
        let properties = pairs
            .into_iter()
            .map(|(name, value)| ControllerProperty { name, value })
            .collect();

        Self { properties }
    }

    /// Adds a property to this collection
    pub fn add_property(&mut self, name: String, value: OSString) {
        self.properties.push(ControllerProperty { name, value });
    }

    /// Finds a property by name
    pub fn find_property(&self, name: &str) -> Option<&ControllerProperty> {
        self.properties.iter().find(|p| p.name == name)
    }

    /// Converts to scenario Properties (using existing vehicle Properties type)
    pub fn to_scenario_properties(&self) -> Properties {
        let scenario_properties = self
            .properties
            .iter()
            .map(|p| Property {
                name: p.name.clone(),
                value: p.value.as_literal().unwrap_or(&"".to_string()).clone(),
            })
            .collect();

        Properties {
            properties: scenario_properties,
            files: vec![],
        }
    }
}

impl ControllerProperty {
    /// Creates a new controller property
    pub fn new(name: String, value: OSString) -> Self {
        Self { name, value }
    }

    /// Creates a property with a literal value
    pub fn with_literal(name: String, value: String) -> Self {
        Self {
            name,
            value: Value::Literal(value),
        }
    }

    /// Creates a property with a parameter reference
    pub fn with_parameter(name: String, parameter_ref: String) -> Self {
        Self {
            name,
            value: Value::Parameter(parameter_ref),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::basic::ParameterDeclaration;

    #[test]
    fn test_controller_catalog_creation() {
        let catalog = ControllerCatalog::new(1, 2);

        assert_eq!(catalog.rev_major.as_literal().unwrap(), &1);
        assert_eq!(catalog.rev_minor.as_literal().unwrap(), &2);
        assert!(catalog.controllers.is_empty());
    }

    #[test]
    fn test_catalog_controller_creation() {
        let controller =
            CatalogController::new("TestController".to_string(), ControllerType::Movement);

        assert_eq!(controller.name, "TestController");
        assert_eq!(controller.controller_type, Some(ControllerType::Movement));
        assert!(controller.parameter_declarations.is_none());
        assert!(controller.properties.is_none());
    }

    #[test]
    fn test_controller_catalog_operations() {
        let mut catalog = ControllerCatalog::new(1, 0);
        let controller1 =
            CatalogController::new("Controller1".to_string(), ControllerType::Movement);
        let controller2 =
            CatalogController::new("Controller2".to_string(), ControllerType::Lateral);

        catalog.add_controller(controller1);
        catalog.add_controller(controller2);

        assert_eq!(catalog.controllers.len(), 2);
        assert!(catalog.find_controller("Controller1").is_some());
        assert!(catalog.find_controller("Controller2").is_some());
        assert!(catalog.find_controller("NonExistent").is_none());

        let names = catalog.controller_names();
        assert!(names.contains(&"Controller1"));
        assert!(names.contains(&"Controller2"));
    }

    #[test]
    fn test_controller_properties() {
        let mut properties = ControllerProperties::default();
        properties.add_property(
            "maxSpeed".to_string(),
            Value::Parameter("speedLimit".to_string()),
        );
        properties.add_property("aggressive".to_string(), Value::Literal("true".to_string()));

        assert_eq!(properties.properties.len(), 2);

        let max_speed = properties.find_property("maxSpeed").unwrap();
        assert!(matches!(max_speed.value, Value::Parameter(_)));

        let aggressive = properties.find_property("aggressive").unwrap();
        assert_eq!(aggressive.value.as_literal().unwrap(), "true");
    }

    #[test]
    fn test_controller_property_creation() {
        let property1 =
            ControllerProperty::with_literal("testProp".to_string(), "testValue".to_string());
        let property2 =
            ControllerProperty::with_parameter("paramProp".to_string(), "paramRef".to_string());

        assert_eq!(property1.name, "testProp");
        assert_eq!(property1.value.as_literal().unwrap(), "testValue");

        assert_eq!(property2.name, "paramProp");
        assert!(matches!(property2.value, Value::Parameter(_)));
    }

    #[test]
    fn test_catalog_controller_with_parameters() {
        let param_decl = ParameterDeclarations {
            parameter_declarations: vec![ParameterDeclaration {
                name: OSString::literal("speed".to_string()),
                parameter_type: ParameterType::Double,
                value: OSString::literal("30.0".to_string()),
                constraint_group: None,
            }],
        };

        let controller = CatalogController::with_parameters(
            "ParameterizedController".to_string(),
            ControllerType::Movement,
            param_decl,
        );

        assert_eq!(controller.name, "ParameterizedController");
        assert!(controller.parameter_declarations.is_some());
        assert_eq!(
            controller
                .parameter_declarations
                .as_ref()
                .unwrap()
                .parameter_declarations
                .len(),
            1
        );
    }

    #[test]
    fn test_controller_serialization() {
        let catalog = ControllerCatalog::new(1, 0);

        // Test XML serialization
        let xml_result = quick_xml::se::to_string(&catalog);
        assert!(xml_result.is_ok());

        let xml = xml_result.unwrap();
        assert!(xml.contains("ControllerCatalog"));
        assert!(xml.contains("revMajor=\"1\""));
        assert!(xml.contains("revMinor=\"0\""));
    }

    #[test]
    fn test_to_scenario_controller() {
        let mut properties = ControllerProperties::default();
        properties.add_property(
            "testProp".to_string(),
            Value::Literal("testValue".to_string()),
        );

        let catalog_controller = CatalogController::with_properties(
            "TestController".to_string(),
            ControllerType::Movement,
            properties,
        );

        let scenario_controller = catalog_controller.to_scenario_controller();

        assert_eq!(
            scenario_controller.name.as_literal().unwrap(),
            "TestController"
        );
        assert_eq!(
            scenario_controller.controller_type,
            Some(ControllerType::Movement)
        );
        assert!(scenario_controller.properties.is_some());
    }

    #[test]
    fn test_defaults() {
        let catalog = ControllerCatalog::default();
        let controller = CatalogController::default();
        let properties = ControllerProperties::default();
        let property = ControllerProperty::default();

        assert_eq!(catalog.rev_major.as_literal().unwrap(), &1);
        assert_eq!(controller.name, "DefaultCatalogController");
        assert!(properties.properties.is_empty());
        assert_eq!(property.name, "defaultProperty");
    }
}

