//! Story and Act builders for scenario behavior definition

use crate::builder::{
    init::InitActionBuilder, scenario::HasEntities, scenario::ScenarioBuilder, BuilderResult,
};
use crate::types::{
    basic::{OSString, ParameterDeclarations, UnsignedInt},
    scenario::{
        init::Init,
        story::{Act, Actors, ManeuverGroup, ScenarioStory},
        storyboard::Storyboard,
        triggers::Trigger,
    },
};

/// Builder for complete storyboard
pub struct StoryboardBuilder {
    scenario_builder: ScenarioBuilder<HasEntities>,
    stories: Vec<ScenarioStory>,
    init: Option<Init>,
    stop_trigger: Option<Trigger>,
}

impl StoryboardBuilder {
    pub fn new(scenario_builder: ScenarioBuilder<HasEntities>) -> Self {
        Self {
            scenario_builder,
            stories: Vec::new(),
            init: None, // No longer use empty default - require explicit init
            stop_trigger: None,
        }
    }

    /// Set initialization actions using InitActionBuilder
    pub fn with_init_actions(mut self, init: Init) -> Self {
        self.init = Some(init);
        self
    }

    /// Create an init action builder for this storyboard
    pub fn create_init_actions(self) -> InitActionBuilderForStoryboard {
        InitActionBuilderForStoryboard::new(self)
    }

    /// Add a story using closure-based configuration
    pub fn add_story<F>(mut self, name: &str, config: F) -> Self
    where
        F: FnOnce(DetachedStoryBuilder) -> DetachedStoryBuilder,
    {
        let story_builder = DetachedStoryBuilder::new(name);
        let configured_builder = config(story_builder);
        let story = configured_builder.build();
        self.stories.push(story);
        self
    }

    /// Add a story to the storyboard (simple method)
    pub fn add_story_simple(&mut self, name: &str) -> StoryBuilder<'_> {
        StoryBuilder::new(self, name)
    }

    /// Add a story to the storyboard (legacy method)
    pub fn add_story_mut(&mut self, name: &str) -> StoryBuilder<'_> {
        StoryBuilder::new(self, name)
    }

    /// Set stop trigger for the entire storyboard
    pub fn with_stop_trigger(mut self, trigger: Trigger) -> Self {
        self.stop_trigger = Some(trigger);
        self
    }

    /// Stop scenario after specified time
    pub fn stop_after_time(self, time: f64) -> BuilderResult<Self> {
        use crate::builder::conditions::{TimeConditionBuilder, TriggerBuilder};

        let time_condition = TimeConditionBuilder::new().at_time(time).build()?;

        let trigger = TriggerBuilder::new()
            .add_condition_group()
            .add_condition(time_condition)
            .finish_group()
            .build()?;

        Ok(self.with_stop_trigger(trigger))
    }

    /// Stop when entity reaches position
    pub fn stop_when_entity_reaches(
        self,
        entity: &str,
        position: crate::types::positions::Position,
    ) -> BuilderResult<Self> {
        use crate::builder::conditions::{ReachPositionConditionBuilder, TriggerBuilder};

        let reach_condition = ReachPositionConditionBuilder::new()
            .for_entity(entity)
            .at_position(position)
            .build()?;

        let trigger = TriggerBuilder::new()
            .add_condition_group()
            .add_condition(reach_condition)
            .finish_group()
            .build()?;

        Ok(self.with_stop_trigger(trigger))
    }

    /// Stop on custom condition
    pub fn stop_on_condition(
        self,
        condition: crate::types::scenario::triggers::Condition,
    ) -> BuilderResult<Self> {
        use crate::builder::conditions::TriggerBuilder;

        let trigger = TriggerBuilder::new()
            .add_condition_group()
            .add_condition(condition)
            .finish_group()
            .build()?;

        Ok(self.with_stop_trigger(trigger))
    }

    /// Finish storyboard and return to scenario builder
    pub fn finish(self) -> ScenarioBuilder<crate::builder::scenario::Complete> {
        let storyboard = Storyboard {
            init: self.init.unwrap_or_else(|| {
                // Provide basic default with environment action if no init specified
                InitActionBuilder::with_default_environment()
                    .build()
                    .unwrap_or_default()
            }),
            stories: self.stories,
            stop_trigger: self.stop_trigger,
        };

        // Convert scenario builder to Complete state
        let mut data = self.scenario_builder.data;
        data.storyboard = Some(storyboard);

        // Create Complete state scenario builder
        ScenarioBuilder::from_data_complete(data)
    }
}

/// Builder for individual stories
pub struct StoryBuilder<'parent> {
    parent: &'parent mut StoryboardBuilder,
    name: String,
    acts: Vec<Act>,
    parameter_declarations: Option<ParameterDeclarations>,
}

impl<'parent> StoryBuilder<'parent> {
    pub fn new(parent: &'parent mut StoryboardBuilder, name: &str) -> Self {
        Self {
            parent,
            name: name.to_string(),
            acts: Vec::new(),
            parameter_declarations: None,
        }
    }

    /// Add parameter declarations to this story
    pub fn with_parameters(mut self, params: ParameterDeclarations) -> Self {
        self.parameter_declarations = Some(params);
        self
    }

    /// Add an act to this story
    ///
    /// # Usage Note
    /// Due to Rust lifetime variance constraints, fluent chaining may be limited.
    /// Use the returned ActBuilder and call `.finish()` to complete the act.
    /// For unlimited fluent chaining, use `create_act()` instead.
    ///
    /// # Example
    /// ```rust,ignore
    /// // Working pattern:
    /// let act_builder = story_builder.add_act("act1");
    /// act_builder.finish();
    ///
    /// // Or use the direct pattern:
    /// story_builder.add_act("act1").finish();
    /// ```
    pub fn add_act<'a>(&'a mut self, name: &str) -> ActBuilder<'a>
    where
        'a: 'parent,
    {
        ActBuilder::new(self, name)
    }

    /// Create a detached act builder (no lifetime constraints)
    ///
    /// This method returns a DetachedActBuilder that has no lifetime parameters
    /// and supports perfect fluent chaining. Use `attach_to()` to add the completed
    /// act to this story.
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut detached_act = story_builder.create_act("act1");
    /// detached_act.with_start_trigger(trigger).attach_to(&mut story_builder);
    /// ```
    pub fn create_act(&self, name: &str) -> DetachedActBuilder {
        DetachedActBuilder::new(name)
    }

    /// Finish this story
    pub fn finish(self) -> &'parent mut StoryboardBuilder {
        let story = ScenarioStory {
            name: OSString::literal(self.name),
            parameter_declarations: self.parameter_declarations,
            acts: self.acts,
        };

        self.parent.stories.push(story);
        self.parent
    }
}

/// Builder for acts within stories  
pub struct ActBuilder<'parent> {
    parent: &'parent mut StoryBuilder<'parent>,
    name: String,
    maneuver_groups: Vec<ManeuverGroup>,
    start_trigger: Option<Trigger>,
    stop_trigger: Option<Trigger>,
}

impl<'parent> ActBuilder<'parent> {
    pub fn new(parent: &'parent mut StoryBuilder<'parent>, name: &str) -> Self {
        Self {
            parent,
            name: name.to_string(),
            maneuver_groups: Vec::new(),
            start_trigger: None,
            stop_trigger: None,
        }
    }

    /// Set start trigger for this act
    pub fn with_start_trigger(mut self, trigger: Trigger) -> Self {
        self.start_trigger = Some(trigger);
        self
    }

    /// Set stop trigger for this act
    pub fn with_stop_trigger(mut self, trigger: Trigger) -> Self {
        self.stop_trigger = Some(trigger);
        self
    }

    /// Add a maneuver to this act
    ///
    /// # Usage Note  
    /// Due to Rust lifetime variance constraints, fluent chaining may be limited.
    /// Use the returned ManeuverBuilder and call `.finish()` to complete the maneuver.
    /// For unlimited fluent chaining, use `create_maneuver()` instead.
    ///
    /// # Example
    /// ```rust,ignore
    /// // Working pattern:
    /// let maneuver_builder = act_builder.add_maneuver("maneuver1", "vehicle1");
    /// maneuver_builder.finish();
    ///
    /// // Or use the direct pattern:
    /// act_builder.add_maneuver("maneuver1", "vehicle1").finish();
    /// ```
    pub fn add_maneuver<'a>(
        &'a mut self,
        name: &str,
        entity_ref: &str,
    ) -> crate::builder::storyboard::maneuver::ManeuverBuilder<'a>
    where
        'a: 'parent,
    {
        crate::builder::storyboard::maneuver::ManeuverBuilder::new(self, name, entity_ref)
    }

    /// Create a detached maneuver builder (no lifetime constraints)
    ///
    /// This method returns a DetachedManeuverBuilder that has no lifetime parameters
    /// and supports perfect fluent chaining. Use `attach_to()` to add the completed
    /// maneuver to this act.
    ///
    /// # Example
    /// ```rust,ignore
    /// let detached_maneuver = act_builder.create_maneuver("maneuver1", "vehicle1");
    /// detached_maneuver.attach_to(&mut act_builder);
    /// ```
    pub fn create_maneuver(
        &self,
        name: &str,
        entity_ref: &str,
    ) -> crate::builder::storyboard::maneuver::DetachedManeuverBuilder {
        crate::builder::storyboard::maneuver::DetachedManeuverBuilder::new(name, entity_ref)
    }

    /// Internal method to add completed maneuver
    pub(crate) fn add_maneuver_to_group(
        &mut self,
        maneuver: crate::types::scenario::story::Maneuver,
        entity_ref: &str,
    ) {
        // Find or create maneuver group
        if self.maneuver_groups.is_empty() {
            self.maneuver_groups.push(ManeuverGroup {
                name: OSString::literal(format!("{}_Group", self.name)),
                maximum_execution_count: Some(UnsignedInt::literal(1)),
                actors: Actors {
                    select_triggering_entities: Some(false),
                    entity_refs: vec![crate::types::scenario::story::EntityRef {
                        entity_ref: crate::types::basic::Value::literal(entity_ref.to_string()),
                    }],
                },
                catalog_reference: None,
                maneuvers: Vec::new(),
            });
        } else {
            // Add entity_ref if not already present
            let entity_ref_val = crate::types::scenario::story::EntityRef {
                entity_ref: crate::types::basic::Value::literal(entity_ref.to_string()),
            };
            let actors = &mut self.maneuver_groups[0].actors;

            // Check if entity already exists
            let already_exists = actors
                .entity_refs
                .iter()
                .any(|e| e.entity_ref.as_literal().unwrap_or(&String::new()) == entity_ref);

            if !already_exists {
                actors.entity_refs.push(entity_ref_val);
            }
        }

        self.maneuver_groups[0].maneuvers.push(maneuver);
    }

    /// Finish this act
    pub fn finish(self) -> &'parent mut StoryBuilder<'parent> {
        let act = Act {
            name: OSString::literal(self.name),
            maneuver_groups: self.maneuver_groups,
            start_trigger: self.start_trigger,
            stop_trigger: self.stop_trigger,
        };

        self.parent.acts.push(act);
        self.parent
    }
}

/// Detached builder for acts (no lifetime constraints)
///
/// This builder has no lifetime parameters and supports perfect fluent chaining.
/// Use `attach_to()` to add the completed act to a story builder.
pub struct DetachedActBuilder {
    name: String,
    maneuver_groups: Vec<ManeuverGroup>,
    start_trigger: Option<Trigger>,
    stop_trigger: Option<Trigger>,
}

impl DetachedActBuilder {
    /// Create a new detached act builder
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            maneuver_groups: Vec::new(),
            start_trigger: None,
            stop_trigger: None,
        }
    }

    /// Set start trigger for this act
    pub fn with_start_trigger(mut self, trigger: Trigger) -> Self {
        self.start_trigger = Some(trigger);
        self
    }

    /// Set stop trigger for this act
    pub fn with_stop_trigger(mut self, trigger: Trigger) -> Self {
        self.stop_trigger = Some(trigger);
        self
    }

    /// Add a maneuver using closure-based configuration
    pub fn add_maneuver<F>(mut self, name: &str, entity_ref: &str, config: F) -> Self
    where
        F: FnOnce(
            crate::builder::storyboard::maneuver::DetachedManeuverBuilder,
        ) -> crate::builder::storyboard::maneuver::DetachedManeuverBuilder,
    {
        let maneuver_builder =
            crate::builder::storyboard::maneuver::DetachedManeuverBuilder::new(name, entity_ref);
        let configured_builder = config(maneuver_builder);
        let maneuver = configured_builder.build();

        // Find or create maneuver group
        if self.maneuver_groups.is_empty() {
            self.maneuver_groups.push(ManeuverGroup {
                name: OSString::literal(format!("{}_Group", self.name)),
                maximum_execution_count: Some(UnsignedInt::literal(1)),
                actors: Actors {
                    select_triggering_entities: Some(false),
                    entity_refs: vec![crate::types::scenario::story::EntityRef {
                        entity_ref: crate::types::basic::Value::literal(entity_ref.to_string()),
                    }],
                },
                catalog_reference: None,
                maneuvers: Vec::new(),
            });
        } else {
            // Add entity_ref if not already present
            let entity_ref_val = crate::types::scenario::story::EntityRef {
                entity_ref: crate::types::basic::Value::literal(entity_ref.to_string()),
            };
            let actors = &mut self.maneuver_groups[0].actors;

            // Check if entity already exists
            let already_exists = actors
                .entity_refs
                .iter()
                .any(|e| e.entity_ref.as_literal().unwrap_or(&String::new()) == entity_ref);

            if !already_exists {
                actors.entity_refs.push(entity_ref_val);
            }
        }

        self.maneuver_groups[0].maneuvers.push(maneuver);
        self
    }

    /// Create a detached maneuver builder
    pub fn create_maneuver(
        &self,
        name: &str,
        entity_ref: &str,
    ) -> crate::builder::storyboard::maneuver::DetachedManeuverBuilder {
        crate::builder::storyboard::maneuver::DetachedManeuverBuilder::new(name, entity_ref)
    }

    /// Add a completed maneuver to this act
    pub fn add_completed_maneuver(
        &mut self,
        maneuver: crate::types::scenario::story::Maneuver,
        entity_ref: &str,
    ) {
        // Find or create maneuver group
        if self.maneuver_groups.is_empty() {
            self.maneuver_groups.push(ManeuverGroup {
                name: OSString::literal(format!("{}_Group", self.name)),
                maximum_execution_count: Some(UnsignedInt::literal(1)),
                actors: Actors {
                    select_triggering_entities: Some(false),
                    entity_refs: vec![crate::types::scenario::story::EntityRef {
                        entity_ref: crate::types::basic::Value::literal(entity_ref.to_string()),
                    }],
                },
                catalog_reference: None,
                maneuvers: Vec::new(),
            });
        } else {
            // Add entity_ref if not already present
            let entity_ref_val = crate::types::scenario::story::EntityRef {
                entity_ref: crate::types::basic::Value::literal(entity_ref.to_string()),
            };
            let actors = &mut self.maneuver_groups[0].actors;

            // Check if entity already exists
            let already_exists = actors
                .entity_refs
                .iter()
                .any(|e| e.entity_ref.as_literal().unwrap_or(&String::new()) == entity_ref);

            if !already_exists {
                actors.entity_refs.push(entity_ref_val);
            }
        }

        self.maneuver_groups[0].maneuvers.push(maneuver);
    }

    /// Attach this act to a story builder
    pub fn attach_to(self, story: &mut StoryBuilder<'_>) {
        let act = Act {
            name: OSString::literal(self.name),
            maneuver_groups: self.maneuver_groups,
            start_trigger: self.start_trigger,
            stop_trigger: self.stop_trigger,
        };
        story.acts.push(act);
    }

    /// Build the final Act object
    pub fn build(self) -> Act {
        Act {
            name: OSString::literal(self.name),
            maneuver_groups: self.maneuver_groups,
            start_trigger: self.start_trigger,
            stop_trigger: self.stop_trigger,
        }
    }
}

/// Helper builder that connects InitActionBuilder to StoryboardBuilder
pub struct InitActionBuilderForStoryboard {
    storyboard_builder: StoryboardBuilder,
    init_builder: InitActionBuilder,
}

impl InitActionBuilderForStoryboard {
    pub fn new(storyboard_builder: StoryboardBuilder) -> Self {
        Self {
            storyboard_builder,
            init_builder: InitActionBuilder::new(),
        }
    }

    /// Add a global environment action with default environment
    pub fn add_global_environment_action(mut self) -> Self {
        self.init_builder = self.init_builder.add_global_environment_action();
        self
    }

    /// Add a teleport action for an entity (convenience method)
    pub fn add_teleport_action(
        mut self,
        entity_ref: &str,
        position: crate::types::positions::Position,
    ) -> Self {
        self.init_builder = self.init_builder.add_teleport_action(entity_ref, position);
        self
    }

    /// Add a speed action for an entity (convenience method)
    pub fn add_speed_action(mut self, entity_ref: &str, speed: f64) -> Self {
        self.init_builder = self.init_builder.add_speed_action(entity_ref, speed);
        self
    }

    /// Create a private action builder for an entity
    pub fn create_private_action(self, entity_ref: &str) -> PrivateActionBuilderForStoryboard {
        PrivateActionBuilderForStoryboard::new(self, entity_ref)
    }

    /// Finish building init actions and return to storyboard builder
    pub fn finish(self) -> BuilderResult<StoryboardBuilder> {
        let init = self.init_builder.build()?;
        Ok(self.storyboard_builder.with_init_actions(init))
    }
}

/// Detached story builder (no lifetime constraints)
pub struct DetachedStoryBuilder {
    name: String,
    acts: Vec<Act>,
    parameter_declarations: Option<ParameterDeclarations>,
}

impl DetachedStoryBuilder {
    /// Create a new detached story builder
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            acts: Vec::new(),
            parameter_declarations: None,
        }
    }

    /// Add parameter declarations to this story
    pub fn with_parameters(mut self, params: ParameterDeclarations) -> Self {
        self.parameter_declarations = Some(params);
        self
    }

    /// Add an act using closure-based configuration
    pub fn add_act<F>(mut self, name: &str, config: F) -> Self
    where
        F: FnOnce(DetachedActBuilder) -> DetachedActBuilder,
    {
        let act_builder = DetachedActBuilder::new(name);
        let configured_builder = config(act_builder);
        let act = configured_builder.build();
        self.acts.push(act);
        self
    }

    /// Build the final story
    pub fn build(self) -> ScenarioStory {
        ScenarioStory {
            name: OSString::literal(self.name),
            parameter_declarations: self.parameter_declarations,
            acts: self.acts,
        }
    }
}

/// Helper builder for private actions within storyboard context
pub struct PrivateActionBuilderForStoryboard {
    parent: InitActionBuilderForStoryboard,
    entity_ref: String,
    private_builder: crate::builder::init::PrivateActionBuilder,
}

impl PrivateActionBuilderForStoryboard {
    pub fn new(parent: InitActionBuilderForStoryboard, entity_ref: &str) -> Self {
        let storyboard_builder = parent.storyboard_builder;
        let private_builder = parent.init_builder.create_private_action(entity_ref);
        Self {
            parent: InitActionBuilderForStoryboard {
                storyboard_builder,
                init_builder: InitActionBuilder::new(), // Placeholder, will be replaced by finish()
            },
            entity_ref: entity_ref.to_string(),
            private_builder,
        }
    }

    /// Add a teleport action to position the entity
    pub fn add_teleport_action(mut self, position: crate::types::positions::Position) -> Self {
        self.private_builder = self.private_builder.add_teleport_action(position);
        self
    }

    /// Add a speed action to set initial velocity
    pub fn add_speed_action(mut self, speed: f64) -> Self {
        self.private_builder = self.private_builder.add_speed_action(speed);
        self
    }

    /// Finish this private action and return to init builder
    pub fn finish(self) -> InitActionBuilderForStoryboard {
        InitActionBuilderForStoryboard {
            storyboard_builder: self.parent.storyboard_builder,
            init_builder: self.private_builder.finish(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::scenario::ScenarioBuilder;

    #[test]
    fn test_storyboard_builder_creation() {
        let scenario_builder = ScenarioBuilder::new()
            .with_header("Test", "Author")
            .with_entities();

        let storyboard_builder = StoryboardBuilder::new(scenario_builder);
        assert_eq!(storyboard_builder.stories.len(), 0);
        assert!(storyboard_builder.init.is_none());
    }

    #[test]
    fn test_story_builder_creation() {
        let scenario_builder = ScenarioBuilder::new()
            .with_header("Test", "Author")
            .with_entities();

        let mut storyboard_builder = StoryboardBuilder::new(scenario_builder);
        let story_builder = StoryBuilder::new(&mut storyboard_builder, "TestStory");

        assert_eq!(story_builder.name, "TestStory");
        assert_eq!(story_builder.acts.len(), 0);
    }

    #[test]
    fn test_maneuver_group_has_actors() {
        let mut act = DetachedActBuilder::new("test_act");

        // Create a simple maneuver
        let maneuver = crate::builder::storyboard::maneuver::DetachedManeuverBuilder::new(
            "test_maneuver",
            "ego",
        )
        .build();

        act.add_completed_maneuver(maneuver, "ego");

        // Verify ManeuverGroup has the entity_ref
        assert_eq!(act.maneuver_groups.len(), 1);
        let actors = &act.maneuver_groups[0].actors;
        assert_eq!(actors.entity_refs.len(), 1);
        assert_eq!(
            actors.entity_refs[0].entity_ref.as_literal().unwrap(),
            "ego"
        );
    }

    #[test]
    fn test_maneuver_group_multiple_actors() {
        let mut act = DetachedActBuilder::new("test_act");

        // Add two maneuvers with different actors
        let maneuver1 =
            crate::builder::storyboard::maneuver::DetachedManeuverBuilder::new("maneuver1", "ego")
                .build();

        let maneuver2 =
            crate::builder::storyboard::maneuver::DetachedManeuverBuilder::new("maneuver2", "npc")
                .build();

        act.add_completed_maneuver(maneuver1, "ego");
        act.add_completed_maneuver(maneuver2, "npc");

        // Verify ManeuverGroup has both entity_refs
        assert_eq!(act.maneuver_groups.len(), 1);
        let actors = &act.maneuver_groups[0].actors;
        assert_eq!(actors.entity_refs.len(), 2);

        let refs: Vec<&str> = actors
            .entity_refs
            .iter()
            .map(|v| v.entity_ref.as_literal().unwrap().as_str())
            .collect();

        assert!(refs.contains(&"ego"));
        assert!(refs.contains(&"npc"));
    }

    #[test]
    fn test_maneuver_group_deduplicates_actors() {
        let mut act = DetachedActBuilder::new("test_act");

        // Add two maneuvers with same actor
        let maneuver1 =
            crate::builder::storyboard::maneuver::DetachedManeuverBuilder::new("maneuver1", "ego")
                .build();

        let maneuver2 =
            crate::builder::storyboard::maneuver::DetachedManeuverBuilder::new("maneuver2", "ego")
                .build();

        act.add_completed_maneuver(maneuver1, "ego");
        act.add_completed_maneuver(maneuver2, "ego");

        // Verify ManeuverGroup has only one entity_ref (deduplicated)
        assert_eq!(act.maneuver_groups.len(), 1);
        let actors = &act.maneuver_groups[0].actors;
        assert_eq!(actors.entity_refs.len(), 1);
        assert_eq!(
            actors.entity_refs[0].entity_ref.as_literal().unwrap(),
            "ego"
        );
    }
}
