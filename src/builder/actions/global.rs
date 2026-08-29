//! Global action builders (EnvironmentAction, EntityAction, ParameterAction, TrafficAction, VariableAction)

use crate::builder::{BuilderError, BuilderResult};
use crate::types::{
    actions::wrappers::{
        AddEntityAction, DeleteEntityAction, EntityAction, EntityActionChoice, VariableAction,
        VariableActionChoice, VariableSetAction,
    },
    basic::OSString,
    environment::Environment,
    positions::Position,
    scenario::init::{EnvironmentAction, GlobalAction},
};

/// Builder for environment actions
#[derive(Debug, Default)]
pub struct EnvironmentActionBuilder {
    entity_ref: Option<String>,
    environment: Option<Environment>,
}

impl EnvironmentActionBuilder {
    /// Create new environment action builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set target entity for this action
    pub fn for_entity(mut self, entity_ref: &str) -> Self {
        self.entity_ref = Some(entity_ref.to_string());
        self
    }

    /// Set environment configuration
    pub fn with_environment(mut self, environment: Environment) -> Self {
        self.environment = Some(environment);
        self
    }

    /// Build the environment action
    pub fn build(self) -> BuilderResult<GlobalAction> {
        self.validate()?;

        let environment_action = EnvironmentAction {
            environment: Some(self.environment.unwrap()),
            catalog_reference: None,
        };

        Ok(GlobalAction {
            environment_action: Some(environment_action),
            ..Default::default()
        })
    }

    fn validate(&self) -> BuilderResult<()> {
        if self.environment.is_none() {
            return Err(BuilderError::validation_error(
                "Environment is required for environment action",
            ));
        }
        Ok(())
    }
}

/// Builder for entity actions (add/delete entities)
///
/// The XSD models `EntityAction` (`Schema/OpenSCENARIO.xsd:1128-1134`) as a **global** action:
/// a required `entityRef` attribute plus a choice of `AddEntityAction`, which carries the
/// position to spawn at, or `DeleteEntityAction`, which is empty. [`Self::build`] therefore
/// produces a [`GlobalAction`], not a private one.
///
/// # Example
///
/// ```rust
/// use openscenario_rs::builder::EntityActionBuilder;
/// use openscenario_rs::types::positions::Position;
///
/// let action = EntityActionBuilder::new()
///     .for_entity("spawned_vehicle")
///     .add_entity(Position::default())
///     .build()?;
/// # Ok::<(), openscenario_rs::builder::BuilderError>(())
/// ```
#[derive(Debug, Default)]
pub struct EntityActionBuilder {
    entity_ref: Option<String>,
    action_type: Option<EntityActionType>,
}

#[derive(Debug)]
enum EntityActionType {
    Add(Box<Position>),
    Delete,
}

impl EntityActionBuilder {
    /// Create new entity action builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set target entity for this action
    pub fn for_entity(mut self, entity_ref: &str) -> Self {
        self.entity_ref = Some(entity_ref.to_string());
        self
    }

    /// Configure to add the entity at the given position
    pub fn add_entity(mut self, position: Position) -> Self {
        self.action_type = Some(EntityActionType::Add(Box::new(position)));
        self
    }

    /// Configure to delete the entity
    pub fn delete_entity(mut self) -> Self {
        self.action_type = Some(EntityActionType::Delete);
        self
    }

    /// Build the entity action
    pub fn build(self) -> BuilderResult<GlobalAction> {
        let entity_ref = self
            .entity_ref
            .ok_or_else(|| BuilderError::missing_field("entity_ref", ".for_entity(name)"))?;

        let action = match self.action_type.ok_or_else(|| {
            BuilderError::missing_field("action_type", ".add_entity(position) or .delete_entity()")
        })? {
            EntityActionType::Add(position) => {
                EntityActionChoice::AddEntityAction(AddEntityAction {
                    position: *position,
                })
            }
            EntityActionType::Delete => {
                EntityActionChoice::DeleteEntityAction(DeleteEntityAction {})
            }
        };

        Ok(GlobalAction {
            entity_action: Some(EntityAction {
                entity_ref: OSString::literal(entity_ref),
                action,
            }),
            ..Default::default()
        })
    }
}

/// Builder for variable actions (set variable values)
///
/// Like [`EntityActionBuilder`], this produces a [`GlobalAction`]. The XSD's `VariableAction`
/// takes a `variableRef` attribute and a choice of `SetAction` or `ModifyAction`; only
/// `SetAction` is exposed here. Note that a variable action targets a *variable*, not an
/// entity, so there is no entity reference to set.
///
/// # Example
///
/// ```rust
/// use openscenario_rs::builder::VariableActionBuilder;
///
/// let action = VariableActionBuilder::new()
///     .set_variable("ego_speed", 27.8)
///     .build()?;
/// # Ok::<(), openscenario_rs::builder::BuilderError>(())
/// ```
#[derive(Debug, Default)]
pub struct VariableActionBuilder {
    variable_name: Option<String>,
    variable_value: Option<f64>,
}

impl VariableActionBuilder {
    /// Create new variable action builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set variable name and value
    pub fn set_variable(mut self, name: &str, value: f64) -> Self {
        self.variable_name = Some(name.to_string());
        self.variable_value = Some(value);
        self
    }

    /// Build the variable action
    pub fn build(self) -> BuilderResult<GlobalAction> {
        let variable_ref = self.variable_name.ok_or_else(|| {
            BuilderError::missing_field("variable_name", ".set_variable(name, value)")
        })?;
        let value = self.variable_value.ok_or_else(|| {
            BuilderError::missing_field("variable_value", ".set_variable(name, value)")
        })?;

        Ok(GlobalAction {
            variable_action: Some(VariableAction {
                variable_ref: OSString::literal(variable_ref),
                action: VariableActionChoice::VariableSetAction(VariableSetAction {
                    value: OSString::literal(value.to_string()),
                }),
            }),
            ..Default::default()
        })
    }
}

// Note: Environment actions are global and don't implement ManeuverAction

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::basic::Value;
    use crate::types::environment::{RoadCondition, TimeOfDay, Weather};

    /// The XSD models EntityAction as a global action, so the built value must land in
    /// `GlobalAction::entity_action` and serialize under the schema's element names.
    #[test]
    fn entity_action_builds_a_global_action_and_round_trips() {
        for (builder, expected_element) in [
            (
                EntityActionBuilder::new()
                    .for_entity("target")
                    .delete_entity(),
                "DeleteEntityAction",
            ),
            (
                EntityActionBuilder::new()
                    .for_entity("spawned")
                    .add_entity(Position::default()),
                "AddEntityAction",
            ),
        ] {
            let action = builder.build().unwrap();
            let entity_action = action.entity_action.as_ref().unwrap();

            let xml = quick_xml::se::to_string_with_root("EntityAction", entity_action).unwrap();
            assert!(
                xml.contains(expected_element),
                "expected <{expected_element}> in {xml}"
            );
            assert!(
                xml.contains("entityRef="),
                "expected the entityRef attribute in {xml}"
            );
        }
    }

    #[test]
    fn entity_action_requires_an_entity_and_a_choice() {
        let missing_entity = EntityActionBuilder::new().delete_entity().build();
        assert!(matches!(
            missing_entity,
            Err(BuilderError::MissingField { ref field, .. }) if field == "entity_ref"
        ));

        let missing_choice = EntityActionBuilder::new().for_entity("target").build();
        assert!(matches!(
            missing_choice,
            Err(BuilderError::MissingField { ref field, .. }) if field == "action_type"
        ));
    }

    #[test]
    fn variable_action_builds_a_global_action() {
        let action = VariableActionBuilder::new()
            .set_variable("speed_limit", 50.0)
            .build()
            .unwrap();

        let variable_action = action.variable_action.as_ref().unwrap();
        assert_eq!(
            variable_action.variable_ref.as_literal().unwrap(),
            "speed_limit"
        );

        let xml = quick_xml::se::to_string_with_root("VariableAction", variable_action).unwrap();
        assert!(xml.contains("SetAction"), "expected <SetAction> in {xml}");
    }

    #[test]
    fn test_environment_action_builder() {
        let environment = Environment {
            name: Value::literal("TestEnvironment".to_string()),
            parameter_declarations: None,
            time_of_day: Some(TimeOfDay {
                animation: crate::types::basic::Boolean::literal(false),
                date_time: "2021-01-01T12:00:00".to_string(),
            }),
            weather: Some(Weather::default()),
            road_condition: Some(RoadCondition {
                friction_scale_factor: crate::types::basic::Double::literal(1.0),
                wetness: None,
                properties: None,
            }),
        };

        let action = EnvironmentActionBuilder::new()
            .for_entity("ego")
            .with_environment(environment)
            .build()
            .unwrap();

        // Verify the action was built correctly
        assert!(action.environment_action.is_some());
        assert_eq!(
            action
                .environment_action
                .unwrap()
                .environment
                .unwrap()
                .name
                .as_literal()
                .unwrap(),
            "TestEnvironment"
        );
    }
}
