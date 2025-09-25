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
use crate::types::enums::ObjectType;
use crate::types::scenario::triggers::EntityRef;
use serde::{Deserialize, Serialize};

/// Main entity selection framework with selection criteria
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntitySelection {
    /// Selection by object type
    #[serde(rename = "ByType", skip_serializing_if = "Option::is_none")]
    pub by_type: Option<ByObjectType>,
    
    /// Selection by entity name pattern
    #[serde(rename = "ByName", skip_serializing_if = "Option::is_none")]
    pub by_name: Option<ByName>,
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

/// Individual distribution entry with entity reference and weight
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityDistributionEntry {
    /// Reference to the entity
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,
    
    /// Probability weight for this entity
    #[serde(rename = "@weight")]
    pub weight: Double,
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
    
    /// Template properties (optional)
    #[serde(rename = "Properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<TemplateProperties>,
    
    /// External object reference (optional)
    #[serde(rename = "ExternalObjectReference", skip_serializing_if = "Option::is_none")]
    pub external_object_reference: Option<ExternalObjectReference>,
}

/// Reference to external object definitions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExternalObjectReference {
    /// Path to the external object file
    #[serde(rename = "@file")]
    pub file: OSString,
    
    /// Name of the object within the external file
    #[serde(rename = "@name")]
    pub name: OSString,
}

/// Entity selection by object type (vehicle, pedestrian, etc.)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ByObjectType {
    /// Type of object to select
    #[serde(rename = "@objectType")]
    pub object_type: ObjectType,
}

/// Generic type-based selection criteria
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ByType {
    /// Type specification for selection
    #[serde(rename = "@type")]
    pub type_spec: OSString,
}

/// Selection by entity name pattern
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ByName {
    /// Name pattern for entity selection
    #[serde(rename = "@name")]
    pub name: OSString,
}

/// Template properties for scenario object templates
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TemplateProperties {
    /// Custom properties as key-value pairs
    #[serde(rename = "Property", default)]
    pub properties: Vec<TemplateProperty>,
}

/// Individual template property
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TemplateProperty {
    /// Property name
    #[serde(rename = "@name")]
    pub name: OSString,
    
    /// Property value
    #[serde(rename = "@value")]
    pub value: OSString,
}

// Default implementations
impl Default for EntitySelection {
    fn default() -> Self {
        Self {
            by_type: Some(ByObjectType::default()),
            by_name: None,
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
            entity_ref: OSString::literal("DefaultEntity".to_string()),
            weight: Double::literal(1.0),
        }
    }
}

impl Default for ScenarioObjectTemplate {
    fn default() -> Self {
        Self {
            name: OSString::literal("DefaultTemplate".to_string()),
            object_type: ObjectType::Vehicle,
            properties: None,
            external_object_reference: None,
        }
    }
}

impl Default for ExternalObjectReference {
    fn default() -> Self {
        Self {
            file: OSString::literal("objects.xml".to_string()),
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
            type_spec: OSString::literal("vehicle".to_string()),
        }
    }
}

impl Default for ByName {
    fn default() -> Self {
        Self {
            name: OSString::literal("*".to_string()),
        }
    }
}

impl Default for TemplateProperties {
    fn default() -> Self {
        Self {
            properties: Vec::new(),
        }
    }
}

impl Default for TemplateProperty {
    fn default() -> Self {
        Self {
            name: OSString::literal("property".to_string()),
            value: OSString::literal("value".to_string()),
        }
    }
}

// Implementation methods
impl EntitySelection {
    /// Create a new entity selection by object type
    pub fn by_object_type(object_type: ObjectType) -> Self {
        Self {
            by_type: Some(ByObjectType { object_type }),
            by_name: None,
        }
    }
    
    /// Create a new entity selection by name pattern
    pub fn by_name(name: impl Into<String>) -> Self {
        Self {
            by_type: None,
            by_name: Some(ByName {
                name: OSString::literal(name.into()),
            }),
        }
    }
    
    /// Create a new entity selection with both type and name criteria
    pub fn by_type_and_name(object_type: ObjectType, name: impl Into<String>) -> Self {
        Self {
            by_type: Some(ByObjectType { object_type }),
            by_name: Some(ByName {
                name: OSString::literal(name.into()),
            }),
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
    
    /// Add a distribution entry
    pub fn add_entry(&mut self, entity_ref: impl Into<String>, weight: f64) {
        self.entries.push(EntityDistributionEntry {
            entity_ref: OSString::literal(entity_ref.into()),
            weight: Double::literal(weight),
        });
    }
    
    /// Create a uniform distribution from entity names
    pub fn uniform(entity_names: Vec<impl Into<String>>) -> Self {
        let weight = 1.0 / entity_names.len() as f64;
        let entries = entity_names
            .into_iter()
            .map(|name| EntityDistributionEntry {
                entity_ref: OSString::literal(name.into()),
                weight: Double::literal(weight),
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
    /// Create a new distribution entry
    pub fn new(entity_ref: impl Into<String>, weight: f64) -> Self {
        Self {
            entity_ref: OSString::literal(entity_ref.into()),
            weight: Double::literal(weight),
        }
    }
}

impl ScenarioObjectTemplate {
    /// Create a new scenario object template
    pub fn new(name: impl Into<String>, object_type: ObjectType) -> Self {
        Self {
            name: OSString::literal(name.into()),
            object_type,
            properties: None,
            external_object_reference: None,
        }
    }
    
    /// Create a template with external object reference
    pub fn with_external_reference(
        name: impl Into<String>,
        object_type: ObjectType,
        file: impl Into<String>,
        object_name: impl Into<String>,
    ) -> Self {
        Self {
            name: OSString::literal(name.into()),
            object_type,
            properties: None,
            external_object_reference: Some(ExternalObjectReference {
                file: OSString::literal(file.into()),
                name: OSString::literal(object_name.into()),
            }),
        }
    }
    
    /// Add a property to the template
    pub fn add_property(&mut self, name: impl Into<String>, value: impl Into<String>) {
        if self.properties.is_none() {
            self.properties = Some(TemplateProperties::default());
        }
        
        if let Some(ref mut props) = self.properties {
            props.properties.push(TemplateProperty {
                name: OSString::literal(name.into()),
                value: OSString::literal(value.into()),
            });
        }
    }
}

impl ExternalObjectReference {
    /// Create a new external object reference
    pub fn new(file: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            file: OSString::literal(file.into()),
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
    pub fn new(type_spec: impl Into<String>) -> Self {
        Self {
            type_spec: OSString::literal(type_spec.into()),
        }
    }
}

impl ByName {
    /// Create a new name selector
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: OSString::literal(name.into()),
        }
    }
    
    /// Create a wildcard selector (matches all)
    pub fn wildcard() -> Self {
        Self::new("*")
    }
    
    /// Create a prefix selector
    pub fn prefix(prefix: impl Into<String>) -> Self {
        Self::new(format!("{}*", prefix.into()))
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
        assert!(selection.by_name.is_none());

        // Test by name
        let selection = EntitySelection::by_name("Ego*");
        assert!(selection.by_type.is_none());
        assert!(selection.by_name.is_some());
        assert_eq!(
            selection.by_name.unwrap().name.as_literal().unwrap(),
            "Ego*"
        );

        // Test by type and name
        let selection = EntitySelection::by_type_and_name(ObjectType::Pedestrian, "Walker*");
        assert!(selection.by_type.is_some());
        assert!(selection.by_name.is_some());
        assert_eq!(selection.by_type.unwrap().object_type, ObjectType::Pedestrian);
        assert_eq!(
            selection.by_name.unwrap().name.as_literal().unwrap(),
            "Walker*"
        );
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
        let entry = EntityDistributionEntry::new("TestEntity", 0.75);
        assert_eq!(entry.entity_ref.as_literal().unwrap(), "TestEntity");
        assert_eq!(entry.weight.as_literal().unwrap(), &0.75);
    }

    #[test]
    fn test_scenario_object_template() {
        let mut template = ScenarioObjectTemplate::new("VehicleTemplate", ObjectType::Vehicle);
        assert_eq!(template.name.as_literal().unwrap(), "VehicleTemplate");
        assert_eq!(template.object_type, ObjectType::Vehicle);
        assert!(template.properties.is_none());

        template.add_property("color", "red");
        template.add_property("mass", "1500");
        assert!(template.properties.is_some());
        assert_eq!(template.properties.as_ref().unwrap().properties.len(), 2);

        let external_template = ScenarioObjectTemplate::with_external_reference(
            "ExternalVehicle",
            ObjectType::Vehicle,
            "vehicles.xml",
            "SportsCar",
        );
        assert!(external_template.external_object_reference.is_some());
        let ext_ref = external_template.external_object_reference.unwrap();
        assert_eq!(ext_ref.file.as_literal().unwrap(), "vehicles.xml");
        assert_eq!(ext_ref.name.as_literal().unwrap(), "SportsCar");
    }

    #[test]
    fn test_external_object_reference() {
        let ext_ref = ExternalObjectReference::new("objects/vehicles.xml", "Sedan");
        assert_eq!(ext_ref.file.as_literal().unwrap(), "objects/vehicles.xml");
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
        let type_selector = ByType::new("custom_vehicle_type");
        assert_eq!(
            type_selector.type_spec.as_literal().unwrap(),
            "custom_vehicle_type"
        );
    }

    #[test]
    fn test_by_name() {
        let wildcard_selector = ByName::wildcard();
        assert_eq!(wildcard_selector.name.as_literal().unwrap(), "*");

        let prefix_selector = ByName::prefix("Ego");
        assert_eq!(prefix_selector.name.as_literal().unwrap(), "Ego*");

        let exact_selector = ByName::new("SpecificEntity");
        assert_eq!(exact_selector.name.as_literal().unwrap(), "SpecificEntity");
    }

    #[test]
    fn test_serialization() {
        let selection = EntitySelection::by_object_type(ObjectType::Vehicle);
        let xml = quick_xml::se::to_string(&selection).unwrap();
        assert!(xml.contains("ByType"));
        assert!(xml.contains("objectType=\"vehicle\""));

        let entities = SelectedEntities::from_names(vec!["Ego", "Target"]);
        let xml = quick_xml::se::to_string(&entities).unwrap();
        assert!(xml.contains("EntityRef"));
        assert!(xml.contains("entityRef=\"Ego\""));
        assert!(xml.contains("entityRef=\"Target\""));

        let distribution = EntityDistribution::uniform(vec!["Car1", "Car2"]);
        let xml = quick_xml::se::to_string(&distribution).unwrap();
        assert!(xml.contains("EntityDistributionEntry"));
        assert!(xml.contains("entityRef=\"Car1\""));
        assert!(xml.contains("weight=\"0.5\""));
    }
}