//! The document's main spine carries eight repeated elements whose XSD particles declare a
//! minimum. Seven take the default `minOccurs="1"`; `Route`'s `Waypoint` declares
//! `minOccurs="2"`. Each field is a `MinVec<T, N>` with `N` read from the schema.
//!
//! Two claims are asserted per field, each in its own test, because a combined test stops
//! at the first failure and the second claim is usually the interesting one:
//!
//!   1. a document with fewer than `N` children is rejected;
//!   2. a document with exactly `N` round-trips byte for byte.
//!
//! The `minOccurs="1"` fields were already rejected on the way in, since none of them
//! carried `#[serde(default)]`; serde reports those as a missing field before the length
//! check runs. What the type adds for them is the other direction: an empty value cannot be
//! constructed, so no serializer can emit `<ConditionGroup/>` or
//! `<Private entityRef="Ego"/>`. Those cases are asserted through the constructors.

use openscenario_rs::types::basic::Value;
use openscenario_rs::types::enums::{Priority, RouteStrategy, TriggeringEntitiesRule};
use openscenario_rs::types::routing::{Route, Waypoint};
use openscenario_rs::types::scenario::init::Private;
use openscenario_rs::types::scenario::story::{
    Act, Event, Maneuver, ScenarioStory, StoryAction, StoryPrivateAction,
};
use openscenario_rs::types::scenario::triggers::{ConditionGroup, EntityRef, TriggeringEntities};
use serde::{Deserialize, Serialize};

fn parse<T: for<'de> Deserialize<'de>>(xml: &str) -> Result<T, quick_xml::de::DeError> {
    quick_xml::de::from_str(xml)
}

fn to_xml<T: Serialize>(value: &T) -> String {
    quick_xml::se::to_string(value).expect("serialization failed")
}

fn one_action() -> Vec<StoryAction> {
    vec![StoryAction::private(
        "Action1",
        StoryPrivateAction::visibility(openscenario_rs::types::actions::VisibilityAction::new(
            true, true, true,
        )),
    )]
}

// ---------------------------------------------------------------------------
// Route.waypoints — XSD Route, `<xsd:element name="Waypoint" minOccurs="2"/>`
//
// This is the field that justifies the mechanism. It never carried
// `#[serde(default)]`, so removing that attribute could not reach it: the element was
// already required, and one waypoint satisfied "required".
// ---------------------------------------------------------------------------

const ONE_WAYPOINT: &str = concat!(
    r#"<Route closed="false" name="R">"#,
    r#"<Waypoint routeStrategy="fastest"><Position><WorldPosition x="0" y="0" z="0"/></Position></Waypoint>"#,
    r#"</Route>"#
);

const TWO_WAYPOINTS: &str = concat!(
    r#"<Route closed="false" name="R">"#,
    r#"<Waypoint routeStrategy="fastest"><Position><WorldPosition x="0" y="0" z="0"/></Position></Waypoint>"#,
    r#"<Waypoint routeStrategy="fastest"><Position><WorldPosition x="1" y="0" z="0"/></Position></Waypoint>"#,
    r#"</Route>"#
);

#[test]
fn a_route_with_one_waypoint_is_rejected_at_parse_time() {
    let err = parse::<Route>(ONE_WAYPOINT).expect_err("one waypoint violates minOccurs=2");
    assert!(
        err.to_string().contains("at least 2"),
        "unexpected error: {err}"
    );
}

#[test]
fn a_route_with_two_waypoints_round_trips_byte_for_byte() {
    let parsed: Route = parse(TWO_WAYPOINTS).expect("two waypoints satisfy minOccurs=2");
    assert_eq!(to_xml(&parsed), TWO_WAYPOINTS);
}

#[test]
fn a_route_with_two_waypoints_keeps_both() {
    let parsed: Route = parse(TWO_WAYPOINTS).expect("two waypoints satisfy minOccurs=2");
    assert_eq!(parsed.waypoints.len(), 2);
}

#[test]
fn a_short_route_cannot_be_constructed() {
    assert!(Route::new("R", false, Vec::new()).is_err());
    assert!(Route::new(
        "R",
        false,
        vec![Waypoint::world_position(
            0.0,
            0.0,
            0.0,
            RouteStrategy::Fastest
        )]
    )
    .is_err());
}

// ---------------------------------------------------------------------------
// ConditionGroup.conditions — XSD ConditionGroup, `Condition` at the default minOccurs="1"
// ---------------------------------------------------------------------------

const ONE_CONDITION: &str = concat!(
    r#"<ConditionGroup><Condition name="C" conditionEdge="rising" delay="0">"#,
    r#"<ByValueCondition><SimulationTimeCondition value="1" rule="greaterThan"/></ByValueCondition>"#,
    r#"</Condition></ConditionGroup>"#
);

#[test]
fn an_empty_condition_group_is_rejected_at_parse_time() {
    assert!(parse::<ConditionGroup>("<ConditionGroup/>").is_err());
}

#[test]
fn a_condition_group_with_one_condition_round_trips_byte_for_byte() {
    let parsed: ConditionGroup = parse(ONE_CONDITION).expect("one condition is enough");
    assert_eq!(to_xml(&parsed), ONE_CONDITION);
}

#[test]
fn an_empty_condition_group_cannot_be_constructed() {
    // The serialization half of the bound: before this, `<ConditionGroup/>` was a value
    // the crate could build and emit even though it could not read it back.
    assert!(ConditionGroup::new(Vec::new()).is_err());
}

// ---------------------------------------------------------------------------
// TriggeringEntities.entity_refs — XSD TriggeringEntities, default minOccurs="1"
// ---------------------------------------------------------------------------

const ONE_ENTITY_REF: &str = concat!(
    r#"<TriggeringEntities triggeringEntitiesRule="any">"#,
    r#"<EntityRef entityRef="Ego"/>"#,
    r#"</TriggeringEntities>"#
);

#[test]
fn triggering_entities_with_no_entity_ref_is_rejected_at_parse_time() {
    assert!(
        parse::<TriggeringEntities>(r#"<TriggeringEntities triggeringEntitiesRule="any"/>"#)
            .is_err()
    );
}

#[test]
fn triggering_entities_with_one_entity_ref_round_trips_byte_for_byte() {
    let parsed: TriggeringEntities = parse(ONE_ENTITY_REF).expect("one entity ref is enough");
    assert_eq!(to_xml(&parsed), ONE_ENTITY_REF);
}

#[test]
fn empty_triggering_entities_cannot_be_constructed() {
    assert!(TriggeringEntities::new(TriggeringEntitiesRule::Any, Vec::new()).is_err());
    assert!(TriggeringEntities::any(vec![EntityRef::new("Ego")]).is_ok());
}

// ---------------------------------------------------------------------------
// Private.private_actions — XSD Private, default minOccurs="1"
// ---------------------------------------------------------------------------

const ONE_PRIVATE_ACTION: &str = concat!(
    r#"<Private entityRef="Ego"><PrivateAction>"#,
    r#"<VisibilityAction graphics="true" sensors="true" traffic="true"/>"#,
    r#"</PrivateAction></Private>"#
);

#[test]
fn a_private_container_with_no_action_is_rejected_at_parse_time() {
    assert!(parse::<Private>(r#"<Private entityRef="Ego"/>"#).is_err());
}

#[test]
fn a_private_container_with_one_action_round_trips_byte_for_byte() {
    let parsed: Private = parse(ONE_PRIVATE_ACTION).expect("one action is enough");
    assert_eq!(to_xml(&parsed), ONE_PRIVATE_ACTION);
}

#[test]
fn an_empty_private_container_cannot_be_constructed() {
    assert!(Private::new("Ego", Vec::new()).is_err());
}

// ---------------------------------------------------------------------------
// Event.actions — XSD Event, `Action` at the default minOccurs="1"
//
// The item type is `StoryAction`, whose `$value` choice was converted earlier. This is
// the composition a `MinVec` of a choice-bearing type has to survive.
// ---------------------------------------------------------------------------

const ONE_EVENT_ACTION: &str = concat!(
    r#"<Event name="E" priority="override">"#,
    r#"<Action name="Action1"><PrivateAction>"#,
    r#"<VisibilityAction graphics="true" sensors="true" traffic="true"/>"#,
    r#"</PrivateAction></Action></Event>"#
);

#[test]
fn an_event_with_no_action_is_rejected_at_parse_time() {
    assert!(parse::<Event>(r#"<Event name="E" priority="override"/>"#).is_err());
}

#[test]
fn an_event_with_one_action_round_trips_byte_for_byte() {
    let parsed: Event = parse(ONE_EVENT_ACTION).expect("one action is enough");
    assert_eq!(to_xml(&parsed), ONE_EVENT_ACTION);
}

#[test]
fn an_event_with_no_action_cannot_be_constructed() {
    assert!(Event::new("E", Priority::Override, Vec::new()).is_err());
    assert!(Event::new("E", Priority::Override, one_action()).is_ok());
}

// ---------------------------------------------------------------------------
// Maneuver.events — XSD Maneuver, `Event` at the default minOccurs="1"
// ---------------------------------------------------------------------------

#[test]
fn a_maneuver_with_no_event_is_rejected_at_parse_time() {
    assert!(parse::<Maneuver>(r#"<Maneuver name="M"/>"#).is_err());
}

#[test]
fn a_maneuver_with_one_event_round_trips_byte_for_byte() {
    let xml = format!(r#"<Maneuver name="M">{ONE_EVENT_ACTION}</Maneuver>"#);
    let parsed: Maneuver = parse(&xml).expect("one event is enough");
    assert_eq!(to_xml(&parsed), xml);
}

#[test]
fn a_maneuver_with_no_event_cannot_be_constructed() {
    assert!(Maneuver::new("M", Vec::new()).is_err());
}

// ---------------------------------------------------------------------------
// Act.maneuver_groups — XSD Act, `ManeuverGroup` at the default minOccurs="1"
// ---------------------------------------------------------------------------

const ONE_MANEUVER_GROUP: &str = concat!(
    r#"<ManeuverGroup name="G" maximumExecutionCount="1">"#,
    r#"<Actors selectTriggeringEntities="false"/>"#,
    r#"</ManeuverGroup>"#
);

#[test]
fn an_act_with_no_maneuver_group_is_rejected_at_parse_time() {
    assert!(parse::<Act>(r#"<Act name="A"/>"#).is_err());
}

#[test]
fn an_act_with_one_maneuver_group_round_trips_byte_for_byte() {
    let xml = format!(r#"<Act name="A">{ONE_MANEUVER_GROUP}</Act>"#);
    let parsed: Act = parse(&xml).expect("one maneuver group is enough");
    assert_eq!(to_xml(&parsed), xml);
}

#[test]
fn an_act_with_no_maneuver_group_cannot_be_constructed() {
    assert!(Act::new("A", Vec::new()).is_err());
}

// ---------------------------------------------------------------------------
// ScenarioStory.acts — XSD Story, `Act` at the default minOccurs="1"
// ---------------------------------------------------------------------------

#[test]
fn a_story_with_no_act_is_rejected_at_parse_time() {
    assert!(parse::<ScenarioStory>(r#"<Story name="S"/>"#).is_err());
}

#[test]
fn a_story_with_one_act_round_trips_byte_for_byte() {
    // `ScenarioStory` carries no `#[serde(rename = "Story")]`, so serializing it on its
    // own writes its Rust name. In production it is reached through `Storyboard`, whose
    // field supplies the element name. The root is therefore given explicitly here so the
    // assertion compares the content rather than the wrapper.
    let xml = format!(r#"<Story name="S"><Act name="A">{ONE_MANEUVER_GROUP}</Act></Story>"#);
    let parsed: ScenarioStory = parse(&xml).expect("one act is enough");
    let mut out = String::new();
    quick_xml::se::to_writer_with_root(&mut out, "Story", &parsed).expect("serialization failed");
    assert_eq!(out, xml);
}

#[test]
fn a_story_with_no_act_cannot_be_constructed() {
    assert!(ScenarioStory::new("S", Vec::new()).is_err());
}

// ---------------------------------------------------------------------------
// The builder defect this conversion exposed.
// ---------------------------------------------------------------------------

#[test]
fn a_private_action_builder_with_no_action_reports_rather_than_emitting_an_empty_container() {
    use openscenario_rs::builder::init::{InitActionBuilder, PrivateActionBuilder};

    // Before, this produced `<Private entityRef="ego"/>` and returned `Ok`.
    assert!(PrivateActionBuilder::new(InitActionBuilder::new(), "ego")
        .build()
        .is_err());
}

#[test]
fn an_entity_ref_carries_its_name_through_a_round_trip() {
    let parsed: TriggeringEntities = parse(ONE_ENTITY_REF).expect("one entity ref is enough");
    assert_eq!(
        parsed.entity_refs[0].entity_ref,
        Value::literal("Ego".to_string())
    );
}
