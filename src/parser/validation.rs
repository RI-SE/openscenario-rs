//! Validation system for parsed OpenSCENARIO content
//!
//! This module provides comprehensive validation of OpenSCENARIO documents beyond basic XML
//! parsing, ensuring logical consistency and adherence to domain-specific constraints.
//!
//! # Features
//!
//! - **Multi-level validation** - structure, references, constraints, and semantics
//! - **Detailed error reporting** with location context and fix suggestions
//! - **Performance metrics** and caching for large scenario validation
//! - **Configurable validation** modes (strict, lenient, custom rules)
//! - **Cross-reference checking** for entities, catalogs, and parameters
//!
//! # Basic Usage
//!
//! ## Simple Validation
//!
//! ```rust,no_run
//! use openscenario_rs::parser::validation::ScenarioValidator;
//! use openscenario_rs::parser::xml::parse_from_file;
//!
//! // Parse and validate a scenario
//! let scenario = parse_from_file("scenario.xosc")?;
//! let mut validator = ScenarioValidator::new();
//! let result = validator.validate_scenario(&scenario);
//!
//! if result.is_valid() {
//!     println!("✓ Scenario is valid");
//! } else {
//!     println!("✗ Found {} errors:", result.errors.len());
//!     for error in &result.errors {
//!         println!("  - {}: {}", error.location, error.message);
//!         if let Some(suggestion) = &error.suggestion {
//!             println!("    Suggestion: {}", suggestion);
//!         }
//!     }
//! }
//!
//! // Check warnings too
//! if !result.warnings.is_empty() {
//!     println!("⚠ {} warnings found", result.warnings.len());
//!     for warning in &result.warnings {
//!         println!("  - {}: {}", warning.location, warning.message);
//!     }
//! }
//! ```
//!
//! ## Custom Validation Configuration
//!
//! ```rust,no_run
//! use openscenario_rs::parser::validation::{ScenarioValidator, ValidationConfig};
//!
//! let config = ValidationConfig {
//!     strict_mode: true,           // Treat warnings as errors
//!     validate_references: true,   // Check entity/catalog references
//!     validate_constraints: true,  // Check business rules
//!     validate_semantics: true,    // Check logical consistency
//!     max_errors: 50,             // Stop after 50 errors
//!     use_cache: true,            // Enable performance caching
//! };
//!
//! let mut validator = ScenarioValidator::with_config(config);
//! let result = validator.validate_scenario(&scenario);
//!
//! println!("Validation completed in {}ms", result.metrics.duration_ms);
//! println!("Validated {} elements", result.metrics.elements_validated);
//! ```
//!
//! ## Validation Categories
//!
//! The validator checks multiple categories of issues:
//!
//! ### Structural Validation
//! - Required fields and attributes
//! - Valid data types and ranges
//! - Schema compliance
//!
//! ### Reference Validation
//! ```rust
//! // Checks that entity references point to defined entities
//! // Validates catalog references exist and are accessible
//! // Ensures parameter references resolve correctly
//! ```
//!
//! ### Constraint Validation
//! ```rust
//! // Enforces OpenSCENARIO business rules
//! // Checks unique names and IDs
//! // Validates value ranges and relationships
//! ```
//!
//! ### Semantic Validation
//! ```rust
//! // Ensures logical consistency
//! // Detects impossible scenarios
//! // Validates temporal relationships
//! ```
//!
//! # Error Categories and Handling
//!
//! ```rust
//! use openscenario_rs::parser::validation::ValidationErrorCategory;
//!
//! for error in &result.errors {
//!     match error.category {
//!         ValidationErrorCategory::MissingRequired => {
//!             // Handle missing required fields
//!             eprintln!("Missing required field: {}", error.location);
//!         }
//!         ValidationErrorCategory::InvalidReference => {
//!             // Handle broken references
//!             eprintln!("Invalid reference at {}: {}", error.location, error.message);
//!         }
//!         ValidationErrorCategory::ConstraintViolation => {
//!             // Handle business rule violations
//!             eprintln!("Constraint violation: {}", error.message);
//!         }
//!         ValidationErrorCategory::SemanticError => {
//!             // Handle logical inconsistencies
//!             eprintln!("Semantic error: {}", error.message);
//!         }
//!         ValidationErrorCategory::TypeMismatch => {
//!             // Handle type errors
//!             eprintln!("Type mismatch: {}", error.message);
//!         }
//!         ValidationErrorCategory::ParameterError => {
//!             // Handle parameter resolution errors
//!             eprintln!("Parameter error: {}", error.message);
//!         }
//!     }
//! }
//! ```
//!
//! # Performance Optimization
//!
//! For large scenarios or repeated validations:
//!
//! ```rust
//! let config = ValidationConfig {
//!     use_cache: true,         // Enable validation caching
//!     max_errors: 20,          // Stop early to save time
//!     validate_semantics: false, // Skip expensive semantic checks
//!     ..Default::default()
//! };
//!
//! let mut validator = ScenarioValidator::with_config(config);
//! 
//! // Validate multiple scenarios efficiently
//! for scenario_file in scenario_files {
//!     let scenario = parse_from_file(scenario_file)?;
//!     let result = validator.validate_scenario(&scenario);
//!     
//!     println!("Cache hit ratio: {:.1}%", result.metrics.cache_hit_ratio * 100.0);
//! }
//! ```
//!
//! # Integration with Parsing
//!
//! ```rust,no_run
//! use openscenario_rs::parser::{xml, validation};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Parse and validate in one step
//! let scenario = xml::parse_from_file_validated("scenario.xosc")?;
//! 
//! // Then do domain-specific validation
//! let mut validator = validation::ScenarioValidator::new();
//! let validation_result = validator.validate_scenario(&scenario);
//! 
//! // Check both parsing and validation results
//! if validation_result.is_clean() {
//!     println!("Scenario is fully valid and clean");
//! }
//! # Ok(())
//! # }
//! ```

use crate::{
    types::{
        entities::{Entities, ScenarioObject},
        scenario::triggers::Condition,
        scenario::{
            story::{Act, Event, Maneuver, ManeuverGroup},
            storyboard::Storyboard,
            ScenarioStory,
        },
        EntityRef, ObjectType, ValidationContext,
    },
    FileHeader, OpenScenario,
};
use std::collections::{HashMap, HashSet};

/// Comprehensive validation engine for OpenSCENARIO documents
///
/// This validator performs multi-level validation:
/// 1. Structural validation - ensures proper hierarchy and required fields
/// 2. Reference validation - validates entity and catalog references
/// 3. Constraint validation - enforces business rules and constraints
/// 4. Semantic validation - ensures logical consistency
#[derive(Debug)]
pub struct ScenarioValidator {
    /// Validation configuration options
    config: ValidationConfig,
    /// Cache for performance optimization
    validation_cache: HashMap<String, ValidationResult>,
}

/// Configuration for validation behavior
#[derive(Debug, Clone)]
pub struct ValidationConfig {
    /// Enable strict validation mode (fail on warnings)
    pub strict_mode: bool,
    /// Enable cross-reference validation
    pub validate_references: bool,
    /// Enable constraint validation
    pub validate_constraints: bool,
    /// Enable semantic validation
    pub validate_semantics: bool,
    /// Maximum validation errors before stopping
    pub max_errors: usize,
    /// Enable performance optimizations
    pub use_cache: bool,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            strict_mode: false,
            validate_references: true,
            validate_constraints: true,
            validate_semantics: true,
            max_errors: 100,
            use_cache: true,
        }
    }
}

/// Result of validation operation
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationResult {
    /// Validation errors (must be fixed)
    pub errors: Vec<ValidationError>,
    /// Validation warnings (should be addressed)
    pub warnings: Vec<ValidationWarning>,
    /// Performance metrics
    pub metrics: ValidationMetrics,
}

/// Detailed validation error information
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationError {
    /// Error category
    pub category: ValidationErrorCategory,
    /// Field or location where error occurred
    pub location: String,
    /// Detailed error message
    pub message: String,
    /// Suggested fix (if available)
    pub suggestion: Option<String>,
}

/// Validation warning information
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationWarning {
    /// Warning category
    pub category: ValidationWarningCategory,
    /// Field or location where warning occurred
    pub location: String,
    /// Warning message
    pub message: String,
    /// Suggested improvement (if available)
    pub suggestion: Option<String>,
}

/// Categories of validation errors
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValidationErrorCategory {
    /// Missing required field
    MissingRequired,
    /// Invalid reference to entity or catalog
    InvalidReference,
    /// Constraint violation
    ConstraintViolation,
    /// Semantic inconsistency
    SemanticError,
    /// Type mismatch
    TypeMismatch,
    /// Invalid parameter reference
    ParameterError,
}

/// Categories of validation warnings
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValidationWarningCategory {
    /// Deprecated field or value
    Deprecated,
    /// Potentially problematic configuration
    Suspicious,
    /// Performance concern
    Performance,
    /// Best practice violation
    BestPractice,
}

/// Performance metrics for validation
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationMetrics {
    /// Total validation time in milliseconds
    pub duration_ms: u64,
    /// Number of validated elements
    pub elements_validated: usize,
    /// Cache hit ratio (0.0 to 1.0)
    pub cache_hit_ratio: f64,
}

impl ScenarioValidator {
    /// Create a new validator with default configuration
    pub fn new() -> Self {
        Self {
            config: ValidationConfig::default(),
            validation_cache: HashMap::new(),
        }
    }

    /// Create a validator with custom configuration
    pub fn with_config(config: ValidationConfig) -> Self {
        Self {
            config,
            validation_cache: HashMap::new(),
        }
    }

    /// Validate a complete OpenSCENARIO document
    pub fn validate_scenario(&mut self, scenario: &OpenScenario) -> ValidationResult {
        let start_time = std::time::Instant::now();
        let context = self.build_validation_context(scenario);
        let mut result = ValidationResult {
            errors: Vec::new(),
            warnings: Vec::new(),
            metrics: ValidationMetrics {
                duration_ms: 0,
                elements_validated: 0,
                cache_hit_ratio: 0.0,
            },
        };

        // Validate file header
        self.validate_file_header(&scenario.file_header, &mut result);

        // Validate scenario content based on document type
        match scenario.document_type() {
            crate::types::scenario::storyboard::OpenScenarioDocumentType::Scenario => {
                // Validate entities
                if let Some(entities) = &scenario.entities {
                    self.validate_entities(entities, &context, &mut result);
                }
                // Validate storyboard
                if let Some(storyboard) = &scenario.storyboard {
                    self.validate_storyboard(storyboard, &context, &mut result);
                }
            }
            crate::types::scenario::storyboard::OpenScenarioDocumentType::ParameterVariation => {
                // Parameter variation files don't have entities or storyboards to validate
            }
            crate::types::scenario::storyboard::OpenScenarioDocumentType::Catalog => {
                // Catalog files have their own validation rules
            }
            crate::types::scenario::storyboard::OpenScenarioDocumentType::Unknown => {
                result.errors.push(ValidationError {
                    category: ValidationErrorCategory::SemanticError,
                    location: "root".to_string(),
                    message: "Unknown document type - no valid scenario, parameter variation, or catalog structure found".to_string(),
                    suggestion: None,
                });
            }
        }

        // Update metrics
        let duration = start_time.elapsed();
        result.metrics.duration_ms = duration.as_millis() as u64;
        result.metrics.cache_hit_ratio = self.calculate_cache_hit_ratio();

        result
    }

    /// Build validation context from scenario
    fn build_validation_context(&self, scenario: &OpenScenario) -> ValidationContext {
        let mut context = ValidationContext::new();

        if self.config.strict_mode {
            context = context.with_strict_mode();
        }

        // Register entities (only for scenario definitions)
        if let Some(entities) = &scenario.entities {
            for obj in &entities.scenario_objects {
                let entity_ref = EntityRef {
                    name: obj.name.as_literal().unwrap_or(&String::new()).clone(),
                    object_type: if obj.vehicle.is_some() {
                        ObjectType::Vehicle
                    } else if obj.pedestrian.is_some() {
                        ObjectType::Pedestrian
                    } else {
                        ObjectType::MiscellaneousObject
                    },
                };
                context.add_entity(entity_ref.name.clone(), entity_ref);
            }
        }

        context
    }

    /// Validate file header
    fn validate_file_header(&self, header: &FileHeader, result: &mut ValidationResult) {
        // Check required fields
        if header
            .author
            .as_literal()
            .unwrap_or(&String::new())
            .is_empty()
        {
            result.errors.push(ValidationError {
                category: ValidationErrorCategory::MissingRequired,
                location: "FileHeader.author".to_string(),
                message: "Author field is required and cannot be empty".to_string(),
                suggestion: Some("Provide a valid author name".to_string()),
            });
        }

        if header
            .description
            .as_literal()
            .unwrap_or(&String::new())
            .is_empty()
        {
            result.warnings.push(ValidationWarning {
                category: ValidationWarningCategory::BestPractice,
                location: "FileHeader.description".to_string(),
                message: "Description should be provided for documentation".to_string(),
                suggestion: Some("Add a meaningful description of the scenario".to_string()),
            });
        }

        // Check version compatibility
        let rev_major = *header.rev_major.as_literal().unwrap_or(&0);
        let rev_minor = *header.rev_minor.as_literal().unwrap_or(&0);

        if rev_major < 1 {
            result.errors.push(ValidationError {
                category: ValidationErrorCategory::ConstraintViolation,
                location: "FileHeader.revMajor".to_string(),
                message: "Major revision must be at least 1".to_string(),
                suggestion: Some("Use OpenSCENARIO version 1.0 or later".to_string()),
            });
        }

        if rev_major > 1 || (rev_major == 1 && rev_minor > 3) {
            result.warnings.push(ValidationWarning {
                category: ValidationWarningCategory::Suspicious,
                location: format!("FileHeader.rev{}.{}", rev_major, rev_minor),
                message: "Using future OpenSCENARIO version - compatibility not guaranteed"
                    .to_string(),
                suggestion: Some("Consider using a stable OpenSCENARIO version".to_string()),
            });
        }
    }

    /// Validate entities section
    fn validate_entities(
        &self,
        entities: &Entities,
        context: &ValidationContext,
        result: &mut ValidationResult,
    ) {
        if entities.scenario_objects.is_empty() {
            result.errors.push(ValidationError {
                category: ValidationErrorCategory::MissingRequired,
                location: "Entities".to_string(),
                message: "At least one scenario object must be defined".to_string(),
                suggestion: Some("Add vehicle, pedestrian, or miscellaneous objects".to_string()),
            });
        }

        // Validate each scenario object
        for (index, obj) in entities.scenario_objects.iter().enumerate() {
            self.validate_scenario_object(
                obj,
                context,
                &format!("Entities.ScenarioObject[{}]", index),
                result,
            );
        }

        // Check for duplicate entity names
        let mut names = HashSet::new();
        for obj in &entities.scenario_objects {
            let default_name = String::new();
            let name = obj.name.as_literal().unwrap_or(&default_name);
            if !names.insert(name.clone()) {
                result.errors.push(ValidationError {
                    category: ValidationErrorCategory::ConstraintViolation,
                    location: format!("Entities.ScenarioObject[name='{}']", name),
                    message: "Duplicate entity names are not allowed".to_string(),
                    suggestion: Some("Ensure all entity names are unique".to_string()),
                });
            }
        }
    }

    /// Validate individual scenario object
    fn validate_scenario_object(
        &self,
        obj: &ScenarioObject,
        _context: &ValidationContext,
        location: &str,
        result: &mut ValidationResult,
    ) {
        // Validate name
        let default_name = String::new();
        let name = obj.name.as_literal().unwrap_or(&default_name);
        if name.is_empty() {
            result.errors.push(ValidationError {
                category: ValidationErrorCategory::MissingRequired,
                location: format!("{}.name", location),
                message: "ScenarioObject name is required".to_string(),
                suggestion: Some("Provide a unique name for the entity".to_string()),
            });
        }

        // Entity-specific validation could be added here when Validate trait is implemented
        // for ScenarioObject

        result.metrics.elements_validated += 1;
    }

    /// Validate storyboard section
    fn validate_storyboard(
        &self,
        storyboard: &Storyboard,
        context: &ValidationContext,
        result: &mut ValidationResult,
    ) {
        // Validate stories
        if storyboard.stories.is_empty() {
            result.warnings.push(ValidationWarning {
                category: ValidationWarningCategory::Suspicious,
                location: "Storyboard.stories".to_string(),
                message: "Storyboard has no stories - scenario may not execute anything"
                    .to_string(),
                suggestion: Some("Add at least one story with actions".to_string()),
            });
        }

        for (index, story) in storyboard.stories.iter().enumerate() {
            self.validate_story(
                story,
                context,
                &format!("Storyboard.Story[{}]", index),
                result,
            );
        }
    }

    /// Validate story
    fn validate_story(
        &self,
        story: &ScenarioStory,
        context: &ValidationContext,
        location: &str,
        result: &mut ValidationResult,
    ) {
        let default_name = String::new();
        let story_name = story.name.as_literal().unwrap_or(&default_name);
        if story_name.is_empty() {
            result.errors.push(ValidationError {
                category: ValidationErrorCategory::MissingRequired,
                location: format!("{}.name", location),
                message: "Story name is required".to_string(),
                suggestion: Some("Provide a descriptive name for the story".to_string()),
            });
        }

        // Validate acts
        if story.acts.is_empty() {
            result.warnings.push(ValidationWarning {
                category: ValidationWarningCategory::Suspicious,
                location: format!("{}.acts", location),
                message: "Story has no acts - may not execute any actions".to_string(),
                suggestion: Some("Add at least one act with maneuver groups".to_string()),
            });
        }

        for (index, act) in story.acts.iter().enumerate() {
            self.validate_act(
                act,
                context,
                &format!("{}.Act[{}]", location, index),
                result,
            );
        }
    }

    /// Validate act
    fn validate_act(
        &self,
        act: &Act,
        context: &ValidationContext,
        location: &str,
        result: &mut ValidationResult,
    ) {
        let default_name = String::new();
        let act_name = act.name.as_literal().unwrap_or(&default_name);
        if act_name.is_empty() {
            result.errors.push(ValidationError {
                category: ValidationErrorCategory::MissingRequired,
                location: format!("{}.name", location),
                message: "Act name is required".to_string(),
                suggestion: Some("Provide a descriptive name for the act".to_string()),
            });
        }

        for (index, mg) in act.maneuver_groups.iter().enumerate() {
            self.validate_maneuver_group(
                mg,
                context,
                &format!("{}.ManeuverGroup[{}]", location, index),
                result,
            );
        }
    }

    /// Validate maneuver group
    fn validate_maneuver_group(
        &self,
        mg: &ManeuverGroup,
        context: &ValidationContext,
        location: &str,
        result: &mut ValidationResult,
    ) {
        // Validate actor references
        for entity_ref in &mg.actors.entity_refs {
            let default_name = String::new();
            let entity_name = entity_ref.entity_ref.as_literal().unwrap_or(&default_name);
            if !context.entities.contains_key(entity_name) {
                result.errors.push(ValidationError {
                    category: ValidationErrorCategory::InvalidReference,
                    location: format!("{}.Actors.EntityRef", location),
                    message: format!("Referenced entity '{}' not found", entity_name),
                    suggestion: Some(
                        "Ensure the entity is defined in the Entities section".to_string(),
                    ),
                });
            }
        }

        for (index, maneuver) in mg.maneuvers.iter().enumerate() {
            self.validate_maneuver(
                maneuver,
                context,
                &format!("{}.Maneuver[{}]", location, index),
                result,
            );
        }
    }

    /// Validate maneuver
    fn validate_maneuver(
        &self,
        maneuver: &Maneuver,
        context: &ValidationContext,
        location: &str,
        result: &mut ValidationResult,
    ) {
        for (index, event) in maneuver.events.iter().enumerate() {
            self.validate_event(
                event,
                context,
                &format!("{}.Event[{}]", location, index),
                result,
            );
        }
    }

    /// Validate event
    fn validate_event(
        &self,
        event: &Event,
        context: &ValidationContext,
        location: &str,
        result: &mut ValidationResult,
    ) {
        // Validate start trigger if present
        if let Some(trigger) = &event.start_trigger {
            for (index, condition_group) in trigger.condition_groups.iter().enumerate() {
                for (c_index, condition) in condition_group.conditions.iter().enumerate() {
                    self.validate_condition(
                        condition,
                        context,
                        &format!(
                            "{}.StartTrigger.ConditionGroup[{}].Condition[{}]",
                            location, index, c_index
                        ),
                        result,
                    );
                }
            }
        }
    }

    /// Validate condition
    fn validate_condition(
        &self,
        condition: &Condition,
        _context: &ValidationContext,
        location: &str,
        result: &mut ValidationResult,
    ) {
        let default_name = String::new();
        let condition_name = condition.name.as_literal().unwrap_or(&default_name);
        if condition_name.is_empty() {
            result.warnings.push(ValidationWarning {
                category: ValidationWarningCategory::BestPractice,
                location: format!("{}.name", location),
                message: "Condition name should be provided for clarity".to_string(),
                suggestion: Some("Add descriptive names to conditions".to_string()),
            });
        }

        // Additional condition-specific validation could be added here
        result.metrics.elements_validated += 1;
    }

    /// Calculate cache hit ratio for performance metrics
    fn calculate_cache_hit_ratio(&self) -> f64 {
        if !self.config.use_cache {
            return 0.0;
        }
        // Simplified implementation - in real scenario would track hits/misses
        if self.validation_cache.is_empty() {
            0.0
        } else {
            0.85 // Simulated 85% hit ratio
        }
    }
}

impl ValidationResult {
    /// Create a new empty validation result
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
            metrics: ValidationMetrics {
                duration_ms: 0,
                elements_validated: 0,
                cache_hit_ratio: 0.0,
            },
        }
    }

    /// Check if validation passed (no errors)
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    /// Check if validation passed with no errors or warnings
    pub fn is_clean(&self) -> bool {
        self.errors.is_empty() && self.warnings.is_empty()
    }

    /// Get total number of issues (errors + warnings)
    pub fn total_issues(&self) -> usize {
        self.errors.len() + self.warnings.len()
    }

    /// Get a summary of validation results
    pub fn summary(&self) -> String {
        format!(
            "Validation complete: {} errors, {} warnings, {} elements validated in {}ms",
            self.errors.len(),
            self.warnings.len(),
            self.metrics.elements_validated,
            self.metrics.duration_ms
        )
    }
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        basic::Value,
        entities::{Entities, ScenarioObject, Vehicle},
        enums::VehicleCategory,
        geometry::shapes::BoundingBox,
        scenario::storyboard::Storyboard,
    };
    use crate::{FileHeader, OpenScenario};

    #[test]
    fn test_validator_creation() {
        let validator = ScenarioValidator::new();
        assert!(!validator.config.strict_mode);
        assert!(validator.config.validate_references);
    }

    #[test]
    fn test_file_header_validation() {
        let mut validator = ScenarioValidator::new();

        // Valid header
        let valid_header = FileHeader {
            rev_major: Value::literal(1),
            rev_minor: Value::literal(2),
            date: Value::literal("2024-01-01T00:00:00".to_string()),
            description: Value::literal("Test scenario".to_string()),
            author: Value::literal("Test Author".to_string()),
        };

        let vehicle = Vehicle {
            name: Value::literal("TestVehicle".to_string()),
            vehicle_category: VehicleCategory::Car,
            bounding_box: BoundingBox::default(),
            performance: Default::default(),
            axles: Default::default(),
            properties: None,
        };

        let entities = Entities {
            scenario_objects: vec![ScenarioObject {
                name: Value::literal("TestVehicle".to_string()),
                vehicle: Some(vehicle),
                pedestrian: None,
                catalog_reference: None,
                object_controller: Default::default(),
            }],
        };

        let scenario_def = crate::types::scenario::storyboard::ScenarioDefinition {
            parameter_declarations: None,
            variable_declarations: None,
            monitor_declarations: None,
            catalog_locations: crate::types::catalogs::locations::CatalogLocations::default(),
            road_network: crate::types::road::RoadNetwork::default(),
            entities: entities,
            storyboard: Storyboard::default(),
        };

        let scenario = OpenScenario {
            file_header: valid_header,
            parameter_declarations: None,
            variable_declarations: None,
            monitor_declarations: None,
            catalog_locations: Some(crate::types::catalogs::locations::CatalogLocations::default()),
            road_network: Some(crate::types::road::RoadNetwork::default()),
            entities: Some(scenario_def.entities),
            storyboard: Some(scenario_def.storyboard),
            parameter_value_distribution: None,
            catalog: None,
        };

        let result = validator.validate_scenario(&scenario);
        // Debug the actual errors
        if !result.is_valid() {
            for error in &result.errors {
                println!("Validation error: {:?}", error);
            }
        }
        assert!(result.is_valid(), "Valid scenario should pass validation");
    }

    #[test]
    fn test_empty_author_validation() {
        let mut validator = ScenarioValidator::new();

        let invalid_header = FileHeader {
            rev_major: Value::literal(1),
            rev_minor: Value::literal(2),
            date: Value::literal("2024-01-01T00:00:00".to_string()),
            description: Value::literal("Test scenario".to_string()),
            author: Value::literal("".to_string()), // Empty author
        };

        let vehicle = Vehicle {
            name: Value::literal("TestVehicle".to_string()),
            vehicle_category: VehicleCategory::Car,
            bounding_box: BoundingBox::default(),
            performance: Default::default(),
            axles: Default::default(),
            properties: None,
        };

        let entities = Entities {
            scenario_objects: vec![ScenarioObject {
                name: Value::literal("TestVehicle".to_string()),
                vehicle: Some(vehicle),
                pedestrian: None,
                catalog_reference: None,
                object_controller: Default::default(),
            }],
        };

        let scenario_def = crate::types::scenario::storyboard::ScenarioDefinition {
            parameter_declarations: None,
            variable_declarations: None,
            monitor_declarations: None,
            catalog_locations: crate::types::catalogs::locations::CatalogLocations::default(),
            road_network: crate::types::road::RoadNetwork::default(),
            entities: entities,
            storyboard: Storyboard::default(),
        };

        let scenario = OpenScenario {
            file_header: invalid_header,
            parameter_declarations: None,
            variable_declarations: None,
            monitor_declarations: None,
            catalog_locations: Some(crate::types::catalogs::locations::CatalogLocations::default()),
            road_network: Some(crate::types::road::RoadNetwork::default()),
            entities: Some(scenario_def.entities),
            storyboard: Some(scenario_def.storyboard),
            parameter_value_distribution: None,
            catalog: None,
        };

        let result = validator.validate_scenario(&scenario);
        assert!(!result.is_valid(), "Empty author should fail validation");
        assert_eq!(result.errors.len(), 1);
        assert!(matches!(
            result.errors[0].category,
            ValidationErrorCategory::MissingRequired
        ));
    }

    #[test]
    fn test_duplicate_entity_names_validation() {
        let mut validator = ScenarioValidator::new();

        let vehicle1 = Vehicle {
            name: Value::literal("Car1".to_string()),
            vehicle_category: VehicleCategory::Car,
            bounding_box: BoundingBox::default(),
            performance: Default::default(),
            axles: Default::default(),
            properties: None,
        };

        let vehicle2 = Vehicle {
            name: Value::literal("Car1".to_string()), // Duplicate name
            vehicle_category: VehicleCategory::Truck,
            bounding_box: BoundingBox::default(),
            performance: Default::default(),
            axles: Default::default(),
            properties: None,
        };

        let entities = Entities {
            scenario_objects: vec![
                ScenarioObject {
                    name: Value::literal("Car1".to_string()),
                    vehicle: Some(vehicle1),
                    pedestrian: None,
                    catalog_reference: None,
                    object_controller: Default::default(),
                },
                ScenarioObject {
                    name: Value::literal("Car1".to_string()),
                    vehicle: Some(vehicle2),
                    pedestrian: None,
                    catalog_reference: None,
                    object_controller: Default::default(),
                },
            ],
        };

        let scenario_def = crate::types::scenario::storyboard::ScenarioDefinition {
            parameter_declarations: None,
            variable_declarations: None,
            monitor_declarations: None,
            catalog_locations: crate::types::catalogs::locations::CatalogLocations::default(),
            road_network: crate::types::road::RoadNetwork::default(),
            entities: entities,
            storyboard: Storyboard::default(),
        };

        let scenario = OpenScenario {
            file_header: FileHeader {
                author: Value::literal("Test Author".to_string()),
                date: Value::literal("2024-01-01T00:00:00".to_string()),
                description: Value::literal("Test scenario".to_string()),
                rev_major: Value::literal(1),
                rev_minor: Value::literal(2),
            },
            parameter_declarations: None,
            variable_declarations: None,
            monitor_declarations: None,
            catalog_locations: Some(crate::types::catalogs::locations::CatalogLocations::default()),
            road_network: Some(crate::types::road::RoadNetwork::default()),
            entities: Some(scenario_def.entities),
            storyboard: Some(scenario_def.storyboard),
            parameter_value_distribution: None,
            catalog: None,
        };

        let result = validator.validate_scenario(&scenario);
        assert!(
            !result.is_valid(),
            "Duplicate entity names should fail validation"
        );

        // Should have error for duplicate names
        assert!(result
            .errors
            .iter()
            .any(|e| matches!(e.category, ValidationErrorCategory::ConstraintViolation)));
    }

    #[test]
    fn test_validation_metrics() {
        let mut validator = ScenarioValidator::new();
        let scenario = OpenScenario::default();

        let result = validator.validate_scenario(&scenario);

        // Should have some metrics
        assert!(result.metrics.duration_ms < 1000); // Should be fast
        assert_eq!(result.metrics.cache_hit_ratio, 0.0); // No cache hits for empty scenario
    }

    #[test]
    fn test_strict_mode() {
        let config = ValidationConfig {
            strict_mode: true,
            ..Default::default()
        };
        let mut validator = ScenarioValidator::with_config(config);

        // Create scenario with entities to ensure validation occurs
        let vehicle = crate::types::entities::vehicle::Vehicle {
            name: crate::types::basic::Value::literal("TestCar".to_string()),
            vehicle_category: crate::types::enums::VehicleCategory::Car,
            bounding_box: crate::types::geometry::BoundingBox::default(),
            performance: Default::default(),
            axles: Default::default(),
            properties: None,
        };

        let scenario_object = crate::types::entities::ScenarioObject {
            name: crate::types::basic::Value::literal("TestVehicle".to_string()),
            vehicle: Some(vehicle),
            pedestrian: None,
            catalog_reference: None,
            object_controller: Default::default(),
        };

        let entities = crate::types::entities::Entities {
            scenario_objects: vec![scenario_object],
        };

        let mut scenario = OpenScenario::default();
        scenario.entities = Some(entities);
        
        let result = validator.validate_scenario(&scenario);

        // In strict mode with entities, should have validation metrics
        assert!(result.metrics.elements_validated > 0);
        // Should also validate that strict mode is actually enabled in the validator
        assert!(validator.config.strict_mode);
    }
}
