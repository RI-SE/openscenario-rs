//! OSR-09: runtime evidence for the `Default` policy's three categories.
//!
//! `docs/type_system_guide.md` sorts `Default` impls into *fabricates content* (remove),
//! *states nothing and the schema permits it* (keep), and *schema-invalid empty* (remove).
//! The third category was named only after six agents had met it, because nothing in the
//! crate ever constructed a defaulted value and validated it — the round-trip harness only
//! ever sees documents that came from a file. These tests close that gap: they build values
//! from `Default`, serialize them, and hand the result to libxml2.
#![cfg(feature = "validation")]

use openscenario_rs::types::actions::movement::TeleportAction;
use openscenario_rs::types::basic::Value;
use openscenario_rs::types::positions::{Position, WorldPosition};
use openscenario_rs::types::scenario::init::{Private, PrivateAction};
use openscenario_rs::types::scenario::storyboard::{FileHeader, OpenScenario, Storyboard};
use openscenario_rs::validation::XsdValidator;

fn header() -> FileHeader {
    FileHeader {
        author: Value::literal("OSR-09".to_string()),
        date: Value::literal("2024-01-01T00:00:00".to_string()),
        description: Value::literal("Default-policy schema validity".to_string()),
        rev_major: Value::literal(1u16),
        rev_minor: Value::literal(3u16),
        license: None,
        properties: None,
    }
}

fn document_with(storyboard: Storyboard) -> OpenScenario {
    OpenScenario {
        file_header: header(),
        parameter_declarations: None,
        variable_declarations: None,
        monitor_declarations: None,
        catalog_locations: Some(Default::default()),
        road_network: Some(Default::default()),
        entities: Some(Default::default()),
        storyboard: Some(storyboard),
        parameter_value_distribution: None,
        catalog: None,
    }
}

fn validate(document: &OpenScenario) -> Vec<String> {
    let xml = openscenario_rs::parser::xml::serialize_to_string(document)
        .expect("serialization must succeed");
    let mut validator = XsdValidator::from_schema_file("Schema/OpenSCENARIO.xsd")
        .expect("bundled schema must load");
    validator
        .validate_str(&xml)
        .expect("validation must run")
        .into_iter()
        .map(|e| e.message)
        .collect()
}

/// Category 2 — *states nothing, and the schema permits it*.
///
/// This is the evidence behind OSR-09's decision to **keep** `Default` on the four
/// structural containers (`ScenarioDefinition` ×2, `Storyboard`, `Init`). Each of their
/// XSD-required children is a non-`Option` Rust field, so it is always emitted; and every
/// child of `CatalogLocations` (`:867-878`), `RoadNetwork` (`:1933-1940`), `Entities`
/// (`:1122-1127`) and `InitActions` (`:1316-1322`) carries `minOccurs="0"`. The empty
/// spine is therefore a document the schema accepts, not a claim about scenario content.
#[test]
fn defaulted_structural_spine_is_schema_valid() {
    let errors = validate(&document_with(Storyboard::default()));
    assert!(
        errors.is_empty(),
        "a fully defaulted scenario spine must validate; got: {errors:?}"
    );
}

/// Category 3 — *schema-invalid empty*, the reason `TeleportAction`'s derive was removed.
///
/// XSD `Position` (`:1738-1751`) is a bare `xsd:choice` with no `minOccurs="0"`, so a
/// branch must be selected. `Position::empty()` — which is what the removed
/// `TeleportAction::default()` produced — serializes to `<Position />` and is rejected.
/// It invents nothing, and is still unusable. That is the distinction the two-category
/// policy could not express.
#[test]
fn empty_position_choice_is_schema_invalid() {
    let mut storyboard = Storyboard::default();
    storyboard.init.actions.private_actions.push(Private {
        entity_ref: Value::literal("ego".to_string()),
        private_actions: vec![PrivateAction {
            teleport_action: Some(TeleportAction::new(Position::empty())),
            ..Default::default()
        }],
    });
    let errors = validate(&document_with(storyboard));
    assert!(
        errors.iter().any(|e| e.contains("Position")),
        "an all-None Position choice must be rejected by the schema; got: {errors:?}"
    );
}

/// The same action with a branch selected validates — showing the rejection above is about
/// the empty choice, not about `TeleportAction` being unrepresentable.
#[test]
fn teleport_action_with_a_selected_branch_is_schema_valid() {
    let mut storyboard = Storyboard::default();
    storyboard.init.actions.private_actions.push(Private {
        entity_ref: Value::literal("ego".to_string()),
        private_actions: vec![PrivateAction {
            teleport_action: Some(TeleportAction::new(Position::world(WorldPosition::new(
                1.0, 2.0,
            )))),
            ..Default::default()
        }],
    });
    let errors = validate(&document_with(storyboard));
    assert!(
        errors.is_empty(),
        "a teleport to an explicit world position must validate; got: {errors:?}"
    );
}
