//! Private and global action builders for entity-specific initialization

use super::actions::InitActionBuilder;
use crate::builder::{BuilderError, BuilderResult};
use crate::types::{
    actions::movement::{
        AbsoluteTargetSpeed, SpeedAction, SpeedActionTarget, TeleportAction, TransitionDynamics,
    },
    basic::{Double, OSString, Value},
    enums::{DynamicsDimension, DynamicsShape},
    environment::Environment,
    positions::Position,
    scenario::init::{EnvironmentAction, GlobalAction, LongitudinalAction, Private, PrivateAction},
};

/// Builder for private actions specific to individual entities
#[derive(Debug)]
pub struct PrivateActionBuilder {
    parent: InitActionBuilder,
    entity_ref: String,
    actions: Vec<PrivateAction>,
}

impl PrivateActionBuilder {
    /// Create a new private action builder
    pub fn new(parent: InitActionBuilder, entity_ref: &str) -> Self {
        Self {
            parent,
            entity_ref: entity_ref.to_string(),
            actions: Vec::new(),
        }
    }

    /// Add a teleport action to position the entity
    pub fn add_teleport_action(mut self, position: Position) -> Self {
        let action = PrivateAction {
            teleport_action: Some(TeleportAction { position }),
            ..Default::default()
        };
        self.actions.push(action);
        self
    }

    /// Add a speed action to set initial velocity
    pub fn add_speed_action(mut self, speed: f64) -> Self {
        let action = PrivateAction {
            longitudinal_action: Some(LongitudinalAction {
                speed_action: Some(SpeedAction {
                    speed_action_dynamics: TransitionDynamics {
                        dynamics_dimension: DynamicsDimension::Time,
                        dynamics_shape: DynamicsShape::Step,
                        value: Double::literal(1.0),
                    },
                    speed_action_target: SpeedActionTarget {
                        absolute: Some(crate::types::actions::movement::AbsoluteTargetSpeed {
                            value: Double::literal(speed),
                        }),
                        relative: None,
                    },
                }),
                longitudinal_distance_action: None,
                speed_profile_action: None,
            }),
            ..Default::default()
        };
        self.actions.push(action);
        self
    }

    /// Add a custom private action
    pub fn add_action(mut self, action: PrivateAction) -> Self {
        self.actions.push(action);
        self
    }

    /// Finish building and return to parent
    pub fn finish(self) -> InitActionBuilder {
        let private = Private {
            entity_ref: Value::literal(self.entity_ref),
            private_actions: self.actions,
        };
        self.parent.add_private(private)
    }

    /// Build the private action container
    pub fn build(self) -> BuilderResult<Private> {
        Ok(Private {
            entity_ref: Value::literal(self.entity_ref),
            private_actions: self.actions,
        })
    }
}

/// Builder for global actions that affect the entire scenario
#[derive(Debug)]
pub struct GlobalActionBuilder {
    parent: InitActionBuilder,
    environment_action: Option<EnvironmentAction>,
}

impl GlobalActionBuilder {
    /// Create a new global action builder
    pub fn new(parent: InitActionBuilder) -> Self {
        Self {
            parent,
            environment_action: None,
        }
    }

    /// Add an environment action with custom environment
    pub fn add_environment_action(mut self, environment: Environment) -> Self {
        self.environment_action = Some(EnvironmentAction { environment });
        self
    }

    /// Add an environment action with default environment
    pub fn add_default_environment_action(mut self) -> Self {
        self.environment_action = Some(EnvironmentAction {
            environment: Environment::default(),
        });
        self
    }

    /// Finish building and return to parent
    pub fn finish(self) -> InitActionBuilder {
        let global_action = GlobalAction {
            environment_action: self.environment_action,
        };
        self.parent.add_global(global_action)
    }

    /// Build the global action
    pub fn build(self) -> BuilderResult<GlobalAction> {
        Ok(GlobalAction {
            environment_action: self.environment_action,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::positions::WorldPositionBuilder;

    #[test]
    fn test_private_action_builder() {
        let position = WorldPositionBuilder::new()
            .at_coordinates(10.0, 20.0, 0.0)
            .build()
            .unwrap();

        let private = PrivateActionBuilder::new(InitActionBuilder::new(), "ego")
            .add_teleport_action(position)
            .add_speed_action(30.0)
            .build()
            .unwrap();

        assert_eq!(private.entity_ref.as_literal().unwrap(), "ego");
        assert_eq!(private.private_actions.len(), 2);

        // First action should be teleport
        assert!(private.private_actions[0].teleport_action.is_some());

        // Second action should be speed
        assert!(private.private_actions[1].longitudinal_action.is_some());
        let longitudinal = private.private_actions[1]
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
    fn test_private_action_builder_fluent() {
        let position = WorldPositionBuilder::new()
            .at_coordinates(0.0, 0.0, 0.0)
            .build()
            .unwrap();

        let init = InitActionBuilder::new()
            .create_private_action("ego")
            .add_teleport_action(position)
            .add_speed_action(25.0)
            .finish()
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
    }

    #[test]
    fn test_global_action_builder() {
        let global = GlobalActionBuilder::new(InitActionBuilder::new())
            .add_default_environment_action()
            .build()
            .unwrap();

        assert!(global.environment_action.is_some());
    }

    #[test]
    fn test_global_action_builder_fluent() {
        let init = InitActionBuilder::new()
            .create_global_action()
            .add_default_environment_action()
            .finish()
            .build()
            .unwrap();

        assert_eq!(init.actions.global_actions.len(), 1);
        assert!(init.actions.global_actions[0].environment_action.is_some());
    }

    #[test]
    fn test_combined_builders() {
        let position = WorldPositionBuilder::new()
            .at_coordinates(5.0, 10.0, 0.0)
            .build()
            .unwrap();

        let init = InitActionBuilder::new()
            .create_global_action()
            .add_default_environment_action()
            .finish()
            .create_private_action("ego")
            .add_teleport_action(position)
            .add_speed_action(40.0)
            .finish()
            .create_private_action("target")
            .add_speed_action(35.0)
            .finish()
            .build()
            .unwrap();

        assert_eq!(init.actions.global_actions.len(), 1);
        assert_eq!(init.actions.private_actions.len(), 2);

        // Check ego entity
        let ego_private = &init.actions.private_actions[0];
        assert_eq!(ego_private.entity_ref.as_literal().unwrap(), "ego");
        assert_eq!(ego_private.private_actions.len(), 2);

        // Check target entity
        let target_private = &init.actions.private_actions[1];
        assert_eq!(target_private.entity_ref.as_literal().unwrap(), "target");
        assert_eq!(target_private.private_actions.len(), 1);
    }
}

