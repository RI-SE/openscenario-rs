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

use serde::{Deserialize, Serialize};
use crate::types::basic::OSString;
use crate::types::environment::Environment;
use crate::types::actions::movement::{SpeedAction, TeleportAction, RoutingAction};

/// Complete Init structure replacing the empty placeholder in storyboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Init {
    #[serde(rename = "Actions")]
    pub actions: Actions,
}

/// Actions container holding all initialization actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actions {
    #[serde(rename = "GlobalAction", default)]
    pub global_actions: Vec<GlobalAction>,
    #[serde(rename = "Private", default)]
    pub private_actions: Vec<Private>,
}

/// Global actions that affect the entire scenario
/// The XML structure has explicit action type elements inside GlobalAction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalAction {
    #[serde(rename = "EnvironmentAction", skip_serializing_if = "Option::is_none")]
    pub environment_action: Option<EnvironmentAction>,
    // EntityAction and InfrastructureAction can be added later as Option fields
}



/// Environment setup action containing complete environment definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentAction {
    #[serde(rename = "Environment")]
    pub environment: Environment,
}

/// Private actions specific to individual entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Private {
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,
    #[serde(rename = "PrivateAction")]
    pub private_actions: Vec<PrivateActionWrapper>,
}

/// Wrapper for individual private actions with explicit element types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateActionWrapper {
    #[serde(rename = "LongitudinalAction", skip_serializing_if = "Option::is_none")]
    pub longitudinal_action: Option<LongitudinalAction>,
    #[serde(rename = "TeleportAction", skip_serializing_if = "Option::is_none")]
    pub teleport_action: Option<TeleportAction>,
    #[serde(rename = "RoutingAction", skip_serializing_if = "Option::is_none")]
    pub routing_action: Option<RoutingAction>,
}

/// Types of private actions available for entity initialization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum PrivateActionType {
    LongitudinalAction(LongitudinalAction),
    TeleportAction(TeleportAction),
    RoutingAction(RoutingAction),
    // LateralAction, SynchronizeAction, etc. can be added later
}

/// Longitudinal movement actions (speed control, etc.)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LongitudinalAction {
    #[serde(rename = "SpeedAction", skip_serializing_if = "Option::is_none")]
    pub speed_action: Option<SpeedAction>,
    // SpeedProfileAction, SynchronizeAction, etc. can be added later as Option fields
}

/// Types of longitudinal actions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum LongitudinalActionType {
    SpeedAction(SpeedAction),
    // SpeedProfileAction, SynchronizeAction, etc. can be added later
}

impl Default for Init {
    fn default() -> Self {
        Self {
            actions: Actions::default(),
        }
    }
}

impl Default for Actions {
    fn default() -> Self {
        Self {
            global_actions: Vec::new(),
            private_actions: Vec::new(),
        }
    }
}

impl Default for GlobalAction {
    fn default() -> Self {
        Self {
            environment_action: Some(EnvironmentAction::default()),
        }
    }
}

impl Default for EnvironmentAction {
    fn default() -> Self {
        Self {
            environment: Environment::default(),
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

impl Default for PrivateActionWrapper {
    fn default() -> Self {
        Self {
            longitudinal_action: Some(LongitudinalAction::default()),
            teleport_action: None,
            routing_action: None,
        }
    }
}

impl Default for LongitudinalAction {
    fn default() -> Self {
        Self {
            speed_action: Some(SpeedAction::default()),
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
    pub fn add_action(mut self, action: PrivateActionWrapper) -> Self {
        self.private_actions.push(action);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::basic::Value;
    use crate::types::environment::{TimeOfDay, Weather, RoadCondition};

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
        assert_eq!(init.actions.private_actions[0].entity_ref.as_literal().unwrap(), "Ego");
    }

    #[test]
    fn test_private_action_builder() {
        let private = Private::new("TestEntity")
            .add_action(PrivateActionWrapper {
                longitudinal_action: Some(LongitudinalAction {
                    speed_action: Some(SpeedAction::default()),
                }),
                teleport_action: None,
                routing_action: None,
            })
            .add_action(PrivateActionWrapper {
                longitudinal_action: None,
                teleport_action: Some(TeleportAction::default()),
                routing_action: None,
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

        assert_eq!(env_action.environment.name.as_literal().unwrap(), "TestEnvironment");
        assert_eq!(env_action.environment.time_of_day.date_time, "2021-12-10T11:00:00");
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
}