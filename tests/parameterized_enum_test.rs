//! Parameterized enumeration attributes.
//!
//! All 37 enumeration `simpleType`s in `Schema/OpenSCENARIO.xsd` are `xsd:union`s whose
//! second member is `<xsd:restriction base="parameter"/>`, so every enum-typed attribute
//! in the schema may carry a parameter reference instead of a literal. The crate models
//! those attributes as `Value<E>`, which accepts either.
//!
//! ## The sigil, and why this file uses `$cat` and not `${cat}`
//!
//! The schema defines two distinct productions (`Schema/OpenSCENARIO.xsd:4-13`):
//!
//! ```text
//! parameter   [$][A-Za-z_][A-Za-z0-9_]*
//! expression  [$][{][ A-Za-z0-9_\+\-\*/%$\(\)\.,]*[\}]
//! ```
//!
//! The scalar unions (`Double`, `Int`, `Boolean`, …) list `expression parameter …`, so both
//! spellings are valid there. **The 37 enum unions list only `parameter`** — none admits
//! `expression`. So `vehicleCategory="${cat}"` is *not* schema-valid; `vehicleCategory="$cat"`
//! is. `xmllint --schema` confirms both halves of that.
//!
//! The crate still *parses* `${cat}` on an enum attribute (its deserializer is deliberately
//! permissive about the two spellings), but it *emits* `$cat`, which is the only spelling the
//! schema accepts on an enum attribute and is valid on every scalar union as well.

use openscenario_rs::types::basic::Value;
use openscenario_rs::types::conditions::entity::EntityCondition;
use openscenario_rs::types::enums::{CoordinateSystem, Role, Rule, VehicleCategory};
use openscenario_rs::{parse_catalog_from_str, parse_from_str, serialize_to_string};
use std::collections::HashMap;

const SCENARIO: &str = include_str!("data/parameterized_enums.xosc");
const CATALOG: &str = include_str!("data/parameterized_enums_catalog.xosc");

fn ego_vehicle(doc: &openscenario_rs::types::OpenScenario) -> &openscenario_rs::types::Vehicle {
    doc.entities
        .as_ref()
        .expect("Entities")
        .scenario_objects
        .iter()
        .find(|o| o.name.as_literal().map(|s| s.as_str()) == Some("Ego"))
        .expect("Ego")
        .vehicle
        .as_ref()
        .expect("Ego is a Vehicle")
}

#[test]
fn required_bare_enum_attribute_accepts_a_parameter() {
    let doc = parse_from_str(SCENARIO).expect("fixture parses");
    let v = ego_vehicle(&doc);
    assert_eq!(v.vehicle_category, Value::Parameter("cat".to_string()));
}

#[test]
fn optional_enum_attribute_accepts_a_parameter() {
    let doc = parse_from_str(SCENARIO).expect("fixture parses");
    let v = ego_vehicle(&doc);
    assert_eq!(v.role, Some(Value::Parameter("vrole".to_string())));
}

#[test]
fn optional_enum_attribute_absent_is_still_none() {
    // Absent and present-but-parameterized must stay distinguishable: `Option<Value<E>>`,
    // not a collapsed `Value<E>` with an invented default.
    let doc = parse_from_str(SCENARIO).expect("fixture parses");
    let target = doc
        .entities
        .as_ref()
        .unwrap()
        .scenario_objects
        .iter()
        .find(|o| o.name.as_literal().map(|s| s.as_str()) == Some("Target"))
        .expect("Target")
        .vehicle
        .as_ref()
        .unwrap();
    assert_eq!(
        target.vehicle_category,
        Value::Literal(VehicleCategory::Car)
    );
    assert_eq!(target.role, None);
}

#[test]
fn condition_rule_and_coordinate_system_accept_parameters() {
    let doc = parse_from_str(SCENARIO).expect("fixture parses");
    let story = &doc.storyboard.as_ref().expect("Storyboard").stories[0];
    let cond = &story.acts[0].maneuver_groups[0].maneuvers[0].events[0]
        .start_trigger
        .as_ref()
        .expect("StartTrigger")
        .condition_groups[0]
        .conditions[0];
    let rel = match &cond
        .by_entity_condition
        .as_ref()
        .expect("ByEntityCondition")
        .entity_condition
    {
        EntityCondition::RelativeDistance(c) => c,
        other => panic!("expected a RelativeDistanceCondition, got {other:?}"),
    };

    assert_eq!(rel.rule, Value::Parameter("cmpRule".to_string()));
    assert_eq!(
        rel.coordinate_system,
        Some(Value::Parameter("cs".to_string()))
    );
}

#[test]
fn catalog_file_enum_attributes_accept_parameters() {
    let cat = parse_catalog_from_str(CATALOG).expect("catalog fixture parses");
    let v = &cat.catalog.vehicles[0];
    assert_eq!(v.vehicle_category, Value::Parameter("cat".to_string()));
    assert_eq!(v.role, Some(Value::Parameter("vrole".to_string())));
}

#[test]
fn parameters_resolve_through_the_existing_value_machinery() {
    let doc = parse_from_str(SCENARIO).expect("fixture parses");
    let v = ego_vehicle(&doc);

    let mut params = HashMap::new();
    params.insert("cat".to_string(), "truck".to_string());
    params.insert("vrole".to_string(), "police".to_string());
    params.insert("cmpRule".to_string(), "greaterThan".to_string());
    params.insert("cs".to_string(), "entity".to_string());

    assert_eq!(
        v.vehicle_category.resolve(&params).unwrap(),
        VehicleCategory::Truck
    );
    assert_eq!(
        v.role.as_ref().unwrap().resolve(&params).unwrap(),
        Role::Police
    );

    let story = &doc.storyboard.as_ref().unwrap().stories[0];
    let cond = &story.acts[0].maneuver_groups[0].maneuvers[0].events[0]
        .start_trigger
        .as_ref()
        .unwrap()
        .condition_groups[0]
        .conditions[0];
    let rel = match &cond.by_entity_condition.as_ref().unwrap().entity_condition {
        EntityCondition::RelativeDistance(c) => c,
        other => panic!("expected a RelativeDistanceCondition, got {other:?}"),
    };
    assert_eq!(rel.rule.resolve(&params).unwrap(), Rule::GreaterThan);
    assert_eq!(
        rel.coordinate_system
            .as_ref()
            .unwrap()
            .resolve(&params)
            .unwrap(),
        CoordinateSystem::Entity
    );
}

#[test]
fn an_unknown_parameter_errors_on_resolve() {
    let doc = parse_from_str(SCENARIO).expect("fixture parses");
    let v = ego_vehicle(&doc);
    let empty = HashMap::new();
    let err = v
        .vehicle_category
        .resolve(&empty)
        .expect_err("unresolvable parameter must error, not fall back to a default");
    assert!(
        err.to_string().contains("cat"),
        "error should name the parameter: {err}"
    );
}

#[test]
fn a_parameter_that_resolves_to_a_non_variant_errors() {
    let doc = parse_from_str(SCENARIO).expect("fixture parses");
    let v = ego_vehicle(&doc);
    let mut params = HashMap::new();
    params.insert("cat".to_string(), "spaceship".to_string());
    v.vehicle_category
        .resolve(&params)
        .expect_err("`spaceship` is not a VehicleCategory, even via a parameter");
}

#[test]
fn a_literal_non_variant_still_fails_at_parse() {
    // The escape-hatch guard. `Value<T>` falls through to `s.parse::<T>()` for anything
    // without a `$` sigil, so an unknown literal must remain a hard parse error --
    // `Value<E>` is not a stringly-typed back door.
    let bad = SCENARIO.replace(
        r#"vehicleCategory="$cat""#,
        r#"vehicleCategory="spaceship""#,
    );
    assert!(
        bad.contains(r#"vehicleCategory="spaceship""#),
        "replacement did not apply"
    );
    let err = parse_from_str(&bad).expect_err("`spaceship` must not parse as a VehicleCategory");
    let msg = err.to_string();
    assert!(
        msg.contains("spaceship"),
        "error should name the offending value: {msg}"
    );
}

#[test]
fn literal_enum_attributes_are_unaffected() {
    // Every corpus file uses literal enum values; after the migration they route through
    // `Value::Literal` and `Display` rather than the derived `Serialize`. This pins that
    // the literal path still produces the schema wire name.
    let doc = parse_from_str(SCENARIO).expect("fixture parses");
    let xml = serialize_to_string(&doc).expect("serializes");
    assert!(
        xml.contains(r#"vehicleCategory="car""#),
        "literal enum lost its wire name:\n{xml}"
    );
}

#[test]
fn parameters_serialize_in_the_schema_s_parameter_form() {
    // See the module docs: the enum unions admit `parameter` (`$cat`) only, never
    // `expression` (`${cat}`). Emitting the braced form here would produce
    // schema-invalid XML with a green round trip -- exactly the silent failure this
    // fixture exists to catch.
    let doc = parse_from_str(SCENARIO).expect("fixture parses");
    let xml = serialize_to_string(&doc).expect("serializes");
    assert!(
        xml.contains(r#"vehicleCategory="$cat""#),
        "expected the unbraced parameter form:\n{xml}"
    );
    assert!(
        !xml.contains("${cat}"),
        "the braced form is schema-invalid on an enum attribute:\n{xml}"
    );
}

#[test]
fn the_braced_spelling_still_parses() {
    // The crate is permissive on input even though it is strict on output.
    let braced = SCENARIO.replace(r#"vehicleCategory="$cat""#, r#"vehicleCategory="${cat}""#);
    let doc = parse_from_str(&braced).expect("braced spelling parses");
    assert_eq!(
        ego_vehicle(&doc).vehicle_category,
        Value::Parameter("cat".to_string())
    );
}
