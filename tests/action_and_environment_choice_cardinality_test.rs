//! `NamedAction` and `EnvironmentAction` used to model their XSD choice as
//! parallel `Option` fields: zero branches parsed to all-`None`, and two
//! branches parsed with both kept, producing a value no schema-valid document
//! can describe. Both are now a single `$value` field over an externally-tagged
//! enum, so serde enforces "exactly one branch" structurally: a document naming
//! none fails with `missing field $value`, and one naming two fails with
//! `duplicate field $value`.

use openscenario_rs::types::actions::wrappers::{
    GlobalAction, NamedAction, NamedActionChoice, SetMonitorAction,
};
use openscenario_rs::types::scenario::init::{EnvironmentAction, EnvironmentActionChoice};

fn de<T: serde::de::DeserializeOwned>(xml: &str) -> T {
    quick_xml::de::from_str(xml).unwrap_or_else(|e| panic!("deserialize failed for {xml}: {e}"))
}

fn de_err<T: serde::de::DeserializeOwned>(xml: &str) -> String {
    match quick_xml::de::from_str::<T>(xml) {
        Ok(_) => panic!("expected deserialize to fail for {xml}"),
        Err(e) => e.to_string(),
    }
}

// ═══ NamedAction — XSD `Action` (:705-712), @name required + bare choice ═══

#[test]
fn named_action_global_branch_round_trips_byte_identically() {
    let xml = r#"<Action name="a1"><GlobalAction><SetMonitorAction monitorRef="m1" value="true"/></GlobalAction></Action>"#;
    let action: NamedAction = de(xml);
    assert!(matches!(action.action, NamedActionChoice::GlobalAction(_)));
    let out = quick_xml::se::to_string_with_root("Action", &action)
        .expect("NamedAction/GlobalAction failed to serialize");
    assert_eq!(out, xml);
}

#[test]
fn named_action_rejects_zero_branches() {
    let xml = r#"<Action name="a1"></Action>"#;
    let err = de_err::<NamedAction>(xml);
    assert!(err.contains("missing field `$value`"), "got: {err}");
}

#[test]
fn named_action_rejects_two_branches() {
    let xml = r#"<Action name="a1"><UserDefinedAction><CustomCommandAction type="c"/></UserDefinedAction><PrivateAction><TeleportAction><Position><WorldPosition x="1" y="2"/></Position></TeleportAction></PrivateAction></Action>"#;
    let err = de_err::<NamedAction>(xml);
    assert!(err.contains("duplicate field `$value`"), "got: {err}");
}

#[test]
fn named_action_via_global_constructor_round_trips() {
    let action = NamedAction::global(
        "a1",
        GlobalAction::SetMonitorAction(SetMonitorAction::new("m1", true)),
    );
    let xml = r#"<Action name="a1"><GlobalAction><SetMonitorAction monitorRef="m1" value="true"/></GlobalAction></Action>"#;
    let out = quick_xml::se::to_string_with_root("Action", &action)
        .expect("NamedAction::global failed to serialize");
    assert_eq!(out, xml);
    let reparsed: NamedAction = de(&out);
    assert_eq!(action, reparsed);
}

// ═══ EnvironmentAction — XSD `EnvironmentAction` (:1195-1200), bare choice ═══

#[test]
fn environment_action_environment_branch_round_trips_byte_identically() {
    let xml = r#"<EnvironmentAction><Environment name="Env1"/></EnvironmentAction>"#;
    let action: EnvironmentAction = de(xml);
    assert!(matches!(
        action.action,
        EnvironmentActionChoice::Environment(_)
    ));
    let out = quick_xml::se::to_string_with_root("EnvironmentAction", &action)
        .expect("EnvironmentAction/Environment failed to serialize");
    assert_eq!(out, xml);
}

#[test]
fn environment_action_catalog_reference_branch_round_trips_byte_identically() {
    let xml = r#"<EnvironmentAction><CatalogReference catalogName="EnvCatalog" entryName="Sunny"/></EnvironmentAction>"#;
    let action: EnvironmentAction = de(xml);
    assert!(matches!(
        action.action,
        EnvironmentActionChoice::CatalogReference(_)
    ));
    let out = quick_xml::se::to_string_with_root("EnvironmentAction", &action)
        .expect("EnvironmentAction/CatalogReference failed to serialize");
    assert_eq!(out, xml);
}

#[test]
fn environment_action_rejects_zero_branches() {
    let xml = r#"<EnvironmentAction></EnvironmentAction>"#;
    let err = de_err::<EnvironmentAction>(xml);
    assert!(err.contains("missing field `$value`"), "got: {err}");
}

#[test]
fn environment_action_rejects_two_branches() {
    let xml = r#"<EnvironmentAction><Environment name="Env1"/><CatalogReference catalogName="EnvCatalog" entryName="Sunny"/></EnvironmentAction>"#;
    let err = de_err::<EnvironmentAction>(xml);
    assert!(err.contains("duplicate field `$value`"), "got: {err}");
}
