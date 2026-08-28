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
use crate::types::entities::{MiscObject, Pedestrian, ScenarioEntityReference, Vehicle};
use crate::types::enums::ObjectType;
use crate::types::scenario::triggers::EntityRef;
use serde::{Deserialize, Serialize};

/// Main entity selection framework with selection criteria
///
/// XSD `EntitySelection` (`:1180-1185`): required attribute `@name` (String);
/// sequence of a required `Members` element of type `SelectedEntities`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntitySelection {
    /// Name of the entity selection (used for references)
    #[serde(rename = "@name")]
    pub name: OSString,

    /// Members of the selection
    #[serde(rename = "Members")]
    pub members: SelectedEntities,
}

/// Container for selected entities with entity references
///
/// XSD `SelectedEntities` (`:2013-2018`): a choice, each branch
/// `maxOccurs="unbounded"`, of `EntityRef` or `ByType`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectedEntities {
    /// List of entity references
    #[serde(rename = "EntityRef", default, skip_serializing_if = "Vec::is_empty")]
    pub entity_refs: Vec<EntityRef>,

    /// List of type-based selections
    #[serde(rename = "ByType", default, skip_serializing_if = "Vec::is_empty")]
    pub by_type: Vec<ByType>,
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
///
/// XSD `ScenarioObjectTemplate` (`:2007-2012`): no attributes; sequence of
/// the `EntityObject` group (`:1168-1176`, a choice of `CatalogReference` |
/// `Vehicle` | `Pedestrian` | `MiscObject` | `ExternalObjectReference`)
/// followed by an optional/repeated `ObjectController`. Mirrors the
/// `EntityObject` group modeling used by `ScenarioObject`
/// (`src/types/entities/mod.rs`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScenarioObjectTemplate {
    /// Vehicle entity (optional)
    #[serde(rename = "Vehicle", skip_serializing_if = "Option::is_none")]
    pub vehicle: Option<Vehicle>,

    /// Pedestrian entity (optional)
    #[serde(rename = "Pedestrian", skip_serializing_if = "Option::is_none")]
    pub pedestrian: Option<Pedestrian>,

    /// Miscellaneous object entity (optional)
    #[serde(
        rename = "MiscObject",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub misc_object: Option<MiscObject>,

    /// External object reference (optional)
    #[serde(
        rename = "ExternalObjectReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub external_object_reference: Option<ExternalObjectReference>,

    /// Entity catalog reference (vehicle or pedestrian, optional)
    #[serde(rename = "CatalogReference", skip_serializing_if = "Option::is_none")]
    pub entity_catalog_reference: Option<ScenarioEntityReference>,

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
            name: OSString::literal("DefaultSelection".to_string()),
            members: SelectedEntities::default(),
        }
    }
}

impl Default for SelectedEntities {
    fn default() -> Self {
        Self {
            entity_refs: vec![EntityRef::default()],
            by_type: Vec::new(),
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
            vehicle: Some(Vehicle::default()),
            pedestrian: None,
            misc_object: None,
            external_object_reference: None,
            entity_catalog_reference: None,
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
    /// Create a new named entity selection
    pub fn new(name: impl Into<String>, members: SelectedEntities) -> Self {
        Self {
            name: OSString::literal(name.into()),
            members,
        }
    }
}

impl SelectedEntities {
    /// Create a new selected entities container
    pub fn new() -> Self {
        Self {
            entity_refs: Vec::new(),
            by_type: Vec::new(),
        }
    }

    /// Create selected entities from a list of entity names
    pub fn from_names(names: Vec<impl Into<String>>) -> Self {
        Self {
            entity_refs: names
                .into_iter()
                .map(|name| EntityRef::new(name.into()))
                .collect(),
            by_type: Vec::new(),
        }
    }

    /// Add an entity reference
    pub fn add_entity(&mut self, entity_name: impl Into<String>) {
        self.entity_refs.push(EntityRef::new(entity_name.into()));
    }

    /// Add a type-based selection
    pub fn add_by_type(&mut self, object_type: ObjectType) {
        self.by_type.push(ByType::new(object_type));
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

    /// Add a distribution entry from a scenario object template and weight
    pub fn add_entry(&mut self, scenario_object_template: ScenarioObjectTemplate, weight: f64) {
        self.entries
            .push(EntityDistributionEntry::new(scenario_object_template, weight));
    }

    /// Create a uniform distribution from scenario object templates
    pub fn uniform(templates: Vec<ScenarioObjectTemplate>) -> Self {
        let weight = 1.0 / templates.len() as f64;
        let entries = templates
            .into_iter()
            .map(|template| EntityDistributionEntry::new(template, weight))
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
    /// Create a new scenario object template wrapping a vehicle
    pub fn new_vehicle(vehicle: Vehicle) -> Self {
        Self {
            vehicle: Some(vehicle),
            pedestrian: None,
            misc_object: None,
            external_object_reference: None,
            entity_catalog_reference: None,
            object_controller: Vec::new(),
        }
    }

    /// Create a new scenario object template wrapping a pedestrian
    pub fn new_pedestrian(pedestrian: Pedestrian) -> Self {
        Self {
            vehicle: None,
            pedestrian: Some(pedestrian),
            misc_object: None,
            external_object_reference: None,
            entity_catalog_reference: None,
            object_controller: Vec::new(),
        }
    }

    /// Create a new scenario object template wrapping a miscellaneous object
    pub fn new_misc_object(misc_object: MiscObject) -> Self {
        Self {
            vehicle: None,
            pedestrian: None,
            misc_object: Some(misc_object),
            external_object_reference: None,
            entity_catalog_reference: None,
            object_controller: Vec::new(),
        }
    }

    /// Create a template referencing an external object definition
    pub fn with_external_reference(object_name: impl Into<String>) -> Self {
        Self {
            vehicle: None,
            pedestrian: None,
            misc_object: None,
            external_object_reference: Some(ExternalObjectReference {
                name: OSString::literal(object_name.into()),
            }),
            entity_catalog_reference: None,
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
        let mut members = SelectedEntities::new();
        members.add_entity("Ego");
        let selection = EntitySelection::new("Selection1", members);
        assert_eq!(selection.name.as_literal().unwrap(), "Selection1");
        assert_eq!(selection.members.entity_refs.len(), 1);
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

        entities.add_by_type(ObjectType::Pedestrian);
        assert_eq!(entities.by_type.len(), 1);
    }

    #[test]
    fn test_entity_distribution() {
        let mut distribution = EntityDistribution::new();
        distribution.add_entry(ScenarioObjectTemplate::new_vehicle(Vehicle::default()), 0.6);
        distribution.add_entry(ScenarioObjectTemplate::new_vehicle(Vehicle::default()), 0.4);

        assert_eq!(distribution.entries.len(), 2);
        assert_eq!(distribution.total_weight(), 1.0);

        let templates = vec![
            ScenarioObjectTemplate::new_vehicle(Vehicle::default()),
            ScenarioObjectTemplate::new_vehicle(Vehicle::default()),
            ScenarioObjectTemplate::new_vehicle(Vehicle::default()),
            ScenarioObjectTemplate::new_vehicle(Vehicle::default()),
        ];
        let uniform_dist = EntityDistribution::uniform(templates);
        assert_eq!(uniform_dist.entries.len(), 4);
        assert!((uniform_dist.total_weight() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_entity_distribution_entry() {
        let template = ScenarioObjectTemplate::new_vehicle(Vehicle::default());
        let entry = EntityDistributionEntry::new(template, 0.75);
        assert!(entry.scenario_object_template.vehicle.is_some());
        assert_eq!(entry.weight.as_literal().unwrap(), &0.75);
    }

    #[test]
    fn test_scenario_object_template() {
        let template = ScenarioObjectTemplate::new_vehicle(Vehicle::default());
        assert!(template.vehicle.is_some());
        assert!(template.pedestrian.is_none());

        let external_template = ScenarioObjectTemplate::with_external_reference("SportsCar");
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
        let mut members = SelectedEntities::new();
        members.add_entity("Ego");
        let selection = EntitySelection::new("Selection1", members);
        let xml = quick_xml::se::to_string(&selection).unwrap();
        assert!(xml.contains("Members"));
        assert!(xml.contains("name=\"Selection1\""));

        let entities = SelectedEntities::from_names(vec!["Ego", "Target"]);
        let xml = quick_xml::se::to_string(&entities).unwrap();
        assert!(xml.contains("EntityRef"));
        assert!(xml.contains("entityRef=\"Ego\""));
        assert!(xml.contains("entityRef=\"Target\""));

        let templates = vec![
            ScenarioObjectTemplate::new_vehicle(Vehicle::default()),
            ScenarioObjectTemplate::new_vehicle(Vehicle::default()),
        ];
        let distribution = EntityDistribution::uniform(templates);
        let xml = quick_xml::se::to_string(&distribution).unwrap();
        assert!(xml.contains("EntityDistributionEntry"));
        assert!(xml.contains("ScenarioObjectTemplate"));
        assert!(xml.contains("weight=\"0.5\""));
    }

    #[test]
    fn test_entity_selection_roundtrip_entity_refs() {
        // TASK: EntitySelection with @name and Members containing two EntityRef.
        let xml = r#"<EntitySelection name="Selection1"><Members><EntityRef entityRef="Ego"/><EntityRef entityRef="Target1"/></Members></EntitySelection>"#;
        let selection: EntitySelection = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(selection.name.as_literal().unwrap(), "Selection1");
        assert_eq!(selection.members.entity_refs.len(), 2);
        assert!(selection.members.by_type.is_empty());

        let serialized = quick_xml::se::to_string(&selection).unwrap();
        let roundtripped: EntitySelection = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(roundtripped, selection);
    }

    #[test]
    fn test_entity_selection_roundtrip_by_type() {
        // TASK: EntitySelection with Members containing a ByType branch.
        let xml = r#"<EntitySelection name="Selection2"><Members><ByType objectType="vehicle"/></Members></EntitySelection>"#;
        let selection: EntitySelection = quick_xml::de::from_str(xml).unwrap();
        assert!(selection.members.entity_refs.is_empty());
        assert_eq!(selection.members.by_type.len(), 1);
        assert_eq!(selection.members.by_type[0].type_spec, ObjectType::Vehicle);

        let serialized = quick_xml::se::to_string(&selection).unwrap();
        let roundtripped: EntitySelection = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(roundtripped, selection);
    }

    #[test]
    fn test_scenario_object_template_catalog_reference_roundtrip() {
        use crate::types::catalogs::references::CatalogReference;
        use crate::types::catalogs::entities::CatalogVehicle;

        let catalog_ref: CatalogReference<CatalogVehicle> =
            CatalogReference::new("VehicleCatalog".to_string(), "Sedan".to_string());
        let template = ScenarioObjectTemplate {
            vehicle: None,
            pedestrian: None,
            misc_object: None,
            external_object_reference: None,
            entity_catalog_reference: Some(ScenarioEntityReference::Vehicle(catalog_ref)),
            object_controller: Vec::new(),
        };

        let xml = quick_xml::se::to_string(&template).unwrap();
        assert!(xml.contains("CatalogReference"));

        let roundtripped: ScenarioObjectTemplate = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(roundtripped, template);
    }

    #[test]
    fn test_scenario_object_template_vehicle_with_controller_roundtrip() {
        use crate::types::controllers::Controller;
        use crate::types::enums::ControllerType;

        let mut template = ScenarioObjectTemplate::new_vehicle(Vehicle::default());
        template
            .object_controller
            .push(ObjectController::with_controller(Controller::new(
                "InlineController".to_string(),
                ControllerType::Movement,
            )));

        let xml = quick_xml::se::to_string(&template).unwrap();
        assert!(xml.contains("Vehicle"));
        assert!(xml.contains("ObjectController"));

        let roundtripped: ScenarioObjectTemplate = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(roundtripped.vehicle.is_some(), true);
        assert_eq!(roundtripped.object_controller.len(), 1);
        assert_eq!(roundtripped, template);
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
