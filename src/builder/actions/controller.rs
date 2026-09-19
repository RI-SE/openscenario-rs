//! Controller action builders (ActivateControllerAction, AssignControllerAction)

use crate::builder::actions::base::{ActionBuilder, ManeuverAction};
use crate::builder::{BuilderError, BuilderResult};
use crate::types::basic::Value;
use crate::types::{
    actions::control::{
        ActivateControllerAction, AssignControllerAction, AssignControllerActionChoice,
        ControllerAction, ControllerActionChoice,
    },
    actions::wrappers::PrivateAction,
    basic::Boolean,
    controllers::Controller,
    enums::ControllerType,
};

/// Builder for activate controller actions
#[derive(Debug, Default)]
pub struct ActivateControllerActionBuilder {
    entity_ref: Option<String>,
    lateral: bool,
    longitudinal: bool,
    lighting: bool,
    animation: bool,
}

impl ActivateControllerActionBuilder {
    /// Create new activate controller action builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set target entity for this action
    pub fn for_entity(mut self, entity_ref: &str) -> Self {
        self.entity_ref = Some(entity_ref.to_string());
        self
    }

    /// Enable lateral control
    pub fn lateral(mut self, enable: bool) -> Self {
        self.lateral = enable;
        self
    }

    /// Enable longitudinal control
    pub fn longitudinal(mut self, enable: bool) -> Self {
        self.longitudinal = enable;
        self
    }

    /// Enable lighting control
    pub fn lighting(mut self, enable: bool) -> Self {
        self.lighting = enable;
        self
    }

    /// Enable animation control
    pub fn animation(mut self, enable: bool) -> Self {
        self.animation = enable;
        self
    }

    /// Enable all control domains
    pub fn all_domains(mut self) -> Self {
        self.lateral = true;
        self.longitudinal = true;
        self.lighting = true;
        self.animation = true;
        self
    }

    /// Enable only movement controls (lateral and longitudinal)
    pub fn movement_only(mut self) -> Self {
        self.lateral = true;
        self.longitudinal = true;
        self.lighting = false;
        self.animation = false;
        self
    }
}

impl ActionBuilder for ActivateControllerActionBuilder {
    fn build_action(self) -> BuilderResult<PrivateAction> {
        let activate_action = ActivateControllerAction {
            lateral: Some(Boolean::literal(self.lateral)),
            longitudinal: Some(Boolean::literal(self.longitudinal)),
            lighting: Some(Boolean::literal(self.lighting)),
            animation: Some(Boolean::literal(self.animation)),
            ..Default::default()
        };

        Ok(PrivateAction::ControllerAction(ControllerAction {
            controller_action: ControllerActionChoice::ActivateControllerAction(activate_action),
        }))
    }

    fn validate(&self) -> BuilderResult<()> {
        // No specific validation needed for activate controller action
        Ok(())
    }
}

impl ManeuverAction for ActivateControllerActionBuilder {
    fn entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

/// Builder for assign controller actions
#[derive(Debug, Default)]
pub struct AssignControllerActionBuilder {
    entity_ref: Option<String>,
    controller: Option<Controller>,
}

impl AssignControllerActionBuilder {
    /// Create new assign controller action builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set target entity for this action
    pub fn for_entity(mut self, entity_ref: &str) -> Self {
        self.entity_ref = Some(entity_ref.to_string());
        self
    }

    /// Set controller to assign
    pub fn controller(mut self, controller: Controller) -> Self {
        self.controller = Some(controller);
        self
    }

    /// Create controller with name and type
    /// Create controller with name and a parameterized `controllerType` --
    /// the attribute's `xsd:union` admits a `parameter` member alongside the enumeration, so `$name` is schema-valid here; `name` omits the `$`.
    pub fn with_controller_type_param(mut self, name: &str, type_param: &str) -> Self {
        self.controller = Some(Controller {
            name: Value::Literal(name.to_string()),
            controller_type: Some(Value::Parameter(type_param.to_string())),
            parameter_declarations: None,
            properties: None,
        });
        self
    }

    /// Create controller with name and type
    pub fn with_controller(mut self, name: &str, controller_type: ControllerType) -> Self {
        self.controller = Some(Controller {
            name: Value::Literal(name.to_string()),
            controller_type: Some(Value::Literal(controller_type)),
            parameter_declarations: None,
            properties: None,
        });
        self
    }
}

impl ActionBuilder for AssignControllerActionBuilder {
    fn build_action(self) -> BuilderResult<PrivateAction> {
        self.validate()?;

        // `validate` above established that `self.controller` is `Some`, which is
        // what lets the choice branch be named unconditionally here.
        let controller = self.controller.ok_or_else(|| {
            BuilderError::validation_error("Controller is required for assign controller action")
        })?;

        let assign_action = AssignControllerAction {
            controller: Some(AssignControllerActionChoice::Controller(controller)),
            ..AssignControllerAction::default()
        };

        Ok(PrivateAction::ControllerAction(ControllerAction {
            controller_action: ControllerActionChoice::AssignControllerAction(assign_action),
        }))
    }

    fn validate(&self) -> BuilderResult<()> {
        if self.controller.is_none() {
            return Err(BuilderError::validation_error(
                "Controller is required for assign controller action",
            ));
        }
        Ok(())
    }
}

impl ManeuverAction for AssignControllerActionBuilder {
    fn entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_activate_controller_action_builder() {
        let action = ActivateControllerActionBuilder::new()
            .for_entity("ego")
            .movement_only()
            .build_action()
            .unwrap();

        // Verify the action was built correctly
        if let PrivateAction::ControllerAction(controller_action) = action {
            let ControllerActionChoice::ActivateControllerAction(activate) =
                controller_action.controller_action
            else {
                panic!("Expected ActivateControllerAction branch");
            };
            assert!(*activate.lateral.unwrap().as_literal().unwrap());
            assert!(*activate.longitudinal.unwrap().as_literal().unwrap());
            assert!(!*activate.lighting.unwrap().as_literal().unwrap());
            assert!(!*activate.animation.unwrap().as_literal().unwrap());
        } else {
            panic!("Expected ControllerAction");
        }
    }

    #[test]
    fn test_assign_controller_action_builder() {
        let controller = Controller {
            name: Value::Literal("TestController".to_string()),
            controller_type: Some(Value::Literal(ControllerType::Movement)),
            parameter_declarations: None,
            properties: None,
        };

        let action = AssignControllerActionBuilder::new()
            .for_entity("ego")
            .controller(controller)
            .build_action()
            .unwrap();

        // Verify the action was built correctly
        if let PrivateAction::ControllerAction(controller_action) = action {
            let ControllerActionChoice::AssignControllerAction(assign) =
                controller_action.controller_action
            else {
                panic!("Expected AssignControllerAction branch");
            };
            let Some(AssignControllerActionChoice::Controller(controller)) = assign.controller
            else {
                panic!("Expected Controller branch");
            };
            assert_eq!(controller.name.as_literal().unwrap(), "TestController");
        } else {
            panic!("Expected ControllerAction");
        }
    }
}
