//! Trajectory and route-based position types for path following

use crate::types::basic::{Double, OSString, ParameterDeclarations};
use crate::types::geometry::shapes::Shape;
use serde::{Deserialize, Serialize};

/// Trajectory definition with shape and parameters
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trajectory {
    /// Name of the trajectory — XSD attribute `name`, `use="required"`
    #[serde(rename = "@name")]
    pub name: OSString,
    /// Whether the trajectory is closed (forms a loop) — XSD attribute `closed`, `use="required"`
    #[serde(rename = "@closed")]
    pub closed: bool,
    /// Parameter declarations for this trajectory — XSD child element
    /// `<ParameterDeclarations>`, `minOccurs="0"`, precedes `<Shape>`.
    #[serde(
        rename = "ParameterDeclarations",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_declarations: Option<ParameterDeclarations>,
    /// Shape definition of the trajectory — XSD child element `<Shape>`, a choice of
    /// Polyline | Clothoid | ClothoidSpline | Nurbs. See `geometry::shapes::Shape`.
    #[serde(rename = "Shape")]
    pub shape: Shape,
}

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

impl Default for Trajectory {
    fn default() -> Self {
        Self {
            name: OSString::literal(String::new()),
            closed: false,
            parameter_declarations: None,
            shape: Shape {
                polyline: Some(crate::types::geometry::shapes::Polyline {
                    vertices: Vec::new(),
                }),
                clothoid: None,
                clothoid_spline: None,
                nurbs: None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trajectory_position_new() {
        let pos = TrajectoryPosition::new(50.0, TrajectoryRef::default());
        assert_eq!(pos.s.as_literal().unwrap(), &50.0);
        assert!(pos.t.is_none());
        assert!(pos.orientation.is_none());
    }

    #[test]
    fn test_trajectory_position_with_offset() {
        let pos = TrajectoryPosition::with_offset(100.0, -1.5, TrajectoryRef::default());
        assert_eq!(pos.s.as_literal().unwrap(), &100.0);
        assert_eq!(pos.t.unwrap().as_literal().unwrap(), &-1.5);
    }

    #[test]
    fn test_trajectory_position_xml_roundtrip() {
        let pos = TrajectoryPosition::new(25.0, TrajectoryRef::default());
        let xml = quick_xml::se::to_string(&pos).unwrap();
        assert!(xml.contains("s=\"25\""));
        let deserialized: TrajectoryPosition = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(pos, deserialized);
    }

    #[test]
    fn test_trajectory_default_is_empty_polyline() {
        let traj = Trajectory::default();
        assert_eq!(traj.name.as_literal(), Some(&String::new()));
        assert!(!traj.closed);
        match &traj.shape.polyline {
            Some(p) => assert!(p.vertices.is_empty()),
            None => panic!("Expected Polyline shape"),
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
        use crate::types::geometry::shapes::Vertex;
        let xml = r#"<Vertex><Position><WorldPosition x="1" y="2"/></Position></Vertex>"#;
        let v: Vertex = quick_xml::de::from_str(xml).unwrap();
        assert!(v.time.is_none());
        assert!(v.position.world_position.is_some());
    }

    /// A Vertex element with a `time` attribute must round-trip correctly.
    #[test]
    fn test_vertex_xml_roundtrip_with_time() {
        use crate::types::basic::Value;
        use crate::types::geometry::shapes::Vertex;
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
        use crate::types::geometry::shapes::Vertex;
        use crate::types::positions::{Position, WorldPosition};

        let vertex = Vertex {
            time: None,
            position: Position {
                world_position: Some(WorldPosition::new(0.0, 0.0)),
                ..Position::empty()
            },
        };
        let xml = quick_xml::se::to_string(&vertex).unwrap();
        assert!(!xml.contains("time="), "time attr must be absent: {xml}");
    }

    /// Polyline with one time-bearing and one time-less vertex must round-trip.
    #[test]
    fn test_polyline_xml_roundtrip_mixed_time() {
        use crate::types::geometry::shapes::{Polyline, Vertex};
        use crate::types::positions::{Position, WorldPosition};

        let polyline = Polyline {
            vertices: vec![
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
        assert_eq!(deserialized.vertices.len(), 2);
        assert!(deserialized.vertices[0].time.is_some());
        assert!(deserialized.vertices[1].time.is_none());
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
            start_position: Position {
                world_position: Some(WorldPosition::new(1.0, 2.0)),
                ..Position::empty()
            },
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
    fn test_clothoid_no_position_not_serialized() {
        use crate::types::positions::{Position, WorldPosition};

        let clothoid = Clothoid {
            curvature: Double::literal(0.2),
            curvature_dot: None,
            curvature_prime: None,
            length: Double::literal(10.0),
            start_time: None,
            stop_time: None,
            start_position: Position {
                world_position: Some(WorldPosition::new(0.0, 0.0)),
                ..Position::empty()
            },
        };
        let xml = quick_xml::se::to_string(&clothoid).unwrap();
        assert!(
            !xml.contains("curvatureDot"),
            "curvatureDot attr must be absent: {xml}"
        );
        let deserialized: Clothoid = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(clothoid, deserialized);
    }

    /// `Trajectory` in `positions::trajectory` must round-trip its optional
    /// `<ParameterDeclarations>` element (XSD Trajectory :2356-2363).
    #[test]
    fn test_trajectory_parameter_declarations_round_trip() {
        let xml = r#"<Trajectory name="Traj1" closed="false">
    <ParameterDeclarations>
        <ParameterDeclaration name="speed" parameterType="double" value="10.0"/>
    </ParameterDeclarations>
    <Shape>
        <Polyline>
            <Vertex><Position><WorldPosition x="0" y="0"/></Position></Vertex>
        </Polyline>
    </Shape>
</Trajectory>"#;
        let trajectory: Trajectory = quick_xml::de::from_str(xml).unwrap();
        let decls = trajectory
            .parameter_declarations
            .as_ref()
            .expect("ParameterDeclarations must be present");
        assert_eq!(decls.parameter_declarations.len(), 1);

        let serialized = quick_xml::se::to_string(&trajectory).unwrap();
        assert!(
            serialized.contains("<ParameterDeclarations>"),
            "serialized: {serialized}"
        );
        let reparsed: Trajectory = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(trajectory, reparsed);
    }
}
