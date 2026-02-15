//! Error types and error handling for the OpenSCENARIO library

use thiserror::Error;

/// Main error type for the OpenSCENARIO library
#[derive(Error, Debug)]
pub enum Error {
    // XML/Serialization
    /// XML deserialization failures
    #[error("XML parsing error: {0}")]
    XmlParseError(#[from] quick_xml::DeError),

    /// XML serialization failures
    #[error("XML serialization error: {0}")]
    XmlSerializeError(#[from] quick_xml::SeError),

    // I/O
    /// File I/O failures
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    // File System Errors
    /// File not found at specified path
    #[error("File not found: {path}")]
    FileNotFound { path: String },

    /// Directory not found at specified path
    #[error("Directory not found: {path}")]
    DirectoryNotFound { path: String },

    /// Cannot read file
    #[error("Cannot read file {path}: {reason}")]
    FileReadError { path: String, reason: String },

    /// Cannot write file
    #[error("Cannot write file {path}: {reason}")]
    FileWriteError { path: String, reason: String },

    // Reference Errors
    /// Entity reference not found
    #[error("Entity '{entity}' not found")]
    EntityNotFound {
        entity: String,
        available: Vec<String>,
    },

    /// Catalog entry not found
    #[error("Catalog entry '{entry}' not found in catalog '{catalog}'")]
    CatalogEntryNotFound { catalog: String, entry: String },

    /// Catalog not found
    #[error("Catalog '{catalog}' not found")]
    CatalogNotFound {
        catalog: String,
        available: Vec<String>,
    },

    // Validation Errors
    /// Schema validation failures
    #[error("Validation error in field '{field}': {message}")]
    ValidationError { field: String, message: String },

    /// Missing required field
    #[error("Missing required field: {field}")]
    MissingRequiredField { field: String },

    /// Invalid value for field
    #[error("Invalid value for field '{field}': {value}. {hint}")]
    InvalidValue {
        field: String,
        value: String,
        hint: String,
    },

    /// Value out of expected range
    #[error("Value out of range for field '{field}': {value}. Expected {min} to {max}")]
    OutOfRange {
        field: String,
        value: String,
        min: String,
        max: String,
    },

    /// Type mismatch
    #[error("Type mismatch for field '{field}': expected {expected}, got {actual}")]
    TypeMismatch {
        field: String,
        expected: String,
        actual: String,
    },

    // Parameter Errors
    /// Parameter resolution failures
    #[error("Parameter '{param}' error: {message}")]
    ParameterError { param: String, message: String },

    /// Parameter not found
    #[error("Parameter '{param}' not found")]
    ParameterNotFound {
        param: String,
        available: Vec<String>,
    },

    /// Circular dependency detected
    #[error("Circular dependency detected: {cycle}")]
    CircularDependency { cycle: String },

    // XML/Structure Errors
    /// Invalid XML structure
    #[error("Invalid XML structure: {message}")]
    InvalidXmlStructure { message: String },

    /// Malformed XML with location context
    #[error("Malformed XML: expected {expected}, found {found} at {location}")]
    MalformedXml {
        expected: String,
        found: String,
        location: String,
    },

    // Catalog Errors (remaining generic cases)
    /// Generic catalog system error
    #[error("Catalog error: {0}")]
    CatalogError(String),

    /// XSD Choice Group parsing errors
    #[error("Choice group error: {message}")]
    ChoiceGroupError { message: String },

    // Parsing/Expression Errors
    /// Failed to parse input
    #[error("Failed to parse '{input}': {reason}")]
    ParseError { input: String, reason: String },

    /// Expression evaluation failed
    #[error("Expression evaluation failed: {expression} - {reason}")]
    ExpressionError { expression: String, reason: String },

    // Constraint Violations
    /// Constraint violation
    #[error("Constraint violation: {constraint}")]
    ConstraintViolation { constraint: String },

    /// Inconsistent state
    #[error("Inconsistent state: {message}")]
    InconsistentState { message: String },
}

impl Error {
    // File System Errors

    /// Create a file not found error
    pub fn file_not_found(path: &str) -> Self {
        Error::FileNotFound {
            path: path.to_string(),
        }
    }

    /// Create a directory not found error
    pub fn directory_not_found(path: &str) -> Self {
        Error::DirectoryNotFound {
            path: path.to_string(),
        }
    }

    /// Create a file read error
    pub fn file_read_error(path: &str, reason: &str) -> Self {
        Error::FileReadError {
            path: path.to_string(),
            reason: reason.to_string(),
        }
    }

    /// Create a file write error
    pub fn file_write_error(path: &str, reason: &str) -> Self {
        Error::FileWriteError {
            path: path.to_string(),
            reason: reason.to_string(),
        }
    }

    // Reference Errors

    /// Create an entity not found error
    pub fn entity_not_found(entity: &str, available: &[String]) -> Self {
        Error::EntityNotFound {
            entity: entity.to_string(),
            available: available.to_vec(),
        }
    }

    /// Create a catalog entry not found error
    pub fn catalog_entry_not_found(catalog: &str, entry: &str) -> Self {
        Error::CatalogEntryNotFound {
            catalog: catalog.to_string(),
            entry: entry.to_string(),
        }
    }

    /// Create a catalog not found error
    pub fn catalog_not_found(catalog: &str, available: &[String]) -> Self {
        Error::CatalogNotFound {
            catalog: catalog.to_string(),
            available: available.to_vec(),
        }
    }

    // Validation Errors

    /// Create a validation error
    pub fn validation_error(field: &str, message: &str) -> Self {
        Error::ValidationError {
            field: field.to_string(),
            message: message.to_string(),
        }
    }

    /// Create a missing required field error
    pub fn missing_field(field: &str) -> Self {
        Error::MissingRequiredField {
            field: field.to_string(),
        }
    }

    /// Create an invalid value error
    pub fn invalid_value(field: &str, value: &str, hint: &str) -> Self {
        Error::InvalidValue {
            field: field.to_string(),
            value: value.to_string(),
            hint: hint.to_string(),
        }
    }

    /// Create an out of range error
    pub fn out_of_range(field: &str, value: &str, min: &str, max: &str) -> Self {
        Error::OutOfRange {
            field: field.to_string(),
            value: value.to_string(),
            min: min.to_string(),
            max: max.to_string(),
        }
    }

    /// Create a type mismatch error
    pub fn type_mismatch(field: &str, expected: &str, actual: &str) -> Self {
        Error::TypeMismatch {
            field: field.to_string(),
            expected: expected.to_string(),
            actual: actual.to_string(),
        }
    }

    // Parameter Errors

    /// Create a parameter error
    pub fn parameter_error(param: &str, message: &str) -> Self {
        Error::ParameterError {
            param: param.to_string(),
            message: message.to_string(),
        }
    }

    /// Create a parameter not found error
    pub fn parameter_not_found(param: &str, available: &[String]) -> Self {
        Error::ParameterNotFound {
            param: param.to_string(),
            available: available.to_vec(),
        }
    }

    // XML/Structure Errors

    /// Create an invalid XML structure error
    pub fn invalid_xml(message: &str) -> Self {
        Error::InvalidXmlStructure {
            message: message.to_string(),
        }
    }

    /// Create a malformed XML error with location
    pub fn malformed_xml(expected: &str, found: &str, location: &str) -> Self {
        Error::MalformedXml {
            expected: expected.to_string(),
            found: found.to_string(),
            location: location.to_string(),
        }
    }

    /// Create a parsing error with location information
    pub fn parsing_error(msg: &str, line: usize, col: usize) -> Self {
        Error::ValidationError {
            field: format!("line {}, column {}", line, col),
            message: msg.to_string(),
        }
    }

    // Other Errors

    /// Create a circular dependency error
    pub fn circular_dependency(cycle: &str) -> Self {
        Error::CircularDependency {
            cycle: cycle.to_string(),
        }
    }

    /// Create a parse error
    pub fn parse_error(input: &str, reason: &str) -> Self {
        Error::ParseError {
            input: input.to_string(),
            reason: reason.to_string(),
        }
    }

    /// Create an expression error
    pub fn expression_error(expression: &str, reason: &str) -> Self {
        Error::ExpressionError {
            expression: expression.to_string(),
            reason: reason.to_string(),
        }
    }

    /// Create a constraint violation error
    pub fn constraint_violation(constraint: &str) -> Self {
        Error::ConstraintViolation {
            constraint: constraint.to_string(),
        }
    }

    /// Create a catalog error
    pub fn catalog_error(message: &str) -> Self {
        Error::CatalogError(message.to_string())
    }

    /// Create a choice group error
    pub fn choice_group_error(message: &str) -> Self {
        Error::ChoiceGroupError {
            message: message.to_string(),
        }
    }

    /// Add context to an error
    pub fn with_context(mut self, context: &str) -> Self {
        match &mut self {
            Error::ValidationError {
                ref mut message, ..
            } => {
                *message = format!("{}: {}", context, message);
            }
            Error::CatalogError(ref mut msg) => {
                *msg = format!("{}: {}", context, msg);
            }
            Error::ChoiceGroupError { ref mut message } => {
                *message = format!("{}: {}", context, message);
            }
            Error::ParameterError {
                ref mut message, ..
            } => {
                *message = format!("{}: {}", context, message);
            }
            Error::FileReadError { ref mut reason, .. } => {
                *reason = format!("{}: {}", context, reason);
            }
            Error::FileWriteError { ref mut reason, .. } => {
                *reason = format!("{}: {}", context, reason);
            }
            Error::ParseError { ref mut reason, .. } => {
                *reason = format!("{}: {}", context, reason);
            }
            Error::ExpressionError { ref mut reason, .. } => {
                *reason = format!("{}: {}", context, reason);
            }
            Error::InvalidValue { ref mut hint, .. } => {
                *hint = format!("{}: {}", context, hint);
            }
            Error::OutOfRange { ref mut value, .. } => {
                *value = format!("{}: {}", context, value);
            }
            _ => {}
        }
        self
    }
}

/// Result type alias for the OpenSCENARIO library
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = Error::file_not_found("/path/to/file.xosc");
        assert!(matches!(err, Error::FileNotFound { path } if path == "/path/to/file.xosc"));
    }

    #[test]
    fn test_entity_not_found() {
        let err = Error::entity_not_found("ego", &["target".to_string()]);
        match err {
            Error::EntityNotFound { entity, available } => {
                assert_eq!(entity, "ego");
                assert_eq!(available, vec!["target"]);
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_catalog_not_found() {
        let err = Error::catalog_not_found("vehicles", &["controllers".to_string()]);
        match err {
            Error::CatalogNotFound { catalog, available } => {
                assert_eq!(catalog, "vehicles");
                assert_eq!(available, vec!["controllers"]);
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_parameter_not_found() {
        let err = Error::parameter_not_found("speed", &["distance".to_string()]);
        match err {
            Error::ParameterNotFound { param, available } => {
                assert_eq!(param, "speed");
                assert_eq!(available, vec!["distance"]);
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_validation_error() {
        let err = Error::validation_error("speed", "must be positive");
        match err {
            Error::ValidationError { field, message } => {
                assert_eq!(field, "speed");
                assert_eq!(message, "must be positive");
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_missing_field() {
        let err = Error::missing_field("name");
        assert!(matches!(err, Error::MissingRequiredField { field } if field == "name"));
    }

    #[test]
    fn test_invalid_value() {
        let err = Error::invalid_value("speed", "-5", "must be positive");
        match err {
            Error::InvalidValue { field, value, hint } => {
                assert_eq!(field, "speed");
                assert_eq!(value, "-5");
                assert_eq!(hint, "must be positive");
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_out_of_range() {
        let err = Error::out_of_range("speed", "150", "0", "120");
        match err {
            Error::OutOfRange {
                field,
                value,
                min,
                max,
            } => {
                assert_eq!(field, "speed");
                assert_eq!(value, "150");
                assert_eq!(min, "0");
                assert_eq!(max, "120");
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_type_mismatch() {
        let err = Error::type_mismatch("speed", "number", "string");
        match err {
            Error::TypeMismatch {
                field,
                expected,
                actual,
            } => {
                assert_eq!(field, "speed");
                assert_eq!(expected, "number");
                assert_eq!(actual, "string");
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_parameter_error() {
        let err = Error::parameter_error("speed", "division by zero");
        match err {
            Error::ParameterError { param, message } => {
                assert_eq!(param, "speed");
                assert_eq!(message, "division by zero");
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_circular_dependency() {
        let err = Error::circular_dependency("A -> B -> C -> A");
        assert!(matches!(err, Error::CircularDependency { .. }));
    }

    #[test]
    fn test_invalid_xml() {
        let err = Error::invalid_xml("Document is empty");
        assert!(matches!(err, Error::InvalidXmlStructure { .. }));
    }

    #[test]
    fn test_malformed_xml() {
        let err = Error::malformed_xml(">", "<", "line 1");
        match err {
            Error::MalformedXml {
                expected,
                found,
                location,
            } => {
                assert_eq!(expected, ">");
                assert_eq!(found, "<");
                assert_eq!(location, "line 1");
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_parse_error() {
        let err = Error::parse_error("abc", "not a number");
        match err {
            Error::ParseError { input, reason } => {
                assert_eq!(input, "abc");
                assert_eq!(reason, "not a number");
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_expression_error() {
        let err = Error::expression_error("1/0", "division by zero");
        match err {
            Error::ExpressionError { expression, reason } => {
                assert_eq!(expression, "1/0");
                assert_eq!(reason, "division by zero");
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_constraint_violation() {
        let err = Error::constraint_violation("speed cannot be negative");
        assert!(matches!(err, Error::ConstraintViolation { .. }));
    }

    #[test]
    fn test_with_context() {
        let err = Error::validation_error("speed", "invalid").with_context("while parsing vehicle");
        match err {
            Error::ValidationError { message, .. } => {
                assert!(message.contains("while parsing vehicle"));
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_error_display() {
        let err = Error::entity_not_found("ego", &["target".to_string()]);
        let msg = format!("{}", err);
        assert!(msg.contains("ego"));
    }

    #[test]
    fn test_catalog_entry_not_found() {
        let err = Error::catalog_entry_not_found("vehicles", "car1");
        match err {
            Error::CatalogEntryNotFound { catalog, entry } => {
                assert_eq!(catalog, "vehicles");
                assert_eq!(entry, "car1");
            }
            _ => panic!("Wrong error type"),
        }
    }
}
