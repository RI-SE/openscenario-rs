//! Action builder module with collection and management utilities

pub mod base;
pub mod movement;

pub use base::{ActionBuilder, ManeuverAction};
pub use movement::{SpeedActionBuilder, TeleportActionBuilder};

use crate::builder::{BuilderError, BuilderResult};
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