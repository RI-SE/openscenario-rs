//! Regression tests for schema-optional omissions (GitHub issue: mandatory-field parsing)
//!
//! A schema-valid scenario may omit every field the XSD marks optional. These tests lock in
//! that such sparse documents parse, and that omitted optionals are not re-emitted.

use openscenario_rs::parser::xml::{parse_from_str, serialize_to_string};

const SPARSE: &str = include_str!("data/minimal_sparse.xosc");

#[test]
fn sparse_scenario_parses() {
    let scenario = parse_from_str(SPARSE).expect("sparse scenario must parse");

    let init = &scenario
        .storyboard
        .as_ref()
        .expect("storyboard present")
        .init;
    let env_action = init.actions.global_actions[0]
        .environment_action
        .as_ref()
        .expect("environment action present");
    let env = env_action
        .environment
        .as_ref()
        .expect("inline environment branch");

    assert_eq!(env.name.as_literal().unwrap(), "SparseEnvironment");
    // Only @name given: everything else must be None
    assert!(env.parameter_declarations.is_none());
    assert!(env.time_of_day.is_none());
    assert!(env.road_condition.is_none());

    let weather = env.weather.as_ref().expect("weather present");
    // Deprecated cloudState omitted, modern fractionalCloudCover used
    assert!(weather.cloud_state.is_none());
    assert!(weather.fractional_cloud_cover.is_some());
    assert!(weather.sun.is_none());
    assert!(weather.fog.is_none());
    assert!(weather.precipitation.is_none());
    assert!(weather.wind.is_none());
    assert!(weather.dome_image.is_none());
}

#[test]
fn sparse_scenario_roundtrip_omits_absent_optionals() {
    let scenario = parse_from_str(SPARSE).expect("sparse scenario must parse");
    let xml = serialize_to_string(&scenario).expect("serialize");

    // Omitted optionals must not be invented on the way out
    assert!(!xml.contains("cloudState"));
    assert!(!xml.contains("<Sun"));
    assert!(!xml.contains("<Fog"));
    assert!(!xml.contains("<Precipitation"));
    assert!(!xml.contains("<TimeOfDay"));
    assert!(!xml.contains("<RoadCondition"));

    // And the result must parse again and serialize identically (fixed point)
    let reparsed = parse_from_str(&xml).expect("round-trip reparse");
    let xml2 = serialize_to_string(&reparsed).expect("serialize again");
    assert_eq!(xml, xml2);
}
