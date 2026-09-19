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
