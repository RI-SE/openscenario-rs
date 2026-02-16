//! Phase 4B: Controller Actions Implementation
//!
//! This file contains:
//! - Controller assignment and activation actions following OpenSCENARIO specification  
//! - Override actions for manual control (throttle, brake, steering, gear)
//! - Controller configuration and parameter setting per OpenSCENARIO XSD schema
//! - Gear control types (manual/automatic) and supporting enumerations
//!
use crate::types::basic::{Boolean, Double, Int};
use crate::types::catalogs::entities::CatalogController;
use crate::types::catalogs::references::CatalogReference;
use crate::types::controllers::Controller;
use serde::{Deserialize, Serialize};

// PHASE 4B: Core Controller Actions Implementation

/// Main controller action wrapper containing all controller action types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[derive(Default)]
pub struct ControllerAction {
    /// Assign controller action
    #[serde(
        rename = "AssignControllerAction",
        skip_serializing_if = "Option::is_none"
    )]
    pub assign_controller_action: Option<AssignControllerAction>,

    /// Override throttle action
    #[serde(
        rename = "OverrideThrottleAction",
        skip_serializing_if = "Option::is_none"
    )]
    pub override_throttle_action: Option<OverrideThrottleAction>,

    /// Override brake action
    #[serde(
        rename = "OverrideBrakeAction",
        skip_serializing_if = "Option::is_none"
    )]
    pub override_brake_action: Option<OverrideBrakeAction>,

    /// Override clutch action
    #[serde(
        rename = "OverrideClutchAction",
        skip_serializing_if = "Option::is_none"
    )]
    pub override_clutch_action: Option<OverrideClutchAction>,

    /// Override parking brake action
    #[serde(
        rename = "OverrideParkingBrakeAction",
        skip_serializing_if = "Option::is_none"
    )]
    pub override_parking_brake_action: Option<OverrideParkingBrakeAction>,

    /// Override steering wheel action
    #[serde(
        rename = "OverrideSteeringWheelAction",
        skip_serializing_if = "Option::is_none"
    )]
    pub override_steering_wheel_action: Option<OverrideSteeringWheelAction>,

    /// Override gear action
    #[serde(rename = "OverrideGearAction", skip_serializing_if = "Option::is_none")]
    pub override_gear_action: Option<OverrideGearAction>,

    /// Activate controller (deprecated but still supported)
    #[serde(
        rename = "ActivateControllerAction",
        skip_serializing_if = "Option::is_none"
    )]
    pub activate_controller_action: Option<ActivateControllerAction>,
}

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
    #[serde(rename = "@longitudinal", skip_serializing_if = "Option::is_none")]
    pub longitudinal: Option<Boolean>,
    #[serde(rename = "@lateral", skip_serializing_if = "Option::is_none")]
    pub lateral: Option<Boolean>,
    #[serde(rename = "@lighting", skip_serializing_if = "Option::is_none")]
    pub lighting: Option<Boolean>,
    #[serde(rename = "@animation", skip_serializing_if = "Option::is_none")]
    pub animation: Option<Boolean>,
}

// Individual Override Actions matching XSD schema names

/// Override brake action (XSD compliant name)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OverrideBrakeAction {
    #[serde(rename = "@active")]
    pub active: Boolean,
    #[serde(rename = "@value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Double>, // deprecated
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub brake_input: Option<BrakeInput>,
}

/// Override throttle action (XSD compliant name)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OverrideThrottleAction {
    #[serde(rename = "@active")]
    pub active: Boolean,
    #[serde(rename = "@value")]
    pub value: Double,
    #[serde(rename = "@maxRate", skip_serializing_if = "Option::is_none")]
    pub max_rate: Option<Double>,
}

/// Override steering wheel action (XSD compliant name)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OverrideSteeringWheelAction {
    #[serde(rename = "@active")]
    pub active: Boolean,
    #[serde(rename = "@value")]
    pub value: Double,
    #[serde(rename = "@maxRate", skip_serializing_if = "Option::is_none")]
    pub max_rate: Option<Double>,
    #[serde(rename = "@maxTorque", skip_serializing_if = "Option::is_none")]
    pub max_torque: Option<Double>,
}

/// Override gear action (XSD compliant name)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OverrideGearAction {
    #[serde(rename = "@active")]
    pub active: Boolean,
    #[serde(rename = "@number", skip_serializing_if = "Option::is_none")]
    pub number: Option<Double>, // deprecated
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub gear: Option<Gear>,
}

/// Override parking brake action (XSD compliant name)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OverrideParkingBrakeAction {
    #[serde(rename = "@active")]
    pub active: Boolean,
    #[serde(rename = "@value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Double>, // deprecated
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub brake_input: Option<BrakeInput>,
}

/// Override clutch action (XSD compliant name)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OverrideClutchAction {
    #[serde(rename = "@active")]
    pub active: Boolean,
    #[serde(rename = "@value")]
    pub value: Double,
    #[serde(rename = "@maxRate", skip_serializing_if = "Option::is_none")]
    pub max_rate: Option<Double>,
}

// PHASE 4B: Supporting Types for Gear Control

/// Manual gear specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ManualGear {
    #[serde(rename = "@gear")]
    pub gear: Int,
}

/// Automatic gear specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AutomaticGear {
    #[serde(rename = "@gear")]
    pub gear: AutomaticGearType,
}

/// Automatic gear type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[derive(Default)]
pub enum AutomaticGearType {
    #[serde(rename = "park")]
    Park,
    #[serde(rename = "reverse")]
    Reverse,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "drive")]
    #[default]
    Drive,
}

/// Base brake type for brake input groups
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Brake {
    #[serde(rename = "@value")]
    pub value: Double,
}

/// BrakeInput group - XSD group wrapper for brake percent/force choice
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BrakeInput {
    #[serde(rename = "BrakePercent")]
    BrakePercent(Brake),
    #[serde(rename = "BrakeForce")]
    BrakeForce(Brake),
}

/// Gear group - XSD group wrapper for manual/automatic gear choice
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Gear {
    #[serde(rename = "ManualGear")]
    ManualGear(ManualGear),
    #[serde(rename = "AutomaticGear")]
    AutomaticGear(AutomaticGear),
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
            lighting: Some(Boolean::literal(false)),
            animation: Some(Boolean::literal(false)),
        }
    }
}

impl Default for ManualGear {
    fn default() -> Self {
        Self {
            gear: Int::literal(1),
        }
    }
}

impl Default for AutomaticGear {
    fn default() -> Self {
        Self {
            gear: AutomaticGearType::Drive,
        }
    }
}


impl Default for Brake {
    fn default() -> Self {
        Self {
            value: Double::literal(0.0),
        }
    }
}

impl Default for BrakeInput {
    fn default() -> Self {
        Self::BrakePercent(Brake::default())
    }
}

impl Default for Gear {
    fn default() -> Self {
        Self::AutomaticGear(AutomaticGear::default())
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

impl ManualGear {
    /// Create manual gear for specific gear number
    pub fn new(gear: i32) -> Self {
        Self {
            gear: Int::literal(gear),
        }
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

impl Gear {
    /// Create manual gear wrapper
    pub fn manual(gear: i32) -> Self {
        Self::ManualGear(ManualGear::new(gear))
    }

    /// Create automatic gear wrapper
    pub fn automatic(gear_type: AutomaticGearType) -> Self {
        Self::AutomaticGear(AutomaticGear { gear: gear_type })
    }

    /// Create manual first gear
    pub fn manual_first() -> Self {
        Self::ManualGear(ManualGear::first())
    }

    /// Create manual neutral gear
    pub fn manual_neutral() -> Self {
        Self::ManualGear(ManualGear::neutral())
    }

    /// Create manual reverse gear
    pub fn manual_reverse() -> Self {
        Self::ManualGear(ManualGear::reverse())
    }

    /// Create automatic park gear
    pub fn automatic_park() -> Self {
        Self::AutomaticGear(AutomaticGear::park())
    }

    /// Create automatic drive gear
    pub fn automatic_drive() -> Self {
        Self::AutomaticGear(AutomaticGear::drive())
    }

    /// Create automatic reverse gear
    pub fn automatic_reverse() -> Self {
        Self::AutomaticGear(AutomaticGear::reverse())
    }

    /// Create automatic neutral gear
    pub fn automatic_neutral() -> Self {
        Self::AutomaticGear(AutomaticGear::neutral())
    }
}

impl Brake {
    /// Create brake with specific value
    pub fn new(value: f64) -> Self {
        Self {
            value: Double::literal(value),
        }
    }

    /// Create zero brake
    pub fn zero() -> Self {
        Self::new(0.0)
    }

    /// Create full brake
    pub fn full() -> Self {
        Self::new(1.0)
    }
}

impl BrakeInput {
    /// Create brake percent input
    pub fn percent(value: f64) -> Self {
        Self::BrakePercent(Brake::new(value))
    }

    /// Create brake force input
    pub fn force(value: f64) -> Self {
        Self::BrakeForce(Brake::new(value))
    }

    /// Get brake value regardless of type
    pub fn value(&self) -> &Double {
        match self {
            Self::BrakePercent(brake) => &brake.value,
            Self::BrakeForce(brake) => &brake.value,
        }
    }

    /// Check if this is a percent-based brake input
    pub fn is_percent(&self) -> bool {
        matches!(self, Self::BrakePercent(_))
    }

    /// Check if this is a force-based brake input
    pub fn is_force(&self) -> bool {
        matches!(self, Self::BrakeForce(_))
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
    fn test_activate_controller_default_serialization() {
        let action = ActivateControllerAction::default();
        let xml = quick_xml::se::to_string(&action).expect("Serialization should succeed");

        // Should contain explicit boolean values, not empty strings
        assert!(xml.contains("longitudinal=\"true\""));
        assert!(xml.contains("lateral=\"true\""));
        assert!(xml.contains("lighting=\"false\""));
        assert!(xml.contains("animation=\"false\""));
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
    fn test_manual_gear_creation() {
        let first_gear = ManualGear::first();
        assert_eq!(first_gear.gear.as_literal().unwrap(), &1);

        let neutral = ManualGear::neutral();
        assert_eq!(neutral.gear.as_literal().unwrap(), &0);

        let reverse = ManualGear::reverse();
        assert_eq!(reverse.gear.as_literal().unwrap(), &(-1));
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
    fn test_controller_action_defaults() {
        let assign = AssignControllerAction::default();
        assert!(assign.controller.is_some());

        let activate = ActivateControllerAction::default();
        assert_eq!(activate.longitudinal.unwrap().as_literal(), Some(&true));
        assert_eq!(activate.lateral.unwrap().as_literal(), Some(&true));

        let controller_action = ControllerAction::default();
        assert!(controller_action.assign_controller_action.is_none());
        assert!(controller_action.override_brake_action.is_none());
        assert!(controller_action.override_throttle_action.is_none());
    }

    // Tests for new group types
    #[test]
    fn test_brake_creation_and_helpers() {
        let brake = Brake::new(0.5);
        assert_eq!(brake.value.as_literal().unwrap(), &0.5);

        let zero_brake = Brake::zero();
        assert_eq!(zero_brake.value.as_literal().unwrap(), &0.0);

        let full_brake = Brake::full();
        assert_eq!(full_brake.value.as_literal().unwrap(), &1.0);

        let default_brake = Brake::default();
        assert_eq!(default_brake.value.as_literal().unwrap(), &0.0);
    }

    #[test]
    fn test_brake_input_group() {
        let percent_brake = BrakeInput::percent(0.7);
        assert!(percent_brake.is_percent());
        assert!(!percent_brake.is_force());
        assert_eq!(percent_brake.value().as_literal(), Some(&0.7));

        let force_brake = BrakeInput::force(500.0);
        assert!(!force_brake.is_percent());
        assert!(force_brake.is_force());
        assert_eq!(force_brake.value().as_literal(), Some(&500.0));

        let default_brake_input = BrakeInput::default();
        assert!(default_brake_input.is_percent());
        assert_eq!(default_brake_input.value().as_literal(), Some(&0.0));
    }

    #[test]
    fn test_gear_group_creation() {
        let manual_gear = Gear::manual(3);
        if let Gear::ManualGear(gear) = manual_gear {
            assert_eq!(gear.gear.as_literal(), Some(&3));
        } else {
            panic!("Expected ManualGear variant");
        }

        let auto_gear = Gear::automatic(AutomaticGearType::Park);
        if let Gear::AutomaticGear(gear) = auto_gear {
            assert_eq!(gear.gear, AutomaticGearType::Park);
        } else {
            panic!("Expected AutomaticGear variant");
        }
    }

    #[test]
    fn test_gear_group_convenience_methods() {
        let manual_first = Gear::manual_first();
        if let Gear::ManualGear(gear) = manual_first {
            assert_eq!(gear.gear.as_literal(), Some(&1));
        } else {
            panic!("Expected ManualGear variant");
        }

        let manual_neutral = Gear::manual_neutral();
        if let Gear::ManualGear(gear) = manual_neutral {
            assert_eq!(gear.gear.as_literal(), Some(&0));
        } else {
            panic!("Expected ManualGear variant");
        }

        let manual_reverse = Gear::manual_reverse();
        if let Gear::ManualGear(gear) = manual_reverse {
            assert_eq!(gear.gear.as_literal(), Some(&(-1)));
        } else {
            panic!("Expected ManualGear variant");
        }

        let auto_park = Gear::automatic_park();
        if let Gear::AutomaticGear(gear) = auto_park {
            assert_eq!(gear.gear, AutomaticGearType::Park);
        } else {
            panic!("Expected AutomaticGear variant");
        }

        let auto_drive = Gear::automatic_drive();
        if let Gear::AutomaticGear(gear) = auto_drive {
            assert_eq!(gear.gear, AutomaticGearType::Drive);
        } else {
            panic!("Expected AutomaticGear variant");
        }
    }

    #[test]
    fn test_gear_group_default() {
        let default_gear = Gear::default();
        if let Gear::AutomaticGear(gear) = default_gear {
            assert_eq!(gear.gear, AutomaticGearType::Drive);
        } else {
            panic!("Expected AutomaticGear variant as default");
        }
    }
}
