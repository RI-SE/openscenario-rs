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
pub mod trajectory;
pub mod world;

pub use relative::RelativeObjectPosition;
pub use road::{
    LaneCoordinate, LanePosition, Orientation, RelativeLanePosition, RelativeRoadPosition,
    RoadCoordinate, RoadPosition,
};
pub use trajectory::{Trajectory, TrajectoryFollowingMode, TrajectoryPosition, TrajectoryRef};
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
    #[serde(rename = "TrajectoryPosition", skip_serializing_if = "Option::is_none")]
    pub trajectory_position: Option<TrajectoryPosition>,
    #[serde(rename = "GeographicPosition", skip_serializing_if = "Option::is_none")]
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
    #[serde(rename = "@dz")]
    pub dz: Double,
}

// Default implementations
impl Default for Position {
    fn default() -> Self {
        Position {
            world_position: Some(WorldPosition::default()),
            relative_world_position: None,
            road_position: None,
            relative_road_position: None,
            lane_position: None,
            relative_lane_position: None,
            trajectory_position: None,
            geographic_position: None,
            relative_object_position: None,
        }
    }
}

impl Default for RelativeWorldPosition {
    fn default() -> Self {
        Self {
            entity_ref: OSString::literal("DefaultEntity".to_string()),
            dx: Double::literal(0.0),
            dy: Double::literal(0.0),
            dz: Double::literal(0.0),
        }
    }
}

// Convenience constructors for Position
impl Position {
    /// Create an empty Position with all fields set to None
    pub fn empty() -> Self {
        Self {
            world_position: None,
            relative_world_position: None,
            road_position: None,
            relative_road_position: None,
            lane_position: None,
            relative_lane_position: None,
            trajectory_position: None,
            geographic_position: None,
            relative_object_position: None,
        }
    }
    /// Create a Position with RelativeRoadPosition
    pub fn relative_road(relative_road_position: RelativeRoadPosition) -> Self {
        Self {
            world_position: None,
            relative_world_position: None,
            road_position: None,
            relative_road_position: Some(relative_road_position),
            lane_position: None,
            relative_lane_position: None,
            trajectory_position: None,
            geographic_position: None,
            relative_object_position: None,
        }
    }

    /// Create a Position with RelativeLanePosition
    pub fn relative_lane(relative_lane_position: RelativeLanePosition) -> Self {
        Self {
            world_position: None,
            relative_world_position: None,
            road_position: None,
            relative_road_position: None,
            lane_position: None,
            relative_lane_position: Some(relative_lane_position),
            trajectory_position: None,
            geographic_position: None,
            relative_object_position: None,
        }
    }

    /// Create a Position with TrajectoryPosition
    pub fn trajectory(trajectory_position: TrajectoryPosition) -> Self {
        Self {
            world_position: None,
            relative_world_position: None,
            road_position: None,
            relative_road_position: None,
            lane_position: None,
            relative_lane_position: None,
            trajectory_position: Some(trajectory_position),
            geographic_position: None,
            relative_object_position: None,
        }
    }

    /// Create a Position with GeographicPosition
    pub fn geographic(geographic_position: GeographicPosition) -> Self {
        Self {
            world_position: None,
            relative_world_position: None,
            road_position: None,
            relative_road_position: None,
            lane_position: None,
            relative_lane_position: None,
            trajectory_position: None,
            geographic_position: Some(geographic_position),
            relative_object_position: None,
        }
    }

    /// Create a Position with RelativeObjectPosition
    pub fn relative_object(relative_object_position: RelativeObjectPosition) -> Self {
        Self {
            world_position: None,
            relative_world_position: None,
            road_position: None,
            relative_road_position: None,
            lane_position: None,
            relative_lane_position: None,
            trajectory_position: None,
            geographic_position: None,
            relative_object_position: Some(relative_object_position),
        }
    }
}
