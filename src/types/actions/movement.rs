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

// PHASE 4A: Core Movement Actions - Implementation

/// Lane change action for lateral lane movements
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LaneChangeAction {
    #[serde(rename = "@targetLaneOffset")]
    pub target_lane_offset: Option<f64>,
    #[serde(rename = "LaneChangeActionDynamics")]
    pub lane_change_action_dynamics: TransitionDynamics,
    #[serde(rename = "LaneChangeTarget")]
    pub lane_change_target: LaneChangeTarget,
}

/// Lane change target specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LaneChangeTarget {
    #[serde(flatten)]
    pub target_choice: LaneChangeTargetChoice,
}

/// Lane change target choice - relative or absolute
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum LaneChangeTargetChoice {
    RelativeTargetLane(RelativeTargetLane),
    AbsoluteTargetLane(AbsoluteTargetLane),
}

/// Relative target lane specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelativeTargetLane {
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,
    #[serde(rename = "@value")]
    pub value: i32,
}

/// Absolute target lane specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AbsoluteTargetLane {
    #[serde(rename = "@value")]
    pub value: i32,
}

/// Lane offset action for lateral positioning within a lane
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LaneOffsetAction {
    #[serde(rename = "@continuous")]
    pub continuous: Boolean,
    #[serde(rename = "LaneOffsetActionDynamics")]
    pub lane_offset_action_dynamics: TransitionDynamics,
    #[serde(rename = "LaneOffsetTarget")]
    pub lane_offset_target: LaneOffsetTarget,
}

/// Lane offset target specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LaneOffsetTarget {
    #[serde(flatten)]
    pub target_choice: LaneOffsetTargetChoice,
}

/// Lane offset target choice - relative or absolute
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum LaneOffsetTargetChoice {
    RelativeTargetLaneOffset(RelativeTargetLaneOffset),
    AbsoluteTargetLaneOffset(AbsoluteTargetLaneOffset),
}

/// Relative target lane offset specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelativeTargetLaneOffset {
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,
    #[serde(rename = "@value")]
    pub value: f64,
}

/// Absolute target lane offset specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AbsoluteTargetLaneOffset {
    #[serde(rename = "@value")]
    pub value: f64,
}

/// Lateral action wrapper for all lateral movement types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LateralAction {
    #[serde(flatten)]
    pub lateral_action_choice: LateralActionChoice,
}

/// Lateral action choice - lane change, offset, or distance keeping
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum LateralActionChoice {
    LaneChangeAction(LaneChangeAction),
    LaneOffsetAction(LaneOffsetAction),
    LateralDistanceAction(LateralDistanceAction),
}

/// Lateral distance action for maintaining lateral distance to an entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LateralDistanceAction {
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,
    #[serde(rename = "@distance")]
    pub distance: f64,
    #[serde(rename = "@freespace")]
    pub freespace: Option<Boolean>,
    #[serde(rename = "@continuous")]
    pub continuous: Boolean,
    #[serde(rename = "DynamicConstraints")]
    pub dynamic_constraints: Option<DynamicConstraints>,
}

/// Longitudinal action wrapper for all longitudinal movement types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LongitudinalAction {
    #[serde(flatten)]
    pub longitudinal_action_choice: LongitudinalActionChoice,
}

/// Longitudinal action choice - speed, distance, or speed profile
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum LongitudinalActionChoice {
    SpeedAction(SpeedAction),
    LongitudinalDistanceAction(LongitudinalDistanceAction),
    SpeedProfileAction(SpeedProfileAction),
}

/// Longitudinal distance action for maintaining longitudinal distance to an entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LongitudinalDistanceAction {
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,
    #[serde(rename = "@distance")]
    pub distance: f64,
    #[serde(rename = "@freespace")]
    pub freespace: Option<Boolean>,
    #[serde(rename = "@continuous")]
    pub continuous: Boolean,
    #[serde(rename = "DynamicConstraints")]
    pub dynamic_constraints: Option<DynamicConstraints>,
}

/// Speed profile action for time-based speed control
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpeedProfileAction {
    #[serde(rename = "@entityRef")]
    pub entity_ref: Option<OSString>,
    #[serde(rename = "DynamicConstraints")]
    pub dynamic_constraints: Option<DynamicConstraints>,
    #[serde(rename = "Entry", default)]
    pub entries: Vec<SpeedProfileEntry>,
}

/// Speed profile entry with time and speed
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpeedProfileEntry {
    #[serde(rename = "@time")]
    pub time: f64,
    #[serde(rename = "@speed")]
    pub speed: f64,
}

/// Dynamic constraints for movement actions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DynamicConstraints {
    #[serde(rename = "@maxLateralAcc")]
    pub max_lateral_acc: Option<f64>,
    #[serde(rename = "@maxSpeed")]
    pub max_speed: Option<f64>,
}

/// Synchronize action for coordinated entity movement
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SynchronizeAction {
    #[serde(rename = "@targetEntityRef")]
    pub target_entity_ref: OSString,
    #[serde(rename = "TargetPositionMaster")]
    pub target_position_master: Position,
    #[serde(rename = "TargetPosition")]
    pub target_position: Position,
    #[serde(rename = "FinalSpeed")]
    pub final_speed: Option<FinalSpeed>,
}

/// Final speed specification for synchronize action
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FinalSpeed {
    #[serde(flatten)]
    pub speed_choice: FinalSpeedChoice,
}

/// Final speed choice - absolute or relative
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum FinalSpeedChoice {
    AbsoluteSpeed(AbsoluteSpeed),
    RelativeSpeedToMaster(RelativeSpeedToMaster),
}

/// Absolute speed specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AbsoluteSpeed {
    #[serde(rename = "@value")]
    pub value: f64,
}

/// Relative speed to master specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelativeSpeedToMaster {
    #[serde(rename = "@value")]
    pub value: f64,
}

/// Acquire position action for moving to a specific position
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AcquirePositionAction {
    #[serde(rename = "Position")]
    pub position: Position,
}

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

// PHASE 4A: Default implementations for new movement actions

impl Default for LaneChangeAction {
    fn default() -> Self {
        Self {
            target_lane_offset: None,
            lane_change_action_dynamics: TransitionDynamics::default(),
            lane_change_target: LaneChangeTarget::default(),
        }
    }
}

impl Default for LaneChangeTarget {
    fn default() -> Self {
        Self {
            target_choice: LaneChangeTargetChoice::RelativeTargetLane(RelativeTargetLane::default()),
        }
    }
}

impl Default for RelativeTargetLane {
    fn default() -> Self {
        Self {
            entity_ref: OSString::literal("DefaultEntity".to_string()),
            value: 1,
        }
    }
}

impl Default for AbsoluteTargetLane {
    fn default() -> Self {
        Self {
            value: 1,
        }
    }
}

impl Default for LaneOffsetAction {
    fn default() -> Self {
        Self {
            continuous: Boolean::literal(false),
            lane_offset_action_dynamics: TransitionDynamics::default(),
            lane_offset_target: LaneOffsetTarget::default(),
        }
    }
}

impl Default for LaneOffsetTarget {
    fn default() -> Self {
        Self {
            target_choice: LaneOffsetTargetChoice::AbsoluteTargetLaneOffset(AbsoluteTargetLaneOffset::default()),
        }
    }
}

impl Default for RelativeTargetLaneOffset {
    fn default() -> Self {
        Self {
            entity_ref: OSString::literal("DefaultEntity".to_string()),
            value: 0.0,
        }
    }
}

impl Default for AbsoluteTargetLaneOffset {
    fn default() -> Self {
        Self {
            value: 0.0,
        }
    }
}

impl Default for LateralAction {
    fn default() -> Self {
        Self {
            lateral_action_choice: LateralActionChoice::LaneChangeAction(LaneChangeAction::default()),
        }
    }
}

impl Default for LateralDistanceAction {
    fn default() -> Self {
        Self {
            entity_ref: OSString::literal("DefaultEntity".to_string()),
            distance: 2.0,
            freespace: Some(Boolean::literal(true)),
            continuous: Boolean::literal(false),
            dynamic_constraints: None,
        }
    }
}

impl Default for LongitudinalAction {
    fn default() -> Self {
        Self {
            longitudinal_action_choice: LongitudinalActionChoice::SpeedAction(SpeedAction::default()),
        }
    }
}

impl Default for LongitudinalDistanceAction {
    fn default() -> Self {
        Self {
            entity_ref: OSString::literal("DefaultEntity".to_string()),
            distance: 10.0,
            freespace: Some(Boolean::literal(true)),
            continuous: Boolean::literal(false),
            dynamic_constraints: None,
        }
    }
}

impl Default for SpeedProfileAction {
    fn default() -> Self {
        Self {
            entity_ref: None,
            dynamic_constraints: None,
            entries: vec![SpeedProfileEntry::default()],
        }
    }
}

impl Default for SpeedProfileEntry {
    fn default() -> Self {
        Self {
            time: 0.0,
            speed: 10.0,
        }
    }
}

impl Default for DynamicConstraints {
    fn default() -> Self {
        Self {
            max_lateral_acc: None,
            max_speed: None,
        }
    }
}

impl Default for SynchronizeAction {
    fn default() -> Self {
        Self {
            target_entity_ref: OSString::literal("DefaultEntity".to_string()),
            target_position_master: Position::default(),
            target_position: Position::default(),
            final_speed: None,
        }
    }
}

impl Default for FinalSpeed {
    fn default() -> Self {
        Self {
            speed_choice: FinalSpeedChoice::AbsoluteSpeed(AbsoluteSpeed::default()),
        }
    }
}

impl Default for AbsoluteSpeed {
    fn default() -> Self {
        Self {
            value: 10.0,
        }
    }
}

impl Default for RelativeSpeedToMaster {
    fn default() -> Self {
        Self {
            value: 0.0,
        }
    }
}

impl Default for AcquirePositionAction {
    fn default() -> Self {
        Self {
            position: Position::default(),
        }
    }
}

// PHASE 4A: Unit tests for new movement actions
#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::positions::Position;
    use crate::types::enums::{DynamicsDimension, DynamicsShape};

    #[test]
    fn test_lane_change_action_creation() {
        let action = LaneChangeAction::default();
        assert!(action.target_lane_offset.is_none());
        assert_eq!(action.lane_change_action_dynamics.dynamics_dimension, DynamicsDimension::Time);
        assert_eq!(action.lane_change_action_dynamics.dynamics_shape, DynamicsShape::Linear);
    }

    #[test]
    fn test_lane_change_target_relative() {
        let relative_target = RelativeTargetLane {
            entity_ref: OSString::literal("TestEntity".to_string()),
            value: 2,
        };
        let target = LaneChangeTarget {
            target_choice: LaneChangeTargetChoice::RelativeTargetLane(relative_target),
        };
        
        if let LaneChangeTargetChoice::RelativeTargetLane(rel) = target.target_choice {
            assert_eq!(rel.entity_ref.as_literal(), Some(&"TestEntity".to_string()));
            assert_eq!(rel.value, 2);
        } else {
            panic!("Expected RelativeTargetLane");
        }
    }

    #[test]
    fn test_lane_offset_action_creation() {
        let action = LaneOffsetAction::default();
        assert_eq!(action.continuous.as_literal(), Some(&false));
        assert_eq!(action.lane_offset_action_dynamics.dynamics_dimension, DynamicsDimension::Time);
    }

    #[test]
    fn test_lateral_distance_action_creation() {
        let action = LateralDistanceAction {
            entity_ref: OSString::literal("TargetEntity".to_string()),
            distance: 3.5,
            freespace: Some(Boolean::literal(true)),
            continuous: Boolean::literal(false),
            dynamic_constraints: Some(DynamicConstraints {
                max_lateral_acc: Some(2.0),
                max_speed: Some(50.0),
            }),
        };
        
        assert_eq!(action.entity_ref.as_literal(), Some(&"TargetEntity".to_string()));
        assert_eq!(action.distance, 3.5);
        assert_eq!(action.freespace.unwrap().as_literal(), Some(&true));
        assert_eq!(action.continuous.as_literal(), Some(&false));
        
        let constraints = action.dynamic_constraints.unwrap();
        assert_eq!(constraints.max_lateral_acc, Some(2.0));
        assert_eq!(constraints.max_speed, Some(50.0));
    }

    #[test]
    fn test_longitudinal_action_choices() {
        // Test with SpeedAction
        let speed_action = LongitudinalAction {
            longitudinal_action_choice: LongitudinalActionChoice::SpeedAction(SpeedAction::default()),
        };
        
        if let LongitudinalActionChoice::SpeedAction(_) = speed_action.longitudinal_action_choice {
            // Expected
        } else {
            panic!("Expected SpeedAction");
        }
        
        // Test with LongitudinalDistanceAction
        let distance_action = LongitudinalAction {
            longitudinal_action_choice: LongitudinalActionChoice::LongitudinalDistanceAction(
                LongitudinalDistanceAction::default()
            ),
        };
        
        if let LongitudinalActionChoice::LongitudinalDistanceAction(dist) = distance_action.longitudinal_action_choice {
            assert_eq!(dist.distance, 10.0);
        } else {
            panic!("Expected LongitudinalDistanceAction");
        }
    }

    #[test]
    fn test_speed_profile_action_creation() {
        let entry1 = SpeedProfileEntry { time: 0.0, speed: 10.0 };
        let entry2 = SpeedProfileEntry { time: 5.0, speed: 20.0 };
        
        let action = SpeedProfileAction {
            entity_ref: Some(OSString::literal("RefEntity".to_string())),
            dynamic_constraints: Some(DynamicConstraints {
                max_lateral_acc: Some(1.5),
                max_speed: Some(30.0),
            }),
            entries: vec![entry1, entry2],
        };
        
        assert_eq!(action.entity_ref.unwrap().as_literal(), Some(&"RefEntity".to_string()));
        assert_eq!(action.entries.len(), 2);
        assert_eq!(action.entries[0].time, 0.0);
        assert_eq!(action.entries[0].speed, 10.0);
        assert_eq!(action.entries[1].time, 5.0);
        assert_eq!(action.entries[1].speed, 20.0);
    }

    #[test]
    fn test_synchronize_action_creation() {
        let action = SynchronizeAction {
            target_entity_ref: OSString::literal("SyncTarget".to_string()),
            target_position_master: Position::default(),
            target_position: Position::default(),
            final_speed: Some(FinalSpeed {
                speed_choice: FinalSpeedChoice::AbsoluteSpeed(AbsoluteSpeed { value: 15.0 }),
            }),
        };
        
        assert_eq!(action.target_entity_ref.as_literal(), Some(&"SyncTarget".to_string()));
        
        if let Some(final_speed) = action.final_speed {
            if let FinalSpeedChoice::AbsoluteSpeed(abs_speed) = final_speed.speed_choice {
                assert_eq!(abs_speed.value, 15.0);
            } else {
                panic!("Expected AbsoluteSpeed");
            }
        }
    }

    #[test]
    fn test_acquire_position_action_creation() {
        let action = AcquirePositionAction {
            position: Position::default(),
        };
        
        // Just ensure it compiles and has the expected structure
        assert_eq!(std::mem::size_of_val(&action.position), std::mem::size_of::<Position>());
    }

    #[test]
    fn test_dynamic_constraints_creation() {
        let constraints = DynamicConstraints {
            max_lateral_acc: Some(3.0),
            max_speed: Some(80.0),
        };
        
        assert_eq!(constraints.max_lateral_acc, Some(3.0));
        assert_eq!(constraints.max_speed, Some(80.0));
        
        let empty_constraints = DynamicConstraints::default();
        assert!(empty_constraints.max_lateral_acc.is_none());
        assert!(empty_constraints.max_speed.is_none());
    }

    #[test]
    fn test_final_speed_choices() {
        // Test absolute speed
        let abs_final = FinalSpeed {
            speed_choice: FinalSpeedChoice::AbsoluteSpeed(AbsoluteSpeed { value: 25.0 }),
        };
        
        if let FinalSpeedChoice::AbsoluteSpeed(abs) = abs_final.speed_choice {
            assert_eq!(abs.value, 25.0);
        }
        
        // Test relative speed to master
        let rel_final = FinalSpeed {
            speed_choice: FinalSpeedChoice::RelativeSpeedToMaster(RelativeSpeedToMaster { value: -5.0 }),
        };
        
        if let FinalSpeedChoice::RelativeSpeedToMaster(rel) = rel_final.speed_choice {
            assert_eq!(rel.value, -5.0);
        }
    }

    #[test]
    fn test_action_defaults() {
        // Test that all new action types have working defaults
        let lane_change = LaneChangeAction::default();
        assert!(lane_change.target_lane_offset.is_none());
        
        let lane_offset = LaneOffsetAction::default();
        assert_eq!(lane_offset.continuous.as_literal(), Some(&false));
        
        let sync_action = SynchronizeAction::default();
        assert_eq!(sync_action.target_entity_ref.as_literal(), Some(&"DefaultEntity".to_string()));
        
        let acquire_action = AcquirePositionAction::default();
        // Just verify it compiles and creates successfully
        let _ = acquire_action.position;
    }
}

// Add movement action validation
// impl ValidateAction for SpeedAction, TeleportAction