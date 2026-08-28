//! Controller actions implementation
//!
//! This file contains:
//! - Controller assignment and activation actions following OpenSCENARIO specification  
//! - Override actions for manual control (throttle, brake, steering, gear)
//! - Controller configuration and parameter setting per OpenSCENARIO XSD schema
//! - Gear control types (manual/automatic) and supporting enumerations
//!
use crate::types::basic::{Boolean, Double, Int, OSString};
use crate::types::catalogs::entities::CatalogController;
use crate::types::catalogs::references::CatalogReference;
use crate::types::controllers::{Controller, ObjectController};
use serde::{Deserialize, Serialize};


/// Main controller action wrapper containing all controller action types
///
/// XSD `ControllerAction` (:978-984): a 3-way choice of `AssignControllerAction`
/// | `OverrideControllerValueAction` | `ActivateControllerAction`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[derive(Default)]
pub struct ControllerAction {
    /// Assign controller action
    #[serde(
        rename = "AssignControllerAction",
        skip_serializing_if = "Option::is_none"
    )]
    pub assign_controller_action: Option<AssignControllerAction>,

    /// Override controller value action
    #[serde(
        rename = "OverrideControllerValueAction",
        skip_serializing_if = "Option::is_none"
    )]
    pub override_controller_value_action: Option<OverrideControllerValueAction>,

    /// Activate controller action (deprecated in OpenSCENARIO 1.2)
    #[serde(
        rename = "ActivateControllerAction",
        skip_serializing_if = "Option::is_none"
    )]
    pub activate_controller_action: Option<ActivateControllerAction>,
}

/// XSD `OverrideControllerValueAction` (:1565-1574): `xsd:all` of six optional
/// children, whose element names differ from their type names.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OverrideControllerValueAction {
    /// `<Throttle>` element of type `OverrideThrottleAction`
    #[serde(rename = "Throttle", skip_serializing_if = "Option::is_none")]
    pub throttle: Option<OverrideThrottleAction>,

    /// `<Brake>` element of type `OverrideBrakeAction`
    #[serde(rename = "Brake", skip_serializing_if = "Option::is_none")]
    pub brake: Option<OverrideBrakeAction>,

    /// `<Clutch>` element of type `OverrideClutchAction`
    #[serde(rename = "Clutch", skip_serializing_if = "Option::is_none")]
    pub clutch: Option<OverrideClutchAction>,

    /// `<ParkingBrake>` element of type `OverrideParkingBrakeAction`
    #[serde(rename = "ParkingBrake", skip_serializing_if = "Option::is_none")]
    pub parking_brake: Option<OverrideParkingBrakeAction>,

    /// `<SteeringWheel>` element of type `OverrideSteeringWheelAction`
    #[serde(rename = "SteeringWheel", skip_serializing_if = "Option::is_none")]
    pub steering_wheel: Option<OverrideSteeringWheelAction>,

    /// `<Gear>` element of type `OverrideGearAction`
    #[serde(rename = "Gear", skip_serializing_if = "Option::is_none")]
    pub gear: Option<OverrideGearAction>,
}

/// Assign controller action for controller assignment with catalog support
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AssignControllerAction {
    #[serde(rename = "@activateLateral", skip_serializing_if = "Option::is_none")]
    pub activate_lateral: Option<Boolean>,
    #[serde(
        rename = "@activateLongitudinal",
        skip_serializing_if = "Option::is_none"
    )]
    pub activate_longitudinal: Option<Boolean>,
    #[serde(rename = "@activateAnimation", skip_serializing_if = "Option::is_none")]
    pub activate_animation: Option<Boolean>,
    #[serde(rename = "@activateLighting", skip_serializing_if = "Option::is_none")]
    pub activate_lighting: Option<Boolean>,
    #[serde(rename = "Controller", skip_serializing_if = "Option::is_none")]
    pub controller: Option<Controller>,
    #[serde(rename = "CatalogReference", skip_serializing_if = "Option::is_none")]
    pub catalog_reference: Option<CatalogReference<CatalogController>>,
    #[serde(rename = "ObjectController", skip_serializing_if = "Option::is_none")]
    pub object_controller: Option<ObjectController>,
}

/// Activate controller action for controller activation control
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActivateControllerAction {
    /// Deprecated reference to a controller by name — XSD `@controllerRef`
    #[serde(rename = "@controllerRef", skip_serializing_if = "Option::is_none")]
    pub controller_ref: Option<OSString>,
    /// Reference to an object controller — XSD `@objectControllerRef`
    #[serde(rename = "@objectControllerRef", skip_serializing_if = "Option::is_none")]
    pub object_controller_ref: Option<OSString>,
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


/// Manual gear specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ManualGear {
    #[serde(rename = "@number")]
    pub number: Int,
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
    #[serde(rename = "p")]
    Park,
    #[serde(rename = "r")]
    Reverse,
    #[serde(rename = "n")]
    Neutral,
    #[serde(rename = "d")]
    #[default]
    Drive,
}

/// Base brake type for brake input groups
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Brake {
    #[serde(rename = "@value")]
    pub value: Double,
    #[serde(rename = "@maxRate", default, skip_serializing_if = "Option::is_none")]
    pub max_rate: Option<Double>,
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


impl Default for AssignControllerAction {
    fn default() -> Self {
        Self {
            controller: None,
            catalog_reference: None,
            object_controller: None,
            activate_lateral: None,
            activate_longitudinal: None,
            activate_animation: None,
            activate_lighting: None,
        }
    }
}


impl Default for ActivateControllerAction {
    fn default() -> Self {
        Self {
            controller_ref: None,
            object_controller_ref: None,
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
            number: Int::literal(1),
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
            max_rate: None,
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


impl AssignControllerAction {
    /// Create assignment with direct controller
    pub fn with_controller(controller: Controller) -> Self {
        Self {
            controller: Some(controller),
            catalog_reference: None,
            object_controller: None,
            activate_lateral: None,
            activate_longitudinal: None,
            activate_animation: None,
            activate_lighting: None,
        }
    }

    /// Create assignment with catalog reference
    pub fn with_catalog_reference(catalog_reference: CatalogReference<CatalogController>) -> Self {
        Self {
            controller: None,
            catalog_reference: Some(catalog_reference),
            object_controller: None,
            activate_lateral: None,
            activate_longitudinal: None,
            activate_animation: None,
            activate_lighting: None,
        }
    }
}

impl ActivateControllerAction {
    /// Create activation with all control domains
    pub fn all_domains(longitudinal: bool, lateral: bool, lighting: bool, animation: bool) -> Self {
        Self {
            controller_ref: None,
            object_controller_ref: None,
            longitudinal: Some(Boolean::literal(longitudinal)),
            lateral: Some(Boolean::literal(lateral)),
            lighting: Some(Boolean::literal(lighting)),
            animation: Some(Boolean::literal(animation)),
        }
    }

    /// Create activation for movement only (longitudinal + lateral)
    pub fn movement_only() -> Self {
        Self {
            controller_ref: None,
            object_controller_ref: None,
            longitudinal: Some(Boolean::literal(true)),
            lateral: Some(Boolean::literal(true)),
            lighting: None,
            animation: None,
        }
    }
}

impl ManualGear {
    /// Create manual gear for specific gear number
    pub fn new(number: i32) -> Self {
        Self {
            number: Int::literal(number),
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
            max_rate: None,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::controllers::Controller;

    #[test]
    fn test_assign_controller_action_creation() {
        let controller = Controller::new(
            "TestController".to_string(),
            crate::types::enums::ControllerType::Movement,
        );
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
        assert_eq!(first_gear.number.as_literal().unwrap(), &1);

        let neutral = ManualGear::neutral();
        assert_eq!(neutral.number.as_literal().unwrap(), &0);

        let reverse = ManualGear::reverse();
        assert_eq!(reverse.number.as_literal().unwrap(), &(-1));
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
        assert!(assign.controller.is_none());
        assert!(assign.catalog_reference.is_none());

        let activate = ActivateControllerAction::default();
        assert_eq!(activate.longitudinal.unwrap().as_literal(), Some(&true));
        assert_eq!(activate.lateral.unwrap().as_literal(), Some(&true));

        let controller_action = ControllerAction::default();
        assert!(controller_action.assign_controller_action.is_none());
        assert!(controller_action.override_controller_value_action.is_none());
        assert!(controller_action.activate_controller_action.is_none());
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
            assert_eq!(gear.number.as_literal(), Some(&3));
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
            assert_eq!(gear.number.as_literal(), Some(&1));
        } else {
            panic!("Expected ManualGear variant");
        }

        let manual_neutral = Gear::manual_neutral();
        if let Gear::ManualGear(gear) = manual_neutral {
            assert_eq!(gear.number.as_literal(), Some(&0));
        } else {
            panic!("Expected ManualGear variant");
        }

        let manual_reverse = Gear::manual_reverse();
        if let Gear::ManualGear(gear) = manual_reverse {
            assert_eq!(gear.number.as_literal(), Some(&(-1)));
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

    // ------------------------------------------------------------------
    // XSD field additions: round-trip regression tests
    // ------------------------------------------------------------------

    #[test]
    fn test_assign_controller_action_object_controller_and_activate_flags_round_trip() {
        let xml = r#"<AssignControllerAction activateLateral="true" activateLongitudinal="false" activateAnimation="true" activateLighting="false">
    <ObjectController>
        <Controller name="AIController" controllerType="movement"/>
    </ObjectController>
</AssignControllerAction>"#;

        let action: AssignControllerAction = quick_xml::de::from_str(xml).unwrap();
        assert!(action.object_controller.is_some());
        assert_eq!(action.activate_lateral.clone().unwrap().as_literal(), Some(&true));
        assert_eq!(
            action.activate_longitudinal.clone().unwrap().as_literal(),
            Some(&false)
        );
        assert_eq!(action.activate_animation.clone().unwrap().as_literal(), Some(&true));
        assert_eq!(action.activate_lighting.clone().unwrap().as_literal(), Some(&false));

        let serialized = quick_xml::se::to_string(&action).unwrap();
        let reparsed: AssignControllerAction = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(action, reparsed);
    }

    #[test]
    fn test_activate_controller_action_controller_refs_round_trip() {
        let xml = r#"<ActivateControllerAction controllerRef="LegacyController" objectControllerRef="ObjController" longitudinal="true" lateral="false"/>"#;

        let action: ActivateControllerAction = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(
            action.controller_ref.unwrap().as_literal(),
            Some(&"LegacyController".to_string())
        );
        assert_eq!(
            action.object_controller_ref.unwrap().as_literal(),
            Some(&"ObjController".to_string())
        );

        let action2 = ActivateControllerAction {
            controller_ref: Some(OSString::literal("LegacyController".to_string())),
            object_controller_ref: Some(OSString::literal("ObjController".to_string())),
            longitudinal: Some(Boolean::literal(true)),
            lateral: Some(Boolean::literal(false)),
            lighting: None,
            animation: None,
        };
        let serialized = quick_xml::se::to_string(&action2).unwrap();
        assert!(serialized.contains(r#"controllerRef="LegacyController""#));
        assert!(serialized.contains(r#"objectControllerRef="ObjController""#));
        let reparsed: ActivateControllerAction = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(action2, reparsed);
    }

    #[test]
    fn test_controller_action_override_controller_value_round_trip() {
        // XSD `OverrideControllerValueAction` (:1565-1574): xsd:all of six
        // optionally-present, differently-named children.
        let xml = r#"<ControllerAction><OverrideControllerValueAction><Brake active="true" value="0.5"/></OverrideControllerValueAction></ControllerAction>"#;

        let action: ControllerAction = quick_xml::de::from_str(xml).unwrap();
        let ov = action
            .override_controller_value_action
            .as_ref()
            .expect("OverrideControllerValueAction should be present");
        assert!(ov.brake.is_some());
        assert!(ov.throttle.is_none());
        let brake = ov.brake.as_ref().unwrap();
        assert_eq!(brake.active.as_literal(), Some(&true));
        assert_eq!(brake.value.clone().unwrap().as_literal(), Some(&0.5));

        let serialized = quick_xml::se::to_string(&action).unwrap();
        assert!(serialized.contains("OverrideControllerValueAction"));
        assert!(serialized.contains("<Brake"));
        let reparsed: ControllerAction = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(action, reparsed);
    }

    #[test]
    fn test_brake_max_rate_round_trip() {
        let brake = Brake {
            value: Double::literal(0.5),
            max_rate: Some(Double::literal(2.0)),
        };
        let xml = quick_xml::se::to_string(&brake).unwrap();
        assert!(xml.contains(r#"maxRate="2""#), "serialized: {xml}");
        let deserialized: Brake = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(brake, deserialized);
    }
}
