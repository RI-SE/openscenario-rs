//! Trajectory and route-based position types for path following

use crate::types::basic::{Boolean, Double, Int, OSString, UnsignedInt, UnsignedShort};
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

impl Default for Trajectory {
    fn default() -> Self {
        Self {
            name: None,
            closed: None,
            shape: TrajectoryShape::Polyline(Polyline { vertex: Vec::new() }),
        }
    }
}

