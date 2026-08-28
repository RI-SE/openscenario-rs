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

use openscenario_rs::types::{
    basic::{Double, OSString},
    entities::{
        ByObjectType, ByType, EntityDistribution, EntityDistributionEntry, EntitySelection,
        ExternalObjectReference, ScenarioObjectTemplate, SelectedEntities,
    },
    enums::ObjectType,
};

#[test]
fn test_entity_selection_by_object_type() {
    let selection = EntitySelection::by_object_type(ObjectType::Vehicle);

    assert!(selection.by_type.is_some());
    assert_eq!(
        selection.by_type.as_ref().unwrap().object_type,
        ObjectType::Vehicle
    );

    // Test serialization
    let xml = quick_xml::se::to_string(&selection).unwrap();
    assert!(xml.contains("ByType"));
    assert!(xml.contains("type=\"vehicle\""));
}

#[test]
fn test_entity_selection_xml_parsing() {
    let xml = r#"
    <EntitySelection>
        <ByType type="vehicle"/>
    </EntitySelection>
    "#;

    let selection: EntitySelection = quick_xml::de::from_str(xml).unwrap();
    assert!(selection.by_type.is_some());
    assert_eq!(selection.by_type.unwrap().object_type, ObjectType::Vehicle);
}

#[test]
fn test_selected_entities_creation() {
    let mut entities = SelectedEntities::new();
    assert_eq!(entities.count(), 0);

    entities.add_entity("Ego");
    entities.add_entity("Target1");
    entities.add_entity("Target2");
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
        .entity_refs
        .iter()
        .filter_map(|e| e.entity_ref.as_literal().cloned())
        .collect();
    assert!(entity_names.contains(&"Ego".to_string()));
    assert!(entity_names.contains(&"Target1".to_string()));
    assert!(entity_names.contains(&"Target2".to_string()));
}

#[test]
fn test_entity_distribution_creation() {
    let mut distribution = EntityDistribution::new();
    distribution.add_entry("Car1", 0.6);
    distribution.add_entry("Car2", 0.4);

    assert_eq!(distribution.entries.len(), 2);
    assert_eq!(distribution.total_weight(), 1.0);

    // Test uniform distribution
    let uniform_dist = EntityDistribution::uniform(vec!["A", "B", "C", "D"]);
    assert_eq!(uniform_dist.entries.len(), 4);
    assert!((uniform_dist.total_weight() - 1.0).abs() < f64::EPSILON);

    // Each entry should have weight 0.25
    for entry in &uniform_dist.entries {
        assert!((entry.weight.as_literal().unwrap() - 0.25).abs() < f64::EPSILON);
    }
}

#[test]
fn test_entity_distribution_xml_serialization() {
    let distribution = EntityDistribution::uniform(vec!["Car1", "Car2"]);
    let xml = quick_xml::se::to_string(&distribution).unwrap();

    assert!(xml.contains("EntityDistributionEntry"));
    assert!(xml.contains("entityRef=\"Car1\""));
    assert!(xml.contains("entityRef=\"Car2\""));
    assert!(xml.contains("weight=\"0.5\""));
}

#[test]
fn test_entity_distribution_xml_parsing() {
    let xml = r#"
    <EntityDistribution>
        <EntityDistributionEntry entityRef="Car1" weight="0.6"/>
        <EntityDistributionEntry entityRef="Car2" weight="0.4"/>
    </EntityDistribution>
    "#;

    let distribution: EntityDistribution = quick_xml::de::from_str(xml).unwrap();
    assert_eq!(distribution.entries.len(), 2);
    assert_eq!(distribution.total_weight(), 1.0);

    let car1_entry = distribution
        .entries
        .iter()
        .find(|e| e.entity_ref.as_literal().map(|s| s.as_str()) == Some("Car1"))
        .unwrap();
    assert_eq!(car1_entry.weight.as_literal().unwrap(), &0.6);

    let car2_entry = distribution
        .entries
        .iter()
        .find(|e| e.entity_ref.as_literal().map(|s| s.as_str()) == Some("Car2"))
        .unwrap();
    assert_eq!(car2_entry.weight.as_literal().unwrap(), &0.4);
}

#[test]
fn test_entity_distribution_entry() {
    let entry = EntityDistributionEntry::new("TestEntity", 0.75);
    assert_eq!(entry.entity_ref.as_literal().unwrap(), "TestEntity");
    assert_eq!(entry.weight.as_literal().unwrap(), &0.75);

    // Test default
    let default_entry = EntityDistributionEntry::default();
    assert_eq!(
        default_entry.entity_ref.as_literal().unwrap(),
        "DefaultEntity"
    );
    assert_eq!(default_entry.weight.as_literal().unwrap(), &1.0);
}

#[test]
fn test_scenario_object_template_basic() {
    let template = ScenarioObjectTemplate::new("VehicleTemplate", ObjectType::Vehicle);
    assert_eq!(template.name.as_literal().unwrap(), "VehicleTemplate");
    assert_eq!(template.object_type, ObjectType::Vehicle);
    assert!(template.external_object_reference.is_none());
}

#[test]
fn test_scenario_object_template_with_external_reference() {
    let template = ScenarioObjectTemplate::with_external_reference(
        "ExternalVehicle",
        ObjectType::Vehicle,
        "SportsCar",
    );

    assert_eq!(template.name.as_literal().unwrap(), "ExternalVehicle");
    assert_eq!(template.object_type, ObjectType::Vehicle);
    assert!(template.external_object_reference.is_some());

    let ext_ref = template.external_object_reference.unwrap();
    assert_eq!(ext_ref.name.as_literal().unwrap(), "SportsCar");
}

#[test]
fn test_scenario_object_template_xml_serialization() {
    let template = ScenarioObjectTemplate::new("TestTemplate", ObjectType::Pedestrian);

    let xml = quick_xml::se::to_string(&template).unwrap();
    assert!(xml.contains("name=\"TestTemplate\""));
    assert!(xml.contains("objectType=\"pedestrian\""));
}

#[test]
fn test_scenario_object_template_xml_parsing() {
    let xml = r#"<ScenarioObjectTemplate name="VehicleTemplate" objectType="vehicle"/>"#;

    let template: ScenarioObjectTemplate = quick_xml::de::from_str(xml).unwrap();
    assert_eq!(template.name.as_literal().unwrap(), "VehicleTemplate");
    assert_eq!(template.object_type, ObjectType::Vehicle);
}

#[test]
fn test_external_object_reference() {
    let ext_ref = ExternalObjectReference::new("Sedan");
    assert_eq!(ext_ref.name.as_literal().unwrap(), "Sedan");

    // Test default
    let default_ref = ExternalObjectReference::default();
    assert_eq!(default_ref.name.as_literal().unwrap(), "DefaultObject");
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
    assert_eq!(vehicle_selector.object_type, ObjectType::Vehicle);

    let pedestrian_selector = ByObjectType::pedestrian();
    assert_eq!(pedestrian_selector.object_type, ObjectType::Pedestrian);

    let misc_selector = ByObjectType::miscellaneous_object();
    assert_eq!(misc_selector.object_type, ObjectType::MiscellaneousObject);

    // Test custom creation
    let custom_selector = ByObjectType::new(ObjectType::Vehicle);
    assert_eq!(custom_selector.object_type, ObjectType::Vehicle);
}

#[test]
fn test_by_object_type_xml_parsing() {
    // XSD: ByObjectType has attribute `type`.
    let xml = r#"<ByObjectType type="pedestrian"/>"#;
    let selector: ByObjectType = quick_xml::de::from_str(xml).unwrap();
    assert_eq!(selector.object_type, ObjectType::Pedestrian);
}

#[test]
fn test_by_type() {
    let type_selector = ByType::new(ObjectType::MiscellaneousObject);
    assert_eq!(type_selector.type_spec, ObjectType::MiscellaneousObject);

    // Test default
    let default_selector = ByType::default();
    assert_eq!(default_selector.type_spec, ObjectType::Vehicle);
}

#[test]
fn test_by_type_xml_parsing() {
    // XSD: ByType has attribute `objectType`.
    let xml = r#"<ByType objectType="pedestrian"/>"#;
    let selector: ByType = quick_xml::de::from_str(xml).unwrap();
    assert_eq!(selector.type_spec, ObjectType::Pedestrian);
}

#[test]
fn test_complex_entity_selection_scenario() {
    // Test a complex scenario with multiple selection types
    let vehicle_selection = EntitySelection::by_object_type(ObjectType::Vehicle);

    let selected_vehicles = SelectedEntities::from_names(vec!["Car1", "Car2", "Truck1"]);
    let selected_pedestrians = SelectedEntities::from_names(vec!["Walker1", "Walker2"]);

    let mut vehicle_distribution = EntityDistribution::new();
    vehicle_distribution.add_entry("Car1", 0.5);
    vehicle_distribution.add_entry("Car2", 0.3);
    vehicle_distribution.add_entry("Truck1", 0.2);

    let vehicle_template = ScenarioObjectTemplate::new("VehicleTemplate", ObjectType::Vehicle);

    // Verify all components work together
    assert!(vehicle_selection.by_type.is_some());
    assert_eq!(selected_vehicles.count(), 3);
    assert_eq!(selected_pedestrians.count(), 2);
    assert_eq!(vehicle_distribution.entries.len(), 3);
    assert_eq!(vehicle_distribution.total_weight(), 1.0);
    assert_eq!(vehicle_template.name.as_literal().unwrap(), "VehicleTemplate");
}

#[test]
fn test_parameter_support_in_entity_selection() {
    // Test distribution with parameter weights
    let entry = EntityDistributionEntry {
        entity_ref: OSString::parameter("VehicleName".to_string()),
        weight: Double::parameter("VehicleWeight".to_string()),
    };

    assert_eq!(entry.entity_ref.as_parameter().unwrap(), "VehicleName");
    assert_eq!(entry.weight.as_parameter().unwrap(), "VehicleWeight");
}

#[test]
fn test_all_defaults() {
    // Test that all types have working defaults
    let _entity_selection = EntitySelection::default();
    let _selected_entities = SelectedEntities::default();
    let _entity_distribution = EntityDistribution::default();
    let _entity_distribution_entry = EntityDistributionEntry::default();
    let _scenario_object_template = ScenarioObjectTemplate::default();
    let _external_object_reference = ExternalObjectReference::default();
    let _by_object_type = ByObjectType::default();
    let _by_type = ByType::default();

    // All defaults should be created without panicking
    assert!(true);
}

#[test]
fn test_serialization_roundtrip() {
    // Test that all types can be serialized and deserialized
    let original_selection = EntitySelection::by_object_type(ObjectType::Vehicle);
    let xml = quick_xml::se::to_string(&original_selection).unwrap();
    let parsed_selection: EntitySelection = quick_xml::de::from_str(&xml).unwrap();
    assert_eq!(original_selection, parsed_selection);

    let original_entities = SelectedEntities::from_names(vec!["A", "B", "C"]);
    let xml = quick_xml::se::to_string(&original_entities).unwrap();
    let parsed_entities: SelectedEntities = quick_xml::de::from_str(&xml).unwrap();
    assert_eq!(original_entities, parsed_entities);

    let original_distribution = EntityDistribution::uniform(vec!["X", "Y"]);
    let xml = quick_xml::se::to_string(&original_distribution).unwrap();
    let parsed_distribution: EntityDistribution = quick_xml::de::from_str(&xml).unwrap();
    assert_eq!(original_distribution, parsed_distribution);
}
