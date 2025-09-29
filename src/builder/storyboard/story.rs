//! Story and Act builders for scenario behavior definition

use crate::builder::{BuilderError, BuilderResult, scenario::ScenarioBuilder, scenario::HasEntities};
use crate::types::{
    scenario::{
        storyboard::Storyboard,
        story::{ScenarioStory, Act, ManeuverGroup, Actors, EntityRef},
        init::Init,
        triggers::Trigger,
    },
    basic::{OSString, UnsignedInt, ParameterDeclarations},
};
use std::marker::PhantomData;

/// Builder for complete storyboard
pub struct StoryboardBuilder {
    scenario_builder: ScenarioBuilder<HasEntities>,
    stories: Vec<ScenarioStory>,
    init: Option<Init>,
}

impl StoryboardBuilder {
    pub fn new(scenario_builder: ScenarioBuilder<HasEntities>) -> Self {
        Self {
            scenario_builder,
            stories: Vec::new(),
            init: Some(Init::default()),
        }
    }
    
    /// Add a story to the storyboard
    pub fn add_story(&mut self, name: &str) -> StoryBuilder<'_> {
        StoryBuilder::new(self, name)
    }
    
    /// Finish storyboard and return to scenario builder
    pub fn finish(mut self) -> ScenarioBuilder<crate::builder::scenario::Complete> {
        let storyboard = Storyboard {
            init: self.init.unwrap_or_default(),
            stories: self.stories,
            stop_trigger: None,
        };
        
        // Convert scenario builder to Complete state
        let mut data = self.scenario_builder.data;
        data.storyboard = Some(storyboard);
        
        ScenarioBuilder {
            _state: PhantomData,
            data,
        }
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
    pub fn add_act(&mut self, name: &str) -> ActBuilder<'_> {
        ActBuilder::new(self, name)
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
    pub fn add_maneuver(&mut self, name: &str, entity_ref: &str) -> crate::builder::storyboard::maneuver::ManeuverBuilder<'_> {
        crate::builder::storyboard::maneuver::ManeuverBuilder::new(self, name, entity_ref)
    }
    
    /// Internal method to add completed maneuver
    pub(crate) fn add_maneuver_to_group(&mut self, maneuver: crate::types::scenario::story::Maneuver) {
        // Find or create maneuver group
        if self.maneuver_groups.is_empty() {
            self.maneuver_groups.push(ManeuverGroup {
                name: OSString::literal(format!("{}_Group", self.name)),
                maximum_execution_count: Some(UnsignedInt::literal(1)),
                actors: Actors {
                    select_triggering_entities: Some(false),
                    entity_refs: Vec::new(),
                },
                catalog_reference: None,
                maneuvers: Vec::new(),
            });
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
        assert!(storyboard_builder.init.is_some());
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
}