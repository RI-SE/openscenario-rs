//! Error types and error handling for the OpenSCENARIO library

use thiserror::Error;

/// Main error type for the OpenSCENARIO library
#[derive(Error, Debug)]
pub enum Error {
    // XML/Serialization
    /// XML deserialization failures. `context` names where the parse was invoked from -- the
    /// file being read, for instance -- and starts `None`; [`with_context`](Error::with_context)
    /// fills or extends it. Without a place to hold it, context passed to `with_context` had
    /// nothing to attach to and was silently dropped, so a syntax error from `parse_from_file`
    /// never named the file.
    #[error("XML parsing error: {source}{}", context.as_deref().map(|c| format!(" ({c})")).unwrap_or_default())]
    XmlParseError {
        #[source]
        source: quick_xml::DeError,
        context: Option<String>,
    },

    /// A typed-parse error found while reading the output of
    /// [`resolve_parameters`](crate::parser::resolve::resolve_parameters). That text is not
    /// always the text the caller wrote: a `CatalogReference` is replaced by the entry it
    /// names, which can hold more or fewer lines than the reference did. `location` names the
    /// line the failure maps back to instead -- of the document that was resolved, or, when the
    /// failure sits inside an inlined entry, of the catalog file that entry came from.
    #[error("XML parsing error: {source} ({location})")]
    ResolvedParseError {
        #[source]
        source: quick_xml::DeError,
        location: String,
    },

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

    /// Create a typed-parse error located in a resolved document, naming where `location` puts
    /// it. See [`ResolvedParseError`](Error::ResolvedParseError).
    pub fn resolved_parse_error(source: quick_xml::DeError, location: &str) -> Self {
        Error::ResolvedParseError {
            source,
            location: location.to_string(),
        }
    }

    /// Create an XML deserialization error with no context yet attached. See
    /// [`XmlParseError`](Error::XmlParseError).
    pub fn xml_parse_error(source: quick_xml::DeError) -> Self {
        Error::XmlParseError {
            source,
            context: None,
        }
    }

    /// Add context to an error.
    ///
    /// The match is exhaustive rather than falling back to a wildcard arm: a variant with
    /// nowhere to put context still gets a named arm that says so, so that adding a new variant
    /// later without deciding what happens to its context is a compile error, not a silent drop
    /// -- which is the defect this method used to have for [`XmlParseError`](Error::XmlParseError).
    pub fn with_context(mut self, context: &str) -> Self {
        match &mut self {
            Error::XmlParseError { context: ctx, .. } => {
                *ctx = Some(match ctx.take() {
                    Some(existing) => format!("{}, {}", existing, context),
                    None => context.to_string(),
                });
            }
            Error::ResolvedParseError {
                ref mut location, ..
            } => {
                *location = format!("{}, {}", location, context);
            }
            Error::ValidationError {
                ref mut message, ..
            } => {
                *message = format!("{}: {}", context, message);
            }
            Error::CatalogError(ref mut msg) => {
                *msg = format!("{}: {}", context, msg);
            }
            Error::ParameterError {
                ref mut message, ..
            } => {
                // `context` is a sentence of its own ("Failed to parse file: ..."), not a
                // clause continuing this one. Appending it keeps the parameter's own message
                // intact instead of interrupting it partway through, as a leading
                // `context: message` did: the reader could not tell where the generic
                // description ended and the specific one began.
                *message = format!("{} ({})", message, context);
            }
            Error::FileReadError { ref mut reason, .. } => {
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
            // No field to carry free-form context. Named explicitly, rather than caught by a
            // wildcard, so that a future variant is a compile error here until someone decides
            // what it should do.
            Error::XmlSerializeError(_)
            | Error::IoError(_)
            | Error::FileNotFound { .. }
            | Error::DirectoryNotFound { .. }
            | Error::EntityNotFound { .. }
            | Error::CatalogEntryNotFound { .. }
            | Error::CatalogNotFound { .. }
            | Error::MissingRequiredField { .. }
            | Error::TypeMismatch { .. }
            | Error::ParameterNotFound { .. }
            | Error::CircularDependency { .. }
            | Error::InvalidXmlStructure { .. }
            | Error::MalformedXml { .. }
            | Error::ConstraintViolation { .. }
            | Error::InconsistentState { .. } => {}
        }
        self
    }
}

/// Result type alias for the OpenSCENARIO library
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    /// Each constructor fills the variant its name promises, and `Display` renders the
    /// message a user reads from those fields (the `#[error(...)]` strings above).
    #[test]
    fn constructors_render_their_user_facing_message() {
        let cases: Vec<(Error, &str)> = vec![
            (
                Error::file_not_found("/path/to/file.xosc"),
                "File not found: /path/to/file.xosc",
            ),
            (
                Error::entity_not_found("ego", &["target".to_string()]),
                "Entity 'ego' not found",
            ),
            (
                Error::catalog_entry_not_found("vehicles", "car1"),
                "Catalog entry 'car1' not found in catalog 'vehicles'",
            ),
            (
                Error::catalog_not_found("vehicles", &["controllers".to_string()]),
                "Catalog 'vehicles' not found",
            ),
            (
                Error::validation_error("speed", "must be positive"),
                "Validation error in field 'speed': must be positive",
            ),
            (Error::missing_field("name"), "Missing required field: name"),
            (
                Error::invalid_value("speed", "-5", "must be positive"),
                "Invalid value for field 'speed': -5. must be positive",
            ),
            (
                Error::out_of_range("speed", "150", "0", "120"),
                "Value out of range for field 'speed': 150. Expected 0 to 120",
            ),
            (
                Error::type_mismatch("speed", "number", "string"),
                "Type mismatch for field 'speed': expected number, got string",
            ),
            (
                Error::parameter_error("speed", "division by zero"),
                "Parameter 'speed' error: division by zero",
            ),
            (
                Error::parameter_not_found("speed", &["distance".to_string()]),
                "Parameter 'speed' not found",
            ),
            (
                Error::circular_dependency("A -> B -> C -> A"),
                "Circular dependency detected: A -> B -> C -> A",
            ),
            (
                Error::invalid_xml("Document is empty"),
                "Invalid XML structure: Document is empty",
            ),
            (
                Error::malformed_xml(">", "<", "line 1"),
                "Malformed XML: expected >, found < at line 1",
            ),
            (
                Error::parse_error("abc", "not a number"),
                "Failed to parse 'abc': not a number",
            ),
            (
                Error::expression_error("1/0", "division by zero"),
                "Expression evaluation failed: 1/0 - division by zero",
            ),
            (
                Error::constraint_violation("speed cannot be negative"),
                "Constraint violation: speed cannot be negative",
            ),
        ];
        for (err, expected) in cases {
            assert_eq!(err.to_string(), expected, "{err:?}");
        }
    }

    /// The not-found constructors keep the candidate names for a caller to offer, even though
    /// `Display` does not print them.
    #[test]
    fn not_found_errors_keep_the_available_names() {
        let available = vec!["a".to_string(), "b".to_string()];
        assert!(matches!(
            Error::entity_not_found("x", &available),
            Error::EntityNotFound { available: ref got, .. } if *got == available
        ));
        assert!(matches!(
            Error::catalog_not_found("x", &available),
            Error::CatalogNotFound { available: ref got, .. } if *got == available
        ));
        assert!(matches!(
            Error::parameter_not_found("x", &available),
            Error::ParameterNotFound { available: ref got, .. } if *got == available
        ));
    }

    /// `with_context` per variant: prepended to the free-text field for most, appended in
    /// parentheses for `ParameterError`, accumulated for `XmlParseError`, and a no-op for a
    /// variant with nowhere to put it.
    #[test]
    fn test_with_context() {
        let de = || quick_xml::DeError::Custom("boom".to_string());
        let cases: Vec<(Error, &str)> = vec![
            (
                Error::validation_error("speed", "invalid").with_context("while parsing vehicle"),
                "Validation error in field 'speed': while parsing vehicle: invalid",
            ),
            (
                Error::catalog_error("no entry").with_context("ctx"),
                "Catalog error: ctx: no entry",
            ),
            (
                Error::file_read_error("a.xosc", "denied").with_context("ctx"),
                "Cannot read file a.xosc: ctx: denied",
            ),
            (
                Error::parse_error("abc", "not a number").with_context("ctx"),
                "Failed to parse 'abc': ctx: not a number",
            ),
            (
                Error::expression_error("1/0", "division by zero").with_context("ctx"),
                "Expression evaluation failed: 1/0 - ctx: division by zero",
            ),
            (
                Error::invalid_value("speed", "-5", "must be positive").with_context("ctx"),
                "Invalid value for field 'speed': -5. ctx: must be positive",
            ),
            (
                Error::out_of_range("speed", "150", "0", "120").with_context("ctx"),
                "Value out of range for field 'speed': ctx: 150. Expected 0 to 120",
            ),
            (
                Error::parameter_error("p", "not declared").with_context("ctx"),
                "Parameter 'p' error: not declared (ctx)",
            ),
            (
                Error::xml_parse_error(de())
                    .with_context("first")
                    .with_context("second"),
                "XML parsing error: boom (first, second)",
            ),
            (
                Error::resolved_parse_error(de(), "line 3").with_context("ctx"),
                "XML parsing error: boom (line 3, ctx)",
            ),
            (
                Error::file_not_found("a.xosc").with_context("ignored"),
                "File not found: a.xosc",
            ),
        ];
        for (err, expected) in cases {
            assert_eq!(err.to_string(), expected, "{err:?}");
        }
    }
}
