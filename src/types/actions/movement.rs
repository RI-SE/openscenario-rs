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
use crate::types::catalogs::references::{CatalogReference, ParameterAssignment};
use crate::types::catalogs::entities::{CatalogTrajectory, CatalogRoute};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[derive(Default)]
pub struct SpeedAction { 
    #[serde(rename = "SpeedActionDynamics")]
    pub speed_action_dynamics: TransitionDynamics, 
    #[serde(rename = "SpeedActionTarget")]
    pub speed_action_target: SpeedActionTarget 
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[derive(Default)]
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

/// Trajectory reference wrapper - can contain direct trajectory or catalog reference
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrajectoryRef {
    /// Direct trajectory definition
    #[serde(rename = "Trajectory", skip_serializing_if = "Option::is_none")]
    pub trajectory: Option<Trajectory>,
    
    /// Reference to a trajectory in a catalog
    #[serde(rename = "CatalogReference", skip_serializing_if = "Option::is_none")]
    pub catalog_reference: Option<CatalogReference<CatalogTrajectory>>,
}

/// Trajectory following mode specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrajectoryFollowingMode {
    #[serde(rename = "@followingMode")]
    pub following_mode: FollowingMode,
}

/// Follow trajectory action with trajectory reference support
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FollowTrajectoryAction {
    /// Direct trajectory definition (for backward compatibility)
    #[serde(rename = "Trajectory", skip_serializing_if = "Option::is_none")]
    pub trajectory: Option<Trajectory>,
    
    /// Reference to a trajectory in a catalog
    #[serde(rename = "CatalogReference", skip_serializing_if = "Option::is_none")]
    pub catalog_reference: Option<CatalogReference<CatalogTrajectory>>,
    
    #[serde(rename = "TrajectoryFollowingMode")]
    pub trajectory_following_mode: TrajectoryFollowingMode,
}

impl Default for FollowTrajectoryAction {
    fn default() -> Self {
        Self {
            trajectory: Some(Trajectory::default()),
            catalog_reference: None,
            trajectory_following_mode: TrajectoryFollowingMode::default(),
        }
    }
}

/// Route definition with waypoints (basic implementation)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Route {
    #[serde(rename = "@name")]
    pub name: OSString,
    #[serde(rename = "@closed")]
    pub closed: Boolean,
    // TODO: Add Waypoints in future implementation
}

/// Route reference wrapper - can contain direct route or catalog reference
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RouteRef {
    /// Direct route definition
    #[serde(rename = "Route", skip_serializing_if = "Option::is_none")]
    pub route: Option<Route>,
    
    /// Reference to a route in a catalog
    #[serde(rename = "CatalogReference", skip_serializing_if = "Option::is_none")]
    pub catalog_reference: Option<CatalogReference<CatalogRoute>>,
}

/// Follow route action with route reference support
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FollowRouteAction {
    /// Route reference (direct or catalog-based)
    #[serde(flatten)]
    pub route_ref: RouteRef,
}

/// Routing action container for trajectory and route-based movement
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoutingAction {
    /// Follow trajectory action
    #[serde(rename = "FollowTrajectoryAction", skip_serializing_if = "Option::is_none")]
    pub follow_trajectory_action: Option<FollowTrajectoryAction>,
    
    /// Follow route action
    #[serde(rename = "FollowRouteAction", skip_serializing_if = "Option::is_none")]
    pub follow_route_action: Option<FollowRouteAction>,
}

impl Default for RoutingAction {
    fn default() -> Self {
        Self {
            follow_trajectory_action: None,
            follow_route_action: None,
        }
    }
}

// Add more movement actions later (Week 10+) - KEEP AS FUTURE WORK
// pub struct LaneChangeAction - lane change maneuvers
// pub struct SynchronizeAction - entity coordination

// Default implementations


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

impl Default for TrajectoryRef {
    fn default() -> Self {
        Self {
            trajectory: Some(Trajectory::default()),
            catalog_reference: None,
        }
    }
}

impl Default for Route {
    fn default() -> Self {
        Self {
            name: OSString::literal("DefaultRoute".to_string()),
            closed: Boolean::literal(false),
        }
    }
}

impl Default for RouteRef {
    fn default() -> Self {
        Self {
            route: Some(Route::default()),
            catalog_reference: None,
        }
    }
}

impl Default for FollowRouteAction {
    fn default() -> Self {
        Self {
            route_ref: RouteRef::default(),
        }
    }
}



// Helper implementations for catalog-based actions

impl TrajectoryRef {
    /// Create a trajectory reference with direct trajectory definition
    pub fn with_trajectory(trajectory: Trajectory) -> Self {
        Self {
            trajectory: Some(trajectory),
            catalog_reference: None,
        }
    }
    
    /// Create a trajectory reference with catalog reference
    pub fn with_catalog_reference(catalog_reference: CatalogReference<CatalogTrajectory>) -> Self {
        Self {
            trajectory: None,
            catalog_reference: Some(catalog_reference),
        }
    }
    
    /// Create a trajectory reference from catalog name and entry name
    pub fn from_catalog(catalog_name: String, entry_name: String) -> Self {
        Self::with_catalog_reference(CatalogReference::new(catalog_name, entry_name))
    }
    
    /// Create a trajectory reference from catalog with parameters
    pub fn from_catalog_with_parameters(
        catalog_name: String, 
        entry_name: String, 
        parameters: Vec<ParameterAssignment>
    ) -> Self {
        Self::with_catalog_reference(CatalogReference::with_parameters(catalog_name, entry_name, parameters))
    }
}

impl RouteRef {
    /// Create a route reference with direct route definition
    pub fn with_route(route: Route) -> Self {
        Self {
            route: Some(route),
            catalog_reference: None,
        }
    }
    
    /// Create a route reference with catalog reference
    pub fn with_catalog_reference(catalog_reference: CatalogReference<CatalogRoute>) -> Self {
        Self {
            route: None,
            catalog_reference: Some(catalog_reference),
        }
    }
    
    /// Create a route reference from catalog name and entry name
    pub fn from_catalog(catalog_name: String, entry_name: String) -> Self {
        Self::with_catalog_reference(CatalogReference::new(catalog_name, entry_name))
    }
    
    /// Create a route reference from catalog with parameters
    pub fn from_catalog_with_parameters(
        catalog_name: String, 
        entry_name: String, 
        parameters: Vec<ParameterAssignment>
    ) -> Self {
        Self::with_catalog_reference(CatalogReference::with_parameters(catalog_name, entry_name, parameters))
    }
}

impl FollowTrajectoryAction {
    /// Create a follow trajectory action with direct trajectory
    pub fn with_trajectory(trajectory: Trajectory, following_mode: FollowingMode) -> Self {
        Self {
            trajectory: Some(trajectory),
            catalog_reference: None,
            trajectory_following_mode: TrajectoryFollowingMode { following_mode },
        }
    }
    
    /// Create a follow trajectory action with catalog reference
    pub fn with_catalog_reference(
        catalog_reference: CatalogReference<CatalogTrajectory>,
        following_mode: FollowingMode,
    ) -> Self {
        Self {
            trajectory: None,
            catalog_reference: Some(catalog_reference),
            trajectory_following_mode: TrajectoryFollowingMode { following_mode },
        }
    }
    
    /// Create a follow trajectory action from catalog name and entry name
    pub fn from_catalog(
        catalog_name: String, 
        entry_name: String,
        following_mode: FollowingMode,
    ) -> Self {
        Self::with_catalog_reference(
            CatalogReference::new(catalog_name, entry_name),
            following_mode
        )
    }
}

impl FollowRouteAction {
    /// Create a follow route action with direct route
    pub fn with_route(route: Route) -> Self {
        Self {
            route_ref: RouteRef::with_route(route),
        }
    }
    
    /// Create a follow route action with catalog reference
    pub fn with_catalog_reference(catalog_reference: CatalogReference<CatalogRoute>) -> Self {
        Self {
            route_ref: RouteRef::with_catalog_reference(catalog_reference),
        }
    }
    
    /// Create a follow route action from catalog name and entry name
    pub fn from_catalog(catalog_name: String, entry_name: String) -> Self {
        Self {
            route_ref: RouteRef::from_catalog(catalog_name, entry_name),
        }
    }
}

impl RoutingAction {
    /// Create a routing action with trajectory following
    pub fn with_trajectory(action: FollowTrajectoryAction) -> Self {
        Self {
            follow_trajectory_action: Some(action),
            follow_route_action: None,
        }
    }
    
    /// Create a routing action with route following
    pub fn with_route(action: FollowRouteAction) -> Self {
        Self {
            follow_trajectory_action: None,
            follow_route_action: Some(action),
        }
    }
    
    /// Create a routing action with trajectory from catalog
    pub fn with_trajectory_from_catalog(
        catalog_name: String,
        entry_name: String,
        following_mode: FollowingMode,
    ) -> Self {
        Self::with_trajectory(FollowTrajectoryAction::from_catalog(
            catalog_name, entry_name, following_mode
        ))
    }
    
    /// Create a routing action with route from catalog
    pub fn with_route_from_catalog(catalog_name: String, entry_name: String) -> Self {
        Self::with_route(FollowRouteAction::from_catalog(catalog_name, entry_name))
    }
}

// Add movement action validation
// impl ValidateAction for SpeedAction, TeleportAction