//! Parsing, manipulation and generation of OpenSCENARIO files.
//!
//! Targets **OpenSCENARIO 1.3**, the schema bundled at `Schema/OpenSCENARIO.xsd`.
//! Optionality follows that schema: a schema-optional field is `Option<T>`, and no
//! default is invented beyond what the schema itself declares. Known gaps are
//! listed in `docs/xsd_gaps.md`.
//!
//! Attributes carrying a scalar are typed [`types::basic::Value<T>`], so `speed="30.0"`
//! and `speed="${target_speed}"` both parse and the reference is resolved later.
//! Enum-valued attributes do not yet accept a parameter; see `docs/xsd_gaps.md`.
//!
//! Two features are off by default: `validation` adds schema validation and the
//! semantic checks, `builder` adds programmatic construction.
//!
//! ```rust,no_run
//! use openscenario_rs::{parse_file, Result, OpenScenarioDocumentType};
//!
//! fn main() -> Result<()> {
//!     let document = parse_file("scenario.xosc")?;
//!     println!("Author: {}", document.file_header.author.as_literal().unwrap());
//!     
//!     match document.document_type() {
//!         OpenScenarioDocumentType::Scenario => {
//!             if let Some(entities) = &document.entities {
//!                 println!("Entities: {}", entities.scenario_objects.len());
//!             }
//!         }
//!         OpenScenarioDocumentType::ParameterVariation => {
//!             println!("Parameter variation file");
//!         }
//!         OpenScenarioDocumentType::Catalog => {
//!             println!("Catalog file");
//!         }
//!         OpenScenarioDocumentType::Unknown => {
//!             println!("Unknown document type");
//!         }
//!     }
//!     Ok(())
//! }
//! ```

// Every instance of this lint in the crate sits on an enum modeling an `xsd:choice`, where
// variant size is set by the schema's branch types, not by a design choice here. Boxing the
// large branch would add an indirection the wire format does not want, purely to quiet the
// lint. The allow is crate-level rather than repeated per enum because the population is
// coextensive with "mirrors an xsd:choice": every new choice conversion would otherwise need
// its own local allow.
#![allow(clippy::large_enum_variant)]

// Module declarations
pub mod catalog;
pub mod error;
pub mod expression;
pub mod parser;
pub mod types;

#[cfg(feature = "builder")]
pub mod builder;

#[cfg(feature = "validation")]
pub mod validation;
// Re-export core types for convenience
pub use error::{Error, Result};
pub use types::scenario::storyboard::{
    FileHeader, OpenScenario, OpenScenarioDocumentType, ScenarioDefinition,
};

// Re-export parser functions
pub use parser::xml::{
    parse_catalog_from_file, parse_catalog_from_str, parse_from_file, parse_from_file_resolved,
    parse_from_str, parse_from_str_resolved, serialize_catalog_to_file,
    serialize_catalog_to_string, serialize_to_file, serialize_to_string,
};

// Re-export choice group infrastructure
pub use parser::choice_groups::{
    parse_choice_group, ChoiceGroupParser, ChoiceGroupRegistry, XsdChoiceGroup,
};

// Re-export expression evaluation
pub use expression::evaluate_expression;

// Re-export catalog system
pub use catalog::{CatalogLoader, CatalogManager, CatalogResolver, ResolvedCatalog};

// Feature-gated re-exports
#[cfg(feature = "builder")]
pub use builder::ScenarioBuilder;

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

/// Parse an OpenSCENARIO document from a string and resolve its parameter references
///
/// [`parse_str`] keeps every `$name` and `${expression}` as written, so that a document
/// round-trips unchanged. This function instead resolves each of them against the document's
/// own `<ParameterDeclarations>`, following the scoping rules of ASAM OpenSCENARIO XML section
/// 9.1, and returns the document with literal values in their place. The name pairs with
/// [`parse_str`] because the two take the same input and differ only in that step.
///
/// Each `<CatalogReference>` is replaced by the catalog entry it names, with the reference's
/// `<ParameterAssignments>` overriding the entry's defaults as section 9.5 describes; see
/// [`parser::resolve`] for the rules. A string has no location of its own, so a relative
/// catalog directory is taken relative to the current working directory; use
/// [`parse_file_resolved`] to take it relative to the scenario file. An undeclared parameter, a
/// failing expression, an invalid declaration or a catalog entry that cannot be located is an
/// error naming the element path and line.
///
/// # Example
/// ```rust
/// use openscenario_rs::parse_str_resolved;
///
/// let xml = r#"
/// <?xml version="1.0" encoding="UTF-8"?>
/// <OpenSCENARIO>
///   <FileHeader author="$author" date="2024-01-01T00:00:00" description="Test" revMajor="1" revMinor="3"/>
///   <ParameterDeclarations>
///     <ParameterDeclaration name="author" parameterType="string" value="Example"/>
///   </ParameterDeclarations>
///   <CatalogLocations/>
///   <RoadNetwork/>
///   <Entities/>
///   <Storyboard><Init><Actions/></Init><StopTrigger/></Storyboard>
/// </OpenSCENARIO>
/// "#;
///
/// let scenario = parse_str_resolved(xml)?;
/// assert_eq!(scenario.file_header.author.as_literal().map(String::as_str), Some("Example"));
/// # Ok::<(), openscenario_rs::Error>(())
/// ```
pub fn parse_str_resolved(xml: &str) -> Result<OpenScenario> {
    parse_from_str_resolved(xml)
}

/// Parse an OpenSCENARIO file and resolve its parameter references and catalog references, as
/// [`parse_str_resolved`] does for a string. A relative catalog directory is taken relative to
/// the directory holding the file.
pub fn parse_file_resolved<P: AsRef<Path>>(path: P) -> Result<OpenScenario> {
    parse_from_file_resolved(path)
}

/// Serialize an OpenSCENARIO document to XML string
///
/// This is a convenience function that wraps `parser::xml::serialize_to_string`.
///
/// # Example
/// ```rust
/// use openscenario_rs::{parse_str, serialize_str};
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
/// let xml = serialize_str(&scenario)?;
/// println!("{}", xml);
/// # Ok::<(), openscenario_rs::Error>(())
/// ```
pub fn serialize_str(scenario: &OpenScenario) -> Result<String> {
    serialize_to_string(scenario)
}
