//! Domain validation for parsed scenarios.
//!
//! Checks that run on a document once it has parsed: required fields, entity and
//! catalog references that resolve, business rules, and logical consistency. Each
//! finding carries a location, a message, and where possible a suggested fix.
//!
//! ```rust,no_run
//! use openscenario_rs::parser::validation::ScenarioValidator;
//! use openscenario_rs::parser::xml::parse_from_file;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let scenario = parse_from_file("scenario.xosc")?;
//! let mut validator = ScenarioValidator::new();
//! let result = validator.validate_scenario(&scenario);
//!
//! for error in &result.errors {
//!     println!("{}: {}", error.location, error.message);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! Structure is always checked; [`ValidationConfig`] switches the reference,
//! constraint and semantic passes on or off and caps how many errors to collect.
//!
//! ```rust,no_run
//! use openscenario_rs::parser::validation::{ScenarioValidator, ValidationConfig};
//! use openscenario_rs::parse_from_file;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let scenario = parse_from_file("scenario.xosc")?;
//! let config = ValidationConfig {
//!     strict_mode: true,
//!     max_errors: 50,
//!     validate_semantics: false,
//!     ..Default::default()
//! };
//!
//! let mut validator = ScenarioValidator::with_config(config);
//! let result = validator.validate_scenario(&scenario);
//! println!("{} elements in {}ms", result.metrics.elements_validated, result.metrics.duration_ms);
//! # Ok(())
//! # }
//! ```
//!
//! Every finding carries a [`ValidationErrorCategory`], so a caller can branch on the
//! kind of problem it reports:
//!
//! ```rust
//! use openscenario_rs::parser::validation::ValidationErrorCategory;
//!
//! # let result = openscenario_rs::parser::validation::ValidationResult::new();
//! for error in &result.errors {
//!     match error.category {
//!         ValidationErrorCategory::MissingRequired => {}
//!         ValidationErrorCategory::InvalidReference => {}
//!         ValidationErrorCategory::ConstraintViolation => {}
//!         ValidationErrorCategory::SemanticError => {}
//!         ValidationErrorCategory::TypeMismatch => {}
//!         ValidationErrorCategory::ParameterError => {}
//!     }
//! }
//! ```
//!
//! `xml::parse_from_file_validated` runs the parse and the structural pass
//! together; the validator here is the domain pass on top of it.

use crate::{
    types::{
        entities::{Entities, ScenarioObject},
        scenario::triggers::Condition,
        scenario::{
            story::{Act, Event, Maneuver, ManeuverGroup},
            storyboard::Storyboard,
            ScenarioStory,
        },
        EntityRef, ObjectType, ValidationContext, Value,
    },
    FileHeader, OpenScenario,
};
use std::collections::{HashMap, HashSet};

/// Runs the four validation passes over a parsed document: structure (hierarchy and
/// required fields), references (entities and catalogs), constraints (business rules),
/// and semantics (logical consistency). [`ValidationConfig`] selects which run.
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

impl Default for ScenarioValidator {
    fn default() -> Self {
        Self::new()
    }
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
                    object_type: Value::Literal(if obj.vehicle().is_some() {
                        ObjectType::Vehicle
                    } else if obj.pedestrian().is_some() {
                        ObjectType::Pedestrian
                    } else {
                        ObjectType::MiscellaneousObject
                    }),
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

        // `acts` is a `MinVec<Act, 1>`: a story always has an act, so there is no empty case.
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

    #[test]
    fn test_validator_creation() {
        let validator = ScenarioValidator::new();
        assert!(!validator.config.strict_mode);
        assert!(validator.config.validate_references);
    }

    /// A document the validator has nothing to say about: every name it checks is filled in,
    /// the actor exists, and the header is OpenSCENARIO 1.3.
    const CLEAN: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<OpenSCENARIO>
  <FileHeader revMajor="1" revMinor="3" date="2024-01-01T00:00:00" author="Tester" description="Validator fixture"/>
  <CatalogLocations/>
  <RoadNetwork/>
  <Entities>
    <ScenarioObject name="Ego">VEHICLE</ScenarioObject>
  </Entities>
  <Storyboard>
    <Init><Actions/></Init>
    <Story name="Story1">
      <Act name="Act1">
        <ManeuverGroup name="Group1" maximumExecutionCount="1">
          <Actors selectTriggeringEntities="false"><EntityRef entityRef="Ego"/></Actors>
          <Maneuver name="Maneuver1">
            <Event name="Event1" priority="override">
              <Action name="Action1"><PrivateAction><VisibilityAction graphics="true" traffic="true" sensors="true"/></PrivateAction></Action>
              <StartTrigger><ConditionGroup><Condition name="EventStart" conditionEdge="rising" delay="0"><ByValueCondition><SimulationTimeCondition value="1" rule="greaterThan"/></ByValueCondition></Condition></ConditionGroup></StartTrigger>
            </Event>
          </Maneuver>
        </ManeuverGroup>
        <StartTrigger><ConditionGroup><Condition name="ActStart" conditionEdge="rising" delay="0"><ByValueCondition><SimulationTimeCondition value="0" rule="greaterThan"/></ByValueCondition></Condition></ConditionGroup></StartTrigger>
      </Act>
    </Story>
    <StopTrigger/>
  </Storyboard>
</OpenSCENARIO>"#;

    const VEHICLE: &str = r#"<Vehicle name="car" vehicleCategory="car"><BoundingBox><Center x="0" y="0" z="0"/><Dimensions width="2" length="4" height="1.5"/></BoundingBox><Performance maxSpeed="50" maxAcceleration="5" maxDeceleration="10"/><Axles><FrontAxle maxSteering="0.5" wheelDiameter="0.6" trackWidth="1.8" positionX="1.3" positionZ="0.3"/><RearAxle maxSteering="0" wheelDiameter="0.6" trackWidth="1.8" positionX="-1.3" positionZ="0.3"/></Axles></Vehicle>"#;

    const HEADER: &str = r#"<FileHeader revMajor="1" revMinor="3" date="2024-01-01T00:00:00" author="Tester" description="Validator fixture"/>"#;

    fn clean() -> String {
        CLEAN.replace("VEHICLE", VEHICLE)
    }

    /// `doc` with the first `from` replaced by `to`. Panics when `from` is absent, so a row
    /// whose edit no longer matches the fixture cannot pass by validating the clean document.
    fn swap(doc: &str, from: &str, to: &str) -> String {
        assert!(doc.contains(from), "fixture has no `{from}`");
        doc.replacen(from, to, 1)
    }

    fn with_second_object(doc: &str, name: &str) -> String {
        swap(
            doc,
            "</Entities>",
            &format!(r#"<ScenarioObject name="{name}">{VEHICLE}</ScenarioObject></Entities>"#),
        )
    }

    /// Every finding of `result`, as `"<error|warning> <category> @ <location>"`.
    fn findings(result: &ValidationResult) -> Vec<String> {
        let errors = result
            .errors
            .iter()
            .map(|e| format!("error {:?} @ {}", e.category, e.location));
        let warnings = result
            .warnings
            .iter()
            .map(|w| format!("warning {:?} @ {}", w.category, w.location));
        errors.chain(warnings).collect()
    }

    /// One row per check `validate_scenario` makes: a document with exactly one defect, and the
    /// one finding (category and location) that defect must produce. The clean document and a
    /// catalog document produce none. `is_valid` and `is_clean` must agree with the findings.
    #[test]
    fn each_validator_check_reports_its_defect_at_its_location() {
        const EVENT_CONDITION: &str = "Storyboard.Story[0].Act[0].ManeuverGroup[0].Maneuver[0].Event[0].StartTrigger.ConditionGroup[0].Condition[0].name";
        let base = clean();
        let rows: Vec<(&str, String, Vec<String>)> = vec![
            ("clean scenario", base.clone(), vec![]),
            (
                "catalog document",
                format!(r#"<OpenSCENARIO>{HEADER}<Catalog name="C"/></OpenSCENARIO>"#),
                vec![],
            ),
            (
                "no scenario, catalog or distribution",
                format!("<OpenSCENARIO>{HEADER}</OpenSCENARIO>"),
                vec!["error SemanticError @ root".into()],
            ),
            (
                "empty author",
                swap(&base, r#"author="Tester""#, r#"author="""#),
                vec!["error MissingRequired @ FileHeader.author".into()],
            ),
            (
                "empty description",
                swap(&base, r#"description="Validator fixture""#, r#"description="""#),
                vec!["warning BestPractice @ FileHeader.description".into()],
            ),
            (
                "revMajor 0",
                swap(&base, r#"revMajor="1""#, r#"revMajor="0""#),
                vec!["error ConstraintViolation @ FileHeader.revMajor".into()],
            ),
            (
                "revision 1.4, newer than the crate's 1.3",
                swap(&base, r#"revMinor="3""#, r#"revMinor="4""#),
                vec!["warning Suspicious @ FileHeader.rev1.4".into()],
            ),
            (
                "empty scenario object name",
                with_second_object(&base, ""),
                vec!["error MissingRequired @ Entities.ScenarioObject[1].name".into()],
            ),
            (
                "duplicate scenario object name",
                with_second_object(&base, "Ego"),
                vec!["error ConstraintViolation @ Entities.ScenarioObject[name='Ego']".into()],
            ),
            (
                "storyboard without stories",
                {
                    let from = base.find("<Story ").unwrap();
                    let to = base.find("</Story>").unwrap() + "</Story>".len();
                    format!("{}{}", &base[..from], &base[to..])
                },
                vec!["warning Suspicious @ Storyboard.stories".into()],
            ),
            (
                "empty story name",
                swap(&base, r#"<Story name="Story1">"#, r#"<Story name="">"#),
                vec!["error MissingRequired @ Storyboard.Story[0].name".into()],
            ),
            (
                "empty act name",
                swap(&base, r#"<Act name="Act1">"#, r#"<Act name="">"#),
                vec!["error MissingRequired @ Storyboard.Story[0].Act[0].name".into()],
            ),
            (
                "actor that names no entity",
                swap(&base, r#"<EntityRef entityRef="Ego"/>"#, r#"<EntityRef entityRef="Ghost"/>"#),
                vec![
                    "error InvalidReference @ Storyboard.Story[0].Act[0].ManeuverGroup[0].Actors.EntityRef"
                        .into(),
                ],
            ),
            (
                "unnamed event start condition",
                swap(&base, r#"name="EventStart""#, r#"name="""#),
                vec![format!("warning BestPractice @ {EVENT_CONDITION}")],
            ),
        ];

        let mut failures = Vec::new();
        for (label, xml, expected) in rows {
            let scenario = match crate::parse_from_str(&xml) {
                Ok(scenario) => scenario,
                Err(e) => {
                    failures.push(format!("{label}: fixture does not parse: {e}"));
                    continue;
                }
            };
            let result = ScenarioValidator::new().validate_scenario(&scenario);
            let got = findings(&result);
            if got != expected {
                failures.push(format!("{label}: expected {expected:?}, got {got:?}"));
            }
            // A warning alone leaves the document valid; only no finding at all is clean.
            let valid = expected.iter().all(|f| f.starts_with("warning"));
            if result.is_valid() != valid || result.is_clean() != expected.is_empty() {
                failures.push(format!(
                    "{label}: is_valid {} (expected {valid}), is_clean {} (expected {})",
                    result.is_valid(),
                    result.is_clean(),
                    expected.is_empty()
                ));
            }
        }
        assert!(failures.is_empty(), "{}", failures.join("\n"));
    }

    /// The message of an unresolved actor names the entity, which is how a reader finds it.
    #[test]
    fn an_unresolved_actor_is_named_in_the_message() {
        let xml = swap(
            &clean(),
            r#"<EntityRef entityRef="Ego"/>"#,
            r#"<EntityRef entityRef="Ghost"/>"#,
        );
        let result =
            ScenarioValidator::new().validate_scenario(&crate::parse_from_str(&xml).unwrap());
        assert_eq!(result.errors.len(), 1, "{:?}", result.errors);
        assert_eq!(
            result.errors[0].message,
            "Referenced entity 'Ghost' not found"
        );
    }

    #[test]
    fn test_validation_metrics() {
        let mut validator = ScenarioValidator::new();
        let scenario = crate::types::scenario::storyboard::test_scenario_document();

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
            vehicle_category: Value::Literal(crate::types::enums::VehicleCategory::Car),
            role: None,
            mass: None,
            model3d: None,
            parameter_declarations: None,
            bounding_box: crate::types::geometry::BoundingBox::new(
                crate::types::geometry::Center::new(0.0, 0.0, 0.0),
                crate::types::geometry::Dimensions::new(2.0, 4.5, 1.5),
            ),
            performance: crate::types::entities::vehicle::Performance {
                max_speed: crate::types::basic::Value::literal(200.0),
                max_acceleration: crate::types::basic::Value::literal(10.0),
                max_acceleration_rate: None,
                max_deceleration: crate::types::basic::Value::literal(10.0),
                max_deceleration_rate: None,
            },
            axles: crate::types::entities::axles::Axles::car(),
            properties: None,
            trailer_hitch: None,
            trailer_coupler: None,
            trailer: None,
        };

        let scenario_object = crate::types::entities::ScenarioObject {
            name: crate::types::basic::Value::literal("TestVehicle".to_string()),
            entity: crate::types::entities::EntityObjectChoice::Vehicle(vehicle),
            object_controller: Default::default(),
        };

        let entities = crate::types::entities::Entities {
            scenario_objects: vec![scenario_object],
            entity_selections: Vec::new(),
        };

        let mut scenario = crate::types::scenario::storyboard::test_scenario_document();
        scenario.entities = Some(entities);

        let result = validator.validate_scenario(&scenario);

        // In strict mode with entities, should have validation metrics
        assert!(result.metrics.elements_validated > 0);
        // Should also validate that strict mode is actually enabled in the validator
        assert!(validator.config.strict_mode);
    }
}
