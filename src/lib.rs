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
pub mod catalog;
pub mod error;
pub mod expression;
pub mod parser;
pub mod types;

#[cfg(feature = "builder")]
pub mod builder;
// Re-export core types for convenience
pub use error::{Error, Result};
pub use types::scenario::storyboard::{FileHeader, OpenScenario};

// Re-export parser functions
pub use parser::xml::{
    parse_catalog_from_file, parse_catalog_from_str, parse_from_file, parse_from_str,
    serialize_catalog_to_file, serialize_catalog_to_string, serialize_to_file, serialize_to_string,
};

// Re-export expression evaluation
pub use expression::evaluate_expression;

// Re-export catalog system
pub use catalog::{
    CatalogCache, CatalogLoader, CatalogManager, CatalogResolver, ParameterSubstitutionEngine,
    ResolvedCatalog,
};

// Feature-gated re-exports
#[cfg(feature = "builder")]
pub use builder::scenario::ScenarioBuilder;

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

/// Parse a catalog file from the filesystem
///
/// This is a convenience function that wraps `parser::xml::parse_catalog_from_file`
/// with additional context and error handling.
///
/// # Example
/// ```rust,no_run
/// use openscenario_rs::parse_catalog_file;
///
/// let catalog = parse_catalog_file("catalogs/vehicles.xosc")?;
/// # Ok::<(), openscenario_rs::Error>(())
/// ```
pub fn parse_catalog_file<P: AsRef<Path>>(path: P) -> Result<types::catalogs::files::CatalogFile> {
    parse_catalog_from_file(path)
}

/// Parse a catalog document from a string
///
/// This is a convenience function that wraps `parser::xml::parse_catalog_from_str`
/// with additional context and error handling.
///
/// # Example
/// ```rust
/// use openscenario_rs::parse_catalog_str;
///
/// let xml = r#"
/// <?xml version="1.0" encoding="UTF-8"?>
/// <OpenSCENARIO>
///   <FileHeader author="Test" date="2024-01-01" description="Test" revMajor="1" revMinor="0"/>
///   <Catalog name="TestCatalog"/>
/// </OpenSCENARIO>
/// "#;
///
/// let catalog = parse_catalog_str(xml)?;
/// # Ok::<(), openscenario_rs::Error>(())
/// ```
pub fn parse_catalog_str(xml: &str) -> Result<types::catalogs::files::CatalogFile> {
    parse_catalog_from_str(xml)
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
