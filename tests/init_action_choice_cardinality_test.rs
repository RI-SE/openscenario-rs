//! Cardinality of the three `Init` action choice groups.
//!
//! XSD `GlobalAction` (`Schema/OpenSCENARIO.xsd:1282-1295`), `PrivateAction` (`:1777-1791`)
//! and `LongitudinalAction` (`:1431-1437`) are each a bare `xsd:choice`. Neither the choice
//! element nor any of its branches carries `minOccurs`, so every branch defaults to 1 and an
//! instance must select exactly one. A document naming no branch and a document naming two
//! are both schema-invalid, and the Rust type is expected to reject both.
//!
//! Each claim is asserted in its own test. A single test that checked the zero-branch case and
//! then the two-branch case would stop at the first failure, and the two-branch case is the one
//! that silently kept both branches before this shape existed.

use openscenario_rs::types::scenario::init::{GlobalAction, LongitudinalAction, PrivateAction};

const GLOBAL_ONE_BRANCH: &str =
    r#"<GlobalAction><SetMonitorAction monitorRef="m1" value="true"/></GlobalAction>"#;
const GLOBAL_ZERO_BRANCHES: &str = r#"<GlobalAction/>"#;
const GLOBAL_TWO_BRANCHES: &str = r#"<GlobalAction><SetMonitorAction monitorRef="m1" value="true"/><EnvironmentAction><Environment name="e"/></EnvironmentAction></GlobalAction>"#;

const PRIVATE_ONE_BRANCH: &str = r#"<PrivateAction><VisibilityAction graphics="true" sensors="true" traffic="true"/></PrivateAction>"#;
const PRIVATE_ZERO_BRANCHES: &str = r#"<PrivateAction/>"#;
const PRIVATE_TWO_BRANCHES: &str = r#"<PrivateAction><VisibilityAction graphics="true" sensors="true" traffic="true"/><ActivateControllerAction lateral="true"/></PrivateAction>"#;

const LONGITUDINAL_ONE_BRANCH: &str = r#"<LongitudinalAction><LongitudinalDistanceAction entityRef="Ego" distance="10" freespace="true" continuous="false"/></LongitudinalAction>"#;
const LONGITUDINAL_ZERO_BRANCHES: &str = r#"<LongitudinalAction/>"#;
const LONGITUDINAL_TWO_BRANCHES: &str = r#"<LongitudinalAction><LongitudinalDistanceAction entityRef="Ego" freespace="true" continuous="false"/><SpeedProfileAction followingMode="follow"><SpeedProfileEntry speed="10"/></SpeedProfileAction></LongitudinalAction>"#;

#[test]
fn global_action_parses_its_selected_branch() {
    let parsed: GlobalAction = quick_xml::de::from_str(GLOBAL_ONE_BRANCH).unwrap();
    let openscenario_rs::types::scenario::init::GlobalActionChoice::SetMonitorAction(action) =
        &parsed.action
    else {
        panic!(
            "expected the SetMonitorAction branch, got {:?}",
            parsed.action
        );
    };
    assert_eq!(action.monitor_ref.as_literal().unwrap(), "m1");
}

#[test]
fn global_action_round_trips_byte_exactly() {
    let parsed: GlobalAction = quick_xml::de::from_str(GLOBAL_ONE_BRANCH).unwrap();
    assert_eq!(
        quick_xml::se::to_string(&parsed).unwrap(),
        GLOBAL_ONE_BRANCH
    );
}

#[test]
fn global_action_rejects_zero_branches() {
    let err = quick_xml::de::from_str::<GlobalAction>(GLOBAL_ZERO_BRANCHES).unwrap_err();
    assert!(
        err.to_string().contains("$value"),
        "expected a missing-$value error, got {err}"
    );
}

#[test]
fn global_action_rejects_two_branches() {
    let err = quick_xml::de::from_str::<GlobalAction>(GLOBAL_TWO_BRANCHES).unwrap_err();
    assert!(
        err.to_string().contains("$value"),
        "expected a duplicate-$value error, got {err}"
    );
}

#[test]
fn private_action_parses_its_selected_branch() {
    let parsed: PrivateAction = quick_xml::de::from_str(PRIVATE_ONE_BRANCH).unwrap();
    let openscenario_rs::types::scenario::init::PrivateActionChoice::VisibilityAction(action) =
        &parsed.action
    else {
        panic!(
            "expected the VisibilityAction branch, got {:?}",
            parsed.action
        );
    };
    assert_eq!(action.graphics.as_literal().unwrap(), &true);
}

#[test]
fn private_action_round_trips_byte_exactly() {
    let parsed: PrivateAction = quick_xml::de::from_str(PRIVATE_ONE_BRANCH).unwrap();
    assert_eq!(
        quick_xml::se::to_string(&parsed).unwrap(),
        PRIVATE_ONE_BRANCH
    );
}

#[test]
fn private_action_rejects_zero_branches() {
    let err = quick_xml::de::from_str::<PrivateAction>(PRIVATE_ZERO_BRANCHES).unwrap_err();
    assert!(
        err.to_string().contains("$value"),
        "expected a missing-$value error, got {err}"
    );
}

#[test]
fn private_action_rejects_two_branches() {
    let err = quick_xml::de::from_str::<PrivateAction>(PRIVATE_TWO_BRANCHES).unwrap_err();
    assert!(
        err.to_string().contains("$value"),
        "expected a duplicate-$value error, got {err}"
    );
}

#[test]
fn longitudinal_action_parses_its_selected_branch() {
    let parsed: LongitudinalAction = quick_xml::de::from_str(LONGITUDINAL_ONE_BRANCH).unwrap();
    let openscenario_rs::types::scenario::init::LongitudinalActionChoice::LongitudinalDistanceAction(
        action,
    ) = &parsed.action
    else {
        panic!(
            "expected the LongitudinalDistanceAction branch, got {:?}",
            parsed.action
        );
    };
    assert_eq!(action.entity_ref.as_literal().unwrap(), "Ego");
}

#[test]
fn longitudinal_action_round_trips_byte_exactly() {
    let parsed: LongitudinalAction = quick_xml::de::from_str(LONGITUDINAL_ONE_BRANCH).unwrap();
    assert_eq!(
        quick_xml::se::to_string(&parsed).unwrap(),
        LONGITUDINAL_ONE_BRANCH
    );
}

#[test]
fn longitudinal_action_rejects_zero_branches() {
    let err =
        quick_xml::de::from_str::<LongitudinalAction>(LONGITUDINAL_ZERO_BRANCHES).unwrap_err();
    assert!(
        err.to_string().contains("$value"),
        "expected a missing-$value error, got {err}"
    );
}

#[test]
fn longitudinal_action_rejects_two_branches() {
    let err = quick_xml::de::from_str::<LongitudinalAction>(LONGITUDINAL_TWO_BRANCHES).unwrap_err();
    assert!(
        err.to_string().contains("$value"),
        "expected a duplicate-$value error, got {err}"
    );
}
