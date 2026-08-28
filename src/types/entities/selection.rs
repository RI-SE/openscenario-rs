//! Entity selection types for OpenSCENARIO scenarios
//!
//! This module provides types for selecting and managing entities in scenarios:
//! - EntitySelection: Main entity selection framework with selection criteria
//! - SelectedEntities: Container for selected entities with entity references
//! - EntityDistribution: Entity distribution system for probabilistic entity spawning
//! - EntityDistributionEntry: Individual distribution entry with entity reference and weight
//! - ScenarioObjectTemplate: Template system for scenario object creation
//! - ExternalObjectReference: Reference to external object definitions
//! - ByObjectType: Entity selection by object type (vehicle, pedestrian, etc.)
//! - ByType: Generic type-based selection criteria

use crate::types::basic::{Double, OSString};
use crate::types::controllers::ObjectController;
use crate::types::enums::ObjectType;
use crate::types::scenario::triggers::EntityRef;
use serde::{Deserialize, Serialize};

/// Main entity selection framework with selection criteria
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntitySelection {
    /// Selection by object type
    #[serde(rename = "ByType", skip_serializing_if = "Option::is_none")]
    pub by_type: Option<ByObjectType>,
}

/// Container for selected entities with entity references
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectedEntities {
    /// List of entity references
    #[serde(rename = "EntityRef")]
    pub entity_refs: Vec<EntityRef>,
}

/// Entity distribution system for probabilistic entity spawning
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityDistribution {
    /// List of distribution entries
    #[serde(rename = "EntityDistributionEntry")]
    pub entries: Vec<EntityDistributionEntry>,
}

/// Individual distribution entry with a scenario object template and weight
///
/// XSD `EntityDistributionEntry` (`:1162-1167`): sequence of required
/// `ScenarioObjectTemplate`; required attribute `@weight` (Double). There is
/// no `@entityRef` attribute in the schema.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityDistributionEntry {
    /// Probability weight for this entry
    #[serde(rename = "@weight")]
    pub weight: Double,

    /// Template describing the scenario object to spawn
    #[serde(rename = "ScenarioObjectTemplate")]
    pub scenario_object_template: ScenarioObjectTemplate,
}

/// Template system for scenario object creation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScenarioObjectTemplate {
    /// Name of the template
    #[serde(rename = "@name")]
    pub name: OSString,

    /// Object type for the template
    #[serde(rename = "@objectType")]
    pub object_type: ObjectType,

    /// External object reference (optional)
    #[serde(
        rename = "ExternalObjectReference",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_object_reference: Option<ExternalObjectReference>,

    /// Object controller configuration (optional, may occur multiple times)
    #[serde(
        rename = "ObjectController",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub object_controller: Vec<ObjectController>,
}

/// Reference to external object definitions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExternalObjectReference {
    /// Name of the object within the external file
    #[serde(rename = "@name")]
    pub name: OSString,
}

/// Entity selection by object type (vehicle, pedestrian, etc.)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ByObjectType {
    /// Type of object to select
    #[serde(rename = "@type")]
    pub object_type: ObjectType,
}

/// Generic type-based selection criteria
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ByType {
    /// Type specification for selection
    #[serde(rename = "@objectType")]
    pub type_spec: ObjectType,
}

// Default implementations
impl Default for EntitySelection {
    fn default() -> Self {
        Self {
            by_type: Some(ByObjectType::default()),
        }
    }
}

impl Default for SelectedEntities {
    fn default() -> Self {
        Self {
            entity_refs: vec![EntityRef::default()],
        }
    }
}

impl Default for EntityDistribution {
    fn default() -> Self {
        Self {
            entries: vec![EntityDistributionEntry::default()],
        }
    }
}

impl Default for EntityDistributionEntry {
    fn default() -> Self {
        Self {
            weight: Double::literal(1.0),
            scenario_object_template: ScenarioObjectTemplate::default(),
        }
    }
}

impl Default for ScenarioObjectTemplate {
    fn default() -> Self {
        Self {
            name: OSString::literal("DefaultTemplate".to_string()),
            object_type: ObjectType::Vehicle,
            external_object_reference: None,
            object_controller: Vec::new(),
        }
    }
}

impl Default for ExternalObjectReference {
    fn default() -> Self {
        Self {
            name: OSString::literal("DefaultObject".to_string()),
        }
    }
}

impl Default for ByObjectType {
    fn default() -> Self {
        Self {
            object_type: ObjectType::Vehicle,
        }
    }
}

impl Default for ByType {
    fn default() -> Self {
        Self {
            type_spec: ObjectType::Vehicle,
        }
    }
}

// Implementation methods
impl EntitySelection {
    /// Create a new entity selection by object type
    pub fn by_object_type(object_type: ObjectType) -> Self {
        Self {
            by_type: Some(ByObjectType { object_type }),
        }
    }
}

impl SelectedEntities {
    /// Create a new selected entities container
    pub fn new() -> Self {
        Self {
            entity_refs: Vec::new(),
        }
    }

    /// Create selected entities from a list of entity names
    pub fn from_names(names: Vec<impl Into<String>>) -> Self {
        Self {
            entity_refs: names
                .into_iter()
                .map(|name| EntityRef::new(name.into()))
                .collect(),
        }
    }

    /// Add an entity reference
    pub fn add_entity(&mut self, entity_name: impl Into<String>) {
        self.entity_refs.push(EntityRef::new(entity_name.into()));
    }

    /// Get the number of selected entities
    pub fn count(&self) -> usize {
        self.entity_refs.len()
    }
}

impl EntityDistribution {
    /// Create a new entity distribution
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Add a distribution entry from a template name and object type
    pub fn add_entry(&mut self, template_name: impl Into<String>, weight: f64) {
        self.entries.push(EntityDistributionEntry {
            weight: Double::literal(weight),
            scenario_object_template: ScenarioObjectTemplate::new(
                template_name,
                ObjectType::Vehicle,
            ),
        });
    }

    /// Create a uniform distribution from template names
    pub fn uniform(template_names: Vec<impl Into<String>>) -> Self {
        let weight = 1.0 / template_names.len() as f64;
        let entries = template_names
            .into_iter()
            .map(|name| EntityDistributionEntry {
                weight: Double::literal(weight),
                scenario_object_template: ScenarioObjectTemplate::new(name, ObjectType::Vehicle),
            })
            .collect();

        Self { entries }
    }

    /// Get the total weight of all entries
    pub fn total_weight(&self) -> f64 {
        self.entries
            .iter()
            .filter_map(|entry| entry.weight.as_literal())
            .sum()
    }
}

impl EntityDistributionEntry {
    /// Create a new distribution entry from a template and weight
    pub fn new(scenario_object_template: ScenarioObjectTemplate, weight: f64) -> Self {
        Self {
            weight: Double::literal(weight),
            scenario_object_template,
        }
    }
}

impl ScenarioObjectTemplate {
    /// Create a new scenario object template
    pub fn new(name: impl Into<String>, object_type: ObjectType) -> Self {
        Self {
            name: OSString::literal(name.into()),
            object_type,
            external_object_reference: None,
            object_controller: Vec::new(),
        }
    }

    /// Create a template with external object reference
    pub fn with_external_reference(
        name: impl Into<String>,
        object_type: ObjectType,
        object_name: impl Into<String>,
    ) -> Self {
        Self {
            name: OSString::literal(name.into()),
            object_type,
            external_object_reference: Some(ExternalObjectReference {
                name: OSString::literal(object_name.into()),
            }),
            object_controller: Vec::new(),
        }
    }
}

impl ExternalObjectReference {
    /// Create a new external object reference
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: OSString::literal(name.into()),
        }
    }
}

impl ByObjectType {
    /// Create a new object type selector
    pub fn new(object_type: ObjectType) -> Self {
        Self { object_type }
    }

    /// Create a vehicle selector
    pub fn vehicle() -> Self {
        Self::new(ObjectType::Vehicle)
    }

    /// Create a pedestrian selector
    pub fn pedestrian() -> Self {
        Self::new(ObjectType::Pedestrian)
    }

    /// Create a miscellaneous object selector
    pub fn miscellaneous_object() -> Self {
        Self::new(ObjectType::MiscellaneousObject)
    }
}

impl ByType {
    /// Create a new type selector
    pub fn new(type_spec: ObjectType) -> Self {
        Self { type_spec }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::enums::ObjectType;

    #[test]
    fn test_entity_selection_creation() {
        // Test by object type
        let selection = EntitySelection::by_object_type(ObjectType::Vehicle);
        assert!(selection.by_type.is_some());
        assert_eq!(selection.by_type.unwrap().object_type, ObjectType::Vehicle);
    }

    #[test]
    fn test_selected_entities() {
        let mut entities = SelectedEntities::new();
        assert_eq!(entities.count(), 0);

        entities.add_entity("Ego");
        entities.add_entity("Target1");
        assert_eq!(entities.count(), 2);

        let entities_from_names = SelectedEntities::from_names(vec!["Car1", "Car2", "Car3"]);
        assert_eq!(entities_from_names.count(), 3);
    }

    #[test]
    fn test_entity_distribution() {
        let mut distribution = EntityDistribution::new();
        distribution.add_entry("Car1", 0.6);
        distribution.add_entry("Car2", 0.4);

        assert_eq!(distribution.entries.len(), 2);
        assert_eq!(distribution.total_weight(), 1.0);

        let uniform_dist = EntityDistribution::uniform(vec!["A", "B", "C", "D"]);
        assert_eq!(uniform_dist.entries.len(), 4);
        assert!((uniform_dist.total_weight() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_entity_distribution_entry() {
        let template = ScenarioObjectTemplate::new("TestEntity", ObjectType::Vehicle);
        let entry = EntityDistributionEntry::new(template, 0.75);
        assert_eq!(
            entry.scenario_object_template.name.as_literal().unwrap(),
            "TestEntity"
        );
        assert_eq!(entry.weight.as_literal().unwrap(), &0.75);
    }

    #[test]
    fn test_scenario_object_template() {
        let template = ScenarioObjectTemplate::new("VehicleTemplate", ObjectType::Vehicle);
        assert_eq!(template.name.as_literal().unwrap(), "VehicleTemplate");
        assert_eq!(template.object_type, ObjectType::Vehicle);

        let external_template = ScenarioObjectTemplate::with_external_reference(
            "ExternalVehicle",
            ObjectType::Vehicle,
            "SportsCar",
        );
        assert!(external_template.external_object_reference.is_some());
        let ext_ref = external_template.external_object_reference.unwrap();
        assert_eq!(ext_ref.name.as_literal().unwrap(), "SportsCar");
    }

    #[test]
    fn test_external_object_reference() {
        let ext_ref = ExternalObjectReference::new("Sedan");
        assert_eq!(ext_ref.name.as_literal().unwrap(), "Sedan");
    }

    #[test]
    fn test_by_object_type() {
        let vehicle_selector = ByObjectType::vehicle();
        assert_eq!(vehicle_selector.object_type, ObjectType::Vehicle);

        let pedestrian_selector = ByObjectType::pedestrian();
        assert_eq!(pedestrian_selector.object_type, ObjectType::Pedestrian);

        let misc_selector = ByObjectType::miscellaneous_object();
        assert_eq!(misc_selector.object_type, ObjectType::MiscellaneousObject);
    }

    #[test]
    fn test_by_type() {
        let type_selector = ByType::new(ObjectType::Vehicle);
        assert_eq!(type_selector.type_spec, ObjectType::Vehicle);
    }

    #[test]
    fn test_serialization() {
        let selection = EntitySelection::by_object_type(ObjectType::Vehicle);
        let xml = quick_xml::se::to_string(&selection).unwrap();
        assert!(xml.contains("ByType"));
        assert!(xml.contains("type=\"vehicle\""));

        let entities = SelectedEntities::from_names(vec!["Ego", "Target"]);
        let xml = quick_xml::se::to_string(&entities).unwrap();
        assert!(xml.contains("EntityRef"));
        assert!(xml.contains("entityRef=\"Ego\""));
        assert!(xml.contains("entityRef=\"Target\""));

        let distribution = EntityDistribution::uniform(vec!["Car1", "Car2"]);
        let xml = quick_xml::se::to_string(&distribution).unwrap();
        assert!(xml.contains("EntityDistributionEntry"));
        assert!(xml.contains("ScenarioObjectTemplate"));
        assert!(xml.contains("name=\"Car1\""));
        assert!(xml.contains("weight=\"0.5\""));
    }

    #[test]
    fn test_by_object_type_and_by_type_attribute_names() {
        // XSD: ByObjectType has attribute `type`; ByType has attribute `objectType`.
        let by_object_type: ByObjectType =
            quick_xml::de::from_str(r#"<ByObjectType type="vehicle"/>"#).unwrap();
        assert_eq!(by_object_type.object_type, ObjectType::Vehicle);

        let by_type: ByType =
            quick_xml::de::from_str(r#"<ByType objectType="pedestrian"/>"#).unwrap();
        assert_eq!(by_type.type_spec, ObjectType::Pedestrian);
    }

    #[test]
    fn test_external_object_reference_roundtrip() {
        // XSD: ExternalObjectReference has exactly one attribute, `name`.
        let ext_ref: ExternalObjectReference =
            quick_xml::de::from_str(r#"<ExternalObjectReference name="Sedan"/>"#).unwrap();
        assert_eq!(ext_ref.name.as_literal().unwrap(), "Sedan");
    }
}
