//! Builders for parameter declarations and the `${...}` references that use them.

use crate::types::{
    basic::{OSString, ParameterDeclaration, ParameterDeclarations, Value},
    enums::ParameterType,
};

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
            parameter_type: Value::Literal(ParameterType::String),
            value: OSString::literal(default_value.to_string()),
            constraint_groups: Vec::new(),
        });
        self
    }

    /// Add a double parameter
    pub fn add_double_parameter(mut self, name: &str, default_value: f64) -> Self {
        self.parameters.push(ParameterDeclaration {
            name: OSString::literal(name.to_string()),
            parameter_type: Value::Literal(ParameterType::Double),
            value: OSString::literal(default_value.to_string()),
            constraint_groups: Vec::new(),
        });
        self
    }

    /// Add an integer parameter
    pub fn add_int_parameter(mut self, name: &str, default_value: i32) -> Self {
        self.parameters.push(ParameterDeclaration {
            name: OSString::literal(name.to_string()),
            parameter_type: Value::Literal(ParameterType::Int),
            value: OSString::literal(default_value.to_string()),
            constraint_groups: Vec::new(),
        });
        self
    }

    /// Add a boolean parameter
    pub fn add_boolean_parameter(mut self, name: &str, default_value: bool) -> Self {
        self.parameters.push(ParameterDeclaration {
            name: OSString::literal(name.to_string()),
            parameter_type: Value::Literal(ParameterType::Boolean),
            value: OSString::literal(default_value.to_string()),
            constraint_groups: Vec::new(),
        });
        self
    }

    /// Add a date/time parameter
    pub fn add_datetime_parameter(mut self, name: &str, default_value: &str) -> Self {
        self.parameters.push(ParameterDeclaration {
            name: OSString::literal(name.to_string()),
            parameter_type: Value::Literal(ParameterType::DateTime),
            value: OSString::literal(default_value.to_string()),
            constraint_groups: Vec::new(),
        });
        self
    }

    /// Add an unsigned short parameter
    pub fn add_unsigned_short_parameter(mut self, name: &str, default_value: u16) -> Self {
        self.parameters.push(ParameterDeclaration {
            name: OSString::literal(name.to_string()),
            parameter_type: Value::Literal(ParameterType::UnsignedShort),
            value: OSString::literal(default_value.to_string()),
            constraint_groups: Vec::new(),
        });
        self
    }

    /// Add an unsigned int parameter
    pub fn add_unsigned_int_parameter(mut self, name: &str, default_value: u32) -> Self {
        self.parameters.push(ParameterDeclaration {
            name: OSString::literal(name.to_string()),
            parameter_type: Value::Literal(ParameterType::UnsignedInt),
            value: OSString::literal(default_value.to_string()),
            constraint_groups: Vec::new(),
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

/// Utility functions for working with parameters
pub mod utils {
    use super::*;

    /// Create a parameter reference string
    pub fn parameter_ref(name: &str) -> String {
        format!("${{{}}}", name)
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
        assert_eq!(
            vehicle_param.parameter_type,
            Value::Literal(ParameterType::String)
        );
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
    fn test_parameter_utils() {
        assert_eq!(utils::parameter_ref("speed"), "${speed}");
    }
}
