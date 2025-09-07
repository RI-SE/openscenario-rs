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

    #[test]
    fn test_parameter_declaration_creation() {
        // Test basic creation
        let param = ParameterDeclaration::new(
            "MaxSpeed".to_string(),
            ParameterType::Double,
            "60.0".to_string(),
        );
        
        assert_eq!(param.name.as_literal().unwrap(), "MaxSpeed");
        assert_eq!(param.parameter_type, ParameterType::Double);
        assert_eq!(param.value.as_literal().unwrap(), "60.0");
        assert!(!param.has_constraints());
    }

    #[test]
    fn test_parameter_declaration_with_constraints() {
        let constraints = ValueConstraintGroup::new(vec![
            ValueConstraint::greater_than("0.0".to_string()),
            ValueConstraint::less_than("100.0".to_string()),
        ]);

        let param = ParameterDeclaration::with_constraints(
            "Speed".to_string(),
            ParameterType::Double,
            "30.0".to_string(),
            constraints,
        );

        assert!(param.has_constraints());
        let constraint_group = param.constraint_group.as_ref().unwrap();
        assert_eq!(constraint_group.value_constraints.len(), 2);
        assert_eq!(constraint_group.value_constraints[0].rule, Rule::GreaterThan);
        assert_eq!(constraint_group.value_constraints[1].rule, Rule::LessThan);
    }

    #[test]
    fn test_parameter_declaration_add_constraint() {
        let mut param = ParameterDeclaration::new(
            "Age".to_string(),
            ParameterType::Int,
            "25".to_string(),
        );

        // Initially no constraints
        assert!(!param.has_constraints());

        // Add first constraint
        param.add_constraint(ValueConstraint::greater_than("0".to_string()));
        assert!(param.has_constraints());

        // Add second constraint
        param.add_constraint(ValueConstraint::less_than("120".to_string()));
        
        let constraints = param.constraint_group.as_ref().unwrap();
        assert_eq!(constraints.value_constraints.len(), 2);
    }

    #[test]
    fn test_value_constraint_helpers() {
        let eq_constraint = ValueConstraint::equal_to("test".to_string());
        assert_eq!(eq_constraint.rule, Rule::EqualTo);
        assert_eq!(eq_constraint.value.as_literal().unwrap(), "test");

        let gt_constraint = ValueConstraint::greater_than("10".to_string());
        assert_eq!(gt_constraint.rule, Rule::GreaterThan);

        let lt_constraint = ValueConstraint::less_than("50".to_string());
        assert_eq!(lt_constraint.rule, Rule::LessThan);
    }

    #[test]
    fn test_range_creation() {
        let range = Range::new(0.0, 100.0);
        assert_eq!(range.lower_limit.as_literal().unwrap(), &0.0);
        assert_eq!(range.upper_limit.as_literal().unwrap(), &100.0);

        let default_range = Range::default();
        assert_eq!(default_range.lower_limit.as_literal().unwrap(), &0.0);
        assert_eq!(default_range.upper_limit.as_literal().unwrap(), &100.0);
    }

    #[test]
    fn test_parameter_declarations_container() {
        let mut declarations = ParameterDeclarations::default();
        assert!(declarations.parameter_declarations.is_empty());

        declarations.parameter_declarations.push(ParameterDeclaration::new(
            "Speed".to_string(),
            ParameterType::Double,
            "30.0".to_string(),
        ));

        declarations.parameter_declarations.push(ParameterDeclaration::new(
            "VehicleName".to_string(),
            ParameterType::String,
            "Ego".to_string(),
        ));

        assert_eq!(declarations.parameter_declarations.len(), 2);
        assert_eq!(declarations.parameter_declarations[0].parameter_type, ParameterType::Double);
        assert_eq!(declarations.parameter_declarations[1].parameter_type, ParameterType::String);
    }
}



// Data Container Types for Scenario Structure

use crate::types::enums::{ParameterType, Rule};

/// Parameter declarations container
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParameterDeclarations {
    #[serde(rename = "ParameterDeclaration")]
    pub parameter_declarations: Vec<ParameterDeclaration>,
}

/// Individual parameter declaration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParameterDeclaration {
    #[serde(rename = "@name")]
    pub name: OSString,
    #[serde(rename = "@parameterType")]
    pub parameter_type: ParameterType,
    #[serde(rename = "@value")]
    pub value: OSString,
    #[serde(rename = "ConstraintGroup", skip_serializing_if = "Option::is_none")]
    pub constraint_group: Option<ValueConstraintGroup>,
}

impl Default for ParameterDeclarations {
    fn default() -> Self {
        Self { parameter_declarations: Vec::new() }
    }
}

impl Default for ParameterDeclaration {
    fn default() -> Self {
        Self {
            name: OSString::literal("DefaultParameter".to_string()),
            parameter_type: ParameterType::String,
            value: OSString::literal("".to_string()),
            constraint_group: None,
        }
    }
}

/// Parameter constraints container
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ValueConstraintGroup {
    #[serde(rename = "ValueConstraint")]
    pub value_constraints: Vec<ValueConstraint>,
}

/// Individual parameter value constraint
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ValueConstraint {
    #[serde(rename = "@rule")]
    pub rule: Rule,
    #[serde(rename = "@value")]
    pub value: OSString,
}

/// Value range specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Range {
    #[serde(rename = "@lowerLimit")]
    pub lower_limit: Double,
    #[serde(rename = "@upperLimit")]
    pub upper_limit: Double,
}

impl Default for ValueConstraintGroup {
    fn default() -> Self {
        Self {
            value_constraints: Vec::new(),
        }
    }
}

impl Default for ValueConstraint {
    fn default() -> Self {
        Self {
            rule: Rule::EqualTo,
            value: OSString::literal("0".to_string()),
        }
    }
}

impl Default for Range {
    fn default() -> Self {
        Self {
            lower_limit: Double::literal(0.0),
            upper_limit: Double::literal(100.0),
        }
    }
}

// Helper methods for ParameterDeclaration
impl ParameterDeclaration {
    /// Create a new parameter declaration with the given name, type, and value
    pub fn new(name: String, parameter_type: ParameterType, value: String) -> Self {
        Self {
            name: OSString::literal(name),
            parameter_type,
            value: OSString::literal(value),
            constraint_group: None,
        }
    }

    /// Create a parameter declaration with constraints
    pub fn with_constraints(
        name: String,
        parameter_type: ParameterType,
        value: String,
        constraints: ValueConstraintGroup,
    ) -> Self {
        Self {
            name: OSString::literal(name),
            parameter_type,
            value: OSString::literal(value),
            constraint_group: Some(constraints),
        }
    }

    /// Add a constraint to this parameter declaration
    pub fn add_constraint(&mut self, constraint: ValueConstraint) {
        if let Some(ref mut group) = self.constraint_group {
            group.value_constraints.push(constraint);
        } else {
            self.constraint_group = Some(ValueConstraintGroup {
                value_constraints: vec![constraint],
            });
        }
    }

    /// Check if the parameter has constraints
    pub fn has_constraints(&self) -> bool {
        self.constraint_group.is_some()
    }
}

// Helper methods for ValueConstraintGroup
impl ValueConstraintGroup {
    /// Create a new constraint group with the given constraints
    pub fn new(constraints: Vec<ValueConstraint>) -> Self {
        Self {
            value_constraints: constraints,
        }
    }

    /// Add a constraint to the group
    pub fn add_constraint(&mut self, constraint: ValueConstraint) {
        self.value_constraints.push(constraint);
    }
}

// Helper methods for ValueConstraint
impl ValueConstraint {
    /// Create a new value constraint
    pub fn new(rule: Rule, value: String) -> Self {
        Self {
            rule,
            value: OSString::literal(value),
        }
    }

    /// Create an equality constraint
    pub fn equal_to(value: String) -> Self {
        Self::new(Rule::EqualTo, value)
    }

    /// Create a greater than constraint
    pub fn greater_than(value: String) -> Self {
        Self::new(Rule::GreaterThan, value)
    }

    /// Create a less than constraint
    pub fn less_than(value: String) -> Self {
        Self::new(Rule::LessThan, value)
    }
}

// Helper methods for Range
impl Range {
    /// Create a new range with the given limits
    pub fn new(lower: f64, upper: f64) -> Self {
        Self {
            lower_limit: Double::literal(lower),
            upper_limit: Double::literal(upper),
        }
    }
}

