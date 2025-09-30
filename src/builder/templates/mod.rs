//! Template builders for common scenario patterns
//!
//! This module provides pre-configured builders for common OpenSCENARIO patterns,
//! making it easier to create executable scenarios quickly.

pub mod basic;

pub use basic::BasicScenarioTemplate;

use crate::builder::{BuilderResult, scenario::{ScenarioBuilder, HasEntities}};

/// Trait for scenario templates
pub trait ScenarioTemplate {
    /// Create a pre-configured scenario builder
    fn create() -> ScenarioBuilder<HasEntities>;
    
    /// Create with custom header information
    fn create_with_header(name: &str, author: &str) -> ScenarioBuilder<HasEntities>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_template_creation() {
        let scenario = BasicScenarioTemplate::create();
        // Template should provide a working foundation
        assert!(scenario.data.file_header.is_some());
    }
}