# Changelog

All notable changes to this project are recorded here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

This release is the result of a four-pass conformance campaign against
`Schema/OpenSCENARIO.xsd`. The Rust types were read against the schema element by element,
and the type model now follows the XSD rather than approximating it. The campaign removed
types the schema does not define, replaced free-form strings with the schema's enumerations,
and completed several choice groups that were only partially modeled. Consequently this
release carries a large number of breaking changes; they are listed individually below.
The conformance ledger, including what the test corpus does and does not prove, is in
[docs/xsd_gaps.md](docs/xsd_gaps.md).

### Added

- **`validation` cargo feature and `XsdValidator`.** `src/validation.rs` exposes libxml-backed
  schema validation: `XsdValidator::{from_schema_file, from_schema_str, validate_str,
  validate_file, validate_document}`, the `ValidationError` diagnostic, `convert_errors`, and
  `classify_error_type`. The schema is parsed once at construction and reused across
  documents. The `xosc-validate` binary now consumes this module and declares
  `required-features = ["validation"]`.
- **Every branch of the `Init` action choices.** The `GlobalAction` choice went from one
  branch to all seven – `EnvironmentAction`, `EntityAction`, `InfrastructureAction`,
  `SetMonitorAction`, `ParameterAction` (deprecated in the schema), `TrafficAction`,
  `VariableAction`. `PrivateAction` went from eight branches to ten, gaining
  `AppearanceAction` and `TrailerAction`. `GlobalAction` gained the `validate()` and
  `get_action_type()` methods it previously lacked.
- **The traffic distribution subtree** and a real `TrafficArea` type.
- **Light and animation appearance actions**, which closed the six orphaned enums
  (`ColorType`, `LightMode`, `VehicleLightType`, `VehicleComponentType`,
  `PedestrianGestureType`, `PedestrianMotionType`).
- **`RoutePosition` and the `InRoutePosition` choice.**
- **The recursive `Trailer` element** on `Vehicle`.
- **Scenario-level `MiscObject` and `ExternalObjectReference`** on `ScenarioObject`.
- **`EntitySelection`**, rebuilt to the schema shape with a required `name` and `Members`.
  `Entities` now carries its selection collection; selections were previously dropped on read.
- The remaining action types, the complete `Action` choice, and the `SteadyState` group.

### Changed

Breaking, unless noted.

- **Nine attributes moved from `OSString`/`String` to the schema's enumerations.**
  `LongitudinalDistanceAction::coordinate_system` and `::displacement`,
  `Timing::domain_absolute_relative`, `CollisionTarget::type`, and `vehicleCategory`,
  `controllerType`, `pedestrianCategory`, `role` and `precipitationType` on the catalog types.
  Code passing `OSString::literal("car")` to any of these no longer compiles; use the enum
  variant.
- **Choice variants are named for their XML elements**, not for their Rust types, and the
  `Rule` wrapper was restored. Variant names on every `#[serde(flatten)]` choice enum must
  match the XSD element name; `tests/choice_flatten_roundtrip_test.rs` pins each site.
- **`AngleType` values corrected** to `heading`, `pitch` and `roll`.
- **`TrajectoryRef` is now a `Trajectory`/`CatalogReference` choice** and is required in
  `TrajectoryPosition`.
- **`TimeReference` is modeled as a choice** of `None` and `Timing`.
- **`Vertex::time` is optional**, per the schema.
- **`ActivateControllerAction` and `ControllerDistribution` moved.** They are now re-exported
  from `types::actions::control` and `types::actions::traffic` respectively.
- **Catalog entity types use the canonical `ParameterDeclarations`** rather than a
  catalog-local copy, and catalog entities resolve to the real scenario types.
- **Type optionality and defaults follow the 1.3 XSD.** Fields the schema marks optional are
  `Option<T>`; no default is invented beyond what the schema defines.
- **Multiple `ObjectController` elements are accepted** on a `ScenarioObject`, per the schema.
- `Actors` may be empty and `ManeuverGroup` may carry no `Maneuver`, both per
  `minOccurs="0"`.

### Removed

- **Five public types with no schema counterpart**: `controllers::ControllerDistribution`,
  `controllers::ActivateControllerAction`, `controllers::ControllerAssignment`,
  `positions::RoadCoordinate` and `positions::LaneCoordinate`. The `controllers` module now
  exports `Controller`, `ControllerProperties` and `ObjectController` only.
- **`Condition` and `ConditionWrapper`**, neither of which the schema defines.
- **Non-schema extension fields** on `ActivateControllerAction` and `SpeedCondition`, and a
  broader sweep of fields and types with no schema counterpart.
- **`Default` impls that invent scenario data.** A default that fabricates a position, a
  speed, or an entity reference produces a document that parses but describes nothing, so
  those impls were removed. Container and choice structs that default to all-`None` remain.
- Divergent duplicate types, folded into their canonical definitions.

### Fixed

- **The builder no longer emits scenario documents missing required elements.**
  `ScenarioBuilder::build()` produced an `OpenScenario` with `CatalogLocations` and
  `RoadNetwork` absent whenever the caller had not set them, which is schema-invalid: the XSD
  group `ScenarioDefinition` declares both without `minOccurs="0"`. Nothing reported it, since
  the output type models every field as `Option` to cover all three document kinds. `build()`
  now returns `BuilderError::MissingField` for either. Pass `CatalogLocations::default()` and
  `RoadNetwork::default()` when a scenario references no catalogs and names no road file; the
  elements are required even though all their children are optional.
  A round-trip harness over 13 builder programs went from 1 passing to 13.
- **Catalog loading reads the real document shape.** Catalogs are parsed as full
  `OpenSCENARIO` documents containing a `Catalog` element, rather than as bare
  `ControllerCatalog`-style roots. Directory loaders now skip files that parse but hold no
  entries of the requested kind, while propagating genuine parse errors instead of swallowing
  them.
- **Attribute and element names that rejected valid XML**, including missing `@`-prefixed
  renames on the entity condition structs and on `ParameterAssignments`.
- Catalog `Maneuver` entries now parse their `Event` sequence; `CatalogMiscObject` gained its
  missing XSD attributes.

### Known gaps

All 37 enumeration simple types in the schema are unions carrying a `parameter` member, so
`vehicleCategory="${cat}"` is schema-valid across 75 attributes. A bare Rust enum cannot
represent that, and the crate does not yet. See
[docs/xsd_gaps.md](docs/xsd_gaps.md) for the full analysis.

## [0.3.2] and earlier

No changelog was kept before this release.
