//! Longitudinal action builders for distance keeping and speed profiles
//!
//! This module provides builders for actions that control longitudinal movement,
//! including maintaining distance to other entities and executing time-based speed profiles.
//!
//! # Available Builders
//!
//! - [`LongitudinalDistanceActionBuilder`] - Maintain longitudinal distance to a leading entity
//! - [`SpeedProfileActionBuilder`] - Define multi-point speed profile over time
//!
//! # Usage Examples
//!
//! ```rust
//! use openscenario_rs::builder::actions::longitudinal::{
//!     LongitudinalDistanceActionBuilder, SpeedProfileActionBuilder
//! };
//!
//! // Create a longitudinal distance action to maintain 10m from lead vehicle
//! let distance_action = LongitudinalDistanceActionBuilder::new()
//!     .for_entity("ego_vehicle")
//!     .from_entity("lead_vehicle")
//!     .at_distance(10.0)
//!     .with_freespace(true);
//!
//! // Create a speed profile action with multiple time/speed points
//! let profile_action = SpeedProfileActionBuilder::new()
//!     .for_entity("ego_vehicle")
//!     .add_entry_direct(0.0, 0.0)
//!     .add_entry_direct(5.0, 30.0)
//!     .add_entry_direct(10.0, 50.0);
//! ```

use crate::builder::actions::base::{ActionBuilder, ManeuverAction};
use crate::builder::{BuilderError, BuilderResult};
use crate::types::{
    actions::movement::{
        LongitudinalAction, LongitudinalActionChoice, LongitudinalDistanceAction,
        SpeedProfileAction, SpeedProfileEntry,
    },
    actions::wrappers::PrivateAction,
    basic::{Boolean, Double, OSString},
};

/// Builder for longitudinal distance actions
#[derive(Debug, Default)]
pub struct LongitudinalDistanceActionBuilder {
    entity_ref: Option<String>,
    target_entity: Option<String>,
    distance: Option<f64>,
    time_gap: Option<f64>,
    freespace: bool,
    continuous: bool,
}

impl LongitudinalDistanceActionBuilder {
    /// Create new longitudinal distance action builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set target entity for this action
    pub fn for_entity(mut self, entity_ref: &str) -> Self {
        self.entity_ref = Some(entity_ref.to_string());
        self
    }

    /// Set the target entity to maintain distance from
    pub fn from_entity(mut self, target_entity: &str) -> Self {
        self.target_entity = Some(target_entity.to_string());
        self
    }

    /// Set fixed distance value (mutually exclusive with time_gap)
    pub fn at_distance(mut self, distance: f64) -> Self {
        self.distance = Some(distance);
        self.time_gap = None; // Mutually exclusive
        self
    }

    /// Set time gap value (mutually exclusive with distance)
    pub fn with_time_gap(mut self, time_gap: f64) -> Self {
        self.time_gap = Some(time_gap);
        self.distance = None; // Mutually exclusive
        self
    }

    /// Set whether to use freespace measurement
    pub fn with_freespace(mut self, freespace: bool) -> Self {
        self.freespace = freespace;
        self
    }

    /// Set whether the action is continuous
    pub fn continuous(mut self, continuous: bool) -> Self {
        self.continuous = continuous;
        self
    }
}

impl ActionBuilder for LongitudinalDistanceActionBuilder {
    fn build_action(self) -> BuilderResult<PrivateAction> {
        self.validate()?;

        let action = LongitudinalDistanceAction {
            entity_ref: OSString::literal(self.target_entity.unwrap()),
            distance: self.distance.map(Double::literal),
            time_gap: self.time_gap.map(Double::literal),
            coordinate_system: None,
            displacement: None,
            freespace: Some(Boolean::literal(self.freespace)),
            continuous: Boolean::literal(self.continuous),
            dynamic_constraints: None,
        };

        Ok(PrivateAction::LongitudinalAction(LongitudinalAction {
            longitudinal_action_choice: LongitudinalActionChoice::LongitudinalDistanceAction(
                action,
            ),
        }))
    }

    fn validate(&self) -> BuilderResult<()> {
        if self.target_entity.is_none() {
            return Err(BuilderError::validation_error(
                "Target entity is required for longitudinal distance action",
            ));
        }

        if self.distance.is_none() && self.time_gap.is_none() {
            return Err(BuilderError::validation_error(
                "Either distance or time_gap must be specified",
            ));
        }

        Ok(())
    }
}

impl ManeuverAction for LongitudinalDistanceActionBuilder {
    fn entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

/// Builder for speed profile actions
#[derive(Debug, Default)]
pub struct SpeedProfileActionBuilder {
    entity_ref: Option<String>,
    entries: Vec<SpeedProfileEntry>,
}

impl SpeedProfileActionBuilder {
    /// Create new speed profile action builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set target entity for this action
    pub fn for_entity(mut self, entity_ref: &str) -> Self {
        self.entity_ref = Some(entity_ref.to_string());
        self
    }

    /// Add a speed profile entry using builder pattern
    pub fn add_entry(self) -> SpeedProfileEntryBuilder {
        SpeedProfileEntryBuilder::new(self)
    }

    /// Add a speed profile entry directly (convenience method)
    pub fn add_entry_direct(mut self, time: f64, speed: f64) -> Self {
        self.entries.push(SpeedProfileEntry {
            time: Double::literal(time),
            speed: Double::literal(speed),
        });
        self
    }
}

impl ActionBuilder for SpeedProfileActionBuilder {
    fn build_action(self) -> BuilderResult<PrivateAction> {
        self.validate()?;

        let action = SpeedProfileAction {
            entity_ref: self
                .entity_ref
                .as_ref()
                .map(|s| OSString::literal(s.clone())),
            entries: self.entries,
            dynamic_constraints: None,
        };

        Ok(PrivateAction::LongitudinalAction(LongitudinalAction {
            longitudinal_action_choice: LongitudinalActionChoice::SpeedProfileAction(action),
        }))
    }

    fn validate(&self) -> BuilderResult<()> {
        if self.entries.len() < 2 {
            return Err(BuilderError::validation_error(
                "Speed profile requires at least 2 entries",
            ));
        }

        // Verify chronological order
        for i in 1..self.entries.len() {
            let prev_time = self.entries[i - 1].time.as_literal().unwrap();
            let curr_time = self.entries[i].time.as_literal().unwrap();
            if curr_time <= prev_time {
                return Err(BuilderError::validation_error(
                    "Speed profile entries must be in chronological order",
                ));
            }
        }

        Ok(())
    }
}

impl ManeuverAction for SpeedProfileActionBuilder {
    fn entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

/// Builder for individual speed profile entries
pub struct SpeedProfileEntryBuilder {
    parent: SpeedProfileActionBuilder,
    time: Option<f64>,
    speed: Option<f64>,
}

impl SpeedProfileEntryBuilder {
    fn new(parent: SpeedProfileActionBuilder) -> Self {
        Self {
            parent,
            time: None,
            speed: None,
        }
    }

    /// Set the time for this entry
    pub fn at_time(mut self, time: f64) -> Self {
        self.time = Some(time);
        self
    }

    /// Set the speed for this entry
    pub fn with_speed(mut self, speed: f64) -> Self {
        self.speed = Some(speed);
        self
    }

    /// Finish building this entry and return to parent builder
    pub fn finish(mut self) -> BuilderResult<SpeedProfileActionBuilder> {
        if self.time.is_none() {
            return Err(BuilderError::validation_error("Entry time is required"));
        }
        if self.speed.is_none() {
            return Err(BuilderError::validation_error("Entry speed is required"));
        }

        let entry = SpeedProfileEntry {
            time: Double::literal(self.time.unwrap()),
            speed: Double::literal(self.speed.unwrap()),
        };

        self.parent.entries.push(entry);
        Ok(self.parent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longitudinal_distance_basic() {
        let builder = LongitudinalDistanceActionBuilder::new()
            .for_entity("ego")
            .from_entity("lead")
            .at_distance(10.0)
            .build_action()
            .unwrap();

        // Verify structure
        match builder {
            PrivateAction::LongitudinalAction(ref action) => {
                match &action.longitudinal_action_choice {
                    LongitudinalActionChoice::LongitudinalDistanceAction(ref dist) => {
                        assert_eq!(dist.distance.as_ref().unwrap().as_literal(), Some(&10.0));
                        assert_eq!(dist.entity_ref.as_literal(), Some(&"lead".to_string()));
                    }
                    _ => panic!("Expected LongitudinalDistanceAction"),
                }
            }
            _ => panic!("Expected LongitudinalAction"),
        }
    }

    #[test]
    fn test_longitudinal_distance_time_gap() {
        let builder = LongitudinalDistanceActionBuilder::new()
            .for_entity("ego")
            .from_entity("lead")
            .with_time_gap(2.5)
            .build_action()
            .unwrap();

        match builder {
            PrivateAction::LongitudinalAction(ref action) => {
                match &action.longitudinal_action_choice {
                    LongitudinalActionChoice::LongitudinalDistanceAction(ref dist) => {
                        assert_eq!(dist.time_gap.as_ref().unwrap().as_literal(), Some(&2.5));
                        assert!(dist.distance.is_none());
                    }
                    _ => panic!("Expected LongitudinalDistanceAction"),
                }
            }
            _ => panic!("Expected LongitudinalAction"),
        }
    }

    #[test]
    fn test_validation_requires_target() {
        let result = LongitudinalDistanceActionBuilder::new()
            .at_distance(10.0)
            .build_action();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Target entity"));
    }

    #[test]
    fn test_validation_requires_distance_or_time_gap() {
        let result = LongitudinalDistanceActionBuilder::new()
            .from_entity("lead")
            .build_action();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("distance or time_gap"));
    }

    #[test]
    fn test_speed_profile_basic() {
        let builder = SpeedProfileActionBuilder::new()
            .for_entity("ego")
            .add_entry_direct(0.0, 0.0)
            .add_entry_direct(5.0, 30.0)
            .add_entry_direct(10.0, 50.0)
            .build_action()
            .unwrap();

        match builder {
            PrivateAction::LongitudinalAction(ref action) => {
                match &action.longitudinal_action_choice {
                    LongitudinalActionChoice::SpeedProfileAction(ref profile) => {
                        assert_eq!(profile.entries.len(), 3);
                        assert_eq!(profile.entries[0].time.as_literal(), Some(&0.0));
                        assert_eq!(profile.entries[0].speed.as_literal(), Some(&0.0));
                        assert_eq!(profile.entries[2].time.as_literal(), Some(&10.0));
                        assert_eq!(profile.entries[2].speed.as_literal(), Some(&50.0));
                    }
                    _ => panic!("Expected SpeedProfileAction"),
                }
            }
            _ => panic!("Expected LongitudinalAction"),
        }
    }

    #[test]
    fn test_speed_profile_entry_builder() {
        let builder = SpeedProfileActionBuilder::new()
            .for_entity("ego")
            .add_entry()
            .at_time(0.0)
            .with_speed(0.0)
            .finish()
            .unwrap()
            .add_entry()
            .at_time(5.0)
            .with_speed(25.0)
            .finish()
            .unwrap()
            .build_action()
            .unwrap();

        match builder {
            PrivateAction::LongitudinalAction(ref action) => {
                match &action.longitudinal_action_choice {
                    LongitudinalActionChoice::SpeedProfileAction(ref profile) => {
                        assert_eq!(profile.entries.len(), 2);
                    }
                    _ => panic!("Expected SpeedProfileAction"),
                }
            }
            _ => panic!("Expected LongitudinalAction"),
        }
    }

    #[test]
    fn test_speed_profile_validation_min_entries() {
        let result = SpeedProfileActionBuilder::new()
            .for_entity("ego")
            .add_entry_direct(0.0, 0.0)
            .build_action();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("at least 2 entries"));
    }

    #[test]
    fn test_speed_profile_validation_chronological() {
        let result = SpeedProfileActionBuilder::new()
            .for_entity("ego")
            .add_entry_direct(5.0, 30.0)
            .add_entry_direct(3.0, 20.0) // Out of order
            .build_action();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("chronological order"));
    }

    #[test]
    fn test_maneuver_action_trait() {
        let builder = LongitudinalDistanceActionBuilder::new().for_entity("ego");
        assert_eq!(builder.entity_ref(), Some("ego"));

        let profile_builder = SpeedProfileActionBuilder::new().for_entity("test_entity");
        assert_eq!(profile_builder.entity_ref(), Some("test_entity"));
    }
}
