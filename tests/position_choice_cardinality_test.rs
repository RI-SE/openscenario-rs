//! Cardinality and round-trip fixtures for the two position choice groups.
//!
//! XSD `Position` (`Schema/OpenSCENARIO.xsd:1738-1749`) and `InRoutePosition` (`:1323-1329`)
//! are both bare `xsd:choice`. Each was previously modeled as parallel `Option` fields, which
//! is the `xsd:all` production: a document naming no branch parsed, and one naming two branches
//! parsed, kept both, and re-serialized both. Holding the branch in a `$value` field makes
//! serde enforce the cardinality, so those two documents are now parse errors.
//!
//! The zero-branch and two-branch claims are asserted in separate tests. A single test would
//! stop at the first failure and never measure the second, which is the one that showed the
//! crate emitting XML no schema-valid tool accepts.

use openscenario_rs::types::positions::PositionChoice;
use openscenario_rs::types::positions::{InRoutePosition, InRoutePositionChoice, Position};

/// Parse, assert the branch, and assert the serialization is byte-identical to the source.
fn round_trip(xml: &str) -> Position {
    let parsed: Position = quick_xml::de::from_str(xml).expect("must parse");
    let ser = quick_xml::se::to_string(&parsed).expect("must serialize");
    assert_eq!(ser, xml, "serialized bytes must equal the source document");
    let back: Position = quick_xml::de::from_str(&ser).expect("must reparse");
    assert_eq!(parsed, back);
    parsed
}

#[test]
fn world_position_branch_round_trips() {
    let xml = r#"<Position><WorldPosition x="1" y="2" z="3"/></Position>"#;
    assert!(matches!(
        round_trip(xml).position,
        PositionChoice::WorldPosition(_)
    ));
}

#[test]
fn relative_world_position_branch_round_trips() {
    let xml =
        r#"<Position><RelativeWorldPosition entityRef="Ego" dx="1" dy="2" dz="3"/></Position>"#;
    assert!(matches!(
        round_trip(xml).position,
        PositionChoice::RelativeWorldPosition(_)
    ));
}

#[test]
fn relative_object_position_branch_round_trips() {
    let xml = r#"<Position><RelativeObjectPosition entityRef="Ego" dx="1" dy="2"/></Position>"#;
    assert!(matches!(
        round_trip(xml).position,
        PositionChoice::RelativeObjectPosition(_)
    ));
}

#[test]
fn road_position_branch_round_trips() {
    let xml = r#"<Position><RoadPosition roadId="1" s="12.5" t="-1.5"/></Position>"#;
    assert!(matches!(
        round_trip(xml).position,
        PositionChoice::RoadPosition(_)
    ));
}

#[test]
fn relative_road_position_branch_round_trips() {
    let xml = r#"<Position><RelativeRoadPosition entityRef="Ego" ds="5" dt="0.5"/></Position>"#;
    assert!(matches!(
        round_trip(xml).position,
        PositionChoice::RelativeRoadPosition(_)
    ));
}

#[test]
fn lane_position_branch_round_trips() {
    let xml = r#"<Position><LanePosition roadId="1" laneId="-1" s="20" offset="0.25"/></Position>"#;
    assert!(matches!(
        round_trip(xml).position,
        PositionChoice::LanePosition(_)
    ));
}

#[test]
fn relative_lane_position_branch_round_trips() {
    let xml = r#"<Position><RelativeLanePosition entityRef="Ego" dLane="-1" ds="10"/></Position>"#;
    assert!(matches!(
        round_trip(xml).position,
        PositionChoice::RelativeLanePosition(_)
    ));
}

#[test]
fn route_position_branch_round_trips() {
    let xml = r#"<Position><RoutePosition><RouteRef><CatalogReference catalogName="RouteCatalog" entryName="EgoRoute"/></RouteRef><InRoutePosition><FromRoadCoordinates pathS="5" t="0"/></InRoutePosition></RoutePosition></Position>"#;
    assert!(matches!(
        round_trip(xml).position,
        PositionChoice::RoutePosition(_)
    ));
}

#[test]
fn geo_position_branch_round_trips() {
    let xml = r#"<Position><GeoPosition latitudeDeg="48.1" longitudeDeg="11.6"/></Position>"#;
    assert!(matches!(
        round_trip(xml).position,
        PositionChoice::GeoPosition(_)
    ));
}

#[test]
fn trajectory_position_branch_round_trips() {
    let xml = r#"<Position><TrajectoryPosition s="7.5"><TrajectoryRef><CatalogReference catalogName="TrajectoryCatalog" entryName="Lane change"/></TrajectoryRef></TrajectoryPosition></Position>"#;
    assert!(matches!(
        round_trip(xml).position,
        PositionChoice::TrajectoryPosition(_)
    ));
}

#[test]
fn position_with_no_branch_is_rejected() {
    let err = quick_xml::de::from_str::<Position>("<Position/>")
        .expect_err("a Position naming no branch must be rejected");
    assert!(
        err.to_string().contains("$value"),
        "expected a missing-field error naming $value, got: {err}"
    );
}

#[test]
fn position_with_two_branches_is_rejected() {
    let xml = r#"<Position><WorldPosition x="1" y="2"/><LanePosition roadId="1" laneId="-1" s="3"/></Position>"#;
    let err = quick_xml::de::from_str::<Position>(xml)
        .expect_err("a Position naming two branches must be rejected");
    assert!(
        err.to_string().contains("duplicate field"),
        "expected a duplicate-field error, got: {err}"
    );
}

#[test]
fn in_route_position_from_current_entity_round_trips() {
    let xml = r#"<InRoutePosition><FromCurrentEntity entityRef="Ego"/></InRoutePosition>"#;
    let parsed: InRoutePosition = quick_xml::de::from_str(xml).unwrap();
    assert!(matches!(
        parsed.position,
        InRoutePositionChoice::FromCurrentEntity(_)
    ));
    assert_eq!(quick_xml::se::to_string(&parsed).unwrap(), xml);
}

#[test]
fn in_route_position_from_road_coordinates_round_trips() {
    let xml = r#"<InRoutePosition><FromRoadCoordinates pathS="12.5" t="-1.75"/></InRoutePosition>"#;
    let parsed: InRoutePosition = quick_xml::de::from_str(xml).unwrap();
    assert!(matches!(
        parsed.position,
        InRoutePositionChoice::FromRoadCoordinates(_)
    ));
    assert_eq!(quick_xml::se::to_string(&parsed).unwrap(), xml);
}

#[test]
fn in_route_position_from_lane_coordinates_round_trips() {
    let xml = r#"<InRoutePosition><FromLaneCoordinates laneId="-1" laneOffset="0" pathS="3"/></InRoutePosition>"#;
    let parsed: InRoutePosition = quick_xml::de::from_str(xml).unwrap();
    assert!(matches!(
        parsed.position,
        InRoutePositionChoice::FromLaneCoordinates(_)
    ));
    assert_eq!(quick_xml::se::to_string(&parsed).unwrap(), xml);
}

#[test]
fn in_route_position_with_no_branch_is_rejected() {
    let err = quick_xml::de::from_str::<InRoutePosition>("<InRoutePosition/>")
        .expect_err("an InRoutePosition naming no branch must be rejected");
    assert!(
        err.to_string().contains("$value"),
        "expected a missing-field error naming $value, got: {err}"
    );
}

#[test]
fn in_route_position_with_two_branches_is_rejected() {
    let xml = r#"<InRoutePosition><FromCurrentEntity entityRef="Ego"/><FromRoadCoordinates pathS="1" t="2"/></InRoutePosition>"#;
    let err = quick_xml::de::from_str::<InRoutePosition>(xml)
        .expect_err("an InRoutePosition naming two branches must be rejected");
    assert!(
        err.to_string().contains("duplicate field"),
        "expected a duplicate-field error, got: {err}"
    );
}
