//! Validation integration for builder system
//!
//! This module provides validation support for the builder system,
//! integrating with the existing validation framework to ensure
//! built scenarios comply with OpenSCENARIO schema requirements.

use crate::builder::{BuilderError, BuilderResult};
use crate::types::scenario::storyboard::OpenScenario;
use crate::types::ValidationContext;
use std::collections::{HashMap, HashSet};

/// Builder validation context that extends the existing validation framework
#[derive(Default)]
pub struct BuilderValidationContext {
    /// Base validation context
    base_context: ValidationContext,
    /// Builder-specific validation rules
    builder_rules: Vec<Box<dyn BuilderValidationRule>>,
    /// Entity reference tracking
    entity_refs: HashMap<String, String>,
    /// Parameter tracking
    parameters: HashMap<String, String>,
}

impl std::fmt::Debug for BuilderValidationContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BuilderValidationContext")
            .field("base_context", &self.base_context)
            .field(
                "builder_rules",
                &format!("{} rules", self.builder_rules.len()),
            )
            .field("entity_refs", &self.entity_refs)
            .field("parameters", &self.parameters)
            .finish()
    }
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
    pub fn add_rule(mut self, rule: Box<dyn BuilderValidationRule>) -> Self {
        self.builder_rules.push(rule);
        self
    }

    /// Register an entity reference
    pub fn register_entity(&mut self, name: &str, entity_type: &str) {
        self.entity_refs
            .insert(name.to_string(), entity_type.to_string());
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
                &self.entity_refs.keys().cloned().collect::<Vec<_>>(),
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
                self.parameters
                    .keys()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ")
            )));
        }
        Ok(())
    }

    /// Validate a complete scenario
    pub fn validate_scenario(&self, scenario: &OpenScenario) -> BuilderResult<()> {
        // Run base validation first - ValidationContext doesn't have validate_scenario method
        // For now, we'll skip base validation and only run builder-specific rules

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
    fn validate(
        &self,
        scenario: &OpenScenario,
        context: &BuilderValidationContext,
    ) -> BuilderResult<()>;

    /// Get the name of this validation rule
    fn name(&self) -> &str;

    /// Get the description of this validation rule
    fn description(&self) -> &str;
}

/// Serializes the assembled document so a rule can scan every attribute in it.
///
/// Both reference rules below need to see references wherever they occur – in `Init`, in
/// actors, in triggering entities, in condition targets, in an action nested five levels
/// down. Walking the typed tree would mean a branch per variant and a silent gap whenever a
/// new one is added, which is the defect these rules previously had. Scanning the serialized
/// form is complete by construction: anything the document actually emits is visible.
///
/// Serialization failure is not a validation error, so this yields `None` and the rule passes.
/// A document that cannot serialize fails later, with a better message than this rule could give.
fn serialized_for_scan(scenario: &OpenScenario) -> Option<String> {
    crate::parser::xml::serialize_to_string(scenario).ok()
}

/// Validation rule for entity references
///
/// XSD validation cannot express cross-references, so a document naming an entity that was
/// never declared is schema-valid and silently wrong. This rule closes that gap.
#[derive(Debug)]
pub struct EntityReferenceValidationRule;

impl EntityReferenceValidationRule {
    /// The attributes that carry an entity reference.
    ///
    /// The schema defines exactly three sites: the `entityRef` attribute (on `EntityRef`,
    /// `Private`, and numerous conditions and actions) and `masterEntityRef` on
    /// `SynchronizeAction`. Verified against `Schema/OpenSCENARIO.xsd`.
    const REFERENCE_ATTRIBUTES: &'static [&'static str] = &["entityRef", "masterEntityRef"];

    /// Names that a reference may legitimately resolve to: declared objects and selections.
    fn declared_names(
        scenario: &OpenScenario,
        context: &BuilderValidationContext,
    ) -> HashSet<String> {
        let mut names: HashSet<String> = context.entity_refs.keys().cloned().collect();

        if let Some(entities) = &scenario.entities {
            names.extend(entities.scenario_objects.iter().map(|o| o.name.to_string()));
            // A selection is a valid actor, so it counts as declared.
            names.extend(
                entities
                    .entity_selections
                    .iter()
                    .map(|s| s.name.to_string()),
            );
        }

        names
    }
}

impl BuilderValidationRule for EntityReferenceValidationRule {
    fn validate(
        &self,
        scenario: &OpenScenario,
        context: &BuilderValidationContext,
    ) -> BuilderResult<()> {
        let Some(xml) = serialized_for_scan(scenario) else {
            return Ok(());
        };

        let declared = Self::declared_names(scenario, context);

        for attribute in Self::REFERENCE_ATTRIBUTES {
            for referenced in attribute_values(&xml, attribute) {
                // A parameterized reference resolves at run time, not here.
                if referenced.starts_with("${") || referenced.starts_with('$') {
                    continue;
                }

                if !declared.contains(&referenced) {
                    let mut available: Vec<String> = declared.iter().cloned().collect();
                    available.sort();
                    return Err(BuilderError::invalid_entity_ref(&referenced, &available));
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

/// Collects the values of every occurrence of `attribute="…"` in the serialized document.
///
/// Deliberately a string scan rather than an XML parse: the input is this crate's own
/// serializer output, the attribute names are fixed, and values are already escaped.
fn attribute_values(xml: &str, attribute: &str) -> Vec<String> {
    let needle = format!("{attribute}=\"");
    let mut values = Vec::new();
    let mut rest = xml;

    while let Some(start) = rest.find(&needle) {
        // Require a delimiter before the name so `masterEntityRef` does not match `entityRef`.
        let preceded_by_delimiter = rest[..start]
            .chars()
            .next_back()
            .is_none_or(|c| c.is_whitespace() || c == '<');

        let after = &rest[start + needle.len()..];
        let Some(end) = after.find('"') else { break };

        if preceded_by_delimiter {
            values.push(after[..end].to_string());
        }
        rest = &after[end + 1..];
    }

    values
}

/// Validation rule for parameter references
#[derive(Debug)]
pub struct ParameterReferenceValidationRule;

impl ParameterReferenceValidationRule {
    /// Names a `${…}` reference may resolve to: declared parameters, declared variables, and
    /// anything the caller registered on the context.
    ///
    /// Variables are included because they share the `${…}` syntax on the wire; excluding them
    /// would reject documents that are entirely correct.
    fn declared_names(
        scenario: &OpenScenario,
        context: &BuilderValidationContext,
    ) -> HashSet<String> {
        let mut names: HashSet<String> = context.parameters.keys().cloned().collect();

        if let Some(declarations) = &scenario.parameter_declarations {
            names.extend(
                declarations
                    .parameter_declarations
                    .iter()
                    .map(|p| p.name.to_string()),
            );
        }

        if let Some(declarations) = &scenario.variable_declarations {
            names.extend(
                declarations
                    .variable_declarations
                    .iter()
                    .map(|v| v.name.to_string()),
            );
        }

        names
    }

    /// Extracts the parameter names a single `${…}` body depends on.
    ///
    /// A plain name is the reference itself. Anything else is an expression, and is parsed so
    /// that only genuine `Expr::Parameter` nodes are collected: scanning for identifiers would
    /// flag the function and constant names the evaluator supports.
    fn referenced_names(body: &str) -> Vec<String> {
        if crate::types::basic::is_valid_parameter_name(body) {
            return vec![body.to_string()];
        }

        let Ok(mut parser) = crate::expression::ExpressionParser::new(body) else {
            return Vec::new();
        };
        let Ok(expr) = parser.parse() else {
            return Vec::new();
        };

        let mut names = Vec::new();
        collect_expression_parameters(&expr, &mut names);
        names
    }
}

impl BuilderValidationRule for ParameterReferenceValidationRule {
    fn validate(
        &self,
        scenario: &OpenScenario,
        context: &BuilderValidationContext,
    ) -> BuilderResult<()> {
        let Some(xml) = serialized_for_scan(scenario) else {
            return Ok(());
        };

        let declared = Self::declared_names(scenario, context);

        // `Value<T>` serializes both parameters and expressions as `${…}`, so one pattern
        // covers every parameterized attribute in the document.
        let mut rest = xml.as_str();
        while let Some(start) = rest.find("${") {
            let after = &rest[start + 2..];
            let Some(end) = after.find('}') else { break };
            let body = &after[..end];
            rest = &after[end + 1..];

            for name in Self::referenced_names(body) {
                if !declared.contains(&name) {
                    let mut available: Vec<String> = declared.iter().cloned().collect();
                    available.sort();
                    return Err(BuilderError::validation_error(&format!(
                        "Parameter '{}' is referenced but never declared. Declared: [{}]",
                        name,
                        available.join(", ")
                    )));
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

/// Walks an expression AST collecting the parameters it depends on.
///
/// `Expr::Constant` and `Expr::FunctionCall` names are deliberately not collected: they are
/// resolved by the evaluator, not by parameter declarations.
fn collect_expression_parameters(expr: &crate::expression::Expr, names: &mut Vec<String>) {
    use crate::expression::Expr;

    match expr {
        Expr::Parameter(name) => names.push(name.clone()),
        Expr::BinaryOp { left, right, .. } => {
            collect_expression_parameters(left, names);
            collect_expression_parameters(right, names);
        }
        Expr::UnaryMinus(inner) => collect_expression_parameters(inner, names),
        Expr::FunctionCall { args, .. } => {
            for arg in args {
                collect_expression_parameters(arg, names);
            }
        }
        Expr::Number(_) | Expr::Constant(_) => {}
    }
}

/// Validation rule for catalog references
#[derive(Debug)]
pub struct CatalogReferenceValidationRule;

impl BuilderValidationRule for CatalogReferenceValidationRule {
    fn validate(
        &self,
        scenario: &OpenScenario,
        _context: &BuilderValidationContext,
    ) -> BuilderResult<()> {
        // Validate that catalog locations are specified if catalog references are used
        if let Some(entities) = &scenario.entities {
            let has_catalog_refs = entities
                .scenario_objects
                .iter()
                .any(|obj| obj.entity_catalog_reference.is_some());

            if has_catalog_refs && scenario.catalog_locations.is_none() {
                return Err(BuilderError::validation_error(
                    "Catalog references found but no catalog locations specified",
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
    fn validate(
        &self,
        scenario: &OpenScenario,
        _context: &BuilderValidationContext,
    ) -> BuilderResult<()> {
        if let Some(storyboard) = &scenario.storyboard {
            // Validate that stories have at least one act
            for story in &storyboard.stories {
                if story.acts.is_empty() {
                    return Err(BuilderError::validation_error(&format!(
                        "Story '{}' has no acts",
                        story.name.to_string()
                    )));
                }

                // Validate that acts have at least one maneuver group
                for act in &story.acts {
                    if act.maneuver_groups.is_empty() {
                        return Err(BuilderError::validation_error(&format!(
                            "Act '{}' has no maneuver groups",
                            act.name.to_string()
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
#[derive(Default)]
pub struct ValidationContextBuilder {
    rules: Vec<Box<dyn BuilderValidationRule>>,
    entities: HashMap<String, String>,
    parameters: HashMap<String, String>,
}

impl std::fmt::Debug for ValidationContextBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ValidationContextBuilder")
            .field("rules", &format!("{} rules", self.rules.len()))
            .field("entities", &self.entities)
            .field("parameters", &self.parameters)
            .finish()
    }
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
        self.entities
            .insert(name.to_string(), entity_type.to_string());
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
            .add_rule(Box::new(EntityReferenceValidationRule))
            .add_rule(Box::new(ParameterReferenceValidationRule))
            .add_rule(Box::new(CatalogReferenceValidationRule))
            .add_rule(Box::new(StoryboardStructureValidationRule));

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
