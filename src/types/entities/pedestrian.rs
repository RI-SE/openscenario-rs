//! Pedestrian entity definition

use super::vehicle::Properties;
use crate::types::basic::{Double, OSString, ParameterDeclarations, Value};
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
    pub pedestrian_category: Value<PedestrianCategory>,

    /// Mass of the pedestrian in kg (REQUIRED by XSD)
    #[serde(rename = "@mass")]
    pub mass: Double,

    /// Role of the pedestrian (civil, police, ambulance, etc.)
    #[serde(rename = "@role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Value<Role>>,

    /// Deprecated model reference; prefer `model3d`.
    #[serde(rename = "@model", default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

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

impl Pedestrian {
    pub fn new_pedestrian(name: String) -> Self {
        Self {
            name: crate::types::basic::Value::literal(name),
            pedestrian_category: Value::Literal(PedestrianCategory::Pedestrian),
            mass: Double::literal(75.0),
            role: Some(Value::Literal(Role::None)),
            model: None,
            model3d: None,
            bounding_box: BoundingBox {
                center: crate::types::geometry::Center::new(0.0, 0.0, 0.0),
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
            pedestrian_category: Value::Literal(PedestrianCategory::Wheelchair),
            mass: Double::literal(85.0),
            role: Some(Value::Literal(Role::Civil)),
            model: None,
            model3d: None,
            bounding_box: BoundingBox {
                center: crate::types::geometry::Center::new(0.0, 0.0, 0.0),
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
            pedestrian_category: Value::Literal(PedestrianCategory::Animal),
            mass: Double::literal(50.0),
            role: Some(Value::Literal(Role::None)),
            model: None,
            model3d: None,
            bounding_box: BoundingBox {
                center: crate::types::geometry::Center::new(0.0, 0.0, 0.0),
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
    fn presets_set_category_mass_role_and_size() {
        // (preset, category, mass, role, width, height)
        let cases = [
            (
                Pedestrian::new_pedestrian("P".into()),
                PedestrianCategory::Pedestrian,
                75.0,
                Role::None,
                0.6,
                1.8,
            ),
            (
                Pedestrian::new_wheelchair("W".into()),
                PedestrianCategory::Wheelchair,
                85.0,
                Role::Civil,
                0.8,
                1.4,
            ),
            (
                Pedestrian::new_animal("A".into()),
                PedestrianCategory::Animal,
                50.0,
                Role::None,
                0.5,
                0.8,
            ),
        ];
        for (pedestrian, category, mass, role, width, height) in cases {
            let name = pedestrian.name.as_literal().unwrap().clone();
            let dims = &pedestrian.bounding_box.dimensions;
            assert_eq!(
                pedestrian.pedestrian_category,
                Value::Literal(category),
                "{name}"
            );
            assert_eq!(pedestrian.mass.as_literal(), Some(&mass), "{name}: mass");
            assert_eq!(pedestrian.role, Some(Value::Literal(role)), "{name}: role");
            assert_eq!(dims.width.as_literal(), Some(&width), "{name}: width");
            assert_eq!(dims.height.as_literal(), Some(&height), "{name}: height");
        }
    }

    #[test]
    fn test_pedestrian_deprecated_model_round_trip() {
        let xml = r#"<Pedestrian name="P1" pedestrianCategory="pedestrian" mass="75" model="oldModel.osgb">
    <BoundingBox>
        <Center x="0" y="0" z="0.9"/>
        <Dimensions width="0.6" length="0.6" height="1.8"/>
    </BoundingBox>
</Pedestrian>"#;
        let pedestrian: Pedestrian = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(pedestrian.model.as_deref(), Some("oldModel.osgb"));

        let serialized = quick_xml::se::to_string(&pedestrian).unwrap();
        assert!(
            serialized.contains(r#"model="oldModel.osgb""#),
            "serialized: {serialized}"
        );
        let reparsed: Pedestrian = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(pedestrian, reparsed);
    }
}
