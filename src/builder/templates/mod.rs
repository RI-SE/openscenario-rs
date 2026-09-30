//! Prebuilt starting points for the common scenario shapes, each returning a
//! `ScenarioBuilder` already past the header and entity stages.

pub mod basic;

pub use basic::BasicScenarioTemplate;

use crate::builder::scenario::{HasEntities, ScenarioBuilder};

/// Trait for scenario templates
pub trait ScenarioTemplate {
    /// Create a pre-configured scenario builder
    fn create() -> ScenarioBuilder<HasEntities>;

    /// Create with custom header information
    fn create_with_header(name: &str, author: &str) -> ScenarioBuilder<HasEntities>;
}
