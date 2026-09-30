//! Cardinality of the leaf types' repeated lists whose XSD `minOccurs` is 2 or 3.
//!
//! Each field here carries its bound in its type as `MinVec<T, N>`. A document carrying
//! fewer items than the schema demands used to parse and re-serialize unchanged; these
//! tests parse the short document and expect `MinVec`'s error, and round-trip the
//! shortest valid one. The `minOccurs = 1` fields are rejected earlier, by serde's
//! `missing field` (`required_vec_rejects_empty_test.rs`), and `MinVec`'s own
//! construction bound is pinned in `min_vec_test.rs`.

use openscenario_rs::types::basic::Value;

// ─── minOccurs = 2 and 3: a real parse hole, now closed ──────────────────────────────

const POLYLINE_ONE: &str =
    r#"<Polyline><Vertex><Position><WorldPosition x="0" y="0"/></Position></Vertex></Polyline>"#;
const POLYLINE_TWO: &str = r#"<Polyline><Vertex><Position><WorldPosition x="0" y="0"/></Position></Vertex><Vertex><Position><WorldPosition x="1" y="1"/></Position></Vertex></Polyline>"#;

#[test]
fn polyline_with_one_vertex_is_rejected() {
    // XSD:1735 — `Vertex` minOccurs="2".
    let e =
        quick_xml::de::from_str::<openscenario_rs::types::geometry::shapes::Polyline>(POLYLINE_ONE)
            .unwrap_err();
    assert_eq!(
        e.to_string(),
        "Validation error in field 'MinVec': expected at least 2 items, got 1"
    );
}

#[test]
fn polyline_with_two_vertices_round_trips_byte_exactly() {
    let p: openscenario_rs::types::geometry::shapes::Polyline =
        quick_xml::de::from_str(POLYLINE_TWO).unwrap();
    assert_eq!(quick_xml::se::to_string(&p).unwrap(), POLYLINE_TWO);
}

const NURBS_ONE_CP: &str = r#"<Nurbs order="3"><ControlPoint><Position><WorldPosition x="0" y="0"/></Position></ControlPoint><Knot value="0"/><Knot value="1"/></Nurbs>"#;
const NURBS_ONE_KNOT: &str = r#"<Nurbs order="3"><ControlPoint><Position><WorldPosition x="0" y="0"/></Position></ControlPoint><ControlPoint><Position><WorldPosition x="1" y="1"/></Position></ControlPoint><Knot value="0"/></Nurbs>"#;
// The second knot is a `$k` parameter reference: a `Value` inside a `MinVec` item keeps its
// `parameter` spelling across the round trip.
const NURBS_VALID: &str = r#"<Nurbs order="3"><ControlPoint><Position><WorldPosition x="0" y="0"/></Position></ControlPoint><ControlPoint><Position><WorldPosition x="1" y="1"/></Position></ControlPoint><Knot value="0"/><Knot value="$k"/></Nurbs>"#;

#[test]
fn nurbs_with_one_control_point_is_rejected() {
    // XSD:1517 — `ControlPoint` minOccurs="2".
    let e =
        quick_xml::de::from_str::<openscenario_rs::types::geometry::shapes::Nurbs>(NURBS_ONE_CP)
            .unwrap_err();
    assert!(e.to_string().contains("at least 2"), "got: {e}");
}

#[test]
fn nurbs_with_one_knot_is_rejected() {
    // XSD:1518 — `Knot` minOccurs="2".
    let e =
        quick_xml::de::from_str::<openscenario_rs::types::geometry::shapes::Nurbs>(NURBS_ONE_KNOT)
            .unwrap_err();
    assert!(e.to_string().contains("at least 2"), "got: {e}");
}

#[test]
fn nurbs_with_two_of_each_round_trips_byte_exactly() {
    let n: openscenario_rs::types::geometry::shapes::Nurbs =
        quick_xml::de::from_str(NURBS_VALID).unwrap();
    assert_eq!(n.knots[0].value, Value::Literal(0.0));
    assert_eq!(n.knots[1].value, Value::Parameter("k".to_string()));
    assert_eq!(quick_xml::se::to_string(&n).unwrap(), NURBS_VALID);
}

const USED_AREA_ONE: &str =
    r#"<UsedArea><Position><WorldPosition x="0" y="0"/></Position></UsedArea>"#;

#[test]
fn used_area_with_one_position_is_rejected() {
    // XSD:2413 — `Position` minOccurs="2".
    let e = quick_xml::de::from_str::<openscenario_rs::types::road::UsedArea>(USED_AREA_ONE)
        .unwrap_err();
    assert!(e.to_string().contains("at least 2"), "got: {e}");
}

const ROAD_RANGE_ONE: &str = r#"<RoadRange><RoadCursor roadId="R" s="0"/></RoadRange>"#;

#[test]
fn road_range_with_one_cursor_is_rejected() {
    // XSD:1951 — `RoadCursor` minOccurs="2".
    let e = quick_xml::de::from_str::<openscenario_rs::types::actions::traffic::RoadRange>(
        ROAD_RANGE_ONE,
    )
    .unwrap_err();
    assert!(e.to_string().contains("at least 2"), "got: {e}");
}

// ─── MIN = 3: the crate's only one, and never exercised before this issue ────────────

const POLYGON_TWO: &str = r#"<Polygon><Position><WorldPosition x="0" y="0"/></Position><Position><WorldPosition x="1" y="0"/></Position></Polygon>"#;
const POLYGON_THREE: &str = r#"<Polygon><Position><WorldPosition x="0" y="0"/></Position><Position><WorldPosition x="1" y="0"/></Position><Position><WorldPosition x="0" y="1"/></Position></Polygon>"#;

#[test]
fn polygon_with_two_positions_is_rejected() {
    // XSD:1730 — `Position` minOccurs="3", the only such field in the crate. This test
    // exercises the minimum of three, after earlier tests verified two and one.
    let e =
        quick_xml::de::from_str::<openscenario_rs::types::actions::traffic::Polygon>(POLYGON_TWO)
            .unwrap_err();
    assert!(
        e.to_string().contains("at least 3"),
        "expected the minimum of three to be named, got: {e}"
    );
}

#[test]
fn polygon_with_three_positions_round_trips_byte_exactly() {
    let p: openscenario_rs::types::actions::traffic::Polygon =
        quick_xml::de::from_str(POLYGON_THREE).unwrap();
    assert_eq!(quick_xml::se::to_string(&p).unwrap(), POLYGON_THREE);
}
