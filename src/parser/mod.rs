//! XML parsing and deserialization for OpenSCENARIO documents
//!
//! This module provides comprehensive parsing capabilities for OpenSCENARIO XML files,
//! including scenario definitions, parameter variations, and catalog files.
//!
//! # Features
//!
//! - **Fast XML parsing** using quick-xml with zero-copy deserialization
//! - **Type-safe validation** with comprehensive error reporting
//! - **Choice group handling** for XSD polymorphic elements
//! - **Catalog file support** for reusable components
//! - **Parameter and expression** resolution
//!
//! # Basic Usage
//!
//! ```rust
//! use openscenario_rs::parser::xml::{parse_from_file, parse_from_str};
//! use openscenario_rs::parser::validation::ScenarioValidator;
//!
//! // Parse a scenario file
//! let scenario = parse_from_file("scenario.xosc")?;
//!
//! // Parse from XML string
//! let xml_content = r#"<?xml version="1.0"?>
//! <OpenSCENARIO>
//!   <FileHeader revMajor="1" revMinor="3" date="2024-01-01T00:00:00"
//!               author="Example" description="Basic scenario"/>
//!   <!-- scenario content -->
//! </OpenSCENARIO>"#;
//! let scenario = parse_from_str(xml_content)?;
//!
//! // Validate the parsed scenario
//! let mut validator = ScenarioValidator::new();
//! let result = validator.validate_scenario(&scenario);
//! if result.is_valid() {
//!     println!("Scenario is valid!");
//! } else {
//!     for error in result.errors {
//!         println!("Error: {}", error.message);
//!     }
//! }
//! ```
//!
//! # Advanced Features
//!
//! ## Custom Validation Configuration
//!
//! ```rust
//! use openscenario_rs::parser::validation::{ScenarioValidator, ValidationConfig};
//!
//! let config = ValidationConfig {
//!     strict_mode: true,
//!     validate_references: true,
//!     validate_constraints: true,
//!     max_errors: 50,
//!     ..Default::default()
//! };
//!
//! let mut validator = ScenarioValidator::with_config(config);
//! let result = validator.validate_scenario(&scenario);
//! ```
//!
//! ## Catalog File Parsing
//!
//! ```rust,no_run
//! use openscenario_rs::parser::xml::{parse_catalog_from_file, serialize_catalog_to_file};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Parse a catalog file
//! let catalog = parse_catalog_from_file("vehicles.xosc")?;
//!
//! // Serialize back to file
//! serialize_catalog_to_file(&catalog, "output_catalog.xosc")?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Performance Considerations
//!
//! - Parsing is optimized for speed with zero-copy deserialization
//! - Enable validation caching for repeated validation operations

pub mod choice_groups;
pub mod deserializer;
pub mod validation;
pub mod xml;
