# Integration tests

42 integration test files covering parsing, serialization, schema conformance and the builder
API. Unit tests live inline in `src/` alongside the code they exercise; this directory holds
the tests that go through the public API.

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

**Round-trip and serialization**

| File | Covers |
|---|---|
| `choice_flatten_roundtrip_test.rs` | A round trip for **every** `#[serde(flatten)]` choice site, asserting the emitted tag is the XSD element name. A new flattened choice belongs here. |
| `actions_serialization_test.rs` | Action serialization across the action tree |
| `entity_conditions_serde_test.rs` | Entity condition serde, the largest file by test count |
| `multiple_actions_test.rs` | Documents carrying several actions |

**Schema conformance**

| File | Covers |
|---|---|
| `xsd_validation_test.rs` | Choice-group `validate()` and `get_action_type()` across the type tree |
| `xsd_choice_groups_test.rs` | Choice-group structure against the schema |
| `xsd_pedestrian_compliance_test.rs` | Pedestrian conformance |
| `init_action_choices_test.rs` | All seven `GlobalAction` and ten `PrivateAction` branches |

**Parsing**

`comprehensive_parsing_test.rs`, `openscenario_integration_test.rs`,
`scenario_parsing_integration_test.rs`, `sparse_scenario_test.rs`,
`steady_state_and_story_action_test.rs`.

**Entities and catalogs**

`entity_selection_test.rs`, `enhanced_vehicle_components_test.rs`,
`vehicle_components_test.rs`, `vehicle_axles_test.rs`, `catalog_system_test.rs`,
`catalog_builders_test.rs`, `initialization_system_test.rs`.

**Conditions**

`entity_conditions_test.rs`, `entity_conditions_integration_test.rs`,
`spatial_conditions_test.rs`, `motion_conditions_test.rs`, `safety_conditions_test.rs`,
`value_conditions_test.rs`, `condition_builders_test.rs`, `temporal_coordinate_test.rs`.

**Positions and geometry**

`position_types_test.rs`, `advanced_positions_test.rs`, `position_builders_test.rs`,
`spatial_operations_test.rs`.

**Distributions**

`deterministic_distributions_test.rs`.

**Builders** (require `--features builder`)

`scenario_builder_test.rs`, `complete_scenario_builder_test.rs`, `detached_builders_test.rs`,
`action_builders_test.rs`, `vehicle_builders_test.rs`, `pedestrian_builder_test.rs`,
`parameter_builders_test.rs`.

## Fixtures

`tests/data/` holds six scenario files. `NOTICE` in that directory records their provenance
and licensing.

| File | Purpose |
|---|---|
| `simple_scenario.xosc` | A minimal well-formed scenario |
| `minimal_sparse.xosc` | A scenario with most optional content absent |
| `alks_scenario.xosc` | An ALKS scenario from openMSL, MPL 2.0 |
| `cut_in_101_exam.xosc` | A large realistic scenario, 126 KB |
| `expressions_scenario.xosc` | Parameter references and expressions |
| `multiple_actions_scenario.xosc` | Several actions in one storyboard |

## What these tests do not cover

The fixtures here are a handful of files, and they are not the conformance check. The corpus
that exercises the schema more broadly lives in the sibling `../test` harness and covers about
53% of the schema's element declarations even so.

Be careful how you read a green round-trip test. serde ignores unknown XML by default, so a
field the Rust types do not model is dropped identically on every pass and the comparison
still succeeds – a passing round trip means *stable*, not *lossless*. See
[docs/xsd_gaps.md](../docs/xsd_gaps.md) for what the gates actually prove and
[CONTRIBUTING.md](../CONTRIBUTING.md) for how to run them.
