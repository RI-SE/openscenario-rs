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

pub mod appearance; // Appearance and visibility actions
pub mod control; // Controller actions
pub mod movement; // Movement actions (SpeedAction, TeleportAction, etc.)
pub mod traffic; // Traffic actions
pub mod trailer; // Trailer actions
pub mod wrappers; // Action wrapper types matching XSD schema

// PHASE 4A: Export all movement actions
pub use movement::{
    AbsoluteTargetLane,
    AbsoluteTargetLaneOffset,
    AcquirePositionAction,
    AssignRouteAction,
    DynamicConstraints,
    FinalSpeed,
    FollowRouteAction,
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

// Export appearance actions
pub use appearance::{
    AnimationAction, AppearanceAction, LightStateAction, SensorReference, SensorReferenceSet,
    VisibilityAction,
};

// Export trailer actions
pub use trailer::{ConnectTrailerAction, DisconnectTrailerAction, TrailerAction};

// Export updated controller action
pub use control::{
    // Keep existing exports too
    ActivateControllerAction,
    // Core controller actions
    AssignControllerAction,
    AutomaticGear,
    AutomaticGearType,
    // Phase 1 Groups: XSD group wrappers
    Brake,
    BrakeInput,
    ControllerAction,
    Gear,
    // Supporting types
    ManualGear,
    OverrideControllerValueAction,
    // Individual override actions (legacy names)
    OverrideControllerValueActionBrake,
    OverrideControllerValueActionClutch,
    OverrideControllerValueActionGear,
    OverrideControllerValueActionParkingBrake,
    OverrideControllerValueActionSteeringWheel,
    OverrideControllerValueActionThrottle,
    // XSD compliant override action names
    OverrideBrakeAction,
    OverrideClutchAction,
    OverrideGearAction,
    OverrideParkingBrakeAction,
    OverrideSteeringWheelAction,
    OverrideThrottleAction,
};

// Export wrapper types from the wrappers module
pub use wrappers::*;





use crate::types::ValidationContext;

// Add action validation trait (Week 6) - KEEP AS FUTURE WORK
pub trait ValidateAction {
    fn validate(&self, ctx: &ValidationContext) -> crate::error::Result<()>;
}
