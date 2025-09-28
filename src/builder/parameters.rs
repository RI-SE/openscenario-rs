//! Parameter builder support for dynamic scenario values
//!
//! This module provides builders for working with OpenSCENARIO parameters,
//! enabling dynamic value substitution and parameterized scenarios.

use crate::builder::{BuilderError, BuilderResult};
use crate::types::{
    basic::{OSString, ParameterDeclarations, ParameterDeclaration, Value},
    enums::ParameterType,
};
use std::collections::HashMap;

/// Builder for parameter declarations
#[derive(Debug, Default)]
pub struct ParameterDeclarationsBuilder {
    parameters: Vec<ParameterDeclaration>,
}

impl ParameterDeclarationsBuilder {
    /// Create a new parameter declarations builder
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add a string parameter
    pub fn add_string_parameter(mut self, name: &str, default_value: &str) -> Self {
        self.parameters.push(ParameterDeclaration {
            name: OSString::literal(name.to_string()),
            parameter_type: ParameterType::String,
            value: OSString::literal(default_value.to_string()),
        });
        self
    }
    
    /// Add a double parameter
    pub fn add_double_parameter(mut self, name: &str, default_value: f64) -> Self {
        self.parameters.push(ParameterDeclaration {
            name: OSString::literal(name.to_string()),
            parameter_type: ParameterType::Double,
            value: OSString::literal(default_value.to_string()),
        });
        self
    }
    
    /// Add an integer parameter
    pub fn add_int_parameter(mut self, name: &str, default_value: i32) -> Self {
        self.parameters.push(ParameterDeclaration {
            name: OSString::literal(name.to_string()),
            parameter_type: ParameterType::Int,
            value: OSString::literal(default_value.to_string()),
        });
        self
    }
    
    /// Add a boolean parameter
    pub fn add_boolean_parameter(mut self, name: &str, default_value: bool) -> Self {
        self.parameters.push(ParameterDeclaration {
            name: OSString::literal(name.to_string()),
            parameter_type: ParameterType::Boolean,
            value: OSString::literal(default_value.to_string()),
        });
        self
    }
    
    /// Add a date/time parameter
    pub fn add_datetime_parameter(mut self, name: &str, default_value: &str) -> Self {
        self.parameters.push(ParameterDeclaration {
            name: OSString::literal(name.to_string()),
            parameter_type: ParameterType::DateTime,
            value: OSString::literal(default_value.to_string()),
        });
        self
    }
    
    /// Add an unsigned short parameter
    pub fn add_unsigned_short_parameter(mut self, name: &str, default_value: u16) -> Self {
        self.parameters.push(ParameterDeclaration {
            name: OSString::literal(name.to_string()),
            parameter_type: ParameterType::UnsignedShort,
            value: OSString::literal(default_value.to_string()),
        });
        self
    }
    
    /// Add an unsigned int parameter
    pub fn add_unsigned_int_parameter(mut self, name: &str, default_value: u32) -> Self {
        self.parameters.push(ParameterDeclaration {
            name: OSString::literal(name.to_string()),
            parameter_type: ParameterType::UnsignedInt,
            value: OSString::literal(default_value.to_string()),
        });
        self
    }
    
    /// Build the parameter declarations
    pub fn build(self) -> ParameterDeclarations {
        ParameterDeclarations {
            parameter_declarations: self.parameters,
        }
    }
    
    /// Get the number of parameters
    pub fn len(&self) -> usize {
        self.parameters.len()
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.parameters.is_empty()
    }
}

/// Builder for parameterized values
#[derive(Debug, Clone)]
pub struct ParameterizedValueBuilder<T> {
    value: Value<T>,
}

impl<T> ParameterizedValueBuilder<T> {
    /// Create a literal value
    pub fn literal(value: T) -> Self {
        Self {
            value: Value::Literal(value),
        }
    }
    
    /// Create a parameter reference
    pub fn parameter(parameter_name: &str) -> Self {
        Self {
            value: Value::Parameter(parameter_name.to_string()),
        }
    }
    
    /// Create an expression
    pub fn expression(expression: &str) -> Self {
        Self {
            value: Value::Expression(expression.to_string()),
        }
    }
    
    /// Build the value
    pub fn build(self) -> Value<T> {
        self.value
    }
}

/// Parameter context for resolving parameter references
#[derive(Debug, Default)]
pub struct ParameterContext {
    parameters: HashMap<String, String>,
}

impl ParameterContext {
    /// Create a new parameter context
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add a parameter value
    pub fn add_parameter(mut self, name: &str, value: &str) -> Self {
        self.parameters.insert(name.to_string(), value.to_string());
        self
    }
    
    /// Set multiple parameters from a map
    pub fn with_parameters(mut self, parameters: HashMap<String, String>) -> Self {
        self.parameters.extend(parameters);
        self
    }
    
    /// Get a parameter value
    pub fn get_parameter(&self, name: &str) -> Option<&str> {
        self.parameters.get(name).map(|s| s.as_str())
    }
    
    /// Resolve a parameter reference to its value
    pub fn resolve_parameter(&self, parameter_name: &str) -> BuilderResult<String> {
        self.parameters
            .get(parameter_name)
            .cloned()
            .ok_or_else(|| BuilderError::validation_error(&format!(
                "Parameter '{}' not found in context", parameter_name
            )))
    }
    
    /// Resolve a Value<T> using this context
    pub fn resolve_value<T>(&self, value: &Value<T>) -> BuilderResult<String>
    where
        T: ToString,
    {
        match value {
            Value::Literal(val) => Ok(val.to_string()),
            Value::Parameter(param_name) => self.resolve_parameter(param_name),
            Value::Expression(expr) => {
                // For now, return the expression as-is
                // In a full implementation, this would evaluate the expression
                Ok(expr.clone())
            }
        }
    }
    
    /// Get all parameters
    pub fn parameters(&self) -> &HashMap<String, String> {
        &self.parameters
    }
}

/// Helper trait for types that can be parameterized
pub trait Parameterizable {
    /// Apply parameter context to resolve all parameter references
    fn apply_parameters(&mut self, context: &ParameterContext) -> BuilderResult<()>;
}

/// Utility functions for working with parameters
pub mod utils {
    use super::*;
    
    /// Create a parameter reference string
    pub fn parameter_ref(name: &str) -> String {
        format!("${{{}}}", name)
    }
    
    /// Check if a string is a parameter reference
    pub fn is_parameter_ref(value: &str) -> bool {
        value.starts_with("${") && value.ends_with('}')
    }
    
    /// Extract parameter name from a parameter reference
    pub fn extract_parameter_name(param_ref: &str) -> Option<&str> {
        if is_parameter_ref(param_ref) {
            Some(&param_ref[2..param_ref.len()-1])
        } else {
            None
        }
    }
    
    /// Create a parameterized OSString
    pub fn parameterized_string(parameter_name: &str) -> OSString {
        OSString::parameter(parameter_name.to_string())
    }
    
    /// Create a parameterized double value
    pub fn parameterized_double(parameter_name: &str) -> crate::types::basic::Double {
        crate::types::basic::Double::parameter(parameter_name.to_string())
    }
    
    /// Create a parameterized int value
    pub fn parameterized_int(parameter_name: &str) -> crate::types::basic::Int {
        crate::types::basic::Int::parameter(parameter_name.to_string())
    }
    
    /// Create a parameterized boolean value
    pub fn parameterized_boolean(parameter_name: &str) -> crate::types::basic::Boolean {
        crate::types::basic::Boolean::parameter(parameter_name.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameter_declarations_builder() {
        let params = ParameterDeclarationsBuilder::new()
            .add_string_parameter("vehicle_name", "ego")
            .add_double_parameter("initial_speed", 25.0)
            .add_int_parameter("lane_id", 1)
            .add_boolean_parameter("enable_logging", true)
            .build();
        
        assert_eq!(params.parameter_declarations.len(), 4);
        
        let vehicle_param = &params.parameter_declarations[0];
        assert_eq!(vehicle_param.name.to_string(), "vehicle_name");
        assert_eq!(vehicle_param.parameter_type, ParameterType::String);
        assert_eq!(vehicle_param.value.to_string(), "ego");
    }

    #[test]
    fn test_parameterized_value_builder() {
        let literal_value = ParameterizedValueBuilder::literal(42.0).build();
        assert!(matches!(literal_value, Value::Literal(42.0)));
        
        let param_value = ParameterizedValueBuilder::<f64>::parameter("speed").build();
        assert!(matches!(param_value, Value::Parameter(ref name) if name == "speed"));
        
        let expr_value = ParameterizedValueBuilder::<f64>::expression("$speed * 2").build();
        assert!(matches!(expr_value, Value::Expression(ref expr) if expr == "$speed * 2"));
    }

    #[test]
    fn test_parameter_context() {
        let context = ParameterContext::new()
            .add_parameter("speed", "30.0")
            .add_parameter("vehicle", "sedan");
        
        assert_eq!(context.get_parameter("speed"), Some("30.0"));
        assert_eq!(context.get_parameter("vehicle"), Some("sedan"));
        assert_eq!(context.get_parameter("unknown"), None);
        
        let resolved = context.resolve_parameter("speed").unwrap();
        assert_eq!(resolved, "30.0");
    }

    #[test]
    fn test_parameter_utils() {
        assert_eq!(utils::parameter_ref("speed"), "${speed}");
        assert!(utils::is_parameter_ref("${speed}"));
        assert!(!utils::is_parameter_ref("speed"));
        assert_eq!(utils::extract_parameter_name("${speed}"), Some("speed"));
        assert_eq!(utils::extract_parameter_name("speed"), None);
    }
}