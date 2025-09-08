//! Scenario structure types

pub mod init;
pub mod storyboard;
pub mod story;
pub mod triggers;

// Re-export main types for convenience
pub use init::{Init, Actions, GlobalAction, EnvironmentAction, Private, PrivateActionWrapper, PrivateActionType, LongitudinalAction, LongitudinalActionType};
pub use storyboard::{OpenScenario, FileHeader, Storyboard};
pub use story::{
    ScenarioStory, Act, ManeuverGroup, Maneuver, Event, Actors, EntityRef,
};