//! XSD schema validation for OpenSCENARIO documents.
//!
//! Available with the `validation` cargo feature. This module owns the single copy of the
//! libxml-backed schema-validation logic used both by the `xosc-validate` binary (which
//! validates *input* files) and by out-of-tree harnesses that need to validate the crate's
//! own *serialized output* — hence the in-memory [`XsdValidator::validate_str`] entry point.
//!
//! The schema is parsed once when the validator is constructed and the resulting
//! [`SchemaValidationContext`] is reused for every document, so validating a corpus does not
//! re-parse the 2500-line XSD per file.
//!
//! ```rust,no_run
//! # #[cfg(feature = "validation")] {
//! use openscenario_rs::validation::XsdValidator;
//!
//! let mut validator = XsdValidator::from_schema_file("Schema/OpenSCENARIO.xsd")?;
//! let errors = validator.validate_str("<OpenSCENARIO/>")?;
//! assert!(!errors.is_empty());
//! # }
//! # Ok::<(), openscenario_rs::Error>(())
//! ```

use crate::error::{Error, Result};
use libxml::error::StructuredError;
use libxml::parser::Parser as XmlParser;
use libxml::schemas::{SchemaParserContext, SchemaValidationContext};
use libxml::tree::Document;
use std::fs;
use std::path::Path;

/// A single schema-validation diagnostic reported by libxml.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    /// 1-based line number in the validated document, if libxml reported one.
    pub line: Option<u32>,
    /// 1-based column number in the validated document, if libxml reported one.
    pub column: Option<u32>,
    /// The raw libxml message, trimmed.
    pub message: String,
    /// Coarse classification derived from the message text.
    pub error_type: String,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let location = match (self.line, self.column) {
            (Some(line), Some(col)) => format!("{}:{}", line, col),
            (Some(line), None) => line.to_string(),
            _ => "?".to_string(),
        };
        write!(
            f,
            "Line {}: {} ({})",
            location, self.message, self.error_type
        )
    }
}

/// Validates XML documents against an XSD schema.
///
/// Construct once, validate many: [`SchemaValidationContext::validate_document`] takes
/// `&mut self`, so the validating methods here also take `&mut self` rather than rebuilding
/// the context per call.
pub struct XsdValidator {
    schema_validation_context: SchemaValidationContext,
}

impl XsdValidator {
    /// Build a validator by reading an XSD schema from disk.
    pub fn from_schema_file<P: AsRef<Path>>(xsd_path: P) -> Result<Self> {
        let xsd_path = xsd_path.as_ref();
        if !xsd_path.exists() {
            return Err(Error::FileNotFound {
                path: xsd_path.display().to_string(),
            });
        }

        let xsd_content = fs::read_to_string(xsd_path).map_err(|e| Error::FileReadError {
            path: xsd_path.display().to_string(),
            reason: e.to_string(),
        })?;

        Self::from_schema_str(&xsd_content)
    }

    /// Build a validator from an in-memory XSD schema.
    pub fn from_schema_str(xsd_content: &str) -> Result<Self> {
        let mut schema_parser = SchemaParserContext::from_buffer(xsd_content);

        let schema_validation_context = SchemaValidationContext::from_parser(&mut schema_parser)
            .map_err(|e| Error::ValidationError {
                field: "schema".to_string(),
                message: format!("Failed to create validation context: {:?}", e),
            })?;

        Ok(Self {
            schema_validation_context,
        })
    }

    /// Validate an in-memory XML string.
    ///
    /// Returns the list of schema violations; an empty list means the document is valid.
    /// `Err` is reserved for the document not being well-formed XML at all.
    pub fn validate_str(&mut self, xml: &str) -> Result<Vec<ValidationError>> {
        let parser = XmlParser::default();
        let document = parser
            .parse_string(xml)
            .map_err(|e| Error::ValidationError {
                field: "xml".to_string(),
                message: format!("Failed to parse XML: {}", e),
            })?;

        Ok(self.validate_document(&document))
    }

    /// Convenience wrapper: read a file and validate its contents.
    pub fn validate_file<P: AsRef<Path>>(&mut self, path: P) -> Result<Vec<ValidationError>> {
        let path = path.as_ref();
        if !path.exists() {
            return Err(Error::FileNotFound {
                path: path.display().to_string(),
            });
        }

        let xml = fs::read_to_string(path).map_err(|e| Error::FileReadError {
            path: path.display().to_string(),
            reason: e.to_string(),
        })?;

        self.validate_str(&xml)
    }

    /// Validate an already-parsed libxml document.
    pub fn validate_document(&mut self, document: &Document) -> Vec<ValidationError> {
        match self.schema_validation_context.validate_document(document) {
            Ok(_) => Vec::new(),
            Err(errors) => convert_errors(errors),
        }
    }
}

/// Converts libxml structured errors into [`ValidationError`]s.
pub fn convert_errors(errors: Vec<StructuredError>) -> Vec<ValidationError> {
    errors
        .into_iter()
        .map(|err| {
            let message = err
                .message
                .as_ref()
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|| "Unknown error".to_string());
            ValidationError {
                line: err.line.map(|l| l as u32),
                column: err.col.map(|c| c as u32),
                error_type: classify_error_type(&message),
                message,
            }
        })
        .collect()
}

/// Coarse classification of a libxml validation message.
pub fn classify_error_type(message: &str) -> String {
    if message.contains("not expected") || message.contains("not allowed") {
        "ElementNotAllowed".to_string()
    } else if message.contains("missing") || message.contains("required") {
        "MissingRequired".to_string()
    } else if message.contains("invalid value") || message.contains("not valid") {
        "InvalidValue".to_string()
    } else if message.contains("type") {
        "TypeMismatch".to_string()
    } else {
        "ValidationError".to_string()
    }
}
