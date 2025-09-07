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
use crate::types::actions::movement::{SpeedAction, TeleportAction};

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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalAction {
    #[serde(flatten)]
    pub action_type: GlobalActionType,
}

/// Types of global actions available for initialization
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum GlobalActionType {
    EnvironmentAction(EnvironmentAction),
    // EntityAction and InfrastructureAction can be added later
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

/// Wrapper for individual private actions (similar to GlobalAction pattern)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateActionWrapper {
    #[serde(flatten)]
    pub action_type: PrivateActionType,
}

/// Types of private actions available for entity initialization
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum PrivateActionType {
    LongitudinalAction(LongitudinalAction),
    TeleportAction(TeleportAction),
    // LateralAction, SynchronizeAction, etc. can be added later
}

/// Longitudinal movement actions (speed control, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongitudinalAction {
    #[serde(flatten)]
    pub action_type: LongitudinalActionType,
}

/// Types of longitudinal actions
#[derive(Debug, Clone, Serialize, Deserialize)]
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
            action_type: GlobalActionType::EnvironmentAction(EnvironmentAction::default()),
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
            action_type: PrivateActionType::LongitudinalAction(LongitudinalAction::default()),
        }
    }
}

impl Default for LongitudinalAction {
    fn default() -> Self {
        Self {
            action_type: LongitudinalActionType::SpeedAction(SpeedAction::default()),
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
                    action_type: GlobalActionType::EnvironmentAction(EnvironmentAction {
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
                action_type: PrivateActionType::LongitudinalAction(LongitudinalAction {
                    action_type: LongitudinalActionType::SpeedAction(SpeedAction::default()),
                }),
            })
            .add_action(PrivateActionWrapper {
                action_type: PrivateActionType::TeleportAction(TeleportAction::default()),
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
                    action_type: GlobalActionType::EnvironmentAction(EnvironmentAction::default()),
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