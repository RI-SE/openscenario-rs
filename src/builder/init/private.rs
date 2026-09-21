//! Private and global action builders for entity-specific initialization

use super::actions::InitActionBuilder;
use crate::builder::actions::ActionBuilder as ActionBuilderTrait;
use crate::builder::actions::{
    AssignRouteActionBuilder, LongitudinalDistanceActionBuilder, SpeedProfileActionBuilder,
    SynchronizeActionBuilder, VisibilityActionBuilder,
};
use crate::builder::{BuilderError, BuilderResult};
use crate::types::basic::Value;
use crate::types::{
    actions::movement::{
        LongitudinalAction as LongitudinalActionType,
        LongitudinalActionChoice as MovementLongitudinalChoice, SpeedAction, SpeedActionTarget,
        TeleportAction, TransitionDynamics,
    },
    actions::wrappers::PrivateAction as PrivateActionWrapper,
    basic::Double,
    enums::{DynamicsDimension, DynamicsShape},
    environment::Environment,
    positions::Position,
    routing::Route,
    scenario::init::{EnvironmentAction, GlobalAction, LongitudinalAction, Private, PrivateAction},
};

/// Builder for private actions specific to individual entities
#[derive(Debug)]
pub struct PrivateActionBuilder {
    parent: InitActionBuilder,
    entity_ref: String,
    actions: Vec<PrivateActionWrapper>,
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

    /// Add a teleport action to position an entity
    pub fn add_teleport_action(mut self, position: Position) -> Self {
        let action = PrivateActionWrapper::TeleportAction(TeleportAction { position });
        self.actions.push(action);
        self
    }

    /// Add a speed action to set initial velocity
    pub fn add_speed_action(mut self, speed: f64) -> Self {
        let speed_action = SpeedAction {
            speed_action_dynamics: TransitionDynamics {
                dynamics_dimension: Value::Literal(DynamicsDimension::Time),
                dynamics_shape: Value::Literal(DynamicsShape::Step),
                following_mode: None,
                value: Double::literal(1.0),
            },
            speed_action_target: SpeedActionTarget {
                target:
                    crate::types::actions::movement::SpeedActionTargetChoice::AbsoluteTargetSpeed(
                        crate::types::actions::movement::AbsoluteTargetSpeed {
                            value: Double::literal(speed),
                        },
                    ),
            },
        };

        let action = PrivateActionWrapper::LongitudinalAction(LongitudinalActionType {
            longitudinal_action_choice: MovementLongitudinalChoice::SpeedAction(speed_action),
        });
        self.actions.push(action);
        self
    }

    /// Add a custom private action
    pub fn add_action(mut self, action: PrivateActionWrapper) -> Self {
        self.actions.push(action);
        self
    }

    // ========== LONGITUDINAL ACTIONS ==========

    /// Add longitudinal distance action (convenience)
    pub fn add_longitudinal_distance_action(mut self, target_entity: &str, distance: f64) -> Self {
        let builder_action = LongitudinalDistanceActionBuilder::new()
            .from_entity(target_entity)
            .at_distance(distance)
            .build_action()
            .unwrap();

        self.actions.push(builder_action);
        self
    }

    /// Add speed profile action with direct entries (convenience)
    pub fn add_speed_profile_action_direct(mut self, entries: Vec<(f64, f64)>) -> Self {
        let mut builder = SpeedProfileActionBuilder::new();
        for (time, speed) in entries {
            builder = builder.add_entry_direct(time, speed);
        }

        if let Ok(builder_action) = builder.build_action() {
            self.actions.push(builder_action);
        }
        self
    }

    // ========== ROUTING ACTIONS ==========

    /// Add assign route action with direct route (convenience)
    pub fn add_assign_route_action(mut self, route: Route) -> Self {
        let builder_action = AssignRouteActionBuilder::new()
            .with_direct_route(route)
            .build_action()
            .unwrap();

        self.actions.push(builder_action);
        self
    }

    /// Add assign route action with catalog reference (convenience)
    pub fn add_assign_route_catalog(
        mut self,
        catalog_name: impl Into<String>,
        entry_name: impl Into<String>,
    ) -> Self {
        let builder_action = AssignRouteActionBuilder::new()
            .with_catalog_route(catalog_name, entry_name)
            .build_action()
            .unwrap();

        self.actions.push(builder_action);
        self
    }

    // ========== SYNCHRONIZATION & VISIBILITY ==========

    /// Add synchronize action (convenience)
    pub fn add_synchronize_action(
        mut self,
        master_entity: &str,
        master_position: Position,
        entity_position: Position,
    ) -> Self {
        let builder_action = SynchronizeActionBuilder::new()
            .with_master(master_entity)
            .master_position(master_position)
            .entity_position(entity_position)
            .build_action()
            .unwrap();

        self.actions.push(builder_action);
        self
    }

    /// Add visibility action (convenience)
    pub fn add_visibility_action(mut self, graphics: bool, sensors: bool, traffic: bool) -> Self {
        let builder_action = VisibilityActionBuilder::new()
            .graphics(graphics)
            .sensors(sensors)
            .traffic(traffic)
            .build_action()
            .unwrap();

        self.actions.push(builder_action);
        self
    }

    /// Make entity visible (convenience)
    pub fn make_visible(self) -> Self {
        self.add_visibility_action(true, true, true)
    }

    /// Make entity invisible (convenience)
    pub fn make_invisible(self) -> Self {
        self.add_visibility_action(false, false, false)
    }

    // ========== INTERNAL HELPER ==========

    /// Convert from the builder's `PrivateAction` enum to the `Init` block's `PrivateAction`.
    ///
    /// The match is exhaustive by construction. An earlier version mapped seven of the ten
    /// branches and sent the rest to a catch-all that produced an action naming no branch at
    /// all, so an activate-controller, appearance or trailer action built through this builder
    /// was silently discarded and re-serialized as an empty `<PrivateAction/>`. The `$value`
    /// shape of `PrivateAction` makes that value unconstructible, hence the catch-all could
    /// not survive the conversion.
    fn convert_to_init_action(action: PrivateActionWrapper) -> PrivateAction {
        match action {
            PrivateActionWrapper::LongitudinalAction(long_action) => {
                PrivateAction::longitudinal(match long_action.longitudinal_action_choice {
                    MovementLongitudinalChoice::SpeedAction(a) => LongitudinalAction::speed(a),
                    MovementLongitudinalChoice::LongitudinalDistanceAction(a) => {
                        LongitudinalAction::longitudinal_distance(a)
                    }
                    MovementLongitudinalChoice::SpeedProfileAction(a) => {
                        LongitudinalAction::speed_profile(a)
                    }
                })
            }
            PrivateActionWrapper::LateralAction(a) => PrivateAction::lateral(a),
            PrivateActionWrapper::VisibilityAction(a) => PrivateAction::visibility(a),
            PrivateActionWrapper::SynchronizeAction(a) => PrivateAction::synchronize(a),
            PrivateActionWrapper::ActivateControllerAction(a) => {
                PrivateAction::activate_controller(a)
            }
            PrivateActionWrapper::ControllerAction(a) => PrivateAction::controller(a),
            PrivateActionWrapper::TeleportAction(a) => PrivateAction::teleport(a),
            PrivateActionWrapper::RoutingAction(a) => PrivateAction::routing(a),
            PrivateActionWrapper::AppearanceAction(a) => PrivateAction::appearance(a),
            PrivateActionWrapper::TrailerAction(a) => PrivateAction::trailer(a),
        }
    }

    /// Finish building and return to parent
    pub fn finish(self) -> InitActionBuilder {
        let private_actions: Vec<PrivateAction> = self
            .actions
            .into_iter()
            .map(Self::convert_to_init_action)
            .collect();

        let private = Private {
            entity_ref: Value::literal(self.entity_ref),
            private_actions,
        };
        self.parent.add_private(private)
    }

    /// Build the private action container
    pub fn build(self) -> BuilderResult<Private> {
        let private_actions: Vec<PrivateAction> = self
            .actions
            .into_iter()
            .map(Self::convert_to_init_action)
            .collect();

        Ok(Private {
            entity_ref: Value::literal(self.entity_ref),
            private_actions,
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
        self.environment_action = Some(EnvironmentAction::environment(environment));
        self
    }

    /// Add an environment action with a named environment
    ///
    /// XSD `Environment` (`Schema/OpenSCENARIO.xsd:1186-1194`) requires `@name`; there is no
    /// schema default, so the caller supplies it rather than getting a silently invented
    /// `"DefaultEnvironment"`.
    pub fn add_named_environment_action(mut self, name: &str) -> Self {
        self.environment_action = Some(EnvironmentAction::environment(Environment::new(name)));
        self
    }

    /// Finish building and return to parent.
    ///
    /// XSD `GlobalAction` (`Schema/OpenSCENARIO.xsd:1282-1295`) is a bare `xsd:choice`, so a
    /// global action naming no branch does not exist. This method has no error channel, hence
    /// a builder on which no branch was selected contributes nothing instead of contributing
    /// an empty `<GlobalAction/>`, which is what the previous shape emitted. Use
    /// [`Self::build`] where the omission should be reported.
    pub fn finish(self) -> InitActionBuilder {
        match self.environment_action {
            Some(environment_action) => self
                .parent
                .add_global(GlobalAction::environment(environment_action)),
            None => self.parent,
        }
    }

    /// Build the global action.
    ///
    /// Fails when no branch was selected, since the schema's choice requires exactly one.
    pub fn build(self) -> BuilderResult<GlobalAction> {
        let environment_action = self.environment_action.ok_or_else(|| {
            BuilderError::missing_field(
                "environment_action",
                ".add_environment_action(environment) or .add_named_environment_action(name)",
            )
        })?;
        Ok(GlobalAction::environment(environment_action))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::positions::WorldPositionBuilder;
    use crate::types::scenario::init::{LongitudinalActionChoice, PrivateActionChoice};

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
        assert_eq!(private.private_actions[0].action_type(), "TeleportAction");

        // Second action should be speed
        let PrivateActionChoice::LongitudinalAction(longitudinal) =
            &private.private_actions[1].action
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

    /// The three branches a catch-all used to swallow. Each was turned into a private action
    /// naming no branch, which re-serialized as an empty `<PrivateAction/>` and dropped the
    /// payload. One assertion per branch, so a regression in one does not hide the others.
    #[test]
    fn builder_preserves_the_branches_the_catch_all_used_to_swallow() {
        for (wrapper, expected) in [
            (
                PrivateActionWrapper::ActivateControllerAction(
                    crate::types::actions::control::ActivateControllerAction::default(),
                ),
                "ActivateControllerAction",
            ),
            (
                PrivateActionWrapper::TrailerAction(
                    crate::types::actions::trailer::TrailerAction {
                        choice: crate::types::actions::trailer::TrailerActionChoice::DisconnectTrailerAction(
                            crate::types::actions::trailer::DisconnectTrailerAction {},
                        ),
                    },
                ),
                "TrailerAction",
            ),
        ] {
            let private = PrivateActionBuilder::new(InitActionBuilder::new(), "ego")
                .add_action(wrapper)
                .build()
                .unwrap();

            assert_eq!(private.private_actions.len(), 1);
            assert_eq!(private.private_actions[0].action_type(), expected);

            let xml =
                quick_xml::se::to_string_with_root("Private", &private).expect("serialize failed");
            assert!(xml.contains(expected), "expected <{expected}> in {xml}");
        }
    }

    #[test]
    fn global_action_builder_reports_a_missing_branch_instead_of_emitting_an_empty_element() {
        let result = GlobalActionBuilder::new(InitActionBuilder::new()).build();
        assert!(
            matches!(result, Err(BuilderError::MissingField { ref field, .. }) if field == "environment_action"),
            "a global action with no branch selected must be reported, got {result:?}"
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
            .add_named_environment_action("TestEnvironment")
            .build()
            .unwrap();

        assert_eq!(global.action_type(), "EnvironmentAction");
    }

    #[test]
    fn test_global_action_builder_fluent() {
        let init = InitActionBuilder::new()
            .create_global_action()
            .add_named_environment_action("TestEnvironment")
            .finish()
            .build()
            .unwrap();

        assert_eq!(init.actions.global_actions.len(), 1);
        assert_eq!(
            init.actions.global_actions[0].action_type(),
            "EnvironmentAction"
        );
    }

    #[test]
    fn test_combined_builders() {
        let position = WorldPositionBuilder::new()
            .at_coordinates(5.0, 10.0, 0.0)
            .build()
            .unwrap();

        let init = InitActionBuilder::new()
            .create_global_action()
            .add_named_environment_action("TestEnvironment")
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
