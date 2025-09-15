//! Variable declaration types for OpenSCENARIO
//!
//! This module contains variable declaration types that follow the same pattern
//! as parameter declarations but for runtime variables rather than compile-time parameters.

use crate::types::basic::OSString;
use crate::types::enums::ParameterType;
use serde::{Deserialize, Serialize};

/// Variable declarations container
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct VariableDeclarations {
    #[serde(rename = "VariableDeclaration", default)]
    pub variable_declarations: Vec<VariableDeclaration>,
}

/// Individual variable declaration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VariableDeclaration {
    #[serde(rename = "@name")]
    pub name: OSString,
    #[serde(rename = "@variableType")]
    pub variable_type: ParameterType,
    #[serde(rename = "@value")]
    pub value: OSString,
}

impl Default for VariableDeclaration {
    fn default() -> Self {
        Self {
            name: OSString::literal("DefaultVariable".to_string()),
            variable_type: ParameterType::String,
            value: OSString::literal("".to_string()),
        }
    }
}

impl VariableDeclarations {
    /// Create empty variable declarations
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with single variable
    pub fn with_variable(name: String, var_type: ParameterType, value: String) -> Self {
        Self {
            variable_declarations: vec![VariableDeclaration {
                name: OSString::literal(name),
                variable_type: var_type,
                value: OSString::literal(value),
            }],
        }
    }

    /// Add a variable declaration
    pub fn add_variable(&mut self, name: String, var_type: ParameterType, value: String) {
        self.variable_declarations.push(VariableDeclaration {
            name: OSString::literal(name),
            variable_type: var_type,
            value: OSString::literal(value),
        });
    }

    /// Check if declarations is empty
    pub fn is_empty(&self) -> bool {
        self.variable_declarations.is_empty()
    }

    /// Get number of variable declarations
    pub fn len(&self) -> usize {
        self.variable_declarations.len()
    }
}

impl VariableDeclaration {
    /// Create new variable declaration
    pub fn new(name: String, var_type: ParameterType, value: String) -> Self {
        Self {
            name: OSString::literal(name),
            variable_type: var_type,
            value: OSString::literal(value),
        }
    }

    /// Create string variable declaration
    pub fn string_variable(name: String, value: String) -> Self {
        Self::new(name, ParameterType::String, value)
    }

    /// Create integer variable declaration
    pub fn int_variable(name: String, value: i32) -> Self {
        Self::new(name, ParameterType::Int, value.to_string())
    }

    /// Create double variable declaration
    pub fn double_variable(name: String, value: f64) -> Self {
        Self::new(name, ParameterType::Double, value.to_string())
    }

    /// Create boolean variable declaration
    pub fn bool_variable(name: String, value: bool) -> Self {
        Self::new(name, ParameterType::Boolean, value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_declarations_creation() {
        let decls = VariableDeclarations::new();
        assert!(decls.is_empty());
        assert_eq!(decls.len(), 0);

        let single_var = VariableDeclarations::with_variable(
            "test_var".to_string(),
            ParameterType::String,
            "test_value".to_string(),
        );
        assert!(!single_var.is_empty());
        assert_eq!(single_var.len(), 1);
    }

    #[test]
    fn test_variable_declaration_creation() {
        let string_var =
            VariableDeclaration::string_variable("name".to_string(), "value".to_string());
        assert_eq!(string_var.variable_type, ParameterType::String);

        let int_var = VariableDeclaration::int_variable("count".to_string(), 42);
        assert_eq!(int_var.variable_type, ParameterType::Int);

        let bool_var = VariableDeclaration::bool_variable("flag".to_string(), true);
        assert_eq!(bool_var.variable_type, ParameterType::Boolean);
    }

    #[test]
    fn test_add_variable() {
        let mut decls = VariableDeclarations::new();
        decls.add_variable(
            "var1".to_string(),
            ParameterType::String,
            "value1".to_string(),
        );
        decls.add_variable("var2".to_string(), ParameterType::Int, "42".to_string());

        assert_eq!(decls.len(), 2);
        assert_eq!(
            decls.variable_declarations[0].variable_type,
            ParameterType::String
        );
        assert_eq!(
            decls.variable_declarations[1].variable_type,
            ParameterType::Int
        );
    }
}
