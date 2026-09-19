//! A `<CatalogReference>` inside a `ScenarioObject` names a catalog and an entry, and
//! nothing else. The schema's `EntityObject` group offers one such element for every kind
//! of entity, so the document does not say whether the entry is a vehicle, a pedestrian or
//! a miscellaneous object.
//!
//! These tests guard against re-introducing a reference type that decides the entity kind
//! while parsing. An earlier shape wrapped the reference in an enum with a vehicle and a
//! pedestrian variant and deserialized it untagged, which made serde take the first variant
//! whose fields matched. Both variants had the same fields, so every reference in every
//! document was resolved as a vehicle, and the emitted XML was byte-identical to the input.
//! The error was therefore invisible to the round-trip and lossy checks alike.

use openscenario_rs::types::entities::{EntityCatalogReference, ScenarioObject};

const PEDESTRIAN_OBJECT: &str = concat!(
    r#"<ScenarioObject name="Pedestrian1">"#,
    r#"<CatalogReference catalogName="PedestrianCatalog" entryName="pedestrian_walker"/>"#,
    r#"</ScenarioObject>"#
);

const OBJECT_WITH_ASSIGNMENTS: &str = concat!(
    r#"<ScenarioObject name="Pedestrian1">"#,
    r#"<CatalogReference catalogName="PedestrianCatalog" entryName="pedestrian_walker">"#,
    r#"<ParameterAssignments>"#,
    r#"<ParameterAssignment parameterRef="Mass" value="80"/>"#,
    r#"<ParameterAssignment parameterRef="Height" value="1.8"/>"#,
    r#"</ParameterAssignments>"#,
    r#"</CatalogReference>"#,
    r#"</ScenarioObject>"#
);

#[test]
fn catalog_reference_is_parsed_without_an_entity_kind() {
    let object: ScenarioObject = quick_xml::de::from_str(PEDESTRIAN_OBJECT).unwrap();

    let catalog_reference = object
        .catalog_reference()
        .expect("the <CatalogReference> child should have been parsed");

    assert_eq!(
        catalog_reference,
        &EntityCatalogReference::new("PedestrianCatalog", "pedestrian_walker")
    );
    assert!(object.vehicle().is_none());
    assert!(object.pedestrian().is_none());
}

#[test]
fn catalog_reference_serializes_back_to_the_source_bytes() {
    let object: ScenarioObject = quick_xml::de::from_str(PEDESTRIAN_OBJECT).unwrap();
    let serialized = quick_xml::se::to_string(&object).unwrap();
    assert_eq!(serialized, PEDESTRIAN_OBJECT);
}

#[test]
fn parameter_assignments_below_a_catalog_reference_survive() {
    let object: ScenarioObject = quick_xml::de::from_str(OBJECT_WITH_ASSIGNMENTS).unwrap();

    let assignments = object
        .catalog_reference()
        .expect("the <CatalogReference> child should have been parsed")
        .parameter_assignments
        .as_ref()
        .expect("the <ParameterAssignments> child should have been parsed");

    assert_eq!(assignments.assignments.len(), 2);
    assert_eq!(assignments.assignments[0].parameter_ref.to_string(), "Mass");
    assert_eq!(
        assignments.assignments[1].parameter_ref.to_string(),
        "Height"
    );

    let serialized = quick_xml::se::to_string(&object).unwrap();
    assert_eq!(serialized, OBJECT_WITH_ASSIGNMENTS);
}
