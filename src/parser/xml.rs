//! XML parsing and serialization, over quick-xml and serde.
//!
//! Every function here returns `Result<T>`, and the error carries the file path and
//! the position that failed, not only the serde message. Parsing
//! strips a UTF-8 BOM if one is present; serialization pretty-prints.
//!
//! ```rust,no_run
//! use openscenario_rs::{parse_from_file, parse_from_str};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let scenario = parse_from_file("my_scenario.xosc")?;
//! println!("author: {}", scenario.file_header.author);
//!
//! let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
//! <OpenSCENARIO>
//!   <FileHeader revMajor="1" revMinor="3" date="2024-01-01T00:00:00"
//!               author="Example" description="Test scenario"/>
//!   <ScenarioDefinition>
//!   </ScenarioDefinition>
//! </OpenSCENARIO>"#;
//! let scenario = parse_from_str(xml)?;
//! # Ok(())
//! # }
//! ```
//!
//! ```rust,no_run
//! use openscenario_rs::{serialize_to_string, serialize_to_file};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let scenario = openscenario_rs::parse_from_file("scenario.xosc")?;
//! let xml_output = serialize_to_string(&scenario)?;
//! serialize_to_file(&scenario, "output.xosc")?;
//! # Ok(())
//! # }
//! ```
//!
//! Catalog files are a separate document root and get their own entry points:
//!
//! ```rust,no_run
//! use openscenario_rs::parser::xml::{
//!     parse_catalog_from_file, serialize_catalog_to_file,
//!     parse_catalog_from_str_validated
//! };
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let catalog = parse_catalog_from_file("vehicles.xosc")?;
//!
//! let catalog_xml = openscenario_rs::serialize_catalog_to_string(&catalog)?;
//! let validated_catalog = parse_catalog_from_str_validated(&catalog_xml)?;
//!
//! serialize_catalog_to_file(&catalog, "updated_vehicles.xosc")?;
//! # Ok(())
//! # }
//! ```
//!
//! The `*_validated` variants run a structural pass on top of the parse; the plain
//! ones return as soon as deserialization succeeds.
//!
//! ```rust,no_run
//! # use openscenario_rs::parser::xml::parse_from_file;
//! match parse_from_file("scenario.xosc") {
//!     Ok(scenario) => println!(
//!         "{} entities",
//!         scenario.entities.as_ref().map_or(0, |e| e.scenario_objects.len())
//!     ),
//!     Err(e) => eprintln!("parse error: {}", e),
//! }
//! ```

use crate::error::{Error, Result};
use crate::types::catalogs::files::CatalogFile;
use crate::types::scenario::storyboard::OpenScenario;
use markup_fmt::{config::FormatOptions, format_text, Language};
use std::fs;
use std::path::Path;

/// Maximum file size for parsing (100 MB)
const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024;

/// Remove BOM (Byte Order Mark) if present
fn remove_bom(content: &str) -> &str {
    // UTF-8 BOM: EF BB BF (represented as \u{FEFF} in decoded string)
    if content.starts_with('\u{FEFF}') {
        // The character is 3 bytes in UTF-8, but as a char it's 1 character
        &content['\u{FEFF}'.len_utf8()..]
    } else {
        content
    }
}

/// Internal helper to parse OpenSCENARIO from file
fn parse_from_file_internal<P: AsRef<Path>>(path: P, validate_xml: bool) -> Result<OpenScenario> {
    let metadata = fs::metadata(&path).map_err(Error::from).map_err(|e| {
        e.with_context(&format!(
            "Failed to read file metadata: {}",
            path.as_ref().display()
        ))
    })?;

    if metadata.len() > MAX_FILE_SIZE {
        return Err(Error::out_of_range(
            "file_size",
            &metadata.len().to_string(),
            "0",
            &MAX_FILE_SIZE.to_string(),
        ));
    }

    let xml_content = fs::read_to_string(&path)
        .map_err(Error::from)
        .map_err(|e| {
            e.with_context(&format!("Failed to read file: {}", path.as_ref().display()))
        })?;

    let cleaned_content = remove_bom(&xml_content);

    if validate_xml {
        validate_xml_structure(cleaned_content).map_err(|e| {
            e.with_context(&format!(
                "XML validation failed for file: {}",
                path.as_ref().display()
            ))
        })?;
    }

    parse_from_str(cleaned_content).map_err(|e| {
        e.with_context(&format!(
            "Failed to parse file: {}",
            path.as_ref().display()
        ))
    })
}

/// Internal helper to parse catalog from file
fn parse_catalog_from_file_internal<P: AsRef<Path>>(
    path: P,
    validate_xml: bool,
) -> Result<CatalogFile> {
    let metadata = fs::metadata(&path).map_err(Error::from).map_err(|e| {
        e.with_context(&format!(
            "Failed to read catalog file metadata: {}",
            path.as_ref().display()
        ))
    })?;

    if metadata.len() > MAX_FILE_SIZE {
        return Err(Error::out_of_range(
            "file_size",
            &metadata.len().to_string(),
            "0",
            &MAX_FILE_SIZE.to_string(),
        ));
    }

    let xml_content = fs::read_to_string(&path)
        .map_err(Error::from)
        .map_err(|e| {
            e.with_context(&format!(
                "Failed to read catalog file: {}",
                path.as_ref().display()
            ))
        })?;

    let cleaned_content = remove_bom(&xml_content);

    if validate_xml {
        validate_catalog_xml_structure(cleaned_content).map_err(|e| {
            e.with_context(&format!(
                "XML validation failed for catalog file: {}",
                path.as_ref().display()
            ))
        })?;
    }

    parse_catalog_from_str(cleaned_content).map_err(|e| {
        e.with_context(&format!(
            "Failed to parse catalog file: {}",
            path.as_ref().display()
        ))
    })
}

/// Parse an OpenSCENARIO document from a string
///
/// This function uses quick-xml's serde integration to deserialize
/// XML into our Rust type system.
#[must_use = "parsing result should be handled"]
pub fn parse_from_str(xml: &str) -> Result<OpenScenario> {
    quick_xml::de::from_str(xml)
        .map_err(Error::from)
        .map_err(|e| e.with_context("Failed to parse OpenSCENARIO XML"))
}

/// Parse an OpenSCENARIO document from a file
///
/// Reads file into memory and then parses it as a string.
#[must_use = "parsing result should be handled"]
pub fn parse_from_file<P: AsRef<Path>>(path: P) -> Result<OpenScenario> {
    parse_from_file_internal(path, false)
}

/// Serialize an OpenSCENARIO document to XML string
///
/// This function uses quick-xml's serde integration to serialize
/// our Rust types back to XML format.
#[must_use = "serialization result should be handled"]
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
#[must_use = "serialization result should be handled"]
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
        return Err(Error::invalid_xml("XML document is empty"));
    }

    if !trimmed.starts_with("<?xml") && !trimmed.starts_with('<') {
        return Err(Error::invalid_xml(
            "XML document must start with XML declaration or root element",
        ));
    }

    if !trimmed.contains("OpenSCENARIO") {
        return Err(Error::invalid_xml(
            "Document does not appear to contain OpenSCENARIO root element",
        ));
    }

    Ok(())
}

/// Parse with validation
///
/// Validates the XML structure before attempting to parse it.
#[must_use = "parsing result should be handled"]
pub fn parse_from_str_validated(xml: &str) -> Result<OpenScenario> {
    validate_xml_structure(xml)?;
    parse_from_str(xml)
}

/// Parse file with validation
///
/// Validates the XML structure before attempting to parse it.
#[must_use = "parsing result should be handled"]
pub fn parse_from_file_validated<P: AsRef<Path>>(path: P) -> Result<OpenScenario> {
    parse_from_file_internal(path, true)
}

// Catalog parsing functions

/// Parse a catalog file from XML string
///
/// This function uses quick-xml's serde integration to deserialize
/// catalog XML into our catalog file structure.
#[must_use = "parsing result should be handled"]
pub fn parse_catalog_from_str(xml: &str) -> Result<CatalogFile> {
    quick_xml::de::from_str(xml)
        .map_err(Error::from)
        .map_err(|e| e.with_context("Failed to parse catalog XML"))
}

/// Parse a catalog file from a file path
///
/// Reads the catalog file into memory and then parses it as a string.
#[must_use = "parsing result should be handled"]
pub fn parse_catalog_from_file<P: AsRef<Path>>(path: P) -> Result<CatalogFile> {
    parse_catalog_from_file_internal(path, false)
}

/// Validate catalog XML structure before parsing
///
/// This function performs basic XML structure validation specific to catalog files.
pub fn validate_catalog_xml_structure(xml: &str) -> Result<()> {
    let trimmed = xml.trim();

    if trimmed.is_empty() {
        return Err(Error::invalid_xml("Catalog XML document is empty"));
    }

    if !trimmed.starts_with("<?xml") && !trimmed.starts_with('<') {
        return Err(Error::invalid_xml(
            "Catalog XML document must start with XML declaration or root element",
        ));
    }

    if !trimmed.contains("OpenSCENARIO") {
        return Err(Error::invalid_xml(
            "Document does not appear to contain OpenSCENARIO root element",
        ));
    }

    if !trimmed.contains("Catalog") {
        return Err(Error::invalid_xml(
            "Document does not appear to contain Catalog element",
        ));
    }

    Ok(())
}

/// Parse catalog with validation
///
/// Validates the XML structure before attempting to parse it.
#[must_use = "parsing result should be handled"]
pub fn parse_catalog_from_str_validated(xml: &str) -> Result<CatalogFile> {
    validate_catalog_xml_structure(xml)?;
    parse_catalog_from_str(xml)
}

/// Parse catalog file with validation
///
/// Validates the XML structure before attempting to parse it.
#[must_use = "parsing result should be handled"]
pub fn parse_catalog_from_file_validated<P: AsRef<Path>>(path: P) -> Result<CatalogFile> {
    parse_catalog_from_file_internal(path, true)
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
        let catalog = CatalogFile::new(
            "TestCatalog".to_string(),
            "TestAuthor".to_string(),
            "Test catalog file".to_string(),
        );

        let xml = serialize_catalog_to_string(&catalog).unwrap();
        assert!(xml.contains("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        assert!(xml.contains("OpenSCENARIO"));
        assert!(xml.contains("Catalog"));
    }
}
