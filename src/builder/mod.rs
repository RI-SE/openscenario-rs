//! Programmatic construction of OpenSCENARIO documents.
//!
//! The builder carries its progress in the type: `ScenarioBuilder<Empty>` has no
//! `with_entities`, `ScenarioBuilder<HasHeader>` does. Ordering the stages wrongly
//! is therefore a compile error. A required value left unset is not; that surfaces
//! at `build()`, as a [`BuilderError`].
//!
//! ```rust
//! use openscenario_rs::builder::ScenarioBuilder;
//! use openscenario_rs::types::catalogs::locations::CatalogLocations;
//! use openscenario_rs::types::enums::ParameterType;
//! use openscenario_rs::types::road::RoadNetwork;
//!
//! // The XSD requires CatalogLocations and RoadNetwork of every scenario
//! // document; the empty forms state nothing but satisfy it.
//! let scenario = ScenarioBuilder::new()
//!     .with_header("Highway Merge Test", "Test Engineer")
//!     .add_parameter("initial_speed", ParameterType::Double, "25.0")
//!     .with_catalog_locations(CatalogLocations::default())
//!     .with_road_network(RoadNetwork::default())
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
//! [`templates`] holds prebuilt starting points for the common shapes:
//!
//! ```rust
//! use openscenario_rs::builder::templates::{BasicScenarioTemplate, ScenarioTemplate};
//!
//! let scenario = BasicScenarioTemplate::create()
//!     .with_storyboard(|storyboard| storyboard)
//!     .build()
//!     .unwrap();
//! ```
//!
//! Detached builders produce a component with no parent attached, so the same vehicle
//! or maneuver can be built once and used in several scenarios:
//!
//! ```rust
//! use openscenario_rs::builder::{DetachedVehicleBuilder, DetachedManeuverBuilder};
//!
//! let ego_vehicle = DetachedVehicleBuilder::new("ego").car().build();
//! let speed_maneuver = DetachedManeuverBuilder::new("speed_up", "ego").build();
//! ```
//!
//! `build()` is where a missing required field surfaces, as a [`BuilderError`]:
//!
//! ```rust
//! use openscenario_rs::builder::{ScenarioBuilder, BuilderError};
//!
//! match ScenarioBuilder::new()
//!     .with_header("Test", "Engineer")
//!     .with_entities()
//!         .add_vehicle("ego", |v| v.car())
//!     .with_storyboard(|storyboard| storyboard)
//!     .build()
//! {
//!     Ok(scenario) => println!("built"),
//!     Err(BuilderError::MissingField { field, .. }) => {
//!         eprintln!("missing required field: {}", field);
//!     }
//!     Err(e) => eprintln!("{}", e),
//! }
//! ```

mod error;
pub use error::{BuilderError, BuilderResult};

pub mod actions;
pub mod catalog;
pub mod conditions;
pub mod entities;
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
