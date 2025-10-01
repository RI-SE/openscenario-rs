//! Controller action builders (ActivateControllerAction, OverrideControllerValueAction, AssignControllerAction)

use crate::builder::{BuilderError, BuilderResult};
use crate::builder::actions::base::{ActionBuilder, ManeuverAction};
use crate::types::{
    actions::control::{
        ActivateControllerAction, OverrideControllerValueAction, AssignControllerAction,
        OverrideControllerValueActionThrottle, OverrideControllerValueActionBrake,
        OverrideControllerValueActionSteeringWheel, OverrideControllerValueActionGear,
        OverrideControllerValueActionParkingBrake, OverrideControllerValueActionClutch,
        ControllerAction, ManualGear
    },
    actions::wrappers::{PrivateAction, CorePrivateAction},
    basic::{Boolean, Double, Value, Int},
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
        };
        
        Ok(PrivateAction {
            action: CorePrivateAction::ControllerAction(
                ControllerAction {
                    assign_controller_action: None,
                    override_controller_value_action: None,
                    activate_controller_action: Some(activate_action),
                }
            ),
        })
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

/// Builder for override controller value actions
#[derive(Debug, Default)]
pub struct OverrideControllerValueActionBuilder {
    entity_ref: Option<String>,
    throttle: Option<(bool, f64)>,
    brake: Option<(bool, f64)>,
    steering: Option<(bool, f64)>,
    gear: Option<(bool, f64)>,
    parking_brake: Option<(bool, f64)>,
    clutch: Option<(bool, f64)>,
}

impl OverrideControllerValueActionBuilder {
    /// Create new override controller value action builder
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set target entity for this action
    pub fn for_entity(mut self, entity_ref: &str) -> Self {
        self.entity_ref = Some(entity_ref.to_string());
        self
    }
    
    /// Override throttle control
    pub fn throttle(mut self, active: bool, value: f64) -> Self {
        self.throttle = Some((active, value));
        self
    }
    
    /// Override brake control
    pub fn brake(mut self, active: bool, value: f64) -> Self {
        self.brake = Some((active, value));
        self
    }
    
    /// Override steering control
    pub fn steering(mut self, active: bool, value: f64) -> Self {
        self.steering = Some((active, value));
        self
    }
    
    /// Override gear control
    pub fn gear(mut self, active: bool, value: f64) -> Self {
        self.gear = Some((active, value));
        self
    }
    
    /// Override parking brake control
    pub fn parking_brake(mut self, active: bool, value: f64) -> Self {
        self.parking_brake = Some((active, value));
        self
    }
    
    /// Override clutch control
    pub fn clutch(mut self, active: bool, value: f64) -> Self {
        self.clutch = Some((active, value));
        self
    }
}

impl ActionBuilder for OverrideControllerValueActionBuilder {
    fn build_action(self) -> BuilderResult<PrivateAction> {
        self.validate()?;
        
        let override_action = OverrideControllerValueAction {
            throttle: self.throttle.map(|(active, value)| OverrideControllerValueActionThrottle {
                active: Boolean::literal(active),
                value: Double::literal(value),
            }),
            brake: self.brake.map(|(active, value)| OverrideControllerValueActionBrake {
                active: Boolean::literal(active),
                value: Double::literal(value),
            }),
            steering_wheel: self.steering.map(|(active, value)| OverrideControllerValueActionSteeringWheel {
                active: Boolean::literal(active),
                value: Double::literal(value),
            }),
            gear: self.gear.map(|(active, value)| OverrideControllerValueActionGear {
                active: Boolean::literal(active),
                manual_gear: Some(ManualGear {
                    gear: Int::literal(value as i32),
                }),
                automatic_gear: None,
            }),
            parking_brake: self.parking_brake.map(|(active, value)| OverrideControllerValueActionParkingBrake {
                active: Boolean::literal(active),
                force: Some(value),
            }),
            clutch: self.clutch.map(|(active, value)| OverrideControllerValueActionClutch {
                active: Boolean::literal(active),
                value: Double::literal(value),
            }),
        };
        
        Ok(PrivateAction {
            action: CorePrivateAction::ControllerAction(
                ControllerAction {
                    assign_controller_action: None,
                    override_controller_value_action: Some(override_action),
                    activate_controller_action: None,
                }
            ),
        })
    }
    
    fn validate(&self) -> BuilderResult<()> {
        if self.throttle.is_none() && self.brake.is_none() && self.steering.is_none() 
           && self.gear.is_none() && self.parking_brake.is_none() && self.clutch.is_none() {
            return Err(BuilderError::validation_error("At least one control override must be specified"));
        }
        Ok(())
    }
}

impl ManeuverAction for OverrideControllerValueActionBuilder {
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
    pub fn with_controller(mut self, name: &str, controller_type: ControllerType) -> Self {
        self.controller = Some(Controller {
            name: Value::Literal(name.to_string()),
            controller_type: Some(controller_type),
            parameter_declarations: None,
            properties: None,
        });
        self
    }
}

impl ActionBuilder for AssignControllerActionBuilder {
    fn build_action(self) -> BuilderResult<PrivateAction> {
        self.validate()?;
        
        let assign_action = AssignControllerAction {
            controller: self.controller,
            catalog_reference: None,
        };
        
        Ok(PrivateAction {
            action: CorePrivateAction::ControllerAction(
                ControllerAction {
                    assign_controller_action: Some(assign_action),
                    override_controller_value_action: None,
                    activate_controller_action: None,
                }
            ),
        })
    }
    
    fn validate(&self) -> BuilderResult<()> {
        if self.controller.is_none() {
            return Err(BuilderError::validation_error("Controller is required for assign controller action"));
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
        if let CorePrivateAction::ControllerAction(controller_action) = action.action {
            let activate = controller_action.activate_controller_action.unwrap();
            assert!(*activate.lateral.unwrap().as_literal().unwrap());
            assert!(*activate.longitudinal.unwrap().as_literal().unwrap());
            assert!(!*activate.lighting.unwrap().as_literal().unwrap());
            assert!(!*activate.animation.unwrap().as_literal().unwrap());
        } else {
            panic!("Expected ControllerAction");
        }
    }

    #[test]
    fn test_override_controller_value_action_builder() {
        let action = OverrideControllerValueActionBuilder::new()
            .for_entity("ego")
            .throttle(true, 0.8)
            .brake(false, 0.0)
            .steering(true, 0.2)
            .build_action()
            .unwrap();
            
        // Verify the action was built correctly
        if let CorePrivateAction::ControllerAction(controller_action) = action.action {
            let override_action = controller_action.override_controller_value_action.unwrap();
            
            let throttle = override_action.throttle.unwrap();
            assert!(*throttle.active.as_literal().unwrap());
            assert_eq!(*throttle.value.as_literal().unwrap(), 0.8);
            
            let brake = override_action.brake.unwrap();
            assert!(!*brake.active.as_literal().unwrap());
            assert_eq!(*brake.value.as_literal().unwrap(), 0.0);
            
            let steering = override_action.steering_wheel.unwrap();
            assert!(*steering.active.as_literal().unwrap());
            assert_eq!(*steering.value.as_literal().unwrap(), 0.2);
        } else {
            panic!("Expected ControllerAction");
        }
    }

    #[test]
    fn test_assign_controller_action_builder() {
        let controller = Controller {
            name: Value::Literal("TestController".to_string()),
            controller_type: Some(ControllerType::Movement),
            parameter_declarations: None,
            properties: None,
        };
        
        let action = AssignControllerActionBuilder::new()
            .for_entity("ego")
            .controller(controller)
            .build_action()
            .unwrap();
            
        // Verify the action was built correctly
        if let CorePrivateAction::ControllerAction(controller_action) = action.action {
            let assign = controller_action.assign_controller_action.unwrap();
            assert_eq!(assign.controller.unwrap().name.as_literal().unwrap(), "TestController");
        } else {
            panic!("Expected ControllerAction");
        }
    }
}