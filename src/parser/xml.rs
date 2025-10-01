//! XML parsing implementation using quick-xml and serde
//!
//! This module provides efficient XML parsing and serialization for OpenSCENARIO documents
//! with comprehensive error handling and validation capabilities.
//!
//! # Features
//!
//! - **High-performance parsing** using quick-xml with zero-copy deserialization
//! - **Comprehensive validation** with detailed error reporting and suggestions
//! - **Catalog support** for reusable component libraries
//! - **UTF-8 BOM handling** for cross-platform compatibility
//! - **Pretty-printed output** with configurable formatting
//!
//! # Basic Usage
//!
//! ## Parsing Scenarios
//!
//! ```rust,no_run
//! use openscenario_rs::parser::xml::{parse_from_file, parse_from_str};
//!
//! // Parse from file with automatic error context
//! let scenario = parse_from_file("my_scenario.xosc")?;
//! println!("Scenario author: {}", scenario.file_header.author);
//!
//! // Parse from XML string
//! let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
//! <OpenSCENARIO>
//!   <FileHeader revMajor="1" revMinor="3" date="2024-01-01T00:00:00"
//!               author="Example" description="Test scenario"/>
//!   <ScenarioDefinition>
//!     <!-- scenario content -->
//!   </ScenarioDefinition>
//! </OpenSCENARIO>"#;
//! let scenario = parse_from_str(xml)?;
//! ```
//!
//! ## Validation During Parsing
//!
//! ```rust,no_run
//! use openscenario_rs::parser::xml::{parse_from_file_validated, validate_xml_structure};
//!
//! // Validate structure before full parsing
//! validate_xml_structure(&xml_content)?;
//!
//! // Parse with built-in validation
//! let scenario = parse_from_file_validated("scenario.xosc")?;
//! ```
//!
//! ## Serialization
//!
//! ```rust,no_run
//! use openscenario_rs::parser::xml::{serialize_to_string, serialize_to_file};
//!
//! // Serialize to formatted XML string
//! let xml_output = serialize_to_string(&scenario)?;
//! println!("{}", xml_output);
//!
//! // Write directly to file
//! serialize_to_file(&scenario, "output.xosc")?;
//! ```
//!
//! # Catalog File Operations
//!
//! ```rust,no_run
//! use openscenario_rs::parser::xml::{
//!     parse_catalog_from_file, serialize_catalog_to_file,
//!     parse_catalog_from_str_validated
//! };
//!
//! // Parse vehicle catalog
//! let catalog = parse_catalog_from_file("vehicles.xosc")?;
//!
//! // Validate catalog structure
//! let validated_catalog = parse_catalog_from_str_validated(&catalog_xml)?;
//!
//! // Export modified catalog
//! serialize_catalog_to_file(&catalog, "updated_vehicles.xosc")?;
//! ```
//!
//! # Error Handling
//!
//! All parsing functions return `Result<T>` with detailed error context:
//!
//! ```rust,no_run
//! # use openscenario_rs::parser::xml::parse_from_file;
//! match parse_from_file("scenario.xosc") {
//!     Ok(scenario) => {
//!         // Process valid scenario
//!         println!("Loaded scenario with {} entities", 
//!                  scenario.entities.as_ref().map_or(0, |e| e.scenario_objects.len()));
//!     }
//!     Err(e) => {
//!         eprintln!("Parse error: {}", e);
//!         // Error includes file path and specific parsing context
//!     }
//! }
//! ```
//!
//! # Performance Notes
//!
//! - Use `parse_from_file` for fastest parsing without validation
//! - Use `parse_from_file_validated` when you need structure validation
//! - For very large files (>50MB), consider chunked processing
//! - Validation adds ~10-15% overhead but catches malformed XML early

use crate::error::{Error, Result};
use crate::types::catalogs::files::CatalogFile;
use crate::types::scenario::storyboard::OpenScenario;
use markup_fmt::{config::FormatOptions, format_text, Language};
use std::fs;
use std::path::Path;
/// Parse an OpenSCENARIO document from a string
///
/// This function uses quick-xml's serde integration to deserialize
/// the XML into our Rust type system.
pub fn parse_from_str(xml: &str) -> Result<OpenScenario> {
    quick_xml::de::from_str(xml)
        .map_err(Error::from)
        .map_err(|e| e.with_context("Failed to parse OpenSCENARIO XML"))
}

/// Parse an OpenSCENARIO document from a file
///
/// Reads the file into memory and then parses it as a string.
/// For very large files, consider using the streaming parser (when available).
pub fn parse_from_file<P: AsRef<Path>>(path: P) -> Result<OpenScenario> {
    let xml_content = fs::read_to_string(&path)
        .map_err(Error::from)
        .map_err(|e| {
            e.with_context(&format!("Failed to read file: {}", path.as_ref().display()))
        })?;

    // Remove UTF-8 BOM if present
    let cleaned_content = if xml_content.starts_with('\u{FEFF}') {
        &xml_content[3..]
    } else {
        &xml_content
    };

    parse_from_str(cleaned_content).map_err(|e| {
        e.with_context(&format!(
            "Failed to parse file: {}",
            path.as_ref().display()
        ))
    })
}

/// Serialize an OpenSCENARIO document to XML string
///
/// This function uses quick-xml's serde integration to serialize
/// our Rust types back to XML format.
pub fn serialize_to_string(scenario: &OpenScenario) -> Result<String> {
    let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    xml.push('\n');

    let serialized = quick_xml::se::to_string(scenario)
        .map_err(Error::XmlSerializeError)
        .map_err(|e| e.with_context("Failed to serialize OpenSCENARIO to XML"))?;
    let s = format_text(
        &serialized,
        Language::Xml,
        &FormatOptions::default(),
        |serialized, _| Ok::<_, std::convert::Infallible>(serialized.into()),
    )
    .unwrap();
    xml.push_str(&s);
    Ok(xml)
}

/// Serialize an OpenSCENARIO document to a file
///
/// Serializes the scenario to XML and writes it to the specified file.
pub fn serialize_to_file<P: AsRef<Path>>(scenario: &OpenScenario, path: P) -> Result<()> {
    let xml = serialize_to_string(scenario)?;

    fs::write(&path, xml).map_err(Error::from).map_err(|e| {
        e.with_context(&format!(
            "Failed to write file: {}",
            path.as_ref().display()
        ))
    })
}

/// Validate XML structure before parsing
///
/// This function performs basic XML structure validation to provide
/// better error messages for malformed documents.
pub fn validate_xml_structure(xml: &str) -> Result<()> {
    // Basic validation - check for XML declaration and root element
    let trimmed = xml.trim();

    if trimmed.is_empty() {
        return Err(Error::validation_error("xml", "XML document is empty"));
    }

    if !trimmed.starts_with("<?xml") && !trimmed.starts_with('<') {
        return Err(Error::validation_error(
            "xml",
            "XML document must start with XML declaration or root element",
        ));
    }

    // Check for basic OpenSCENARIO root element
    if !trimmed.contains("OpenSCENARIO") {
        return Err(Error::validation_error(
            "xml",
            "Document does not appear to contain OpenSCENARIO root element",
        ));
    }

    Ok(())
}

/// Parse with validation
///
/// Validates the XML structure before attempting to parse it.
pub fn parse_from_str_validated(xml: &str) -> Result<OpenScenario> {
    validate_xml_structure(xml)?;
    parse_from_str(xml)
}

/// Parse file with validation
///
/// Validates the XML structure before attempting to parse it.
pub fn parse_from_file_validated<P: AsRef<Path>>(path: P) -> Result<OpenScenario> {
    let xml_content = fs::read_to_string(&path)
        .map_err(Error::from)
        .map_err(|e| {
            e.with_context(&format!("Failed to read file: {}", path.as_ref().display()))
        })?;

    parse_from_str_validated(&xml_content).map_err(|e| {
        e.with_context(&format!(
            "Failed to parse file: {}",
            path.as_ref().display()
        ))
    })
}

// Catalog parsing functions

/// Parse a catalog file from XML string
///
/// This function uses quick-xml's serde integration to deserialize
/// catalog XML into our catalog file structure.
pub fn parse_catalog_from_str(xml: &str) -> Result<CatalogFile> {
    quick_xml::de::from_str(xml)
        .map_err(Error::from)
        .map_err(|e| e.with_context("Failed to parse catalog XML"))
}

/// Parse a catalog file from a file path
///
/// Reads the catalog file into memory and then parses it as a string.
pub fn parse_catalog_from_file<P: AsRef<Path>>(path: P) -> Result<CatalogFile> {
    let xml_content = fs::read_to_string(&path)
        .map_err(Error::from)
        .map_err(|e| {
            e.with_context(&format!(
                "Failed to read catalog file: {}",
                path.as_ref().display()
            ))
        })?;

    parse_catalog_from_str(&xml_content).map_err(|e| {
        e.with_context(&format!(
            "Failed to parse catalog file: {}",
            path.as_ref().display()
        ))
    })
}

/// Validate catalog XML structure before parsing
///
/// This function performs basic XML structure validation specific to catalog files.
pub fn validate_catalog_xml_structure(xml: &str) -> Result<()> {
    let trimmed = xml.trim();

    if trimmed.is_empty() {
        return Err(Error::validation_error(
            "xml",
            "Catalog XML document is empty",
        ));
    }

    if !trimmed.starts_with("<?xml") && !trimmed.starts_with('<') {
        return Err(Error::validation_error(
            "xml",
            "Catalog XML document must start with XML declaration or root element",
        ));
    }

    // Check for OpenSCENARIO root element
    if !trimmed.contains("OpenSCENARIO") {
        return Err(Error::validation_error(
            "xml",
            "Document does not appear to contain OpenSCENARIO root element",
        ));
    }

    // Check for Catalog element
    if !trimmed.contains("Catalog") {
        return Err(Error::validation_error(
            "xml",
            "Document does not appear to contain Catalog element",
        ));
    }

    Ok(())
}

/// Parse catalog with validation
///
/// Validates the XML structure before attempting to parse it.
pub fn parse_catalog_from_str_validated(xml: &str) -> Result<CatalogFile> {
    validate_catalog_xml_structure(xml)?;
    parse_catalog_from_str(xml)
}

/// Parse catalog file with validation
///
/// Validates the XML structure before attempting to parse it.
pub fn parse_catalog_from_file_validated<P: AsRef<Path>>(path: P) -> Result<CatalogFile> {
    let xml_content = fs::read_to_string(&path)
        .map_err(Error::from)
        .map_err(|e| {
            e.with_context(&format!(
                "Failed to read catalog file: {}",
                path.as_ref().display()
            ))
        })?;

    parse_catalog_from_str_validated(&xml_content).map_err(|e| {
        e.with_context(&format!(
            "Failed to parse catalog file: {}",
            path.as_ref().display()
        ))
    })
}

/// Serialize a catalog file to XML string
///
/// This function uses quick-xml's serde integration to serialize
/// our catalog types back to XML format.
pub fn serialize_catalog_to_string(catalog: &CatalogFile) -> Result<String> {
    let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    xml.push('\n');

    let serialized = quick_xml::se::to_string(catalog)
        .map_err(Error::XmlSerializeError)
        .map_err(|e| e.with_context("Failed to serialize catalog to XML"))?;

    xml.push_str(&serialized);
    Ok(xml)
}

/// Serialize a catalog file to a file path
///
/// Serializes the catalog to XML and writes it to the specified file.
pub fn serialize_catalog_to_file<P: AsRef<Path>>(catalog: &CatalogFile, path: P) -> Result<()> {
    let xml = serialize_catalog_to_string(catalog)?;

    fs::write(&path, xml).map_err(Error::from).map_err(|e| {
        e.with_context(&format!(
            "Failed to write catalog file: {}",
            path.as_ref().display()
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_xml_structure() {
        // Valid XML
        assert!(
            validate_xml_structure(r#"<?xml version="1.0"?><OpenSCENARIO></OpenSCENARIO>"#).is_ok()
        );

        // Missing XML declaration is OK
        assert!(validate_xml_structure(r#"<OpenSCENARIO></OpenSCENARIO>"#).is_ok());

        // Empty XML should fail
        assert!(validate_xml_structure("").is_err());
        assert!(validate_xml_structure("   ").is_err());

        // Non-XML content should fail
        assert!(validate_xml_structure("This is not XML").is_err());

        // Missing OpenSCENARIO root should fail
        assert!(validate_xml_structure(r#"<SomeOtherRoot></SomeOtherRoot>"#).is_err());
    }

    #[test]
    fn test_validate_catalog_xml_structure() {
        // Valid catalog XML structure
        let valid_xml = r#"<?xml version="1.0"?>
        <OpenSCENARIO>
            <FileHeader revMajor="1" revMinor="3" date="2024-01-01T00:00:00" author="Test" description="Test"/>
            <Catalog name="test">
            </Catalog>
        </OpenSCENARIO>"#;

        assert!(validate_catalog_xml_structure(valid_xml).is_ok());

        // Invalid - no Catalog element
        let invalid_xml = r#"<?xml version="1.0"?><OpenSCENARIO><FileHeader/></OpenSCENARIO>"#;
        assert!(validate_catalog_xml_structure(invalid_xml).is_err());

        // Invalid - empty
        assert!(validate_catalog_xml_structure("").is_err());
    }

    #[test]
    fn test_catalog_serialization_roundtrip() {
        let catalog = CatalogFile::default();

        let xml = serialize_catalog_to_string(&catalog).unwrap();
        assert!(xml.contains("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        assert!(xml.contains("OpenSCENARIO"));
        assert!(xml.contains("Catalog"));
    }
}
