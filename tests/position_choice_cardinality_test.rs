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

use openscenario_rs::types::basic::Double;
use openscenario_rs::types::positions::{
    GeographicPosition, InRoutePosition, LanePosition, Position, RelativeLanePosition,
    RelativeObjectPosition, RelativeRoadPosition, RelativeWorldPosition, RoadPosition,
    RoutePosition, TrajectoryPosition, TrajectoryRef, WorldPosition,
};
use openscenario_rs::types::routing::RouteRef;

/// Parse, and assert the serialization is byte-identical to the source.
fn round_trip(xml: &str) -> Position {
    let parsed: Position = quick_xml::de::from_str(xml).expect("must parse");
    let ser = quick_xml::se::to_string(&parsed).expect("must serialize");
    assert_eq!(ser, xml, "serialized bytes must equal the source document");
    let back: Position = quick_xml::de::from_str(&ser).expect("must reparse");
    assert_eq!(parsed, back);
    parsed
}

/// Reads a `Position` through one of its per-branch accessors.
type Accessor = fn(&Position) -> bool;

/// Every per-branch accessor on `Position`, keyed by name.
const ACCESSORS: [(&str, Accessor); 10] = [
    ("world_position", |p| p.world_position().is_some()),
    ("relative_world_position", |p| {
        p.relative_world_position().is_some()
    }),
    ("relative_object_position", |p| {
        p.relative_object_position().is_some()
    }),
    ("road_position", |p| p.road_position().is_some()),
    ("relative_road_position", |p| {
        p.relative_road_position().is_some()
    }),
    ("lane_position", |p| p.lane_position().is_some()),
    ("relative_lane_position", |p| {
        p.relative_lane_position().is_some()
    }),
    ("route_position", |p| p.route_position().is_some()),
    ("geographic_position", |p| p.geographic_position().is_some()),
    ("trajectory_position", |p| p.trajectory_position().is_some()),
];

/// One branch of XSD `Position`: a schema-valid document naming it, the accessor that must
/// read it, and the same value built through the branch's `Position` constructor.
struct Branch {
    xml: &'static str,
    accessor: &'static str,
    build: fn() -> Position,
}

/// Every branch, in schema order (`Schema/OpenSCENARIO.xsd:1739-1748`).
const BRANCHES: [Branch; 10] = [
    Branch {
        xml: r#"<Position><WorldPosition x="1" y="2" z="3"/></Position>"#,
        accessor: "world_position",
        build: || Position::world(WorldPosition::with_z(1.0, 2.0, 3.0)),
    },
    Branch {
        xml: r#"<Position><RelativeWorldPosition entityRef="Ego" dx="1" dy="2" dz="3"/></Position>"#,
        accessor: "relative_world_position",
        build: || {
            Position::relative_world(RelativeWorldPosition {
                dz: Some(Double::literal(3.0)),
                ..RelativeWorldPosition::new("Ego", 1.0, 2.0)
            })
        },
    },
    Branch {
        xml: r#"<Position><RelativeObjectPosition entityRef="Ego" dx="1" dy="2"/></Position>"#,
        accessor: "relative_object_position",
        build: || Position::relative_object(RelativeObjectPosition::new("Ego", 1.0, 2.0)),
    },
    Branch {
        xml: r#"<Position><RoadPosition roadId="1" s="12.5" t="-1.5"/></Position>"#,
        accessor: "road_position",
        build: || Position::road(RoadPosition::new("1".to_string(), 12.5, -1.5)),
    },
    Branch {
        xml: r#"<Position><RelativeRoadPosition entityRef="Ego" ds="5" dt="0.5"/></Position>"#,
        accessor: "relative_road_position",
        build: || Position::relative_road(RelativeRoadPosition::new("Ego".to_string(), 5.0, 0.5)),
    },
    Branch {
        xml: r#"<Position><LanePosition roadId="1" laneId="-1" s="20" offset="0.25"/></Position>"#,
        accessor: "lane_position",
        build: || {
            Position::lane(LanePosition::new(
                "1".to_string(),
                "-1".to_string(),
                20.0,
                0.25,
            ))
        },
    },
    Branch {
        xml: r#"<Position><RelativeLanePosition entityRef="Ego" dLane="-1" ds="10"/></Position>"#,
        accessor: "relative_lane_position",
        build: || {
            Position::relative_lane(RelativeLanePosition {
                offset: None,
                ..RelativeLanePosition::new("Ego".to_string(), -1, 10.0, 0.0)
            })
        },
    },
    Branch {
        xml: r#"<Position><RoutePosition><RouteRef><CatalogReference catalogName="RouteCatalog" entryName="EgoRoute"/></RouteRef><InRoutePosition><FromRoadCoordinates pathS="5" t="0"/></InRoutePosition></RoutePosition></Position>"#,
        accessor: "route_position",
        build: || {
            Position::route(RoutePosition::new(
                RouteRef::catalog("RouteCatalog", "EgoRoute"),
                InRoutePosition::from_road_coordinates(Double::literal(5.0), Double::literal(0.0)),
            ))
        },
    },
    Branch {
        xml: r#"<Position><GeoPosition latitudeDeg="48.1" longitudeDeg="11.6"/></Position>"#,
        accessor: "geographic_position",
        build: || {
            Position::geographic(GeographicPosition {
                latitude: None,
                longitude: None,
                height: None,
                latitude_deg: Some(Double::literal(48.1)),
                longitude_deg: Some(Double::literal(11.6)),
                altitude: None,
                vertical_road_selection: None,
                orientation: None,
            })
        },
    },
    Branch {
        xml: r#"<Position><TrajectoryPosition s="7.5"><TrajectoryRef><CatalogReference catalogName="TrajectoryCatalog" entryName="Lane change"/></TrajectoryRef></TrajectoryPosition></Position>"#,
        accessor: "trajectory_position",
        build: || {
            Position::trajectory(TrajectoryPosition::new(
                7.5,
                TrajectoryRef::from_catalog(
                    "TrajectoryCatalog".to_string(),
                    "Lane change".to_string(),
                ),
            ))
        },
    },
];

/// Each branch survives a byte-identical round trip, which also pins the branch: the element
/// name is the variant name, so a document parsed into the wrong variant re-serializes under
/// a different element name. The branch's `Position` constructor must build exactly the
/// parsed value, and exactly one accessor, the branch's own, must read it.
#[test]
fn every_position_branch_round_trips_and_matches_its_constructor_and_accessor() {
    for branch in BRANCHES {
        let xml = branch.xml;
        let parsed = round_trip(xml);
        assert_eq!(
            parsed,
            (branch.build)(),
            "the {} constructor disagrees with {xml}",
            branch.accessor
        );
        for (accessor, reads) in ACCESSORS {
            assert_eq!(
                reads(&parsed),
                accessor == branch.accessor,
                "accessor {accessor} on the {} branch: {xml}",
                branch.accessor
            );
        }
    }
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
    let current = parsed
        .from_current_entity_ref()
        .expect("FromCurrentEntity branch");
    assert_eq!(current.entity_ref.as_literal().unwrap(), "Ego");
    assert_eq!(quick_xml::se::to_string(&parsed).unwrap(), xml);
}

#[test]
fn in_route_position_from_road_coordinates_round_trips() {
    let xml = r#"<InRoutePosition><FromRoadCoordinates pathS="12.5" t="-1.75"/></InRoutePosition>"#;
    let parsed: InRoutePosition = quick_xml::de::from_str(xml).unwrap();
    let road = parsed
        .from_road_coordinates_ref()
        .expect("FromRoadCoordinates branch");
    assert_eq!(road.path_s.as_literal().unwrap(), &12.5);
    assert_eq!(road.t.as_literal().unwrap(), &-1.75);
    assert_eq!(quick_xml::se::to_string(&parsed).unwrap(), xml);
}

#[test]
fn in_route_position_from_lane_coordinates_round_trips() {
    let xml = r#"<InRoutePosition><FromLaneCoordinates laneId="-1" laneOffset="0" pathS="3"/></InRoutePosition>"#;
    let parsed: InRoutePosition = quick_xml::de::from_str(xml).unwrap();
    let lane = parsed
        .from_lane_coordinates_ref()
        .expect("FromLaneCoordinates branch");
    assert_eq!(lane.lane_id.as_literal().unwrap(), "-1");
    assert_eq!(
        lane.lane_offset.as_ref().unwrap().as_literal().unwrap(),
        &0.0
    );
    assert_eq!(lane.path_s.as_literal().unwrap(), &3.0);
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
