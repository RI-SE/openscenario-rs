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

use crate::types::basic::{Boolean, Double, Int, OSString};
use crate::types::catalogs::entities::CatalogTrajectory;
use crate::types::catalogs::references::{CatalogReference, ParameterAssignment};
use crate::types::enums::{DynamicsDimension, DynamicsShape, FollowingMode, SpeedTargetValueType};
use crate::types::geometry::shapes::Shape;
use crate::types::positions::Position;
use crate::types::routing::{Route, RouteRef};
use serde::{Deserialize, Deserializer, Serialize};

/// Custom deserializer for optional Double that handles empty strings gracefully
fn deserialize_optional_double<'de, D>(deserializer: D) -> Result<Option<Double>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = Option::<String>::deserialize(deserializer)?;
    match s {
        Some(s) if s.is_empty() => Ok(None),
        Some(s) => {
            // Try to deserialize as Double (Value<f64>)
            match s.parse::<f64>() {
                Ok(value) => Ok(Some(Double::literal(value))),
                Err(_) => {
                    // If it's not a literal, try to parse as parameter/expression
                    if s.starts_with("${") && s.ends_with('}') {
                        let content = &s[2..s.len() - 1];
                        if content.contains(|c: char| "+-*/%()".contains(c)) {
                            Ok(Some(Double::expression(content.to_string())))
                        } else {
                            Ok(Some(Double::parameter(content.to_string())))
                        }
                    } else if s.starts_with('$') {
                        Ok(Some(Double::parameter(s[1..].to_string())))
                    } else {
                        Err(serde::de::Error::custom(format!("Invalid Double value: {}", s)))
                    }
                }
            }
        },
        None => Ok(None),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SpeedAction {
    #[serde(rename = "SpeedActionDynamics")]
    pub speed_action_dynamics: TransitionDynamics,
    #[serde(rename = "SpeedActionTarget")]
    pub speed_action_target: SpeedActionTarget,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TeleportAction {
    #[serde(rename = "Position")]
    pub position: Position,
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
    #[serde(
        rename = "AbsoluteTargetSpeed",
        skip_serializing_if = "Option::is_none"
    )]
    pub absolute: Option<AbsoluteTargetSpeed>,
    #[serde(
        rename = "RelativeTargetSpeed",
        skip_serializing_if = "Option::is_none"
    )]
    pub relative: Option<RelativeTargetSpeed>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AbsoluteTargetSpeed {
    #[serde(rename = "@value")]
    pub value: Double,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelativeTargetSpeed {
    #[serde(rename = "@value")]
    pub value: Double,
    #[serde(rename = "@entityRef")]
    pub entity_ref: String,
    #[serde(rename = "@speedTargetValueType")]
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

/// Time reference for trajectory following
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimeReference {
    #[serde(rename = "Timing")]
    pub timing: Timing,
}

/// Timing specification for trajectory following
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Timing {
    #[serde(rename = "@domainAbsoluteRelative")]
    pub domain_absolute_relative: OSString,
    #[serde(rename = "@scale")]
    pub scale: Double,
    #[serde(rename = "@offset")]
    pub offset: Double,
}

/// Follow trajectory action with trajectory reference support
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FollowTrajectoryAction {
    /// Direct trajectory definition (deprecated, optional)
    #[serde(rename = "Trajectory", skip_serializing_if = "Option::is_none")]
    pub trajectory: Option<Trajectory>,

    /// Reference to a trajectory in a catalog (deprecated, optional)
    #[serde(rename = "CatalogReference", skip_serializing_if = "Option::is_none")]
    pub catalog_reference: Option<CatalogReference<CatalogTrajectory>>,

    /// Time reference for trajectory following (required)
    #[serde(rename = "TimeReference")]
    pub time_reference: TimeReference,

    /// Trajectory reference wrapper (optional)
    #[serde(rename = "TrajectoryRef", skip_serializing_if = "Option::is_none")]
    pub trajectory_ref: Option<TrajectoryRef>,

    /// Trajectory following mode (required)
    #[serde(rename = "TrajectoryFollowingMode")]
    pub trajectory_following_mode: TrajectoryFollowingMode,

    /// Initial distance offset attribute (optional)
    #[serde(rename = "@initialDistanceOffset", skip_serializing_if = "Option::is_none")]
    pub initial_distance_offset: Option<Double>,
}

impl Default for FollowTrajectoryAction {
    fn default() -> Self {
        Self {
            trajectory: Some(Trajectory::default()),
            catalog_reference: None,
            time_reference: TimeReference::default(),
            trajectory_ref: None,
            trajectory_following_mode: TrajectoryFollowingMode::default(),
            initial_distance_offset: None,
        }
    }
}

/// Assign route action for setting entity routes
///
/// Assigns a route to an entity, either through direct route definition
/// or catalog reference, enabling route-based navigation scenarios.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename = "AssignRouteAction")]
pub struct AssignRouteAction {
    /// Route reference (direct or catalog-based)
    #[serde(flatten)]
    pub route: RouteRef,
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
    /// Assign route action
    #[serde(rename = "AssignRouteAction", skip_serializing_if = "Option::is_none")]
    pub assign_route_action: Option<AssignRouteAction>,

    /// Follow trajectory action
    #[serde(
        rename = "FollowTrajectoryAction",
        skip_serializing_if = "Option::is_none"
    )]
    pub follow_trajectory_action: Option<FollowTrajectoryAction>,

    /// Follow route action
    #[serde(rename = "FollowRouteAction", skip_serializing_if = "Option::is_none")]
    pub follow_route_action: Option<FollowRouteAction>,
}

impl Default for RoutingAction {
    fn default() -> Self {
        Self {
            assign_route_action: None,
            follow_trajectory_action: None,
            follow_route_action: None,
        }
    }
}

// PHASE 4A: Core Movement Actions - Implementation

/// Lane change action for lateral lane movements
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LaneChangeAction {
    #[serde(rename = "@targetLaneOffset", default, deserialize_with = "deserialize_optional_double")]
    pub target_lane_offset: Option<Double>,
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
    pub value: Int,
}

/// Absolute target lane specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AbsoluteTargetLane {
    #[serde(rename = "@value")]
    pub value: OSString,
}

/// Lane offset action for lateral positioning within a lane
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LaneOffsetAction {
    #[serde(rename = "@continuous")]
    pub continuous: Boolean,
    #[serde(rename = "LaneOffsetActionDynamics")]
    pub dynamics: LaneOffsetActionDynamics,
    #[serde(rename = "LaneOffsetTarget")]
    pub target: LaneOffsetTarget,
}

/// Lane offset action dynamics specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LaneOffsetActionDynamics {
    #[serde(rename = "@dynamicsShape")]
    pub dynamics_shape: DynamicsShape,
    #[serde(rename = "@maxLateralAcc")]
    pub max_lateral_acc: Option<Double>,
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
    pub value: Double,
}

/// Absolute target lane offset specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AbsoluteTargetLaneOffset {
    #[serde(rename = "@value")]
    pub value: Double,
}

/// Lateral action wrapper for all lateral movement types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LateralAction {
    #[serde(flatten)]
    pub lateral_choice: LateralActionChoice,
}

/// Lateral action choice - lane change, offset, or distance keeping
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LateralActionChoice {
    #[serde(rename = "LaneChangeAction")]
    LaneChangeAction(LaneChangeAction),
    #[serde(rename = "LaneOffsetAction")]
    LaneOffsetAction(LaneOffsetAction),
    #[serde(rename = "LateralDistanceAction")]
    LateralDistanceAction(LateralDistanceAction),
}

/// Lateral distance action for maintaining lateral distance to an entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LateralDistanceAction {
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,
    #[serde(rename = "@distance")]
    pub distance: Option<Double>,
    #[serde(rename = "@freespace")]
    pub freespace: Boolean,
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
    
    /// Fixed distance value (mutually exclusive with timeGap)
    #[serde(rename = "@distance", skip_serializing_if = "Option::is_none")]
    pub distance: Option<Double>,
    
    /// Time gap value (mutually exclusive with distance)
    #[serde(rename = "@timeGap", skip_serializing_if = "Option::is_none")]
    pub time_gap: Option<Double>,
    
    /// Coordinate system for distance measurement
    #[serde(rename = "@coordinateSystem", skip_serializing_if = "Option::is_none")]
    pub coordinate_system: Option<OSString>,
    
    /// Displacement type for leading referenced entity
    #[serde(rename = "@displacement", skip_serializing_if = "Option::is_none")]
    pub displacement: Option<OSString>,
    
    #[serde(rename = "@freespace", skip_serializing_if = "Option::is_none")]
    pub freespace: Option<Boolean>,
    #[serde(rename = "@continuous")]
    pub continuous: Boolean,
    #[serde(rename = "DynamicConstraints", skip_serializing_if = "Option::is_none")]
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
    pub time: Double,
    #[serde(rename = "@speed")]
    pub speed: Double,
}

/// Dynamic constraints for movement actions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DynamicConstraints {
    #[serde(rename = "@maxLateralAcc")]
    pub max_lateral_acc: Option<Double>,
    #[serde(rename = "@maxSpeed")]
    pub max_speed: Option<Double>,
}

/// Synchronize action for coordinated entity movement
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SynchronizeAction {
    /// Reference to the master entity to synchronize with
    #[serde(rename = "@masterEntityRef")]
    pub master_entity_ref: OSString,

    /// Position of the master entity to synchronize with
    #[serde(rename = "TargetPositionMaster")]
    pub target_position_master: Position,

    /// Target position for this entity
    #[serde(rename = "TargetPosition")]
    pub target_position: Position,

    /// Optional final speed after synchronization
    #[serde(rename = "FinalSpeed", skip_serializing_if = "Option::is_none")]
    pub final_speed: Option<FinalSpeed>,

    /// Optional tolerance for master position matching
    #[serde(
        rename = "@targetToleranceMaster",
        skip_serializing_if = "Option::is_none"
    )]
    pub target_tolerance_master: Option<Double>,

    /// Optional tolerance for target position matching  
    #[serde(rename = "@targetTolerance", skip_serializing_if = "Option::is_none")]
    pub target_tolerance: Option<Double>,
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
    pub value: Double,
}

/// Relative speed to master specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelativeSpeedToMaster {
    #[serde(rename = "@value")]
    pub value: Double,
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

impl Default for TimeReference {
    fn default() -> Self {
        Self {
            timing: Timing::default(),
        }
    }
}

impl Default for Timing {
    fn default() -> Self {
        Self {
            domain_absolute_relative: OSString::literal("absolute".to_string()),
            scale: Double::literal(1.0),
            offset: Double::literal(0.0),
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

impl Default for AssignRouteAction {
    fn default() -> Self {
        Self {
            route: RouteRef::default(),
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
        parameters: Vec<ParameterAssignment>,
    ) -> Self {
        Self::with_catalog_reference(CatalogReference::with_parameters(
            catalog_name,
            entry_name,
            parameters,
        ))
    }
}

impl FollowTrajectoryAction {
    /// Create a follow trajectory action with direct trajectory
    pub fn with_trajectory(trajectory: Trajectory, following_mode: FollowingMode) -> Self {
        Self {
            trajectory: Some(trajectory),
            catalog_reference: None,
            time_reference: TimeReference::default(),
            trajectory_ref: None,
            trajectory_following_mode: TrajectoryFollowingMode { following_mode },
            initial_distance_offset: None,
        }
    }

    /// Validates the FollowTrajectoryAction according to XSD requirements
    /// TimeReference is required, trajectory sources are optional but at most one should be present
    pub fn validate(&self) -> Result<(), String> {
        let trajectory_count = [
            self.trajectory.is_some(),
            self.catalog_reference.is_some(),
            self.trajectory_ref.is_some(),
        ].iter().filter(|&&x| x).count();

        if trajectory_count > 1 {
            return Err("FollowTrajectoryAction can contain at most one trajectory source (Trajectory, CatalogReference, or TrajectoryRef), found multiple".to_string());
        }

        // TimeReference is required and always present due to struct definition
        Ok(())
    }

    /// Create a follow trajectory action with catalog reference
    pub fn with_catalog_reference(
        catalog_reference: CatalogReference<CatalogTrajectory>,
        following_mode: FollowingMode,
    ) -> Self {
        Self {
            trajectory: None,
            catalog_reference: Some(catalog_reference),
            time_reference: TimeReference::default(),
            trajectory_ref: None,
            trajectory_following_mode: TrajectoryFollowingMode { following_mode },
            initial_distance_offset: None,
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
            following_mode,
        )
    }
}

impl FollowRouteAction {
    /// Create a follow route action with direct route
    pub fn with_route(route: Route) -> Self {
        Self {
            route_ref: RouteRef::direct(route),
        }
    }

    /// Create a follow route action from catalog name and entry name
    pub fn from_catalog(catalog_name: impl Into<String>, entry_name: impl Into<String>) -> Self {
        Self {
            route_ref: RouteRef::catalog(catalog_name, entry_name),
        }
    }
}

impl AssignRouteAction {
    /// Create a new assign route action with direct route
    pub fn new(route: RouteRef) -> Self {
        Self { route }
    }

    /// Create an assign route action with direct route definition
    pub fn direct_route(route: Route) -> Self {
        Self {
            route: RouteRef::direct(route),
        }
    }

    /// Create an assign route action with catalog route reference
    pub fn catalog_route(catalog_name: impl Into<String>, entry_name: impl Into<String>) -> Self {
        Self {
            route: RouteRef::catalog(catalog_name, entry_name),
        }
    }
}

impl RoutingAction {
    /// Create a routing action with assign route action
    pub fn with_assign_route(action: AssignRouteAction) -> Self {
        Self {
            assign_route_action: Some(action),
            follow_trajectory_action: None,
            follow_route_action: None,
        }
    }
    /// Create a routing action with trajectory following
    pub fn with_trajectory(action: FollowTrajectoryAction) -> Self {
        Self {
            assign_route_action: None,
            follow_trajectory_action: Some(action),
            follow_route_action: None,
        }
    }

    /// Create a routing action with route following
    pub fn with_route(action: FollowRouteAction) -> Self {
        Self {
            assign_route_action: None,
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
            catalog_name,
            entry_name,
            following_mode,
        ))
    }

    /// Create a routing action with route from catalog
    pub fn with_route_from_catalog(catalog_name: String, entry_name: String) -> Self {
        Self::with_route(FollowRouteAction::from_catalog(catalog_name, entry_name))
    }
}

// PHASE 4A: Helper methods for LaneChangeAction system

impl LaneChangeAction {
    /// Create a new LaneChangeAction with the specified dynamics and target
    pub fn new(dynamics: TransitionDynamics, target: LaneChangeTarget) -> Self {
        Self {
            lane_change_action_dynamics: dynamics,
            lane_change_target: target,
            target_lane_offset: None,
        }
    }

    /// Set the target lane offset for this lane change action
    pub fn with_offset(mut self, offset: Double) -> Self {
        self.target_lane_offset = Some(offset);
        self
    }
}

impl LaneChangeTarget {
    /// Create a relative lane change target
    pub fn relative(entity_ref: impl Into<String>, value: i32) -> Self {
        Self {
            target_choice: LaneChangeTargetChoice::RelativeTargetLane(RelativeTargetLane {
                entity_ref: OSString::literal(entity_ref.into()),
                value: Int::literal(value),
            }),
        }
    }

    /// Create an absolute lane change target
    pub fn absolute(lane_id: impl Into<String>) -> Self {
        Self {
            target_choice: LaneChangeTargetChoice::AbsoluteTargetLane(AbsoluteTargetLane {
                value: OSString::literal(lane_id.into()),
            }),
        }
    }
}

impl RelativeTargetLane {
    /// Create a new relative target lane
    pub fn new(entity_ref: impl Into<String>, value: i32) -> Self {
        Self {
            entity_ref: OSString::literal(entity_ref.into()),
            value: Int::literal(value),
        }
    }
}

impl AbsoluteTargetLane {
    /// Create a new absolute target lane
    pub fn new(lane_id: impl Into<String>) -> Self {
        Self {
            value: OSString::literal(lane_id.into()),
        }
    }
}

// PHASE 4A: Helper methods for LaneOffsetAction system

impl LaneOffsetAction {
    /// Create a new LaneOffsetAction with the specified dynamics and target
    pub fn new(
        dynamics: LaneOffsetActionDynamics,
        target: LaneOffsetTarget,
        continuous: bool,
    ) -> Self {
        Self {
            dynamics,
            target,
            continuous: Boolean::literal(continuous),
        }
    }

    /// Set the continuous flag for this lane offset action
    pub fn with_continuous(mut self, continuous: bool) -> Self {
        self.continuous = Boolean::literal(continuous);
        self
    }
}

impl LaneOffsetActionDynamics {
    /// Create new lane offset action dynamics
    pub fn new(dynamics_shape: DynamicsShape) -> Self {
        Self {
            dynamics_shape,
            max_lateral_acc: None,
        }
    }

    /// Set the maximum lateral acceleration
    pub fn with_max_acceleration(mut self, max_acc: f64) -> Self {
        self.max_lateral_acc = Some(Double::literal(max_acc));
        self
    }
}

impl LaneOffsetTarget {
    /// Create a relative lane offset target
    pub fn relative(entity_ref: impl Into<String>, value: f64) -> Self {
        Self {
            target_choice: LaneOffsetTargetChoice::RelativeTargetLaneOffset(
                RelativeTargetLaneOffset {
                    entity_ref: OSString::literal(entity_ref.into()),
                    value: Double::literal(value),
                },
            ),
        }
    }

    /// Create an absolute lane offset target
    pub fn absolute(value: f64) -> Self {
        Self {
            target_choice: LaneOffsetTargetChoice::AbsoluteTargetLaneOffset(
                AbsoluteTargetLaneOffset {
                    value: Double::literal(value),
                },
            ),
        }
    }
}

impl RelativeTargetLaneOffset {
    /// Create a new relative target lane offset
    pub fn new(entity_ref: impl Into<String>, value: f64) -> Self {
        Self {
            entity_ref: OSString::literal(entity_ref.into()),
            value: Double::literal(value),
        }
    }
}

impl AbsoluteTargetLaneOffset {
    /// Create a new absolute target lane offset
    pub fn new(value: f64) -> Self {
        Self {
            value: Double::literal(value),
        }
    }
}

// PHASE 4A: Helper methods for LateralAction system

impl LateralAction {
    /// Create a lateral action with lane change
    pub fn lane_change(action: LaneChangeAction) -> Self {
        Self {
            lateral_choice: LateralActionChoice::LaneChangeAction(action),
        }
    }

    /// Create a lateral action with lane offset
    pub fn lane_offset(action: LaneOffsetAction) -> Self {
        Self {
            lateral_choice: LateralActionChoice::LaneOffsetAction(action),
        }
    }

    /// Create a lateral action with lateral distance
    pub fn lateral_distance(action: LateralDistanceAction) -> Self {
        Self {
            lateral_choice: LateralActionChoice::LateralDistanceAction(action),
        }
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
            value: Int::literal(1),
        }
    }
}

impl Default for AbsoluteTargetLane {
    fn default() -> Self {
        Self {
            value: OSString::literal("1".to_string()),
        }
    }
}

impl Default for LaneOffsetAction {
    fn default() -> Self {
        Self {
            continuous: Boolean::literal(false),
            dynamics: LaneOffsetActionDynamics::default(),
            target: LaneOffsetTarget::default(),
        }
    }
}

impl Default for LaneOffsetTarget {
    fn default() -> Self {
        Self {
            target_choice: LaneOffsetTargetChoice::AbsoluteTargetLaneOffset(
                AbsoluteTargetLaneOffset::default(),
            ),
        }
    }
}

impl Default for RelativeTargetLaneOffset {
    fn default() -> Self {
        Self {
            entity_ref: OSString::literal("DefaultEntity".to_string()),
            value: Double::literal(0.0),
        }
    }
}

impl Default for AbsoluteTargetLaneOffset {
    fn default() -> Self {
        Self {
            value: Double::literal(0.0),
        }
    }
}

impl Default for LateralAction {
    fn default() -> Self {
        Self {
            lateral_choice: LateralActionChoice::LaneChangeAction(LaneChangeAction::default()),
        }
    }
}

impl Default for LaneOffsetActionDynamics {
    fn default() -> Self {
        Self {
            dynamics_shape: DynamicsShape::Linear,
            max_lateral_acc: None,
        }
    }
}

impl Default for LateralDistanceAction {
    fn default() -> Self {
        Self {
            entity_ref: OSString::literal("DefaultEntity".to_string()),
            distance: Some(Double::literal(2.0)),
            freespace: Boolean::literal(true),
            continuous: Boolean::literal(false),
            dynamic_constraints: None,
        }
    }
}

impl Default for LongitudinalAction {
    fn default() -> Self {
        Self {
            longitudinal_action_choice: LongitudinalActionChoice::SpeedAction(
                SpeedAction::default(),
            ),
        }
    }
}

impl Default for LongitudinalDistanceAction {
    fn default() -> Self {
        Self {
            entity_ref: OSString::literal("DefaultEntity".to_string()),
            distance: Some(Double::literal(10.0)),
            time_gap: None,
            coordinate_system: None,
            displacement: None,
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
            time: Double::literal(0.0),
            speed: Double::literal(10.0),
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
            master_entity_ref: OSString::literal("DefaultEntity".to_string()),
            target_position_master: Position::default(),
            target_position: Position::default(),
            final_speed: None,
            target_tolerance_master: None,
            target_tolerance: None,
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
            value: Double::literal(10.0),
        }
    }
}

impl Default for RelativeSpeedToMaster {
    fn default() -> Self {
        Self {
            value: Double::literal(0.0),
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
    use crate::types::enums::{DynamicsDimension, DynamicsShape};
    use crate::types::positions::Position;

    #[test]
    fn test_lane_change_action_creation() {
        let action = LaneChangeAction::default();
        assert!(action.target_lane_offset.is_none());
        assert_eq!(
            action.lane_change_action_dynamics.dynamics_dimension,
            DynamicsDimension::Time
        );
        assert_eq!(
            action.lane_change_action_dynamics.dynamics_shape,
            DynamicsShape::Linear
        );
    }

    #[test]
    fn test_lane_change_target_relative() {
        let relative_target = RelativeTargetLane {
            entity_ref: OSString::literal("TestEntity".to_string()),
            value: Int::literal(2),
        };
        let target = LaneChangeTarget {
            target_choice: LaneChangeTargetChoice::RelativeTargetLane(relative_target),
        };

        if let LaneChangeTargetChoice::RelativeTargetLane(rel) = target.target_choice {
            assert_eq!(rel.entity_ref.as_literal(), Some(&"TestEntity".to_string()));
            assert_eq!(rel.value.as_literal(), Some(&2));
        } else {
            panic!("Expected RelativeTargetLane");
        }
    }

    #[test]
    fn test_lane_change_action_with_helper_methods() {
        let dynamics = TransitionDynamics::default();
        let target = LaneChangeTarget::relative("Ego", -1);
        let action = LaneChangeAction::new(dynamics, target);

        assert!(action.target_lane_offset.is_none());
    }

    #[test]
    fn test_lane_change_with_offset() {
        let dynamics = TransitionDynamics::default();
        let target = LaneChangeTarget::absolute("1");
        let action = LaneChangeAction::new(dynamics, target).with_offset(Double::literal(0.5));

        assert_eq!(
            action.target_lane_offset.unwrap().as_literal().unwrap(),
            &0.5
        );
    }

    #[test]
    fn test_lane_change_target_absolute() {
        let target = LaneChangeTarget::absolute("lane_1");

        if let LaneChangeTargetChoice::AbsoluteTargetLane(abs) = target.target_choice {
            assert_eq!(abs.value.as_literal(), Some(&"lane_1".to_string()));
        } else {
            panic!("Expected AbsoluteTargetLane");
        }
    }

    #[test]
    fn test_relative_target_lane_helper() {
        let relative = RelativeTargetLane::new("TestEntity", -2);
        assert_eq!(
            relative.entity_ref.as_literal(),
            Some(&"TestEntity".to_string())
        );
        assert_eq!(relative.value.as_literal(), Some(&-2));
    }

    #[test]
    fn test_absolute_target_lane_helper() {
        let absolute = AbsoluteTargetLane::new("lane_2");
        assert_eq!(absolute.value.as_literal(), Some(&"lane_2".to_string()));
    }

    #[test]
    fn test_xml_serialization_lane_change() {
        let action = LaneChangeAction::new(
            TransitionDynamics::default(),
            LaneChangeTarget::relative("Ego", -1),
        );

        let xml = quick_xml::se::to_string(&action).unwrap();
        assert!(xml.contains("LaneChangeAction"));
        assert!(xml.contains("RelativeTargetLane"));
        assert!(xml.contains("entityRef=\"Ego\""));
        assert!(xml.contains("value=\"-1\""));
    }

    #[test]
    fn test_xml_serialization_with_offset() {
        let action = LaneChangeAction::new(
            TransitionDynamics::default(),
            LaneChangeTarget::absolute("1"),
        )
        .with_offset(Double::literal(0.5));

        let xml = quick_xml::se::to_string(&action).unwrap();
        assert!(xml.contains("targetLaneOffset=\"0.5\""));
        assert!(xml.contains("AbsoluteTargetLane"));
    }

    #[test]
    fn test_xml_deserialization() {
        let xml = r#"
        <LaneChangeAction targetLaneOffset="0.5">
            <LaneChangeActionDynamics dynamicsDimension="time" dynamicsShape="linear" value="2.0" />
            <LaneChangeTarget>
                <RelativeTargetLane entityRef="Ego" value="-1" />
            </LaneChangeTarget>
        </LaneChangeAction>"#;

        let action: Result<LaneChangeAction, _> = quick_xml::de::from_str(xml);
        assert!(action.is_ok());

        let action = action.unwrap();
        assert_eq!(
            action.target_lane_offset.unwrap().as_literal().unwrap(),
            &0.5
        );

        if let LaneChangeTargetChoice::RelativeTargetLane(rel) =
            action.lane_change_target.target_choice
        {
            assert_eq!(rel.entity_ref.as_literal(), Some(&"Ego".to_string()));
            assert_eq!(rel.value.as_literal(), Some(&-1));
        } else {
            panic!("Expected RelativeTargetLane");
        }
    }

    #[test]
    fn test_xml_round_trip() {
        let original = LaneChangeAction::new(
            TransitionDynamics {
                dynamics_dimension: DynamicsDimension::Time,
                dynamics_shape: DynamicsShape::Linear,
                value: Double::literal(2.0),
            },
            LaneChangeTarget::relative("TestEntity", 2),
        )
        .with_offset(Double::literal(1.5));

        let xml = quick_xml::se::to_string(&original).unwrap();
        let deserialized: LaneChangeAction = quick_xml::de::from_str(&xml).unwrap();

        assert_eq!(
            original.target_lane_offset.unwrap().as_literal(),
            deserialized.target_lane_offset.unwrap().as_literal()
        );
        assert_eq!(
            original.lane_change_action_dynamics.value.as_literal(),
            deserialized.lane_change_action_dynamics.value.as_literal()
        );
    }

    #[test]
    fn test_lane_offset_action_creation() {
        let action = LaneOffsetAction::default();
        assert_eq!(action.continuous.as_literal(), Some(&false));
        assert_eq!(action.dynamics.dynamics_shape, DynamicsShape::Linear);
    }

    #[test]
    fn test_lane_offset_action_with_helper_methods() {
        let dynamics =
            LaneOffsetActionDynamics::new(DynamicsShape::Linear).with_max_acceleration(2.0);
        let target = LaneOffsetTarget::relative("Ego", 1.5);
        let action = LaneOffsetAction::new(dynamics, target, true);

        assert_eq!(action.continuous.as_literal(), Some(&true));
        assert_eq!(
            action.dynamics.max_lateral_acc.unwrap().as_literal(),
            Some(&2.0)
        );
    }

    #[test]
    fn test_lane_offset_target_relative() {
        let target = LaneOffsetTarget::relative("TestEntity", -0.5);

        if let LaneOffsetTargetChoice::RelativeTargetLaneOffset(rel) = target.target_choice {
            assert_eq!(rel.entity_ref.as_literal(), Some(&"TestEntity".to_string()));
            assert_eq!(rel.value.as_literal(), Some(&-0.5));
        } else {
            panic!("Expected RelativeTargetLaneOffset");
        }
    }

    #[test]
    fn test_lane_offset_target_absolute() {
        let target = LaneOffsetTarget::absolute(2.0);

        if let LaneOffsetTargetChoice::AbsoluteTargetLaneOffset(abs) = target.target_choice {
            assert_eq!(abs.value.as_literal(), Some(&2.0));
        } else {
            panic!("Expected AbsoluteTargetLaneOffset");
        }
    }

    #[test]
    fn test_lateral_action_helpers() {
        let lane_change = LaneChangeAction::default();
        let lateral_action = LateralAction::lane_change(lane_change);

        if let LateralActionChoice::LaneChangeAction(_) = lateral_action.lateral_choice {
            // Expected
        } else {
            panic!("Expected LaneChangeAction");
        }

        let lane_offset = LaneOffsetAction::default();
        let lateral_action = LateralAction::lane_offset(lane_offset);

        if let LateralActionChoice::LaneOffsetAction(_) = lateral_action.lateral_choice {
            // Expected
        } else {
            panic!("Expected LaneOffsetAction");
        }
    }

    #[test]
    fn test_xml_serialization_lane_offset() {
        let action = LaneOffsetAction::new(
            LaneOffsetActionDynamics::new(DynamicsShape::Linear),
            LaneOffsetTarget::relative("Ego", 1.0),
            true,
        );

        let xml = quick_xml::se::to_string(&action).unwrap();
        assert!(xml.contains("LaneOffsetAction"));
        assert!(xml.contains("continuous=\"true\""));
        assert!(xml.contains("RelativeTargetLaneOffset"));
        assert!(xml.contains("entityRef=\"Ego\""));
        assert!(xml.contains("value=\"1\""));
    }

    #[test]
    fn test_xml_round_trip_lane_offset() {
        let original = LaneOffsetAction::new(
            LaneOffsetActionDynamics::new(DynamicsShape::Linear).with_max_acceleration(1.5),
            LaneOffsetTarget::absolute(0.5),
            false,
        );

        let xml = quick_xml::se::to_string(&original).unwrap();
        let deserialized: LaneOffsetAction = quick_xml::de::from_str(&xml).unwrap();

        assert_eq!(
            original.continuous.as_literal(),
            deserialized.continuous.as_literal()
        );
        assert_eq!(
            original.dynamics.max_lateral_acc.unwrap().as_literal(),
            deserialized.dynamics.max_lateral_acc.unwrap().as_literal()
        );
    }

    #[test]
    fn test_lateral_distance_action_creation() {
        let action = LateralDistanceAction {
            entity_ref: OSString::literal("TargetEntity".to_string()),
            distance: Some(Double::literal(3.5)),
            freespace: Boolean::literal(true),
            continuous: Boolean::literal(false),
            dynamic_constraints: Some(DynamicConstraints {
                max_lateral_acc: Some(Double::literal(2.0)),
                max_speed: Some(Double::literal(50.0)),
            }),
        };

        assert_eq!(
            action.entity_ref.as_literal(),
            Some(&"TargetEntity".to_string())
        );
        assert_eq!(action.distance.unwrap().as_literal(), Some(&3.5));
        assert_eq!(action.freespace.as_literal(), Some(&true));
        assert_eq!(action.continuous.as_literal(), Some(&false));

        let constraints = action.dynamic_constraints.unwrap();
        assert_eq!(
            constraints.max_lateral_acc.unwrap().as_literal(),
            Some(&2.0)
        );
        assert_eq!(constraints.max_speed.unwrap().as_literal(), Some(&50.0));
    }

    #[test]
    fn test_longitudinal_action_choices() {
        // Test with SpeedAction
        let speed_action = LongitudinalAction {
            longitudinal_action_choice: LongitudinalActionChoice::SpeedAction(
                SpeedAction::default(),
            ),
        };

        if let LongitudinalActionChoice::SpeedAction(_) = speed_action.longitudinal_action_choice {
            // Expected
        } else {
            panic!("Expected SpeedAction");
        }

        // Test with LongitudinalDistanceAction
        let distance_action = LongitudinalAction {
            longitudinal_action_choice: LongitudinalActionChoice::LongitudinalDistanceAction(
                LongitudinalDistanceAction::default(),
            ),
        };

        if let LongitudinalActionChoice::LongitudinalDistanceAction(dist) =
            distance_action.longitudinal_action_choice
        {
            assert_eq!(dist.distance.as_literal(), Some(&10.0));
        } else {
            panic!("Expected LongitudinalDistanceAction");
        }
    }

    #[test]
    fn test_speed_profile_action_creation() {
        let entry1 = SpeedProfileEntry {
            time: Double::literal(0.0),
            speed: Double::literal(10.0),
        };
        let entry2 = SpeedProfileEntry {
            time: Double::literal(5.0),
            speed: Double::literal(20.0),
        };

        let action = SpeedProfileAction {
            entity_ref: Some(OSString::literal("RefEntity".to_string())),
            dynamic_constraints: Some(DynamicConstraints {
                max_lateral_acc: Some(Double::literal(1.5)),
                max_speed: Some(Double::literal(30.0)),
            }),
            entries: vec![entry1, entry2],
        };

        assert_eq!(
            action.entity_ref.unwrap().as_literal(),
            Some(&"RefEntity".to_string())
        );
        assert_eq!(action.entries.len(), 2);
        assert_eq!(action.entries[0].time.as_literal(), Some(&0.0));
        assert_eq!(action.entries[0].speed.as_literal(), Some(&10.0));
        assert_eq!(action.entries[1].time.as_literal(), Some(&5.0));
        assert_eq!(action.entries[1].speed.as_literal(), Some(&20.0));
    }

    #[test]
    fn test_synchronize_action_creation() {
        let action = SynchronizeAction {
            master_entity_ref: OSString::literal("SyncTarget".to_string()),
            target_position_master: Position::default(),
            target_position: Position::default(),
            final_speed: Some(FinalSpeed {
                speed_choice: FinalSpeedChoice::AbsoluteSpeed(AbsoluteSpeed {
                    value: Double::literal(15.0),
                }),
            }),
            target_tolerance_master: Some(Double::literal(1.0)),
            target_tolerance: Some(Double::literal(2.0)),
        };

        assert_eq!(
            action.master_entity_ref.as_literal(),
            Some(&"SyncTarget".to_string())
        );

        if let Some(final_speed) = action.final_speed {
            if let FinalSpeedChoice::AbsoluteSpeed(abs_speed) = final_speed.speed_choice {
                assert_eq!(abs_speed.value.as_literal(), Some(&15.0));
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
        assert_eq!(
            std::mem::size_of_val(&action.position),
            std::mem::size_of::<Position>()
        );
    }

    #[test]
    fn test_dynamic_constraints_creation() {
        let constraints = DynamicConstraints {
            max_lateral_acc: Some(Double::literal(3.0)),
            max_speed: Some(Double::literal(80.0)),
        };

        assert_eq!(
            constraints.max_lateral_acc.unwrap().as_literal(),
            Some(&3.0)
        );
        assert_eq!(constraints.max_speed.unwrap().as_literal(), Some(&80.0));

        let empty_constraints = DynamicConstraints::default();
        assert!(empty_constraints.max_lateral_acc.is_none());
        assert!(empty_constraints.max_speed.is_none());
    }

    #[test]
    fn test_final_speed_choices() {
        // Test absolute speed
        let abs_final = FinalSpeed {
            speed_choice: FinalSpeedChoice::AbsoluteSpeed(AbsoluteSpeed {
                value: Double::literal(25.0),
            }),
        };

        if let FinalSpeedChoice::AbsoluteSpeed(abs) = abs_final.speed_choice {
            assert_eq!(abs.value.as_literal(), Some(&25.0));
        }

        // Test relative speed to master
        let rel_final = FinalSpeed {
            speed_choice: FinalSpeedChoice::RelativeSpeedToMaster(RelativeSpeedToMaster {
                value: Double::literal(-5.0),
            }),
        };

        if let FinalSpeedChoice::RelativeSpeedToMaster(rel) = rel_final.speed_choice {
            assert_eq!(rel.value.as_literal(), Some(&-5.0));
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
        assert_eq!(
            sync_action.master_entity_ref.as_literal(),
            Some(&"DefaultEntity".to_string())
        );

        let acquire_action = AcquirePositionAction::default();
        // Just verify it compiles and creates successfully
        let _ = acquire_action.position;
    }

    #[test]
    fn test_follow_trajectory_action_validation() {
        // Test valid action with direct trajectory
        let valid_trajectory = FollowTrajectoryAction {
            trajectory: Some(Trajectory::default()),
            catalog_reference: None,
            time_reference: TimeReference::default(),
            trajectory_ref: None,
            trajectory_following_mode: TrajectoryFollowingMode::default(),
            initial_distance_offset: None,
        };
        assert!(valid_trajectory.validate().is_ok());

        // Test valid action with catalog reference
        let valid_catalog = FollowTrajectoryAction {
            trajectory: None,
            catalog_reference: Some(CatalogReference::new("catalog".to_string(), "entry".to_string())),
            time_reference: TimeReference::default(),
            trajectory_ref: None,
            trajectory_following_mode: TrajectoryFollowingMode::default(),
            initial_distance_offset: None,
        };
        assert!(valid_catalog.validate().is_ok());

        // Test valid action with trajectory ref
        let valid_ref = FollowTrajectoryAction {
            trajectory: None,
            catalog_reference: None,
            time_reference: TimeReference::default(),
            trajectory_ref: Some(TrajectoryRef::default()),
            trajectory_following_mode: TrajectoryFollowingMode::default(),
            initial_distance_offset: None,
        };
        assert!(valid_ref.validate().is_ok());

        // Test valid action with no trajectory source (only TimeReference required)
        let valid_none = FollowTrajectoryAction {
            trajectory: None,
            catalog_reference: None,
            time_reference: TimeReference::default(),
            trajectory_ref: None,
            trajectory_following_mode: TrajectoryFollowingMode::default(),
            initial_distance_offset: None,
        };
        assert!(valid_none.validate().is_ok());

        // Test invalid action with multiple trajectory sources
        let invalid_multiple = FollowTrajectoryAction {
            trajectory: Some(Trajectory::default()),
            catalog_reference: Some(CatalogReference::new("catalog".to_string(), "entry".to_string())),
            time_reference: TimeReference::default(),
            trajectory_ref: None,
            trajectory_following_mode: TrajectoryFollowingMode::default(),
            initial_distance_offset: None,
        };
        assert!(invalid_multiple.validate().is_err());
    }
}

// Add movement action validation
// impl ValidateAction for SpeedAction, TeleportAction
