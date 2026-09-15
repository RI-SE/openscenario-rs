//! Parsing of OpenSCENARIO documents: scenarios, catalogs, and parameter variations.
//!
//! [`xml`] holds the entry points, [`validation`] the domain checks that run after
//! a successful parse, and [`choice_groups`] the handling for the XSD constructs
//! serde cannot express directly.
//!
//! ```rust,no_run
//! use openscenario_rs::parser::xml::{parse_from_file, parse_from_str};
//! use openscenario_rs::parser::validation::ScenarioValidator;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let scenario = parse_from_file("scenario.xosc")?;
//!
//! let xml_content = r#"<?xml version="1.0"?>
//! <OpenSCENARIO>
//!   <FileHeader revMajor="1" revMinor="3" date="2024-01-01T00:00:00"
//!               author="Example" description="Basic scenario"/>
//! </OpenSCENARIO>"#;
//! let scenario = parse_from_str(xml_content)?;
//!
//! let mut validator = ScenarioValidator::new();
//! for error in validator.validate_scenario(&scenario).errors {
//!     println!("{}", error.message);
//! }
//! # Ok(())
//! # }
//! ```

pub mod choice_groups;
pub mod validation;
pub mod xml;
