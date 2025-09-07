//! OpenSCENARIO-rs: Rust library for parsing and manipulating OpenSCENARIO files
//! 
//! This library provides a type-safe, serde-based implementation for parsing, 
//! manipulating, and generating OpenSCENARIO files in Rust.
//!
//! # Features
//!
//! - **Type-safe parsing** - Full type system for OpenSCENARIO specification
//! - **Parameter support** - Handle `${parameter}` references with resolution
//! - **Validation** - Schema validation and semantic checks (with `validation` feature)
//! - **Builder pattern** - Programmatic scenario construction (with `builder` feature) 
//! - **Streaming** - Memory-efficient parsing for large files (with `streaming` feature)
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use openscenario_rs::{parse_file, Result};
//!
//! fn main() -> Result<()> {
//!     let scenario = parse_file("scenario.xosc")?;
//!     println!("Author: {}", scenario.file_header.author.as_literal().unwrap());
//!     println!("Entities: {}", scenario.entities.scenario_objects.len());
//!     Ok(())
//! }
//! ```

// Module declarations
pub mod error;
pub mod types;
pub mod parser;

#[cfg(feature = "builder")]
pub mod builder;

#[cfg(feature = "validation")]
pub mod validation;

#[cfg(feature = "streaming")]
pub mod streaming;

// Re-export core types for convenience
pub use error::{Error, Result};
pub use types::scenario::storyboard::{OpenScenario, FileHeader};

// Re-export parser functions
pub use parser::xml::{parse_from_str, parse_from_file, serialize_to_string, serialize_to_file};

// Feature-gated re-exports
#[cfg(feature = "builder")]
pub use builder::scenario::ScenarioBuilder;

#[cfg(feature = "validation")]
pub use validation::Validate;

#[cfg(feature = "streaming")]
pub use parser::streaming::*;

// High-level convenience functions
use std::path::Path;

/// Parse an OpenSCENARIO file from the filesystem
/// 
/// This is a convenience function that wraps `parser::xml::parse_from_file`
/// with additional context and error handling.
///
/// # Example
/// ```rust,no_run
/// use openscenario_rs::parse_file;
/// 
/// let scenario = parse_file("examples/highway.xosc")?;
/// # Ok::<(), openscenario_rs::Error>(())
/// ```
pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<OpenScenario> {
    parse_from_file(path)
}

/// Parse an OpenSCENARIO document from a string
/// 
/// This is a convenience function that wraps `parser::xml::parse_from_str`
/// with additional context and error handling.
///
/// # Example
/// ```rust
/// use openscenario_rs::parse_str;
/// 
/// let xml = r#"
/// <?xml version="1.0" encoding="UTF-8"?>
/// <OpenSCENARIO>
///   <FileHeader author="Test" date="2024-01-01" description="Test" revMajor="1" revMinor="0"/>
///   <Entities/>
///   <Storyboard><Init><Actions/></Init></Storyboard>
/// </OpenSCENARIO>
/// "#;
/// 
/// let scenario = parse_str(xml)?;
/// # Ok::<(), openscenario_rs::Error>(())
/// ```
pub fn parse_str(xml: &str) -> Result<OpenScenario> {
    parse_from_str(xml)
}

/// Serialize an OpenSCENARIO document to XML string
///
/// This is a convenience function that wraps `parser::xml::serialize_to_string`.
///
/// # Example  
/// ```rust
/// use openscenario_rs::{serialize_str, OpenScenario};
/// 
/// let scenario = OpenScenario::default();
/// let xml = serialize_str(&scenario)?;
/// println!("{}", xml);
/// # Ok::<(), openscenario_rs::Error>(())
/// ```
pub fn serialize_str(scenario: &OpenScenario) -> Result<String> {
    serialize_to_string(scenario)
}

/// Validate an OpenSCENARIO document (requires `validation` feature)
#[cfg(feature = "validation")]
pub fn validate_scenario(scenario: &OpenScenario) -> Result<()> {
    use validation::ValidationContext;
    let ctx = ValidationContext::new();
    scenario.validate(&ctx)
}