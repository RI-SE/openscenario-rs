//! Builders for the actions an entity performs during a scenario.
//!
//! Movement is [`SpeedActionBuilder`] and [`TeleportActionBuilder`]; lateral motion
//! is [`LaneChangeActionBuilder`], [`LateralDistanceActionBuilder`] and
//! [`LaneOffsetActionBuilder`]; controllers are [`ActivateControllerActionBuilder`]
//! and [`AssignControllerActionBuilder`]. [`EnvironmentActionBuilder`],
//! [`EntityActionBuilder`] and [`VariableActionBuilder`] are the global actions; they
//! change the world rather than one entity.
//!
//! These builders attach to a maneuver and are reached through it, not constructed
//! directly. For an action built on its own, use the `Detached*` variant in
//! [`crate::builder`]. `ActionCollection` is internal bookkeeping for a maneuver's
//! action list.

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
pub use routing::AssignRouteActionBuilder;
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
