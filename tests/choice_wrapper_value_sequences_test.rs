//! Sequences below the element wrappers that host an `xsd:choice`.
//!
//! `#[serde(flatten)]` makes serde buffer an element's children into a
//! `Content` map through `deserialize_any`. quick-xml cannot know at that point
//! that a child will later be read back as a sequence, so a `Vec<T>` replayed
//! out of the buffer fails with `invalid type: map, expected a sequence`, even
//! when the element occurs only once.
//!
//! The wrappers exercised here therefore host their choice enum behind the
//! special field name `$value`, which takes the element name from the
//! serialized type so that variant names become element tags. That reads the
//! child from the live reader, so a sequence below it deserializes, and serde
//! enforces the choice cardinality structurally: no branch is a missing
//! `$value` and two branches are a duplicate one.
//!
//! `Route` requires `minOccurs="2"` `Waypoint` children
//! (`Schema/OpenSCENARIO.xsd:1955-1962`), so before this shape no conformant
//! inline route could be parsed by either of the two paths that reach one.

use openscenario_rs::types::actions::movement::{AssignRouteAction, LongitudinalAction};
use openscenario_rs::types::positions::route::RouteRefElement;
use openscenario_rs::types::scenario::story::StoryGlobalAction;

fn de<T: serde::de::DeserializeOwned>(xml: &str) -> T {
    quick_xml::de::from_str(xml).unwrap_or_else(|e| panic!("deserialize failed for {xml}: {e}"))
}

fn de_err<T: serde::de::DeserializeOwned>(xml: &str) -> String {
    match quick_xml::de::from_str::<T>(xml) {
        Ok(_) => panic!("expected deserialize to fail for {xml}"),
        Err(e) => e.to_string(),
    }
}

fn round_trip<T>(xml: &str)
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    let parsed: T = de(xml);
    let emitted = quick_xml::se::to_string(&parsed).expect("serialize failed");
    assert_eq!(emitted.as_bytes(), xml.as_bytes());
}

/// An inline route with the two waypoints the schema requires.
const INLINE_ROUTE: &str = concat!(
    r#"<Route closed="false" name="R">"#,
    r#"<Waypoint routeStrategy="fastest"><Position><WorldPosition x="0" y="0"/></Position></Waypoint>"#,
    r#"<Waypoint routeStrategy="fastest"><Position><WorldPosition x="1" y="1"/></Position></Waypoint>"#,
    r#"</Route>"#
);

/// A `TrafficAction` carrying `RoadRange`, whose `RoadCursor` is `minOccurs="2"`.
const TRAFFIC_AREA_ACTION: &str = concat!(
    r#"<TrafficAction><TrafficAreaAction numberOfEntities="100" continuous="true">"#,
    r#"<TrafficDistribution><TrafficDistributionEntry weight="1"><EntityDistribution>"#,
    r#"<EntityDistributionEntry weight="1"><ScenarioObjectTemplate>"#,
    r#"<CatalogReference catalogName="c" entryName="e"/>"#,
    r#"</ScenarioObjectTemplate></EntityDistributionEntry></EntityDistribution>"#,
    r#"</TrafficDistributionEntry></TrafficDistribution>"#,
    r#"<TrafficArea><RoadRange><RoadCursor roadId="R" s="100"/>"#,
    r#"<RoadCursor roadId="R" s="500"/></RoadRange></TrafficArea>"#,
    r#"</TrafficAreaAction></TrafficAction>"#
);

// ── AssignRouteAction (XSD `AssignRouteAction`, :786-791) ────────────────────

#[test]
fn assign_route_action_parses_inline_route_with_two_waypoints() {
    let xml = format!("<AssignRouteAction>{INLINE_ROUTE}</AssignRouteAction>");
    round_trip::<AssignRouteAction>(&xml);
}

#[test]
fn assign_route_action_rejects_zero_branches() {
    let err = de_err::<AssignRouteAction>("<AssignRouteAction/>");
    assert!(
        err.contains("missing field `$value`"),
        "unexpected error: {err}"
    );
}

#[test]
fn assign_route_action_rejects_two_branches() {
    let xml = format!(
        "<AssignRouteAction>{INLINE_ROUTE}<CatalogReference catalogName=\"c\" entryName=\"e\"/></AssignRouteAction>"
    );
    let err = de_err::<AssignRouteAction>(&xml);
    assert!(
        err.contains("duplicate field `$value`"),
        "unexpected error: {err}"
    );
}

// ── RouteRefElement (XSD `RouteRef`, :1975-1980) ─────────────────────────────

#[test]
fn route_ref_element_parses_inline_route_with_two_waypoints() {
    let xml = format!("<RouteRef>{INLINE_ROUTE}</RouteRef>");
    round_trip::<RouteRefElement>(&xml);
}

#[test]
fn route_ref_element_rejects_zero_branches() {
    let err = de_err::<RouteRefElement>("<RouteRef/>");
    assert!(
        err.contains("missing field `$value`"),
        "unexpected error: {err}"
    );
}

#[test]
fn route_ref_element_rejects_two_branches() {
    let xml = format!(
        "<RouteRef>{INLINE_ROUTE}<CatalogReference catalogName=\"c\" entryName=\"e\"/></RouteRef>"
    );
    let err = de_err::<RouteRefElement>(&xml);
    assert!(
        err.contains("duplicate field `$value`"),
        "unexpected error: {err}"
    );
}

// ── LongitudinalAction (XSD `LongitudinalAction`, :1431-1437) ────────────────

const SPEED_PROFILE_ACTION: &str = concat!(
    r#"<SpeedProfileAction followingMode="position">"#,
    r#"<SpeedProfileEntry time="0" speed="10"/>"#,
    r#"<SpeedProfileEntry time="1" speed="20"/>"#,
    r#"</SpeedProfileAction>"#
);

#[test]
fn longitudinal_action_parses_speed_profile_with_two_entries() {
    let xml = format!("<LongitudinalAction>{SPEED_PROFILE_ACTION}</LongitudinalAction>");
    round_trip::<LongitudinalAction>(&xml);
}

#[test]
fn longitudinal_action_rejects_zero_branches() {
    let err = de_err::<LongitudinalAction>("<LongitudinalAction/>");
    assert!(
        err.contains("missing field `$value`"),
        "unexpected error: {err}"
    );
}

#[test]
fn longitudinal_action_rejects_two_branches() {
    let xml = format!(
        "<LongitudinalAction>{SPEED_PROFILE_ACTION}<LongitudinalDistanceAction entityRef=\"e\" freespace=\"true\" continuous=\"true\"/></LongitudinalAction>"
    );
    let err = de_err::<LongitudinalAction>(&xml);
    assert!(
        err.contains("duplicate field `$value`"),
        "unexpected error: {err}"
    );
}

// ── StoryGlobalAction (XSD `GlobalAction`, :1282-1293) ───────────────────────

#[test]
fn story_global_action_parses_traffic_action_with_sequences() {
    let xml = format!("<GlobalAction>{TRAFFIC_AREA_ACTION}</GlobalAction>");
    round_trip::<StoryGlobalAction>(&xml);
}

#[test]
fn story_global_action_rejects_zero_branches() {
    let err = de_err::<StoryGlobalAction>("<GlobalAction/>");
    assert!(
        err.contains("missing field `$value`"),
        "unexpected error: {err}"
    );
}

#[test]
fn story_global_action_rejects_two_branches() {
    let xml = format!(
        "<GlobalAction>{TRAFFIC_AREA_ACTION}<SetMonitorAction monitorRef=\"m\" value=\"true\"/></GlobalAction>"
    );
    let err = de_err::<StoryGlobalAction>(&xml);
    assert!(
        err.contains("duplicate field `$value`"),
        "unexpected error: {err}"
    );
}
