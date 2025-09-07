//! Scenario structure types

pub mod storyboard;
pub mod story;
pub mod triggers;

// Re-export main types for convenience
pub use storyboard::{OpenScenario, FileHeader, Storyboard, Init};
pub use story::{
    ScenarioStory, Act, ManeuverGroup, Maneuver, Event, Actors, EntityRef,
};