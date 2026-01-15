//! Pedestrian entity definition

use super::vehicle::Properties;
use crate::types::basic::{Double, OSString, ParameterDeclarations};
use crate::types::enums::{PedestrianCategory, Role};
use crate::types::geometry::BoundingBox;
use serde::{Deserialize, Serialize};

/// Pedestrian entity definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pedestrian {
    /// Name of the pedestrian
    #[serde(rename = "@name")]
    pub name: OSString,

    /// Category of the pedestrian (pedestrian, wheelchair, animal)
    #[serde(rename = "@pedestrianCategory")]
    pub pedestrian_category: PedestrianCategory,

    /// Mass of the pedestrian in kg (REQUIRED by XSD)
    #[serde(rename = "@mass")]
    pub mass: Double,

    /// Role of the pedestrian (civil, police, ambulance, etc.)
    #[serde(rename = "@role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,

    /// 3D model file path
    #[serde(rename = "@model3d", skip_serializing_if = "Option::is_none")]
    pub model3d: Option<String>,

    /// Bounding box defining the pedestrian's spatial extents
    #[serde(rename = "BoundingBox")]
    pub bounding_box: BoundingBox,

    /// Additional properties
    #[serde(rename = "Properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Properties>,

    /// Parameter declarations
    #[serde(
        rename = "ParameterDeclarations",
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_declarations: Option<ParameterDeclarations>,
}

impl Default for Pedestrian {
    fn default() -> Self {
        Self {
            name: crate::types::basic::Value::literal("DefaultPedestrian".to_string()),
            pedestrian_category: PedestrianCategory::Pedestrian,
            mass: Double::literal(75.0),
            role: None,
            model3d: None,
            bounding_box: BoundingBox {
                center: crate::types::geometry::Center::default(),
                dimensions: crate::types::geometry::Dimensions {
                    width: crate::types::basic::Value::literal(0.6),
                    length: crate::types::basic::Value::literal(0.6),
                    height: crate::types::basic::Value::literal(1.8),
                },
            },
            properties: None,
            parameter_declarations: None,
        }
    }
}

impl Pedestrian {
    pub fn new_pedestrian(name: String) -> Self {
        Self {
            name: crate::types::basic::Value::literal(name),
            pedestrian_category: PedestrianCategory::Pedestrian,
            mass: Double::literal(75.0),
            role: Some(Role::None),
            model3d: None,
            bounding_box: BoundingBox {
                center: crate::types::geometry::Center::default(),
                dimensions: crate::types::geometry::Dimensions {
                    width: Double::literal(0.6),
                    length: Double::literal(0.6),
                    height: Double::literal(1.8),
                },
            },
            properties: None,
            parameter_declarations: None,
        }
    }

    pub fn new_wheelchair(name: String) -> Self {
        Self {
            name: crate::types::basic::Value::literal(name),
            pedestrian_category: PedestrianCategory::Wheelchair,
            mass: Double::literal(85.0),
            role: Some(Role::Civil),
            model3d: None,
            bounding_box: BoundingBox {
                center: crate::types::geometry::Center::default(),
                dimensions: crate::types::geometry::Dimensions {
                    width: Double::literal(0.8),
                    length: Double::literal(1.0),
                    height: Double::literal(1.4),
                },
            },
            properties: None,
            parameter_declarations: None,
        }
    }

    pub fn new_animal(name: String) -> Self {
        Self {
            name: crate::types::basic::Value::literal(name),
            pedestrian_category: PedestrianCategory::Animal,
            mass: Double::literal(50.0),
            role: Some(Role::None),
            model3d: None,
            bounding_box: BoundingBox {
                center: crate::types::geometry::Center::default(),
                dimensions: crate::types::geometry::Dimensions {
                    width: Double::literal(0.5),
                    length: Double::literal(0.5),
                    height: Double::literal(0.8),
                },
            },
            properties: None,
            parameter_declarations: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pedestrian_default() {
        let pedestrian = Pedestrian::default();

        assert_eq!(pedestrian.name.as_literal().unwrap(), "DefaultPedestrian");
        assert_eq!(
            pedestrian.pedestrian_category,
            PedestrianCategory::Pedestrian
        );

        // Should have pedestrian-appropriate dimensions
        assert_eq!(
            pedestrian
                .bounding_box
                .dimensions
                .width
                .as_literal()
                .unwrap(),
            &0.6
        );
        assert_eq!(
            pedestrian
                .bounding_box
                .dimensions
                .height
                .as_literal()
                .unwrap(),
            &1.8
        );
    }

    #[test]
    fn test_pedestrian_creation() {
        let pedestrian = Pedestrian {
            name: crate::types::basic::Value::literal("TestPedestrian".to_string()),
            pedestrian_category: PedestrianCategory::Wheelchair,
            mass: Double::literal(85.0),
            role: Some(Role::Civil),
            model3d: None,
            bounding_box: BoundingBox::default(),
            properties: None,
            parameter_declarations: None,
        };

        assert_eq!(pedestrian.name.as_literal().unwrap(), "TestPedestrian");
        assert_eq!(
            pedestrian.pedestrian_category,
            PedestrianCategory::Wheelchair
        );
    }

    #[test]
    fn test_pedestrian_serialization() {
        let pedestrian = Pedestrian::default();

        // Test that serialization works
        let xml = quick_xml::se::to_string(&pedestrian).unwrap();
        assert!(xml.contains("name=\"DefaultPedestrian\""));
        assert!(xml.contains("mass=\"75\""));
        assert!(xml.contains("pedestrianCategory=\"pedestrian\""));
        assert!(xml.contains("BoundingBox"));
    }

    #[test]
    fn test_new_pedestrian() {
        let pedestrian = Pedestrian::new_pedestrian("Ped1".to_string());

        assert_eq!(pedestrian.name.as_literal().unwrap(), "Ped1");
        assert_eq!(
            pedestrian.pedestrian_category,
            PedestrianCategory::Pedestrian
        );
        assert_eq!(pedestrian.mass.as_literal().unwrap(), &75.0);
        assert_eq!(pedestrian.role.unwrap(), Role::None);
    }

    #[test]
    fn test_new_wheelchair() {
        let pedestrian = Pedestrian::new_wheelchair("Wheel1".to_string());

        assert_eq!(pedestrian.name.as_literal().unwrap(), "Wheel1");
        assert_eq!(
            pedestrian.pedestrian_category,
            PedestrianCategory::Wheelchair
        );
        assert_eq!(pedestrian.mass.as_literal().unwrap(), &85.0);
        assert_eq!(pedestrian.role.unwrap(), Role::Civil);
    }

    #[test]
    fn test_new_animal() {
        let pedestrian = Pedestrian::new_animal("Dog1".to_string());

        assert_eq!(pedestrian.name.as_literal().unwrap(), "Dog1");
        assert_eq!(pedestrian.pedestrian_category, PedestrianCategory::Animal);
        assert_eq!(pedestrian.mass.as_literal().unwrap(), &50.0);
        assert_eq!(pedestrian.role.unwrap(), Role::None);
    }
}
