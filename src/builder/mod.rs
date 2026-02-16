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
//!         .add_vehicle("ego_vehicle", |v| v.car())
//!         .add_vehicle("target_vehicle", |v| v.truck())
//!     .with_storyboard(|storyboard| {
//!         storyboard
//!     })
//!     .build()
//!     .unwrap();
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
//!         .add_vehicle("ego", |v| v.car())
//!     .with_storyboard(|storyboard| {
//!         storyboard
//!     })
//!     .build()
//!     .unwrap();
//! ```
//!
//! ## Template-Based Construction
//!
//! ```rust
//! use openscenario_rs::builder::templates::{BasicScenarioTemplate, ScenarioTemplate};
//!
//! // Use predefined templates for common patterns
//! let template_builder = BasicScenarioTemplate::create();
//! let scenario = template_builder
//!     .with_storyboard(|storyboard| {
//!         // Configure storyboard with your scenario logic
//!         storyboard
//!     })
//!     .build()
//!     .unwrap();
//! ```
//!
//! ## Detached Builders
//!
//! For complex scenarios, you can use detached builders:
//!
//! ```rust
//! use openscenario_rs::builder::{DetachedVehicleBuilder, DetachedManeuverBuilder};
//! use openscenario_rs::ScenarioBuilder;
//!
//! // Build components separately
//! let ego_vehicle = DetachedVehicleBuilder::new("ego")
//!     .car()
//!     .build();
//!
//! let speed_maneuver = DetachedManeuverBuilder::new("speed_up", "ego")
//!     .build();
//!
//! // Combine into scenario using closure-based builders
//! let scenario = ScenarioBuilder::new()
//!     .with_header("Complex Test", "Engineer")
//!     .with_entities()
//!         .add_vehicle("ego", |v| v.car())
//!     .with_storyboard(|storyboard| {
//!         storyboard
//!     })
//!     .build()
//!     .unwrap();
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
//!         .add_vehicle("ego", |v| v.car())
//!     .with_storyboard(|storyboard| {
//!         storyboard
//!     })
//!     .build()
//! {
//!     Ok(scenario) => {
//!         println!("Scenario built successfully!");
//!         // Use scenario...
//!     }
//!     Err(BuilderError::MissingField { field, .. }) => {
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

mod error;
pub use error::{BuilderError, BuilderResult};

pub mod actions;
pub mod catalog;
pub mod conditions;
pub mod entities;
pub mod fluent;
pub mod init;
pub mod parameters;
pub mod positions;
pub mod scenario;
pub mod storyboard;
pub mod templates;
pub mod validation;

pub use actions::{
    ActivateControllerActionBuilder, EntityActionBuilder, EnvironmentActionBuilder,
    FollowTrajectoryActionBuilder, LaneChangeActionBuilder, LaneOffsetActionBuilder,
    LateralDistanceActionBuilder, PolylineBuilder, SpeedActionBuilder, TeleportActionBuilder,
    TrajectoryBuilder, VariableActionBuilder, VertexBuilder,
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
    ActBuilder, DetachedActBuilder, DetachedFollowTrajectoryActionBuilder, DetachedManeuverBuilder,
    DetachedSpeedActionBuilder, DetachedStoryBuilder, ManeuverBuilder, StoryBuilder,
    StoryboardBuilder,
};
pub use templates::{BasicScenarioTemplate, ScenarioTemplate};
pub use validation::{BuilderValidatable, BuilderValidationContext, ValidationContextBuilder};
