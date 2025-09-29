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
pub mod catalog;    // Sprint 5
pub mod parameters; // Sprint 5
pub mod validation; // Sprint 5

// Export main builders
pub use scenario::ScenarioBuilder;
pub use entities::VehicleBuilder;
pub use actions::{SpeedActionBuilder, TeleportActionBuilder};
pub use storyboard::{StoryboardBuilder, StoryBuilder, ActBuilder, ManeuverBuilder, DetachedActBuilder, DetachedManeuverBuilder, DetachedSpeedActionBuilder, DetachedTeleportActionBuilder};
pub use conditions::{TriggerBuilder, TimeConditionBuilder, SpeedConditionBuilder, DistanceConditionBuilder};
pub use catalog::{CatalogLocationsBuilder, CatalogEntityBuilder, VehicleCatalogReferenceBuilder, PedestrianCatalogReferenceBuilder};
pub use parameters::{ParameterDeclarationsBuilder, ParameterizedValueBuilder, ParameterContext};
pub use validation::{BuilderValidationContext, ValidationContextBuilder, BuilderValidatable};
