//! Error types and error handling for the OpenSCENARIO library

use thiserror::Error;

/// Main error type for the OpenSCENARIO library
#[derive(Error, Debug)]
pub enum Error {
    /// XML parsing failures
    #[error("XML parsing error: {0}")]
    XmlParseError(#[from] quick_xml::DeError),

    /// File I/O failures
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Schema validation failures
    #[error("Validation error in field '{field}': {message}")]
    ValidationError { field: String, message: String },

    /// Parameter resolution failures
    #[error("Parameter error for '{param}': {value}")]
    ParameterError { param: String, value: String },

    /// Missing entity reference
    #[error("Entity not found: {entity}")]
    EntityNotFound { entity: String },

    /// Missing catalog reference
    #[error("Catalog entry not found: {entry} in catalog {catalog}")]
    CatalogNotFound { catalog: String, entry: String },

    /// Catalog system errors
    #[error("Catalog error: {0}")]
    CatalogError(String),
}

impl Error {
    /// Add context to an error
    pub fn with_context(self, context: &str) -> Self {
        match self {
            Error::ValidationError { field, message } => Error::ValidationError {
                field,
                message: format!("{}: {}", context, message),
            },
            other => other,
        }
    }

    /// Create a parsing error with location information
    pub fn parsing_error(msg: &str, line: usize, col: usize) -> Self {
        Error::ValidationError {
            field: format!("line {}, column {}", line, col),
            message: msg.to_string(),
        }
    }

    /// Create a parameter error
    pub fn parameter_error(param: &str, value: &str) -> Self {
        Error::ParameterError {
            param: param.to_string(),
            value: value.to_string(),
        }
    }

    /// Create a validation error
    pub fn validation_error(field: &str, message: &str) -> Self {
        Error::ValidationError {
            field: field.to_string(),
            message: message.to_string(),
        }
    }

    /// Create a catalog error
    pub fn catalog_error(message: &str) -> Self {
        Error::CatalogError(message.to_string())
    }
}

/// Result type alias for the OpenSCENARIO library
pub type Result<T> = std::result::Result<T, Error>;
