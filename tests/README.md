# Integration tests

64 integration test files covering parsing, serialization, cardinality, choice-group wire
format, schema conformance and the builder API. Unit tests live inline in `src/` alongside the
code they exercise; this directory holds the tests that go through the public API.

## Running

```bash
cargo test                                    # everything not feature-gated
cargo test --features builder,validation      # everything
cargo test --test xsd_validation_test         # one file
cargo test roundtrip                          # by name
```

## Layout

Tests are grouped by the area of the crate they exercise. Put a new test next to its
neighbors rather than in a new file.

**Choice groups and cardinality**

| File | Covers |
|---|---|
| `choice_wrappers_roundtrip_test.rs` | Round trip for `#[serde(rename = "$value")]` choice wrappers; a new flattened choice belongs here first. |
| `choice_wrapper_sequences_test.rs` | Sequences nested below a `$value` choice wrapper |
| `choice_wrapper_value_sequences_test.rs` | Sequences below the element wrappers that host an `xsd:choice` |
| `movement_control_choice_test.rs` | Cardinality of the movement and controller choice groups, largest file by test count |
| `appearance_choice_cardinality_test.rs` | Cardinality of the appearance choice groups |
| `action_and_environment_choice_cardinality_test.rs` | `NamedAction` / `EnvironmentAction` choice cardinality |
| `condition_choice_cardinality_test.rs` | Condition and trigger choice groups |
| `position_choice_cardinality_test.rs` | The two position choice groups |
| `document_root_choice_test.rs` | The document root's `OpenScenarioCategory` choice |
| `init_action_choice_cardinality_test.rs` | The three `Init` action choice groups |
| `init_action_choices_test.rs` | `<Storyboard><Init><Actions>` choice-group round trips |
| `story_and_routing_choice_cardinality_test.rs` | The three story/routing `xsd:choice` groups |
| `stochastic_distribution_choice_test.rs` | `StochasticDistribution`'s distribution choice |
| `distribution_definition_element_names_test.rs` | `DistributionDefinition` group element names |
| `storyboard_spine_cardinality_test.rs` | The document's eight repeated main-spine elements (requires `--features builder`) |
| `xsd_cardinality_test.rs` | Cardinality/required-content rules serde alone cannot enforce |
| `leaf_type_cardinality_test.rs` | Leaf types whose XSD `minOccurs` is 2 or 3 |
| `required_vec_rejects_empty_test.rs` | `Vec` fields below required (`minOccurs >= 1`) elements |
| `min_vec_test.rs` | `MinVec<T, MIN>`'s construction side (requires `--features builder`) |

**Serialization and wire form**

| File | Covers |
|---|---|
| `actions_serialization_test.rs` | Action wrapper constructors emit the XSD wire form |
| `entity_conditions_serde_test.rs` | Wire form of `EntityCondition`, the largest condition file |
| `value_conditions_test.rs` | Wire form of every `ByValueCondition` branch |
| `spatial_conditions_test.rs` | Wire form of the spatial conditions |
| `multiple_actions_test.rs` | Documents carrying several actions per event |
| `position_types_test.rs` | The optional `<Orientation>` child on relative positions |
| `properties_consolidation_test.rs` | The shared `Properties` container |
| `property_value_parameter_test.rs` | `Property`'s `@name`/`@value` |
| `enum_wire_names_test.rs` | Wire-name conformance for `src/types/enums.rs` |
| `parameterized_enum_test.rs` | Parameterized enumeration attributes |
| `typed_lexical_space_test.rs` | Lexical spaces of typed `bool`/`dateTime` attributes |
| `datetime_lexical_fidelity_test.rs` | `XsdDateTime` lexical fidelity, `TimeOfDay::date_time`'s type |
| `datetime_value_equality_test.rs` | `XsdDateTime` equality and hashing |

**Schema conformance**

| File | Covers |
|---|---|
| `xsd_validation_test.rs` | Wire-level fixes for two XSD shapes |
| `default_schema_validity_test.rs` | Every XSD-backed `Default` impl, serialized and validated against the schema (requires `--features validation`) |

**Parsing**

| File | Covers |
|---|---|
| `openscenario_integration_test.rs` | Whole-document parsing of the repository's own fixtures, the keeper for this area |
| `sparse_scenario_test.rs` | Schema-optional omissions |
| `steady_state_and_story_action_test.rs` | The XSD `SteadyState` group and full story-level actions |
| `document_parameter_resolution_test.rs` | Resolving a document against its own `<ParameterDeclarations>` |
| `parameter_scope_test.rs` | Parameter visibility per ASAM OpenSCENARIO XML 1.3 §9.1 |
| `expression_evaluator_functions_test.rs` | §9.2 arithmetic and Boolean operators |
| `resolved_parse_error_location_test.rs` | Where a typed-parse error is reported after resolution |
| `xml_parse_error_context_test.rs` | What a caller reads when a file-reading entry point fails |
| `examples_smoke_test.rs` | Runs every example under `examples/` and asserts it exits successfully |

**Entities and catalogs**

| File | Covers |
|---|---|
| `entity_catalog_reference_test.rs` | A `<CatalogReference>` inside a `ScenarioObject` |
| `catalog_entry_name_parameter_test.rs` | `@name` on the nine catalog entry types |
| `catalog_geometry_routing_twins_test.rs` | `Route`/`CatalogRoute`, `Polyline`/`CatalogPolyline`, `Nurbs`/`CatalogNurbs` |
| `catalog_multi_entry_test.rs` | A `<Catalog>` element with more than one child entry |
| `catalog_parameter_assignment_test.rs` | Catalog references assigning values to their entries' parameters, the largest catalog file |
| `catalog_root_agreement_test.rs` | The two types that can parse a catalog document from its root |
| `catalog_system_test.rs` | Catalog files through the crate's file-level entry points |
| `vehicle_components_test.rs` | Vehicle bounding-box geometry behaviour |

**Distributions**

| File | Covers |
|---|---|
| `deterministic_distributions_test.rs` | `Deterministic` single/multi-parameter distribution deserialization |

**Builders** (require `--features builder`; a few also need `validation`)

| File | Covers |
|---|---|
| `scenario_builder_test.rs` | `ScenarioBuilder` parameter support, the keeper for this area |
| `complete_scenario_builder_test.rs` | A complete scenario with actions, and an `Init` with none |
| `storyboard_builder_test.rs` | `StoryboardBuilder` stop-trigger shortcuts and the fluent init-action chain |
| `scenario_declarations_test.rs` | `ScenarioBuilder` parameter/variable declarations |
| `detached_builders_test.rs` | `DetachedActBuilder`/`DetachedManeuverBuilder` without lifetime issues |
| `action_builders_test.rs` | `SpeedActionBuilder`/`TeleportActionBuilder` |
| `catalog_builders_test.rs` | Vehicle/pedestrian catalog reference builders |
| `vehicle_builders_test.rs` | `add_vehicle` filing vehicles under their scenario-object name |
| `pedestrian_builder_test.rs` | `add_pedestrian` filing pedestrians under their scenario-object name |
| `parameter_builders_test.rs` | `ParameterDeclarationsBuilder`/`ParameterizedValueBuilder` |
| `initialization_system_test.rs` | Scenario initialization through the public builder API |

## Fixtures

`tests/data/` holds the scenario files below, plus two directories of fixtures dedicated to a
single test file (`catalog_parameter_assignments/`, used by `catalog_parameter_assignment_test.rs`;
`resolved_parse_error_location/`, used by `resolved_parse_error_location_test.rs`). `NOTICE` in
`tests/data/` records provenance and licensing for the third-party files.

| File | Purpose |
|---|---|
| `simple_scenario.xosc` | A minimal well-formed scenario |
| `minimal_sparse.xosc` | A scenario with most optional content absent |
| `alks_scenario.xosc` | An ALKS scenario from openMSL, MPL 2.0 |
| `cut_in_101_exam.xosc` | A large realistic scenario, 126 KB |
| `expressions_scenario.xosc` | Parameter references and expressions |
| `multiple_actions_scenario.xosc` | Several actions in one storyboard |
| `document_parameter_resolution.xosc` | Parameter resolution across nested scope |
| `parameterized_enums.xosc` / `parameterized_enums_catalog.xosc` | Parameterized enumeration attributes, main document and catalog |

## What these tests do not cover

The fixtures here are a handful of files, and they are not the conformance check. The corpus
that exercises the schema more broadly lives in the `conformance` workspace member and covers
about 53% of the schema's element declarations even so.

Be careful how you read a green round-trip test. serde ignores unknown XML by default, so a
field the Rust types do not model is dropped identically on every pass and the comparison
still succeeds – a passing round trip means *stable*, not *lossless*. See
[docs/xsd_gaps.md](../docs/xsd_gaps.md) for what the gates actually prove and
[CONTRIBUTING.md](../CONTRIBUTING.md) for how to run them.

`src/` line coverage exercised by `tests/` plus the inline `#[cfg(test)]` modules, measured with
`builder` and `validation` instrumented, is 82.46% (6372/7727 lines). The weakest owners sit in
the builder module and the stand-alone validator, not in the wire-format code this directory
otherwise exercises closely:

| File | Covered |
|---|---|
| `src/builder/positions/mod.rs`, `src/builder/storyboard/mod.rs`, `src/builder/actions/mod.rs`, `src/builder/entities/mod.rs` | 0% (re-export modules with no logic of their own) |
| `src/builder/storyboard/maneuver.rs` | 11.8% (59/498) |
| `src/validation.rs` | 29.9% (20/67) |
| `src/builder/actions/controller.rs` | 46.1% (35/76) |
| `src/builder/init/private.rs` | 50.9% (55/108) |
| `src/types/positions/route.rs` | 53.6% (15/28) |

A test added against one of these owners should assert an observable result at its public
boundary (an emitted element/attribute, a parsed field, a specific error) rather than add a
constructor-only probe — see *Tests* in [CONTRIBUTING.md](../CONTRIBUTING.md).
