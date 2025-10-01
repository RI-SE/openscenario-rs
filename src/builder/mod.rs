//! Builder module root for programmatic scenario construction

// Import from new error module
mod error;
pub use error::{BuilderError, BuilderResult};

// Existing modules
pub mod positions;
pub mod scenario; 
pub mod fluent;

// New modules (Sprint 1+)
pub mod entities;   // Sprint 1
pub mod actions;    // Sprint 2
pub mod storyboard; // Sprint 3
pub mod conditions; // Sprint 4
pub mod init;       // Phase 1 - Critical for executable scenarios
pub mod templates;  // Phase 1 - Common scenario patterns
pub mod catalog;    // Sprint 5
pub mod parameters; // Sprint 5
pub mod validation; // Sprint 5

// Export main builders
pub use scenario::ScenarioBuilder;
pub use entities::{VehicleBuilder, DetachedVehicleBuilder};
pub use actions::{
    SpeedActionBuilder, TeleportActionBuilder, EnvironmentActionBuilder,
    LaneChangeActionBuilder, LateralDistanceActionBuilder, LaneOffsetActionBuilder,
    ActivateControllerActionBuilder, OverrideControllerValueActionBuilder,
    EntityActionBuilder, VariableActionBuilder
};
pub use storyboard::{StoryboardBuilder, StoryBuilder, ActBuilder, ManeuverBuilder, DetachedActBuilder, DetachedStoryBuilder, DetachedManeuverBuilder, DetachedSpeedActionBuilder};
pub use conditions::{
    TriggerBuilder, TimeConditionBuilder, SpeedConditionBuilder, ValueSpeedConditionBuilder,
    ParameterConditionBuilder, VariableConditionBuilder, AccelerationConditionBuilder,
    TraveledDistanceConditionBuilder, ReachPositionConditionBuilder, RelativeDistanceConditionBuilder,
    CollisionConditionBuilder
};
pub use init::{InitActionBuilder, PrivateActionBuilder, GlobalActionBuilder};
pub use templates::{BasicScenarioTemplate, ScenarioTemplate};
pub use catalog::{CatalogLocationsBuilder, CatalogEntityBuilder, VehicleCatalogReferenceBuilder, PedestrianCatalogReferenceBuilder};
pub use parameters::{ParameterDeclarationsBuilder, ParameterizedValueBuilder, ParameterContext};
pub use validation::{BuilderValidationContext, ValidationContextBuilder, BuilderValidatable};
