//! Road condition and infrastructure types
//!
//! This file contains:
//! - RoadCondition with friction scale factors for surface properties
//! - Road surface properties affecting vehicle dynamics
//!
use crate::types::basic::Double;
use crate::types::entities::vehicle::Properties;
use crate::types::enums::Wetness;
use serde::{Deserialize, Serialize};

/// Road surface conditions affecting vehicle dynamics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoadCondition {
    #[serde(rename = "@frictionScaleFactor")]
    pub friction_scale_factor: Double,
    #[serde(rename = "@wetness", default, skip_serializing_if = "Option::is_none")]
    pub wetness: Option<Wetness>,
    #[serde(
        rename = "Properties",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub properties: Option<Properties>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_road_condition_minimal() {
        let xml = r#"<RoadCondition frictionScaleFactor="1.0"/>"#;
        let rc: RoadCondition = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(rc.friction_scale_factor.as_literal(), Some(&1.0));
        assert!(rc.wetness.is_none());
        assert!(rc.properties.is_none());
    }

    #[test]
    fn test_road_condition_roundtrip_wetness() {
        let rc = RoadCondition {
            friction_scale_factor: Double::literal(0.8),
            wetness: Some(Wetness::Moist),
            properties: None,
        };
        let xml = quick_xml::se::to_string(&rc).unwrap();
        let deserialized: RoadCondition = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(rc, deserialized);
    }
}
