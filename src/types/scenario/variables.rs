//! Variable declarations. Same shape as parameter declarations, but variables may
//! change during the run where parameters are fixed once resolved.

use crate::types::basic::{OSString, Value};
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
    pub variable_type: Value<ParameterType>,
    #[serde(rename = "@value")]
    pub value: OSString,
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
                variable_type: Value::Literal(var_type),
                value: OSString::literal(value),
            }],
        }
    }

    /// Add a variable declaration
    pub fn add_variable(&mut self, name: String, var_type: ParameterType, value: String) {
        self.variable_declarations.push(VariableDeclaration {
            name: OSString::literal(name),
            variable_type: Value::Literal(var_type),
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
            variable_type: Value::Literal(var_type),
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

    /// Each typed helper states its own `@variableType` and writes the value in the text form
    /// the `@value` attribute carries.
    #[test]
    fn test_variable_declaration_creation() {
        let cases = [
            (
                VariableDeclaration::string_variable("name".to_string(), "value".to_string()),
                ParameterType::String,
                "value",
            ),
            (
                VariableDeclaration::int_variable("count".to_string(), 42),
                ParameterType::Int,
                "42",
            ),
            (
                VariableDeclaration::double_variable("ratio".to_string(), 1.5),
                ParameterType::Double,
                "1.5",
            ),
            (
                VariableDeclaration::bool_variable("flag".to_string(), true),
                ParameterType::Boolean,
                "true",
            ),
        ];
        for (decl, var_type, value) in cases {
            assert_eq!(
                decl.variable_type,
                Value::Literal(var_type.clone()),
                "{var_type:?}"
            );
            assert_eq!(
                decl.value.as_literal(),
                Some(&value.to_string()),
                "{var_type:?}"
            );
        }
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
            Value::Literal(ParameterType::String)
        );
        assert_eq!(
            decls.variable_declarations[1].variable_type,
            Value::Literal(ParameterType::Int)
        );
    }
}
