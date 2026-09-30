//! Trajectory and route-based position types for path following

use crate::types::basic::Double;
use serde::{Deserialize, Serialize};

// The `Trajectory` struct that used to live here was a dead duplicate of
// `actions::movement::Trajectory` (the canonical one, boxed inside `TrajectoryRef`). It had
// zero consumers outside its own `impl`, its own unit tests and the `pub use` in
// `positions/mod.rs`, and it was also schema-wrong: `@closed` is XSD type `Boolean`
// (`Schema/OpenSCENARIO.xsd:2361`), a union that admits `$param`, and this copy declared it
// as a plain Rust `bool`. Removed; use `crate::types::actions::movement::Trajectory`.

/// Clothoid trajectory segment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Clothoid {
    /// Curvature at start
    #[serde(rename = "@curvature")]
    pub curvature: Double,
    /// Curvature derivative (clothoid parameter) — deprecated, use `curvature_prime`
    #[serde(
        rename = "@curvatureDot",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub curvature_dot: Option<Double>,
    /// Curvature derivative (current replacement for `curvatureDot`)
    #[serde(
        rename = "@curvaturePrime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub curvature_prime: Option<Double>,
    /// Length of the clothoid
    #[serde(rename = "@length")]
    pub length: Double,
    /// Start time
    #[serde(
        rename = "@startTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_time: Option<Double>,
    /// Stop time
    #[serde(rename = "@stopTime", default, skip_serializing_if = "Option::is_none")]
    pub stop_time: Option<Double>,
    /// Start position — required per XSD (`<xsd:sequence>` mandates exactly one `<Position>` child)
    #[serde(rename = "Position")]
    pub start_position: crate::types::positions::Position,
}

/// Reference to a trajectory, either defined inline or via catalog — XSD choice
/// of `<Trajectory>` | `<CatalogReference>` (xsd:2380-2385).
pub use crate::types::actions::movement::TrajectoryRef;

/// Position along a trajectory
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "TrajectoryPosition")]
pub struct TrajectoryPosition {
    /// S-coordinate along trajectory
    #[serde(rename = "@s")]
    pub s: Double,

    /// T-coordinate (lateral offset from trajectory)
    #[serde(rename = "@t", skip_serializing_if = "Option::is_none")]
    pub t: Option<Double>,

    /// Orientation relative to trajectory direction
    #[serde(rename = "Orientation", skip_serializing_if = "Option::is_none")]
    pub orientation: Option<crate::types::positions::road::Orientation>,

    /// Reference to the trajectory being followed — required per XSD
    #[serde(rename = "TrajectoryRef")]
    pub trajectory_ref: TrajectoryRef,
}

impl TrajectoryPosition {
    /// Create a new trajectory position
    pub fn new(s: f64, trajectory_ref: TrajectoryRef) -> Self {
        Self {
            s: Double::literal(s),
            t: None,
            orientation: None,
            trajectory_ref,
        }
    }

    /// Create trajectory position with lateral offset
    pub fn with_offset(s: f64, t: f64, trajectory_ref: TrajectoryRef) -> Self {
        Self {
            s: Double::literal(s),
            t: Some(Double::literal(t)),
            orientation: None,
            trajectory_ref,
        }
    }

    /// Add orientation to trajectory position
    pub fn with_orientation(
        mut self,
        orientation: crate::types::positions::road::Orientation,
    ) -> Self {
        self.orientation = Some(orientation);
        self
    }

    /// Create trajectory position at distance with offset
    pub fn at_distance(s: f64, t: f64, trajectory_ref: TrajectoryRef) -> Self {
        Self::with_offset(s, t, trajectory_ref)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::geometry::shapes::{Polyline, Shape, Vertex};
    use crate::types::positions::Position;

    /// A minimal schema-valid `Shape` for tests that need a concrete trajectory shape but
    /// do not exercise its content.
    ///
    /// Two vertices, because XSD `Polyline` (`Schema/OpenSCENARIO.xsd:1735`) declares
    /// `Vertex` with `minOccurs="2"`. This helper used to build one and call itself
    /// schema-valid; the plain `Vec` it was built on could not tell the difference.
    fn minimal_shape() -> Shape {
        Shape::polyline(Polyline {
            vertices: crate::types::basic::MinVec::new(vec![
                Vertex::new(Position::world_origin()),
                Vertex::new(Position::world_origin()),
            ])
            .unwrap(),
        })
    }

    #[test]
    fn test_trajectory_position_new() {
        let pos = TrajectoryPosition::new(
            50.0,
            TrajectoryRef::with_trajectory(crate::types::actions::movement::Trajectory::new(
                "TestTrajectory",
                false,
                minimal_shape(),
            )),
        );
        assert_eq!(pos.s.as_literal().unwrap(), &50.0);
        assert!(pos.t.is_none());
        assert!(pos.orientation.is_none());
    }

    #[test]
    fn test_trajectory_position_with_offset() {
        let pos = TrajectoryPosition::with_offset(
            100.0,
            -1.5,
            TrajectoryRef::with_trajectory(crate::types::actions::movement::Trajectory::new(
                "TestTrajectory",
                false,
                minimal_shape(),
            )),
        );
        assert_eq!(pos.s.as_literal().unwrap(), &100.0);
        assert_eq!(pos.t.unwrap().as_literal().unwrap(), &-1.5);

        // `at_distance` is the same constructor under another name.
        let pos = TrajectoryPosition::at_distance(
            200.0,
            -2.5,
            TrajectoryRef::with_trajectory(crate::types::actions::movement::Trajectory::new(
                "TestTrajectory",
                false,
                minimal_shape(),
            )),
        );
        assert_eq!(pos.s.as_literal().unwrap(), &200.0);
        assert_eq!(pos.t.unwrap().as_literal().unwrap(), &-2.5);
        assert!(pos.orientation.is_none());
    }

    #[test]
    fn test_trajectory_position_xml_roundtrip() {
        let pos = TrajectoryPosition::new(
            25.0,
            TrajectoryRef::with_trajectory(crate::types::actions::movement::Trajectory::new(
                "TestTrajectory",
                false,
                minimal_shape(),
            )),
        );
        let xml = quick_xml::se::to_string(&pos).unwrap();
        assert!(xml.contains("s=\"25\""));
        let deserialized: TrajectoryPosition = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(pos, deserialized);
    }

    // ------------------------------------------------------------------
    // Clothoid deserialization tests
    // ------------------------------------------------------------------

    /// Clothoid attributes must be serialized as XML attributes (not child elements)
    /// and must round-trip correctly.
    #[test]
    fn test_clothoid_xml_roundtrip() {
        use crate::types::positions::{Position, WorldPosition};

        let clothoid = Clothoid {
            curvature: Double::literal(0.1),
            curvature_dot: Some(Double::literal(0.01)),
            curvature_prime: None,
            length: Double::literal(50.0),
            start_time: None,
            stop_time: None,
            start_position: Position::world(WorldPosition::new(1.0, 2.0)),
        };
        let xml = quick_xml::se::to_string(&clothoid).unwrap();
        assert!(xml.contains(r#"curvature="0.1""#), "serialized: {xml}");
        assert!(xml.contains(r#"curvatureDot="0.01""#), "serialized: {xml}");
        assert!(xml.contains(r#"length="50""#), "serialized: {xml}");
        assert!(xml.contains("Position"), "serialized: {xml}");

        let deserialized: Clothoid = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(clothoid, deserialized);
    }

    /// Clothoid without curvatureDot must not emit a curvatureDot attribute,
    /// and the required Position element must still round-trip.
    #[test]
    fn test_clothoid_without_curvature_dot_omits_it() {
        use crate::types::positions::{Position, WorldPosition};

        let clothoid = Clothoid {
            curvature: Double::literal(0.2),
            curvature_dot: None,
            curvature_prime: None,
            length: Double::literal(10.0),
            start_time: None,
            stop_time: None,
            start_position: Position::world(WorldPosition::new(0.0, 0.0)),
        };
        let xml = quick_xml::se::to_string(&clothoid).unwrap();
        assert!(
            !xml.contains("curvatureDot"),
            "curvatureDot attr must be absent: {xml}"
        );
        let deserialized: Clothoid = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(clothoid, deserialized);
    }
}
