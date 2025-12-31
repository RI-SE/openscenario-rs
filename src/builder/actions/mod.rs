//! Action builder module with collection and management utilities
//!
//! This module provides builders for creating various types of actions in OpenSCENARIO scenarios.
//! Actions define what entities do during scenario execution, including movement, speed changes,
//! lane changes, and environment modifications.
//!
//! # Action Categories
//!
//! ## Movement Actions
//! - [`SpeedActionBuilder`] - Control entity speed (absolute or relative)
//! - [`TeleportActionBuilder`] - Instantly move entities to new positions
//!
//! ## Lateral Actions  
//! - [`LaneChangeActionBuilder`] - Execute lane change maneuvers
//! - [`LateralDistanceActionBuilder`] - Maintain lateral distance to targets
//! - [`LaneOffsetActionBuilder`] - Apply lateral offset from lane center
//!
//! ## Controller Actions
//! - [`ActivateControllerActionBuilder`] - Activate entity controllers
//! - [`OverrideControllerValueActionBuilder`] - Override controller parameters
//! - [`AssignControllerActionBuilder`] - Assign controllers to entities
//!
//! ## Global Actions
//! - [`EnvironmentActionBuilder`] - Modify weather, time of day, lighting
//! - [`EntityActionBuilder`] - Add/remove entities during scenario
//! - [`VariableActionBuilder`] - Modify scenario variables
//!
//! # Basic Usage
//!
//! ```rust
//! use openscenario_rs::builder::actions::{SpeedActionBuilder, TeleportActionBuilder};
//!
//! // Create a speed action
//! let speed_action = SpeedActionBuilder::new()
//!     .for_entity("ego_vehicle")
//!     .to_speed(25.0);
//!
//! // Create a teleport action
//! let teleport_action = TeleportActionBuilder::new()
//!     .for_entity("ego_vehicle")
//!     .to_world_position(100.0, 50.0, 0.0);
//! ```
//!
//! # Action Collections
//!
//! Use `ActionCollection` to manage multiple actions in a maneuver:
//!
//! ```rust
//! use openscenario_rs::builder::actions::{ActionCollection, SpeedActionBuilder};
//!
//! let actions = ActionCollection::new()
//!     .add_action(SpeedActionBuilder::new()
//!         .for_entity("ego")
//!         .to_speed(30.0))?;
//!
//! let action_list = actions.into_actions();
//! ```

pub mod base;
pub mod controller;
pub mod global;
pub mod lateral;
pub mod movement;
pub mod trajectory;

pub use base::{ActionBuilder, ManeuverAction};
pub use controller::{
    ActivateControllerActionBuilder, AssignControllerActionBuilder,
    OverrideControllerValueActionBuilder,
};
pub use global::{EntityActionBuilder, EnvironmentActionBuilder, VariableActionBuilder};
pub use lateral::{LaneChangeActionBuilder, LaneOffsetActionBuilder, LateralDistanceActionBuilder};
pub use movement::{SpeedActionBuilder, TeleportActionBuilder};
pub use trajectory::{
    FollowTrajectoryActionBuilder, PolylineBuilder, TrajectoryBuilder, VertexBuilder,
};

use crate::builder::BuilderResult;
use crate::types::actions::wrappers::PrivateAction;

/// Collection of actions for a maneuver
#[derive(Debug, Default)]
pub struct ActionCollection {
    actions: Vec<PrivateAction>,
}

impl ActionCollection {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an action to the collection
    pub fn add_action<A: ActionBuilder>(mut self, action_builder: A) -> BuilderResult<Self> {
        let action = action_builder.build_action()?;
        self.actions.push(action);
        Ok(self)
    }

    /// Get all actions
    pub fn into_actions(self) -> Vec<PrivateAction> {
        self.actions
    }
}
