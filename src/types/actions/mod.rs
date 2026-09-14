//! Action types, re-exported from the submodules that define them: [`movement`],
//! [`control`], [`appearance`], [`traffic`], [`trailer`], and the [`wrappers`] that
//! model the XSD's action choice groups.
pub mod appearance; // Appearance and visibility actions
pub mod control; // Controller actions
pub mod movement; // Movement actions (SpeedAction, TeleportAction, etc.)
pub mod traffic; // Traffic actions
pub mod trailer; // Trailer actions
pub mod wrappers; // Action wrapper types matching XSD schema

pub use movement::{
    AbsoluteTargetLane, AbsoluteTargetLaneOffset, AcquirePositionAction, AssignRouteAction,
    DynamicConstraints, FinalSpeed, FollowTrajectoryAction, LaneChangeAction, LaneChangeTarget,
    LaneChangeTargetChoice, LaneOffsetAction, LaneOffsetActionDynamics, LaneOffsetTarget,
    LaneOffsetTargetChoice, LateralAction, LateralActionChoice, LateralDistanceAction,
    LongitudinalAction, LongitudinalDistanceAction, RelativeTargetLane, RelativeTargetLaneOffset,
    RoutingAction, SpeedAction, SpeedProfileAction, SynchronizeAction, TargetDistanceSteadyState,
    TargetTimeSteadyState, TeleportAction, Trajectory, TrajectoryFollowingMode,
};

pub use crate::types::enums::VehicleCategory;
pub use traffic::{
    CentralSwarmObject,
    ControllerDistribution,
    DirectionOfTravelDistribution,
    Lane,
    Phase,
    Polygon,
    RoadCursor,
    RoadRange,
    TrafficArea,
    TrafficAreaAction,
    // Supporting types
    TrafficDefinition,
    TrafficDistribution,
    TrafficDistributionEntry,
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
    VehicleCategoryDistribution,
};

// Export appearance actions
pub use appearance::{
    AnimationAction, AnimationFile, AnimationState, AnimationType, AppearanceAction, Color,
    ColorCmyk, ColorRgb, ComponentAnimation, LightState, LightStateAction, LightType,
    PedestrianAnimation, PedestrianGesture, SensorReference, SensorReferenceSet,
    UserDefinedAnimation, UserDefinedComponent, UserDefinedLight, VehicleComponent, VehicleLight,
    VisibilityAction,
};

// Export trailer actions
pub use trailer::{ConnectTrailerAction, DisconnectTrailerAction, TrailerAction};

// Export updated controller action
pub use control::{
    ActivateControllerAction, AssignControllerAction, AutomaticGear, AutomaticGearType, Brake,
    BrakeInput, ControllerAction, Gear, ManualGear, OverrideBrakeAction, OverrideClutchAction,
    OverrideGearAction, OverrideParkingBrakeAction, OverrideSteeringWheelAction,
    OverrideThrottleAction,
};

// Export wrapper types from the wrappers module
pub use wrappers::*;

// `pub trait ValidateAction` removed. It had zero impls crate-wide — its only
// mention was a commented-out line at `actions/movement.rs:2166`. `types::mod::Validate` is
// the live validation trait; use that.
