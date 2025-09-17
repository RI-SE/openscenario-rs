//! Parameter substitution engine for catalog resolution
//!
//! This module handles:
//! - Parameter parsing and substitution in catalog entities
//! - Parameter validation against schemas and constraints
//! - Nested parameter reference resolution
//! - Context-aware parameter evaluation

use crate::error::{Error, Result};
use crate::types::basic::Value;
use crate::types::catalogs::entities::{CatalogEntity, ParameterDefinition};
use regex::Regex;
use std::collections::HashMap;

/// Parameter substitution engine
pub struct ParameterSubstitutionEngine {
    /// Current parameter context (parameter name -> resolved value)
    parameter_context: HashMap<String, String>,
    /// Parameter definitions with schemas and constraints
    parameter_definitions: HashMap<String, ParameterDefinition>,
    /// Cached regex for parameter detection
    parameter_regex: Regex,
}

impl ParameterSubstitutionEngine {
    /// Create a new parameter substitution engine
    pub fn new() -> Self {
        Self {
            parameter_context: HashMap::new(),
            parameter_definitions: HashMap::new(),
            parameter_regex: Regex::new(r"\$\{([^}]+)\}").unwrap(),
        }
    }

    /// Create a parameter engine with initial context
    pub fn with_context(context: HashMap<String, String>) -> Self {
        Self {
            parameter_context: context,
            parameter_definitions: HashMap::new(),
            parameter_regex: Regex::new(r"\$\{([^}]+)\}").unwrap(),
        }
    }

    /// Add parameter definitions from a catalog entity schema
    pub fn add_parameter_schema<T: CatalogEntity>(&mut self) {
        let schema = T::parameter_schema();
        for param_def in schema {
            self.parameter_definitions
                .insert(param_def.name.clone(), param_def);
        }
    }

    /// Set a parameter value in the context
    pub fn set_parameter(&mut self, name: String, value: String) -> Result<()> {
        // Validate against schema if available
        if let Some(param_def) = self.parameter_definitions.get(&name) {
            self.validate_parameter_value(&name, &value, param_def)?;
        }

        self.parameter_context.insert(name, value);
        Ok(())
    }

    /// Set multiple parameter values
    pub fn set_parameters(&mut self, parameters: HashMap<String, String>) -> Result<()> {
        for (name, value) in parameters {
            self.set_parameter(name, value)?;
        }
        Ok(())
    }

    /// Get a parameter value from the context
    pub fn get_parameter(&self, name: &str) -> Option<&String> {
        self.parameter_context.get(name)
    }

    /// Resolve a parameter expression
    ///
    /// Handles expressions like:
    /// - `${ParameterName}` - Simple parameter reference
    /// - `${ParameterName + 10}` - Expression evaluation (future enhancement)
    pub fn resolve_parameter_expression(&self, expression: &str) -> Result<String> {
        if !self.parameter_regex.is_match(expression) {
            // No parameters to substitute - return as-is
            return Ok(expression.to_string());
        }

        let mut result = expression.to_string();

        // Find all parameter references
        for captures in self.parameter_regex.captures_iter(expression) {
            let full_match = captures.get(0).unwrap().as_str();
            let param_name = captures.get(1).unwrap().as_str().trim();

            // Look up the parameter value
            let param_value = self.parameter_context.get(param_name).ok_or_else(|| {
                Error::catalog_error(&format!(
                    "Parameter '{}' not found in substitution context",
                    param_name
                ))
            })?;

            result = result.replace(full_match, param_value);
        }

        Ok(result)
    }

    /// Resolve parameters in a Value<T> field
    pub fn resolve_value<T>(&self, value: &Value<T>) -> Result<T>
    where
        T: std::str::FromStr + Clone,
        T::Err: std::fmt::Display,
    {
        match value {
            Value::Literal(v) => Ok(v.clone()),
            Value::Parameter(param_name) => {
                let param_value = self.parameter_context.get(param_name).ok_or_else(|| {
                    Error::catalog_error(&format!(
                        "Parameter '{}' not found in substitution context",
                        param_name
                    ))
                })?;

                param_value.parse().map_err(|e| {
                    Error::catalog_error(&format!(
                        "Failed to parse parameter '{}' value '{}': {}",
                        param_name, param_value, e
                    ))
                })
            }
            Value::Expression(expr) => {
                let resolved_expr = self.resolve_parameter_expression(expr)?;
                resolved_expr.parse().map_err(|e| {
                    Error::catalog_error(&format!(
                        "Failed to parse resolved expression '{}': {}",
                        resolved_expr, e
                    ))
                })
            }
        }
    }

    /// Substitute parameters in a catalog entity to produce a resolved scenario entity
    pub fn substitute_parameters<T: CatalogEntity>(
        &self,
        catalog_entity: T,
        additional_params: &HashMap<String, String>,
    ) -> Result<T::ResolvedType> {
        // Merge additional parameters with our context
        let mut combined_context = self.parameter_context.clone();
        combined_context.extend(additional_params.clone());

        // Use the catalog entity's own resolution method
        catalog_entity.into_scenario_entity(combined_context)
    }

    /// Validate a parameter value against its definition
    fn validate_parameter_value(
        &self,
        name: &str,
        value: &str,
        definition: &ParameterDefinition,
    ) -> Result<()> {
        // Type validation
        match definition.parameter_type.as_str() {
            "Double" | "Float" => {
                value.parse::<f64>().map_err(|_| {
                    Error::catalog_error(&format!(
                        "Parameter '{}' must be a valid number, got '{}'",
                        name, value
                    ))
                })?;
            }
            "Integer" | "Int" => {
                value.parse::<i32>().map_err(|_| {
                    Error::catalog_error(&format!(
                        "Parameter '{}' must be a valid integer, got '{}'",
                        name, value
                    ))
                })?;
            }
            "Boolean" => {
                value.parse::<bool>().map_err(|_| {
                    Error::catalog_error(&format!(
                        "Parameter '{}' must be a valid boolean (true/false), got '{}'",
                        name, value
                    ))
                })?;
            }
            "String" => {
                // String values are always valid
            }
            _ => {
                // Unknown parameter type - log warning but don't fail
            }
        }

        Ok(())
    }

    /// Get all parameter names in the current context
    pub fn parameter_names(&self) -> Vec<&String> {
        self.parameter_context.keys().collect()
    }

    /// Clear the parameter context
    pub fn clear_context(&mut self) {
        self.parameter_context.clear();
    }

    /// Get the current parameter context
    pub fn context(&self) -> &HashMap<String, String> {
        &self.parameter_context
    }

    /// Create a child engine with additional context
    pub fn with_additional_context(&self, additional: HashMap<String, String>) -> Self {
        let mut context = self.parameter_context.clone();
        context.extend(additional);

        Self {
            parameter_context: context,
            parameter_definitions: self.parameter_definitions.clone(),
            parameter_regex: self.parameter_regex.clone(),
        }
    }
}

impl Default for ParameterSubstitutionEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::basic::*;
    use crate::types::catalogs::entities::CatalogVehicle;
    #[test]
    fn test_parameter_substitution_engine_creation() {
        let engine = ParameterSubstitutionEngine::new();
        assert_eq!(engine.parameter_names().len(), 0);

        let mut context = HashMap::new();
        context.insert("TestParam".to_string(), "42".to_string());
        let engine_with_context = ParameterSubstitutionEngine::with_context(context);
        assert_eq!(engine_with_context.parameter_names().len(), 1);
    }

    #[test]
    fn test_parameter_setting_and_getting() {
        let mut engine = ParameterSubstitutionEngine::new();

        // Set a parameter
        engine
            .set_parameter("MaxSpeed".to_string(), "60.0".to_string())
            .unwrap();
        assert_eq!(engine.get_parameter("MaxSpeed").unwrap(), "60.0");

        // Set multiple parameters
        let mut params = HashMap::new();
        params.insert("MinSpeed".to_string(), "0.0".to_string());
        params.insert("Acceleration".to_string(), "5.0".to_string());
        engine.set_parameters(params).unwrap();

        assert_eq!(engine.parameter_names().len(), 3);
        assert_eq!(engine.get_parameter("MinSpeed").unwrap(), "0.0");
        assert_eq!(engine.get_parameter("Acceleration").unwrap(), "5.0");
    }

    #[test]
    fn test_parameter_expression_resolution() {
        let mut engine = ParameterSubstitutionEngine::new();
        engine
            .set_parameter("MaxSpeed".to_string(), "60.0".to_string())
            .unwrap();
        engine
            .set_parameter("VehicleName".to_string(), "TestVehicle".to_string())
            .unwrap();

        // Simple parameter reference
        let result = engine.resolve_parameter_expression("${MaxSpeed}").unwrap();
        assert_eq!(result, "60.0");

        // Multiple parameters in one expression
        let result = engine
            .resolve_parameter_expression("Vehicle ${VehicleName} with speed ${MaxSpeed}")
            .unwrap();
        assert_eq!(result, "Vehicle TestVehicle with speed 60.0");

        // No parameters - should return as-is
        let result = engine.resolve_parameter_expression("NoParameters").unwrap();
        assert_eq!(result, "NoParameters");

        // Missing parameter should error
        let result = engine.resolve_parameter_expression("${MissingParam}");
        assert!(result.is_err());
    }

    #[test]
    fn test_value_resolution() {
        let mut engine = ParameterSubstitutionEngine::new();
        engine
            .set_parameter("TestFloat".to_string(), "42.5".to_string())
            .unwrap();
        engine
            .set_parameter("TestInt".to_string(), "100".to_string())
            .unwrap();
        engine
            .set_parameter("TestString".to_string(), "Hello".to_string())
            .unwrap();

        // Literal value
        let literal_value: Double = Value::Literal(25.0);
        assert_eq!(engine.resolve_value(&literal_value).unwrap(), 25.0);

        // Parameter value
        let param_value: Double = Value::Parameter("TestFloat".to_string());
        assert_eq!(engine.resolve_value(&param_value).unwrap(), 42.5);

        // Expression value
        let expr_value: Int = Value::Expression("${TestInt}".to_string());
        assert_eq!(engine.resolve_value(&expr_value).unwrap(), 100);

        // String value
        let string_value: OSString = Value::Parameter("TestString".to_string());
        assert_eq!(engine.resolve_value(&string_value).unwrap(), "Hello");
    }

    #[test]
    fn test_parameter_schema_validation() {
        let mut engine = ParameterSubstitutionEngine::new();

        // Add schema for vehicle parameters
        engine.add_parameter_schema::<CatalogVehicle>();

        // Valid parameter values should work
        assert!(engine
            .set_parameter("MaxSpeed".to_string(), "60.0".to_string())
            .is_ok());

        // Invalid parameter values should be caught
        // Note: The current implementation doesn't enforce strict validation for unknown parameters
        // This could be enhanced in the future
    }

    #[test]
    fn test_context_operations() {
        let mut engine = ParameterSubstitutionEngine::new();
        engine
            .set_parameter("Param1".to_string(), "Value1".to_string())
            .unwrap();
        engine
            .set_parameter("Param2".to_string(), "Value2".to_string())
            .unwrap();

        // Test getting context
        let context = engine.context();
        assert_eq!(context.len(), 2);
        assert_eq!(context.get("Param1").unwrap(), "Value1");

        // Test parameter names
        let names = engine.parameter_names();
        assert_eq!(names.len(), 2);

        // Test clearing context
        engine.clear_context();
        assert_eq!(engine.parameter_names().len(), 0);
    }

    #[test]
    fn test_child_engine_with_additional_context() {
        let mut parent_engine = ParameterSubstitutionEngine::new();
        parent_engine
            .set_parameter("ParentParam".to_string(), "ParentValue".to_string())
            .unwrap();

        let mut additional = HashMap::new();
        additional.insert("ChildParam".to_string(), "ChildValue".to_string());

        let child_engine = parent_engine.with_additional_context(additional);

        // Child should have both parent and additional parameters
        assert_eq!(child_engine.parameter_names().len(), 2);
        assert_eq!(
            child_engine.get_parameter("ParentParam").unwrap(),
            "ParentValue"
        );
        assert_eq!(
            child_engine.get_parameter("ChildParam").unwrap(),
            "ChildValue"
        );

        // Parent should remain unchanged
        assert_eq!(parent_engine.parameter_names().len(), 1);
    }
}
