//! Entity selection and distribution.
//!
//! `EntitySelection` names a group of entities, either explicitly through
//! `SelectedEntities` or by object type through `ByType`. `EntityDistribution`
//! describes a weighted set of `ScenarioObjectTemplate`s for a simulator to draw
//! from; a template may carry an `ExternalObjectReference` in place of an inline
//! definition.

use crate::types::basic::{Double, OSString, Value};
use crate::types::controllers::ObjectController;
use crate::types::entities::{
    EntityCatalogReference, EntityObjectChoice, MiscObject, Pedestrian, Vehicle,
};
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
/// XSD `SelectedEntities` (`:2013-2018`): a bare `xsd:choice` of `EntityRef` |
/// `ByType`, each branch `maxOccurs="unbounded"` and neither `minOccurs="0"`. A
/// document therefore holds one or more `EntityRef`s **or** one or more
/// `ByType`s, never a mixture and never neither. The repetition lives inside
/// the chosen branch, since `$value` takes one element name per instance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectedEntities {
    /// The chosen branch: one or more entity references, or one or more
    /// type-based selections.
    #[serde(rename = "$value")]
    pub choice: SelectedEntitiesChoice,
}

/// The two branches of the `SelectedEntities` choice (XSD:2013-2018).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SelectedEntitiesChoice {
    /// One or more explicit entity references
    EntityRef(Vec<EntityRef>),
    /// One or more type-based selections
    ByType(Vec<ByType>),
}

/// Entity distribution system for probabilistic entity spawning
///
/// XSD `EntityDistribution`: `EntityDistributionEntry` has `maxOccurs="unbounded"` with no
/// `minOccurs="0"`, so a schema-valid distribution needs at least one entry. The derived
/// `Default`'s `Vec::new()` is not schema-valid content on its own, but — like
/// `ConditionGroup` (`scenario/triggers.rs`) — it states nothing invented, unlike the
/// previous hand-written impl which filled the gap with a fabricated entry. Kept per the
/// container/choice policy as a construction convenience, and because `EntityDistribution`
/// has a `pub fn new()` that clippy's `new_without_default` otherwise flags.
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
/// followed by an optional/repeated `ObjectController`. The group ref carries
/// no `minOccurs`, so exactly one branch is required, the same choice
/// `ScenarioObject` (`src/types/entities/mod.rs`) models; both share
/// `EntityObjectChoice`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScenarioObjectTemplate {
    /// The entity definition, exactly one of the `EntityObject` group's branches.
    #[serde(rename = "$value")]
    pub entity: EntityObjectChoice,

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
    pub object_type: Value<ObjectType>,
}

/// Generic type-based selection criteria
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ByType {
    /// Type specification for selection
    #[serde(rename = "@objectType")]
    pub type_spec: Value<ObjectType>,
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
    /// Create a selected-entities container from an explicit list of entity
    /// references. The XSD choice requires at least one branch member, so
    /// there is no schema-valid empty constructor.
    pub fn from_names(names: Vec<impl Into<String>>) -> Self {
        Self {
            choice: SelectedEntitiesChoice::EntityRef(
                names
                    .into_iter()
                    .map(|name| EntityRef::new(name.into()))
                    .collect(),
            ),
        }
    }

    /// Create a selected-entities container choosing a single object type.
    pub fn from_by_type(object_type: ObjectType) -> Self {
        Self {
            choice: SelectedEntitiesChoice::ByType(vec![ByType::new(object_type)]),
        }
    }

    /// The entity references, if this is the `EntityRef` branch.
    pub fn entity_refs(&self) -> &[EntityRef] {
        match &self.choice {
            SelectedEntitiesChoice::EntityRef(refs) => refs,
            SelectedEntitiesChoice::ByType(_) => &[],
        }
    }

    /// The type-based selections, if this is the `ByType` branch.
    pub fn by_type(&self) -> &[ByType] {
        match &self.choice {
            SelectedEntitiesChoice::ByType(types) => types,
            SelectedEntitiesChoice::EntityRef(_) => &[],
        }
    }

    /// Get the number of selected entities in the `EntityRef` branch.
    pub fn count(&self) -> usize {
        self.entity_refs().len()
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
        self.entries.push(EntityDistributionEntry::new(
            scenario_object_template,
            weight,
        ));
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
            entity: EntityObjectChoice::Vehicle(vehicle),
            object_controller: Vec::new(),
        }
    }

    /// Create a new scenario object template wrapping a pedestrian
    pub fn new_pedestrian(pedestrian: Pedestrian) -> Self {
        Self {
            entity: EntityObjectChoice::Pedestrian(pedestrian),
            object_controller: Vec::new(),
        }
    }

    /// Create a new scenario object template wrapping a miscellaneous object
    pub fn new_misc_object(misc_object: MiscObject) -> Self {
        Self {
            entity: EntityObjectChoice::MiscObject(misc_object),
            object_controller: Vec::new(),
        }
    }

    /// Create a template referencing an external object definition
    pub fn with_external_reference(object_name: impl Into<String>) -> Self {
        Self {
            entity: EntityObjectChoice::ExternalObjectReference(ExternalObjectReference {
                name: OSString::literal(object_name.into()),
            }),
            object_controller: Vec::new(),
        }
    }

    /// The vehicle, if this template's entity branch is `Vehicle`
    pub fn vehicle(&self) -> Option<&Vehicle> {
        match &self.entity {
            EntityObjectChoice::Vehicle(v) => Some(v),
            _ => None,
        }
    }

    /// The pedestrian, if this template's entity branch is `Pedestrian`
    pub fn pedestrian(&self) -> Option<&Pedestrian> {
        match &self.entity {
            EntityObjectChoice::Pedestrian(p) => Some(p),
            _ => None,
        }
    }

    /// The miscellaneous object, if this template's entity branch is `MiscObject`
    pub fn misc_object(&self) -> Option<&MiscObject> {
        match &self.entity {
            EntityObjectChoice::MiscObject(m) => Some(m),
            _ => None,
        }
    }

    /// The external object reference, if this template's entity branch is `ExternalObjectReference`
    pub fn external_object_reference(&self) -> Option<&ExternalObjectReference> {
        match &self.entity {
            EntityObjectChoice::ExternalObjectReference(r) => Some(r),
            _ => None,
        }
    }

    /// The catalog reference, if this template's entity branch is `CatalogReference`
    pub fn entity_catalog_reference(&self) -> Option<&EntityCatalogReference> {
        match &self.entity {
            EntityObjectChoice::CatalogReference(r) => Some(r),
            _ => None,
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
        Self {
            object_type: Value::Literal(object_type),
        }
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
        Self {
            type_spec: Value::Literal(type_spec),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::enums::ObjectType;

    #[test]
    fn test_entity_selection_creation() {
        let members = SelectedEntities::from_names(vec!["Ego"]);
        let selection = EntitySelection::new("Selection1", members);
        assert_eq!(selection.name.as_literal().unwrap(), "Selection1");
        assert_eq!(selection.members.entity_refs().len(), 1);
    }

    #[test]
    fn test_selected_entities() {
        let entities = SelectedEntities::from_names(vec!["Ego", "Target1"]);
        assert_eq!(entities.count(), 2);

        let entities_from_names = SelectedEntities::from_names(vec!["Car1", "Car2", "Car3"]);
        assert_eq!(entities_from_names.count(), 3);

        let by_type_entities = SelectedEntities::from_by_type(ObjectType::Pedestrian);
        assert_eq!(by_type_entities.by_type().len(), 1);
    }

    #[test]
    fn test_entity_distribution() {
        let mut distribution = EntityDistribution::new();
        distribution.add_entry(
            ScenarioObjectTemplate::new_vehicle(Vehicle::new_car("TestVehicle".to_string())),
            0.6,
        );
        distribution.add_entry(
            ScenarioObjectTemplate::new_vehicle(Vehicle::new_car("TestVehicle".to_string())),
            0.4,
        );

        assert_eq!(distribution.entries.len(), 2);
        assert_eq!(distribution.total_weight(), 1.0);

        let templates = vec![
            ScenarioObjectTemplate::new_vehicle(Vehicle::new_car("TestVehicle".to_string())),
            ScenarioObjectTemplate::new_vehicle(Vehicle::new_car("TestVehicle".to_string())),
            ScenarioObjectTemplate::new_vehicle(Vehicle::new_car("TestVehicle".to_string())),
            ScenarioObjectTemplate::new_vehicle(Vehicle::new_car("TestVehicle".to_string())),
        ];
        let uniform_dist = EntityDistribution::uniform(templates);
        assert_eq!(uniform_dist.entries.len(), 4);
        assert!((uniform_dist.total_weight() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_entity_distribution_entry() {
        let template =
            ScenarioObjectTemplate::new_vehicle(Vehicle::new_car("TestVehicle".to_string()));
        let entry = EntityDistributionEntry::new(template, 0.75);
        assert!(entry.scenario_object_template.vehicle().is_some());
        assert_eq!(entry.weight.as_literal().unwrap(), &0.75);
    }

    #[test]
    fn test_scenario_object_template() {
        let template =
            ScenarioObjectTemplate::new_vehicle(Vehicle::new_car("TestVehicle".to_string()));
        assert!(template.vehicle().is_some());
        assert!(template.pedestrian().is_none());

        let external_template = ScenarioObjectTemplate::with_external_reference("SportsCar");
        assert!(external_template.external_object_reference().is_some());
        let ext_ref = external_template.external_object_reference().unwrap();
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
        assert_eq!(
            vehicle_selector.object_type,
            Value::Literal(ObjectType::Vehicle)
        );

        let pedestrian_selector = ByObjectType::pedestrian();
        assert_eq!(
            pedestrian_selector.object_type,
            Value::Literal(ObjectType::Pedestrian)
        );

        let misc_selector = ByObjectType::miscellaneous_object();
        assert_eq!(
            misc_selector.object_type,
            Value::Literal(ObjectType::MiscellaneousObject)
        );
    }

    #[test]
    fn test_by_type() {
        let type_selector = ByType::new(ObjectType::Vehicle);
        assert_eq!(type_selector.type_spec, Value::Literal(ObjectType::Vehicle));
    }

    #[test]
    fn test_serialization() {
        let members = SelectedEntities::from_names(vec!["Ego"]);
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
            ScenarioObjectTemplate::new_vehicle(Vehicle::new_car("TestVehicle".to_string())),
            ScenarioObjectTemplate::new_vehicle(Vehicle::new_car("TestVehicle".to_string())),
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
        assert_eq!(selection.members.entity_refs().len(), 2);
        assert!(selection.members.by_type().is_empty());

        let serialized = quick_xml::se::to_string(&selection).unwrap();
        let roundtripped: EntitySelection = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(roundtripped, selection);
    }

    #[test]
    fn test_entity_selection_roundtrip_by_type() {
        // TASK: EntitySelection with Members containing a ByType branch.
        let xml = r#"<EntitySelection name="Selection2"><Members><ByType objectType="vehicle"/></Members></EntitySelection>"#;
        let selection: EntitySelection = quick_xml::de::from_str(xml).unwrap();
        assert!(selection.members.entity_refs().is_empty());
        assert_eq!(selection.members.by_type().len(), 1);
        assert_eq!(
            selection.members.by_type()[0].type_spec,
            Value::Literal(ObjectType::Vehicle)
        );

        let serialized = quick_xml::se::to_string(&selection).unwrap();
        let roundtripped: EntitySelection = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(roundtripped, selection);
    }

    #[test]
    fn test_scenario_object_template_catalog_reference_roundtrip() {
        let catalog_ref = EntityCatalogReference::new("VehicleCatalog", "Sedan");
        let template = ScenarioObjectTemplate {
            entity: EntityObjectChoice::CatalogReference(catalog_ref),
            object_controller: Vec::new(),
        };

        let xml = quick_xml::se::to_string(&template).unwrap();
        assert!(xml.contains("CatalogReference"));

        let roundtripped: ScenarioObjectTemplate = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(roundtripped, template);
    }

    #[test]
    fn test_scenario_object_template_catalog_reference_absent_parameter_assignments_roundtrip() {
        let xml = r#"<ScenarioObjectTemplate><CatalogReference catalogName="VehicleCatalog" entryName="Sedan"/></ScenarioObjectTemplate>"#;
        let template: ScenarioObjectTemplate = quick_xml::de::from_str(xml).unwrap();
        let catalog_reference = template
            .entity_catalog_reference()
            .expect("CatalogReference must parse");
        assert!(
            catalog_reference.parameter_assignments.is_none(),
            "absent <ParameterAssignments> must yield None"
        );

        let ser = quick_xml::se::to_string(&template).unwrap();
        assert_eq!(ser, xml, "byte-exact round trip, no ParameterAssignments");
    }

    #[test]
    fn test_scenario_object_template_catalog_reference_with_parameter_assignments_roundtrip() {
        let xml = r#"<ScenarioObjectTemplate><CatalogReference catalogName="VehicleCatalog" entryName="Sedan"><ParameterAssignments><ParameterAssignment parameterRef="Color" value="Red"/></ParameterAssignments></CatalogReference></ScenarioObjectTemplate>"#;
        let template: ScenarioObjectTemplate = quick_xml::de::from_str(xml).unwrap();
        let assignments = template
            .entity_catalog_reference()
            .and_then(|r| r.parameter_assignments.as_ref())
            .expect("present <ParameterAssignments> must yield Some");
        assert_eq!(assignments.assignments.len(), 1);
        assert_eq!(
            assignments.assignments[0]
                .parameter_ref
                .as_literal()
                .unwrap(),
            "Color"
        );

        let ser = quick_xml::se::to_string(&template).unwrap();
        assert_eq!(ser, xml, "byte-exact round trip with ParameterAssignments");
    }

    #[test]
    fn test_scenario_object_template_vehicle_with_controller_roundtrip() {
        use crate::types::controllers::Controller;
        use crate::types::enums::ControllerType;

        let mut template =
            ScenarioObjectTemplate::new_vehicle(Vehicle::new_car("TestVehicle".to_string()));
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
        assert!(roundtripped.vehicle().is_some());
        assert_eq!(roundtripped.object_controller.len(), 1);
        assert_eq!(roundtripped, template);
    }

    #[test]
    fn test_by_object_type_and_by_type_attribute_names() {
        // XSD: ByObjectType has attribute `type`; ByType has attribute `objectType`.
        let by_object_type: ByObjectType =
            quick_xml::de::from_str(r#"<ByObjectType type="vehicle"/>"#).unwrap();
        assert_eq!(
            by_object_type.object_type,
            Value::Literal(ObjectType::Vehicle)
        );

        let by_type: ByType =
            quick_xml::de::from_str(r#"<ByType objectType="pedestrian"/>"#).unwrap();
        assert_eq!(by_type.type_spec, Value::Literal(ObjectType::Pedestrian));
    }

    #[test]
    fn test_external_object_reference_roundtrip() {
        // XSD: ExternalObjectReference has exactly one attribute, `name`.
        let ext_ref: ExternalObjectReference =
            quick_xml::de::from_str(r#"<ExternalObjectReference name="Sedan"/>"#).unwrap();
        assert_eq!(ext_ref.name.as_literal().unwrap(), "Sedan");
    }

    // `SelectedEntities` (XSD:2013-2018) is a bare choice of `EntityRef` |
    // `ByType`, both branches `maxOccurs="unbounded"`. The repetition lives
    // inside the chosen branch, so two `EntityRef` siblings are a single
    // valid branch while an `EntityRef` beside a `ByType` is two branches.

    #[test]
    fn test_selected_entities_one_entity_ref_round_trips_byte_exact() {
        let xml = r#"<SelectedEntities><EntityRef entityRef="Ego"/></SelectedEntities>"#;
        let parsed: SelectedEntities = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.entity_refs().len(), 1);
        let serialized = quick_xml::se::to_string(&parsed).unwrap();
        assert_eq!(serialized, xml);
    }

    #[test]
    fn test_selected_entities_two_entity_refs_are_one_branch_and_round_trip_byte_exact() {
        let xml = r#"<SelectedEntities><EntityRef entityRef="Ego"/><EntityRef entityRef="Target1"/></SelectedEntities>"#;
        let parsed: SelectedEntities = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.entity_refs().len(), 2);
        let serialized = quick_xml::se::to_string(&parsed).unwrap();
        assert_eq!(serialized, xml);
    }

    #[test]
    fn test_selected_entities_zero_branches_rejected() {
        let xml = r#"<SelectedEntities/>"#;
        let result: Result<SelectedEntities, _> = quick_xml::de::from_str(xml);
        assert!(result.is_err(), "empty SelectedEntities must be rejected");
    }

    #[test]
    fn test_selected_entities_entity_ref_and_by_type_together_rejected() {
        let xml = r#"<SelectedEntities><EntityRef entityRef="Ego"/><ByType objectType="vehicle"/></SelectedEntities>"#;
        let result: Result<SelectedEntities, _> = quick_xml::de::from_str(xml);
        assert!(
            result.is_err(),
            "EntityRef beside ByType is two branches and must be rejected"
        );
    }

    // `ScenarioObjectTemplate` (XSD:2007-2012) shares `EntityObjectChoice`
    // with `ScenarioObject`; the choice is required (the group ref carries
    // no `minOccurs`).

    #[test]
    fn test_scenario_object_template_zero_branches_rejected() {
        let xml = r#"<ScenarioObjectTemplate/>"#;
        let result: Result<ScenarioObjectTemplate, _> = quick_xml::de::from_str(xml);
        assert!(
            result.is_err(),
            "ScenarioObjectTemplate with no entity branch must be rejected"
        );
    }

    #[test]
    fn test_scenario_object_template_two_branches_rejected() {
        let xml = r#"<ScenarioObjectTemplate><CatalogReference catalogName="Cat" entryName="Entry"/><ExternalObjectReference name="Sedan"/></ScenarioObjectTemplate>"#;
        let result: Result<ScenarioObjectTemplate, _> = quick_xml::de::from_str(xml);
        assert!(
            result.is_err(),
            "CatalogReference beside ExternalObjectReference must be rejected"
        );
    }
}
