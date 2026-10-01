//! Wire form of the spatial conditions: `ReachPositionCondition` (XSD:1819-1825),
//! `DistanceCondition` (XSD:1072-1085) and `RelativeDistanceCondition` (XSD:1843-1851).
//!
//! Each row builds the condition with a public constructor (and its `with_*` setters), then
//! asserts that the schema-valid XML parses to exactly that value and that the value
//! serializes back to exactly that XML. A wrong attribute name, a constructor that stores
//! the wrong field or enum, or a setter that writes the wrong attribute fails the row.

use openscenario_rs::types::basic::OSString;
use openscenario_rs::types::conditions::{
    DistanceCondition, ReachPositionCondition, RelativeDistanceCondition,
};
use openscenario_rs::types::enums::{
    CoordinateSystem, RelativeDistanceType, RoutingAlgorithm, Rule,
};
use openscenario_rs::types::positions::Position;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

fn entity(name: &str) -> OSString {
    OSString::literal(name.to_string())
}

fn assert_wire<T>(xml: &str, expected: &T)
where
    T: Serialize + DeserializeOwned + PartialEq + Debug,
{
    let parsed: T =
        quick_xml::de::from_str(xml).unwrap_or_else(|e| panic!("failed to parse {xml}: {e}"));
    assert_eq!(&parsed, expected, "parsed value differs for {xml}");
    let emitted = quick_xml::se::to_string(expected).expect("serialize condition");
    assert_eq!(emitted, xml);
}

const ORIGIN: &str = r#"<Position><WorldPosition x="0" y="0"/></Position>"#;

#[test]
fn reach_position_condition_matches_xsd_wire_form() {
    assert_wire(
        r#"<ReachPositionCondition tolerance="2"><Position><WorldPosition x="100" y="200" z="1" h="1.57" p="0" r="0"/></Position></ReachPositionCondition>"#,
        &ReachPositionCondition::at_world_position(100.0, 200.0, 1.0, 1.57, 2.0),
    );
    assert_wire(
        &format!(r#"<ReachPositionCondition tolerance="0.5">{ORIGIN}</ReachPositionCondition>"#),
        &ReachPositionCondition::new(Position::world_origin(), 0.5),
    );
}

#[test]
fn distance_condition_matches_xsd_wire_form() {
    assert_wire(
        &format!(
            r#"<DistanceCondition value="10" freespace="true" rule="equalTo">{ORIGIN}</DistanceCondition>"#
        ),
        &DistanceCondition::new(Position::world_origin(), 10.0, true, Rule::EqualTo),
    );
    assert_wire(
        &format!(
            r#"<DistanceCondition value="30" freespace="false" rule="lessThan" coordinateSystem="road" relativeDistanceType="longitudinal" routingAlgorithm="shortest">{ORIGIN}</DistanceCondition>"#
        ),
        &DistanceCondition::less_than(Position::world_origin(), 30.0, false)
            .with_coordinate_system(CoordinateSystem::Road)
            .with_distance_type(RelativeDistanceType::Longitudinal)
            .with_routing_algorithm(RoutingAlgorithm::Shortest),
    );
    assert_wire(
        &format!(
            r#"<DistanceCondition value="50" freespace="true" rule="greaterThan">{ORIGIN}</DistanceCondition>"#
        ),
        &DistanceCondition::greater_than(Position::world_origin(), 50.0, true),
    );
}

#[test]
fn relative_distance_condition_matches_xsd_wire_form() {
    assert_wire(
        r#"<RelativeDistanceCondition entityRef="Lead" value="12.5" freespace="true" relativeDistanceType="cartesianDistance" rule="greaterThan"/>"#,
        &RelativeDistanceCondition::new(
            entity("Lead"),
            12.5,
            true,
            RelativeDistanceType::Cartesian,
            Rule::GreaterThan,
        ),
    );
    assert_wire(
        r#"<RelativeDistanceCondition entityRef="Lead" value="18" freespace="true" relativeDistanceType="longitudinal" rule="greaterOrEqual" coordinateSystem="lane" routingAlgorithm="assignedRoute"/>"#,
        &RelativeDistanceCondition::longitudinal(entity("Lead"), 18.0, true, Rule::GreaterOrEqual)
            .with_coordinate_system(CoordinateSystem::Lane)
            .with_routing_algorithm(RoutingAlgorithm::AssignedRoute),
    );
    assert_wire(
        r#"<RelativeDistanceCondition entityRef="Adjacent" value="3" freespace="false" relativeDistanceType="lateral" rule="lessThan"/>"#,
        &RelativeDistanceCondition::lateral(entity("Adjacent"), 3.0, false, Rule::LessThan),
    );
    assert_wire(
        r#"<RelativeDistanceCondition entityRef="Lead" value="15" freespace="false" relativeDistanceType="euclidianDistance" rule="notEqualTo"/>"#,
        &RelativeDistanceCondition::euclidian(entity("Lead"), 15.0, false, Rule::NotEqualTo),
    );
}
