//! Controller and control override action types
//!
//! This file contains:
//! - Controller assignment and activation actions  
//! - Override actions for manual control (throttle, brake, steering, gear)
//! - Controller configuration and parameter setting
//! - Control system integration and handoff logic
//! - Multi-controller coordination and conflict resolution
//!
//! Contributes to project by:
//! - Enabling realistic vehicle control through AI controllers and manual overrides
//! - Supporting simulation of different driver behaviors and capabilities
//! - Providing fine-grained control over vehicle dynamics and responses
//! - Facilitating testing of control systems and human-machine interfaces
//! - Enabling smooth transitions between automated and manual control modes

use serde::{Deserialize, Serialize};
use crate::types::basic::Value;
use crate::types::controllers::{
    ActivateControllerAction, OverrideControllerValueAction, ControllerAssignment
};

/// Control action wrapper that encompasses all controller-related actions.
///
/// This wrapper provides a unified interface for all types of controller actions,
/// from activation and parameter overrides to manual control inputs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ControlAction {
    /// Activate a controller for an entity
    #[serde(rename = "ActivateControllerAction")]
    ActivateController(ActivateControllerAction),

    /// Override controller parameter values
    #[serde(rename = "OverrideControllerValueAction")]
    OverrideController(OverrideControllerValueAction),

    /// Assign a controller to an entity
    #[serde(rename = "ControllerAssignment")]
    AssignController(ControllerAssignment),

    /// Manual throttle override
    #[serde(rename = "ThrottleAction")]
    Throttle(ThrottleAction),

    /// Manual brake override
    #[serde(rename = "BrakeAction")]
    Brake(BrakeAction),

    /// Manual steering override  
    #[serde(rename = "SteeringAction")]
    Steering(SteeringAction),

    /// Manual gear change
    #[serde(rename = "GearAction")]
    Gear(GearAction),

    /// Manual clutch override
    #[serde(rename = "ClutchAction")]
    Clutch(ClutchAction),

    /// Manual parking brake control
    #[serde(rename = "ParkingBrakeAction")]
    ParkingBrake(ParkingBrakeAction),
}

/// Manual throttle control action.
///
/// Overrides automatic throttle control with a specific throttle value.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ThrottleAction {
    /// Throttle pedal position (0.0 = no throttle, 1.0 = full throttle)
    #[serde(rename = "@value")]
    pub value: Value<f64>,

    /// Whether this override is currently active
    #[serde(rename = "@active")]
    pub active: Value<bool>,
}

impl Default for ThrottleAction {
    fn default() -> Self {
        Self {
            value: Value::Literal(0.0),
            active: Value::Literal(true),
        }
    }
}

/// Manual brake control action.
///
/// Overrides automatic brake control with a specific brake force.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BrakeAction {
    /// Brake pedal position (0.0 = no braking, 1.0 = full braking)
    #[serde(rename = "@value")]
    pub value: Value<f64>,

    /// Whether this override is currently active
    #[serde(rename = "@active")]
    pub active: Value<bool>,
}

impl Default for BrakeAction {
    fn default() -> Self {
        Self {
            value: Value::Literal(0.0),
            active: Value::Literal(true),
        }
    }
}

/// Manual steering control action.
///
/// Overrides automatic steering control with a specific steering angle.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SteeringAction {
    /// Steering wheel angle in radians (negative = left, positive = right)
    #[serde(rename = "@value")]
    pub value: Value<f64>,

    /// Whether this override is currently active
    #[serde(rename = "@active")]
    pub active: Value<bool>,
}

impl Default for SteeringAction {
    fn default() -> Self {
        Self {
            value: Value::Literal(0.0),
            active: Value::Literal(true),
        }
    }
}

/// Manual gear selection action.
///
/// Overrides automatic transmission with a specific gear selection.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GearAction {
    /// Gear selection (gear number for manual, mode for automatic)
    #[serde(rename = "@gear")]
    pub gear: Value<String>,

    /// Whether this override is currently active
    #[serde(rename = "@active")]
    pub active: Value<bool>,
}

impl Default for GearAction {
    fn default() -> Self {
        Self {
            gear: Value::Literal("neutral".to_string()),
            active: Value::Literal(true),
        }
    }
}

/// Manual clutch control action.
///
/// Overrides automatic clutch control with a specific clutch position.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClutchAction {
    /// Clutch pedal position (0.0 = engaged, 1.0 = fully disengaged)
    #[serde(rename = "@value")]
    pub value: Value<f64>,

    /// Whether this override is currently active
    #[serde(rename = "@active")]
    pub active: Value<bool>,
}

impl Default for ClutchAction {
    fn default() -> Self {
        Self {
            value: Value::Literal(0.0),
            active: Value::Literal(true),
        }
    }
}

/// Manual parking brake control action.
///
/// Controls the parking brake state independently of the main brake system.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParkingBrakeAction {
    /// Parking brake force (0.0 = released, 1.0 = fully engaged)
    #[serde(rename = "@value")]
    pub value: Value<f64>,

    /// Whether the parking brake is currently active
    #[serde(rename = "@active")]
    pub active: Value<bool>,
}

impl Default for ParkingBrakeAction {
    fn default() -> Self {
        Self {
            value: Value::Literal(0.0),
            active: Value::Literal(false),
        }
    }
}

// Helper implementations for common control operations

impl ThrottleAction {
    /// Creates a throttle action with the specified value.
    pub fn new(value: f64) -> Self {
        Self {
            value: Value::Literal(value.clamp(0.0, 1.0)),
            active: Value::Literal(true),
        }
    }

    /// Creates an inactive throttle action.
    pub fn inactive() -> Self {
        Self {
            value: Value::Literal(0.0),
            active: Value::Literal(false),
        }
    }
}

impl BrakeAction {
    /// Creates a brake action with the specified value.
    pub fn new(value: f64) -> Self {
        Self {
            value: Value::Literal(value.clamp(0.0, 1.0)),
            active: Value::Literal(true),
        }
    }

    /// Creates an inactive brake action.
    pub fn inactive() -> Self {
        Self {
            value: Value::Literal(0.0),
            active: Value::Literal(false),
        }
    }
}

impl SteeringAction {
    /// Creates a steering action with the specified angle in radians.
    pub fn new(angle_rad: f64) -> Self {
        Self {
            value: Value::Literal(angle_rad),
            active: Value::Literal(true),
        }
    }

    /// Creates a steering action with the specified angle in degrees.
    pub fn from_degrees(angle_deg: f64) -> Self {
        Self::new(angle_deg.to_radians())
    }

    /// Creates an inactive steering action.
    pub fn inactive() -> Self {
        Self {
            value: Value::Literal(0.0),
            active: Value::Literal(false),
        }
    }
}

impl GearAction {
    /// Creates a gear action for the specified gear.
    pub fn new(gear: String) -> Self {
        Self {
            gear: Value::Literal(gear),
            active: Value::Literal(true),
        }
    }

    /// Creates a gear action for neutral.
    pub fn neutral() -> Self {
        Self::new("neutral".to_string())
    }

    /// Creates a gear action for park.
    pub fn park() -> Self {
        Self::new("park".to_string())
    }

    /// Creates a gear action for reverse.
    pub fn reverse() -> Self {
        Self::new("reverse".to_string())
    }

    /// Creates a gear action for drive.
    pub fn drive() -> Self {
        Self::new("drive".to_string())
    }
}

impl ClutchAction {
    /// Creates a clutch action with the specified position.
    pub fn new(position: f64) -> Self {
        Self {
            value: Value::Literal(position.clamp(0.0, 1.0)),
            active: Value::Literal(true),
        }
    }

    /// Creates a clutch action for fully engaged clutch.
    pub fn engaged() -> Self {
        Self::new(0.0)
    }

    /// Creates a clutch action for fully disengaged clutch.
    pub fn disengaged() -> Self {
        Self::new(1.0)
    }
}

impl ParkingBrakeAction {
    /// Creates a parking brake action with the specified force.
    pub fn new(force: f64) -> Self {
        Self {
            value: Value::Literal(force.clamp(0.0, 1.0)),
            active: Value::Literal(true),
        }
    }

    /// Creates a parking brake action for released state.
    pub fn released() -> Self {
        Self {
            value: Value::Literal(0.0),
            active: Value::Literal(false),
        }
    }

    /// Creates a parking brake action for engaged state.
    pub fn engaged() -> Self {
        Self::new(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::controllers::ActivateControllerAction;

    #[test]
    fn test_throttle_action_creation() {
        let throttle = ThrottleAction::new(0.75);
        assert_eq!(throttle.value.as_literal().unwrap(), &0.75);
        assert_eq!(throttle.active.as_literal().unwrap(), &true);

        let inactive_throttle = ThrottleAction::inactive();
        assert_eq!(inactive_throttle.active.as_literal().unwrap(), &false);
    }

    #[test]
    fn test_brake_action_creation() {
        let brake = BrakeAction::new(0.5);
        assert_eq!(brake.value.as_literal().unwrap(), &0.5);
        assert_eq!(brake.active.as_literal().unwrap(), &true);
    }

    #[test]
    fn test_steering_action_creation() {
        let steering = SteeringAction::new(0.5);
        assert_eq!(steering.value.as_literal().unwrap(), &0.5);

        let steering_degrees = SteeringAction::from_degrees(90.0);
        assert!((steering_degrees.value.as_literal().unwrap() - std::f64::consts::FRAC_PI_2).abs() < 1e-10);
    }

    #[test]
    fn test_gear_action_creation() {
        let gear = GearAction::drive();
        assert_eq!(gear.gear.as_literal().unwrap(), "drive");

        let neutral = GearAction::neutral();
        assert_eq!(neutral.gear.as_literal().unwrap(), "neutral");
    }

    #[test]
    fn test_clutch_action_creation() {
        let engaged = ClutchAction::engaged();
        assert_eq!(engaged.value.as_literal().unwrap(), &0.0);

        let disengaged = ClutchAction::disengaged();
        assert_eq!(disengaged.value.as_literal().unwrap(), &1.0);
    }

    #[test]
    fn test_parking_brake_action_creation() {
        let released = ParkingBrakeAction::released();
        assert_eq!(released.value.as_literal().unwrap(), &0.0);
        assert_eq!(released.active.as_literal().unwrap(), &false);

        let engaged = ParkingBrakeAction::engaged();
        assert_eq!(engaged.value.as_literal().unwrap(), &1.0);
        assert_eq!(engaged.active.as_literal().unwrap(), &true);
    }

    #[test]
    fn test_control_action_variants() {
        let activate = ControlAction::ActivateController(ActivateControllerAction::default());
        let throttle = ControlAction::Throttle(ThrottleAction::new(0.5));
        let brake = ControlAction::Brake(BrakeAction::new(0.3));

        // All variants should be valid
        match activate {
            ControlAction::ActivateController(_) => (),
            _ => panic!("Expected ActivateController variant"),
        }

        match throttle {
            ControlAction::Throttle(_) => (),
            _ => panic!("Expected Throttle variant"),
        }

        match brake {
            ControlAction::Brake(_) => (),
            _ => panic!("Expected Brake variant"),
        }
    }

    #[test]
    fn test_throttle_value_clamping() {
        let over_max = ThrottleAction::new(1.5);
        assert_eq!(over_max.value.as_literal().unwrap(), &1.0);

        let under_min = ThrottleAction::new(-0.5);
        assert_eq!(under_min.value.as_literal().unwrap(), &0.0);
    }

    #[test]
    fn test_control_action_serialization() {
        let throttle = ThrottleAction::new(0.8);
        
        // Test XML serialization
        let xml = quick_xml::se::to_string(&throttle).unwrap();
        assert!(xml.contains("0.8"));
        assert!(xml.contains("true"));

        // Test deserialization
        let deserialized: ThrottleAction = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(throttle, deserialized);
    }
}