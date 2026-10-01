//! Maneuver builders for entity behavior sequences

use crate::builder::{
    actions::{
        ActionBuilder, FollowTrajectoryActionBuilder, SpeedActionBuilder, TeleportActionBuilder,
    },
    BuilderError, BuilderResult,
};
use crate::types::basic::Value;
use crate::types::{
    basic::{MinVec, OSString},
    enums::Priority,
    positions::Position,
    scenario::{
        story::{
            Event, Maneuver, StoryAction, StoryActionChoice, StoryPrivateAction,
            StoryPrivateActionChoice,
        },
        triggers::Trigger,
    },
};

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

    /// Create a detached follow trajectory action builder
    pub fn create_follow_trajectory_action(&self) -> DetachedFollowTrajectoryActionBuilder {
        DetachedFollowTrajectoryActionBuilder::new(&self.entity_ref)
    }

    /// Add a completed event to this maneuver
    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }

    /// Attach this maneuver to a detached act builder
    pub fn attach_to_detached(
        self,
        act: &mut super::story::DetachedActBuilder,
    ) -> BuilderResult<()> {
        let maneuver = Maneuver {
            name: OSString::literal(self.maneuver_name),
            events: MinVec::new(self.events)?,
            parameter_declarations: None,
        };
        act.add_completed_maneuver(maneuver, &self.entity_ref);
        Ok(())
    }

    /// Build the final Maneuver object
    ///
    /// XSD `Maneuver` (`Schema/OpenSCENARIO.xsd:1453`) requires at least one `Event`.
    pub fn build(self) -> BuilderResult<Maneuver> {
        Ok(Maneuver {
            name: OSString::literal(self.maneuver_name),
            events: MinVec::new(self.events)?,
            parameter_declarations: None,
        })
    }
}

/// Detached builder for speed action events (no lifetime constraints)
pub struct DetachedSpeedActionBuilder {
    action_builder: SpeedActionBuilder,
    event_name: Option<String>,
    start_trigger: Option<Trigger>,
}

impl DetachedSpeedActionBuilder {
    /// Create a new detached speed action builder
    pub fn new(entity_ref: &str) -> Self {
        Self {
            action_builder: SpeedActionBuilder::new().for_entity(entity_ref),
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

    /// Attach this speed action to a detached maneuver builder
    pub fn attach_to_detached(self, maneuver: &mut DetachedManeuverBuilder) -> BuilderResult<()> {
        let private_action = self.action_builder.build_action()?;

        // Convert PrivateAction to StoryPrivateAction
        let story_private_action = match private_action {
            crate::types::actions::wrappers::PrivateAction::LongitudinalAction(long_action) => {
                // Convert movement::LongitudinalAction to init::LongitudinalAction
                let init_long_action =
                    crate::types::scenario::init::LongitudinalAction::from(long_action);
                StoryPrivateAction {
                    action: StoryPrivateActionChoice::LongitudinalAction(init_long_action),
                }
            }
            _ => {
                return Err(BuilderError::validation_error("Unsupported action type"));
            }
        };

        let event = Event {
            name: OSString::literal(self.event_name.unwrap_or_else(|| "SpeedEvent".to_string())),
            maximum_execution_count: None,
            priority: Value::Literal(Priority::Override),
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
            actions: MinVec::new(vec![StoryAction {
                name: OSString::literal("SpeedAction".to_string()),
                action: StoryActionChoice::PrivateAction(story_private_action),
            }])?,
        };

        maneuver.add_event(event);
        Ok(())
    }

    /// Build the final Event object
    pub fn build(self) -> BuilderResult<Event> {
        let private_action = self.action_builder.build_action()?;

        // Convert PrivateAction to StoryPrivateAction
        let story_private_action = match private_action {
            crate::types::actions::wrappers::PrivateAction::LongitudinalAction(long_action) => {
                // Convert movement::LongitudinalAction to init::LongitudinalAction
                let init_long_action =
                    crate::types::scenario::init::LongitudinalAction::from(long_action);
                StoryPrivateAction {
                    action: StoryPrivateActionChoice::LongitudinalAction(init_long_action),
                }
            }
            _ => {
                return Err(BuilderError::validation_error("Unsupported action type"));
            }
        };

        Ok(Event {
            name: OSString::literal(self.event_name.unwrap_or_else(|| "SpeedEvent".to_string())),
            maximum_execution_count: None,
            priority: Value::Literal(Priority::Override),
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
            actions: MinVec::new(vec![StoryAction {
                name: OSString::literal("SpeedAction".to_string()),
                action: StoryActionChoice::PrivateAction(story_private_action),
            }])?,
        })
    }
}

/// Detached builder for teleport action events (no lifetime constraints)
pub struct DetachedTeleportActionBuilder {
    action_builder: TeleportActionBuilder,
    event_name: Option<String>,
    start_trigger: Option<Trigger>,
}

impl DetachedTeleportActionBuilder {
    /// Create a new detached teleport action builder
    pub fn new(entity_ref: &str) -> Self {
        Self {
            action_builder: TeleportActionBuilder::new().for_entity(entity_ref),
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

    /// Attach this teleport action to a detached maneuver builder
    pub fn attach_to_detached(self, maneuver: &mut DetachedManeuverBuilder) -> BuilderResult<()> {
        let private_action = self.action_builder.build_action()?;

        // Convert PrivateAction to StoryPrivateAction
        let story_private_action = match private_action {
            crate::types::actions::wrappers::PrivateAction::TeleportAction(teleport_action) => {
                StoryPrivateAction {
                    action: StoryPrivateActionChoice::TeleportAction(teleport_action),
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
            priority: Value::Literal(Priority::Override),
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
            actions: MinVec::new(vec![StoryAction {
                name: OSString::literal("TeleportAction".to_string()),
                action: StoryActionChoice::PrivateAction(story_private_action),
            }])?,
        };

        maneuver.add_event(event);
        Ok(())
    }

    /// Build the final Event object
    pub fn build(self) -> BuilderResult<Event> {
        let private_action = self.action_builder.build_action()?;

        // Convert PrivateAction to StoryPrivateAction
        let story_private_action = match private_action {
            crate::types::actions::wrappers::PrivateAction::TeleportAction(teleport_action) => {
                StoryPrivateAction {
                    action: StoryPrivateActionChoice::TeleportAction(teleport_action),
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
            priority: Value::Literal(Priority::Override),
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
            actions: MinVec::new(vec![StoryAction {
                name: OSString::literal("TeleportAction".to_string()),
                action: StoryActionChoice::PrivateAction(story_private_action),
            }])?,
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

/// Detached builder for follow trajectory action events (no lifetime constraints)
pub struct DetachedFollowTrajectoryActionBuilder {
    action_builder: FollowTrajectoryActionBuilder,
    event_name: Option<String>,
    start_trigger: Option<Trigger>,
}

impl DetachedFollowTrajectoryActionBuilder {
    /// Create a new detached follow trajectory action builder
    pub fn new(entity_ref: &str) -> Self {
        Self {
            action_builder: FollowTrajectoryActionBuilder::new().for_entity(entity_ref),
            event_name: None,
            start_trigger: None,
        }
    }

    /// Set event name
    pub fn named(mut self, name: &str) -> Self {
        self.event_name = Some(name.to_string());
        self
    }

    /// Set the trajectory to follow
    pub fn with_trajectory(
        mut self,
        trajectory: crate::types::actions::movement::Trajectory,
    ) -> Self {
        self.action_builder = self.action_builder.with_trajectory(trajectory);
        self
    }

    /// Set following mode to "follow"
    pub fn following_mode_follow(mut self) -> Self {
        self.action_builder = self.action_builder.following_mode_follow();
        self
    }

    /// Set following mode to "position"
    pub fn following_mode_position(mut self) -> Self {
        self.action_builder = self.action_builder.following_mode_position();
        self
    }

    /// Set initial distance offset
    pub fn initial_distance_offset(mut self, offset: f64) -> Self {
        self.action_builder = self.action_builder.initial_distance_offset(offset);
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

    /// Attach this follow trajectory action to a detached maneuver builder
    pub fn attach_to_detached(self, maneuver: &mut DetachedManeuverBuilder) -> BuilderResult<()> {
        let private_action = self.action_builder.build_action()?;

        // Convert PrivateAction to StoryPrivateAction
        let story_private_action = match private_action {
            crate::types::actions::wrappers::PrivateAction::RoutingAction(routing_action) => {
                StoryPrivateAction {
                    action: StoryPrivateActionChoice::RoutingAction(routing_action),
                }
            }
            _ => {
                return Err(BuilderError::validation_error(
                    "Expected RoutingAction for trajectory following",
                ));
            }
        };

        let event = Event {
            name: OSString::literal(
                self.event_name
                    .unwrap_or_else(|| "FollowTrajectoryEvent".to_string()),
            ),
            maximum_execution_count: None,
            priority: Value::Literal(Priority::Override),
            start_trigger: self.start_trigger.or_else(|| {
                // Provide default immediate trigger
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
            actions: MinVec::new(vec![StoryAction {
                name: OSString::literal("FollowTrajectoryAction".to_string()),
                action: StoryActionChoice::PrivateAction(story_private_action),
            }])?,
        };

        maneuver.events.push(event);
        Ok(())
    }

    /// Build the final Event object
    pub fn build(self) -> BuilderResult<Event> {
        let private_action = self.action_builder.build_action()?;

        // Convert PrivateAction to StoryPrivateAction
        let story_private_action = match private_action {
            crate::types::actions::wrappers::PrivateAction::RoutingAction(routing_action) => {
                StoryPrivateAction {
                    action: StoryPrivateActionChoice::RoutingAction(routing_action),
                }
            }
            _ => {
                return Err(BuilderError::validation_error(
                    "Expected RoutingAction for trajectory following",
                ));
            }
        };

        Ok(Event {
            name: OSString::literal(
                self.event_name
                    .unwrap_or_else(|| "FollowTrajectoryEvent".to_string()),
            ),
            maximum_execution_count: None,
            priority: Value::Literal(Priority::Override),
            start_trigger: self.start_trigger.or_else(|| {
                // Provide default immediate trigger
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
            actions: MinVec::new(vec![StoryAction {
                name: OSString::literal("FollowTrajectoryAction".to_string()),
                action: StoryActionChoice::PrivateAction(story_private_action),
            }])?,
        })
    }
}

// ==================== NEW DETACHED BUILDERS FOR ADDITIONAL ACTIONS ====================

/// Detached builder for longitudinal distance action
pub struct DetachedLongitudinalDistanceActionBuilder {
    action_builder: crate::builder::actions::LongitudinalDistanceActionBuilder,
    event_name: Option<String>,
    start_trigger: Option<Trigger>,
}

impl DetachedLongitudinalDistanceActionBuilder {
    pub fn new(entity_ref: &str) -> Self {
        Self {
            action_builder: crate::builder::actions::LongitudinalDistanceActionBuilder::new()
                .for_entity(entity_ref),
            event_name: None,
            start_trigger: None,
        }
    }

    pub fn named(mut self, name: &str) -> Self {
        self.event_name = Some(name.to_string());
        self
    }

    pub fn from_entity(mut self, target: &str) -> Self {
        self.action_builder = self.action_builder.from_entity(target);
        self
    }

    pub fn at_distance(mut self, distance: f64) -> Self {
        self.action_builder = self.action_builder.at_distance(distance);
        self
    }

    pub fn with_trigger(mut self, trigger: Trigger) -> Self {
        self.start_trigger = Some(trigger);
        self
    }

    pub fn attach_to_detached(self, maneuver: &mut DetachedManeuverBuilder) -> BuilderResult<()> {
        let private_action = self.action_builder.build_action()?;
        let story_private_action = convert_private_action_to_story(private_action);

        maneuver.add_event(Event {
            name: OSString::literal(
                self.event_name
                    .unwrap_or_else(|| "LongitudinalDistanceEvent".to_string()),
            ),
            maximum_execution_count: None,
            priority: Value::Literal(Priority::Override),
            start_trigger: self.start_trigger.or_else(default_trigger),
            actions: MinVec::new(vec![StoryAction {
                name: OSString::literal("LongitudinalDistanceAction".to_string()),
                action: StoryActionChoice::PrivateAction(story_private_action),
            }])?,
        });
        Ok(())
    }
}

/// Detached builder for speed profile action
pub struct DetachedSpeedProfileActionBuilder {
    action_builder: crate::builder::actions::SpeedProfileActionBuilder,
    event_name: Option<String>,
    start_trigger: Option<Trigger>,
}

impl DetachedSpeedProfileActionBuilder {
    pub fn new(entity_ref: &str) -> Self {
        Self {
            action_builder: crate::builder::actions::SpeedProfileActionBuilder::new()
                .for_entity(entity_ref),
            event_name: None,
            start_trigger: None,
        }
    }

    pub fn named(mut self, name: &str) -> Self {
        self.event_name = Some(name.to_string());
        self
    }

    pub fn add_entry_direct(mut self, time: f64, speed: f64) -> Self {
        self.action_builder = self.action_builder.add_entry_direct(time, speed);
        self
    }

    pub fn with_trigger(mut self, trigger: Trigger) -> Self {
        self.start_trigger = Some(trigger);
        self
    }

    pub fn attach_to_detached(self, maneuver: &mut DetachedManeuverBuilder) -> BuilderResult<()> {
        let private_action = self.action_builder.build_action()?;
        let story_private_action = convert_private_action_to_story(private_action);

        maneuver.add_event(Event {
            name: OSString::literal(
                self.event_name
                    .unwrap_or_else(|| "SpeedProfileEvent".to_string()),
            ),
            maximum_execution_count: None,
            priority: Value::Literal(Priority::Override),
            start_trigger: self.start_trigger.or_else(default_trigger),
            actions: MinVec::new(vec![StoryAction {
                name: OSString::literal("SpeedProfileAction".to_string()),
                action: StoryActionChoice::PrivateAction(story_private_action),
            }])?,
        });
        Ok(())
    }
}

/// Detached builder for assign route action
pub struct DetachedAssignRouteActionBuilder {
    action_builder: crate::builder::actions::AssignRouteActionBuilder,
    event_name: Option<String>,
    start_trigger: Option<Trigger>,
}

impl DetachedAssignRouteActionBuilder {
    pub fn new(entity_ref: &str) -> Self {
        Self {
            action_builder: crate::builder::actions::AssignRouteActionBuilder::new()
                .for_entity(entity_ref),
            event_name: None,
            start_trigger: None,
        }
    }

    pub fn named(mut self, name: &str) -> Self {
        self.event_name = Some(name.to_string());
        self
    }

    pub fn with_catalog_route(
        mut self,
        catalog_name: impl Into<String>,
        entry_name: impl Into<String>,
    ) -> Self {
        self.action_builder = self
            .action_builder
            .with_catalog_route(catalog_name, entry_name);
        self
    }

    pub fn with_trigger(mut self, trigger: Trigger) -> Self {
        self.start_trigger = Some(trigger);
        self
    }

    pub fn attach_to_detached(self, maneuver: &mut DetachedManeuverBuilder) -> BuilderResult<()> {
        let private_action = self.action_builder.build_action()?;
        let story_private_action = convert_private_action_to_story(private_action);

        maneuver.add_event(Event {
            name: OSString::literal(
                self.event_name
                    .unwrap_or_else(|| "AssignRouteEvent".to_string()),
            ),
            maximum_execution_count: None,
            priority: Value::Literal(Priority::Override),
            start_trigger: self.start_trigger.or_else(default_trigger),
            actions: MinVec::new(vec![StoryAction {
                name: OSString::literal("AssignRouteAction".to_string()),
                action: StoryActionChoice::PrivateAction(story_private_action),
            }])?,
        });
        Ok(())
    }
}

/// Detached builder for synchronize action
pub struct DetachedSynchronizeActionBuilder {
    action_builder: crate::builder::actions::SynchronizeActionBuilder,
    event_name: Option<String>,
    start_trigger: Option<Trigger>,
}

impl DetachedSynchronizeActionBuilder {
    pub fn new(entity_ref: &str) -> Self {
        Self {
            action_builder: crate::builder::actions::SynchronizeActionBuilder::new()
                .for_entity(entity_ref),
            event_name: None,
            start_trigger: None,
        }
    }

    pub fn named(mut self, name: &str) -> Self {
        self.event_name = Some(name.to_string());
        self
    }

    pub fn with_master(mut self, master: &str) -> Self {
        self.action_builder = self.action_builder.with_master(master);
        self
    }

    pub fn master_position(mut self, position: Position) -> Self {
        self.action_builder = self.action_builder.master_position(position);
        self
    }

    pub fn entity_position(mut self, position: Position) -> Self {
        self.action_builder = self.action_builder.entity_position(position);
        self
    }

    pub fn with_trigger(mut self, trigger: Trigger) -> Self {
        self.start_trigger = Some(trigger);
        self
    }

    pub fn attach_to_detached(self, maneuver: &mut DetachedManeuverBuilder) -> BuilderResult<()> {
        let private_action = self.action_builder.build_action()?;
        let story_private_action = convert_private_action_to_story(private_action);

        maneuver.add_event(Event {
            name: OSString::literal(
                self.event_name
                    .unwrap_or_else(|| "SynchronizeEvent".to_string()),
            ),
            maximum_execution_count: None,
            priority: Value::Literal(Priority::Override),
            start_trigger: self.start_trigger.or_else(default_trigger),
            actions: MinVec::new(vec![StoryAction {
                name: OSString::literal("SynchronizeAction".to_string()),
                action: StoryActionChoice::PrivateAction(story_private_action),
            }])?,
        });
        Ok(())
    }
}

/// Detached builder for visibility action
pub struct DetachedVisibilityActionBuilder {
    action_builder: crate::builder::actions::VisibilityActionBuilder,
    event_name: Option<String>,
    start_trigger: Option<Trigger>,
}

impl DetachedVisibilityActionBuilder {
    pub fn new(entity_ref: &str) -> Self {
        Self {
            action_builder: crate::builder::actions::VisibilityActionBuilder::new()
                .for_entity(entity_ref),
            event_name: None,
            start_trigger: None,
        }
    }

    pub fn named(mut self, name: &str) -> Self {
        self.event_name = Some(name.to_string());
        self
    }

    pub fn visible(mut self) -> Self {
        self.action_builder = self.action_builder.visible();
        self
    }

    pub fn invisible(mut self) -> Self {
        self.action_builder = self.action_builder.invisible();
        self
    }

    pub fn graphics(mut self, visible: bool) -> Self {
        self.action_builder = self.action_builder.graphics(visible);
        self
    }

    pub fn with_trigger(mut self, trigger: Trigger) -> Self {
        self.start_trigger = Some(trigger);
        self
    }

    pub fn attach_to_detached(self, maneuver: &mut DetachedManeuverBuilder) -> BuilderResult<()> {
        let private_action = self.action_builder.build_action()?;
        let story_private_action = convert_private_action_to_story(private_action);

        maneuver.add_event(Event {
            name: OSString::literal(
                self.event_name
                    .unwrap_or_else(|| "VisibilityEvent".to_string()),
            ),
            maximum_execution_count: None,
            priority: Value::Literal(Priority::Override),
            start_trigger: self.start_trigger.or_else(default_trigger),
            actions: MinVec::new(vec![StoryAction {
                name: OSString::literal("VisibilityAction".to_string()),
                action: StoryActionChoice::PrivateAction(story_private_action),
            }])?,
        });
        Ok(())
    }
}

// Helper function for default trigger
fn default_trigger() -> Option<Trigger> {
    crate::builder::conditions::TriggerBuilder::new()
        .add_condition(
            crate::builder::conditions::TimeConditionBuilder::new()
                .at_time(0.0)
                .build()
                .ok()?,
        )
        .build()
        .ok()
}

// Helper function for converting builder PrivateAction to story PrivateAction
fn convert_private_action_to_story(
    action: crate::types::actions::wrappers::PrivateAction,
) -> StoryPrivateAction {
    use crate::types::actions::wrappers::PrivateAction;

    match action {
        PrivateAction::LongitudinalAction(long_action) => {
            let init_long_action =
                crate::types::scenario::init::LongitudinalAction::from(long_action);
            StoryPrivateAction {
                action: StoryPrivateActionChoice::LongitudinalAction(init_long_action),
            }
        }
        PrivateAction::LateralAction(lat_action) => StoryPrivateAction {
            action: StoryPrivateActionChoice::LateralAction(lat_action),
        },
        PrivateAction::VisibilityAction(vis_action) => StoryPrivateAction {
            action: StoryPrivateActionChoice::VisibilityAction(vis_action),
        },
        PrivateAction::SynchronizeAction(sync_action) => StoryPrivateAction {
            action: StoryPrivateActionChoice::SynchronizeAction(sync_action),
        },
        PrivateAction::TeleportAction(teleport_action) => StoryPrivateAction {
            action: StoryPrivateActionChoice::TeleportAction(teleport_action),
        },
        PrivateAction::RoutingAction(routing_action) => StoryPrivateAction {
            action: StoryPrivateActionChoice::RoutingAction(routing_action),
        },
        PrivateAction::ActivateControllerAction(activate_action) => StoryPrivateAction {
            action: StoryPrivateActionChoice::ActivateControllerAction(activate_action),
        },
        PrivateAction::ControllerAction(controller_action) => StoryPrivateAction {
            action: StoryPrivateActionChoice::ControllerAction(controller_action),
        },
        PrivateAction::AppearanceAction(appearance_action) => StoryPrivateAction {
            action: StoryPrivateActionChoice::AppearanceAction(appearance_action),
        },
        PrivateAction::TrailerAction(trailer_action) => StoryPrivateAction {
            action: StoryPrivateActionChoice::TrailerAction(trailer_action),
        },
    }
}
