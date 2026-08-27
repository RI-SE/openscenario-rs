//! Miscellaneous object entity definition

use super::vehicle::Properties;
use crate::types::basic::{Double, OSString, ParameterDeclarations};
use crate::types::enums::MiscObjectCategory;
use crate::types::geometry::BoundingBox;
use serde::{Deserialize, Serialize};

/// Miscellaneous object entity definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MiscObject {
    /// Mass of the miscellaneous object in kg (REQUIRED by XSD)
    #[serde(rename = "@mass")]
    pub mass: Double,

    /// Category of the miscellaneous object
    #[serde(rename = "@miscObjectCategory")]
    pub misc_object_category: MiscObjectCategory,

    /// Name of the miscellaneous object
    #[serde(rename = "@name")]
    pub name: OSString,

    /// Path to an external 3D model
    #[serde(
        rename = "@model3d",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub model3d: Option<OSString>,

    /// Parameter declarations
    #[serde(
        rename = "ParameterDeclarations",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_declarations: Option<ParameterDeclarations>,

    /// Bounding box defining the object's spatial extents
    #[serde(rename = "BoundingBox")]
    pub bounding_box: BoundingBox,

    /// Additional properties
    #[serde(rename = "Properties", default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Properties>,
}

impl MiscObject {
    /// Create a new miscellaneous object
    pub fn new(name: String, mass: f64, category: MiscObjectCategory) -> Self {
        Self {
            mass: Double::literal(mass),
            misc_object_category: category,
            name: crate::types::basic::Value::literal(name),
            model3d: None,
            parameter_declarations: None,
            bounding_box: BoundingBox::default(),
            properties: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_misc_object_new() {
        let obj = MiscObject::new("Barrier1".to_string(), 100.0, MiscObjectCategory::Barrier);

        assert_eq!(obj.name.as_literal().unwrap(), "Barrier1");
        assert_eq!(obj.mass.as_literal().unwrap(), &100.0);
        assert_eq!(obj.misc_object_category, MiscObjectCategory::Barrier);
        assert!(obj.model3d.is_none());
        assert!(obj.properties.is_none());
    }

    #[test]
    fn test_misc_object_roundtrip() {
        let obj = MiscObject::new("Obstacle1".to_string(), 50.0, MiscObjectCategory::Obstacle);

        let xml = quick_xml::se::to_string(&obj).unwrap();
        assert!(xml.contains("mass=\"50\""));
        assert!(xml.contains("miscObjectCategory=\"obstacle\""));
        assert!(xml.contains("name=\"Obstacle1\""));
        assert!(xml.contains("BoundingBox"));

        let deserialized: MiscObject = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(deserialized.name.as_literal().unwrap(), "Obstacle1");
        assert_eq!(deserialized.mass.as_literal().unwrap(), &50.0);
        assert_eq!(
            deserialized.misc_object_category,
            MiscObjectCategory::Obstacle
        );
    }

    #[test]
    fn test_misc_object_with_model3d() {
        let mut obj = MiscObject::new("Gantry1".to_string(), 500.0, MiscObjectCategory::Gantry);
        obj.model3d = Some(crate::types::basic::Value::literal(
            "models/gantry.osgb".to_string(),
        ));

        let xml = quick_xml::se::to_string(&obj).unwrap();
        assert!(xml.contains("model3d=\"models/gantry.osgb\""));

        let deserialized: MiscObject = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(
            deserialized.model3d.unwrap().as_literal().unwrap(),
            "models/gantry.osgb"
        );
    }
}
