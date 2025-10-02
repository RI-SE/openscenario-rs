//! Builder module for programmatic OpenSCENARIO construction
//!
//! This module provides a comprehensive, type-safe API for building OpenSCENARIO documents
//! programmatically. The builder system uses compile-time state validation to ensure
//! scenarios are constructed correctly and completely.
//!
//! # Key Features
//!
//! - **Type-safe construction** - Invalid operations are caught at compile time
//! - **Fluent API** - Chainable methods for readable code
//! - **Comprehensive coverage** - All OpenSCENARIO elements supported
//! - **Validation** - Built-in validation during construction
//! - **Templates** - Pre-built patterns for common scenarios
//!
//! # Quick Start
//!
//! ```rust
//! use openscenario_rs::builder::ScenarioBuilder;
//! use openscenario_rs::types::enums::ParameterType;
//!
//! // Build a complete scenario
//! let scenario = ScenarioBuilder::new()
//!     .with_header("Highway Merge Test", "Test Engineer")
//!     .add_parameter("initial_speed", ParameterType::Double, "25.0")
//!     .with_entities()
//!         .add_vehicle("ego_vehicle")
//!             .car()
//!             .finish()
//!         .add_vehicle("target_vehicle")
//!             .truck()
//!             .finish()
//!     .with_storyboard()
//!         .add_story("merge_scenario")
//!             .add_act("initialization")
//!                 .add_maneuver("ego_setup", "ego_vehicle")
//!                     .add_teleport_action()
//!                         .to_world_position(0.0, 0.0, 0.0)
//!                         .finish()?
//!                     .finish()
//!                 .finish()
//!             .add_act("merge_action")
//!                 .add_maneuver("ego_merge", "ego_vehicle")
//!                     .add_speed_action()
//!                         .to_speed(30.0)
//!                         .finish()?
//!                     .finish()
//!                 .finish()
//!             .finish()
//!         .finish()
//!     .build()?;
//! ```
//!
//! # Builder Categories
//!
//! ## Core Builders
//!
//! - [`ScenarioBuilder`] - Main entry point for scenario construction
//! - [`VehicleBuilder`] - Vehicle entity creation and configuration
//! - [`StoryboardBuilder`] - Storyboard structure and flow control
//!
//! ## Action Builders
//!
//! - [`SpeedActionBuilder`] - Vehicle speed control actions
//! - [`TeleportActionBuilder`] - Entity positioning actions  
//! - [`LaneChangeActionBuilder`] - Lane change maneuvers
//! - [`EnvironmentActionBuilder`] - Environment and weather control
//!
//! ## Condition Builders
//!
//! - [`TriggerBuilder`] - Event triggering logic
//! - [`SpeedConditionBuilder`] - Speed-based triggers
//! - [`TimeConditionBuilder`] - Time-based triggers
//! - [`RelativeDistanceConditionBuilder`] - Distance-based triggers
//!
//! ## Utility Builders
//!
//! - [`CatalogLocationsBuilder`] - Catalog reference management
//! - [`ParameterDeclarationsBuilder`] - Parameter definition
//! - [`InitActionBuilder`] - Initial entity setup
//!
//! # Advanced Usage
//!
//! ## Parameterized Scenarios
//!
//! ```rust
//! use openscenario_rs::builder::ScenarioBuilder;
//! use openscenario_rs::types::enums::ParameterType;
//!
//! let scenario = ScenarioBuilder::new()
//!     .with_header("Parameterized Test", "Engineer")
//!     .add_parameter("target_speed", ParameterType::Double, "30.0")
//!     .add_parameter("following_distance", ParameterType::Double, "50.0")
//!     .with_entities()
//!         .add_vehicle("ego")
//!             .car()
//!             .finish()
//!     .with_storyboard()
//!         .add_story("speed_test")
//!             .add_act("acceleration")
//!                 .add_maneuver("reach_target", "ego")
//!                     .add_speed_action()
//!                         .to_speed(30.0)  // Could use parameter reference
//!                         .finish()?
//!                     .finish()
//!                 .finish()
//!             .finish()
//!         .finish()
//!     .build()?;
//! ```
//!
//! ## Template-Based Construction
//!
//! ```rust
//! use openscenario_rs::builder::templates::{BasicScenarioTemplate, ScenarioTemplate};
//!
//! // Use predefined templates for common patterns
//! let template = BasicScenarioTemplate::new()
//!     .with_single_vehicle("ego_car")
//!     .with_straight_road(1000.0)
//!     .with_acceleration_test(0.0, 30.0, 5.0);
//!
//! let scenario = template.build("Acceleration Test", "Test Engineer")?;
//! ```
//!
//! ## Detached Builders
//!
//! For complex scenarios, you can use detached builders:
//!
//! ```rust
//! use openscenario_rs::builder::{DetachedVehicleBuilder, DetachedManeuverBuilder};
//!
//! // Build components separately
//! let ego_vehicle = DetachedVehicleBuilder::new("ego")
//!     .car()
//!     .build()?;
//!
//! let speed_maneuver = DetachedManeuverBuilder::new("speed_up")
//!     .add_speed_action()
//!         .to_speed(25.0)
//!         .finish()?
//!     .build()?;
//!
//! // Combine into scenario
//! let scenario = ScenarioBuilder::new()
//!     .with_header("Complex Test", "Engineer")
//!     .with_entities()
//!         .add_existing_vehicle(ego_vehicle)
//!     .with_storyboard()
//!         .add_story("main")
//!             .add_act("test_act")
//!                 .add_existing_maneuver(speed_maneuver, "ego")
//!                 .finish()
//!             .finish()
//!         .finish()
//!     .build()?;
//! ```
//!
//! # Validation and Error Handling
//!
//! ```rust
//! use openscenario_rs::builder::{ScenarioBuilder, BuilderError};
//!
//! match ScenarioBuilder::new()
//!     .with_header("Test", "Engineer")
//!     .with_entities()
//!         .add_vehicle("ego")
//!             .car()
//!             .finish()
//!     .with_storyboard()
//!         .add_story("story")
//!             .finish()
//!         .finish()
//!     .build()
//! {
//!     Ok(scenario) => {
//!         println!("Scenario built successfully!");
//!         // Use scenario...
//!     }
//!     Err(BuilderError::ValidationError(msg)) => {
//!         eprintln!("Validation failed: {}", msg);
//!     }
//!     Err(BuilderError::MissingRequiredField(field)) => {
//!         eprintln!("Missing required field: {}", field);
//!     }
//!     Err(e) => {
//!         eprintln!("Builder error: {}", e);
//!     }
//! }
//! ```
//!
//! # Performance Tips
//!
//! - Use detached builders for reusable components
//! - Build templates once and reuse for similar scenarios
//! - Use parameter references instead of literal values for flexibility
//! - Validate during construction rather than after building

// Import from new error module
mod error;
pub use error::{BuilderError, BuilderResult};

// Existing modules
pub mod fluent;
pub mod positions;
pub mod scenario;

// New modules (Sprint 1+)
pub mod actions; // Sprint 2
pub mod catalog; // Sprint 5
pub mod conditions; // Sprint 4
pub mod entities; // Sprint 1
pub mod init; // Phase 1 - Critical for executable scenarios
pub mod parameters; // Sprint 5
pub mod storyboard; // Sprint 3
pub mod templates; // Phase 1 - Common scenario patterns
pub mod validation; // Sprint 5

// Export main builders
pub use actions::{
    ActivateControllerActionBuilder, EntityActionBuilder, EnvironmentActionBuilder,
    LaneChangeActionBuilder, LaneOffsetActionBuilder, LateralDistanceActionBuilder,
    OverrideControllerValueActionBuilder, SpeedActionBuilder, TeleportActionBuilder,
    VariableActionBuilder,
};
pub use catalog::{
    CatalogEntityBuilder, CatalogLocationsBuilder, PedestrianCatalogReferenceBuilder,
    VehicleCatalogReferenceBuilder,
};
pub use conditions::{
    AccelerationConditionBuilder, CollisionConditionBuilder, ParameterConditionBuilder,
    ReachPositionConditionBuilder, RelativeDistanceConditionBuilder, SpeedConditionBuilder,
    TimeConditionBuilder, TraveledDistanceConditionBuilder, TriggerBuilder,
    ValueSpeedConditionBuilder, VariableConditionBuilder,
};
pub use entities::{DetachedVehicleBuilder, VehicleBuilder};
pub use init::{GlobalActionBuilder, InitActionBuilder, PrivateActionBuilder};
pub use parameters::{ParameterContext, ParameterDeclarationsBuilder, ParameterizedValueBuilder};
pub use scenario::ScenarioBuilder;
pub use storyboard::{
    ActBuilder, DetachedActBuilder, DetachedManeuverBuilder, DetachedSpeedActionBuilder,
    DetachedStoryBuilder, ManeuverBuilder, StoryBuilder, StoryboardBuilder,
};
pub use templates::{BasicScenarioTemplate, ScenarioTemplate};
pub use validation::{BuilderValidatable, BuilderValidationContext, ValidationContextBuilder};
