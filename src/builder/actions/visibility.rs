//! Visibility action builder for controlling entity visibility
//!
//! This module provides a builder for visibility actions, which control
//! whether entities are visible to graphics systems, sensors, and traffic.
//!
//! # Usage Examples
//!
//! ```rust
//! use openscenario_rs::builder::actions::visibility::VisibilityActionBuilder;
//!
//! // Make entity fully visible
//! let visible_action = VisibilityActionBuilder::new()
//!     .for_entity("ego_vehicle")
//!     .visible();
//!
//! // Make entity invisible
//! let invisible_action = VisibilityActionBuilder::new()
//!     .for_entity("ego_vehicle")
//!     .invisible();
//!
//! // Custom visibility settings
//! let custom_action = VisibilityActionBuilder::new()
//!     .for_entity("ego_vehicle")
//!     .graphics(true)
//!     .sensors(false)
//!     .traffic(true);
//! ```

use crate::builder::actions::base::{ActionBuilder, ManeuverAction};
use crate::builder::BuilderResult;
use crate::types::{
    actions::appearance::VisibilityAction,
    actions::wrappers::{CorePrivateAction, PrivateAction},
    basic::Boolean,
};

/// Builder for visibility actions
#[derive(Debug)]
pub struct VisibilityActionBuilder {
    entity_ref: Option<String>,
    graphics: bool,
    sensors: bool,
    traffic: bool,
}

impl Default for VisibilityActionBuilder {
    fn default() -> Self {
        Self {
            entity_ref: None,
            graphics: true,
            sensors: true,
            traffic: true,
        }
    }
}

impl VisibilityActionBuilder {
    /// Create new visibility action builder with all visibility enabled by default
    pub fn new() -> Self {
        Self::default()
    }

    /// Set target entity for this action
    pub fn for_entity(mut self, entity_ref: &str) -> Self {
        self.entity_ref = Some(entity_ref.to_string());
        self
    }

    /// Set graphics visibility
    pub fn graphics(mut self, visible: bool) -> Self {
        self.graphics = visible;
        self
    }

    /// Set sensor visibility
    pub fn sensors(mut self, visible: bool) -> Self {
        self.sensors = visible;
        self
    }

    /// Set traffic visibility
    pub fn traffic(mut self, visible: bool) -> Self {
        self.traffic = visible;
        self
    }

    /// Make entity fully visible (convenience method)
    pub fn visible(mut self) -> Self {
        self.graphics = true;
        self.sensors = true;
        self.traffic = true;
        self
    }

    /// Make entity fully invisible (convenience method)
    pub fn invisible(mut self) -> Self {
        self.graphics = false;
        self.sensors = false;
        self.traffic = false;
        self
    }
}

impl ActionBuilder for VisibilityActionBuilder {
    fn build_action(self) -> BuilderResult<PrivateAction> {
        let action = VisibilityAction {
            graphics: Boolean::literal(self.graphics),
            sensors: Boolean::literal(self.sensors),
            traffic: Boolean::literal(self.traffic),
            sensor_reference_set: None,
        };

        Ok(PrivateAction {
            action: CorePrivateAction::VisibilityAction(action),
        })
    }

    fn validate(&self) -> BuilderResult<()> {
        Ok(()) // No validation needed
    }
}

impl ManeuverAction for VisibilityActionBuilder {
    fn entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visibility_visible() {
        let builder = VisibilityActionBuilder::new()
            .for_entity("ego")
            .visible()
            .build_action()
            .unwrap();

        match builder.action {
            CorePrivateAction::VisibilityAction(ref action) => {
                assert_eq!(action.graphics.as_literal(), Some(&true));
                assert_eq!(action.sensors.as_literal(), Some(&true));
                assert_eq!(action.traffic.as_literal(), Some(&true));
            }
            _ => panic!("Expected VisibilityAction"),
        }
    }

    #[test]
    fn test_visibility_invisible() {
        let builder = VisibilityActionBuilder::new()
            .for_entity("ego")
            .invisible()
            .build_action()
            .unwrap();

        match builder.action {
            CorePrivateAction::VisibilityAction(ref action) => {
                assert_eq!(action.graphics.as_literal(), Some(&false));
                assert_eq!(action.sensors.as_literal(), Some(&false));
                assert_eq!(action.traffic.as_literal(), Some(&false));
            }
            _ => panic!("Expected VisibilityAction"),
        }
    }

    #[test]
    fn test_visibility_custom() {
        let builder = VisibilityActionBuilder::new()
            .for_entity("ego")
            .graphics(true)
            .sensors(false)
            .traffic(true)
            .build_action()
            .unwrap();

        match builder.action {
            CorePrivateAction::VisibilityAction(ref action) => {
                assert_eq!(action.graphics.as_literal(), Some(&true));
                assert_eq!(action.sensors.as_literal(), Some(&false));
                assert_eq!(action.traffic.as_literal(), Some(&true));
            }
            _ => panic!("Expected VisibilityAction"),
        }
    }

    #[test]
    fn test_default_visibility() {
        let builder = VisibilityActionBuilder::new()
            .for_entity("ego")
            .build_action()
            .unwrap();

        match builder.action {
            CorePrivateAction::VisibilityAction(ref action) => {
                // Default is all visible
                assert_eq!(action.graphics.as_literal(), Some(&true));
                assert_eq!(action.sensors.as_literal(), Some(&true));
                assert_eq!(action.traffic.as_literal(), Some(&true));
            }
            _ => panic!("Expected VisibilityAction"),
        }
    }

    #[test]
    fn test_maneuver_action_trait() {
        let builder = VisibilityActionBuilder::new().for_entity("test_entity");
        assert_eq!(builder.entity_ref(), Some("test_entity"));
    }
}
