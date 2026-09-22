//! Regression: `ScenarioBuilder` used to hardcode `variable_declarations` and
//! `monitor_declarations` to `None` in `build_scenario`, so no builder call could reach
//! either element even though the XSD group `ScenarioDefinition` (`Schema/OpenSCENARIO.xsd:1989`)
//! declares both `minOccurs="0"`, the same cardinality as `ParameterDeclarations`. These tests
//! prove both are now reachable through the same `with_*`/`add_*` idiom `ParameterDeclarations`
//! already used, and that what the builder emits round-trips byte-exactly through the source
//! document a real consumer would write.

#![cfg(feature = "builder")]

use openscenario_rs::types::basic::OSString;
use openscenario_rs::types::catalogs::locations::CatalogLocations;
use openscenario_rs::types::enums::ParameterType;
use openscenario_rs::types::road::RoadNetwork;
use openscenario_rs::types::scenario::monitors::MonitorDeclarations;
use openscenario_rs::types::scenario::variables::VariableDeclarations;
use openscenario_rs::ScenarioBuilder;

/// The compact form `quick_xml::se::to_string` emits for the minimal scenario document with
/// one `VariableDeclaration` added through the builder. The date is overwritten to a fixed
/// value after `build()` because `with_header` stamps the call time, which a byte-exact
/// comparison cannot depend on.
const ONE_VARIABLE_SCENARIO: &str = concat!(
    r#"<OpenSCENARIO>"#,
    r#"<FileHeader author="Author" date="2024-01-01T00:00:00" description="Test" revMajor="1" revMinor="3"/>"#,
    r#"<VariableDeclarations><VariableDeclaration name="lap_count" variableType="int" value="0"/></VariableDeclarations>"#,
    r#"<CatalogLocations/><RoadNetwork/><Entities/>"#,
    r#"<Storyboard><Init><Actions><GlobalAction><EnvironmentAction><Environment name="Environment"/></EnvironmentAction></GlobalAction></Actions></Init></Storyboard>"#,
    r#"</OpenSCENARIO>"#,
);

const ONE_MONITOR_SCENARIO: &str = concat!(
    r#"<OpenSCENARIO>"#,
    r#"<FileHeader author="Author" date="2024-01-01T00:00:00" description="Test" revMajor="1" revMinor="3"/>"#,
    r#"<MonitorDeclarations><MonitorDeclaration name="collision_seen" value="false"/></MonitorDeclarations>"#,
    r#"<CatalogLocations/><RoadNetwork/><Entities/>"#,
    r#"<Storyboard><Init><Actions><GlobalAction><EnvironmentAction><Environment name="Environment"/></EnvironmentAction></GlobalAction></Actions></Init></Storyboard>"#,
    r#"</OpenSCENARIO>"#,
);

fn minimal_builder_with_header() -> ScenarioBuilder<openscenario_rs::builder::scenario::HasHeader> {
    ScenarioBuilder::new()
        .with_header("Test", "Author")
        .with_catalog_locations(CatalogLocations::default())
        .with_road_network(RoadNetwork::default())
}

#[test]
fn add_variable_is_reachable_and_round_trips_byte_exact() {
    let mut scenario = minimal_builder_with_header()
        .add_variable("lap_count", ParameterType::Int, "0")
        .with_entities()
        .with_storyboard(|storyboard| storyboard)
        .build()
        .expect("a scenario with one variable declaration must build");

    assert!(
        scenario.variable_declarations.is_some(),
        "add_variable() must reach VariableDeclarations; before this fix build_scenario hardcoded None"
    );
    let vars = scenario.variable_declarations.as_ref().unwrap();
    assert_eq!(vars.variable_declarations.len(), 1);

    scenario.file_header.date = OSString::literal("2024-01-01T00:00:00".to_string());
    let serialized = quick_xml::se::to_string(&scenario).expect("serialize must succeed");
    assert_eq!(
        serialized.as_bytes(),
        ONE_VARIABLE_SCENARIO.as_bytes(),
        "serialized output must equal the source document byte-for-byte"
    );

    let reparsed: openscenario_rs::types::scenario::storyboard::OpenScenario =
        quick_xml::de::from_str(&serialized).expect("round-trip reparse must succeed");
    assert_eq!(
        reparsed
            .variable_declarations
            .unwrap()
            .variable_declarations
            .len(),
        1
    );
}

#[test]
fn with_variables_replaces_rather_than_accumulates() {
    let mut vars = VariableDeclarations::default();
    vars.add_variable(
        "seen".to_string(),
        ParameterType::Boolean,
        "false".to_string(),
    );

    let scenario = minimal_builder_with_header()
        .add_variable("dropped", ParameterType::Int, "1")
        .with_variables(vars)
        .with_entities()
        .with_storyboard(|storyboard| storyboard)
        .build()
        .expect("build must succeed");

    let declared = scenario.variable_declarations.unwrap();
    assert_eq!(
        declared.variable_declarations.len(),
        1,
        "with_variables() replaces prior declarations, matching with_parameters()"
    );
    assert_eq!(
        declared.variable_declarations[0].name.as_literal().unwrap(),
        "seen"
    );
}

#[test]
fn add_monitor_is_reachable_and_round_trips_byte_exact() {
    let mut scenario = minimal_builder_with_header()
        .add_monitor("collision_seen", false)
        .with_entities()
        .with_storyboard(|storyboard| storyboard)
        .build()
        .expect("a scenario with one monitor declaration must build");

    assert!(
        scenario.monitor_declarations.is_some(),
        "add_monitor() must reach MonitorDeclarations; before this fix build_scenario hardcoded None"
    );
    let monitors = scenario.monitor_declarations.as_ref().unwrap();
    assert_eq!(monitors.monitor_declarations.len(), 1);

    scenario.file_header.date = OSString::literal("2024-01-01T00:00:00".to_string());
    let serialized = quick_xml::se::to_string(&scenario).expect("serialize must succeed");
    assert_eq!(
        serialized.as_bytes(),
        ONE_MONITOR_SCENARIO.as_bytes(),
        "serialized output must equal the source document byte-for-byte"
    );

    let reparsed: openscenario_rs::types::scenario::storyboard::OpenScenario =
        quick_xml::de::from_str(&serialized).expect("round-trip reparse must succeed");
    assert_eq!(
        reparsed
            .monitor_declarations
            .unwrap()
            .monitor_declarations
            .len(),
        1
    );
}

#[test]
fn with_monitors_replaces_rather_than_accumulates() {
    let mut monitors = MonitorDeclarations::default();
    monitors.add_monitor("kept".to_string(), true);

    let scenario = minimal_builder_with_header()
        .add_monitor("dropped", false)
        .with_monitors(monitors)
        .with_entities()
        .with_storyboard(|storyboard| storyboard)
        .build()
        .expect("build must succeed");

    let declared = scenario.monitor_declarations.unwrap();
    assert_eq!(
        declared.monitor_declarations.len(),
        1,
        "with_monitors() replaces prior declarations, matching with_parameters()"
    );
    assert_eq!(
        declared.monitor_declarations[0].name.as_literal().unwrap(),
        "kept"
    );
}

/// The three optional `ScenarioDefinition` members coexist: setting one must not clobber
/// another, since each is stored as its own field in `PartialScenarioData`.
#[test]
fn parameter_variable_and_monitor_declarations_coexist() {
    let scenario = minimal_builder_with_header()
        .add_parameter("initial_speed", ParameterType::Double, "25.0")
        .add_variable("lap_count", ParameterType::Int, "0")
        .add_monitor("collision_seen", false)
        .with_entities()
        .with_storyboard(|storyboard| storyboard)
        .build()
        .expect("build must succeed");

    assert!(scenario.parameter_declarations.is_some());
    assert!(scenario.variable_declarations.is_some());
    assert!(scenario.monitor_declarations.is_some());
}
