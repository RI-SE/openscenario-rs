//! Base traits and utilities for action builders

use crate::builder::BuilderResult;
use crate::types::actions::wrappers::PrivateAction;
use crate::types::scenario::init::GlobalAction;

/// Common trait for all action builders
pub trait ActionBuilder {
    /// Build the action into a PrivateAction
    fn build_action(self) -> BuilderResult<PrivateAction>;
    
    /// Validate the action configuration
    fn validate(&self) -> BuilderResult<()>;
}

/// Common trait for builders that can be added to maneuvers
pub trait ManeuverAction: ActionBuilder {
    /// Get the entity reference this action applies to
    fn entity_ref(&self) -> Option<&str>;
}

/// Common trait for global action builders
pub trait GlobalActionBuilder {
    /// Build the action into a GlobalAction
    fn build_global_action(self) -> BuilderResult<GlobalAction>;
    
    /// Validate the action configuration
    fn validate(&self) -> BuilderResult<()>;
}