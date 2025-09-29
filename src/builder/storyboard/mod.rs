//! Storyboard builders for scenario behavior definition

pub mod story;
pub mod maneuver;

pub use story::{StoryboardBuilder, StoryBuilder, ActBuilder, DetachedActBuilder};
pub use maneuver::{ManeuverBuilder, SpeedActionEventBuilder, TeleportActionEventBuilder, DetachedManeuverBuilder, DetachedSpeedActionBuilder, DetachedTeleportActionBuilder};

use crate::builder::{BuilderError, BuilderResult};
use crate::types::{
    scenario::{
        storyboard::Storyboard,
        story::ScenarioStory,
        init::Init,
    },
};

/// Collection of stories for a storyboard
#[derive(Debug, Default)]
pub struct StoryCollection {
    stories: Vec<ScenarioStory>,
}

impl StoryCollection {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add a story to the collection
    pub fn add_story(mut self, story: ScenarioStory) -> Self {
        self.stories.push(story);
        self
    }
    
    /// Get all stories
    pub fn into_stories(self) -> Vec<ScenarioStory> {
        self.stories
    }
}