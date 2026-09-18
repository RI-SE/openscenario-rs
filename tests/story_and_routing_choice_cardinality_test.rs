//! Cardinality and round-trip tests for the three `xsd:choice` groups that
//! `Action` (XSD:705-712), `PrivateAction` (XSD:1777-1791) and `RoutingAction`
//! (XSD:1981-1988) declare.
//!
//! Each of the three is a bare `xsd:choice`, so `minOccurs` and `maxOccurs`
//! both default to 1 and exactly one branch is required. Modeling that as
//! parallel `Option` fields expresses `xsd:all` with optional members instead,
//! which accepts a document naming no branch and a document naming two, the
//! second of which is then silently discarded. An externally-tagged enum
//! behind `$value` expresses the choice itself, so serde rejects both cases
//! structurally.
//!
//! Every cardinality claim is asserted in a test of its own. A single test
//! stops at its first failure, so the two-branch claim would never be measured
//! if it shared a test with the zero-branch claim.

use openscenario_rs::types::actions::movement::RoutingAction;
use openscenario_rs::types::scenario::story::{StoryAction, StoryPrivateAction};

fn de<T: serde::de::DeserializeOwned>(xml: &str) -> T {
    quick_xml::de::from_str(xml).unwrap_or_else(|e| panic!("deserialize failed for {xml}: {e}"))
}

fn ser<T: serde::Serialize>(root: &str, v: &T) -> String {
    quick_xml::se::to_string_with_root(root, v).expect("serialize failed")
}

fn reject<T: serde::de::DeserializeOwned + std::fmt::Debug>(xml: &str) -> String {
    match quick_xml::de::from_str::<T>(xml) {
        Ok(v) => panic!("schema-invalid document was accepted: {xml}\nparsed as: {v:?}"),
        Err(e) => e.to_string(),
    }
}

// --- RoutingAction (XSD:1981-1988) ------------------------------------------

#[test]
fn routing_action_rejects_zero_branches() {
    let err = reject::<RoutingAction>(r#"<RoutingAction/>"#);
    assert!(err.contains("$value"), "got: {err}");
}

#[test]
fn routing_action_rejects_two_branches() {
    let err = reject::<RoutingAction>(
        r#"<RoutingAction><RandomRouteAction/><AcquirePositionAction><Position><WorldPosition x="1" y="2"/></Position></AcquirePositionAction></RoutingAction>"#,
    );
    assert!(err.contains("$value"), "got: {err}");
}

#[test]
fn routing_action_single_branch_round_trips_byte_identically() {
    let xml = r#"<RoutingAction><RandomRouteAction/></RoutingAction>"#;
    let action: RoutingAction = de(xml);
    assert_eq!(ser("RoutingAction", &action), xml);
}

#[test]
fn routing_action_acquire_position_branch_round_trips_byte_identically() {
    let xml = r#"<RoutingAction><AcquirePositionAction><Position><WorldPosition x="1" y="2"/></Position></AcquirePositionAction></RoutingAction>"#;
    let action: RoutingAction = de(xml);
    assert_eq!(ser("RoutingAction", &action), xml);
}

// --- Action, as modeled by StoryAction (XSD:705-712) ------------------------

#[test]
fn story_action_rejects_zero_branches() {
    let err = reject::<StoryAction>(r#"<Action name="x"/>"#);
    assert!(err.contains("$value"), "got: {err}");
}

#[test]
fn story_action_rejects_two_branches() {
    let err = reject::<StoryAction>(
        r#"<Action name="x"><UserDefinedAction><CustomCommandAction type="t"/></UserDefinedAction><PrivateAction><TeleportAction><Position><WorldPosition x="1" y="2"/></Position></TeleportAction></PrivateAction></Action>"#,
    );
    assert!(err.contains("$value"), "got: {err}");
}

#[test]
fn story_action_single_branch_round_trips_byte_identically() {
    let xml = r#"<Action name="x"><UserDefinedAction><CustomCommandAction type="t"/></UserDefinedAction></Action>"#;
    let action: StoryAction = de(xml);
    assert_eq!(action.name.to_string(), "x");
    assert_eq!(ser("Action", &action), xml);
}

// --- PrivateAction, as modeled by StoryPrivateAction (XSD:1777-1791) --------

#[test]
fn story_private_action_rejects_zero_branches() {
    let err = reject::<StoryPrivateAction>(r#"<PrivateAction/>"#);
    assert!(err.contains("$value"), "got: {err}");
}

#[test]
fn story_private_action_rejects_two_branches() {
    let err = reject::<StoryPrivateAction>(
        r#"<PrivateAction><TeleportAction><Position><WorldPosition x="1" y="2"/></Position></TeleportAction><RoutingAction><RandomRouteAction/></RoutingAction></PrivateAction>"#,
    );
    assert!(err.contains("$value"), "got: {err}");
}

#[test]
fn story_private_action_single_branch_round_trips_byte_identically() {
    let xml =
        r#"<PrivateAction><RoutingAction><RandomRouteAction/></RoutingAction></PrivateAction>"#;
    let action: StoryPrivateAction = de(xml);
    assert_eq!(ser("PrivateAction", &action), xml);
}
