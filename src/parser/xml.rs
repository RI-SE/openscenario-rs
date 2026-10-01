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
use serde::Deserialize;
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
    let xml_content = read_scenario_file(&path)?;
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

/// Read a scenario file into memory, refusing one larger than [`MAX_FILE_SIZE`].
fn read_scenario_file<P: AsRef<Path>>(path: P) -> Result<String> {
    read_limited(path.as_ref())
}

/// Read `path` as text, refusing a file larger than [`MAX_FILE_SIZE`]. Every failure names the path.
fn read_limited(path: &Path) -> Result<String> {
    let metadata = fs::metadata(path).map_err(|e| Error::file_read(path, &e))?;

    if metadata.len() > MAX_FILE_SIZE {
        return Err(Error::out_of_range(
            "file_size",
            &metadata.len().to_string(),
            "0",
            &MAX_FILE_SIZE.to_string(),
        )
        .with_context(&format!("File too large: {}", path.display())));
    }

    fs::read_to_string(path).map_err(|e| Error::file_read(path, &e))
}

/// Internal helper to parse catalog from file
fn parse_catalog_from_file_internal<P: AsRef<Path>>(
    path: P,
    validate_xml: bool,
) -> Result<CatalogFile> {
    let xml_content = read_limited(path.as_ref())?;

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
        .map_err(Error::xml_parse_error)
        .map_err(|e| e.with_context("Failed to parse OpenSCENARIO XML"))
}

/// Parse an OpenSCENARIO document from a file
///
/// Reads file into memory and then parses it as a string.
#[must_use = "parsing result should be handled"]
pub fn parse_from_file<P: AsRef<Path>>(path: P) -> Result<OpenScenario> {
    parse_from_file_internal(path, false)
}

/// Parse an OpenSCENARIO document from a string with its parameter references and catalog
/// references resolved.
///
/// The document is first passed through [`resolve_parameters`](super::resolve::resolve_parameters),
/// which replaces every `$name` and `${expression}` attribute value by its value under the
/// document's own `<ParameterDeclarations>` and every `<CatalogReference>` by the catalog entry
/// it names. The result is then parsed as [`parse_from_str`] would. A string has no location of
/// its own, so a relative catalog directory is taken relative to the current working directory;
/// [`parse_from_file_resolved`] takes it relative to the file instead.
#[must_use = "parsing result should be handled"]
pub fn parse_from_str_resolved(xml: &str) -> Result<OpenScenario> {
    parse_resolved(xml, Path::new(""))
}

/// Parse an OpenSCENARIO document from a file with its parameter references and catalog
/// references resolved, as [`parse_from_str_resolved`] does for a string. A relative catalog
/// directory is taken relative to the directory holding the file.
#[must_use = "parsing result should be handled"]
pub fn parse_from_file_resolved<P: AsRef<Path>>(path: P) -> Result<OpenScenario> {
    let xml_content = read_scenario_file(&path)?;
    let base_dir = path.as_ref().parent().unwrap_or(Path::new(""));
    parse_resolved(&xml_content, base_dir).map_err(|e| {
        e.with_context(&format!(
            "Failed to parse file: {}",
            path.as_ref().display()
        ))
    })
}

fn parse_resolved(xml: &str, base_dir: &Path) -> Result<OpenScenario> {
    let (resolved, map) = super::resolve::resolve_parameters_with_map(remove_bom(xml), base_dir)?;
    parse_resolved_str(&resolved, &map)
}

/// Parse the output of [`resolve_parameters_with_map`](super::resolve::resolve_parameters_with_map)
/// as [`parse_from_str`] does, except that a typed-parse error is reported against the line
/// `map` says it came from, rather than against its position in text the caller never wrote.
fn parse_resolved_str(resolved: &str, map: &super::resolve::LineMap) -> Result<OpenScenario> {
    let mut deserializer = quick_xml::de::Deserializer::from_str(resolved);
    OpenScenario::deserialize(&mut deserializer).map_err(|source| {
        // `error_position` only ever moves for an XML syntax error; the great majority of
        // typed-parse failures are semantic (`serde::de::Error::custom`, raised while
        // converting a well-formed attribute's text), which leaves it at 0. `buffer_position`
        // tracks the reader regardless of why deserialization stopped: the byte just past the
        // last event it read, which is the event that held the failing value.
        let offset = deserializer.get_ref().get_ref().buffer_position() as usize;
        let line = line_at(resolved, offset);
        Error::resolved_parse_error(source, &map.locate(line))
    })
}

/// The 1-based line of `text` containing byte offset `offset`.
fn line_at(text: &str, offset: usize) -> usize {
    text.as_bytes()[..offset.min(text.len())]
        .iter()
        .filter(|b| **b == b'\n')
        .count()
        + 1
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

    fs::write(&path, xml).map_err(|e| Error::file_write(path.as_ref(), &e))
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
        .map_err(Error::xml_parse_error)
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

    fs::write(&path, xml).map_err(|e| Error::file_write(path.as_ref(), &e))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Each structural check rejects its input with its own message, the one a caller sees
    /// before any typed parse runs.
    #[test]
    fn validate_xml_structure_names_the_check_that_failed() {
        let accepted = [
            r#"<?xml version="1.0"?><OpenSCENARIO></OpenSCENARIO>"#,
            r#"<OpenSCENARIO></OpenSCENARIO>"#,
        ];
        for xml in accepted {
            assert!(validate_xml_structure(xml).is_ok(), "{xml}");
        }
        let rejected = [
            ("", "XML document is empty"),
            ("   ", "XML document is empty"),
            (
                "This is not XML",
                "XML document must start with XML declaration or root element",
            ),
            (
                "<SomeOtherRoot></SomeOtherRoot>",
                "Document does not appear to contain OpenSCENARIO root element",
            ),
        ];
        for (xml, message) in rejected {
            let err = validate_xml_structure(xml).unwrap_err();
            assert_eq!(
                err.to_string(),
                format!("Invalid XML structure: {message}"),
                "{xml:?}"
            );
        }
    }

    /// As above for a catalog document, which must also hold a `Catalog` element.
    #[test]
    fn validate_catalog_xml_structure_names_the_check_that_failed() {
        let valid_xml = r#"<?xml version="1.0"?>
        <OpenSCENARIO>
            <FileHeader revMajor="1" revMinor="3" date="2024-01-01T00:00:00" author="Test" description="Test"/>
            <Catalog name="test">
            </Catalog>
        </OpenSCENARIO>"#;
        assert!(validate_catalog_xml_structure(valid_xml).is_ok());

        let rejected = [
            ("", "Catalog XML document is empty"),
            (
                "This is not XML",
                "Catalog XML document must start with XML declaration or root element",
            ),
            (
                "<Root><Catalog/></Root>",
                "Document does not appear to contain OpenSCENARIO root element",
            ),
            (
                r#"<?xml version="1.0"?><OpenSCENARIO><FileHeader/></OpenSCENARIO>"#,
                "Document does not appear to contain Catalog element",
            ),
        ];
        for (xml, message) in rejected {
            let err = validate_catalog_xml_structure(xml).unwrap_err();
            assert_eq!(
                err.to_string(),
                format!("Invalid XML structure: {message}"),
                "{xml:?}"
            );
        }
    }

    /// The `_validated` string entry points run the structural pass first: a document it
    /// rejects fails with the structural message, not the typed parser's error, and a document
    /// it accepts is parsed.
    #[test]
    fn str_validated_entry_points_run_the_structural_pass_before_parsing() {
        let err = parse_from_str_validated("<Root/>").unwrap_err();
        assert_eq!(
            err.to_string(),
            "Invalid XML structure: Document does not appear to contain OpenSCENARIO root element"
        );
        let err = parse_catalog_from_str_validated(
            r#"<OpenSCENARIO><FileHeader revMajor="1" revMinor="3" date="2024-01-01T00:00:00" author="a" description="d"/></OpenSCENARIO>"#,
        )
        .unwrap_err();
        assert_eq!(
            err.to_string(),
            "Invalid XML structure: Document does not appear to contain Catalog element"
        );

        let scenario =
            serialize_to_string(&crate::types::scenario::storyboard::test_scenario_document())
                .unwrap();
        let parsed = parse_from_str_validated(&scenario).unwrap();
        assert_eq!(
            parsed.file_header.author.as_literal().unwrap(),
            "Test Author"
        );
        let catalog = parse_catalog_from_str_validated(
            r#"<OpenSCENARIO><FileHeader revMajor="1" revMinor="3" date="2024-01-01T00:00:00" author="a" description="d"/><Catalog name="Vehicles"/></OpenSCENARIO>"#,
        )
        .unwrap();
        assert_eq!(catalog.catalog.name.as_literal().unwrap(), "Vehicles");
    }

    /// `serialize_to_file` and `serialize_catalog_to_file` write exactly what their `_to_string`
    /// counterparts return.
    #[test]
    fn serialize_to_file_writes_the_serialized_string() {
        let dir = tempfile::TempDir::new().unwrap();

        let scenario = crate::types::scenario::storyboard::test_scenario_document();
        let path = dir.path().join("scenario.xosc");
        serialize_to_file(&scenario, &path).unwrap();
        assert_eq!(
            fs::read_to_string(&path).unwrap(),
            serialize_to_string(&scenario).unwrap()
        );

        let catalog = parse_catalog_from_str(
            r#"<OpenSCENARIO><FileHeader revMajor="1" revMinor="3" date="2024-01-01T00:00:00" author="a" description="d"/><Catalog name="Vehicles"/></OpenSCENARIO>"#,
        )
        .unwrap();
        let path = dir.path().join("catalog.xosc");
        serialize_catalog_to_file(&catalog, &path).unwrap();
        assert_eq!(
            fs::read_to_string(&path).unwrap(),
            serialize_catalog_to_string(&catalog).unwrap()
        );
    }
}
