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

// Value enum that can hold either a literal value, a parameter reference, or an expression
//
// OpenSCENARIO supports parameter references using ${parameterName} syntax and 
// mathematical expressions using ${expression} syntax.
// This enum allows us to represent both compile-time literals and runtime parameters/expressions.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value<T> {
    /// A literal value known at parse time
    Literal(T),
    /// A parameter reference that will be resolved at runtime
    Parameter(std::string::String),
    /// A mathematical expression that will be evaluated at runtime
    Expression(std::string::String),
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
            Value::Expression(expr) => {
                // For now, we'll treat expressions as parameters that need to be resolved
                // In a full implementation, we would parse and evaluate the mathematical expression
                let resolved_expr = resolve_expression::<T>(expr, params)?;
                resolved_expr.parse::<T>().map_err(|e| {
                    Error::parameter_error(
                        expr,
                        &format!("failed to parse expression result '{}': {}", resolved_expr, e),
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
            Value::Expression(_) => None,
        }
    }

    /// Get the parameter name if this is a parameter, otherwise None
    pub fn as_parameter(&self) -> Option<&str> {
        match self {
            Value::Literal(_) => None,
            Value::Parameter(name) => Some(name),
            Value::Expression(_) => None,
        }
    }
    
    /// Get the expression if this is an expression, otherwise None
    pub fn as_expression(&self) -> Option<&str> {
        match self {
            Value::Literal(_) => None,
            Value::Parameter(_) => None,
            Value::Expression(expr) => Some(expr),
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
    
    /// Create an expression
    pub fn expression(expr: std::string::String) -> Self {
        Value::Expression(expr)
    }
}

// Custom serde implementation to handle ${param} and ${expression} syntax
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

        // Check if this is a parameter reference or expression ${content}
        if s.starts_with("${") && s.ends_with('}') && s.len() > 3 {
            let content = &s[2..s.len() - 1];
            // Check if it's a simple parameter (no operators)
            if is_valid_parameter_name(content) && !content.contains(|c| "+-*/%()".contains(c)) {
                Ok(Value::Parameter(content.to_string()))
            } else {
                // Treat as expression
                Ok(Value::Expression(content.to_string()))
            }
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
            Value::Expression(expr) => format!("${{{}}}", expr).serialize(serializer),
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

// TODO: Add any missing basic types needed for actions and conditions (Week 4)
// TODO: pub type DateTime = Value<chrono::DateTime<chrono::Utc>>; - already defined but may need chrono feature



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

/// Check if a string is an expression (contains mathematical operators)
pub fn is_expression(s: &str) -> bool {
    s.contains(|c| "+-*/%()".contains(c))
}

// Check if a parameter name is valid
//
// Valid parameter names contain only alphanumeric characters and underscores
pub fn is_valid_parameter_name(name: &str) -> bool {
    !name.is_empty()
        && name.chars().all(|c| c.is_alphanumeric() || c == '_')
        && !name.chars().next().unwrap().is_ascii_digit() // Can't start with digit
}

/// Resolve a simple expression by substituting parameters
/// 
/// This is a basic implementation that only handles parameter substitution
/// A full implementation would parse and evaluate mathematical expressions
fn resolve_expression<T: FromStr>(
    expr: &str, 
    params: &HashMap<std::string::String, std::string::String>
) -> Result<String> 
where 
    T::Err: std::fmt::Display 
{
    // For now, just substitute parameter references in the expression
    let mut result = expr.to_string();
    
    // Find and replace parameter references in the expression
    for (param_name, param_value) in params {
        let param_ref = format!("${{{}}}", param_name);
        if result.contains(&param_ref) {
            result = result.replace(&param_ref, param_value);
        }
    }
    
    // In a full implementation, we would parse and evaluate the expression here
    // For now, we'll assume the expression is already resolved or return it as-is
    Ok(result)
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
    
    #[test]
    fn test_value_creation() {
        let literal = Value::<f64>::literal(10.0);
        assert!(matches!(literal, Value::Literal(10.0)));
        
        let parameter = Value::<String>::parameter("speed".to_string());
        assert!(matches!(parameter, Value::Parameter(_)));
        
        let expression = Value::<String>::expression("speed + 10".to_string());
        assert!(matches!(expression, Value::Expression(_)));
    }
    
    #[test]
    fn test_value_resolution() {
        let mut params = HashMap::new();
        params.insert("speed".to_string(), "30.0".to_string());
        params.insert("acceleration".to_string(), "2.5".to_string());
        
        // Test literal resolution
        let literal = Value::<f64>::literal(10.0);
        assert_eq!(literal.resolve(&params).unwrap(), 10.0);
        
        // Test parameter resolution
        let parameter = Value::<f64>::parameter("speed".to_string());
        assert_eq!(parameter.resolve(&params).unwrap(), 30.0);
        
        // Test expression resolution (basic)
        let expression = Value::<String>::expression("speed".to_string());
        assert_eq!(expression.resolve(&params).unwrap(), "speed");
    }
}

