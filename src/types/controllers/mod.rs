//! Controller definitions and the properties that configure them. The actions that
//! assign and activate a controller live in [`crate::types::actions::control`].

use crate::types::basic::{Directory, OSString, ParameterDeclarations, Value};
use crate::types::catalogs::references::ControllerCatalogReference;
use crate::types::entities::vehicle::Properties;
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
    pub controller_type: Option<Value<ControllerType>>,

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
/// XSD `ObjectController` (`Schema/OpenSCENARIO.xsd:1522-1528`) is a bare `xsd:choice` of
/// `CatalogReference | Controller`; neither branch carries `minOccurs="0"`, so exactly one
/// must be present. The choice lives in the single `choice` field below rather than as two
/// parallel `Option`s, so the type cannot hold zero or both at once.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ObjectController {
    /// Optional name attribute for the controller
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<OSString>,

    /// The controller definition or catalog reference, exactly one of the two.
    #[serde(rename = "$value")]
    pub choice: ObjectControllerChoice,
}

/// The two branches of the `ObjectController` choice (XSD:1522-1528).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ObjectControllerChoice {
    /// Reference to a controller in a catalog
    CatalogReference(ControllerCatalogReference),
    /// Direct controller definition
    Controller(Controller),
}

/// Catalog location for controller definitions.
///
/// Specifies where controller catalog files can be found.
///
/// `#[derive(Default)]` removed — `Directory`'s `Default` fabricated an
/// empty `@path` (`Schema/OpenSCENARIO.xsd:1067-1069` declares `@path` `use="required"`
/// with no schema default). This type has no constructor or call site of its own; it
/// appears to be an unused duplicate of `catalogs::locations::ControllerCatalogLocation`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
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
            controller_type: Some(Value::Literal(controller_type)),
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
            controller_type: Some(Value::Literal(controller_type)),
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
            controller_type: Some(Value::Literal(controller_type)),
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
            choice: ObjectControllerChoice::Controller(controller),
        }
    }

    /// Creates an ObjectController with a catalog reference.
    pub fn with_catalog_reference(catalog_reference: ControllerCatalogReference) -> Self {
        Self {
            name: None,
            choice: ObjectControllerChoice::CatalogReference(catalog_reference),
        }
    }

    /// Creates an ObjectController with a name and direct controller definition.
    pub fn with_named_controller(name: String, controller: Controller) -> Self {
        Self {
            name: Some(Value::Literal(name)),
            choice: ObjectControllerChoice::Controller(controller),
        }
    }

    /// Creates an ObjectController with a name and catalog reference.
    pub fn with_named_catalog_reference(
        name: String,
        catalog_reference: ControllerCatalogReference,
    ) -> Self {
        Self {
            name: Some(Value::Literal(name)),
            choice: ObjectControllerChoice::CatalogReference(catalog_reference),
        }
    }

    /// The direct controller definition, if this is the `Controller` branch.
    pub fn controller(&self) -> Option<&Controller> {
        match &self.choice {
            ObjectControllerChoice::Controller(c) => Some(c),
            ObjectControllerChoice::CatalogReference(_) => None,
        }
    }

    /// The catalog reference, if this is the `CatalogReference` branch.
    pub fn catalog_reference(&self) -> Option<&ControllerCatalogReference> {
        match &self.choice {
            ObjectControllerChoice::CatalogReference(c) => Some(c),
            ObjectControllerChoice::Controller(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::basic::OSString;
    use crate::types::entities::vehicle::Property;
    use crate::types::enums::ControllerType;

    #[test]
    fn test_controller_creation() {
        let controller = Controller::new("TestController".to_string(), ControllerType::Movement);

        assert_eq!(controller.name.as_literal().unwrap(), "TestController");
        assert_eq!(
            controller.controller_type,
            Some(Value::Literal(ControllerType::Movement))
        );
    }

    #[test]
    fn test_object_controller_with_direct_controller() {
        let controller = Controller::new("DirectController".to_string(), ControllerType::Lateral);
        let object_controller = ObjectController::with_controller(controller);

        assert!(object_controller.controller().is_some());
        assert!(object_controller.catalog_reference().is_none());
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
        let mut properties = Properties::default();
        let property = Property {
            name: OSString::literal("testProp".to_string()),
            value: OSString::literal("testValue".to_string()),
        };
        properties.properties.push(property);

        assert_eq!(properties.properties.len(), 1);
    }

    #[test]
    fn test_controller_defaults() {
        // A controller built via a branch constructor leaves the *other* branch
        // unreachable: `ObjectControllerChoice` can hold exactly one of the two,
        // never both and never neither.
        let object_controller =
            ObjectController::with_catalog_reference(ControllerCatalogReference::new(
                "ControllerCatalog".to_string(),
                "TestController".to_string(),
            ));
        let properties = Properties::default();

        assert!(object_controller.controller().is_none());
        assert!(object_controller.catalog_reference().is_some());
        assert!(properties.properties.is_empty());
    }

    #[test]
    fn test_object_controller_named_controller() {
        let named_controller = ObjectController::with_named_controller(
            "TestController".to_string(),
            Controller::new("TestController".to_string(), ControllerType::Movement),
        );
        assert!(named_controller.controller().is_some());
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

    /// `ObjectControllerChoice` reaching serde is the enforcement: an empty
    /// `<ObjectController/>` is schema-invalid (XSD:1522-1528, neither branch
    /// carries `minOccurs="0"`) and must be rejected, not silently accepted as
    /// the pre-conversion `validate()` did.
    #[test]
    fn test_object_controller_zero_branches_rejected() {
        let xml = r#"<ObjectController/>"#;
        let result: Result<ObjectController, _> = quick_xml::de::from_str(xml);
        assert!(result.is_err(), "empty ObjectController must be rejected");
    }

    #[test]
    fn test_object_controller_two_branches_rejected() {
        let xml = r#"<ObjectController><Controller name="C1" controllerType="movement"/><CatalogReference catalogName="Cat" entryName="Entry"/></ObjectController>"#;
        let result: Result<ObjectController, _> = quick_xml::de::from_str(xml);
        assert!(
            result.is_err(),
            "Controller and CatalogReference together must be rejected"
        );
    }

    #[test]
    fn test_object_controller_controller_round_trips_byte_exact() {
        let xml = r#"<ObjectController><Controller name="C1" controllerType="movement"/></ObjectController>"#;
        let parsed: ObjectController = quick_xml::de::from_str(xml).unwrap();
        assert!(parsed.controller().is_some());
        let serialized = quick_xml::se::to_string(&parsed).unwrap();
        assert_eq!(serialized, xml);
    }

    #[test]
    fn test_object_controller_catalog_reference_round_trips_byte_exact() {
        let xml = r#"<ObjectController><CatalogReference catalogName="Cat" entryName="Entry"/></ObjectController>"#;
        let parsed: ObjectController = quick_xml::de::from_str(xml).unwrap();
        assert!(parsed.catalog_reference().is_some());
        let serialized = quick_xml::se::to_string(&parsed).unwrap();
        assert_eq!(serialized, xml);
    }
}
