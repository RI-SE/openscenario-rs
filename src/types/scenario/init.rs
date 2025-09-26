//! Initialization system for OpenSCENARIO scenarios
//!
//! This file contains:
//! - Init structure with Actions container for scenario initialization
//! - GlobalAction types for environment and infrastructure setup
//! - Private actions for entity-specific initialization
//! - Integration with existing action and environment systems
//!
//! Contributes to project by:
//! - Enabling complete scenario initialization parsing from XOSC files
//! - Supporting environment setup through EnvironmentAction
//! - Providing entity-specific private actions (speed, teleport, etc.)
//! - Facilitating integration between initialization and story phases

use crate::types::actions::control::ActivateControllerAction;
use crate::types::actions::movement::{
    LongitudinalDistanceAction, RoutingAction, SpeedAction, SpeedProfileAction, SynchronizeAction, TeleportAction,
};
use crate::types::basic::OSString;
use crate::types::environment::Environment;
use serde::{Deserialize, Serialize};

/// Complete Init structure replacing the empty placeholder in storyboard
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Init {
    #[serde(rename = "Actions")]
    pub actions: Actions,
}

/// Actions container holding all initialization actions
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Actions {
    #[serde(rename = "GlobalAction", default)]
    pub global_actions: Vec<GlobalAction>,
    #[serde(rename = "Private", default)]
    pub private_actions: Vec<Private>,
}

/// Global actions that affect the entire scenario
/// The XML structure has explicit action type elements inside GlobalAction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GlobalAction {
    #[serde(rename = "EnvironmentAction", skip_serializing_if = "Option::is_none")]
    pub environment_action: Option<EnvironmentAction>,
    // EntityAction and InfrastructureAction can be added later as Option fields
}

/// Environment setup action containing complete environment definition
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EnvironmentAction {
    #[serde(rename = "Environment")]
    pub environment: Environment,
}

/// Private actions specific to individual entities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Private {
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,
    #[serde(rename = "PrivateAction")]
    pub private_actions: Vec<PrivateAction>,
}

/// Private actions that can be applied to individual entities
/// XSD requires exactly one child element (choice group)
/// The PrivateAction element in XML contains one of these action types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrivateAction {
    /// Exactly one of these fields should be present (XML choice group)
    #[serde(rename = "LongitudinalAction", skip_serializing_if = "Option::is_none", default)]
    pub longitudinal_action: Option<LongitudinalAction>,
    #[serde(rename = "LateralAction", skip_serializing_if = "Option::is_none", default)]
    pub lateral_action: Option<crate::types::actions::movement::LateralAction>,
    #[serde(rename = "TeleportAction", skip_serializing_if = "Option::is_none", default)]
    pub teleport_action: Option<TeleportAction>,
    #[serde(rename = "RoutingAction", skip_serializing_if = "Option::is_none", default)]
    pub routing_action: Option<RoutingAction>,
    #[serde(rename = "SynchronizeAction", skip_serializing_if = "Option::is_none", default)]
    pub synchronize_action: Option<SynchronizeAction>,
    #[serde(rename = "ActivateControllerAction", skip_serializing_if = "Option::is_none", default)]
    pub activate_controller_action: Option<ActivateControllerAction>,
    // Note: Other action types like VisibilityAction, ControllerAction,
    // AppearanceAction, TrailerAction can be added when implemented
}



impl PrivateAction {
    /// Get the action type contained in this PrivateAction
    pub fn get_action_type(&self) -> Option<&str> {
        if self.longitudinal_action.is_some() {
            Some("LongitudinalAction")
        } else if self.lateral_action.is_some() {
            Some("LateralAction")
        } else if self.teleport_action.is_some() {
            Some("TeleportAction")
        } else if self.routing_action.is_some() {
            Some("RoutingAction")
        } else if self.synchronize_action.is_some() {
            Some("SynchronizeAction")
        } else if self.activate_controller_action.is_some() {
            Some("ActivateControllerAction")
        } else {
            None
        }
    }

    /// Validates that exactly one action type is present (XSD choice group requirement)
    pub fn validate(&self) -> Result<(), String> {
        let action_count = [
            self.longitudinal_action.is_some(),
            self.lateral_action.is_some(),
            self.teleport_action.is_some(),
            self.routing_action.is_some(),
            self.synchronize_action.is_some(),
            self.activate_controller_action.is_some(),
        ].iter().filter(|&&x| x).count();

        match action_count {
            1 => Ok(()),
            0 => Err("PrivateAction must contain exactly one action type, found none".to_string()),
            _ => Err("PrivateAction must contain exactly one action type, found multiple".to_string()),
        }
    }
}

impl Default for PrivateAction {
    fn default() -> Self {
        Self {
            longitudinal_action: None,
            lateral_action: None,
            teleport_action: None,
            routing_action: None,
            synchronize_action: None,
            activate_controller_action: None,
        }
    }
}

/// Longitudinal movement actions (speed control, etc.)
/// XSD requires exactly one child element (choice group)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LongitudinalAction {
    /// Exactly one of these fields should be present (XML choice group)
    #[serde(rename = "SpeedAction", skip_serializing_if = "Option::is_none", default)]
    pub speed_action: Option<SpeedAction>,
    #[serde(rename = "LongitudinalDistanceAction", skip_serializing_if = "Option::is_none", default)]
    pub longitudinal_distance_action: Option<LongitudinalDistanceAction>,
    #[serde(rename = "SpeedProfileAction", skip_serializing_if = "Option::is_none", default)]
    pub speed_profile_action: Option<SpeedProfileAction>,
}



impl LongitudinalAction {
    /// Get the action type contained in this LongitudinalAction
    pub fn get_action_type(&self) -> Option<&str> {
        if self.speed_action.is_some() {
            Some("SpeedAction")
        } else if self.longitudinal_distance_action.is_some() {
            Some("LongitudinalDistanceAction")
        } else if self.speed_profile_action.is_some() {
            Some("SpeedProfileAction")
        } else {
            None
        }
    }

    /// Validates that exactly one action type is present (XSD choice group requirement)
    pub fn validate(&self) -> Result<(), String> {
        let action_count = [
            self.speed_action.is_some(),
            self.longitudinal_distance_action.is_some(),
            self.speed_profile_action.is_some(),
        ].iter().filter(|&&x| x).count();

        match action_count {
            1 => Ok(()),
            0 => Err("LongitudinalAction must contain exactly one action type (SpeedAction, LongitudinalDistanceAction, or SpeedProfileAction), found none".to_string()),
            _ => Err("LongitudinalAction must contain exactly one action type (SpeedAction, LongitudinalDistanceAction, or SpeedProfileAction), found multiple".to_string()),
        }
    }
}

impl Default for LongitudinalAction {
    fn default() -> Self {
        Self {
            speed_action: Some(SpeedAction::default()),
            longitudinal_distance_action: None,
            speed_profile_action: None,
        }
    }
}

/// Types of longitudinal actions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum LongitudinalActionType {
    SpeedAction(SpeedAction),
    // SpeedProfileAction, SynchronizeAction, etc. can be added later
}

impl Default for GlobalAction {
    fn default() -> Self {
        Self {
            environment_action: Some(EnvironmentAction::default()),
        }
    }
}

impl Default for Private {
    fn default() -> Self {
        Self {
            entity_ref: crate::types::basic::Value::literal("DefaultEntity".to_string()),
            private_actions: Vec::new(),
        }
    }
}





impl Private {
    /// Create a new Private action container for the specified entity
    pub fn new(entity_ref: &str) -> Self {
        Self {
            entity_ref: crate::types::basic::Value::literal(entity_ref.to_string()),
            private_actions: Vec::new(),
        }
    }

    /// Add a private action to this entity's initialization
    pub fn add_action(mut self, action: PrivateAction) -> Self {
        self.private_actions.push(action);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::basic::Value;
    use crate::types::environment::{RoadCondition, TimeOfDay, Weather};

    #[test]
    fn test_init_creation() {
        let init = Init {
            actions: Actions {
                global_actions: vec![GlobalAction {
                    environment_action: Some(EnvironmentAction {
                        environment: Environment::default(),
                    }),
                }],
                private_actions: vec![Private::new("Ego")],
            },
        };

        assert_eq!(init.actions.global_actions.len(), 1);
        assert_eq!(init.actions.private_actions.len(), 1);
        assert_eq!(
            init.actions.private_actions[0]
                .entity_ref
                .as_literal()
                .unwrap(),
            "Ego"
        );
    }

    #[test]
    fn test_private_action_builder() {
        let private = Private::new("TestEntity")
            .add_action(PrivateAction {
                longitudinal_action: Some(LongitudinalAction {
                    speed_action: Some(SpeedAction::default()),
                    longitudinal_distance_action: None,
                    speed_profile_action: None,
                }),
                lateral_action: None,
                teleport_action: None,
                routing_action: None,
                synchronize_action: None,
                activate_controller_action: None,
            })
            .add_action(PrivateAction {
                longitudinal_action: None,
                lateral_action: None,
                teleport_action: Some(TeleportAction::default()),
                routing_action: None,
                synchronize_action: None,
                activate_controller_action: None,
            });

        assert_eq!(private.entity_ref.as_literal().unwrap(), "TestEntity");
        assert_eq!(private.private_actions.len(), 2);
    }

    #[test]
    fn test_init_default() {
        let init = Init::default();

        assert!(init.actions.global_actions.is_empty());
        assert!(init.actions.private_actions.is_empty());
    }

    #[test]
    fn test_environment_action_creation() {
        let env_action = EnvironmentAction {
            environment: Environment {
                name: Value::literal("TestEnvironment".to_string()),
                time_of_day: TimeOfDay {
                    animation: Value::literal(false),
                    date_time: "2021-12-10T11:00:00".to_string(),
                },
                weather: Weather::default(),
                road_condition: RoadCondition::default(),
            },
        };

        assert_eq!(
            env_action.environment.name.as_literal().unwrap(),
            "TestEnvironment"
        );
        assert_eq!(
            env_action.environment.time_of_day.date_time,
            "2021-12-10T11:00:00"
        );
    }

    #[test]
    fn test_init_serialization() {
        let init = Init {
            actions: Actions {
                global_actions: vec![GlobalAction {
                    environment_action: Some(EnvironmentAction::default()),
                }],
                private_actions: vec![Private::new("Ego")],
            },
        };

        let serialized = quick_xml::se::to_string(&init).unwrap();
        assert!(serialized.contains("<Actions"));
        assert!(serialized.contains("<GlobalAction"));
        assert!(serialized.contains("<Private"));
        assert!(serialized.contains("entityRef=\"Ego\""));
    }

    #[test]
    fn test_longitudinal_action_validation() {
        // Test valid action with SpeedAction
        let valid_speed = LongitudinalAction {
            speed_action: Some(SpeedAction::default()),
            longitudinal_distance_action: None,
            speed_profile_action: None,
        };
        assert!(valid_speed.validate().is_ok());

        // Test valid action with LongitudinalDistanceAction
        let valid_distance = LongitudinalAction {
            speed_action: None,
            longitudinal_distance_action: Some(LongitudinalDistanceAction::default()),
            speed_profile_action: None,
        };
        assert!(valid_distance.validate().is_ok());

        // Test valid action with SpeedProfileAction
        let valid_profile = LongitudinalAction {
            speed_action: None,
            longitudinal_distance_action: None,
            speed_profile_action: Some(SpeedProfileAction::default()),
        };
        assert!(valid_profile.validate().is_ok());

        // Test invalid action with no actions
        let invalid_none = LongitudinalAction {
            speed_action: None,
            longitudinal_distance_action: None,
            speed_profile_action: None,
        };
        assert!(invalid_none.validate().is_err());

        // Test invalid action with multiple actions
        let invalid_multiple = LongitudinalAction {
            speed_action: Some(SpeedAction::default()),
            longitudinal_distance_action: Some(LongitudinalDistanceAction::default()),
            speed_profile_action: None,
        };
        assert!(invalid_multiple.validate().is_err());
    }

    #[test]
    fn test_private_action_validation() {
        // Test valid action with LongitudinalAction
        let valid_longitudinal = PrivateAction {
            longitudinal_action: Some(LongitudinalAction::default()),
            lateral_action: None,
            teleport_action: None,
            routing_action: None,
            synchronize_action: None,
            activate_controller_action: None,
        };
        assert!(valid_longitudinal.validate().is_ok());

        // Test invalid action with no actions
        let invalid_none = PrivateAction::default();
        // Default has speed_action, so it should be valid
        assert!(invalid_none.validate().is_ok());

        // Test invalid action with multiple actions
        let invalid_multiple = PrivateAction {
            longitudinal_action: Some(LongitudinalAction::default()),
            lateral_action: Some(crate::types::actions::movement::LateralAction::default()),
            teleport_action: None,
            routing_action: None,
            synchronize_action: None,
            activate_controller_action: None,
        };
        assert!(invalid_multiple.validate().is_err());
    }
}
