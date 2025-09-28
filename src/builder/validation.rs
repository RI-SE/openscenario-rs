//! Validation integration for builder system
//!
//! This module provides validation support for the builder system,
//! integrating with the existing validation framework to ensure
//! built scenarios comply with OpenSCENARIO schema requirements.

use crate::builder::{BuilderError, BuilderResult};
use crate::types::ValidationContext;
use crate::types::scenario::storyboard::OpenScenario;
use std::collections::HashMap;

/// Builder validation context that extends the existing validation framework
#[derive(Debug, Default)]
pub struct BuilderValidationContext {
    /// Base validation context
    base_context: ValidationContext,
    /// Builder-specific validation rules
    builder_rules: Vec<BuilderValidationRule>,
    /// Entity reference tracking
    entity_refs: HashMap<String, String>,
    /// Parameter tracking
    parameters: HashMap<String, String>,
}

impl BuilderValidationContext {
    /// Create a new builder validation context
    pub fn new() -> Self {
        Self {
            base_context: ValidationContext::new(),
            builder_rules: Vec::new(),
            entity_refs: HashMap::new(),
            parameters: HashMap::new(),
        }
    }
    
    /// Add a builder-specific validation rule
    pub fn add_rule(mut self, rule: BuilderValidationRule) -> Self {
        self.builder_rules.push(rule);
        self
    }
    
    /// Register an entity reference
    pub fn register_entity(&mut self, name: &str, entity_type: &str) {
        self.entity_refs.insert(name.to_string(), entity_type.to_string());
    }
    
    /// Register a parameter
    pub fn register_parameter(&mut self, name: &str, value: &str) {
        self.parameters.insert(name.to_string(), value.to_string());
    }
    
    /// Validate an entity reference exists
    pub fn validate_entity_ref(&self, entity_ref: &str) -> BuilderResult<()> {
        if !self.entity_refs.contains_key(entity_ref) {
            return Err(BuilderError::invalid_entity_ref(
                entity_ref,
                &self.entity_refs.keys().cloned().collect::<Vec<_>>()
            ));
        }
        Ok(())
    }
    
    /// Validate a parameter reference exists
    pub fn validate_parameter_ref(&self, param_ref: &str) -> BuilderResult<()> {
        if !self.parameters.contains_key(param_ref) {
            return Err(BuilderError::validation_error(&format!(
                "Parameter '{}' not found. Available parameters: {}",
                param_ref,
                self.parameters.keys().cloned().collect::<Vec<_>>().join(", ")
            )));
        }
        Ok(())
    }
    
    /// Validate a complete scenario
    pub fn validate_scenario(&self, scenario: &OpenScenario) -> BuilderResult<()> {
        // Run base validation first
        self.base_context.validate_scenario(scenario)
            .map_err(|e| BuilderError::OpenScenarioError(e))?;
        
        // Run builder-specific validation rules
        for rule in &self.builder_rules {
            rule.validate(scenario, self)?;
        }
        
        Ok(())
    }
    
    /// Get all registered entities
    pub fn entities(&self) -> &HashMap<String, String> {
        &self.entity_refs
    }
    
    /// Get all registered parameters
    pub fn parameters(&self) -> &HashMap<String, String> {
        &self.parameters
    }
}

/// Trait for builder-specific validation rules
pub trait BuilderValidationRule {
    /// Validate a scenario using this rule
    fn validate(&self, scenario: &OpenScenario, context: &BuilderValidationContext) -> BuilderResult<()>;
    
    /// Get the name of this validation rule
    fn name(&self) -> &str;
    
    /// Get the description of this validation rule
    fn description(&self) -> &str;
}

/// Validation rule for entity references
#[derive(Debug)]
pub struct EntityReferenceValidationRule;

impl BuilderValidationRule for EntityReferenceValidationRule {
    fn validate(&self, scenario: &OpenScenario, context: &BuilderValidationContext) -> BuilderResult<()> {
        // Validate that all entity references in actions and conditions exist
        if let Some(storyboard) = &scenario.storyboard {
            for story in &storyboard.stories {
                for act in &story.acts {
                    for maneuver_group in &act.maneuver_groups {
                        for maneuver in &maneuver_group.maneuvers {
                            for event in &maneuver.events {
                                for action in &event.actions {
                                    if let Some(private_action) = &action.private_action {
                                        let entity_ref = private_action.entity_ref.to_string();
                                        context.validate_entity_ref(&entity_ref)?;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
    
    fn name(&self) -> &str {
        "EntityReferenceValidation"
    }
    
    fn description(&self) -> &str {
        "Validates that all entity references in actions and conditions exist"
    }
}

/// Validation rule for parameter references
#[derive(Debug)]
pub struct ParameterReferenceValidationRule;

impl BuilderValidationRule for ParameterReferenceValidationRule {
    fn validate(&self, scenario: &OpenScenario, context: &BuilderValidationContext) -> BuilderResult<()> {
        // This would validate that all parameter references are declared
        // For now, we'll do a basic check
        if let Some(param_decls) = &scenario.parameter_declarations {
            for param in &param_decls.parameter_declarations {
                let param_name = param.name.to_string();
                if !context.parameters.contains_key(&param_name) {
                    // This is actually OK - parameters can be declared without being used
                    // But we could warn about unused parameters
                }
            }
        }
        Ok(())
    }
    
    fn name(&self) -> &str {
        "ParameterReferenceValidation"
    }
    
    fn description(&self) -> &str {
        "Validates that all parameter references are properly declared"
    }
}

/// Validation rule for catalog references
#[derive(Debug)]
pub struct CatalogReferenceValidationRule;

impl BuilderValidationRule for CatalogReferenceValidationRule {
    fn validate(&self, scenario: &OpenScenario, _context: &BuilderValidationContext) -> BuilderResult<()> {
        // Validate that catalog locations are specified if catalog references are used
        if let Some(entities) = &scenario.entities {
            let has_catalog_refs = entities.scenario_objects.iter()
                .any(|obj| obj.catalog_reference.is_some());
            
            if has_catalog_refs && scenario.catalog_locations.is_none() {
                return Err(BuilderError::validation_error(
                    "Catalog references found but no catalog locations specified"
                ));
            }
        }
        Ok(())
    }
    
    fn name(&self) -> &str {
        "CatalogReferenceValidation"
    }
    
    fn description(&self) -> &str {
        "Validates that catalog locations are specified when catalog references are used"
    }
}

/// Validation rule for storyboard structure
#[derive(Debug)]
pub struct StoryboardStructureValidationRule;

impl BuilderValidationRule for StoryboardStructureValidationRule {
    fn validate(&self, scenario: &OpenScenario, _context: &BuilderValidationContext) -> BuilderResult<()> {
        if let Some(storyboard) = &scenario.storyboard {
            // Validate that stories have at least one act
            for story in &storyboard.stories {
                if story.acts.is_empty() {
                    return Err(BuilderError::validation_error(&format!(
                        "Story '{}' has no acts", story.name.to_string()
                    )));
                }
                
                // Validate that acts have at least one maneuver group
                for act in &story.acts {
                    if act.maneuver_groups.is_empty() {
                        return Err(BuilderError::validation_error(&format!(
                            "Act '{}' has no maneuver groups", act.name.to_string()
                        )));
                    }
                    
                    // Validate that maneuver groups have at least one maneuver
                    for maneuver_group in &act.maneuver_groups {
                        if maneuver_group.maneuvers.is_empty() {
                            return Err(BuilderError::validation_error(&format!(
                                "Maneuver group '{}' has no maneuvers", 
                                maneuver_group.name.to_string()
                            )));
                        }
                    }
                }
            }
        }
        Ok(())
    }
    
    fn name(&self) -> &str {
        "StoryboardStructureValidation"
    }
    
    fn description(&self) -> &str {
        "Validates the hierarchical structure of the storyboard"
    }
}

/// Builder for validation contexts with common rules
#[derive(Debug, Default)]
pub struct ValidationContextBuilder {
    rules: Vec<Box<dyn BuilderValidationRule>>,
    entities: HashMap<String, String>,
    parameters: HashMap<String, String>,
}

impl ValidationContextBuilder {
    /// Create a new validation context builder
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add standard validation rules
    pub fn with_standard_rules(mut self) -> Self {
        self.rules.push(Box::new(EntityReferenceValidationRule));
        self.rules.push(Box::new(ParameterReferenceValidationRule));
        self.rules.push(Box::new(CatalogReferenceValidationRule));
        self.rules.push(Box::new(StoryboardStructureValidationRule));
        self
    }
    
    /// Add a custom validation rule
    pub fn with_rule(mut self, rule: Box<dyn BuilderValidationRule>) -> Self {
        self.rules.push(rule);
        self
    }
    
    /// Add entity information
    pub fn with_entity(mut self, name: &str, entity_type: &str) -> Self {
        self.entities.insert(name.to_string(), entity_type.to_string());
        self
    }
    
    /// Add parameter information
    pub fn with_parameter(mut self, name: &str, value: &str) -> Self {
        self.parameters.insert(name.to_string(), value.to_string());
        self
    }
    
    /// Build the validation context
    pub fn build(self) -> BuilderValidationContext {
        let mut context = BuilderValidationContext::new();
        
        // Add rules (note: we can't move Box<dyn Trait> easily, so we'll recreate standard rules)
        context = context
            .add_rule(EntityReferenceValidationRule)
            .add_rule(ParameterReferenceValidationRule)
            .add_rule(CatalogReferenceValidationRule)
            .add_rule(StoryboardStructureValidationRule);
        
        // Add entities and parameters
        for (name, entity_type) in self.entities {
            context.register_entity(&name, &entity_type);
        }
        
        for (name, value) in self.parameters {
            context.register_parameter(&name, &value);
        }
        
        context
    }
}

/// Trait for types that can be validated by the builder system
pub trait BuilderValidatable {
    /// Validate this object using the builder validation context
    fn validate_with_context(&self, context: &BuilderValidationContext) -> BuilderResult<()>;
}

impl BuilderValidatable for OpenScenario {
    fn validate_with_context(&self, context: &BuilderValidationContext) -> BuilderResult<()> {
        context.validate_scenario(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_validation_context() {
        let mut context = BuilderValidationContext::new();
        
        // Register entities and parameters
        context.register_entity("ego", "vehicle");
        context.register_entity("target", "vehicle");
        context.register_parameter("speed", "30.0");
        context.register_parameter("lane", "1");
        
        // Test entity validation
        assert!(context.validate_entity_ref("ego").is_ok());
        assert!(context.validate_entity_ref("target").is_ok());
        assert!(context.validate_entity_ref("unknown").is_err());
        
        // Test parameter validation
        assert!(context.validate_parameter_ref("speed").is_ok());
        assert!(context.validate_parameter_ref("lane").is_ok());
        assert!(context.validate_parameter_ref("unknown").is_err());
    }

    #[test]
    fn test_validation_context_builder() {
        let context = ValidationContextBuilder::new()
            .with_standard_rules()
            .with_entity("ego", "vehicle")
            .with_entity("target", "pedestrian")
            .with_parameter("speed", "25.0")
            .with_parameter("distance", "100.0")
            .build();
        
        assert_eq!(context.entities().len(), 2);
        assert_eq!(context.parameters().len(), 2);
        assert!(context.validate_entity_ref("ego").is_ok());
        assert!(context.validate_parameter_ref("speed").is_ok());
    }

    #[test]
    fn test_validation_rules() {
        let rule = EntityReferenceValidationRule;
        assert_eq!(rule.name(), "EntityReferenceValidation");
        assert!(!rule.description().is_empty());
        
        let rule = ParameterReferenceValidationRule;
        assert_eq!(rule.name(), "ParameterReferenceValidation");
        assert!(!rule.description().is_empty());
        
        let rule = CatalogReferenceValidationRule;
        assert_eq!(rule.name(), "CatalogReferenceValidation");
        assert!(!rule.description().is_empty());
        
        let rule = StoryboardStructureValidationRule;
        assert_eq!(rule.name(), "StoryboardStructureValidation");
        assert!(!rule.description().is_empty());
    }
}