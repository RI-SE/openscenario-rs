//! Maneuver builders for entity behavior sequences

use crate::builder::{BuilderError, BuilderResult, actions::{SpeedActionBuilder, TeleportActionBuilder, ActionBuilder}};
use crate::types::{
    scenario::{
        story::{Maneuver, Event, StoryAction, StoryPrivateAction},
        triggers::{Trigger, ConditionGroup},
    },
    actions::wrappers::PrivateAction,
    basic::OSString,
    enums::Priority,
};

/// Builder for maneuvers (entity behavior sequences)
pub struct ManeuverBuilder<'parent> {
    parent: &'parent mut super::story::ActBuilder<'parent>,
    maneuver_name: String,
    entity_ref: String,
    events: Vec<Event>,
}

impl<'parent> ManeuverBuilder<'parent> {
    pub fn new(parent: &'parent mut super::story::ActBuilder<'parent>, name: &str, entity_ref: &str) -> Self {
        Self {
            parent,
            maneuver_name: name.to_string(),
            entity_ref: entity_ref.to_string(),
            events: Vec::new(),
        }
    }
    
    /// Add a speed action event
    pub fn add_speed_action(&mut self) -> SpeedActionEventBuilder<'_> {
        SpeedActionEventBuilder::new(self)
    }
    
    /// Add a teleport action event
    pub fn add_teleport_action(&mut self) -> TeleportActionEventBuilder<'_> {
        TeleportActionEventBuilder::new(self)
    }
    
    /// Finish this maneuver
    pub fn finish(self) -> &'parent mut super::story::ActBuilder<'parent> {
        let maneuver = Maneuver {
            name: OSString::literal(self.maneuver_name),
            events: self.events,
            parameter_declarations: None,
        };
        
        self.parent.add_maneuver_to_group(maneuver);
        self.parent
    }
}

/// Builder for speed action events within maneuvers
pub struct SpeedActionEventBuilder<'parent> {
    parent: &'parent mut ManeuverBuilder<'parent>,
    action_builder: SpeedActionBuilder,
    event_name: Option<String>,
    start_trigger: Option<Trigger>,
}

impl<'parent> SpeedActionEventBuilder<'parent> {
    pub fn new(parent: &'parent mut ManeuverBuilder<'parent>) -> Self {
        Self {
            action_builder: SpeedActionBuilder::new().for_entity(&parent.entity_ref),
            parent,
            event_name: None,
            start_trigger: None,
        }
    }
    
    /// Set event name
    pub fn named(mut self, name: &str) -> Self {
        self.event_name = Some(name.to_string());
        self
    }
    
    /// Set target speed
    pub fn to_speed(mut self, speed: f64) -> Self {
        self.action_builder = self.action_builder.to_speed(speed);
        self
    }
    
    /// Set custom start trigger for this event
    pub fn with_trigger(mut self, trigger: Trigger) -> Self {
        self.start_trigger = Some(trigger);
        self
    }
    
    /// Start building a custom trigger
    pub fn triggered_by(self) -> EventTriggerBuilder<SpeedActionEventBuilder<'parent>> {
        EventTriggerBuilder::new(self)
    }
    
    /// Finish the speed action and add to maneuver
    pub fn finish(self) -> BuilderResult<&'parent mut ManeuverBuilder<'parent>> {
        let private_action = self.action_builder.build_action()?;
        
        // Convert PrivateAction to StoryPrivateAction
        let story_private_action = match private_action.action {
            crate::types::actions::wrappers::CorePrivateAction::LongitudinalAction(long_action) => {
                // Convert movement::LongitudinalAction to init::LongitudinalAction
                let init_long_action = crate::types::scenario::init::LongitudinalAction {
                    speed_action: match &long_action.longitudinal_action_choice {
                        crate::types::actions::movement::LongitudinalActionChoice::SpeedAction(speed_action) => Some(speed_action.clone()),
                        _ => None,
                    },
                    longitudinal_distance_action: match &long_action.longitudinal_action_choice {
                        crate::types::actions::movement::LongitudinalActionChoice::LongitudinalDistanceAction(dist_action) => Some(dist_action.clone()),
                        _ => None,
                    },
                    speed_profile_action: match &long_action.longitudinal_action_choice {
                        crate::types::actions::movement::LongitudinalActionChoice::SpeedProfileAction(profile_action) => Some(profile_action.clone()),
                        _ => None,
                    },
                };
                StoryPrivateAction {
                    longitudinal_action: Some(init_long_action),
                    lateral_action: None,
                    visibility_action: None,
                    synchronize_action: None,
                    controller_action: None,
                    teleport_action: None,
                    routing_action: None,
                    appearance_action: None,
                    trailer_action: None,
                }
            },
            crate::types::actions::wrappers::CorePrivateAction::TeleportAction(teleport_action) => {
                StoryPrivateAction {
                    longitudinal_action: None,
                    lateral_action: None,
                    visibility_action: None,
                    synchronize_action: None,
                    controller_action: None,
                    teleport_action: Some(teleport_action),
                    routing_action: None,
                    appearance_action: None,
                    trailer_action: None,
                }
            },
            _ => {
                return Err(BuilderError::validation_error("Unsupported action type"));
            }
        };
        
        let event = Event {
            name: OSString::literal(self.event_name.unwrap_or_else(|| "SpeedEvent".to_string())),
            maximum_execution_count: None,
            priority: Some(Priority::Override),
            start_trigger: self.start_trigger.or_else(|| Some(Trigger {
                condition_groups: vec![ConditionGroup { conditions: Vec::new() }],
            })),
            actions: vec![StoryAction {
                name: OSString::literal("SpeedAction".to_string()),
                private_action: Some(story_private_action),
            }],
        };
        
        self.parent.events.push(event);
        Ok(self.parent)
    }
}

/// Similar builder for teleport actions
pub struct TeleportActionEventBuilder<'parent> {
    parent: &'parent mut ManeuverBuilder<'parent>,
    action_builder: TeleportActionBuilder,
    event_name: Option<String>,
    start_trigger: Option<Trigger>,
}

impl<'parent> TeleportActionEventBuilder<'parent> {
    pub fn new(parent: &'parent mut ManeuverBuilder<'parent>) -> Self {
        Self {
            action_builder: TeleportActionBuilder::new().for_entity(&parent.entity_ref),
            parent,
            event_name: None,
            start_trigger: None,
        }
    }
    
    /// Set event name
    pub fn named(mut self, name: &str) -> Self {
        self.event_name = Some(name.to_string());
        self
    }
    
    /// Set custom start trigger for this event
    pub fn with_trigger(mut self, trigger: Trigger) -> Self {
        self.start_trigger = Some(trigger);
        self
    }
    
    /// Start position configuration
    pub fn to(self) -> TeleportPositionEventBuilder<'parent> {
        TeleportPositionEventBuilder::new(self)
    }
}

/// Position builder for teleport events
pub struct TeleportPositionEventBuilder<'parent> {
    parent: TeleportActionEventBuilder<'parent>,
}

impl<'parent> TeleportPositionEventBuilder<'parent> {
    pub fn new(parent: TeleportActionEventBuilder<'parent>) -> Self {
        Self { parent }
    }
    
    /// Set world position and finish
    pub fn world_position(mut self, x: f64, y: f64, z: f64) -> BuilderResult<&'parent mut ManeuverBuilder<'parent>> {
        self.parent.action_builder = self.parent.action_builder.to().world_position(x, y, z);
        
        let private_action = self.parent.action_builder.build_action()?;
        
        // Convert PrivateAction to StoryPrivateAction
        let story_private_action = match private_action.action {
            crate::types::actions::wrappers::CorePrivateAction::LongitudinalAction(long_action) => {
                // Convert movement::LongitudinalAction to init::LongitudinalAction
                let init_long_action = crate::types::scenario::init::LongitudinalAction {
                    speed_action: match &long_action.longitudinal_action_choice {
                        crate::types::actions::movement::LongitudinalActionChoice::SpeedAction(speed_action) => Some(speed_action.clone()),
                        _ => None,
                    },
                    longitudinal_distance_action: match &long_action.longitudinal_action_choice {
                        crate::types::actions::movement::LongitudinalActionChoice::LongitudinalDistanceAction(dist_action) => Some(dist_action.clone()),
                        _ => None,
                    },
                    speed_profile_action: match &long_action.longitudinal_action_choice {
                        crate::types::actions::movement::LongitudinalActionChoice::SpeedProfileAction(profile_action) => Some(profile_action.clone()),
                        _ => None,
                    },
                };
                StoryPrivateAction {
                    longitudinal_action: Some(init_long_action),
                    lateral_action: None,
                    visibility_action: None,
                    synchronize_action: None,
                    controller_action: None,
                    teleport_action: None,
                    routing_action: None,
                    appearance_action: None,
                    trailer_action: None,
                }
            },
            crate::types::actions::wrappers::CorePrivateAction::TeleportAction(teleport_action) => {
                StoryPrivateAction {
                    longitudinal_action: None,
                    lateral_action: None,
                    visibility_action: None,
                    synchronize_action: None,
                    controller_action: None,
                    teleport_action: Some(teleport_action),
                    routing_action: None,
                    appearance_action: None,
                    trailer_action: None,
                }
            },
            _ => {
                return Err(BuilderError::validation_error("Unsupported action type"));
            }
        };
        
        let event = Event {
            name: OSString::literal(self.parent.event_name.unwrap_or_else(|| "TeleportEvent".to_string())),
            maximum_execution_count: None,
            priority: Some(Priority::Override),
            start_trigger: self.parent.start_trigger.or_else(|| Some(Trigger {
                condition_groups: vec![ConditionGroup { conditions: Vec::new() }],
            })),
            actions: vec![StoryAction {
                name: OSString::literal("TeleportAction".to_string()),
                private_action: Some(story_private_action),
            }],
        };
        
        self.parent.parent.events.push(event);
        Ok(self.parent.parent)
    }
}

/// Builder for event triggers
pub struct EventTriggerBuilder<P> {
    parent: P,
    trigger_builder: crate::builder::conditions::TriggerBuilder,
}

impl<P> EventTriggerBuilder<P> {
    pub fn new(parent: P) -> Self {
        Self {
            parent,
            trigger_builder: crate::builder::conditions::TriggerBuilder::new(),
        }
    }
    
    /// Add time condition
    pub fn time_condition(mut self, time: f64) -> Self {
        let condition = crate::builder::conditions::TimeConditionBuilder::new()
            .at_time(time)
            .build()
            .unwrap(); // TODO: Better error handling
        self.trigger_builder = self.trigger_builder.add_condition(condition);
        self
    }
    
    /// Add speed condition
    pub fn speed_condition(mut self, entity_ref: &str, speed: f64) -> Self {
        let condition = crate::builder::conditions::SpeedConditionBuilder::new()
            .for_entity(entity_ref)
            .speed_above(speed)
            .build()
            .unwrap(); // TODO: Better error handling
        self.trigger_builder = self.trigger_builder.add_condition(condition);
        self
    }
}

impl<'a> EventTriggerBuilder<SpeedActionEventBuilder<'a>> {
    /// Finish trigger and return to speed action event builder
    pub fn finish(self) -> SpeedActionEventBuilder<'a> {
        let trigger = self.trigger_builder.build().unwrap(); // TODO: Better error handling
        self.parent.with_trigger(trigger)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::scenario::ScenarioBuilder;
    use crate::builder::storyboard::StoryboardBuilder;

    #[test]
    fn test_maneuver_builder_creation() {
        let scenario_builder = ScenarioBuilder::new()
            .with_header("Test", "Author")
            .with_entities();
            
        let mut storyboard_builder = StoryboardBuilder::new(scenario_builder);
        let mut story_builder = storyboard_builder.add_story("TestStory");
        let mut act_builder = story_builder.add_act("TestAct");
        
        let maneuver_builder = ManeuverBuilder::new(&mut act_builder, "TestManeuver", "ego");
        
        assert_eq!(maneuver_builder.maneuver_name, "TestManeuver");
        assert_eq!(maneuver_builder.entity_ref, "ego");
        assert_eq!(maneuver_builder.events.len(), 0);
    }
}