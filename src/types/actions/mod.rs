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

pub mod control;
pub mod movement; // Movement actions (SpeedAction, TeleportAction, etc.) // Phase 4B: Controller actions
                                                                          // pub mod appearance; // Appearance actions (skip for MVP)
pub mod traffic; // Phase 4C: Traffic actions

// PHASE 4A: Export all movement actions
pub use movement::{
    AbsoluteTargetLane,
    AbsoluteTargetLaneOffset,
    AcquirePositionAction,
    DynamicConstraints,
    FinalSpeed,
    FollowTrajectoryAction,
    // New Phase 4A actions
    LaneChangeAction,
    LaneChangeTarget,
    LaneChangeTargetChoice,
    LaneOffsetAction,
    LaneOffsetActionDynamics,
    LaneOffsetTarget,
    LaneOffsetTargetChoice,
    LateralAction,
    LateralActionChoice,
    LateralDistanceAction,
    LongitudinalAction,
    LongitudinalDistanceAction,
    RelativeTargetLane,
    RelativeTargetLaneOffset,
    RoutingAction,
    // Original actions
    SpeedAction,
    SpeedProfileAction,
    SynchronizeAction,
    TeleportAction,
    Trajectory,
    TrajectoryFollowingMode,
};

// PHASE 4B: Export all controller actions
pub use control::{
    ActivateControllerAction,
    // Core controller actions
    AssignControllerAction,
    AutomaticGear,
    AutomaticGearType,
    // Phase 1 Groups: XSD group wrappers
    Brake,
    BrakeInput,
    Gear,
    // Supporting types
    ManualGear,
    OverrideControllerValueAction,
    // Individual override actions
    OverrideControllerValueActionBrake,
    OverrideControllerValueActionClutch,
    OverrideControllerValueActionGear,
    OverrideControllerValueActionParkingBrake,
    OverrideControllerValueActionSteeringWheel,
    OverrideControllerValueActionThrottle,
};

// PHASE 4C: Export all traffic actions
pub use traffic::{
    CentralSwarmObject,
    ControllerDistribution,
    // Traffic signal system types
    Phase,
    TrafficArea,
    TrafficAreaAction,
    TrafficAreaVertex,
    // Supporting types
    TrafficDefinition,
    TrafficSignalAction,
    TrafficSignalController,
    TrafficSignalControllerAction,
    TrafficSignalGroupState,
    TrafficSignalState,
    TrafficSignalStateAction,
    TrafficSinkAction,
    // Core traffic actions
    TrafficSourceAction,
    TrafficStopAction,
    TrafficSwarmAction,
    VehicleCategory,
    VehicleCategoryDistribution,
};

use serde::{Deserialize, Serialize};

// PHASE 4A+4B+4C: Complete Action enum with movement, controller, and traffic actions
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

    // Phase 4C: Traffic actions
    TrafficSource(TrafficSourceAction),
    TrafficSink(TrafficSinkAction),
    TrafficSwarm(TrafficSwarmAction),
    TrafficArea(TrafficAreaAction),
    TrafficSignal(TrafficSignalAction),
    TrafficSignalState(TrafficSignalStateAction),
    TrafficSignalController(TrafficSignalControllerAction),
    TrafficStop(TrafficStopAction),
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
    pub action: Action,
}

use crate::types::ValidationContext;

// Add action validation trait (Week 6) - KEEP AS FUTURE WORK
pub trait ValidateAction {
    fn validate(&self, ctx: &ValidationContext) -> crate::error::Result<()>;
}
