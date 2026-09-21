//! The document root models the `xsd:choice` in the group `OpenScenarioCategory`
//! (`Schema/OpenSCENARIO.xsd:1538`) as parallel `Option` fields rather than as the
//! `$value` enum used for every other choice in this crate.
//!
//! That is forced by the schema's shape, not chosen. Two of the three branches are
//! single elements, but `ScenarioDefinition` (`:1989`) is a sequence of seven sibling
//! elements inlined into the root, and a `$value` field writes one element name per
//! instance. Making it a variant emits a wrapping `<ScenarioDefinition>` element the
//! schema never declares.
//!
//! These tests therefore pin two different things: that each branch parses and round
//! trips byte-exactly, and that the cardinality the type cannot enforce is at least
//! reported rather than silently resolved in favor of one branch.

use openscenario_rs::types::scenario::storyboard::{OpenScenario, OpenScenarioDocumentType};

const HDR: &str = concat!(
    r#"<FileHeader author="a" date="2024-01-01T00:00:00" description="d""#,
    r#" revMajor="1" revMinor="3"/>"#
);

fn document(body: &str) -> String {
    format!("<OpenSCENARIO>{HDR}{body}</OpenSCENARIO>")
}

const SCENARIO_BODY: &str = concat!(
    r#"<CatalogLocations/>"#,
    r#"<RoadNetwork/>"#,
    r#"<Entities/>"#,
    r#"<Storyboard><Init><Actions/></Init></Storyboard>"#,
);

const CATALOG_BODY: &str = r#"<Catalog name="c"/>"#;

const DISTRIBUTION_BODY: &str = concat!(
    r#"<ParameterValueDistribution>"#,
    r#"<ScenarioFile filepath="s.xosc"/>"#,
    r#"<Deterministic>"#,
    r#"<DeterministicSingleParameterDistribution parameterName="p">"#,
    r#"<DistributionSet><Element value="1"/></DistributionSet>"#,
    r#"</DeterministicSingleParameterDistribution>"#,
    r#"</Deterministic>"#,
    r#"</ParameterValueDistribution>"#,
);

fn parse(xml: &str) -> OpenScenario {
    quick_xml::de::from_str(xml).expect("the document must parse")
}

// --- Branch 1: ScenarioDefinition (`:1989`) ---

#[test]
fn scenario_branch_parses() {
    let doc = parse(&document(SCENARIO_BODY));
    assert_eq!(doc.document_type(), OpenScenarioDocumentType::Scenario);
    assert!(doc.entities.is_some());
    assert!(doc.storyboard.is_some());
    assert!(doc.catalog.is_none());
    assert!(doc.parameter_value_distribution.is_none());
}

#[test]
fn scenario_branch_round_trips_byte_exact() {
    let source = document(SCENARIO_BODY);
    let serialized = quick_xml::se::to_string(&parse(&source)).expect("serialize must succeed");
    assert_eq!(serialized.as_bytes(), source.as_bytes());
}

// --- Branch 2: CatalogDefinition (`:862`) ---
//
// The group contributes the single element `<Catalog>` of type `Catalog`. The root used
// to hold a wrapper struct around `CatalogContent` here, so it looked for a `<Catalog>`
// child inside the `<Catalog>` element: every real catalog document failed to parse with
// ``missing field `Catalog` ``, and serialization emitted two nested `<Catalog>` elements.

#[test]
fn catalog_branch_parses() {
    let doc = parse(&document(CATALOG_BODY));
    assert_eq!(doc.document_type(), OpenScenarioDocumentType::Catalog);
    let catalog = doc.catalog.expect("the Catalog branch must be populated");
    assert_eq!(catalog.name.to_string(), "c");
}

#[test]
fn catalog_branch_round_trips_byte_exact() {
    let source = document(CATALOG_BODY);
    let serialized = quick_xml::se::to_string(&parse(&source)).expect("serialize must succeed");
    assert_eq!(serialized.as_bytes(), source.as_bytes());
}

#[test]
fn catalog_branch_emits_exactly_one_catalog_element() {
    let serialized =
        quick_xml::se::to_string(&parse(&document(CATALOG_BODY))).expect("serialize must succeed");
    assert_eq!(serialized.matches("<Catalog").count(), 1);
}

#[test]
fn a_catalog_document_from_the_corpus_parses_through_the_root() {
    let path = concat!(
        "conformance/corpus/OSC-NCAP-scenarios/OpenSCENARIO/NCAP/",
        "Catalogs/Vehicles/Vehicles.xosc"
    );
    let doc = openscenario_rs::parse_from_file(path).expect("a catalog document parses");
    assert!(doc.is_catalog());
    assert!(!doc.catalog.expect("populated").vehicles.is_empty());
}

// --- Branch 3: ParameterValueDistributionDefinition (`:1667`) ---

#[test]
fn distribution_branch_parses() {
    let doc = parse(&document(DISTRIBUTION_BODY));
    assert_eq!(
        doc.document_type(),
        OpenScenarioDocumentType::ParameterVariation
    );
    assert!(doc.parameter_value_distribution.is_some());
}

#[test]
fn distribution_branch_round_trips_byte_exact() {
    let source = document(DISTRIBUTION_BODY);
    let serialized = quick_xml::se::to_string(&parse(&source)).expect("serialize must succeed");
    assert_eq!(serialized.as_bytes(), source.as_bytes());
}

// --- Cardinality, asserted one claim per test ---
//
// `$value` would make both of these parse errors. Here they parse, so the check that is
// available is that neither is reported as a document of any kind.

#[test]
fn a_root_naming_no_branch_is_not_any_document_type() {
    let doc = parse(&document(""));
    assert_eq!(doc.document_type(), OpenScenarioDocumentType::Unknown);
}

#[test]
fn a_root_naming_two_branches_is_not_a_scenario() {
    let doc = parse(&document(&format!("{SCENARIO_BODY}{CATALOG_BODY}")));
    assert_ne!(doc.document_type(), OpenScenarioDocumentType::Scenario);
}

#[test]
fn a_root_naming_two_branches_is_not_a_catalog() {
    let doc = parse(&document(&format!("{SCENARIO_BODY}{CATALOG_BODY}")));
    assert_ne!(doc.document_type(), OpenScenarioDocumentType::Catalog);
}

#[test]
fn a_root_naming_two_branches_keeps_both_and_reports_neither() {
    let doc = parse(&document(&format!("{SCENARIO_BODY}{CATALOG_BODY}")));
    assert!(doc.storyboard.is_some());
    assert!(doc.catalog.is_some());
    assert_eq!(doc.document_type(), OpenScenarioDocumentType::Unknown);
}

/// A partial scenario branch names one branch but is not a complete scenario document.
#[test]
fn an_incomplete_scenario_branch_is_not_a_scenario() {
    let doc = parse(&document(r#"<CatalogLocations/><RoadNetwork/>"#));
    assert_eq!(doc.document_type(), OpenScenarioDocumentType::Unknown);
}
