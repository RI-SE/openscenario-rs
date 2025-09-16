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
#[serde(rename_all = "camelCase")]
pub struct Vertex {
    /// Time at this vertex (optional)
    pub time: Option<Double>,
    /// Position at this vertex
    pub position: crate::types::positions::Position,
}

/// Clothoid trajectory segment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clothoid {
    /// Curvature at start
    pub curvature: Double,
    /// Curvature derivative (clothoid parameter)
    pub curvature_dot: Double,
    /// Length of the clothoid
    pub length: Double,
    /// Start position
    pub start_position: Option<crate::types::positions::Position>,
}

/// Trajectory following mode
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TrajectoryFollowingMode {
    /// Follow trajectory position exactly
    Position,
    /// Follow trajectory timing
    Timing,
}

/// Reference to a trajectory in a catalog
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrajectoryRef {
    /// Name/ID of the trajectory
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
    pub fn with_orientation(mut self, orientation: crate::types::positions::road::Orientation) -> Self {
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
