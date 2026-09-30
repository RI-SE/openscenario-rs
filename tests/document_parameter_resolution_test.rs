//! Resolving a parsed document against its own `<ParameterDeclarations>`, following the scope
//! rule of ASAM OpenSCENARIO XML section 9.1: a parameter is visible in the subtree of the
//! element that declares it, and the smallest enclosing declaration wins.
//!
//! `parse_str` keeps every reference as written; `parse_str_resolved` replaces each one by its
//! value. Each claim is its own test, so that one failing claim cannot hide the next.

use openscenario_rs::parser::resolve::resolve_parameters;
use openscenario_rs::types::basic::Value;
use openscenario_rs::types::entities::EntityObjectChoice;
use openscenario_rs::{parse_str, parse_str_resolved, OpenScenario};
use std::path::Path;

const FIXTURE: &str = include_str!("data/document_parameter_resolution.xosc");

fn ego_property_value(document: &OpenScenario) -> Value<String> {
    let entities = document.entities.as_ref().expect("entities present");
    let EntityObjectChoice::Vehicle(vehicle) = &entities.scenario_objects[0].entity else {
        panic!("Ego is an inline Vehicle");
    };
    vehicle
        .properties
        .as_ref()
        .expect("properties present")
        .properties[0]
        .value
        .clone()
}

/// The `value` of every `<AbsoluteTargetSpeed>` in the serialized document, in document order.
fn target_speeds(document: &OpenScenario) -> Vec<String> {
    // Unformatted, so that each element keeps its attributes on one line.
    let xml = quick_xml::se::to_string(document).expect("document serializes");
    xml.split("<AbsoluteTargetSpeed value=\"")
        .skip(1)
        .map(|rest| rest.split('"').next().unwrap().to_string())
        .collect()
}

/// A scenario around the given global declarations and one `Property` value.
fn scenario_with(declarations: &str, property_value: &str) -> String {
    FIXTURE
        .replace(
            r#"<Property name="speedLimit" value="$p"/>"#,
            &format!(r#"<Property name="speedLimit" value="{property_value}"/>"#),
        )
        .replace(
            r#"<ParameterDeclaration name="p" parameterType="string" value="120"/>"#,
            declarations,
        )
}

// --- A declared parameter feeding a Property ----------------------------------------------

#[test]
fn a_property_parameter_resolves_through_the_resolving_entry_point() {
    let document = parse_str_resolved(FIXTURE).expect("fixture resolves");
    assert_eq!(
        ego_property_value(&document),
        Value::Literal("120".to_string())
    );
}

// --- Section 9.1's shadowing example -------------------------------------------------------

/// "If there are multiple parameters with the same name and overlapping scopes in the scenario,
/// only the parameter with the smallest scope that subsumes the location is accessible."
/// The `Overtake` maneuver redeclares the global `ego_speed`; its sibling `Follow` does not.
#[test]
fn a_maneuver_declaration_shadows_the_global_inside_the_maneuver_only() {
    let document = parse_str_resolved(FIXTURE).expect("fixture resolves");
    assert_eq!(target_speeds(&document), vec!["30", "10"]);
}

#[test]
fn without_resolution_every_use_stays_a_reference() {
    let document = parse_str(FIXTURE).expect("fixture parses");
    assert_eq!(
        ego_property_value(&document),
        Value::Parameter("p".to_string())
    );
    assert_eq!(target_speeds(&document), vec!["$ego_speed", "$ego_speed"]);
}

#[test]
fn an_element_sees_its_own_declarations_in_its_own_attributes() {
    // The scope is "the subtree rooted in the element", which includes the element itself.
    let xml = FIXTURE.replace(
        r#"<Maneuver name="Overtake">"#,
        r#"<Maneuver name="$ego_speed">"#,
    );
    let resolved = resolve_parameters(&xml, Path::new("")).expect("fixture resolves");
    assert!(resolved.contains(r#"<Maneuver name="30">"#), "{resolved}");
}

// --- Expressions ---------------------------------------------------------------------------

#[test]
fn an_expression_referencing_a_parameter_resolves() {
    let document = parse_str_resolved(FIXTURE).expect("fixture resolves");
    let entities = document.entities.as_ref().unwrap();
    let EntityObjectChoice::Vehicle(vehicle) = &entities.scenario_objects[0].entity else {
        panic!("Ego is an inline Vehicle");
    };
    assert_eq!(vehicle.performance.max_speed, Value::Literal(50.0));
}

#[test]
fn a_failing_expression_fails_with_its_cause_and_location() {
    let xml = FIXTURE.replace(
        r#"maxSpeed="${$ego_speed + 40}""#,
        r#"maxSpeed="${$ego_speed / 0}""#,
    );
    let msg = parse_str_resolved(&xml).unwrap_err().to_string();
    assert!(msg.to_lowercase().contains("division by zero"), "{msg}");
    assert!(msg.contains("maxSpeed"), "{msg}");
    assert!(
        msg.contains("ScenarioObject[@name='Ego']/Vehicle[@name='car']/Performance"),
        "{msg}"
    );
}

#[test]
fn a_boolean_expression_is_evaluated_as_a_boolean() {
    let xml = FIXTURE.replace(
        r#"<Actors selectTriggeringEntities="false">"#,
        r#"<Actors selectTriggeringEntities="${$ego_speed > 20}">"#,
    );
    let resolved = resolve_parameters(&xml, Path::new("")).expect("fixture resolves");
    assert!(
        resolved.contains(r#"selectTriggeringEntities="false""#),
        "{resolved}"
    );
}

// --- Errors carry the element path -----------------------------------------------------------

#[test]
fn an_undeclared_parameter_fails_naming_it_and_its_element_path() {
    let xml = scenario_with(
        r#"<ParameterDeclaration name="p" parameterType="string" value="120"/>"#,
        "$undeclared",
    );
    let msg = parse_str_resolved(&xml).unwrap_err().to_string();
    assert!(msg.contains("'undeclared'"), "{msg}");
    assert!(
        msg.contains(
            "/OpenSCENARIO/Entities/ScenarioObject[@name='Ego']/Vehicle[@name='car']/Properties/\
             Property[@name='speedLimit']"
        ),
        "{msg}"
    );
    assert!(msg.contains("line 24"), "{msg}");
}

#[test]
fn a_parameter_of_a_sibling_subtree_is_not_visible() {
    // `Overtake`'s declaration of `local` does not reach the `Follow` maneuver.
    let xml = FIXTURE
        .replace(
            r#"<ParameterDeclaration name="ego_speed" parameterType="double" value="30"/>"#,
            r#"<ParameterDeclaration name="ego_speed" parameterType="double" value="30"/>
              <ParameterDeclaration name="local" parameterType="string" value="x"/>"#,
        )
        .replace(r#"<Event name="Outside""#, r#"<Event name="$local""#);
    let msg = parse_str_resolved(&xml).unwrap_err().to_string();
    assert!(msg.contains("'local'"), "{msg}");
    assert!(msg.contains("Maneuver[@name='Follow']"), "{msg}");
}

#[test]
fn an_invalid_declaration_fails_naming_it_and_its_path() {
    let xml = scenario_with(
        r#"<ParameterDeclaration name="p" parameterType="int" value="fast"/>"#,
        "$p",
    );
    let msg = parse_str_resolved(&xml).unwrap_err().to_string();
    assert!(msg.contains("'p'"), "{msg}");
    assert!(
        msg.contains("/OpenSCENARIO/ParameterDeclarations/ParameterDeclaration[@name='p']"),
        "{msg}"
    );
}

// --- Declarations referencing declarations -------------------------------------------------

#[test]
fn a_declaration_referencing_an_earlier_one_resolves_and_is_written_back() {
    let resolved = resolve_parameters(FIXTURE, Path::new("")).expect("fixture resolves");
    assert!(
        resolved.contains(
            r#"<ParameterDeclaration name="double_speed" parameterType="double" value="20"/>"#
        ),
        "{resolved}"
    );
}

#[test]
fn a_declaration_referencing_a_later_one_fails_naming_both() {
    let xml = scenario_with(
        r#"<ParameterDeclaration name="p" parameterType="string" value="$later"/>
           <ParameterDeclaration name="later" parameterType="string" value="x"/>"#,
        "$p",
    );
    let msg = parse_str_resolved(&xml).unwrap_err().to_string();
    assert!(msg.contains("'p'"), "{msg}");
    assert!(msg.contains("`later`"), "{msg}");
    assert!(msg.contains("declared after it"), "{msg}");
}

// --- What is not resolved --------------------------------------------------------------------

#[test]
fn a_document_without_references_is_written_back_unchanged() {
    let xml = FIXTURE
        .replace("$p", "120")
        .replace("$ego_speed", "10")
        .replace("${10 * 2}", "20")
        .replace("${10 + 40}", "50");
    assert!(!xml.contains('$'));
    assert_eq!(
        resolve_parameters(&xml, Path::new("")).expect("resolves"),
        xml
    );
}

#[test]
fn a_parameter_value_distribution_has_nothing_to_resolve() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<OpenSCENARIO>
  <FileHeader revMajor="1" revMinor="3" date="2024-01-01T00:00:00" description="d" author="a"/>
  <ParameterValueDistribution>
    <ScenarioFile filepath="scenario.xosc"/>
    <Deterministic>
      <DeterministicSingleParameterDistribution parameterName="ego_speed">
        <DistributionSet>
          <Element value="10"/>
          <Element value="20"/>
        </DistributionSet>
      </DeterministicSingleParameterDistribution>
    </Deterministic>
  </ParameterValueDistribution>
</OpenSCENARIO>"#;
    assert_eq!(
        resolve_parameters(xml, Path::new("")).expect("resolves"),
        xml
    );
    parse_str_resolved(xml).expect("distribution parses");
}
