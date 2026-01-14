//! Movement action builders (SpeedAction, TeleportAction, etc.)
//!
//! This module provides builders for actions that control entity movement,
//! including speed changes and position updates.
//!
//! # Available Builders
//!
//! - [`SpeedActionBuilder`] - Control entity speed (absolute speed targets)
//! - [`TeleportActionBuilder`] - Instantly move entities to new positions
//!
//! # Usage Examples
//!
//! ```rust
//! use openscenario_rs::builder::actions::movement::{SpeedActionBuilder, TeleportActionBuilder};
//!
//! // Create a speed action to set vehicle to 25 m/s
//! let speed_action = SpeedActionBuilder::new()
//!     .for_entity("ego_vehicle")
//!     .to_speed(25.0);
//!
//! // Teleport vehicle to specific world coordinates
//! let teleport_action = TeleportActionBuilder::new()
//!     .for_entity("ego_vehicle")
//!     .to_world_position(100.0, 200.0, 0.0);
//! ```

use crate::builder::actions::base::{ActionBuilder, ManeuverAction};
use crate::builder::positions::PositionBuilder;
use crate::builder::{BuilderError, BuilderResult};
use crate::types::{
    actions::movement::{
        AbsoluteTargetSpeed, LongitudinalAction, LongitudinalActionChoice, SpeedAction,
        SpeedActionTarget, TeleportAction, TransitionDynamics,
    },
    actions::wrappers::{PrivateAction, PrivateAction},
    basic::Double,
    enums::{DynamicsDimension, DynamicsShape},
    positions::Position,
};

/// Builder for speed actions
#[derive(Debug, Default)]
pub struct SpeedActionBuilder {
    entity_ref: Option<String>,
    target_speed: Option<f64>,
}

impl SpeedActionBuilder {
    /// Create new speed action builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set target entity for this action
    pub fn for_entity(mut self, entity_ref: &str) -> Self {
        self.entity_ref = Some(entity_ref.to_string());
        self
    }

    /// Set absolute target speed
    pub fn to_speed(mut self, speed: f64) -> Self {
        self.target_speed = Some(speed);
        self
    }

    /// Set relative speed change
    pub fn change_by(mut self, delta: f64) -> Self {
        self.target_speed = Some(delta);
        self
    }
}

impl ActionBuilder for SpeedActionBuilder {
    fn build_action(self) -> BuilderResult<PrivateAction> {
        self.validate()?;

        let speed_action = SpeedAction {
            speed_action_dynamics: TransitionDynamics {
                dynamics_dimension: DynamicsDimension::Time,
                dynamics_shape: DynamicsShape::Linear,
                value: Double::literal(1.0),
            },
            speed_action_target: SpeedActionTarget {
                absolute: Some(AbsoluteTargetSpeed {
                    value: Double::literal(self.target_speed.unwrap()),
                }),
                relative: None,
            },
        };

        Ok(PrivateAction {
            action: PrivateAction::LongitudinalAction(LongitudinalAction {
                longitudinal_action_choice: LongitudinalActionChoice::SpeedAction(speed_action),
            }),
        })
    }

    fn validate(&self) -> BuilderResult<()> {
        if self.target_speed.is_none() {
            return Err(BuilderError::validation_error("Target speed is required"));
        }
        Ok(())
    }
}

impl ManeuverAction for SpeedActionBuilder {
    fn entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

/// Builder for teleport actions
#[derive(Debug, Default)]
pub struct TeleportActionBuilder {
    entity_ref: Option<String>,
    position: Option<Position>,
}

impl TeleportActionBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn for_entity(mut self, entity_ref: &str) -> Self {
        self.entity_ref = Some(entity_ref.to_string());
        self
    }

    pub fn to_position(mut self, position: Position) -> Self {
        self.position = Some(position);
        self
    }

    /// Start position configuration (integrate with existing position builders)
    pub fn to(self) -> TeleportPositionBuilder {
        TeleportPositionBuilder::new(self)
    }
}

impl ActionBuilder for TeleportActionBuilder {
    fn build_action(self) -> BuilderResult<PrivateAction> {
        self.validate()?;

        let teleport_action = TeleportAction {
            position: self.position.unwrap(),
        };

        Ok(PrivateAction {
            action: PrivateAction::TeleportAction(teleport_action),
        })
    }

    fn validate(&self) -> BuilderResult<()> {
        if self.position.is_none() {
            return Err(BuilderError::validation_error(
                "Position is required for teleport action",
            ));
        }
        Ok(())
    }
}

impl ManeuverAction for TeleportActionBuilder {
    fn entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

/// Position builder for teleport actions
pub struct TeleportPositionBuilder {
    parent: TeleportActionBuilder,
}

impl TeleportPositionBuilder {
    fn new(parent: TeleportActionBuilder) -> Self {
        Self { parent }
    }

    /// Use world coordinates
    pub fn world_position(mut self, x: f64, y: f64, z: f64) -> TeleportActionBuilder {
        // Use existing position builders
        let position = crate::builder::positions::WorldPositionBuilder::new()
            .x(x)
            .y(y)
            .z(z)
            .finish()
            .unwrap(); // TODO: Better error handling

        self.parent.position = Some(position);
        self.parent
    }

    /// Use lane coordinates
    pub fn lane_position(mut self, road_id: &str, lane_id: &str, s: f64) -> TeleportActionBuilder {
        let position = crate::builder::positions::LanePositionBuilder::new()
            .road(road_id)
            .lane(lane_id)
            .s(s)
            .offset(0.0)
            .finish()
            .unwrap(); // TODO: Better error handling

        self.parent.position = Some(position);
        self.parent
    }
}
