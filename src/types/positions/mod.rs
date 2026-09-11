//! Position type module for all spatial positioning systems
//!
//! This file contains:
//! - Base position traits and common positioning behaviors
//! - Position conversion utilities between coordinate systems
//! - Orientation handling and coordinate system transformations
//! - Position validation and constraint checking
//! - Spatial relationship calculations and utilities
//!
use crate::types::basic::{Double, OSString};
use serde::{Deserialize, Serialize};

pub mod relative;
pub mod road;
pub mod route;
pub mod trajectory;
pub mod world;

pub use crate::types::actions::movement::TrajectoryRef;
pub use relative::RelativeObjectPosition;
pub use road::{
    LanePosition, Orientation, RelativeLanePosition, RelativeRoadPosition, RoadPosition,
};
pub use route::{
    InRoutePosition, PositionInLaneCoordinates, PositionInRoadCoordinates, PositionOfCurrentEntity,
    RoutePosition, RouteRefElement,
};
pub use trajectory::{Trajectory, TrajectoryPosition};
pub use world::{GeographicPosition, WorldPosition};

/// Wrapper for Position element that contains position variants
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Position {
    #[serde(rename = "WorldPosition", skip_serializing_if = "Option::is_none")]
    pub world_position: Option<WorldPosition>,
    #[serde(
        rename = "RelativeWorldPosition",
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_world_position: Option<RelativeWorldPosition>,
    #[serde(rename = "RoadPosition", skip_serializing_if = "Option::is_none")]
    pub road_position: Option<RoadPosition>,
    #[serde(
        rename = "RelativeRoadPosition",
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_road_position: Option<RelativeRoadPosition>,
    #[serde(rename = "LanePosition", skip_serializing_if = "Option::is_none")]
    pub lane_position: Option<LanePosition>,
    #[serde(
        rename = "RelativeLanePosition",
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_lane_position: Option<RelativeLanePosition>,
    #[serde(rename = "RoutePosition", skip_serializing_if = "Option::is_none")]
    pub route_position: Option<RoutePosition>,
    #[serde(rename = "TrajectoryPosition", skip_serializing_if = "Option::is_none")]
    pub trajectory_position: Option<TrajectoryPosition>,
    #[serde(rename = "GeoPosition", skip_serializing_if = "Option::is_none")]
    pub geographic_position: Option<GeographicPosition>,
    #[serde(
        rename = "RelativeObjectPosition",
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_object_position: Option<RelativeObjectPosition>,
}

/// Relative world position relative to an entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct RelativeWorldPosition {
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,
    #[serde(rename = "@dx")]
    pub dx: Double,
    #[serde(rename = "@dy")]
    pub dy: Double,
    /// XSD: optional attribute
    #[serde(rename = "@dz", default, skip_serializing_if = "Option::is_none")]
    pub dz: Option<Double>,
    /// Orientation relative to the reference entity — XSD:1912, `minOccurs="0"`
    #[serde(
        rename = "Orientation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub orientation: Option<Orientation>,
}

impl RelativeWorldPosition {
    /// Create a new relative world position (required `entityRef`, `dx`, `dy`; XSD:1910-1922)
    pub fn new(entity_ref: &str, dx: f64, dy: f64) -> Self {
        Self {
            entity_ref: OSString::literal(entity_ref.to_string()),
            dx: Double::literal(dx),
            dy: Double::literal(dy),
            dz: None,
            orientation: None,
        }
    }
}

// Convenience constructors for Position
impl Position {
    /// A `WorldPosition` at the origin — an explicit, schema-valid placeholder.
    ///
    /// For call sites that need *a* position but do not care which: tests, doc examples,
    /// and fixtures asserting something other than the position itself. It is named for
    /// what it is. It replaced `Position::world_origin()`, which produced an all-`None` choice
    /// — schema-invalid (XSD `Position`, `Schema/OpenSCENARIO.xsd:1738-1751`, is a bare
    /// `xsd:choice`) while reading like a neutral value. Never reach for this in code that
    /// describes a real scenario: a position at (0, 0, 0) is content, and inventing it is
    /// category 1 of the `Default` policy.
    pub fn world_origin() -> Self {
        Self::world(WorldPosition::new(0.0, 0.0))
    }

    /// No branch selected — every choice field `None`.
    ///
    /// **Not schema-valid on its own**, for the reason given on
    /// [`Position::world_origin`]. It is the base for building a position one branch at a
    /// time; anything that serializes needs a branch filled in first.
    pub fn empty() -> Self {
        Self {
            world_position: None,
            relative_world_position: None,
            road_position: None,
            relative_road_position: None,
            lane_position: None,
            relative_lane_position: None,
            route_position: None,
            trajectory_position: None,
            geographic_position: None,
            relative_object_position: None,
        }
    }
    /// Create a Position with WorldPosition (the `WorldPosition` branch of the
    /// XSD `Position` choice, `Schema/OpenSCENARIO.xsd:1738-1751`).
    pub fn world(world_position: WorldPosition) -> Self {
        Self {
            world_position: Some(world_position),
            ..Self::empty()
        }
    }
    /// Create a Position with RelativeRoadPosition
    pub fn relative_road(relative_road_position: RelativeRoadPosition) -> Self {
        Self {
            relative_road_position: Some(relative_road_position),
            ..Self::empty()
        }
    }

    /// Create a Position with RelativeLanePosition
    pub fn relative_lane(relative_lane_position: RelativeLanePosition) -> Self {
        Self {
            relative_lane_position: Some(relative_lane_position),
            ..Self::empty()
        }
    }

    /// Create a Position with TrajectoryPosition
    pub fn trajectory(trajectory_position: TrajectoryPosition) -> Self {
        Self {
            trajectory_position: Some(trajectory_position),
            ..Self::empty()
        }
    }

    /// Create a Position with GeographicPosition
    pub fn geographic(geographic_position: GeographicPosition) -> Self {
        Self {
            geographic_position: Some(geographic_position),
            ..Self::empty()
        }
    }

    /// Create a Position with RelativeObjectPosition
    pub fn relative_object(relative_object_position: RelativeObjectPosition) -> Self {
        Self {
            relative_object_position: Some(relative_object_position),
            ..Self::empty()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::basic::Value;

    /// Replaces `test_position_default_is_all_none`, whose subject — the derived
    /// `Default` — OSR-10 removed. `world_origin` is the honest replacement, and its
    /// contract is the opposite one: it *does* select a branch, which is the whole point.
    #[test]
    fn test_position_world_origin_selects_the_world_branch() {
        let pos = Position::world_origin();
        let world = pos
            .world_position
            .as_ref()
            .expect("world_origin must select the WorldPosition branch");
        assert_eq!(world.x, Value::Literal(0.0));
        assert_eq!(world.y, Value::Literal(0.0));
        assert!(pos.lane_position.is_none());
        assert!(pos.road_position.is_none());
        assert_ne!(pos, Position::empty());
    }

    #[test]
    fn test_position_empty_has_all_none() {
        let pos = Position::empty();
        assert!(pos.world_position.is_none());
        assert!(pos.relative_world_position.is_none());
        assert!(pos.road_position.is_none());
        assert!(pos.lane_position.is_none());
        assert!(pos.trajectory_position.is_none());
        assert!(pos.geographic_position.is_none());
        assert!(pos.relative_object_position.is_none());
    }

    #[test]
    fn test_position_trajectory_constructor() {
        let tp = TrajectoryPosition::new(
            10.0,
            TrajectoryRef::with_trajectory(crate::types::actions::movement::Trajectory::new(
                "TestTrajectory",
                false,
                crate::types::geometry::shapes::Shape::empty(),
            )),
        );
        let pos = Position::trajectory(tp.clone());
        assert!(pos.trajectory_position.is_some());
        assert!(pos.world_position.is_none());
    }

    #[test]
    fn test_position_geographic_constructor() {
        let gp = GeographicPosition::new(48.0, 11.0);
        let pos = Position::geographic(gp);
        assert!(pos.geographic_position.is_some());
        assert!(pos.world_position.is_none());
    }

    #[test]
    fn test_relative_world_position_new() {
        // OSR-04 (agent G): `RelativeWorldPosition` no longer has a fabricating `Default`
        // (it invented entityRef="DefaultEntity"); `::new` requires the real fields.
        let rwp = RelativeWorldPosition::new("Ego", 1.0, 2.0);
        assert_eq!(rwp.entity_ref.as_literal().unwrap(), "Ego");
        assert_eq!(rwp.dx.as_literal().unwrap(), &1.0);
        assert!(rwp.dz.is_none());
    }

    #[test]
    fn test_relative_world_position_parse_without_dz() {
        let xml = r#"<RelativeWorldPosition entityRef="Ego" dx="1" dy="2"/>"#;
        let pos: RelativeWorldPosition = quick_xml::de::from_str(xml).unwrap();
        assert!(pos.dz.is_none());
    }

    #[test]
    fn test_relative_world_position_serialize_none_dz_omitted() {
        let pos = RelativeWorldPosition::new("Ego", 1.0, 2.0);
        let xml = quick_xml::se::to_string(&pos).unwrap();
        assert!(!xml.contains("dz="), "serialized: {xml}");
    }

    #[test]
    fn test_position_xml_roundtrip() {
        let pos = Position {
            world_position: Some(WorldPosition::new(1.0, 2.0)),
            ..Position::empty()
        };
        let xml = quick_xml::se::to_string(&pos).unwrap();
        assert!(xml.contains("WorldPosition"));
        let deserialized: Position = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(pos, deserialized);
    }

    /// XSD:1912 — `RelativeWorldPosition` carries an optional `<Orientation>`
    /// child element (`minOccurs="0"`), which was previously missing from the
    /// Rust type. Verify it round-trips through XML serialize/deserialize.
    #[test]
    fn test_relative_world_position_orientation_round_trip() {
        let pos = RelativeWorldPosition {
            entity_ref: OSString::literal("Ego".to_string()),
            dx: Double::literal(1.0),
            dy: Double::literal(2.0),
            dz: Some(Double::literal(0.5)),
            orientation: Some(Orientation {
                h: Some(Double::literal(1.57)),
                p: None,
                r: None,
                reference_context: Some(Value::Literal(
                    crate::types::enums::ReferenceContext::Relative,
                )),
            }),
        };

        let xml = quick_xml::se::to_string(&pos).unwrap();
        assert!(
            xml.contains("<Orientation"),
            "Orientation element missing from serialized XML: {xml}"
        );

        let deserialized: RelativeWorldPosition = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(pos, deserialized);
        assert!(deserialized.orientation.is_some());
    }

    /// Regression guard: a `RelativeWorldPosition` without an `<Orientation>`
    /// child must still parse, leaving the field `None`.
    #[test]
    fn test_relative_world_position_parse_without_orientation() {
        let xml = r#"<RelativeWorldPosition entityRef="Ego" dx="1" dy="2"/>"#;
        let pos: RelativeWorldPosition = quick_xml::de::from_str(xml).unwrap();
        assert!(pos.orientation.is_none());
    }
}
