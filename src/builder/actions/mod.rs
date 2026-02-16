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
//! // SpeedActionBuilder and TeleportActionBuilder are used within maneuver builders
//! // They are not used directly in this way anymore
//! // See DetachedSpeedActionBuilder and DetachedTeleportActionBuilder for standalone usage
//! ```
//!
//! # Action Collections
//!
//! ActionCollection is used internally for managing actions. For creating actions,
//! see the detached builders in the builder module.
//!
//! ```rust
//! // Action builders are used within maneuver builders
//! // See storyboard/maneuver module for detailed usage examples
//! ```

pub mod base;
pub mod controller;
pub mod global;
pub mod lateral;
pub mod longitudinal;
pub mod movement;
pub mod routing;
pub mod synchronize;
pub mod trajectory;
pub mod visibility;

pub use base::{ActionBuilder, ManeuverAction};
pub use controller::{ActivateControllerActionBuilder, AssignControllerActionBuilder};
pub use global::{EntityActionBuilder, EnvironmentActionBuilder, VariableActionBuilder};
pub use lateral::{LaneChangeActionBuilder, LaneOffsetActionBuilder, LateralDistanceActionBuilder};
pub use longitudinal::{LongitudinalDistanceActionBuilder, SpeedProfileActionBuilder};
pub use movement::{SpeedActionBuilder, TeleportActionBuilder};
pub use routing::{AssignRouteActionBuilder, FollowRouteActionBuilder};
pub use synchronize::SynchronizeActionBuilder;
pub use trajectory::{
    FollowTrajectoryActionBuilder, PolylineBuilder, TrajectoryBuilder, VertexBuilder,
};
pub use visibility::VisibilityActionBuilder;

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
