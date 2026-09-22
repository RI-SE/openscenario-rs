//! Comprehensive tests for entity selection types
//!
//! Tests all 8 critical entity selection types:
//! - EntitySelection
//! - SelectedEntities
//! - EntityDistribution
//! - EntityDistributionEntry
//! - ScenarioObjectTemplate
//! - ExternalObjectReference
//! - ByObjectType
//! - ByType

use openscenario_rs::types::basic::Value;
use openscenario_rs::types::{
    basic::{Double, OSString},
    entities::{
        ByObjectType, ByType, EntityDistribution, EntityDistributionEntry, EntityObjectChoice,
        EntitySelection, ExternalObjectReference, ScenarioObjectTemplate, SelectedEntities,
        Vehicle,
    },
    enums::ObjectType,
};

#[test]
fn test_entity_selection_creation() {
    let members = SelectedEntities::from_names(vec!["Ego"]);
    let selection = EntitySelection::new("Selection1", members);

    assert_eq!(selection.name.as_literal().unwrap(), "Selection1");
    assert_eq!(selection.members.entity_refs().len(), 1);

    // Test serialization
    let xml = quick_xml::se::to_string(&selection).unwrap();
    assert!(xml.contains("Members"));
    assert!(xml.contains("name=\"Selection1\""));
}

#[test]
fn test_entity_selection_xml_parsing_entity_refs() {
    let xml = r#"
    <EntitySelection name="Selection1">
        <Members>
            <EntityRef entityRef="Ego"/>
        </Members>
    </EntitySelection>
    "#;

    let selection: EntitySelection = quick_xml::de::from_str(xml).unwrap();
    assert_eq!(selection.name.as_literal().unwrap(), "Selection1");
    assert_eq!(selection.members.entity_refs().len(), 1);
    assert!(selection.members.by_type().is_empty());
}

#[test]
fn test_entity_selection_xml_parsing_by_type() {
    let xml = r#"
    <EntitySelection name="Selection2">
        <Members>
            <ByType objectType="vehicle"/>
        </Members>
    </EntitySelection>
    "#;

    let selection: EntitySelection = quick_xml::de::from_str(xml).unwrap();
    assert!(selection.members.entity_refs().is_empty());
    assert_eq!(selection.members.by_type().len(), 1);
    assert_eq!(
        selection.members.by_type()[0].type_spec,
        Value::Literal(ObjectType::Vehicle)
    );
}

#[test]
fn test_selected_entities_creation() {
    let entities = SelectedEntities::from_names(vec!["Ego", "Target1", "Target2"]);
    assert_eq!(entities.count(), 3);

    // Test from names
    let entities_from_names = SelectedEntities::from_names(vec!["Car1", "Car2", "Car3"]);
    assert_eq!(entities_from_names.count(), 3);
}

#[test]
fn test_selected_entities_xml_serialization() {
    let entities = SelectedEntities::from_names(vec!["Ego", "Target"]);
    let xml = quick_xml::se::to_string(&entities).unwrap();

    assert!(xml.contains("EntityRef"));
    assert!(xml.contains("entityRef=\"Ego\""));
    assert!(xml.contains("entityRef=\"Target\""));
}

#[test]
fn test_selected_entities_xml_parsing() {
    let xml = r#"
    <SelectedEntities>
        <EntityRef entityRef="Ego"/>
        <EntityRef entityRef="Target1"/>
        <EntityRef entityRef="Target2"/>
    </SelectedEntities>
    "#;

    let entities: SelectedEntities = quick_xml::de::from_str(xml).unwrap();
    assert_eq!(entities.count(), 3);

    let entity_names: Vec<String> = entities
        .entity_refs()
        .iter()
        .filter_map(|e| e.entity_ref.as_literal().cloned())
        .collect();
    assert!(entity_names.contains(&"Ego".to_string()));
    assert!(entity_names.contains(&"Target1".to_string()));
    assert!(entity_names.contains(&"Target2".to_string()));
}

#[test]
fn test_entity_distribution_creation() {
    // XSD `EntityDistribution` (:1164) declares `EntityDistributionEntry` with the default
    // `minOccurs="1"`. The entries are supplied once, here: the empty `new()` and the
    // `add_entry` mutation path it needed could both describe a distribution with no
    // entries, which is not a document this crate can emit.
    let distribution = EntityDistribution::new(vec![
        EntityDistributionEntry::new(
            ScenarioObjectTemplate::new_vehicle(Vehicle::new_car("TestVehicle".to_string())),
            0.6,
        ),
        EntityDistributionEntry::new(
            ScenarioObjectTemplate::new_vehicle(Vehicle::new_car("TestVehicle".to_string())),
            0.4,
        ),
    ])
    .unwrap();

    assert_eq!(distribution.entries.len(), 2);
    assert_eq!(distribution.total_weight(), 1.0);

    // Test uniform distribution
    let templates: Vec<ScenarioObjectTemplate> = (0..4)
        .map(|_| ScenarioObjectTemplate::new_vehicle(Vehicle::new_car("TestVehicle".to_string())))
        .collect();
    let uniform_dist = EntityDistribution::uniform(templates).unwrap();
    assert_eq!(uniform_dist.entries.len(), 4);
    assert!((uniform_dist.total_weight() - 1.0).abs() < f64::EPSILON);

    // Each entry should have weight 0.25
    for entry in &uniform_dist.entries {
        assert!((entry.weight.as_literal().unwrap() - 0.25).abs() < f64::EPSILON);
    }
}

#[test]
fn test_entity_distribution_xml_serialization() {
    let templates = vec![
        ScenarioObjectTemplate::new_vehicle(Vehicle::new_car("TestVehicle".to_string())),
        ScenarioObjectTemplate::new_vehicle(Vehicle::new_car("TestVehicle".to_string())),
    ];
    let distribution = EntityDistribution::uniform(templates).unwrap();
    let xml = quick_xml::se::to_string(&distribution).unwrap();

    assert!(xml.contains("EntityDistributionEntry"));
    assert!(xml.contains("ScenarioObjectTemplate"));
    assert!(xml.contains("Vehicle"));
    assert!(xml.contains("weight=\"0.5\""));
}

#[test]
fn test_entity_distribution_xml_parsing() {
    let xml = r#"
    <EntityDistribution>
        <EntityDistributionEntry weight="0.6">
            <ScenarioObjectTemplate>
                <CatalogReference catalogName="VehicleCatalog" entryName="Car1"/>
            </ScenarioObjectTemplate>
        </EntityDistributionEntry>
        <EntityDistributionEntry weight="0.4">
            <ScenarioObjectTemplate>
                <CatalogReference catalogName="VehicleCatalog" entryName="Car2"/>
            </ScenarioObjectTemplate>
        </EntityDistributionEntry>
    </EntityDistribution>
    "#;

    let distribution: EntityDistribution = quick_xml::de::from_str(xml).unwrap();
    assert_eq!(distribution.entries.len(), 2);
    assert_eq!(distribution.total_weight(), 1.0);

    let weight_06 = distribution
        .entries
        .iter()
        .find(|e| e.weight.as_literal() == Some(&0.6))
        .unwrap();
    assert!(weight_06
        .scenario_object_template
        .entity_catalog_reference()
        .is_some());

    let weight_04 = distribution
        .entries
        .iter()
        .find(|e| e.weight.as_literal() == Some(&0.4))
        .unwrap();
    assert!(weight_04
        .scenario_object_template
        .entity_catalog_reference()
        .is_some());
}

#[test]
fn test_entity_distribution_entry() {
    let template = ScenarioObjectTemplate::new_vehicle(Vehicle::new_car("TestVehicle".to_string()));
    let entry = EntityDistributionEntry::new(template, 0.75);
    assert!(entry.scenario_object_template.vehicle().is_some());
    assert_eq!(entry.weight.as_literal().unwrap(), &0.75);

    // Test construction via ::new
    let default_entry = EntityDistributionEntry::new(
        ScenarioObjectTemplate::new_vehicle(Vehicle::new_car("TestVehicle".to_string())),
        1.0,
    );
    assert!(default_entry.scenario_object_template.vehicle().is_some());
    assert_eq!(default_entry.weight.as_literal().unwrap(), &1.0);
}

#[test]
fn test_scenario_object_template_basic() {
    let template = ScenarioObjectTemplate::new_vehicle(Vehicle::new_car("TestVehicle".to_string()));
    assert!(template.vehicle().is_some());
    assert!(template.pedestrian().is_none());
    assert!(template.external_object_reference().is_none());
}

#[test]
fn test_scenario_object_template_with_external_reference() {
    let template = ScenarioObjectTemplate::with_external_reference("SportsCar");

    assert!(template.vehicle().is_none());
    assert!(template.external_object_reference().is_some());

    let ext_ref = template.external_object_reference().unwrap();
    assert_eq!(ext_ref.name.as_literal().unwrap(), "SportsCar");
}

#[test]
fn test_scenario_object_template_xml_serialization() {
    use openscenario_rs::types::entities::Pedestrian;

    let template =
        ScenarioObjectTemplate::new_pedestrian(Pedestrian::new_pedestrian("Walker1".to_string()));

    let xml = quick_xml::se::to_string(&template).unwrap();
    assert!(xml.contains("Pedestrian"));
}

#[test]
fn test_scenario_object_template_xml_parsing() {
    let xml = r#"<ScenarioObjectTemplate><CatalogReference catalogName="VehicleCatalog" entryName="VehicleTemplate"/></ScenarioObjectTemplate>"#;

    let template: ScenarioObjectTemplate = quick_xml::de::from_str(xml).unwrap();
    assert!(template.entity_catalog_reference().is_some());
}

#[test]
fn test_external_object_reference() {
    let ext_ref = ExternalObjectReference::new("Sedan");
    assert_eq!(ext_ref.name.as_literal().unwrap(), "Sedan");

    // Test construction via ::new
    let default_ref = ExternalObjectReference::new("Sedan");
    assert_eq!(default_ref.name.as_literal().unwrap(), "Sedan");
}

#[test]
fn test_external_object_reference_xml_parsing() {
    let xml = r#"
    <ExternalObjectReference name="SportsCar"/>
    "#;

    let ext_ref: ExternalObjectReference = quick_xml::de::from_str(xml).unwrap();
    assert_eq!(ext_ref.name.as_literal().unwrap(), "SportsCar");
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

    // Test custom creation
    let custom_selector = ByObjectType::new(ObjectType::Vehicle);
    assert_eq!(
        custom_selector.object_type,
        Value::Literal(ObjectType::Vehicle)
    );
}

#[test]
fn test_by_object_type_xml_parsing() {
    // XSD: ByObjectType has attribute `type`.
    let xml = r#"<ByObjectType type="pedestrian"/>"#;
    let selector: ByObjectType = quick_xml::de::from_str(xml).unwrap();
    assert_eq!(selector.object_type, Value::Literal(ObjectType::Pedestrian));
}

#[test]
fn test_by_type() {
    let type_selector = ByType::new(ObjectType::MiscellaneousObject);
    assert_eq!(
        type_selector.type_spec,
        Value::Literal(ObjectType::MiscellaneousObject)
    );

    // Test construction via ::new
    let default_selector = ByType::new(ObjectType::Vehicle);
    assert_eq!(
        default_selector.type_spec,
        Value::Literal(ObjectType::Vehicle)
    );
}

#[test]
fn test_by_type_xml_parsing() {
    // XSD: ByType has attribute `objectType`.
    let xml = r#"<ByType objectType="pedestrian"/>"#;
    let selector: ByType = quick_xml::de::from_str(xml).unwrap();
    assert_eq!(selector.type_spec, Value::Literal(ObjectType::Pedestrian));
}

#[test]
fn test_complex_entity_selection_scenario() {
    // Test a complex scenario with multiple selection types
    let members = SelectedEntities::from_by_type(ObjectType::Vehicle);
    let vehicle_selection = EntitySelection::new("VehicleSelection", members);

    let selected_vehicles = SelectedEntities::from_names(vec!["Car1", "Car2", "Truck1"]);
    let selected_pedestrians = SelectedEntities::from_names(vec!["Walker1", "Walker2"]);

    let vehicle_distribution = EntityDistribution::new(vec![
        EntityDistributionEntry::new(
            ScenarioObjectTemplate::new_vehicle(Vehicle::new_car("TestVehicle".to_string())),
            0.5,
        ),
        EntityDistributionEntry::new(
            ScenarioObjectTemplate::new_vehicle(Vehicle::new_car("TestVehicle".to_string())),
            0.3,
        ),
        EntityDistributionEntry::new(
            ScenarioObjectTemplate::new_vehicle(Vehicle::new_car("TestVehicle".to_string())),
            0.2,
        ),
    ])
    .unwrap();

    let vehicle_template =
        ScenarioObjectTemplate::new_vehicle(Vehicle::new_car("TestVehicle".to_string()));

    // Verify all components work together
    assert_eq!(vehicle_selection.members.by_type().len(), 1);
    assert_eq!(selected_vehicles.count(), 3);
    assert_eq!(selected_pedestrians.count(), 2);
    assert_eq!(vehicle_distribution.entries.len(), 3);
    assert_eq!(vehicle_distribution.total_weight(), 1.0);
    assert!(vehicle_template.vehicle().is_some());
}

#[test]
fn test_parameter_support_in_entity_selection() {
    // Test distribution with parameter weight and a parameterized external reference name
    let entry = EntityDistributionEntry {
        weight: Double::parameter("VehicleWeight".to_string()),
        scenario_object_template: ScenarioObjectTemplate {
            entity: EntityObjectChoice::ExternalObjectReference(ExternalObjectReference {
                name: OSString::parameter("VehicleName".to_string()),
            }),
            object_controller: Vec::new(),
        },
    };

    assert_eq!(
        entry
            .scenario_object_template
            .external_object_reference()
            .unwrap()
            .name
            .as_parameter()
            .unwrap(),
        "VehicleName"
    );
    assert_eq!(entry.weight.as_parameter().unwrap(), "VehicleWeight");
}

#[test]
fn test_all_defaults() {
    // these types no longer fabricate content via `Default` — each
    // requires an explicit, stated value instead. `SelectedEntities`'s choice
    // requires at least one branch member, so it is built from names or a
    // type rather than an empty constructor; `EntityDistribution` keeps its
    // explicit `new()` for the same reason.
    let _entity_selection =
        EntitySelection::new("Selection1", SelectedEntities::from_names(vec!["Ego"]));
    let _selected_entities = SelectedEntities::from_names(vec!["Ego"]);
    let _entity_distribution = EntityDistribution::new(vec![EntityDistributionEntry::new(
        ScenarioObjectTemplate::new_vehicle(Vehicle::new_car("TestVehicle".to_string())),
        1.0,
    )])
    .unwrap();
    let _entity_distribution_entry = EntityDistributionEntry::new(
        ScenarioObjectTemplate::new_vehicle(Vehicle::new_car("TestVehicle".to_string())),
        1.0,
    );
    let _scenario_object_template =
        ScenarioObjectTemplate::new_vehicle(Vehicle::new_car("TestVehicle".to_string()));
    let _external_object_reference = ExternalObjectReference::new("Sedan");
    let _by_object_type = ByObjectType::vehicle();
    let _by_type = ByType::new(ObjectType::Vehicle);

    // All values should be created without panicking
    assert!(true);
}

#[test]
fn test_serialization_roundtrip() {
    // Test that all types can be serialized and deserialized
    let members = SelectedEntities::from_by_type(ObjectType::Vehicle);
    let original_selection = EntitySelection::new("Selection1", members);
    let xml = quick_xml::se::to_string(&original_selection).unwrap();
    let parsed_selection: EntitySelection = quick_xml::de::from_str(&xml).unwrap();
    assert_eq!(original_selection, parsed_selection);

    let original_entities = SelectedEntities::from_names(vec!["A", "B", "C"]);
    let xml = quick_xml::se::to_string(&original_entities).unwrap();
    let parsed_entities: SelectedEntities = quick_xml::de::from_str(&xml).unwrap();
    assert_eq!(original_entities, parsed_entities);

    let templates = vec![
        ScenarioObjectTemplate::new_vehicle(Vehicle::new_car("TestVehicle".to_string())),
        ScenarioObjectTemplate::new_vehicle(Vehicle::new_car("TestVehicle".to_string())),
    ];
    let original_distribution = EntityDistribution::uniform(templates).unwrap();
    let xml = quick_xml::se::to_string(&original_distribution).unwrap();
    let parsed_distribution: EntityDistribution = quick_xml::de::from_str(&xml).unwrap();
    assert_eq!(original_distribution, parsed_distribution);
}
