//! Main init action builder implementation

use super::private::{GlobalActionBuilder, PrivateActionBuilder};
use crate::builder::BuilderResult;
use crate::types::{
    environment::Environment,
    positions::Position,
    scenario::init::{Actions, EnvironmentAction, GlobalAction, Init, Private},
};

/// Builder for complete Init structure with actions
#[derive(Debug, Default)]
pub struct InitActionBuilder {
    global_actions: Vec<GlobalAction>,
    private_actions: Vec<Private>,
}

impl InitActionBuilder {
    /// Create a new init action builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a global environment action with default environment
    pub fn add_global_environment_action(mut self) -> Self {
        let global_action = GlobalAction {
            environment_action: Some(EnvironmentAction {
                environment: Environment::default(),
            }),
        };
        self.global_actions.push(global_action);
        self
    }

    /// Add a custom global action
    pub fn add_global_action(mut self, action: GlobalAction) -> Self {
        self.global_actions.push(action);
        self
    }

    /// Create a global action builder
    pub fn create_global_action(self) -> GlobalActionBuilder {
        GlobalActionBuilder::new(self)
    }

    /// Add a private action container for an entity
    pub fn add_private_action(mut self, entity_ref: &str) -> Self {
        let private = Private::new(entity_ref);
        self.private_actions.push(private);
        self
    }

    /// Create a private action builder for an entity
    pub fn create_private_action(self, entity_ref: &str) -> PrivateActionBuilder {
        PrivateActionBuilder::new(self, entity_ref)
    }

    /// Add a teleport action for an entity (convenience method)
    pub fn add_teleport_action(mut self, entity_ref: &str, position: Position) -> Self {
        // Find existing private action or create new one
        let private_index = self
            .private_actions
            .iter()
            .position(|p| p.entity_ref.as_literal().unwrap_or(&String::new()) == entity_ref);

        if let Some(index) = private_index {
            // Add to existing private action
            let teleport_action = crate::types::scenario::init::PrivateAction {
                teleport_action: Some(crate::types::actions::movement::TeleportAction { position }),
                ..Default::default()
            };
            self.private_actions[index]
                .private_actions
                .push(teleport_action);
        } else {
            // Create new private action
            let teleport_action = crate::types::scenario::init::PrivateAction {
                teleport_action: Some(crate::types::actions::movement::TeleportAction { position }),
                ..Default::default()
            };
            let private = Private::new(entity_ref).add_action(teleport_action);
            self.private_actions.push(private);
        }

        self
    }

    /// Add a speed action for an entity (convenience method)
    pub fn add_speed_action(mut self, entity_ref: &str, speed: f64) -> Self {
        // Find existing private action or create new one
        let private_index = self
            .private_actions
            .iter()
            .position(|p| p.entity_ref.as_literal().unwrap_or(&String::new()) == entity_ref);

        if let Some(index) = private_index {
            // Add to existing private action
            let speed_action = crate::types::scenario::init::PrivateAction {
                longitudinal_action: Some(crate::types::scenario::init::LongitudinalAction {
                    speed_action: Some(crate::types::actions::movement::SpeedAction {
                        speed_action_dynamics:
                            crate::types::actions::movement::TransitionDynamics {
                                dynamics_dimension: crate::types::enums::DynamicsDimension::Time,
                                dynamics_shape: crate::types::enums::DynamicsShape::Step,
                                value: crate::types::basic::Double::literal(1.0),
                            },
                        speed_action_target: crate::types::actions::movement::SpeedActionTarget {
                            absolute: Some(crate::types::actions::movement::AbsoluteTargetSpeed {
                                value: crate::types::basic::Double::literal(speed),
                            }),
                            relative: None,
                        },
                    }),
                    longitudinal_distance_action: None,
                    speed_profile_action: None,
                }),
                ..Default::default()
            };
            self.private_actions[index]
                .private_actions
                .push(speed_action);
        } else {
            // Create new private action
            let speed_action = crate::types::scenario::init::PrivateAction {
                longitudinal_action: Some(crate::types::scenario::init::LongitudinalAction {
                    speed_action: Some(crate::types::actions::movement::SpeedAction {
                        speed_action_dynamics:
                            crate::types::actions::movement::TransitionDynamics {
                                dynamics_dimension: crate::types::enums::DynamicsDimension::Time,
                                dynamics_shape: crate::types::enums::DynamicsShape::Step,
                                value: crate::types::basic::Double::literal(1.0),
                            },
                        speed_action_target: crate::types::actions::movement::SpeedActionTarget {
                            absolute: Some(crate::types::actions::movement::AbsoluteTargetSpeed {
                                value: crate::types::basic::Double::literal(speed),
                            }),
                            relative: None,
                        },
                    }),
                    longitudinal_distance_action: None,
                    speed_profile_action: None,
                }),
                ..Default::default()
            };
            let private = Private::new(entity_ref).add_action(speed_action);
            self.private_actions.push(private);
        }

        self
    }

    /// Internal method to add a completed private action
    pub(crate) fn add_private(mut self, private: Private) -> Self {
        self.private_actions.push(private);
        self
    }

    /// Internal method to add a completed global action
    pub(crate) fn add_global(mut self, global: GlobalAction) -> Self {
        self.global_actions.push(global);
        self
    }

    /// Build the final Init structure
    pub fn build(self) -> BuilderResult<Init> {
        Ok(Init {
            actions: Actions {
                global_actions: self.global_actions,
                private_actions: self.private_actions,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::positions::WorldPositionBuilder;

    #[test]
    fn test_init_action_builder_empty() {
        let init = InitActionBuilder::new().build().unwrap();

        assert!(init.actions.global_actions.is_empty());
        assert!(init.actions.private_actions.is_empty());
    }

    #[test]
    fn test_init_action_builder_with_environment() {
        let init = InitActionBuilder::new()
            .add_global_environment_action()
            .build()
            .unwrap();

        assert_eq!(init.actions.global_actions.len(), 1);
        assert!(init.actions.global_actions[0].environment_action.is_some());
    }

    #[test]
    fn test_init_action_builder_with_teleport() {
        let position = WorldPositionBuilder::new()
            .at_coordinates(10.0, 20.0, 0.0)
            .build()
            .unwrap();

        let init = InitActionBuilder::new()
            .add_teleport_action("ego", position)
            .build()
            .unwrap();

        assert_eq!(init.actions.private_actions.len(), 1);
        assert_eq!(
            init.actions.private_actions[0]
                .entity_ref
                .as_literal()
                .unwrap(),
            "ego"
        );
        assert_eq!(init.actions.private_actions[0].private_actions.len(), 1);
        assert!(init.actions.private_actions[0].private_actions[0]
            .teleport_action
            .is_some());
    }

    #[test]
    fn test_init_action_builder_with_speed() {
        let init = InitActionBuilder::new()
            .add_speed_action("ego", 30.0)
            .build()
            .unwrap();

        assert_eq!(init.actions.private_actions.len(), 1);
        assert_eq!(
            init.actions.private_actions[0]
                .entity_ref
                .as_literal()
                .unwrap(),
            "ego"
        );
        assert_eq!(init.actions.private_actions[0].private_actions.len(), 1);
        assert!(init.actions.private_actions[0].private_actions[0]
            .longitudinal_action
            .is_some());

        let longitudinal = &init.actions.private_actions[0].private_actions[0]
            .longitudinal_action
            .as_ref()
            .unwrap();
        assert!(longitudinal.speed_action.is_some());

        let speed_action = longitudinal.speed_action.as_ref().unwrap();
        assert_eq!(
            speed_action
                .speed_action_target
                .absolute
                .as_ref()
                .unwrap()
                .value
                .as_literal()
                .unwrap(),
            &30.0
        );
    }

    #[test]
    fn test_init_action_builder_multiple_actions_same_entity() {
        let position = WorldPositionBuilder::new()
            .at_coordinates(10.0, 20.0, 0.0)
            .build()
            .unwrap();

        let init = InitActionBuilder::new()
            .add_teleport_action("ego", position)
            .add_speed_action("ego", 30.0)
            .build()
            .unwrap();

        assert_eq!(init.actions.private_actions.len(), 1);
        assert_eq!(
            init.actions.private_actions[0]
                .entity_ref
                .as_literal()
                .unwrap(),
            "ego"
        );
        assert_eq!(init.actions.private_actions[0].private_actions.len(), 2);

        // First action should be teleport
        assert!(init.actions.private_actions[0].private_actions[0]
            .teleport_action
            .is_some());
        // Second action should be speed
        assert!(init.actions.private_actions[0].private_actions[1]
            .longitudinal_action
            .is_some());
    }

    #[test]
    fn test_init_action_builder_multiple_entities() {
        let position1 = WorldPositionBuilder::new()
            .at_coordinates(0.0, 0.0, 0.0)
            .build()
            .unwrap();

        let position2 = WorldPositionBuilder::new()
            .at_coordinates(10.0, 0.0, 0.0)
            .build()
            .unwrap();

        let init = InitActionBuilder::new()
            .add_teleport_action("ego", position1)
            .add_speed_action("ego", 30.0)
            .add_teleport_action("target", position2)
            .add_speed_action("target", 25.0)
            .build()
            .unwrap();

        assert_eq!(init.actions.private_actions.len(), 2);

        // Check ego entity
        let ego_private = &init.actions.private_actions[0];
        assert_eq!(ego_private.entity_ref.as_literal().unwrap(), "ego");
        assert_eq!(ego_private.private_actions.len(), 2);

        // Check target entity
        let target_private = &init.actions.private_actions[1];
        assert_eq!(target_private.entity_ref.as_literal().unwrap(), "target");
        assert_eq!(target_private.private_actions.len(), 2);
    }
}
