//! `MinVec<T, MIN>` is the mechanism for the schema particles whose `minOccurs` is above
//! one: `Vertex` and `Waypoint` require two children, `Polygon`'s `Position` requires
//! three. No serde attribute expresses such a bound, so the check lives in the type.
//!
//! Nothing in `src/types/` uses `MinVec` yet. These tests therefore stand in for the
//! fields that will adopt it, with local structs that mirror the real ones element for
//! element and carry the crate's own item types. Each assertion is a test of its own, so
//! a failure in one does not hide the next.

use openscenario_rs::types::basic::{Double, MinVec, OSString, Value};
use openscenario_rs::types::geometry::shapes::{Knot, Vertex};
use openscenario_rs::types::scenario::story::{StoryAction, StoryActionChoice};
use serde::{Deserialize, Serialize};

fn parse<T: for<'de> Deserialize<'de>>(xml: &str) -> Result<T, quick_xml::de::DeError> {
    quick_xml::de::from_str(xml)
}

fn to_xml<T: Serialize>(value: &T) -> String {
    quick_xml::se::to_string(value).expect("serialization failed")
}

/// Mirrors `Polyline`, whose XSD `Vertex` particle carries `minOccurs="2"`, with the
/// bound stated in the field type rather than left to the caller.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "Polyline")]
struct BoundedPolyline {
    #[serde(rename = "Vertex")]
    vertices: MinVec<Vertex, 2>,
}

// --- The four core assertions ------------------------------------------------------

#[test]
fn two_vertices_round_trip_byte_for_byte() {
    let xml = r#"<Polyline><Vertex><Position><WorldPosition x="1" y="2"/></Position></Vertex><Vertex><Position><WorldPosition x="3" y="4"/></Position></Vertex></Polyline>"#;
    let parsed: BoundedPolyline = parse(xml).expect("two vertices must parse");
    assert_eq!(parsed.vertices.len(), 2);
    assert_eq!(to_xml(&parsed), xml);
}

#[test]
fn one_vertex_is_rejected_at_parse_time() {
    let err = parse::<BoundedPolyline>(
        r#"<Polyline><Vertex><Position><WorldPosition x="1" y="2"/></Position></Vertex></Polyline>"#,
    )
    .unwrap_err();
    assert_eq!(
        err.to_string(),
        "Validation error in field 'MinVec': expected at least 2 items, got 1"
    );
}

#[test]
fn no_vertex_is_rejected_at_parse_time() {
    let err = parse::<BoundedPolyline>("<Polyline/>").unwrap_err();
    assert_eq!(err.to_string(), "missing field `Vertex`");
}

#[test]
fn a_short_value_cannot_be_constructed() {
    let err = MinVec::<Vertex, 2>::new(vec![]).unwrap_err();
    assert_eq!(
        err.to_string(),
        "Validation error in field 'MinVec': expected at least 2 items, got 0"
    );
    assert!(MinVec::<Vertex, 2>::try_from(Vec::new()).is_err());
}

/// The same element modeled the way the crate models it today. `Vec<Vertex>` states no
/// lower bound, so the one-vertex document below is accepted and written back out in the
/// same schema-invalid shape. This is the state `BoundedPolyline` above is measured
/// against; the two types differ only in the field type.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "Polyline")]
struct UnboundedPolyline {
    #[serde(rename = "Vertex")]
    vertices: Vec<Vertex>,
}

#[test]
fn a_plain_vec_field_accepts_the_document_min_vec_rejects() {
    let xml = r#"<Polyline><Vertex><Position><WorldPosition x="1" y="2"/></Position></Vertex></Polyline>"#;
    let parsed: UnboundedPolyline = parse(xml).expect("a plain Vec accepts one vertex");
    assert_eq!(parsed.vertices.len(), 1);
    assert_eq!(to_xml(&parsed), xml);
    assert!(parse::<BoundedPolyline>(xml).is_err());
}

// --- Composition 1: `Value<T>` payloads --------------------------------------------

/// Mirrors the `Knot` half of `Nurbs` (`minOccurs="2"`), whose item holds a `Double`,
/// that is a `Value<f64>`. The parameter reference below proves the wrapper does not
/// interfere with the parameter production surviving the round trip.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "Knots")]
struct BoundedKnots {
    #[serde(rename = "Knot")]
    knots: MinVec<Knot, 2>,
}

#[test]
fn min_vec_of_items_holding_value_round_trips_with_a_parameter() {
    let xml = r#"<Knots><Knot value="0"/><Knot value="$k"/></Knots>"#;
    let parsed: BoundedKnots = parse(xml).expect("two knots must parse");
    assert_eq!(parsed.knots[0].value, Value::Literal(0.0));
    assert_eq!(parsed.knots[1].value, Value::Parameter("k".to_string()));
    assert_eq!(to_xml(&parsed), xml);
}

#[test]
fn min_vec_of_one_knot_is_rejected() {
    let err = parse::<BoundedKnots>(r#"<Knots><Knot value="0"/></Knots>"#).unwrap_err();
    assert_eq!(
        err.to_string(),
        "Validation error in field 'MinVec': expected at least 2 items, got 1"
    );
}

/// `Value<T>` directly as the repeated item, rather than inside one. The element text is
/// the whole payload here, which is the shape a repeated scalar element would take.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "Weights")]
struct BoundedWeights {
    #[serde(rename = "Weight")]
    weights: MinVec<Double, 2>,
}

#[test]
fn min_vec_of_bare_value_items_round_trips() {
    let xml = r#"<Weights><Weight>1</Weight><Weight>$w</Weight></Weights>"#;
    let parsed: BoundedWeights = parse(xml).expect("two weights must parse");
    assert_eq!(parsed.weights[0], Value::Literal(1.0));
    assert_eq!(parsed.weights[1], Value::Parameter("w".to_string()));
    assert_eq!(to_xml(&parsed), xml);
}

// --- Composition 2: `$value` choice enums as items ----------------------------------

/// Mirrors `Event.actions`, which holds `StoryAction`. `StoryAction` carries its choice
/// behind `$value`, and `$value` does not nest. The question this answers is whether a
/// `MinVec` of such a type needs an element wrapper of its own: it does not, because the
/// item is a struct and the enum stays one level below it.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "Event")]
struct BoundedEvent {
    #[serde(rename = "@name")]
    name: OSString,
    #[serde(rename = "Action")]
    actions: MinVec<StoryAction, 2>,
}

#[test]
fn min_vec_of_value_choice_items_round_trips_byte_for_byte() {
    let xml = concat!(
        r#"<Event name="e">"#,
        r#"<Action name="a1"><UserDefinedAction><CustomCommandAction type="t"/></UserDefinedAction></Action>"#,
        r#"<Action name="a2"><PrivateAction><TeleportAction><Position><WorldPosition x="1" y="2"/></Position></TeleportAction></PrivateAction></Action>"#,
        r#"</Event>"#
    );
    let parsed: BoundedEvent = parse(xml).expect("two actions must parse");
    assert_eq!(parsed.actions.len(), 2);
    assert_eq!(parsed.actions[0].name, Value::Literal("a1".to_string()));
    assert_eq!(parsed.actions[1].name, Value::Literal("a2".to_string()));
    assert_eq!(to_xml(&parsed), xml);
}

#[test]
fn min_vec_of_value_choice_items_rejects_a_short_list() {
    let xml = concat!(
        r#"<Event name="e">"#,
        r#"<Action name="a1"><UserDefinedAction><CustomCommandAction type="t"/></UserDefinedAction></Action>"#,
        r#"</Event>"#
    );
    let err = parse::<BoundedEvent>(xml).unwrap_err();
    assert_eq!(
        err.to_string(),
        "Validation error in field 'MinVec': expected at least 2 items, got 1"
    );
}

/// The stricter reading of the same question: a `MinVec` whose items are the choice enum
/// itself, sitting directly under `$value`. `$value` does not nest, so this is where a
/// wrapper would be needed if one were needed anywhere. It is not: each variant already
/// carries an element wrapper struct, so every level still writes exactly one element
/// name, and the bound is enforced in the same place.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "Actions")]
struct BoundedActionChoices {
    #[serde(rename = "$value")]
    actions: MinVec<StoryActionChoice, 2>,
}

#[test]
fn min_vec_of_bare_choice_enums_under_value_round_trips_and_holds_the_bound() {
    let xml = concat!(
        r#"<Actions>"#,
        r#"<UserDefinedAction><CustomCommandAction type="t"/></UserDefinedAction>"#,
        r#"<UserDefinedAction><CustomCommandAction type="u"/></UserDefinedAction>"#,
        r#"</Actions>"#
    );
    let parsed: BoundedActionChoices = parse(xml).expect("two branches must parse");
    assert_eq!(parsed.actions.len(), 2);
    assert_eq!(to_xml(&parsed), xml);

    let short = r#"<Actions><UserDefinedAction><CustomCommandAction type="t"/></UserDefinedAction></Actions>"#;
    let err = parse::<BoundedActionChoices>(short).unwrap_err();
    assert_eq!(
        err.to_string(),
        "Validation error in field 'MinVec': expected at least 2 items, got 1"
    );
}

// --- Composition 3: the builder's generic plumbing ----------------------------------

/// The existing builders accumulate into a plain `Vec` and return `BuilderResult` from
/// `build()`. `BuilderError` carries `From<openscenario_rs::error::Error>`, which is what
/// `MinVec::new` returns, so the bound check is one `?` in the same place the other
/// build-time checks already sit. This stands in for that plumbing.
#[cfg(feature = "builder")]
mod builder_plumbing {
    use super::*;
    use openscenario_rs::builder::{BuilderError, BuilderResult};
    use openscenario_rs::types::positions::{Position, WorldPosition};

    struct PolylineBuilder {
        vertices: Vec<Vertex>,
    }

    impl PolylineBuilder {
        fn new() -> Self {
            Self {
                vertices: Vec::new(),
            }
        }

        fn add_vertex(mut self, vertex: Vertex) -> Self {
            self.vertices.push(vertex);
            self
        }

        fn build(self) -> BuilderResult<BoundedPolyline> {
            Ok(BoundedPolyline {
                vertices: MinVec::new(self.vertices)?,
            })
        }
    }

    fn vertex(x: f64, y: f64) -> Vertex {
        Vertex::new(Position::world(WorldPosition::new(x, y)))
    }

    #[test]
    fn a_builder_accumulating_two_vertices_builds() {
        let built = PolylineBuilder::new()
            .add_vertex(vertex(1.0, 2.0))
            .add_vertex(vertex(3.0, 4.0))
            .build()
            .expect("two vertices must build");
        assert_eq!(built.vertices.len(), 2);
    }

    #[test]
    fn a_builder_accumulating_one_vertex_fails_at_build() {
        let err = PolylineBuilder::new()
            .add_vertex(vertex(1.0, 2.0))
            .build()
            .unwrap_err();
        assert!(matches!(err, BuilderError::OpenScenarioError(_)));
        assert_eq!(
            err.to_string(),
            "OpenSCENARIO error: Validation error in field 'MinVec': expected at least 2 items, got 1"
        );
    }
}

// ---------------------------------------------------------------------------
// `MIN = 1`
//
// Every test above uses `MIN = 2`, because that is the value the mechanism was built
// for. Most schema particles that adopt `MinVec` are `minOccurs="1"` instead, and the
// length check is `items.len() < MIN`, so `MIN = 1` follows from the same arithmetic.
// That is a reading of the code rather than a measurement, hence these tests.
// ---------------------------------------------------------------------------

/// Mirrors `ConditionGroup`, whose XSD `Condition` particle takes the default
/// `minOccurs="1"`.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "Group")]
struct AtLeastOne {
    #[serde(rename = "Item")]
    items: MinVec<OSString, 1>,
}

#[test]
fn a_single_item_round_trips_byte_for_byte_at_min_one() {
    let xml = "<Group><Item>only</Item></Group>";
    let parsed: AtLeastOne = parse(xml).expect("one item satisfies MIN = 1");
    assert_eq!(parsed.items.as_slice().len(), 1);
    assert_eq!(to_xml(&parsed), xml);
}

#[test]
fn several_items_round_trip_byte_for_byte_at_min_one() {
    let xml = "<Group><Item>first</Item><Item>second</Item></Group>";
    let parsed: AtLeastOne = parse(xml).expect("two items satisfy MIN = 1");
    assert_eq!(parsed.items.as_slice().len(), 2);
    assert_eq!(to_xml(&parsed), xml);
}

/// At `MIN = 1` the empty document never reaches the length check. A field with no
/// `#[serde(default)]` fails earlier, in serde, with `missing field`, because there is no
/// vector to measure. The document is still rejected, so the bound holds; only the
/// diagnostic differs from the `MIN >= 2` case. Hence what `MinVec` adds at `MIN = 1` is
/// the construction side: an empty value cannot be built, so no serializer can emit one.
#[test]
fn an_empty_document_is_rejected_at_min_one() {
    let err = parse::<AtLeastOne>("<Group/>").expect_err("zero items violate MIN = 1");
    assert!(
        err.to_string().contains("missing field"),
        "unexpected error: {err}"
    );
}

#[test]
fn a_zero_length_value_cannot_be_constructed_at_min_one() {
    assert!(MinVec::<OSString, 1>::new(Vec::new()).is_err());
    assert!(MinVec::<OSString, 1>::new(vec![Value::literal("one".to_string())]).is_ok());
}

// ---------------------------------------------------------------------------
// `from_min` and `push`
//
// A caller with one guaranteed element (a `first: T, rest: Vec<T>` constructor, the
// shape several `::new` methods in the crate use) had no way to build a `MinVec`
// without going through `new` and its `Result`, even though the call could never fail.
// `from_min` closes that: the array argument is sized to `MIN` at the type level, so
// supplying it is the proof, with nothing left to check at runtime. `push` is the
// matching operation for growing a value the caller already holds.
// ---------------------------------------------------------------------------

#[test]
fn from_min_compiles_away_the_result_at_min_one() {
    // No `?`, no `.unwrap()`, no `Result` anywhere in this line: `from_min` returns
    // `MinVec` directly.
    let one: MinVec<u8, 1> = MinVec::from_min([1], vec![]);
    assert_eq!(one.as_slice(), &[1]);

    let several: MinVec<u8, 1> = MinVec::from_min([1], vec![2, 3]);
    assert_eq!(several.as_slice(), &[1, 2, 3]);
}

#[test]
fn from_min_takes_exactly_min_guaranteed_items_at_min_two() {
    // At `MIN = 2` a single guaranteed element is not proof of the bound; the array
    // must carry both. This is `MinVec<u8, 2>::from_min([1], ...)` failing to compile,
    // demonstrated the only way a compile failure can be: by writing the call that
    // does type-check.
    let two: MinVec<u8, 2> = MinVec::from_min([1, 2], vec![]);
    assert_eq!(two.as_slice(), &[1, 2]);

    let more: MinVec<u8, 2> = MinVec::from_min([1, 2], vec![3, 4]);
    assert_eq!(more.as_slice(), &[1, 2, 3, 4]);
}

#[test]
fn push_grows_a_live_min_vec_without_a_result() {
    let mut one: MinVec<u8, 1> = MinVec::from_min([1], vec![]);
    one.push(2);
    one.push(3);
    assert_eq!(one.as_slice(), &[1, 2, 3]);
}

#[test]
fn new_still_rejects_a_short_list_alongside_the_new_constructors() {
    // `from_min` and `push` are additions; `new`'s existing check is unchanged.
    assert!(MinVec::<u8, 2>::new(vec![1]).is_err());
    assert!(MinVec::<u8, 2>::new(vec![1, 2]).is_ok());
}
