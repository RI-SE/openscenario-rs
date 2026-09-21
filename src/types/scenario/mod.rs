//! Scenario structure types

pub mod init;
pub mod monitors;
pub mod story;
pub mod storyboard;
pub mod triggers;
pub mod variables;

// Re-export main types for convenience
pub use init::{
    Actions, EnvironmentAction, GlobalAction, Init, LongitudinalAction, Private, PrivateAction,
};
pub use monitors::{MonitorDeclaration, MonitorDeclarations};
pub use story::{Act, Actors, EntityRef, Event, Maneuver, ManeuverGroup, ScenarioStory};
pub use storyboard::{FileHeader, OpenScenario, Storyboard};
pub use variables::{VariableDeclaration, VariableDeclarations};

// `ScenarioDefinition` is not defined here. The XSD group of that name (`:1989`) is
// modeled by `storyboard::ScenarioDefinition`, which this module's parent re-exports.
