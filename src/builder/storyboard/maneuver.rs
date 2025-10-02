//! Maneuver builders for entity behavior sequences

use crate::builder::{
    actions::{ActionBuilder, SpeedActionBuilder, TeleportActionBuilder},
    BuilderError, BuilderResult,
};
use crate::types::{
    basic::OSString,
    enums::Priority,
    scenario::{
        story::{Event, Maneuver, StoryAction, StoryPrivateAction},
        triggers::{ConditionGroup, Trigger},
    },
};

/// Builder for maneuvers (entity behavior sequences)
pub struct ManeuverBuilder<'parent> {
    parent: &'parent mut super::story::ActBuilder<'parent>,
    maneuver_name: String,
    entity_ref: String,
    events: Vec<Event>,
}

impl<'parent> ManeuverBuilder<'parent> {
    pub fn new(
        parent: &'parent mut super::story::ActBuilder<'parent>,
        name: &str,
        entity_ref: &str,
    ) -> Self {
        Self {
            parent,
            maneuver_name: name.to_string(),
            entity_ref: entity_ref.to_string(),
            events: Vec::new(),
        }
    }

    /// Add a speed action event
    ///
    /// # Usage Note
    /// Due to Rust lifetime variance constraints, fluent chaining may be limited.
    /// Use the returned SpeedActionEventBuilder and call `.finish()` to complete the action.
    /// For unlimited fluent chaining, use `create_speed_action()` instead.
    ///
    /// # Example
    /// ```rust,ignore
    /// // Working pattern:
    /// let speed_builder = maneuver_builder.add_speed_action();
    /// speed_builder.to_speed(30.0).finish();
    ///
    /// // Or use the direct pattern:
    /// maneuver_builder.add_speed_action().to_speed(30.0).finish();
    /// ```
    pub fn add_speed_action<'a>(&'a mut self) -> SpeedActionEventBuilder<'a>
    where
        'a: 'parent,
    {
        SpeedActionEventBuilder::new(self)
    }

    /// Create a detached speed action builder (no lifetime constraints)
    pub fn create_speed_action(&self) -> DetachedSpeedActionBuilder {
        DetachedSpeedActionBuilder::new(&self.entity_ref)
    }

    /// Add a teleport action event
    ///
    /// # Usage Note  
    /// Due to Rust lifetime variance constraints, fluent chaining may be limited.
    /// Use the returned TeleportActionEventBuilder and call `.finish()` to complete the action.
    /// For unlimited fluent chaining, use `create_teleport_action()` instead.
    ///
    /// # Example
    /// ```rust,ignore
    /// // Working pattern:
    /// let teleport_builder = maneuver_builder.add_teleport_action();
    /// teleport_builder.finish();
    ///
    /// // Or use the direct pattern:
    /// maneuver_builder.add_teleport_action().finish();
    /// ```
    pub fn add_teleport_action<'a>(&'a mut self) -> TeleportActionEventBuilder<'a>
    where
        'a: 'parent,
    {
        TeleportActionEventBuilder::new(self)
    }

    /// Create a detached teleport action builder (no lifetime constraints)
    pub fn create_teleport_action(&self) -> DetachedTeleportActionBuilder {
        DetachedTeleportActionBuilder::new(&self.entity_ref)
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
            }
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
            }
            _ => {
                return Err(BuilderError::validation_error("Unsupported action type"));
            }
        };

        let event = Event {
            name: OSString::literal(self.event_name.unwrap_or_else(|| "SpeedEvent".to_string())),
            maximum_execution_count: None,
            priority: Some(Priority::Override),
            start_trigger: self.start_trigger.or_else(|| {
                // Provide default immediate trigger instead of empty trigger
                crate::builder::conditions::TriggerBuilder::new()
                    .add_condition(
                        crate::builder::conditions::TimeConditionBuilder::new()
                            .at_time(0.0)
                            .build()
                            .unwrap(),
                    )
                    .build()
                    .ok()
            }),
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
    pub fn world_position(
        mut self,
        x: f64,
        y: f64,
        z: f64,
    ) -> BuilderResult<&'parent mut ManeuverBuilder<'parent>> {
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
            }
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
            }
            _ => {
                return Err(BuilderError::validation_error("Unsupported action type"));
            }
        };

        let event = Event {
            name: OSString::literal(
                self.parent
                    .event_name
                    .unwrap_or_else(|| "TeleportEvent".to_string()),
            ),
            maximum_execution_count: None,
            priority: Some(Priority::Override),
            start_trigger: self.parent.start_trigger.or_else(|| {
                Some(Trigger {
                    condition_groups: vec![ConditionGroup {
                        conditions: Vec::new(),
                    }],
                })
            }),
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
        let condition = crate::builder::conditions::ValueSpeedConditionBuilder::new()
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

/// Detached builder for maneuvers (no lifetime constraints)
pub struct DetachedManeuverBuilder {
    maneuver_name: String,
    entity_ref: String,
    events: Vec<Event>,
}

impl DetachedManeuverBuilder {
    /// Create a new detached maneuver builder
    pub fn new(name: &str, entity_ref: &str) -> Self {
        Self {
            maneuver_name: name.to_string(),
            entity_ref: entity_ref.to_string(),
            events: Vec::new(),
        }
    }

    /// Add a speed action using closure-based configuration
    pub fn add_speed_action<F>(mut self, config: F) -> BuilderResult<Self>
    where
        F: FnOnce(DetachedSpeedActionBuilder) -> DetachedSpeedActionBuilder,
    {
        let speed_builder = DetachedSpeedActionBuilder::new(&self.entity_ref);
        let configured_builder = config(speed_builder);
        configured_builder.attach_to_detached(&mut self)?;
        Ok(self)
    }

    /// Create a detached speed action builder
    pub fn create_speed_action(&self) -> DetachedSpeedActionBuilder {
        DetachedSpeedActionBuilder::new(&self.entity_ref)
    }

    /// Create a detached teleport action builder
    pub fn create_teleport_action(&self) -> DetachedTeleportActionBuilder {
        DetachedTeleportActionBuilder::new(&self.entity_ref)
    }

    /// Add a completed event to this maneuver
    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }

    /// Attach this maneuver to an act builder
    pub fn attach_to(self, act: &mut super::story::ActBuilder<'_>) {
        let maneuver = Maneuver {
            name: OSString::literal(self.maneuver_name),
            events: self.events,
            parameter_declarations: None,
        };
        act.add_maneuver_to_group(maneuver);
    }

    /// Attach this maneuver to a detached act builder
    pub fn attach_to_detached(self, act: &mut super::story::DetachedActBuilder) {
        let maneuver = Maneuver {
            name: OSString::literal(self.maneuver_name),
            events: self.events,
            parameter_declarations: None,
        };
        act.add_completed_maneuver(maneuver);
    }

    /// Build the final Maneuver object
    pub fn build(self) -> Maneuver {
        Maneuver {
            name: OSString::literal(self.maneuver_name),
            events: self.events,
            parameter_declarations: None,
        }
    }
}

/// Detached builder for speed action events (no lifetime constraints)
pub struct DetachedSpeedActionBuilder {
    entity_ref: String,
    action_builder: SpeedActionBuilder,
    event_name: Option<String>,
    start_trigger: Option<Trigger>,
}

impl DetachedSpeedActionBuilder {
    /// Create a new detached speed action builder
    pub fn new(entity_ref: &str) -> Self {
        Self {
            action_builder: SpeedActionBuilder::new().for_entity(entity_ref),
            entity_ref: entity_ref.to_string(),
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

    /// Add a time-based trigger (convenience method)
    pub fn with_time_trigger(mut self, time: f64) -> BuilderResult<Self> {
        let trigger = crate::builder::conditions::TriggerBuilder::new()
            .add_condition(
                crate::builder::conditions::TimeConditionBuilder::new()
                    .at_time(time)
                    .build()?,
            )
            .build()?;
        self.start_trigger = Some(trigger);
        Ok(self)
    }

    /// Start immediately (time = 0.0)
    pub fn start_immediately(self) -> BuilderResult<Self> {
        self.with_time_trigger(0.0)
    }

    /// Attach this speed action to a maneuver builder
    pub fn attach_to(self, maneuver: &mut ManeuverBuilder<'_>) -> BuilderResult<()> {
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
            }
            _ => {
                return Err(BuilderError::validation_error("Unsupported action type"));
            }
        };

        let event = Event {
            name: OSString::literal(self.event_name.unwrap_or_else(|| "SpeedEvent".to_string())),
            maximum_execution_count: None,
            priority: Some(Priority::Override),
            start_trigger: self.start_trigger.or_else(|| {
                // Provide default immediate trigger instead of empty trigger
                crate::builder::conditions::TriggerBuilder::new()
                    .add_condition(
                        crate::builder::conditions::TimeConditionBuilder::new()
                            .at_time(0.0)
                            .build()
                            .unwrap(),
                    )
                    .build()
                    .ok()
            }),
            actions: vec![StoryAction {
                name: OSString::literal("SpeedAction".to_string()),
                private_action: Some(story_private_action),
            }],
        };

        maneuver.events.push(event);
        Ok(())
    }

    /// Attach this speed action to a detached maneuver builder
    pub fn attach_to_detached(self, maneuver: &mut DetachedManeuverBuilder) -> BuilderResult<()> {
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
            }
            _ => {
                return Err(BuilderError::validation_error("Unsupported action type"));
            }
        };

        let event = Event {
            name: OSString::literal(self.event_name.unwrap_or_else(|| "SpeedEvent".to_string())),
            maximum_execution_count: None,
            priority: Some(Priority::Override),
            start_trigger: self.start_trigger.or_else(|| {
                // Provide default immediate trigger instead of empty trigger
                crate::builder::conditions::TriggerBuilder::new()
                    .add_condition(
                        crate::builder::conditions::TimeConditionBuilder::new()
                            .at_time(0.0)
                            .build()
                            .unwrap(),
                    )
                    .build()
                    .ok()
            }),
            actions: vec![StoryAction {
                name: OSString::literal("SpeedAction".to_string()),
                private_action: Some(story_private_action),
            }],
        };

        maneuver.add_event(event);
        Ok(())
    }

    /// Build the final Event object
    pub fn build(self) -> BuilderResult<Event> {
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
            }
            _ => {
                return Err(BuilderError::validation_error("Unsupported action type"));
            }
        };

        Ok(Event {
            name: OSString::literal(self.event_name.unwrap_or_else(|| "SpeedEvent".to_string())),
            maximum_execution_count: None,
            priority: Some(Priority::Override),
            start_trigger: self.start_trigger.or_else(|| {
                // Provide default immediate trigger instead of empty trigger
                crate::builder::conditions::TriggerBuilder::new()
                    .add_condition(
                        crate::builder::conditions::TimeConditionBuilder::new()
                            .at_time(0.0)
                            .build()
                            .unwrap(),
                    )
                    .build()
                    .ok()
            }),
            actions: vec![StoryAction {
                name: OSString::literal("SpeedAction".to_string()),
                private_action: Some(story_private_action),
            }],
        })
    }
}

/// Detached builder for teleport action events (no lifetime constraints)
pub struct DetachedTeleportActionBuilder {
    entity_ref: String,
    action_builder: TeleportActionBuilder,
    event_name: Option<String>,
    start_trigger: Option<Trigger>,
}

impl DetachedTeleportActionBuilder {
    /// Create a new detached teleport action builder
    pub fn new(entity_ref: &str) -> Self {
        Self {
            action_builder: TeleportActionBuilder::new().for_entity(entity_ref),
            entity_ref: entity_ref.to_string(),
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
    pub fn to(self) -> DetachedTeleportPositionBuilder {
        DetachedTeleportPositionBuilder::new(self)
    }

    /// Attach this teleport action to a maneuver builder
    pub fn attach_to(self, maneuver: &mut ManeuverBuilder<'_>) -> BuilderResult<()> {
        let private_action = self.action_builder.build_action()?;

        // Convert PrivateAction to StoryPrivateAction
        let story_private_action = match private_action.action {
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
            }
            _ => {
                return Err(BuilderError::validation_error("Unsupported action type"));
            }
        };

        let event = Event {
            name: OSString::literal(
                self.event_name
                    .unwrap_or_else(|| "TeleportEvent".to_string()),
            ),
            maximum_execution_count: None,
            priority: Some(Priority::Override),
            start_trigger: self.start_trigger.or_else(|| {
                // Provide default immediate trigger instead of empty trigger
                crate::builder::conditions::TriggerBuilder::new()
                    .add_condition(
                        crate::builder::conditions::TimeConditionBuilder::new()
                            .at_time(0.0)
                            .build()
                            .unwrap(),
                    )
                    .build()
                    .ok()
            }),
            actions: vec![StoryAction {
                name: OSString::literal("TeleportAction".to_string()),
                private_action: Some(story_private_action),
            }],
        };

        maneuver.events.push(event);
        Ok(())
    }

    /// Attach this teleport action to a detached maneuver builder
    pub fn attach_to_detached(self, maneuver: &mut DetachedManeuverBuilder) -> BuilderResult<()> {
        let private_action = self.action_builder.build_action()?;

        // Convert PrivateAction to StoryPrivateAction
        let story_private_action = match private_action.action {
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
            }
            _ => {
                return Err(BuilderError::validation_error("Unsupported action type"));
            }
        };

        let event = Event {
            name: OSString::literal(
                self.event_name
                    .unwrap_or_else(|| "TeleportEvent".to_string()),
            ),
            maximum_execution_count: None,
            priority: Some(Priority::Override),
            start_trigger: self.start_trigger.or_else(|| {
                // Provide default immediate trigger instead of empty trigger
                crate::builder::conditions::TriggerBuilder::new()
                    .add_condition(
                        crate::builder::conditions::TimeConditionBuilder::new()
                            .at_time(0.0)
                            .build()
                            .unwrap(),
                    )
                    .build()
                    .ok()
            }),
            actions: vec![StoryAction {
                name: OSString::literal("TeleportAction".to_string()),
                private_action: Some(story_private_action),
            }],
        };

        maneuver.add_event(event);
        Ok(())
    }

    /// Build the final Event object
    pub fn build(self) -> BuilderResult<Event> {
        let private_action = self.action_builder.build_action()?;

        // Convert PrivateAction to StoryPrivateAction
        let story_private_action = match private_action.action {
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
            }
            _ => {
                return Err(BuilderError::validation_error("Unsupported action type"));
            }
        };

        Ok(Event {
            name: OSString::literal(
                self.event_name
                    .unwrap_or_else(|| "TeleportEvent".to_string()),
            ),
            maximum_execution_count: None,
            priority: Some(Priority::Override),
            start_trigger: self.start_trigger.or_else(|| {
                // Provide default immediate trigger instead of empty trigger
                crate::builder::conditions::TriggerBuilder::new()
                    .add_condition(
                        crate::builder::conditions::TimeConditionBuilder::new()
                            .at_time(0.0)
                            .build()
                            .unwrap(),
                    )
                    .build()
                    .ok()
            }),
            actions: vec![StoryAction {
                name: OSString::literal("TeleportAction".to_string()),
                private_action: Some(story_private_action),
            }],
        })
    }
}

/// Position builder for detached teleport events
pub struct DetachedTeleportPositionBuilder {
    parent: DetachedTeleportActionBuilder,
}

impl DetachedTeleportPositionBuilder {
    /// Create a new detached teleport position builder
    pub fn new(parent: DetachedTeleportActionBuilder) -> Self {
        Self { parent }
    }

    /// Set world position and finish
    pub fn world_position(mut self, x: f64, y: f64, z: f64) -> DetachedTeleportActionBuilder {
        self.parent.action_builder = self.parent.action_builder.to().world_position(x, y, z);
        self.parent
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
        let mut story_builder = storyboard_builder.add_story_simple("TestStory");
        let mut act_builder = story_builder.add_act("TestAct");

        let maneuver_builder = ManeuverBuilder::new(&mut act_builder, "TestManeuver", "ego");

        assert_eq!(maneuver_builder.maneuver_name, "TestManeuver");
        assert_eq!(maneuver_builder.entity_ref, "ego");
        assert_eq!(maneuver_builder.events.len(), 0);
    }
}
