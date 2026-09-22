//! `Route`/`CatalogRoute`, `Polyline`/`CatalogPolyline` and `Nurbs`/`CatalogNurbs` each
//! model the same XSD production twice: once for the scenario-body type and once for
//! its catalog-file counterpart. Both copies were converted to `MinVec` independently
//! (OSS-32), and both are live production paths — the catalog copies are read by
//! `catalog/loader.rs` and held by `CatalogFile`; the scenario copies are read wherever
//! a route or trajectory is used inline. Nothing keeps the two copies of each pair in
//! step with each other, so this file pins that agreement: the leaf element every pair
//! duplicates (`Waypoint`, `Vertex`, `ControlPoint`, `Knot`) must parse to the same
//! content and re-serialize to the same bytes through both types, and the `minOccurs="2"`
//! bound every container in every pair carries must reject a one-item list on both sides.

use openscenario_rs::types::catalogs::routes::{CatalogRoute, RouteWaypoint};
use openscenario_rs::types::catalogs::trajectories::{
    CatalogNurbs, CatalogPolyline, CatalogVertex, NurbsControlPoint, NurbsKnot,
};
use openscenario_rs::types::geometry::shapes::{ControlPoint, Knot, Nurbs, Polyline, Vertex};
use openscenario_rs::types::routing::{Route, Waypoint};

// ---------------------------------------------------------------------
// Route.waypoints (:1958) vs CatalogRoute.waypoints
// ---------------------------------------------------------------------

const WAYPOINT_XML: &str = r#"<Waypoint routeStrategy="shortest">
    <Position><WorldPosition x="1" y="2" z="3"/></Position>
</Waypoint>"#;

#[test]
fn waypoint_and_route_waypoint_parse_and_serialize_identically() {
    let via_sibling: Waypoint = quick_xml::de::from_str(WAYPOINT_XML).unwrap();
    let via_catalog: RouteWaypoint = quick_xml::de::from_str(WAYPOINT_XML).unwrap();

    assert_eq!(via_sibling.position, via_catalog.position);
    assert_eq!(via_sibling.route_strategy, via_catalog.route_strategy);

    let bytes_sibling = quick_xml::se::to_string(&via_sibling).unwrap();
    let bytes_catalog = quick_xml::se::to_string(&via_catalog).unwrap();
    assert_eq!(bytes_sibling, bytes_catalog);
}

#[test]
fn route_and_catalog_route_both_reject_one_waypoint() {
    let short = r#"<Route name="R" closed="false">
    <Waypoint routeStrategy="shortest">
        <Position><WorldPosition x="0" y="0" z="0"/></Position>
    </Waypoint>
</Route>"#;

    assert!(
        quick_xml::de::from_str::<Route>(short).is_err(),
        "Route must reject a single waypoint (XSD :1958 minOccurs=\"2\")"
    );
    assert!(
        quick_xml::de::from_str::<CatalogRoute>(short).is_err(),
        "CatalogRoute must reject a single waypoint (XSD :1958 minOccurs=\"2\")"
    );
}

// ---------------------------------------------------------------------
// Polyline.vertices (:1735) vs CatalogPolyline.vertices
// ---------------------------------------------------------------------

const VERTEX_XML: &str = r#"<Vertex time="1.5">
    <Position><WorldPosition x="1" y="2" z="3"/></Position>
</Vertex>"#;

#[test]
fn vertex_and_catalog_vertex_parse_and_serialize_identically() {
    let via_sibling: Vertex = quick_xml::de::from_str(VERTEX_XML).unwrap();
    let via_catalog: CatalogVertex = quick_xml::de::from_str(VERTEX_XML).unwrap();

    assert_eq!(via_sibling.position, via_catalog.position);
    assert_eq!(via_sibling.time, via_catalog.time);

    let bytes_sibling = quick_xml::se::to_string(&via_sibling).unwrap();
    let bytes_catalog = quick_xml::se::to_string(&via_catalog).unwrap();
    assert_eq!(bytes_sibling, bytes_catalog);
}

#[test]
fn polyline_and_catalog_polyline_both_reject_one_vertex() {
    let short = r#"<Polyline>
    <Vertex><Position><WorldPosition x="0" y="0" z="0"/></Position></Vertex>
</Polyline>"#;

    assert!(
        quick_xml::de::from_str::<Polyline>(short).is_err(),
        "Polyline must reject a single vertex (XSD :1735 minOccurs=\"2\")"
    );
    assert!(
        quick_xml::de::from_str::<CatalogPolyline>(short).is_err(),
        "CatalogPolyline must reject a single vertex (XSD :1735 minOccurs=\"2\")"
    );
}

// ---------------------------------------------------------------------
// Nurbs.control_points (:1517) / Nurbs.knots (:1518) vs CatalogNurbs
// ---------------------------------------------------------------------

const CONTROL_POINT_XML: &str = r#"<ControlPoint time="2.5" weight="1.0">
    <Position><WorldPosition x="1" y="2" z="3"/></Position>
</ControlPoint>"#;

const KNOT_XML: &str = r#"<Knot value="0.5"/>"#;

#[test]
fn control_point_and_nurbs_control_point_parse_and_serialize_identically() {
    let via_sibling: ControlPoint = quick_xml::de::from_str(CONTROL_POINT_XML).unwrap();
    let via_catalog: NurbsControlPoint = quick_xml::de::from_str(CONTROL_POINT_XML).unwrap();

    assert_eq!(via_sibling.position, via_catalog.position);
    assert_eq!(via_sibling.time, via_catalog.time);
    assert_eq!(via_sibling.weight, via_catalog.weight);

    let bytes_sibling = quick_xml::se::to_string(&via_sibling).unwrap();
    let bytes_catalog = quick_xml::se::to_string(&via_catalog).unwrap();
    assert_eq!(bytes_sibling, bytes_catalog);
}

#[test]
fn knot_and_nurbs_knot_parse_and_serialize_identically() {
    let via_sibling: Knot = quick_xml::de::from_str(KNOT_XML).unwrap();
    let via_catalog: NurbsKnot = quick_xml::de::from_str(KNOT_XML).unwrap();

    assert_eq!(via_sibling.value, via_catalog.value);

    let bytes_sibling = quick_xml::se::to_string(&via_sibling).unwrap();
    let bytes_catalog = quick_xml::se::to_string(&via_catalog).unwrap();
    assert_eq!(bytes_sibling, bytes_catalog);
}

#[test]
fn nurbs_and_catalog_nurbs_both_reject_one_control_point_and_one_knot() {
    let short = r#"<Nurbs order="3">
    <ControlPoint><Position><WorldPosition x="0" y="0" z="0"/></Position></ControlPoint>
    <Knot value="0.0"/>
</Nurbs>"#;

    assert!(
        quick_xml::de::from_str::<Nurbs>(short).is_err(),
        "Nurbs must reject a single control point/knot (XSD :1517-1518 minOccurs=\"2\")"
    );
    assert!(
        quick_xml::de::from_str::<CatalogNurbs>(short).is_err(),
        "CatalogNurbs must reject a single control point/knot (XSD :1517-1518 minOccurs=\"2\")"
    );
}
