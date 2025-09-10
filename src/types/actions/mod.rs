//! Action type module organizing all OpenSCENARIO action definitions
//!
//! This file contains:
//! - Re-exports from action submodules (movement, control, appearance, traffic)
//! - Base Action trait defining common action behaviors
//! - Action validation logic and constraint checking
//! - Action execution context and state management
//! - Cross-cutting action concerns (timing, priority, conditions)
//!
//! Contributes to project by:
//! - Organizing 48+ action types into logical, manageable categories
//! - Providing consistent interface for all action types
//! - Enabling polymorphic action handling and execution
//! - Supporting action composition and complex scenario building
//! - Facilitating action validation and constraint enforcement

pub mod movement;   // Movement actions (SpeedAction, TeleportAction, etc.)
pub mod control;    // Phase 4B: Controller actions 
// pub mod appearance; // Appearance actions (skip for MVP)
// pub mod traffic;    // Traffic actions (skip for MVP)

// PHASE 4A: Export all movement actions
pub use movement::{
    // Original actions
    SpeedAction, TeleportAction, RoutingAction, FollowTrajectoryAction, 
    Trajectory, TrajectoryFollowingMode,
    // New Phase 4A actions
    LaneChangeAction, LaneOffsetAction, LateralAction, LateralDistanceAction,
    LongitudinalAction, LongitudinalDistanceAction, SpeedProfileAction,
    SynchronizeAction, AcquirePositionAction,
    // Supporting types
    LaneChangeTarget, LaneOffsetTarget, DynamicConstraints, FinalSpeed
};

// PHASE 4B: Export all controller actions
pub use control::{
    // Core controller actions
    AssignControllerAction, ActivateControllerAction, OverrideControllerValueAction,
    // Individual override actions
    OverrideControllerValueActionBrake, OverrideControllerValueActionThrottle,
    OverrideControllerValueActionSteeringWheel, OverrideControllerValueActionGear,
    OverrideControllerValueActionParkingBrake, OverrideControllerValueActionClutch,
    // Supporting types
    ManualGear, AutomaticGear, AutomaticGearType
};

use serde::{Deserialize, Serialize};

// PHASE 4A+4B: Complete Action enum with movement and controller actions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum Action { 
    // Original actions
    Speed(SpeedAction), 
    Teleport(TeleportAction),
    FollowTrajectory(FollowTrajectoryAction),
    Routing(RoutingAction),
    
    // Phase 4A: New movement actions
    LaneChange(LaneChangeAction),
    LaneOffset(LaneOffsetAction),
    Lateral(LateralAction),
    LateralDistance(LateralDistanceAction),
    Longitudinal(LongitudinalAction),
    LongitudinalDistance(LongitudinalDistanceAction),
    SpeedProfile(SpeedProfileAction),
    Synchronize(SynchronizeAction),
    AcquirePosition(AcquirePositionAction),
    
    // Phase 4B: Controller actions
    AssignController(AssignControllerAction),
    ActivateController(ActivateControllerAction),
    OverrideControllerValue(OverrideControllerValueAction),
    OverrideBrake(OverrideControllerValueActionBrake),
    OverrideThrottle(OverrideControllerValueActionThrottle),
    OverrideSteeringWheel(OverrideControllerValueActionSteeringWheel),
    OverrideGear(OverrideControllerValueActionGear),
    OverrideParkingBrake(OverrideControllerValueActionParkingBrake),
    OverrideClutch(OverrideControllerValueActionClutch),
}

impl Default for Action {
    fn default() -> Self {
        Action::Speed(SpeedAction::default())
    }
}

// Define Action wrapper with common attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionWrapper { 
    #[serde(rename = "@name")] 
    pub name: String, 
    #[serde(flatten)] 
    pub action: Action 
}

use crate::types::ValidationContext;

// Add action validation trait (Week 6) - KEEP AS FUTURE WORK
pub trait ValidateAction { 
    fn validate(&self, ctx: &ValidationContext) -> crate::error::Result<()>; 
}