//! Movement action types for entity positioning and motion
//!
//! This file contains:
//! - Movement actions (SpeedAction, LaneChangeAction, TeleportAction, etc.)
//! - Dynamics and transition specifications for smooth movement
//! - Target specification types (absolute, relative, following)
//! - Spatial relationship actions (distance keeping, synchronization)
//! - Trajectory following and path planning actions
//!
//! Contributes to project by:
//! - Implementing the core movement capabilities for scenario entities
//! - Providing smooth, realistic motion through transition dynamics
//! - Supporting both simple positioning and complex trajectory following
//! - Enabling entity coordination through synchronization actions
//! - Offering flexible target specification (absolute vs. relative positioning)

use crate::types::{Double};
use crate::types::basic::{OSString, Boolean};
use crate::types::enums::{DynamicsDimension, DynamicsShape, SpeedTargetValueType, FollowingMode};
use crate::types::positions::Position;
use crate::types::geometry::shapes::Shape;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpeedAction { 
    #[serde(rename = "SpeedActionDynamics")]
    pub speed_action_dynamics: TransitionDynamics, 
    #[serde(rename = "SpeedActionTarget")]
    pub speed_action_target: SpeedActionTarget 
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TeleportAction { 
    #[serde(rename = "Position")]
    pub position: Position 
}

// Remove duplicate import

// Define supporting types for SpeedAction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransitionDynamics {
    #[serde(rename = "@dynamicsDimension")]
    pub dynamics_dimension: DynamicsDimension,
    #[serde(rename = "@dynamicsShape")]
    pub dynamics_shape: DynamicsShape,
    #[serde(rename = "@value")]
    pub value: Double,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpeedActionTarget {
    #[serde(rename = "AbsoluteTargetSpeed", skip_serializing_if = "Option::is_none")]
    pub absolute: Option<AbsoluteTargetSpeed>,
    #[serde(rename = "RelativeTargetSpeed", skip_serializing_if = "Option::is_none")]
    pub relative: Option<RelativeTargetSpeed>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AbsoluteTargetSpeed { 
    #[serde(rename = "@value")]
    pub value: Double 
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelativeTargetSpeed {
    #[serde(rename = "@value")]
    pub value: Double,
    #[serde(rename = "@entityRef")]
    pub entity_ref: String,
    #[serde(rename = "@valueType")]
    pub value_type: SpeedTargetValueType,
    #[serde(rename = "@continuous")]
    pub continuous: bool,
}

/// Complete trajectory definition with shape and metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Trajectory {
    #[serde(rename = "@name")]
    pub name: OSString,
    #[serde(rename = "@closed")]
    pub closed: Boolean,
    #[serde(rename = "Shape")]
    pub shape: Shape,
}

/// Trajectory following mode specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrajectoryFollowingMode {
    #[serde(rename = "@followingMode")]
    pub following_mode: FollowingMode,
}

/// Follow trajectory action containing complete trajectory definition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FollowTrajectoryAction {
    #[serde(rename = "Trajectory")]
    pub trajectory: Trajectory,
    #[serde(rename = "TrajectoryFollowingMode")]
    pub trajectory_following_mode: TrajectoryFollowingMode,
}

/// Routing action container for trajectory-based movement
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoutingAction {
    #[serde(rename = "FollowTrajectoryAction")]
    pub follow_trajectory_action: FollowTrajectoryAction,
}

// Add more movement actions later (Week 10+) - KEEP AS FUTURE WORK
// pub struct LaneChangeAction - lane change maneuvers
// pub struct SynchronizeAction - entity coordination

// Default implementations
impl Default for SpeedAction {
    fn default() -> Self {
        Self {
            speed_action_dynamics: TransitionDynamics::default(),
            speed_action_target: SpeedActionTarget::default(),
        }
    }
}

impl Default for TeleportAction {
    fn default() -> Self {
        Self {
            position: Position::default(),
        }
    }
}

impl Default for TransitionDynamics {
    fn default() -> Self {
        Self {
            dynamics_dimension: DynamicsDimension::Time,
            dynamics_shape: DynamicsShape::Linear,
            value: Double::literal(1.0),
        }
    }
}

impl Default for SpeedActionTarget {
    fn default() -> Self {
        Self {
            absolute: Some(AbsoluteTargetSpeed::default()),
            relative: None,
        }
    }
}

impl Default for AbsoluteTargetSpeed {
    fn default() -> Self {
        Self {
            value: Double::literal(10.0),
        }
    }
}

impl Default for RelativeTargetSpeed {
    fn default() -> Self {
        Self {
            value: Double::literal(0.0),
            entity_ref: "DefaultEntity".to_string(),
            value_type: SpeedTargetValueType::Delta,
            continuous: false,
        }
    }
}

impl Default for Trajectory {
    fn default() -> Self {
        Self {
            name: OSString::literal("DefaultTrajectory".to_string()),
            closed: Boolean::literal(false),
            shape: Shape::default(),
        }
    }
}

impl Default for TrajectoryFollowingMode {
    fn default() -> Self {
        Self {
            following_mode: FollowingMode::Follow,
        }
    }
}

impl Default for FollowTrajectoryAction {
    fn default() -> Self {
        Self {
            trajectory: Trajectory::default(),
            trajectory_following_mode: TrajectoryFollowingMode::default(),
        }
    }
}

impl Default for RoutingAction {
    fn default() -> Self {
        Self {
            follow_trajectory_action: FollowTrajectoryAction::default(),
        }
    }
}

// Add movement action validation
// impl ValidateAction for SpeedAction, TeleportAction