//! Every branch a `PrivateActionBuilder` is handed survives the trip into an `Init` block.
//!
//! `PrivateActionBuilder::convert_to_init_action` used to map seven of the XSD `PrivateAction`
//! choice's ten branches (`Schema/OpenSCENARIO.xsd:1777-1791`) and send the rest to a catch-all
//! that produced an action naming no branch. A caller who added an activate-controller,
//! appearance or trailer action got an empty `<PrivateAction/>` back — schema-invalid, and
//! silent, because the old parallel-`Option` shape could represent "no branch selected". The
//! match is now exhaustive; these tests pin that it stays so.
//!
//! The branch each test checks is asserted twice and separately: once on the built value, and
//! once on the serialized bytes. The first says the builder chose a branch, the second says the
//! branch reaches the document — the defect was visible only in the second.

#![cfg(feature = "builder")]

use openscenario_rs::builder::init::{GlobalActionBuilder, InitActionBuilder};
use openscenario_rs::types::actions::appearance::{AppearanceAction, AppearanceActionChoice};
use openscenario_rs::types::actions::control::ActivateControllerAction;
use openscenario_rs::types::actions::trailer::{
    DisconnectTrailerAction, TrailerAction, TrailerActionChoice,
};
use openscenario_rs::types::actions::wrappers::PrivateAction as PrivateActionWrapper;

/// Builds an `Init` holding exactly one private action for `"Ego"`, and returns it.
fn init_with(action: PrivateActionWrapper) -> openscenario_rs::types::scenario::init::Init {
    InitActionBuilder::new()
        .create_private_action("Ego")
        .add_action(action)
        .finish()
        .build()
        .unwrap()
}

fn only_private_action(
    init: &openscenario_rs::types::scenario::init::Init,
) -> &openscenario_rs::types::scenario::init::PrivateAction {
    let private = &init.actions.private_actions[0];
    assert_eq!(
        private.private_actions.len(),
        1,
        "expected exactly the one action that was added"
    );
    &private.private_actions[0]
}

fn appearance_action() -> AppearanceAction {
    let light_state = quick_xml::de::from_str(
        r#"<LightStateAction><LightType><VehicleLight vehicleLightType="brakeLights"/></LightType><LightState mode="on"/></LightStateAction>"#,
    )
    .expect("the LightStateAction fixture must parse");
    AppearanceAction::new(AppearanceActionChoice::LightStateAction(light_state))
}

#[test]
fn activate_controller_branch_survives_the_builder() {
    let init = init_with(PrivateActionWrapper::ActivateControllerAction(
        ActivateControllerAction::default(),
    ));
    assert_eq!(
        only_private_action(&init).action_type(),
        "ActivateControllerAction"
    );
}

#[test]
fn activate_controller_branch_reaches_the_document() {
    let init = init_with(PrivateActionWrapper::ActivateControllerAction(
        ActivateControllerAction::default(),
    ));
    let xml = quick_xml::se::to_string(only_private_action(&init)).unwrap();
    assert!(
        xml.contains("<ActivateControllerAction"),
        "the branch was dropped and re-serialized as {xml}"
    );
}

#[test]
fn appearance_branch_survives_the_builder() {
    let init = init_with(PrivateActionWrapper::AppearanceAction(appearance_action()));
    assert_eq!(only_private_action(&init).action_type(), "AppearanceAction");
}

#[test]
fn appearance_branch_reaches_the_document() {
    let init = init_with(PrivateActionWrapper::AppearanceAction(appearance_action()));
    let xml = quick_xml::se::to_string(only_private_action(&init)).unwrap();
    assert!(
        xml.contains("<AppearanceAction"),
        "the branch was dropped and re-serialized as {xml}"
    );
}

#[test]
fn trailer_branch_survives_the_builder() {
    let init = init_with(PrivateActionWrapper::TrailerAction(TrailerAction {
        choice: TrailerActionChoice::DisconnectTrailerAction(DisconnectTrailerAction {}),
    }));
    assert_eq!(only_private_action(&init).action_type(), "TrailerAction");
}

#[test]
fn trailer_branch_reaches_the_document() {
    let init = init_with(PrivateActionWrapper::TrailerAction(TrailerAction {
        choice: TrailerActionChoice::DisconnectTrailerAction(DisconnectTrailerAction {}),
    }));
    let xml = quick_xml::se::to_string(only_private_action(&init)).unwrap();
    assert!(
        xml.contains("<TrailerAction"),
        "the branch was dropped and re-serialized as {xml}"
    );
}

#[test]
fn global_action_builder_reports_a_missing_branch_instead_of_inventing_one() {
    let result = GlobalActionBuilder::new(InitActionBuilder::new()).build();
    assert!(
        result.is_err(),
        "a GlobalAction naming no branch is schema-invalid and must not build"
    );
}

#[test]
fn global_action_builder_finish_contributes_nothing_when_no_branch_was_selected() {
    let init = GlobalActionBuilder::new(InitActionBuilder::new())
        .finish()
        .build()
        .unwrap();
    assert!(
        init.actions.global_actions.is_empty(),
        "finish() has no error channel, so it must contribute nothing rather than an empty element"
    );
}
