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
        ByName, ByObjectType, ByType, EntityDistribution, EntityDistributionEntry, EntitySelection,
        ExternalObjectReference, ScenarioObjectTemplate, SelectedEntities, TemplateProperties,
        TemplateProperty,
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
    assert!(selection.by_name.is_none());

    // Test serialization
    let xml = quick_xml::se::to_string(&selection).unwrap();
    assert!(xml.contains("ByType"));
    assert!(xml.contains("objectType=\"vehicle\""));
}

#[test]
fn test_entity_selection_by_name() {
    let selection = EntitySelection::by_name("Ego*");

    assert!(selection.by_type.is_none());
    assert!(selection.by_name.is_some());
    assert_eq!(
        selection
            .by_name
            .as_ref()
            .unwrap()
            .name
            .as_literal()
            .unwrap(),
        "Ego*"
    );

    // Test serialization
    let xml = quick_xml::se::to_string(&selection).unwrap();
    assert!(xml.contains("ByName"));
    assert!(xml.contains("name=\"Ego*\""));
}

#[test]
fn test_entity_selection_by_type_and_name() {
    let selection = EntitySelection::by_type_and_name(ObjectType::Pedestrian, "Walker*");

    assert!(selection.by_type.is_some());
    assert!(selection.by_name.is_some());
    assert_eq!(
        selection.by_type.as_ref().unwrap().object_type,
        ObjectType::Pedestrian
    );
    assert_eq!(
        selection
            .by_name
            .as_ref()
            .unwrap()
            .name
            .as_literal()
            .unwrap(),
        "Walker*"
    );
}

#[test]
fn test_entity_selection_xml_parsing() {
    let xml = r#"
    <EntitySelection>
        <ByType objectType="vehicle"/>
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
    assert!(template.properties.is_none());
    assert!(template.external_object_reference.is_none());
}

#[test]
fn test_scenario_object_template_with_properties() {
    let mut template = ScenarioObjectTemplate::new("VehicleTemplate", ObjectType::Vehicle);
    template.add_property("color", "red");
    template.add_property("mass", "1500");
    template.add_property("maxSpeed", "60");

    assert!(template.properties.is_some());
    let props = template.properties.unwrap();
    assert_eq!(props.properties.len(), 3);

    // Check specific properties
    let color_prop = props
        .properties
        .iter()
        .find(|p| p.name.as_literal().map(|s| s.as_str()) == Some("color"))
        .unwrap();
    assert_eq!(color_prop.value.as_literal().unwrap(), "red");

    let mass_prop = props
        .properties
        .iter()
        .find(|p| p.name.as_literal().map(|s| s.as_str()) == Some("mass"))
        .unwrap();
    assert_eq!(mass_prop.value.as_literal().unwrap(), "1500");
}

#[test]
fn test_scenario_object_template_with_external_reference() {
    let template = ScenarioObjectTemplate::with_external_reference(
        "ExternalVehicle",
        ObjectType::Vehicle,
        "vehicles.xml",
        "SportsCar",
    );

    assert_eq!(template.name.as_literal().unwrap(), "ExternalVehicle");
    assert_eq!(template.object_type, ObjectType::Vehicle);
    assert!(template.external_object_reference.is_some());

    let ext_ref = template.external_object_reference.unwrap();
    assert_eq!(ext_ref.file.as_literal().unwrap(), "vehicles.xml");
    assert_eq!(ext_ref.name.as_literal().unwrap(), "SportsCar");
}

#[test]
fn test_scenario_object_template_xml_serialization() {
    let mut template = ScenarioObjectTemplate::new("TestTemplate", ObjectType::Pedestrian);
    template.add_property("height", "1.8");
    template.add_property("weight", "75");

    let xml = quick_xml::se::to_string(&template).unwrap();
    assert!(xml.contains("name=\"TestTemplate\""));
    assert!(xml.contains("objectType=\"pedestrian\""));
    assert!(xml.contains("Properties"));
    assert!(xml.contains("Property"));
    assert!(xml.contains("name=\"height\""));
    assert!(xml.contains("value=\"1.8\""));
}

#[test]
fn test_scenario_object_template_xml_parsing() {
    let xml = r#"
    <ScenarioObjectTemplate name="VehicleTemplate" objectType="vehicle">
        <Properties>
            <Property name="color" value="blue"/>
            <Property name="mass" value="1200"/>
        </Properties>
    </ScenarioObjectTemplate>
    "#;

    let template: ScenarioObjectTemplate = quick_xml::de::from_str(xml).unwrap();
    assert_eq!(template.name.as_literal().unwrap(), "VehicleTemplate");
    assert_eq!(template.object_type, ObjectType::Vehicle);
    assert!(template.properties.is_some());

    let props = template.properties.unwrap();
    assert_eq!(props.properties.len(), 2);

    let color_prop = props
        .properties
        .iter()
        .find(|p| p.name.as_literal().map(|s| s.as_str()) == Some("color"))
        .unwrap();
    assert_eq!(color_prop.value.as_literal().unwrap(), "blue");
}

#[test]
fn test_external_object_reference() {
    let ext_ref = ExternalObjectReference::new("objects/vehicles.xml", "Sedan");
    assert_eq!(ext_ref.file.as_literal().unwrap(), "objects/vehicles.xml");
    assert_eq!(ext_ref.name.as_literal().unwrap(), "Sedan");

    // Test default
    let default_ref = ExternalObjectReference::default();
    assert_eq!(default_ref.file.as_literal().unwrap(), "objects.xml");
    assert_eq!(default_ref.name.as_literal().unwrap(), "DefaultObject");
}

#[test]
fn test_external_object_reference_xml_parsing() {
    let xml = r#"
    <ExternalObjectReference file="catalogs/vehicles.xml" name="SportsCar"/>
    "#;

    let ext_ref: ExternalObjectReference = quick_xml::de::from_str(xml).unwrap();
    assert_eq!(ext_ref.file.as_literal().unwrap(), "catalogs/vehicles.xml");
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
    let xml = r#"<ByType objectType="pedestrian"/>"#;
    let selector: ByObjectType = quick_xml::de::from_str(xml).unwrap();
    assert_eq!(selector.object_type, ObjectType::Pedestrian);
}

#[test]
fn test_by_type() {
    let type_selector = ByType::new("custom_vehicle_type");
    assert_eq!(
        type_selector.type_spec.as_literal().unwrap(),
        "custom_vehicle_type"
    );

    // Test default
    let default_selector = ByType::default();
    assert_eq!(default_selector.type_spec.as_literal().unwrap(), "vehicle");
}

#[test]
fn test_by_type_xml_parsing() {
    let xml = r#"<ByType type="special_vehicle"/>"#;
    let selector: ByType = quick_xml::de::from_str(xml).unwrap();
    assert_eq!(selector.type_spec.as_literal().unwrap(), "special_vehicle");
}

#[test]
fn test_by_name() {
    let wildcard_selector = ByName::wildcard();
    assert_eq!(wildcard_selector.name.as_literal().unwrap(), "*");

    let prefix_selector = ByName::prefix("Ego");
    assert_eq!(prefix_selector.name.as_literal().unwrap(), "Ego*");

    let exact_selector = ByName::new("SpecificEntity");
    assert_eq!(exact_selector.name.as_literal().unwrap(), "SpecificEntity");

    // Test default
    let default_selector = ByName::default();
    assert_eq!(default_selector.name.as_literal().unwrap(), "*");
}

#[test]
fn test_by_name_xml_parsing() {
    let xml = r#"<ByName name="Vehicle*"/>"#;
    let selector: ByName = quick_xml::de::from_str(xml).unwrap();
    assert_eq!(selector.name.as_literal().unwrap(), "Vehicle*");
}

#[test]
fn test_template_property() {
    let prop = TemplateProperty {
        name: OSString::literal("maxSpeed".to_string()),
        value: OSString::literal("120".to_string()),
    };

    assert_eq!(prop.name.as_literal().unwrap(), "maxSpeed");
    assert_eq!(prop.value.as_literal().unwrap(), "120");

    // Test default
    let default_prop = TemplateProperty::default();
    assert_eq!(default_prop.name.as_literal().unwrap(), "property");
    assert_eq!(default_prop.value.as_literal().unwrap(), "value");
}

#[test]
fn test_template_properties() {
    let mut props = TemplateProperties::default();
    assert_eq!(props.properties.len(), 0);

    props.properties.push(TemplateProperty {
        name: OSString::literal("color".to_string()),
        value: OSString::literal("red".to_string()),
    });

    props.properties.push(TemplateProperty {
        name: OSString::literal("mass".to_string()),
        value: OSString::literal("1500".to_string()),
    });

    assert_eq!(props.properties.len(), 2);
}

#[test]
fn test_complex_entity_selection_scenario() {
    // Test a complex scenario with multiple selection types
    let vehicle_selection = EntitySelection::by_object_type(ObjectType::Vehicle);
    let pedestrian_selection = EntitySelection::by_name("Walker*");

    let selected_vehicles = SelectedEntities::from_names(vec!["Car1", "Car2", "Truck1"]);
    let selected_pedestrians = SelectedEntities::from_names(vec!["Walker1", "Walker2"]);

    let mut vehicle_distribution = EntityDistribution::new();
    vehicle_distribution.add_entry("Car1", 0.5);
    vehicle_distribution.add_entry("Car2", 0.3);
    vehicle_distribution.add_entry("Truck1", 0.2);

    let mut vehicle_template = ScenarioObjectTemplate::new("VehicleTemplate", ObjectType::Vehicle);
    vehicle_template.add_property("maxSpeed", "60");
    vehicle_template.add_property("color", "blue");

    // Verify all components work together
    assert_eq!(selected_vehicles.count(), 3);
    assert_eq!(selected_pedestrians.count(), 2);
    assert_eq!(vehicle_distribution.entries.len(), 3);
    assert_eq!(vehicle_distribution.total_weight(), 1.0);
    assert!(vehicle_template.properties.is_some());
    assert_eq!(vehicle_template.properties.unwrap().properties.len(), 2);
}

#[test]
fn test_parameter_support_in_entity_selection() {
    // Test that entity selection types support parameters
    let selection = EntitySelection {
        by_type: Some(ByObjectType {
            object_type: ObjectType::Vehicle,
        }),
        by_name: Some(ByName {
            name: OSString::parameter("EntityNamePattern".to_string()),
        }),
    };

    assert!(selection.by_name.is_some());
    assert_eq!(
        selection.by_name.unwrap().name.as_parameter().unwrap(),
        "EntityNamePattern"
    );

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
    let _by_name = ByName::default();
    let _template_properties = TemplateProperties::default();
    let _template_property = TemplateProperty::default();

    // All defaults should be created without panicking
    assert!(true);
}

#[test]
fn test_serialization_roundtrip() {
    // Test that all types can be serialized and deserialized
    let original_selection = EntitySelection::by_type_and_name(ObjectType::Vehicle, "Car*");
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
