//! The honest `Default` detector.
//!
//! `docs/type_system_guide.md` sorts `Default` impls into *fabricates content* (remove),
//! *states nothing and the schema permits it* (keep), and *schema-invalid empty* (remove).
//! Every detector the `Default` campaign used before this file was a **textual proxy** for
//! that rule: `grep "^impl Default for"` sees hand-written impls but says nothing about
//! validity; `grep 'literal("Default'` sees fabricated names only; and "derive on a struct
//! with a required field" cannot see a struct whose fields are all `Option` but whose XSD
//! particle is a choice that requires a branch (`ObjectController`, `Position`).
//!
//! This file replaces the proxies with the rule itself. For every type in the crate that
//! implements `Default` **and** is backed by an XSD `complexType`, it constructs
//! `T::default()`, serializes it as the root of a document, and validates that document
//! against `Schema/OpenSCENARIO.xsd`.
//!
//! # How a bare type is validated
//!
//! Most XSD types are not root-serializable on their own — the schema declares exactly one
//! global element, `OpenSCENARIO`. Rather than hand-writing a minimal enclosing document per
//! type (which is both laborious and a place for the test to cheat), the detector derives a
//! **probe schema** from the bundled one: it splices a global
//! `<xsd:element name="Probe_T" type="T"/>` declaration into a copy of `OpenSCENARIO.xsd` for
//! every registered type, then serializes `T::default()` under the matching root tag. The
//! content model being checked is the real one, straight out of the shipped schema.
//!
//! # The registry, and what keeps it in sync
//!
//! [`cases`] is a **hand-maintained list** — say it plainly, because an unmaintained list is
//! just a fourth proxy. What keeps it honest is [`every_default_in_src_is_classified`]: it
//! scans `src/` for every `#[derive(..., Default, ...)]` and `impl Default for`, and fails
//! unless each type it finds appears either in this registry or in [`NOT_XSD_BACKED`] with a
//! reason. Adding a `Default` anywhere in `src/` therefore breaks the build until it has been
//! classified. Counts are asserted too, the way `tests/enum_wire_names_test.rs` does for the
//! 37 enums, so a *removal* that silently shrinks coverage is equally loud.
#![cfg(feature = "validation")]

use openscenario_rs::types::actions::movement::TeleportAction;
use openscenario_rs::types::basic::Value;
use openscenario_rs::types::positions::{Position, WorldPosition};
use openscenario_rs::types::scenario::init::{Private, PrivateAction};
use openscenario_rs::types::scenario::storyboard::{FileHeader, OpenScenario, Storyboard};
use openscenario_rs::validation::XsdValidator;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

const SCHEMA_PATH: &str = "Schema/OpenSCENARIO.xsd";

// ---------------------------------------------------------------------------------------
// The detector
// ---------------------------------------------------------------------------------------

/// One registered type: `T::default()`, serialized under its probe root tag.
struct DefaultCase {
    /// Fully-qualified Rust path, e.g. `openscenario_rs::types::positions::Position`.
    rust_path: &'static str,
    /// The XSD `complexType` that backs it. Usually the same identifier as the Rust type;
    /// it differs where the Rust name was disambiguated (`CatalogWeather` → `Weather`).
    xsd_type: &'static str,
    /// Serializes `T::default()` with `<Probe_{xsd_type}>` as the root element.
    serialize: fn() -> Result<String, quick_xml::se::SeError>,
}

impl DefaultCase {
    /// Bare Rust identifier, for matching against the `src/` scan.
    fn rust_name(&self) -> &'static str {
        self.rust_path.rsplit("::").next().unwrap()
    }
}

macro_rules! case {
    ($ty:path, $xsd:literal) => {
        DefaultCase {
            rust_path: stringify!($ty),
            xsd_type: $xsd,
            serialize: || {
                quick_xml::se::to_string_with_root(
                    concat!("Probe_", $xsd),
                    &<$ty as Default>::default(),
                )
            },
        }
    };
}

/// Every type in the crate that implements `Default` and is backed by an XSD `complexType`.
///
/// Hand-maintained; kept in sync by [`every_default_in_src_is_classified`].
fn cases() -> Vec<DefaultCase> {
    vec![
        case!(
            openscenario_rs::types::actions::appearance::PedestrianAnimation,
            "PedestrianAnimation"
        ),
        case!(
            openscenario_rs::types::actions::control::OverrideControllerValueAction,
            "OverrideControllerValueAction"
        ),
        case!(
            openscenario_rs::types::actions::control::ActivateControllerAction,
            "ActivateControllerAction"
        ),
        case!(
            openscenario_rs::types::actions::control::AssignControllerAction,
            "AssignControllerAction"
        ),
        case!(
            openscenario_rs::types::actions::movement::NoneElement,
            "None"
        ),
        case!(
            openscenario_rs::types::actions::movement::DynamicConstraints,
            "DynamicConstraints"
        ),
        case!(
            openscenario_rs::types::actions::traffic::TrafficStopAction,
            "TrafficStopAction"
        ),
        case!(
            openscenario_rs::types::actions::trailer::DisconnectTrailerAction,
            "DisconnectTrailerAction"
        ),
        case!(
            openscenario_rs::types::actions::wrappers::DeleteEntityAction,
            "DeleteEntityAction"
        ),
        case!(
            openscenario_rs::types::actions::wrappers::RandomRouteAction,
            "RandomRouteAction"
        ),
        case!(
            openscenario_rs::types::basic::ParameterDeclarations,
            "ParameterDeclarations"
        ),
        case!(
            openscenario_rs::types::catalogs::controllers::ControllerProperties,
            "Properties"
        ),
        case!(
            openscenario_rs::types::catalogs::environments::CatalogWeather,
            "Weather"
        ),
        case!(
            openscenario_rs::types::catalogs::locations::CatalogLocations,
            "CatalogLocations"
        ),
        case!(
            openscenario_rs::types::catalogs::references::ParameterAssignments,
            "ParameterAssignments"
        ),
        case!(
            openscenario_rs::types::controllers::ControllerProperties,
            "Properties"
        ),
        case!(
            openscenario_rs::types::distributions::deterministic::Deterministic,
            "Deterministic"
        ),
        case!(openscenario_rs::types::entities::Entities, "Entities"),
        case!(
            openscenario_rs::types::entities::vehicle::Properties,
            "Properties"
        ),
        case!(
            openscenario_rs::types::entities::vehicle::CustomContent,
            "CustomContent"
        ),
        case!(
            openscenario_rs::types::environment::weather::Weather,
            "Weather"
        ),
        case!(
            openscenario_rs::types::positions::road::Orientation,
            "Orientation"
        ),
        case!(openscenario_rs::types::road::RoadNetwork, "RoadNetwork"),
        case!(
            openscenario_rs::types::road::TrafficSignals,
            "TrafficSignals"
        ),
        case!(
            openscenario_rs::types::routing::ParameterDeclarations,
            "ParameterDeclarations"
        ),
        case!(openscenario_rs::types::scenario::init::Init, "Init"),
        case!(
            openscenario_rs::types::scenario::init::Actions,
            "InitActions"
        ),
        case!(
            openscenario_rs::types::scenario::monitors::MonitorDeclarations,
            "MonitorDeclarations"
        ),
        case!(
            openscenario_rs::types::scenario::storyboard::Storyboard,
            "Storyboard"
        ),
        case!(
            openscenario_rs::types::scenario::triggers::Trigger,
            "Trigger"
        ),
        case!(
            openscenario_rs::types::scenario::variables::VariableDeclarations,
            "VariableDeclarations"
        ),
    ]
}

/// Types in `src/` that implement `Default` but have no XSD `complexType` behind them, so
/// the schema has nothing to say about their defaults. Each needs a reason.
///
/// The scan in [`every_default_in_src_is_classified`] requires every `Default` in `src/` to
/// be either here or in [`cases`].
const NOT_XSD_BACKED: &[(&str, &str)] = &[
    (
        "ScenarioDefinition",
        "Not an XSD complexType: the Rust struct groups the scenario half of `OpenScenario`'s \
         xsd:choice (:1712-1725). Its own validity is covered by the `OpenScenario` document tests.",
    ),
    (
        "ParameterContext",
        "Runtime parameter-resolution state (`src/types/mod.rs`), never serialized.",
    ),
    (
        "ValidationContext",
        "Non-XSD tooling: strictness flags for `types::mod::Validate`.",
    ),
    (
        "ValidationConfig",
        "Non-XSD tooling: `src/parser/validation.rs` configuration.",
    ),
    (
        "ValidationResult",
        "Non-XSD tooling: `src/parser/validation.rs` diagnostics accumulator.",
    ),
    (
        "ScenarioValidator",
        "Non-XSD tooling: `src/parser/validation.rs` driver.",
    ),
    (
        "ChoiceGroupRegistry",
        "Parser infrastructure (`src/parser/choice_groups.rs`), never serialized.",
    ),
    (
        "CatalogManager",
        "Catalog subsystem state (`src/catalog/`), never serialized.",
    ),
    (
        "CatalogLoader",
        "Catalog subsystem state (`src/catalog/`), never serialized.",
    ),
    (
        "CatalogResolver",
        "Catalog subsystem state (`src/catalog/`), never serialized.",
    ),
    (
        "ParameterSubstitutionEngine",
        "Catalog subsystem state (`src/catalog/`), never serialized.",
    ),
];

/// Builds a copy of the bundled schema carrying one global `Probe_T` element per registered
/// XSD type, so a bare `T::default()` can be validated against the real content model.
fn probe_schema(cases: &[DefaultCase]) -> String {
    let schema = std::fs::read_to_string(SCHEMA_PATH).expect("bundled schema must be readable");
    let open_tag_end = schema
        .find("<xsd:schema")
        .and_then(|start| schema[start..].find('>').map(|offset| start + offset + 1));
    let insert_at = open_tag_end.expect("schema must have an <xsd:schema> element");

    let mut probes = String::new();
    let mut seen = BTreeSet::new();
    for case in cases {
        if seen.insert(case.xsd_type) {
            probes.push_str(&format!(
                "\n\t<xsd:element name=\"Probe_{0}\" type=\"{0}\"/>",
                case.xsd_type
            ));
        }
    }

    let mut out = String::with_capacity(schema.len() + probes.len());
    out.push_str(&schema[..insert_at]);
    out.push_str(&probes);
    out.push_str(&schema[insert_at..]);
    out
}

/// **The detector.** For every registered type: `T::default()` → XML → libxml, against the
/// bundled schema. A failure here means `T::default()` constructs a value that cannot be
/// written as valid OpenSCENARIO — category 1 or category 3 of the `Default` policy.
#[test]
fn every_defaulted_xsd_type_serializes_to_schema_valid_xml() {
    let cases = cases();
    let mut validator = XsdValidator::from_schema_str(&probe_schema(&cases))
        .expect("probe schema must load; a failure here means a Probe_ element is malformed");

    let mut flagged: Vec<String> = Vec::new();
    for case in &cases {
        match (case.serialize)() {
            Err(e) => flagged.push(format!(
                "{} (xsd {}): default value does not serialize at all: {e}",
                case.rust_path, case.xsd_type
            )),
            Ok(xml) => {
                let errors = validator
                    .validate_str(&xml)
                    .expect("serialized default must at least be well-formed XML");
                if !errors.is_empty() {
                    let messages: Vec<_> = errors.iter().map(|e| e.message.as_str()).collect();
                    flagged.push(format!(
                        "{} (xsd {}): {}\n      serialized as: {}",
                        case.rust_path,
                        case.xsd_type,
                        messages.join(" | "),
                        xml
                    ));
                }
            }
        }
    }

    assert!(
        flagged.is_empty(),
        "{} of {} defaulted XSD-backed types serialize to schema-invalid XML:\n  - {}",
        flagged.len(),
        cases.len(),
        flagged.join("\n  - ")
    );
}

// ---------------------------------------------------------------------------------------
// What keeps the hand-maintained registry in sync
// ---------------------------------------------------------------------------------------

/// Number of registry entries. Asserted so that a *removal* is as loud as an addition —
/// the same guard `tests/enum_wire_names_test.rs` puts on the 37 enum tables.
const EXPECTED_CASES: usize = 31;

/// Every `Default` site the source scan finds, as `(type name, file)`.
fn scan_src_for_default_impls() -> BTreeMap<String, BTreeSet<String>> {
    let mut found: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut stack = vec![Path::new("src").to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).expect("src/ must be readable") {
            let path = entry.expect("directory entry must be readable").path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if path.extension().and_then(|e| e.to_str()) != Some("rs") {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("source file must be readable");
            let display = path.to_string_lossy().replace('\\', "/");
            let lines: Vec<&str> = text.lines().collect();
            for (i, line) in lines.iter().enumerate() {
                let trimmed = line.trim_start();
                if let Some(rest) = trimmed.strip_prefix("impl Default for ") {
                    let name: String = rest
                        .chars()
                        .take_while(|c| c.is_alphanumeric() || *c == '_')
                        .collect();
                    if !name.is_empty() {
                        found.entry(name).or_default().insert(display.clone());
                    }
                    continue;
                }
                if !trimmed.starts_with("#[derive(") {
                    continue;
                }
                // A derive list may wrap across lines; join until the parens balance.
                let mut block = String::from(*line);
                let mut end = i;
                while block.matches('(').count() > block.matches(')').count()
                    && end + 1 < lines.len()
                {
                    end += 1;
                    block.push_str(lines[end]);
                }
                if !block
                    .split(|c: char| !c.is_alphanumeric() && c != '_')
                    .any(|token| token == "Default")
                {
                    continue;
                }
                // The declaration is the next `struct`/`enum` line, past any further attributes.
                for line in lines.iter().take((end + 12).min(lines.len())).skip(end + 1) {
                    let decl = line.trim_start().trim_start_matches("pub ");
                    let rest = match decl
                        .strip_prefix("struct ")
                        .or_else(|| decl.strip_prefix("enum "))
                    {
                        Some(rest) => rest,
                        None => continue,
                    };
                    let name: String = rest
                        .chars()
                        .take_while(|c| c.is_alphanumeric() || *c == '_')
                        .collect();
                    if !name.is_empty() {
                        found.entry(name).or_default().insert(display.clone());
                    }
                    break;
                }
            }
        }
    }
    found
}

/// The registry is hand-maintained; this is the mechanism that stops it drifting.
///
/// Every `Default` in `src/` must be accounted for: probed by [`cases`], declared under
/// `src/builder/` (builder scaffolding is never serialized, so the schema has no opinion on
/// it), or listed in [`NOT_XSD_BACKED`] with a reason. A new `Default` fails this test until
/// it is classified — which is the only thing standing between the registry and a fourth
/// textual proxy.
#[test]
fn every_default_in_src_is_classified() {
    let scanned = scan_src_for_default_impls();
    assert!(
        scanned.len() > 75,
        "the source scan found only {} Default sites — it has almost certainly broken",
        scanned.len()
    );

    let cases = cases();
    assert_eq!(
        cases.len(),
        EXPECTED_CASES,
        "registry size changed; update EXPECTED_CASES deliberately, not reflexively"
    );

    let registered: BTreeSet<&str> = cases.iter().map(|c| c.rust_name()).collect();
    let excused: BTreeSet<&str> = NOT_XSD_BACKED.iter().map(|(name, _)| *name).collect();

    let unclassified: Vec<String> = scanned
        .iter()
        .filter(|(name, files)| {
            !registered.contains(name.as_str())
                && !excused.contains(name.as_str())
                && !files.iter().all(|f| f.starts_with("src/builder/"))
        })
        .map(|(name, files)| {
            format!(
                "{name} ({})",
                files.iter().cloned().collect::<Vec<_>>().join(", ")
            )
        })
        .collect();
    assert!(
        unclassified.is_empty(),
        "these types implement Default but are neither probed by `cases()` nor listed in \
         NOT_XSD_BACKED. Add each to whichever is right — do not widen the exclusion rule:\n  - {}",
        unclassified.join("\n  - ")
    );

    // And the reverse: a registry entry for a type that no longer implements `Default`
    // would silently stop testing anything.
    let stale: Vec<&str> = registered
        .iter()
        .copied()
        .filter(|name| !scanned.contains_key(*name))
        .collect();
    assert!(
        stale.is_empty(),
        "registry entries whose type no longer implements Default: {stale:?}"
    );

    let unused: Vec<&str> = excused
        .iter()
        .copied()
        .filter(|name| !scanned.contains_key(*name))
        .collect();
    assert!(
        unused.is_empty(),
        "NOT_XSD_BACKED entries that no longer implement Default: {unused:?}"
    );
}

/// Every registered XSD type name really is a `complexType` in the bundled schema — a typo
/// would otherwise make `probe_schema` silently unloadable or, worse, probe the wrong type.
#[test]
fn every_registered_xsd_type_exists_in_the_schema() {
    let schema = std::fs::read_to_string(SCHEMA_PATH).expect("bundled schema must be readable");
    let missing: Vec<&str> = cases()
        .iter()
        .map(|c| c.xsd_type)
        .filter(|t| !schema.contains(&format!("<xsd:complexType name=\"{t}\">")))
        .collect();
    assert!(
        missing.is_empty(),
        "registered XSD type names absent from {SCHEMA_PATH}: {missing:?}"
    );
}

// ---------------------------------------------------------------------------------------
// Narrative cases: the three categories, in full documents
// ---------------------------------------------------------------------------------------

fn header() -> FileHeader {
    FileHeader {
        author: Value::literal("openscenario-rs".to_string()),
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
    let mut validator =
        XsdValidator::from_schema_file(SCHEMA_PATH).expect("bundled schema must load");
    validator
        .validate_str(&xml)
        .expect("validation must run")
        .into_iter()
        .map(|e| e.message)
        .collect()
}

/// Category 2 — *states nothing, and the schema permits it*.
///
/// This is the evidence behind the decision to **keep** `Default` on the four
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
            ..PrivateAction::empty()
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
            ..PrivateAction::empty()
        }],
    });
    let errors = validate(&document_with(storyboard));
    assert!(
        errors.is_empty(),
        "a teleport to an explicit world position must validate; got: {errors:?}"
    );
}
