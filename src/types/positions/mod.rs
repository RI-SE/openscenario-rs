//! Position types. `Position` is the XSD choice: exactly one of world, relative,
//! road, lane, trajectory or route coordinates. Its constructors each select one
//! branch, since a `Position` with no branch set is not schema-valid.
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
    InRoutePosition, InRoutePositionChoice, PositionInLaneCoordinates, PositionInRoadCoordinates,
    PositionOfCurrentEntity, RoutePosition, RouteRefElement,
};
pub use trajectory::TrajectoryPosition;
pub use world::{GeographicPosition, WorldPosition};

/// The `<Position>` element: exactly one of the ten coordinate systems.
///
/// XSD `Position` (`Schema/OpenSCENARIO.xsd:1738-1749`) is a bare `xsd:choice`, so an
/// instance names exactly one branch. The branch is held in a `$value` field carrying an
/// externally tagged enum, which makes that cardinality a property of the type: serde
/// rejects a document naming no branch with ``missing field `$value` `` and one naming two
/// with ``duplicate field `$value` ``. The previous shape was ten parallel `Option` fields,
/// which is the `xsd:all` production rather than the choice, and which accepted both of
/// those documents and re-serialized them unchanged.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename = "Position")]
pub struct Position {
    /// The coordinate system named by this `<Position>` element.
    ///
    /// `$value` takes the element name from the variant name, so each variant is spelled
    /// exactly as the XSD element it models.
    #[serde(rename = "$value")]
    pub position: PositionChoice,
}

/// The ten branches of the XSD `Position` choice, in schema order
/// (`Schema/OpenSCENARIO.xsd:1739-1748`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PositionChoice {
    /// Absolute position in world coordinates.
    WorldPosition(WorldPosition),
    /// Position offset from a reference entity in world coordinates.
    RelativeWorldPosition(RelativeWorldPosition),
    /// Position offset from a reference entity in that entity's coordinates.
    RelativeObjectPosition(RelativeObjectPosition),
    /// Absolute position in road coordinates.
    RoadPosition(RoadPosition),
    /// Position offset from a reference entity in road coordinates.
    RelativeRoadPosition(RelativeRoadPosition),
    /// Absolute position in lane coordinates.
    LanePosition(LanePosition),
    /// Position offset from a reference entity in lane coordinates.
    RelativeLanePosition(RelativeLanePosition),
    /// Position along a route.
    RoutePosition(RoutePosition),
    /// Position in geographic coordinates. The XSD element is `GeoPosition`, hence the
    /// variant name differs from the Rust type name `GeographicPosition`.
    GeoPosition(GeographicPosition),
    /// Position along a trajectory.
    TrajectoryPosition(TrajectoryPosition),
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
    /// what it is. Never reach for this in code that describes a real scenario: a position
    /// at (0, 0, 0) is content, and inventing it is category 1 of the `Default` policy.
    pub fn world_origin() -> Self {
        Self::world(WorldPosition::new(0.0, 0.0))
    }

    /// Create a Position holding the `WorldPosition` branch.
    pub fn world(world_position: WorldPosition) -> Self {
        Self {
            position: PositionChoice::WorldPosition(world_position),
        }
    }

    /// Create a Position holding the `RelativeWorldPosition` branch.
    pub fn relative_world(relative_world_position: RelativeWorldPosition) -> Self {
        Self {
            position: PositionChoice::RelativeWorldPosition(relative_world_position),
        }
    }

    /// Create a Position holding the `RoadPosition` branch.
    pub fn road(road_position: RoadPosition) -> Self {
        Self {
            position: PositionChoice::RoadPosition(road_position),
        }
    }

    /// Create a Position holding the `RelativeRoadPosition` branch.
    pub fn relative_road(relative_road_position: RelativeRoadPosition) -> Self {
        Self {
            position: PositionChoice::RelativeRoadPosition(relative_road_position),
        }
    }

    /// Create a Position holding the `LanePosition` branch.
    pub fn lane(lane_position: LanePosition) -> Self {
        Self {
            position: PositionChoice::LanePosition(lane_position),
        }
    }

    /// Create a Position holding the `RelativeLanePosition` branch.
    pub fn relative_lane(relative_lane_position: RelativeLanePosition) -> Self {
        Self {
            position: PositionChoice::RelativeLanePosition(relative_lane_position),
        }
    }

    /// Create a Position holding the `RoutePosition` branch.
    pub fn route(route_position: RoutePosition) -> Self {
        Self {
            position: PositionChoice::RoutePosition(route_position),
        }
    }

    /// Create a Position holding the `TrajectoryPosition` branch.
    pub fn trajectory(trajectory_position: TrajectoryPosition) -> Self {
        Self {
            position: PositionChoice::TrajectoryPosition(trajectory_position),
        }
    }

    /// Create a Position holding the `GeoPosition` branch.
    pub fn geographic(geographic_position: GeographicPosition) -> Self {
        Self {
            position: PositionChoice::GeoPosition(geographic_position),
        }
    }

    /// Create a Position holding the `RelativeObjectPosition` branch.
    pub fn relative_object(relative_object_position: RelativeObjectPosition) -> Self {
        Self {
            position: PositionChoice::RelativeObjectPosition(relative_object_position),
        }
    }
}

/// Per-branch read accessors.
///
/// These are views over the single branch the choice holds, not storage. They exist so a
/// caller interested in one coordinate system does not have to write the `match` arm and
/// the discard arm; a caller that handles several branches should match on
/// [`Position::position`] directly instead.
impl Position {
    /// The `WorldPosition` branch, if that is the branch held.
    pub fn world_position(&self) -> Option<&WorldPosition> {
        match &self.position {
            PositionChoice::WorldPosition(v) => Some(v),
            _ => None,
        }
    }

    /// The `RelativeWorldPosition` branch, if that is the branch held.
    pub fn relative_world_position(&self) -> Option<&RelativeWorldPosition> {
        match &self.position {
            PositionChoice::RelativeWorldPosition(v) => Some(v),
            _ => None,
        }
    }

    /// The `RelativeObjectPosition` branch, if that is the branch held.
    pub fn relative_object_position(&self) -> Option<&RelativeObjectPosition> {
        match &self.position {
            PositionChoice::RelativeObjectPosition(v) => Some(v),
            _ => None,
        }
    }

    /// The `RoadPosition` branch, if that is the branch held.
    pub fn road_position(&self) -> Option<&RoadPosition> {
        match &self.position {
            PositionChoice::RoadPosition(v) => Some(v),
            _ => None,
        }
    }

    /// The `RelativeRoadPosition` branch, if that is the branch held.
    pub fn relative_road_position(&self) -> Option<&RelativeRoadPosition> {
        match &self.position {
            PositionChoice::RelativeRoadPosition(v) => Some(v),
            _ => None,
        }
    }

    /// The `LanePosition` branch, if that is the branch held.
    pub fn lane_position(&self) -> Option<&LanePosition> {
        match &self.position {
            PositionChoice::LanePosition(v) => Some(v),
            _ => None,
        }
    }

    /// The `RelativeLanePosition` branch, if that is the branch held.
    pub fn relative_lane_position(&self) -> Option<&RelativeLanePosition> {
        match &self.position {
            PositionChoice::RelativeLanePosition(v) => Some(v),
            _ => None,
        }
    }

    /// The `RoutePosition` branch, if that is the branch held.
    pub fn route_position(&self) -> Option<&RoutePosition> {
        match &self.position {
            PositionChoice::RoutePosition(v) => Some(v),
            _ => None,
        }
    }

    /// The `GeoPosition` branch, if that is the branch held.
    pub fn geographic_position(&self) -> Option<&GeographicPosition> {
        match &self.position {
            PositionChoice::GeoPosition(v) => Some(v),
            _ => None,
        }
    }

    /// The `TrajectoryPosition` branch, if that is the branch held.
    pub fn trajectory_position(&self) -> Option<&TrajectoryPosition> {
        match &self.position {
            PositionChoice::TrajectoryPosition(v) => Some(v),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::basic::Value;

    #[test]
    fn test_position_world_origin_selects_the_world_branch() {
        let pos = Position::world_origin();
        let world = pos
            .world_position()
            .expect("world_origin must select the WorldPosition branch");
        assert_eq!(world.x, Value::Literal(0.0));
        assert_eq!(world.y, Value::Literal(0.0));
        assert!(pos.lane_position().is_none());
        assert!(pos.road_position().is_none());
    }

    #[test]
    fn test_relative_world_position_parse_without_dz_or_orientation() {
        // `@dz` and `<Orientation>` are both optional (XSD:1910-1922).
        let xml = r#"<RelativeWorldPosition entityRef="Ego" dx="1" dy="2"/>"#;
        let pos: RelativeWorldPosition = quick_xml::de::from_str(xml).unwrap();
        assert!(pos.dz.is_none());
        assert!(pos.orientation.is_none());
    }

    #[test]
    fn test_relative_world_position_new_serializes_required_attributes_only() {
        let pos = RelativeWorldPosition::new("Ego", 1.0, 2.0);
        let xml = quick_xml::se::to_string(&pos).unwrap();
        assert!(xml.contains(r#"entityRef="Ego""#), "serialized: {xml}");
        assert!(xml.contains(r#"dx="1""#), "serialized: {xml}");
        assert!(xml.contains(r#"dy="2""#), "serialized: {xml}");
        assert!(!xml.contains("dz="), "serialized: {xml}");
        assert!(!xml.contains("<Orientation"), "serialized: {xml}");
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
}
