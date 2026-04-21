//! Action type module organizing all OpenSCENARIO action definitions
//!
//! This file contains:
//! - Re-exports from action submodules (movement, control, appearance, traffic)
//! - Base Action trait defining common action behaviors
//! - Action validation logic and constraint checking
//! - Action execution context and state management
//! - Cross-cutting action concerns (timing, priority, conditions)
//!
pub mod appearance; // Appearance and visibility actions
pub mod control; // Controller actions
pub mod movement; // Movement actions (SpeedAction, TeleportAction, etc.)
pub mod traffic; // Traffic actions
pub mod trailer; // Trailer actions
pub mod wrappers; // Action wrapper types matching XSD schema

pub use movement::{
    AbsoluteTargetLane,
    AbsoluteTargetLaneOffset,
    AcquirePositionAction,
    AssignRouteAction,
    DynamicConstraints,
    FinalSpeed,
    FollowRouteAction,
    FollowTrajectoryAction,
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
    SpeedAction,
    SpeedProfileAction,
    SynchronizeAction,
    TeleportAction,
    Trajectory,
    TrajectoryFollowingMode,
};

pub use traffic::{
    CentralSwarmObject,
    ControllerDistribution,
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
    ActivateControllerAction,
    AssignControllerAction,
    AutomaticGear,
    AutomaticGearType,
    Brake,
    BrakeInput,
    ControllerAction,
    Gear,
    ManualGear,
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

pub trait ValidateAction {
    fn validate(&self, ctx: &ValidationContext) -> crate::error::Result<()>;
}
