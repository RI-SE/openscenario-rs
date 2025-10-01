//! Global action builders (EnvironmentAction, EntityAction, ParameterAction, TrafficAction, VariableAction)

use crate::builder::{BuilderError, BuilderResult};
use crate::builder::actions::base::ActionBuilder;
use crate::types::{
    scenario::init::{GlobalAction, EnvironmentAction},
    environment::Environment,
    actions::wrappers::PrivateAction,
    basic::OSString,
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
            environment: self.environment.unwrap(),
        };
        
        Ok(GlobalAction {
            environment_action: Some(environment_action),
        })
    }
    
    /// Build the environment action as a private action (for compatibility)
    pub fn build_action(self) -> BuilderResult<PrivateAction> {
        // Environment actions are global actions, not private actions
        // For now, return an error indicating this is not supported
        Err(BuilderError::validation_error("Environment actions are global actions and cannot be used as private actions"))
    }
    
    fn validate(&self) -> BuilderResult<()> {
        if self.environment.is_none() {
            return Err(BuilderError::validation_error("Environment is required for environment action"));
        }
        Ok(())
    }
}

/// Builder for entity actions (add/delete entities)
#[derive(Debug, Default)]
pub struct EntityActionBuilder {
    entity_ref: Option<String>,
    action_type: Option<EntityActionType>,
}

#[derive(Debug)]
enum EntityActionType {
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
    
    /// Configure to delete the entity
    pub fn delete_entity(mut self) -> Self {
        self.action_type = Some(EntityActionType::Delete);
        self
    }
    
    /// Build the entity action as a private action (placeholder)
    pub fn build_action(self) -> BuilderResult<PrivateAction> {
        // Entity actions are typically global actions, not private actions
        // For now, return an error indicating this is not supported
        Err(BuilderError::validation_error("Entity actions are not yet implemented as private actions"))
    }
}

/// Builder for variable actions (set variable values)
#[derive(Debug, Default)]
pub struct VariableActionBuilder {
    entity_ref: Option<String>,
    variable_name: Option<String>,
    variable_value: Option<f64>,
}

impl VariableActionBuilder {
    /// Create new variable action builder
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set target entity for this action
    pub fn for_entity(mut self, entity_ref: &str) -> Self {
        self.entity_ref = Some(entity_ref.to_string());
        self
    }
    
    /// Set variable name and value
    pub fn set_variable(mut self, name: &str, value: f64) -> Self {
        self.variable_name = Some(name.to_string());
        self.variable_value = Some(value);
        self
    }
    
    /// Build the variable action as a private action (placeholder)
    pub fn build_action(self) -> BuilderResult<PrivateAction> {
        // Variable actions are typically global actions, not private actions
        // For now, return an error indicating this is not supported
        Err(BuilderError::validation_error("Variable actions are not yet implemented as private actions"))
    }
}

// Note: Environment actions are global and don't implement ManeuverAction

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::environment::{Weather, RoadCondition, TimeOfDay};
    use crate::types::basic::Value;

    #[test]
    fn test_environment_action_builder() {
        let environment = Environment {
            name: Value::literal("TestEnvironment".to_string()),
            time_of_day: TimeOfDay::default(),
            weather: Weather::default(),
            road_condition: RoadCondition::default(),
        };
        
        let action = EnvironmentActionBuilder::new()
            .for_entity("ego")
            .with_environment(environment)
            .build()
            .unwrap();
            
        // Verify the action was built correctly
        assert!(action.environment_action.is_some());
        assert_eq!(action.environment_action.unwrap().environment.name.as_literal().unwrap(), "TestEnvironment");
    }
}