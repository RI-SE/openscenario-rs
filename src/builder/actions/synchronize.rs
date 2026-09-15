//! Synchronize action builder.
//!
//! The action holds an entity's arrival at its own target position in step with a
//! master entity's arrival at the master position.
//!
//! ```rust
//! use openscenario_rs::builder::actions::synchronize::SynchronizeActionBuilder;
//! use openscenario_rs::types::positions::Position;
//!
//! let sync_action = SynchronizeActionBuilder::new()
//!     .for_entity("ego_vehicle")
//!     .with_master("lead_vehicle")
//!     .master_position(Position::world_origin())
//!     .entity_position(Position::world_origin());
//! ```

use crate::builder::actions::base::{ActionBuilder, ManeuverAction};
use crate::builder::{BuilderError, BuilderResult};
use crate::types::{
    actions::movement::SynchronizeAction, actions::wrappers::PrivateAction, basic::OSString,
    positions::Position,
};

/// Builder for synchronize actions
#[derive(Debug, Default)]
pub struct SynchronizeActionBuilder {
    entity_ref: Option<String>,
    master_entity_ref: Option<String>,
    target_position_master: Option<Position>,
    target_position: Option<Position>,
}

impl SynchronizeActionBuilder {
    /// Create new synchronize action builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set target entity for this action
    pub fn for_entity(mut self, entity_ref: &str) -> Self {
        self.entity_ref = Some(entity_ref.to_string());
        self
    }

    /// Set the master entity reference to synchronize with
    pub fn with_master(mut self, master_entity_ref: &str) -> Self {
        self.master_entity_ref = Some(master_entity_ref.to_string());
        self
    }

    /// Set the target position for the master entity
    pub fn master_position(mut self, position: Position) -> Self {
        self.target_position_master = Some(position);
        self
    }

    /// Set the target position for this entity
    pub fn entity_position(mut self, position: Position) -> Self {
        self.target_position = Some(position);
        self
    }
}

impl ActionBuilder for SynchronizeActionBuilder {
    fn build_action(self) -> BuilderResult<PrivateAction> {
        self.validate()?;

        let action = SynchronizeAction {
            master_entity_ref: OSString::literal(self.master_entity_ref.unwrap()),
            target_position_master: self.target_position_master.unwrap(),
            target_position: self.target_position.unwrap(),
            final_speed: None,
            target_tolerance_master: None,
            target_tolerance: None,
        };

        Ok(PrivateAction::SynchronizeAction(action))
    }

    fn validate(&self) -> BuilderResult<()> {
        if self.master_entity_ref.is_none() {
            return Err(BuilderError::validation_error(
                "Master entity reference is required",
            ));
        }
        if self.target_position_master.is_none() {
            return Err(BuilderError::validation_error(
                "Master target position is required",
            ));
        }
        if self.target_position.is_none() {
            return Err(BuilderError::validation_error(
                "Entity target position is required",
            ));
        }
        Ok(())
    }
}

impl ManeuverAction for SynchronizeActionBuilder {
    fn entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_synchronize_basic() {
        let master_pos = Position::world_origin();
        let entity_pos = Position::world_origin();

        let builder = SynchronizeActionBuilder::new()
            .for_entity("ego")
            .with_master("lead")
            .master_position(master_pos)
            .entity_position(entity_pos)
            .build_action()
            .unwrap();

        match builder {
            PrivateAction::SynchronizeAction(ref action) => {
                assert_eq!(
                    action.master_entity_ref.as_literal(),
                    Some(&"lead".to_string())
                );
            }
            _ => panic!("Expected SynchronizeAction"),
        }
    }

    #[test]
    fn test_validation_requires_master() {
        let result = SynchronizeActionBuilder::new()
            .for_entity("ego")
            .build_action();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Master entity reference"));
    }

    #[test]
    fn test_validation_requires_positions() {
        let result = SynchronizeActionBuilder::new()
            .for_entity("ego")
            .with_master("lead")
            .build_action();

        assert!(result.is_err());
    }

    #[test]
    fn test_maneuver_action_trait() {
        let builder = SynchronizeActionBuilder::new().for_entity("test_entity");
        assert_eq!(builder.entity_ref(), Some("test_entity"));
    }
}
