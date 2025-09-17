//! Pedestrian entity definition

use crate::types::basic::OSString;
use crate::types::enums::PedestrianCategory;
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

    /// Bounding box defining the pedestrian's spatial extents
    #[serde(rename = "BoundingBox")]
    pub bounding_box: BoundingBox,
}

impl Default for Pedestrian {
    fn default() -> Self {
        Self {
            name: crate::types::basic::Value::literal("DefaultPedestrian".to_string()),
            pedestrian_category: PedestrianCategory::Pedestrian,
            bounding_box: BoundingBox {
                center: crate::types::geometry::Center::default(),
                dimensions: crate::types::geometry::Dimensions {
                    width: crate::types::basic::Value::literal(0.6), // Typical person width
                    length: crate::types::basic::Value::literal(0.6), // Typical person depth
                    height: crate::types::basic::Value::literal(1.8), // Typical person height
                },
            },
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
            bounding_box: BoundingBox::default(),
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
        assert!(xml.contains("pedestrianCategory=\"pedestrian\""));
        assert!(xml.contains("BoundingBox"));
    }
}
