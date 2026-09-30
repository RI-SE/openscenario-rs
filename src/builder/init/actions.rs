//! Main init action builder implementation

use super::private::{GlobalActionBuilder, PrivateActionBuilder};
use crate::builder::BuilderResult;
use crate::types::basic::Value;
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

    /// Add a global environment action with a named environment
    ///
    /// XSD `Environment` (`Schema/OpenSCENARIO.xsd:1186-1194`) requires `@name`; there is no
    /// schema default, so the caller supplies it rather than getting a silently invented
    /// `"DefaultEnvironment"`.
    pub fn add_global_environment_action(mut self, name: &str) -> Self {
        let global_action =
            GlobalAction::environment(EnvironmentAction::environment(Environment::new(name)));
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

    /// Create a private action builder for an entity
    pub fn create_private_action(self, entity_ref: &str) -> PrivateActionBuilder {
        PrivateActionBuilder::new(self, entity_ref)
    }

    /// Append one already-built private action to `entity_ref`, reusing that entity's
    /// `<Private>` container when it already exists.
    ///
    /// A live `Private` already holds at least one action, so growing its `MinVec` by one
    /// cannot fall below that bound. `MinVec::push` says exactly that, in place, which
    /// removes the earlier take-apart-and-rebuild-through-`MinVec::new` dance along with the
    /// `Result` it forced onto every caller.
    fn push_private_action(
        &mut self,
        entity_ref: &str,
        action: crate::types::scenario::init::PrivateAction,
    ) {
        let existing = self
            .private_actions
            .iter()
            .position(|p| p.entity_ref.as_literal().unwrap_or(&String::new()) == entity_ref);

        match existing {
            Some(index) => self.private_actions[index].private_actions.push(action),
            None => self
                .private_actions
                .push(Private::from_min(entity_ref, action, Vec::new())),
        }
    }

    /// Add a teleport action for an entity (convenience method)
    pub fn add_teleport_action(mut self, entity_ref: &str, position: Position) -> Self {
        let action = crate::types::scenario::init::PrivateAction::teleport(
            crate::types::actions::movement::TeleportAction { position },
        );
        self.push_private_action(entity_ref, action);
        self
    }

    /// Add a speed action for an entity (convenience method)
    pub fn add_speed_action(mut self, entity_ref: &str, speed: f64) -> Self {
        let speed_action = crate::types::actions::movement::SpeedAction {
            speed_action_dynamics: crate::types::actions::movement::TransitionDynamics {
                dynamics_dimension: Value::Literal(crate::types::enums::DynamicsDimension::Time),
                dynamics_shape: Value::Literal(crate::types::enums::DynamicsShape::Step),
                following_mode: None,
                value: crate::types::basic::Double::literal(1.0),
            },
            speed_action_target: crate::types::actions::movement::SpeedActionTarget {
                target:
                    crate::types::actions::movement::SpeedActionTargetChoice::AbsoluteTargetSpeed(
                        crate::types::actions::movement::AbsoluteTargetSpeed {
                            value: crate::types::basic::Double::literal(speed),
                        },
                    ),
            },
        };
        let action = crate::types::scenario::init::PrivateAction::longitudinal(
            crate::types::scenario::init::LongitudinalAction::speed(speed_action),
        );
        self.push_private_action(entity_ref, action);
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
                user_defined_actions: Vec::new(),
                private_actions: self.private_actions,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::positions::WorldPositionBuilder;
    use crate::types::scenario::init::{LongitudinalActionChoice, PrivateActionChoice};

    #[test]
    fn test_init_action_builder_empty() {
        let init = InitActionBuilder::new().build().unwrap();

        assert!(init.actions.global_actions.is_empty());
        assert!(init.actions.private_actions.is_empty());
    }

    #[test]
    fn test_init_action_builder_with_environment() {
        let init = InitActionBuilder::new()
            .add_global_environment_action("TestEnvironment")
            .build()
            .unwrap();

        assert_eq!(init.actions.global_actions.len(), 1);
        assert_eq!(
            init.actions.global_actions[0].action_type(),
            "EnvironmentAction"
        );
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
        let PrivateActionChoice::LongitudinalAction(longitudinal) =
            &init.actions.private_actions[0].private_actions[0].action
        else {
            panic!("expected the LongitudinalAction branch");
        };
        let LongitudinalActionChoice::SpeedAction(speed_action) = &longitudinal.action else {
            panic!("expected the SpeedAction branch");
        };
        let crate::types::actions::movement::SpeedActionTargetChoice::AbsoluteTargetSpeed(absolute) =
            &speed_action.speed_action_target.target
        else {
            panic!("Expected AbsoluteTargetSpeed branch");
        };
        assert_eq!(absolute.value.as_literal().unwrap(), &30.0);
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
        assert_eq!(
            init.actions.private_actions[0].private_actions[0].action_type(),
            "TeleportAction"
        );
        // Second action should be speed
        assert_eq!(
            init.actions.private_actions[0].private_actions[1].action_type(),
            "LongitudinalAction"
        );
    }

    /// `push_private_action` grows an existing `Private`'s `MinVec` in place with
    /// `MinVec::push`, rather than removing the container, rebuilding it through the
    /// fallible `MinVec::new`, and reinserting it. This adds a third action to the same
    /// entity to exercise that path past its first growth, and the whole chain below takes
    /// no `Result` at any step — a change to `push_private_action` that reintroduced the
    /// old rebuild-through-`new` pattern would force a `?` back into this call site and
    /// fail to compile.
    #[test]
    fn test_init_action_builder_grows_same_entity_past_two_actions() {
        let position = WorldPositionBuilder::new()
            .at_coordinates(0.0, 0.0, 0.0)
            .build()
            .unwrap();

        let init = InitActionBuilder::new()
            .add_teleport_action("ego", position)
            .add_speed_action("ego", 10.0)
            .add_speed_action("ego", 20.0)
            .build()
            .unwrap();

        assert_eq!(init.actions.private_actions.len(), 1);
        assert_eq!(init.actions.private_actions[0].private_actions.len(), 3);
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
