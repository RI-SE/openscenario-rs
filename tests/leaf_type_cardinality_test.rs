//! Cardinality of the leaf types' required-repeated lists.
//!
//! Each field here carries its XSD `minOccurs` in its type as `MinVec<T, N>`. What that buys
//! is **not uniform**, and these tests are written to assert the claim that is actually true
//! for each field rather than the one the mechanism suggests:
//!
//! * For `minOccurs >= 2`, `MinVec` closes a real **parse** hole. A document carrying one
//!   item where the schema demands two parsed happily before this change and re-serialized
//!   unchanged. Those tests parse XML and expect an error naming `MinVec`.
//! * For `minOccurs = 1`, the empty document was **already** rejected, one layer earlier, by
//!   serde reporting `missing field <Item>` — `MinVec` never sees the vector. What `MinVec`
//!   adds there is the **serialization** side: the inner field is private, so a short value
//!   cannot be constructed and the crate can no longer emit one. Those tests assert
//!   constructibility, not parsing, because asserting a parse fix would credit `MinVec` with
//!   something it did not do.
//!
//! Each claim is a separate test. A single test that checked the short case and then the
//! valid case would stop at the first failure, and the short case is the interesting one.

use openscenario_rs::types::basic::MinVec;

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
    assert!(
        e.to_string().contains("at least 2"),
        "expected the minimum to be named, got: {e}"
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
const NURBS_VALID: &str = r#"<Nurbs order="3"><ControlPoint><Position><WorldPosition x="0" y="0"/></Position></ControlPoint><ControlPoint><Position><WorldPosition x="1" y="1"/></Position></ControlPoint><Knot value="0"/><Knot value="1"/></Nurbs>"#;

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
    assert_eq!(quick_xml::se::to_string(&n).unwrap(), NURBS_VALID);
}

const USED_AREA_ONE: &str =
    r#"<UsedArea><Position><WorldPosition x="0" y="0"/></Position></UsedArea>"#;
const USED_AREA_TWO: &str = r#"<UsedArea><Position><WorldPosition x="0" y="0"/></Position><Position><WorldPosition x="1" y="1"/></Position></UsedArea>"#;

#[test]
fn used_area_with_one_position_is_rejected() {
    // XSD:2413 — `Position` minOccurs="2".
    let e = quick_xml::de::from_str::<openscenario_rs::types::road::UsedArea>(USED_AREA_ONE)
        .unwrap_err();
    assert!(e.to_string().contains("at least 2"), "got: {e}");
}

#[test]
fn used_area_with_two_positions_round_trips_byte_exactly() {
    let u: openscenario_rs::types::road::UsedArea = quick_xml::de::from_str(USED_AREA_TWO).unwrap();
    assert_eq!(quick_xml::se::to_string(&u).unwrap(), USED_AREA_TWO);
}

const ROAD_RANGE_ONE: &str = r#"<RoadRange><RoadCursor roadId="R" s="0"/></RoadRange>"#;
const ROAD_RANGE_TWO: &str =
    r#"<RoadRange><RoadCursor roadId="R" s="0"/><RoadCursor roadId="R" s="10"/></RoadRange>"#;

#[test]
fn road_range_with_one_cursor_is_rejected() {
    // XSD:1951 — `RoadCursor` minOccurs="2".
    let e = quick_xml::de::from_str::<openscenario_rs::types::actions::traffic::RoadRange>(
        ROAD_RANGE_ONE,
    )
    .unwrap_err();
    assert!(e.to_string().contains("at least 2"), "got: {e}");
}

#[test]
fn road_range_with_two_cursors_round_trips_byte_exactly() {
    let r: openscenario_rs::types::actions::traffic::RoadRange =
        quick_xml::de::from_str(ROAD_RANGE_TWO).unwrap();
    assert_eq!(quick_xml::se::to_string(&r).unwrap(), ROAD_RANGE_TWO);
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

// ─── minOccurs = 1: the serialization side, which is what actually changed ───────────
//
// The parse side was already closed — serde reports `missing field` before `MinVec` is
// reached — so these assert what `MinVec` genuinely adds: the short value cannot be built,
// therefore the crate cannot emit one.

#[test]
fn a_min_one_list_reports_the_empty_case_at_construction() {
    assert!(MinVec::<u8, 1>::new(vec![]).is_err());
    assert!(MinVec::<u8, 1>::new(vec![1]).is_ok());
}

#[test]
fn an_empty_min_one_document_is_rejected_by_serde_before_minvec_sees_it() {
    // This is the honest statement of what happens, and it is why the `minOccurs=1`
    // conversions are a serialization fix rather than a parse fix.
    let e = quick_xml::de::from_str::<openscenario_rs::types::geometry::shapes::ClothoidSpline>(
        "<ClothoidSpline/>",
    )
    .unwrap_err();
    let s = e.to_string();
    assert!(
        s.contains("missing field"),
        "expected serde's missing-field error, got: {s}"
    );
    assert!(
        !s.contains("at least"),
        "MinVec must not be the layer that reports this: {s}"
    );
}

#[test]
fn a_min_two_list_reports_the_short_case_at_construction() {
    assert!(MinVec::<u8, 2>::new(vec![1]).is_err());
    assert!(MinVec::<u8, 2>::new(vec![1, 2]).is_ok());
}

#[test]
fn a_min_three_list_reports_the_short_case_at_construction() {
    assert!(MinVec::<u8, 3>::new(vec![1, 2]).is_err());
    assert!(MinVec::<u8, 3>::new(vec![1, 2, 3]).is_ok());
}
