//! `CatalogController`: a controller definition in its catalog-file form, carrying the
//! parameter declarations that a `CatalogReference` supplies values for.

use crate::types::basic::OSString;
use crate::types::basic::ParameterDeclarations;
use crate::types::basic::Value;
use crate::types::controllers::Controller;
use crate::types::entities::vehicle::Properties;
use crate::types::enums::ControllerType;
use serde::{Deserialize, Serialize};

/// Controller definition within a catalog
///
/// Extends the base Controller type with catalog-specific functionality
/// including parameter declarations and reusable properties.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Controller")]
pub struct CatalogController {
    /// Unique name for this controller in the catalog. XSD `Controller`
    /// (`:970`, the same complex type `CatalogController` mirrors) types
    /// `@name` as the schema's `String`, a union including the parameter
    /// member.
    #[serde(rename = "@name")]
    pub name: OSString,

    /// Type of controller (interactive, external, etc.)
    #[serde(rename = "@controllerType", skip_serializing_if = "Option::is_none")]
    pub controller_type: Option<Value<ControllerType>>,

    /// Parameter declarations for this controller
    #[serde(
        rename = "ParameterDeclarations",
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_declarations: Option<ParameterDeclarations>,

    /// Controller-specific properties
    #[serde(rename = "Properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Properties>,
}

// Implementation methods for catalog controllers

impl CatalogController {
    /// Creates a new catalog controller with the specified name and type
    pub fn new(name: String, controller_type: ControllerType) -> Self {
        Self {
            name: OSString::literal(name),
            controller_type: Some(Value::Literal(controller_type)),
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
            name: OSString::literal(name),
            controller_type: Some(Value::Literal(controller_type)),
            parameter_declarations: Some(parameters),
            properties: None,
        }
    }

    /// Creates a catalog controller with properties
    pub fn with_properties(
        name: String,
        controller_type: ControllerType,
        properties: Properties,
    ) -> Self {
        Self {
            name: OSString::literal(name),
            controller_type: Some(Value::Literal(controller_type)),
            parameter_declarations: None,
            properties: Some(properties),
        }
    }

    /// Converts this catalog controller to a scenario controller
    /// with parameter substitution (placeholder for future implementation)
    pub fn to_scenario_controller(&self) -> Controller {
        Controller {
            name: self.name.clone(),
            controller_type: self.controller_type.clone(),
            parameter_declarations: self.parameter_declarations.clone(),
            properties: self.properties.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::basic::ParameterDeclaration;
    use crate::types::entities::vehicle::Property;

    use crate::types::enums::ParameterType;

    #[test]
    fn test_catalog_controller_creation() {
        let controller =
            CatalogController::new("TestController".to_string(), ControllerType::Movement);

        assert_eq!(controller.name.as_literal().unwrap(), "TestController");
        assert_eq!(
            controller.controller_type,
            Some(Value::Literal(ControllerType::Movement))
        );
        assert!(controller.parameter_declarations.is_none());
        assert!(controller.properties.is_none());
    }

    #[test]
    fn test_controller_properties() {
        let properties = Properties {
            properties: vec![
                Property {
                    name: OSString::literal("maxSpeed".to_string()),
                    value: OSString::literal("30".to_string()),
                },
                Property {
                    name: OSString::literal("aggressive".to_string()),
                    value: OSString::literal("true".to_string()),
                },
            ],
            files: vec![],
            custom_content: vec![],
        };

        assert_eq!(properties.properties.len(), 2);

        let aggressive = properties
            .properties
            .iter()
            .find(|p| p.name.as_literal().map(String::as_str) == Some("aggressive"))
            .unwrap();
        assert_eq!(
            aggressive.value.as_literal().map(String::as_str),
            Some("true")
        );
    }

    #[test]
    fn test_controller_property_creation() {
        let property = Property {
            name: OSString::literal("testProp".to_string()),
            value: OSString::literal("testValue".to_string()),
        };

        assert_eq!(
            property.name.as_literal().map(String::as_str),
            Some("testProp")
        );
        assert_eq!(
            property.value.as_literal().map(String::as_str),
            Some("testValue")
        );
    }

    #[test]
    fn test_catalog_controller_with_parameters() {
        let param_decl = ParameterDeclarations {
            parameter_declarations: vec![ParameterDeclaration {
                name: OSString::literal("speed".to_string()),
                parameter_type: Value::Literal(ParameterType::Double),
                value: OSString::literal("30.0".to_string()),
                constraint_groups: Vec::new(),
            }],
        };

        let controller = CatalogController::with_parameters(
            "ParameterizedController".to_string(),
            ControllerType::Movement,
            param_decl,
        );

        assert_eq!(
            controller.name.as_literal().unwrap(),
            "ParameterizedController"
        );
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
    fn test_to_scenario_controller() {
        let properties = Properties {
            properties: vec![Property {
                name: OSString::literal("testProp".to_string()),
                value: OSString::literal("testValue".to_string()),
            }],
            files: vec![],
            custom_content: vec![],
        };

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
            Some(Value::Literal(ControllerType::Movement))
        );
        assert!(scenario_controller.properties.is_some());
    }

    #[test]
    fn test_defaults() {
        // `Properties::default()` is honest: XSD `Properties` has all of
        // `Property`/`File`/`CustomContent` at `minOccurs="0"`, so an empty
        // collection states nothing. `CatalogController` has no `Default` —
        // `@name` is `use="required"` with no schema default, so callers must
        // supply one explicitly via `new`.
        let properties = Properties::default();
        let controller =
            CatalogController::new("ExplicitController".to_string(), ControllerType::Movement);
        let property = Property {
            name: OSString::literal("explicitProp".to_string()),
            value: OSString::literal("value".to_string()),
        };

        assert_eq!(controller.name.as_literal().unwrap(), "ExplicitController");
        assert!(properties.properties.is_empty());
        assert_eq!(
            property.name.as_literal().map(String::as_str),
            Some("explicitProp")
        );
    }
}
