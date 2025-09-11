//! Phase 4B: Controller Actions Implementation
//!
//! This file contains:
//! - Controller assignment and activation actions following OpenSCENARIO specification  
//! - Override actions for manual control (throttle, brake, steering, gear)
//! - Controller configuration and parameter setting per OpenSCENARIO XSD schema
//! - Gear control types (manual/automatic) and supporting enumerations
//!
//! Contributes to project by:
//! - Enabling realistic vehicle control through AI controllers and manual overrides
//! - Supporting simulation of different driver behaviors and capabilities
//! - Providing fine-grained control over vehicle dynamics and responses
//! - Following complete OpenSCENARIO action specification for controller management

use crate::types::basic::Boolean;
use crate::types::catalogs::entities::CatalogController;
use crate::types::catalogs::references::CatalogReference;
use crate::types::controllers::Controller;
use serde::{Deserialize, Serialize};

// PHASE 4B: Core Controller Actions Implementation

/// Assign controller action for controller assignment with catalog support
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AssignControllerAction {
    #[serde(rename = "Controller")]
    pub controller: Option<Controller>,
    #[serde(rename = "CatalogReference")]
    pub catalog_reference: Option<CatalogReference<CatalogController>>,
}

/// Activate controller action for controller activation control
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActivateControllerAction {
    #[serde(rename = "@longitudinal")]
    pub longitudinal: Option<Boolean>,
    #[serde(rename = "@lateral")]
    pub lateral: Option<Boolean>,
    #[serde(rename = "@lighting")]
    pub lighting: Option<Boolean>,
    #[serde(rename = "@animation")]
    pub animation: Option<Boolean>,
}

/// Override controller value action - composite override action
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OverrideControllerValueAction {
    #[serde(rename = "Throttle")]
    pub throttle: Option<OverrideControllerValueActionThrottle>,
    #[serde(rename = "Brake")]
    pub brake: Option<OverrideControllerValueActionBrake>,
    #[serde(rename = "Clutch")]
    pub clutch: Option<OverrideControllerValueActionClutch>,
    #[serde(rename = "ParkingBrake")]
    pub parking_brake: Option<OverrideControllerValueActionParkingBrake>,
    #[serde(rename = "SteeringWheel")]
    pub steering_wheel: Option<OverrideControllerValueActionSteeringWheel>,
    #[serde(rename = "Gear")]
    pub gear: Option<OverrideControllerValueActionGear>,
}

/// Individual Override Actions (6 types)

/// Override brake action
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OverrideControllerValueActionBrake {
    #[serde(rename = "@active")]
    pub active: Boolean,
    #[serde(rename = "@value")]
    pub value: f64,
}

/// Override throttle action
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OverrideControllerValueActionThrottle {
    #[serde(rename = "@active")]
    pub active: Boolean,
    #[serde(rename = "@value")]
    pub value: f64,
}

/// Override steering wheel action
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OverrideControllerValueActionSteeringWheel {
    #[serde(rename = "@active")]
    pub active: Boolean,
    #[serde(rename = "@value")]
    pub value: f64,
}

/// Override gear action
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OverrideControllerValueActionGear {
    #[serde(rename = "@active")]
    pub active: Boolean,
    #[serde(rename = "ManualGear")]
    pub manual_gear: Option<ManualGear>,
    #[serde(rename = "AutomaticGear")]
    pub automatic_gear: Option<AutomaticGear>,
}

/// Override parking brake action
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OverrideControllerValueActionParkingBrake {
    #[serde(rename = "@active")]
    pub active: Boolean,
    #[serde(rename = "@force")]
    pub force: Option<f64>,
}

/// Override clutch action
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OverrideControllerValueActionClutch {
    #[serde(rename = "@active")]
    pub active: Boolean,
    #[serde(rename = "@value")]
    pub value: f64,
}

// PHASE 4B: Supporting Types for Gear Control

/// Manual gear specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ManualGear {
    #[serde(rename = "@gear")]
    pub gear: i32,
}

/// Automatic gear specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AutomaticGear {
    #[serde(rename = "@gear")]
    pub gear: AutomaticGearType,
}

/// Automatic gear type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AutomaticGearType {
    #[serde(rename = "park")]
    Park,
    #[serde(rename = "reverse")]
    Reverse,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "drive")]
    Drive,
}

// PHASE 4B: Default implementations

impl Default for AssignControllerAction {
    fn default() -> Self {
        Self {
            controller: Some(Controller::default()),
            catalog_reference: None,
        }
    }
}

impl Default for ActivateControllerAction {
    fn default() -> Self {
        Self {
            longitudinal: Some(Boolean::literal(true)),
            lateral: Some(Boolean::literal(true)),
            lighting: None,
            animation: None,
        }
    }
}

impl Default for OverrideControllerValueAction {
    fn default() -> Self {
        Self {
            throttle: None,
            brake: None,
            clutch: None,
            parking_brake: None,
            steering_wheel: None,
            gear: None,
        }
    }
}

impl Default for OverrideControllerValueActionBrake {
    fn default() -> Self {
        Self {
            active: Boolean::literal(true),
            value: 0.0,
        }
    }
}

impl Default for OverrideControllerValueActionThrottle {
    fn default() -> Self {
        Self {
            active: Boolean::literal(true),
            value: 0.0,
        }
    }
}

impl Default for OverrideControllerValueActionSteeringWheel {
    fn default() -> Self {
        Self {
            active: Boolean::literal(true),
            value: 0.0,
        }
    }
}

impl Default for OverrideControllerValueActionGear {
    fn default() -> Self {
        Self {
            active: Boolean::literal(true),
            manual_gear: Some(ManualGear::default()),
            automatic_gear: None,
        }
    }
}

impl Default for OverrideControllerValueActionParkingBrake {
    fn default() -> Self {
        Self {
            active: Boolean::literal(false),
            force: Some(0.0),
        }
    }
}

impl Default for OverrideControllerValueActionClutch {
    fn default() -> Self {
        Self {
            active: Boolean::literal(true),
            value: 0.0,
        }
    }
}

impl Default for ManualGear {
    fn default() -> Self {
        Self { gear: 1 }
    }
}

impl Default for AutomaticGear {
    fn default() -> Self {
        Self {
            gear: AutomaticGearType::Drive,
        }
    }
}

impl Default for AutomaticGearType {
    fn default() -> Self {
        AutomaticGearType::Drive
    }
}

// PHASE 4B: Helper implementations

impl AssignControllerAction {
    /// Create assignment with direct controller
    pub fn with_controller(controller: Controller) -> Self {
        Self {
            controller: Some(controller),
            catalog_reference: None,
        }
    }

    /// Create assignment with catalog reference
    pub fn with_catalog_reference(catalog_reference: CatalogReference<CatalogController>) -> Self {
        Self {
            controller: None,
            catalog_reference: Some(catalog_reference),
        }
    }
}

impl ActivateControllerAction {
    /// Create activation with all control domains
    pub fn all_domains(longitudinal: bool, lateral: bool, lighting: bool, animation: bool) -> Self {
        Self {
            longitudinal: Some(Boolean::literal(longitudinal)),
            lateral: Some(Boolean::literal(lateral)),
            lighting: Some(Boolean::literal(lighting)),
            animation: Some(Boolean::literal(animation)),
        }
    }

    /// Create activation for movement only (longitudinal + lateral)
    pub fn movement_only() -> Self {
        Self {
            longitudinal: Some(Boolean::literal(true)),
            lateral: Some(Boolean::literal(true)),
            lighting: None,
            animation: None,
        }
    }
}

impl OverrideControllerValueAction {
    /// Create brake override
    pub fn brake_override(active: bool, value: f64) -> Self {
        Self {
            brake: Some(OverrideControllerValueActionBrake {
                active: Boolean::literal(active),
                value,
            }),
            throttle: None,
            clutch: None,
            parking_brake: None,
            steering_wheel: None,
            gear: None,
        }
    }

    /// Create throttle override
    pub fn throttle_override(active: bool, value: f64) -> Self {
        Self {
            throttle: Some(OverrideControllerValueActionThrottle {
                active: Boolean::literal(active),
                value,
            }),
            brake: None,
            clutch: None,
            parking_brake: None,
            steering_wheel: None,
            gear: None,
        }
    }

    /// Create steering override
    pub fn steering_override(active: bool, value: f64) -> Self {
        Self {
            steering_wheel: Some(OverrideControllerValueActionSteeringWheel {
                active: Boolean::literal(active),
                value,
            }),
            brake: None,
            throttle: None,
            clutch: None,
            parking_brake: None,
            gear: None,
        }
    }
}

impl ManualGear {
    /// Create manual gear for specific gear number
    pub fn new(gear: i32) -> Self {
        Self { gear }
    }

    /// Neutral gear
    pub fn neutral() -> Self {
        Self::new(0)
    }

    /// First gear
    pub fn first() -> Self {
        Self::new(1)
    }

    /// Reverse gear
    pub fn reverse() -> Self {
        Self::new(-1)
    }
}

impl AutomaticGear {
    /// Create automatic gear for park
    pub fn park() -> Self {
        Self {
            gear: AutomaticGearType::Park,
        }
    }

    /// Create automatic gear for reverse
    pub fn reverse() -> Self {
        Self {
            gear: AutomaticGearType::Reverse,
        }
    }

    /// Create automatic gear for neutral
    pub fn neutral() -> Self {
        Self {
            gear: AutomaticGearType::Neutral,
        }
    }

    /// Create automatic gear for drive
    pub fn drive() -> Self {
        Self {
            gear: AutomaticGearType::Drive,
        }
    }
}

// PHASE 4B: Unit tests for all controller actions
#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::controllers::Controller;

    #[test]
    fn test_assign_controller_action_creation() {
        let controller = Controller::default();
        let action = AssignControllerAction::with_controller(controller);

        assert!(action.controller.is_some());
        assert!(action.catalog_reference.is_none());
    }

    #[test]
    fn test_activate_controller_action_creation() {
        let action = ActivateControllerAction::all_domains(true, true, false, false);

        assert_eq!(action.longitudinal.unwrap().as_literal(), Some(&true));
        assert_eq!(action.lateral.unwrap().as_literal(), Some(&true));
        assert_eq!(action.lighting.unwrap().as_literal(), Some(&false));
        assert_eq!(action.animation.unwrap().as_literal(), Some(&false));
    }

    #[test]
    fn test_activate_controller_movement_only() {
        let action = ActivateControllerAction::movement_only();

        assert_eq!(action.longitudinal.unwrap().as_literal(), Some(&true));
        assert_eq!(action.lateral.unwrap().as_literal(), Some(&true));
        assert!(action.lighting.is_none());
        assert!(action.animation.is_none());
    }

    #[test]
    fn test_override_controller_brake() {
        let action = OverrideControllerValueAction::brake_override(true, 0.7);

        assert!(action.brake.is_some());
        let brake = action.brake.unwrap();
        assert_eq!(brake.active.as_literal(), Some(&true));
        assert_eq!(brake.value, 0.7);

        assert!(action.throttle.is_none());
        assert!(action.steering_wheel.is_none());
    }

    #[test]
    fn test_override_controller_throttle() {
        let action = OverrideControllerValueAction::throttle_override(true, 0.5);

        assert!(action.throttle.is_some());
        let throttle = action.throttle.unwrap();
        assert_eq!(throttle.active.as_literal(), Some(&true));
        assert_eq!(throttle.value, 0.5);
    }

    #[test]
    fn test_override_controller_steering() {
        let action = OverrideControllerValueAction::steering_override(true, 0.2);

        assert!(action.steering_wheel.is_some());
        let steering = action.steering_wheel.unwrap();
        assert_eq!(steering.active.as_literal(), Some(&true));
        assert_eq!(steering.value, 0.2);
    }

    #[test]
    fn test_manual_gear_creation() {
        let first_gear = ManualGear::first();
        assert_eq!(first_gear.gear, 1);

        let neutral = ManualGear::neutral();
        assert_eq!(neutral.gear, 0);

        let reverse = ManualGear::reverse();
        assert_eq!(reverse.gear, -1);
    }

    #[test]
    fn test_automatic_gear_creation() {
        let park = AutomaticGear::park();
        assert_eq!(park.gear, AutomaticGearType::Park);

        let drive = AutomaticGear::drive();
        assert_eq!(drive.gear, AutomaticGearType::Drive);

        let reverse = AutomaticGear::reverse();
        assert_eq!(reverse.gear, AutomaticGearType::Reverse);

        let neutral = AutomaticGear::neutral();
        assert_eq!(neutral.gear, AutomaticGearType::Neutral);
    }

    #[test]
    fn test_gear_override_action() {
        let gear_action = OverrideControllerValueActionGear {
            active: Boolean::literal(true),
            manual_gear: Some(ManualGear::first()),
            automatic_gear: None,
        };

        assert_eq!(gear_action.active.as_literal(), Some(&true));
        assert!(gear_action.manual_gear.is_some());
        assert!(gear_action.automatic_gear.is_none());
        assert_eq!(gear_action.manual_gear.unwrap().gear, 1);
    }

    #[test]
    fn test_parking_brake_action() {
        let parking_brake = OverrideControllerValueActionParkingBrake {
            active: Boolean::literal(true),
            force: Some(1.0),
        };

        assert_eq!(parking_brake.active.as_literal(), Some(&true));
        assert_eq!(parking_brake.force, Some(1.0));
    }

    #[test]
    fn test_clutch_action() {
        let clutch = OverrideControllerValueActionClutch {
            active: Boolean::literal(true),
            value: 0.5,
        };

        assert_eq!(clutch.active.as_literal(), Some(&true));
        assert_eq!(clutch.value, 0.5);
    }

    #[test]
    fn test_controller_action_defaults() {
        let assign = AssignControllerAction::default();
        assert!(assign.controller.is_some());

        let activate = ActivateControllerAction::default();
        assert_eq!(activate.longitudinal.unwrap().as_literal(), Some(&true));
        assert_eq!(activate.lateral.unwrap().as_literal(), Some(&true));

        let override_action = OverrideControllerValueAction::default();
        assert!(override_action.brake.is_none());
        assert!(override_action.throttle.is_none());
    }
}

