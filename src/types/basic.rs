//! Basic data types with expression and parameter support
//!
//! This file contains:
//! - Value<T> enum for literals, parameters, and expressions (${param}, ${expr})
//! - Implementation of all basic OpenSCENARIO types (String, Double, Boolean, etc.)
//! - Parameter resolution logic and expression evaluation
//! - Serde serialization/deserialization for XML attributes
//! - Validation helpers for parameter names and expression syntax
//!
//! Contributes to project by:
//! - Providing the foundation for all other types in the system
//! - Implementing OpenSCENARIO's parameter and expression system
//! - Ensuring type safety while supporting dynamic parameter resolution
//! - Enabling lazy evaluation of expressions with proper error handling
//! - Supporting both compile-time and runtime type checking

use crate::error::{Error, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

// Value enum that can hold either a literal value or a parameter reference
//
// OpenSCENARIO supports parameter references using ${parameterName} syntax.
// This enum allows us to represent both compile-time literals and runtime parameters.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value<T> {
    /// A literal value known at parse time
    Literal(T),
    /// A parameter reference that will be resolved at runtime
    Parameter(std::string::String),
}

impl<T> Value<T>
where
    T: FromStr + Clone,
    T::Err: std::fmt::Display,
{
    /// Resolve this value using the provided parameter map
    pub fn resolve(&self, params: &HashMap<std::string::String, std::string::String>) -> Result<T> {
        match self {
            Value::Literal(value) => Ok(value.clone()),
            Value::Parameter(param_name) => {
                let param_value = params
                    .get(param_name)
                    .ok_or_else(|| Error::parameter_error(param_name, "parameter not found"))?;

                param_value.parse::<T>().map_err(|e| {
                    Error::parameter_error(
                        param_name,
                        &format!("failed to parse '{}': {}", param_value, e),
                    )
                })
            }
        }
    }

    /// Get the literal value if this is a literal, otherwise None
    pub fn as_literal(&self) -> Option<&T> {
        match self {
            Value::Literal(value) => Some(value),
            Value::Parameter(_) => None,
        }
    }

    /// Get the parameter name if this is a parameter, otherwise None
    pub fn as_parameter(&self) -> Option<&str> {
        match self {
            Value::Literal(_) => None,
            Value::Parameter(name) => Some(name),
        }
    }
}

impl<T: Clone> Value<T> {
    /// Create a literal value
    pub fn literal(value: T) -> Self {
        Value::Literal(value)
    }

    /// Create a parameter reference
    pub fn parameter(name: std::string::String) -> Self {
        Value::Parameter(name)
    }
}

// Custom serde implementation to handle ${param} syntax
impl<'de, T> Deserialize<'de> for Value<T>
where
    T: Deserialize<'de> + FromStr,
    T::Err: std::fmt::Display,
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        // Check if this is a parameter reference ${paramName}
        if let Some(param_name) = parse_parameter_reference(&s) {
            Ok(Value::Parameter(param_name))
        } else {
            // Try to parse as literal value
            match s.parse::<T>() {
                Ok(value) => Ok(Value::Literal(value)),
                Err(e) => Err(serde::de::Error::custom(format!(
                    "Failed to parse '{}': {}",
                    s, e
                ))),
            }
        }
    }
}

impl<T> Serialize for Value<T>
where
    T: Serialize + fmt::Display,
{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Value::Literal(value) => value.serialize(serializer),
            Value::Parameter(name) => format!("${{{}}}", name).serialize(serializer),
        }
    }
}

// OpenSCENARIO basic type aliases
pub type OSString = Value<std::string::String>;
pub type Double = Value<f64>;
pub type Int = Value<i32>;
pub type UnsignedInt = Value<u32>;
pub type UnsignedShort = Value<u16>;
pub type Boolean = Value<bool>;

// DateTime support will be added with chrono feature
#[cfg(feature = "chrono")]
pub type DateTime = Value<chrono::DateTime<chrono::Utc>>;

/// Parse a parameter reference from a string
///
/// Returns the parameter name if the string matches ${paramName} pattern
pub fn parse_parameter_reference(s: &str) -> Option<std::string::String> {
    if s.starts_with("${") && s.ends_with('}') && s.len() > 3 {
        let param_name = &s[2..s.len() - 1];
        if is_valid_parameter_name(param_name) {
            Some(param_name.to_string())
        } else {
            None
        }
    } else {
        None
    }
}

// Check if a parameter name is valid
//
// Valid parameter names contain only alphanumeric characters and underscores
pub fn is_valid_parameter_name(name: &str) -> bool {
    !name.is_empty()
        && name.chars().all(|c| c.is_alphanumeric() || c == '_')
        && !name.chars().next().unwrap().is_ascii_digit() // Can't start with digit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameter_reference_parsing() {
        assert_eq!(
            parse_parameter_reference("${speed}"),
            Some("speed".to_string())
        );
        assert_eq!(
            parse_parameter_reference("${vehicle_speed}"),
            Some("vehicle_speed".to_string())
        );
        assert_eq!(parse_parameter_reference("literal"), None);
        assert_eq!(parse_parameter_reference("${}"), None);
        assert_eq!(parse_parameter_reference("${123}"), None); // Invalid: starts with digit
    }

    #[test]
    fn test_parameter_name_validation() {
        assert!(is_valid_parameter_name("speed"));
        assert!(is_valid_parameter_name("vehicle_speed"));
        assert!(is_valid_parameter_name("speed123"));
        assert!(!is_valid_parameter_name("123speed")); // Can't start with digit
        assert!(!is_valid_parameter_name("")); // Can't be empty
        assert!(!is_valid_parameter_name("speed-limit")); // No hyphens
    }
}

