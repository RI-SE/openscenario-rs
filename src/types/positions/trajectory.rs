//! Trajectory and route-based position types for path following

use crate::types::basic::{Double, OSString};
use serde::{Deserialize, Serialize};

/// Trajectory definition with shape and parameters
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Trajectory {
    /// Name of the trajectory
    pub name: Option<OSString>,
    /// Whether the trajectory is closed (forms a loop)
    pub closed: Option<bool>,
    /// Shape definition of the trajectory
    pub shape: TrajectoryShape,
}

/// Shape of a trajectory (polyline, clothoid, NURBS, etc.)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum TrajectoryShape {
    /// Simple polyline trajectory
    Polyline(Polyline),
    /// Clothoid-based trajectory
    Clothoid(Clothoid),
}

/// Polyline trajectory with vertices
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Polyline {
    /// Vertices defining the polyline
    pub vertex: Vec<Vertex>,
}

/// Vertex in a trajectory
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vertex {
    /// Time at this vertex (optional — XSD does not mark it as required)
    #[serde(rename = "@time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Double>,
    /// Position at this vertex
    #[serde(rename = "Position")]
    pub position: crate::types::positions::Position,
}

/// Clothoid trajectory segment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Clothoid {
    /// Curvature at start
    #[serde(rename = "@curvature")]
    pub curvature: Double,
    /// Curvature derivative (clothoid parameter)
    #[serde(rename = "@curvatureDot")]
    pub curvature_dot: Double,
    /// Length of the clothoid
    #[serde(rename = "@length")]
    pub length: Double,
    /// Start position
    #[serde(rename = "Position", skip_serializing_if = "Option::is_none")]
    pub start_position: Option<crate::types::positions::Position>,
}

/// Trajectory following mode
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TrajectoryFollowingMode {
    /// Follow trajectory position exactly
    #[serde(rename = "position")]
    Position,
    /// Follow trajectory timing
    #[serde(rename = "timing")]
    Timing,
}

/// Reference to a trajectory in a catalog
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrajectoryRef {
    /// Name/ID of the trajectory
    #[serde(rename = "@trajectory")]
    pub trajectory: OSString,
}

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
}

impl TrajectoryPosition {
    /// Create a new trajectory position
    pub fn new(s: f64) -> Self {
        Self {
            s: Double::literal(s),
            t: None,
            orientation: None,
        }
    }

    /// Create trajectory position with lateral offset
    pub fn with_offset(s: f64, t: f64) -> Self {
        Self {
            s: Double::literal(s),
            t: Some(Double::literal(t)),
            orientation: None,
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
    pub fn at_distance(s: f64, t: f64) -> Self {
        Self::with_offset(s, t)
    }
}

impl Default for TrajectoryPosition {
    fn default() -> Self {
        Self {
            s: Double::literal(0.0),
            t: None,
            orientation: None,
        }
    }
}

impl Default for Trajectory {
    fn default() -> Self {
        Self {
            name: None,
            closed: None,
            shape: TrajectoryShape::Polyline(Polyline { vertex: Vec::new() }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trajectory_position_new() {
        let pos = TrajectoryPosition::new(50.0);
        assert_eq!(pos.s.as_literal().unwrap(), &50.0);
        assert!(pos.t.is_none());
        assert!(pos.orientation.is_none());
    }

    #[test]
    fn test_trajectory_position_with_offset() {
        let pos = TrajectoryPosition::with_offset(100.0, -1.5);
        assert_eq!(pos.s.as_literal().unwrap(), &100.0);
        assert_eq!(pos.t.unwrap().as_literal().unwrap(), &-1.5);
    }

    #[test]
    fn test_trajectory_position_xml_roundtrip() {
        let pos = TrajectoryPosition::new(25.0);
        let xml = quick_xml::se::to_string(&pos).unwrap();
        assert!(xml.contains("s=\"25\""));
        let deserialized: TrajectoryPosition = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(pos, deserialized);
    }

    #[test]
    fn test_trajectory_default_is_empty_polyline() {
        let traj = Trajectory::default();
        assert!(traj.name.is_none());
        assert!(traj.closed.is_none());
        match &traj.shape {
            TrajectoryShape::Polyline(p) => assert!(p.vertex.is_empty()),
            _ => panic!("Expected Polyline shape"),
        }
    }

    // ------------------------------------------------------------------
    // Vertex deserialization tests — covering the primary bug fix
    // ------------------------------------------------------------------

    /// A Vertex element with no `time` attribute must deserialize successfully.
    /// Previously `rename_all = "camelCase"` caused quick-xml to fail with
    /// "missing field '@time'" even though the XSD marks `time` as optional.
    #[test]
    fn test_vertex_xml_roundtrip_without_time() {
        let xml = r#"<Vertex><Position><WorldPosition x="1" y="2"/></Position></Vertex>"#;
        let v: Vertex = quick_xml::de::from_str(xml).unwrap();
        assert!(v.time.is_none());
        assert!(v.position.world_position.is_some());
    }

    /// A Vertex element with a `time` attribute must round-trip correctly.
    #[test]
    fn test_vertex_xml_roundtrip_with_time() {
        use crate::types::basic::Value;
        use crate::types::positions::{Position, WorldPosition};

        let vertex = Vertex {
            time: Some(Double::literal(1.5)),
            position: Position {
                world_position: Some(WorldPosition::new(10.0, 20.0)),
                ..Position::empty()
            },
        };
        let xml = quick_xml::se::to_string(&vertex).unwrap();
        assert!(xml.contains(r#"time="1.5""#), "serialized XML: {xml}");
        assert!(xml.contains("Position"), "serialized XML: {xml}");

        let deserialized: Vertex = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(vertex, deserialized);
    }

    /// Serializing a Vertex without time must NOT emit a `time` attribute.
    #[test]
    fn test_vertex_no_time_not_serialized() {
        use crate::types::positions::{Position, WorldPosition};

        let vertex = Vertex {
            time: None,
            position: Position {
                world_position: Some(WorldPosition::default()),
                ..Position::empty()
            },
        };
        let xml = quick_xml::se::to_string(&vertex).unwrap();
        assert!(!xml.contains("time="), "time attr must be absent: {xml}");
    }

    /// Polyline with one time-bearing and one time-less vertex must round-trip.
    #[test]
    fn test_polyline_xml_roundtrip_mixed_time() {
        use crate::types::positions::{Position, WorldPosition};

        let polyline = Polyline {
            vertex: vec![
                Vertex {
                    time: Some(Double::literal(0.0)),
                    position: Position {
                        world_position: Some(WorldPosition::new(0.0, 0.0)),
                        ..Position::empty()
                    },
                },
                Vertex {
                    time: None,
                    position: Position {
                        world_position: Some(WorldPosition::new(5.0, 5.0)),
                        ..Position::empty()
                    },
                },
            ],
        };
        let xml = quick_xml::se::to_string(&polyline).unwrap();
        let deserialized: Polyline = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(polyline, deserialized);
        assert_eq!(deserialized.vertex.len(), 2);
        assert!(deserialized.vertex[0].time.is_some());
        assert!(deserialized.vertex[1].time.is_none());
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
            curvature_dot: Double::literal(0.01),
            length: Double::literal(50.0),
            start_position: Some(Position {
                world_position: Some(WorldPosition::new(1.0, 2.0)),
                ..Position::empty()
            }),
        };
        let xml = quick_xml::se::to_string(&clothoid).unwrap();
        assert!(xml.contains(r#"curvature="0.1""#),    "serialized: {xml}");
        assert!(xml.contains(r#"curvatureDot="0.01""#), "serialized: {xml}");
        assert!(xml.contains(r#"length="50""#),         "serialized: {xml}");
        assert!(xml.contains("Position"),               "serialized: {xml}");

        let deserialized: Clothoid = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(clothoid, deserialized);
    }

    /// Clothoid without a start position must not emit a Position element.
    #[test]
    fn test_clothoid_no_position_not_serialized() {
        let clothoid = Clothoid {
            curvature: Double::literal(0.2),
            curvature_dot: Double::literal(0.0),
            length: Double::literal(10.0),
            start_position: None,
        };
        let xml = quick_xml::se::to_string(&clothoid).unwrap();
        assert!(!xml.contains("<Position"), "Position element must be absent: {xml}");
        let deserialized: Clothoid = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(clothoid, deserialized);
    }
}
