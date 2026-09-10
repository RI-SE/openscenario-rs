# Changelog

All notable changes to this project are recorded here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

This release is the result of a five-pass conformance campaign against
`Schema/OpenSCENARIO.xsd`. The Rust types were read against the schema element by element,
and the type model now follows the XSD rather than approximating it. The campaign removed
types the schema does not define, replaced free-form strings with the schema's enumerations,
and completed several choice groups that were only partially modeled. Consequently this
release carries a large number of breaking changes; they are listed individually below.
The conformance ledger, including what the test corpus does and does not prove, is in
[docs/xsd_gaps.md](docs/xsd_gaps.md).

### Added

- **`FromStr`/`Display` on all 37 `src/types/enums.rs` enumerations (OSR-05).** Nine enums
  (`TriggeringEntitiesRule`, `Priority`, `StoryboardElementState`, `StoryboardElementType`,
  `ParameterType`, `CoordinateSystem`, `ReferenceContext`, `SpeedTargetValueType`,
  `DynamicsShape`) previously had neither, which blocked wrapping them in `Value<T>`
  (`src/types/basic.rs`) — `Value<T>` serializes through `Display`, not the derived
  `Serialize`, so any variant with a `Display` string that disagreed with its
  `#[serde(rename)]` would have silently produced schema-invalid XML. A new
  table-driven test, `tests/enum_wire_names_test.rs`, derives each enum's expected wire
  name by round-tripping every variant through `serde_json` (never by transcribing the
  `#[serde(rename)]` attribute by hand) and asserts `Display`/`FromStr` agree with it, for
  all 37 enums / 221 variants. Run against the unmodified tree before any code changed,
  covering the 28 enums that already had both impls (181 variants), it found **zero
  mismatches**. The renames, `Display` match, and `FromStr` match were previously three
  independent hand-written transcriptions of the same table; a new `osc_enum!` macro
  (`src/types/enums.rs`) now generates all three plus an `ALL: &[Self]` slice per enum
  from one table, making that drift structurally impossible going forward.
  `src/types/enums.rs` shrank from 1929 to 963 lines. No field changed type and no impl
  was removed — this issue only adds impls and refactors existing ones into the macro; the
  conformance harness (`report`/`lossy`/`validate`) is byte-identical to baseline. Wrapping
  any field in `Value<Enum>` remains out of scope (tracked separately).
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

- **90 enum-typed attributes now accept parameter references (OSR-06).** All 37 enumeration
  `simpleType`s in `Schema/OpenSCENARIO.xsd` are `xsd:union`s whose second member is
  `<xsd:restriction base="parameter"/>`, so `<Vehicle vehicleCategory="$cat">` is
  schema-valid. The crate modelled each of the 75 XSD attributes so declared as a bare Rust
  enum and rejected all of them at parse. Those **90 fields across 25 files** (54 required,
  36 optional) now hold `Value<E>` and `Option<Value<E>>` respectively; the catalog twins
  moved in lockstep with their scenario counterparts. `Value<T>` was extended rather than
  duplicated, so `Resolve<T>` (`src/types/mod.rs`) and
  `CatalogParameterSubstitution::resolve_value` (`src/catalog/parameters.rs`) work on enums
  for free. Per-enum aliases (`VehicleCategoryValue`, `RuleValue`, … one per enum) are in
  `src/types/basic.rs`.

  This breaks direct field access and struct literals: `vehicle.vehicle_category` is now
  `Value<VehicleCategory>`, so `assert_eq!(v.vehicle_category, VehicleCategory::Car)` becomes
  `assert_eq!(v.vehicle_category, Value::Literal(VehicleCategory::Car))`, and
  `matches!(x, Enum::Variant)` becomes a `Value::Literal(...)` comparison. Optional fields
  stay `Option<Value<E>>` — absent and present-but-parameterized are different states and are
  not collapsed.

  **Builder setters are unchanged.** Every setter still takes the bare enum and wraps
  internally, so builder code needs no migration:
  `VehicleBuilder::with_category`/`DetachedVehicleBuilder::with_category`,
  `PedestrianBuilder::with_role`, `SpeedProfileActionBuilder::with_following_mode`,
  `AssignControllerActionBuilder::with_controller`,
  `LaneOffsetActionBuilder::with_simple_dynamics`, `AccelerationConditionBuilder::with_direction`,
  and the `time_rule`/`speed_rule`/`distance_rule` condition setters. Each gained a parallel
  parameter form: `with_category_param`, `with_role_param`, `with_following_mode_param`,
  `with_controller_type_param`, `with_simple_dynamics_param`, `with_direction_param`, and
  `rule_param`, each taking the bare parameter name without the `$`.

  `Value<E>` is not a stringly-typed escape hatch: `vehicleCategory="spaceship"` is still a
  hard parse error, pinned by `tests/parameterized_enum_test.rs`.

- **`Value::Parameter` now serializes as `$name`, not `${name}` (OSR-06).** The schema
  defines `parameter` as `[$][A-Za-z_][A-Za-z0-9_]*` and `expression` as `[$][{]…[\}]`
  (`Schema/OpenSCENARIO.xsd:4-13`). Every scalar union lists both members, so the braced
  spelling validated there and went unchallenged; **all 37 enumeration unions list
  `parameter` alone**, so `vehicleCategory="${cat}"` is schema-invalid. Emitting the braced
  form on the attributes migrated above would have produced invalid XML with a green round
  trip, since `Value<T>` reads and writes through the same string. The unbraced form is valid
  on every union in the schema. `Value::Expression` is unchanged, and deserialization still
  accepts both spellings, so existing documents parse either way — only output changed.
  `src/builder/validation.rs`'s undeclared-parameter rule now scans for both spellings; it
  previously looked only for `${`.

- **`AutomaticGearType` is no longer redeclared in `src/types/actions/control.rs`.** That
  module carried a second copy of the enum with its own `#[serde(rename)]` transcription of
  the same wire names; only the `src/types/enums.rs` copy is generated by `osc_enum!` and
  therefore has the `Display`/`FromStr` that `Value<T>` goes through, and only it is covered
  by `tests/enum_wire_names_test.rs`. The path `types::actions::control::AutomaticGearType`
  now re-exports the canonical enum, so use sites are unaffected; it no longer derives
  `Default` (the canonical enum does not).

- **`Default` is now implemented for `Value<T> where T: Default`,** forwarding to
  `Value::Literal(T::default())`. This preserves the exact default a container had before its
  enum-typed field was wrapped; it states nothing that `T::default()` did not already state.

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
- **`Default` impls that invent scenario data (OSR-03, first tier).** `Default` was
  implemented on types whose defaults invented scenario content the schema does not define —
  a `GeographicPosition` at latitude 0, an `EntityRef` pointing at `"DefaultEntity"`, an
  `Event` containing a fabricated `Priority::Overwrite` action, a `CatalogTimeOfDay`
  timestamped `2021-01-01T12:00:00`. A document built from these parses cleanly and describes
  something nobody wrote. Replaced by explicit constructors: `EntityRef::new`, `Event::new`,
  `Act::new`, `ManeuverGroup::new`, `Maneuver::new`, `Story::new`; `GeographicPosition::new`
  and `CatalogTimeOfDay::new` already existed. This affects construction only — parsing was
  never impacted, since no required field carried `#[serde(default)]`. Container and choice
  structs that default to all-`None` or empty keep their `Default`. This is a first,
  verified-small tier; roughly 110 further fabricating impls remain and are tracked
  separately (see `docs/type_system_guide.md`'s "Default policy" section).
- **`Default` impls that invent scenario data (OSR-04, agent A: `types/actions/movement.rs`,
  `types/positions/route.rs`, `types/routing/mod.rs`).** 21 further fabricating impls
  removed, including the `Route`/`Waypoint`/`RouteRef` carve-out OSR-03 could not complete:
  `RouteRef::default()` silently picked the `Direct` branch of a schema choice, `Route`
  invented the name `"DefaultRoute"`, and `Waypoint` invented `RouteStrategy::Shortest`. None
  of `Route`'s, `Waypoint`'s or `RouteRef`'s XSD attributes carry a `default="…"` — all are
  `use="required"`. `AssignRouteAction` and `RouteRefElement` now use explicit constructors
  instead of `#[derive(Default)]`; `RoutePosition`'s derive is removed for the same reason
  (it holds a `RouteRefElement`). Also removed: `PositionOfCurrentEntity`,
  `PositionInRoadCoordinates`, `PositionInLaneCoordinates` (each invented a coordinate or
  `"DefaultEntity"`), and 15 impls in `movement.rs` — `FollowTrajectoryAction` (fabricated a
  whole child `Trajectory`), `RelativeTargetSpeed`, `Timing`, `TimeReference` (a `None`/
  `Timing` choice; `Default` silently picked `None` — replaced by explicit
  `TimeReference::none()`/`::timing()`), `AbsoluteTargetLane`, `LaneOffsetAction`,
  `LaneOffsetTarget` (another silently-picked choice branch), `RelativeTargetLaneOffset`,
  `AbsoluteTargetLaneOffset`, `LaneOffsetActionDynamics`, `LateralDistanceAction`,
  `SynchronizeAction`, `FinalSpeed`, `AbsoluteSpeed`, `RelativeSpeedToMaster`. Each gained an
  explicit `::new` (or, for choices, named variant constructors); most already had one.
  A further ~13 fabricating impls in `movement.rs` (`TransitionDynamics`,
  `SpeedActionTarget`, `AbsoluteTargetSpeed`, `Trajectory`, `TrajectoryFollowingMode`,
  `TrajectoryRef`, `LaneChangeTarget`, `RelativeTargetLane`, `LateralAction`,
  `LongitudinalAction`, `LongitudinalDistanceAction`, `SpeedProfileAction`,
  `SpeedProfileEntry`) could not be removed within this agent's scope: their `Default` is
  required by call sites in `src/types/scenario/init.rs`,
  `src/types/actions/wrappers.rs`, and shared `tests/*.rs` files outside OSR-04 agent A's
  file list. Each is marked with an `(OSR-04)` comment at its definition. `TeleportAction`
  and `AcquirePositionAction` keep their derived `Default`: both wrap a single `Position`,
  whose own `Default` (out of this agent's scope) is an all-`None` choice with no branch
  selected, so it states nothing rather than fabricating a position.
- Divergent duplicate types, folded into their canonical definitions.

### Fixed

- **The builder's cross-reference rules actually check something.**
  `ParameterReferenceValidationRule` was a no-op whose body was an empty `if` with a comment,
  and it asked the wrong question besides: whether declarations are *used*, which is fine
  either way, rather than whether references *resolve*, which is not.
  `EntityReferenceValidationRule` inspected only `ManeuverGroup.actors`, so an entity named
  anywhere else went unchecked. Both now scan the serialized document, which is complete by
  construction and does not grow a silent gap each time a new action or condition is modelled.
  XSD validation cannot express cross-references, so these rules are the only thing standing
  between a caller and a schema-valid document that names something which does not exist.
  The three shipped templates were committing exactly that mistake and are fixed here.
- **`EntityActionBuilder` can build something.** Its only method returned
  `BuilderResult<PrivateAction>` and unconditionally returned `Err`, and its private enum had
  one variant where the schema's choice has two. `EntityAction` is a *global* action
  (`Schema/OpenSCENARIO.xsd:1128-1134`), so `build()` now returns a `GlobalAction`, and
  `add_entity(position)` joins `delete_entity()`. `VariableActionBuilder` had the same defect
  and gains a real `build()`; its `for_entity` is removed, since a variable action targets a
  variable and the type has no entity reference. The always-erroring `build_action` methods
  are gone from all three global builders.
- **The builder's file header no longer claims OpenSCENARIO 1.0.** `with_header()` hard-coded
  `revMajor`/`revMinor` to 1/0 while the crate targets, validates against, and models 1.3, so
  every document it produced understated its own revision. The default is now 1.3, and
  `ScenarioBuilder::with_revision(major, minor)` overrides it for consumers that need an
  earlier one.
- **`CatalogLocationsBuilder` covers all eight catalog kinds.** It previously offered vehicle,
  pedestrian and controller only; misc-object, environment, maneuver, trajectory and route
  locations could not be set through the builder at all, though `CatalogLocations` has always
  carried the fields.
- **`examples/basic_parsing.rs` runs.** It read a hard-coded path that is not in the repository
  and panicked, with an `.expect()` message naming a different file than the one it opened. A
  new `tests/examples_smoke_test.rs` now executes every example and fails if any exits
  non-zero, plus a cheap guard asserting no example is missing from that list.
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
- **The conformance gates are runnable from a fresh checkout.** They previously lived in an
  untracked sibling directory that CONTRIBUTING.md and docs/xsd_gaps.md described but a clone
  of this repository did not contain. The repo is now a Cargo workspace; `conformance` is a
  member crate holding the corpus binaries (`report`, `lossy`, `validate`, `builder`) that were
  previously external, and `scripts/fetch-corpus.sh` fetches the (still unvendored, MPL-2.0)
  corpus on demand. No change to `openscenario-rs`'s public API.

### Known gaps

None currently tracked. The parameterized-enumeration gap listed here through pass 4 is
closed — see *90 enum-typed attributes now accept parameter references* under **Changed**.
See [docs/xsd_gaps.md](docs/xsd_gaps.md) for the full ledger.

## [0.3.2] and earlier

No changelog was kept before this release.
