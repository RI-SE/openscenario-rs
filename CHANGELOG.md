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
  A further 13 fabricating impls in `movement.rs` (`TransitionDynamics`,
  `SpeedActionTarget`, `AbsoluteTargetSpeed`, `Trajectory`, `TrajectoryFollowingMode`,
  `TrajectoryRef`, `LaneChangeTarget`, `RelativeTargetLane`, `LateralAction`,
  `LongitudinalAction`, `LongitudinalDistanceAction`, `SpeedProfileAction`,
  `SpeedProfileEntry`) could not be removed within agent A's file-disjoint scope: their
  `Default` was required by call sites in `src/types/scenario/init.rs`,
  `src/types/actions/wrappers.rs`, and shared `tests/*.rs` files outside agent A's file list.
  `TeleportAction` and `AcquirePositionAction` keep their derived `Default`: both wrap a
  single `Position`, whose own `Default` (out of this issue's scope) is an all-`None` choice
  with no branch selected, so it states nothing rather than fabricating a position.
- **`Default` impls that invent scenario data (OSR-04, agent A′: the 13 impls agent A could
  not finish).** Working under the revised removal-target partition (agents own *types*, not
  files, and may fix call sites wherever the compiler points), agent A′ removed all 13:
  `TransitionDynamics` (invented `DynamicsDimension::Time`/`DynamicsShape::Linear`/`1.0`),
  `SpeedActionTarget` (silently picked the absolute branch of a schema choice),
  `AbsoluteTargetSpeed` (invented `10.0`), `Trajectory` (invented `"DefaultTrajectory"`),
  `TrajectoryFollowingMode` (invented `FollowingMode::Follow`), `TrajectoryRef` (silently
  picked the direct-`Trajectory` branch, fabricating a whole child trajectory),
  `LaneChangeTarget` (silently picked the relative-lane branch), `RelativeTargetLane`
  (invented `"DefaultEntity"`/`1`), `LateralAction` (silently picked the lane-change branch),
  `LongitudinalAction` (silently picked the speed branch), `LongitudinalDistanceAction`
  (invented `"DefaultEntity"`/`10.0`/`true`/`false`), `SpeedProfileAction` (fabricated a whole
  child `SpeedProfileEntry`), `SpeedProfileEntry` (invented `0.0`/`10.0`). None of the
  underlying XSD attributes carry a `default="…"` — all are `use="required"`. Each gained an
  explicit constructor (`::new`, or named branch constructors for the choice types); the
  `#[derive(Default)]` on `SpeedAction` and `LaneChangeAction`, which depended on these, was
  also removed in favour of explicit `::new`. Call sites fixed in
  `src/types/scenario/init.rs`, `src/types/positions/mod.rs`,
  `src/types/positions/trajectory.rs`, `examples/action_wrappers_demo.rs`,
  `tests/xsd_validation_test.rs`, `tests/advanced_positions_test.rs` and
  `tests/actions_serialization_test.rs`. This closes out all fabricating `Default` impls in
  `types/actions/movement.rs`, `types/positions/route.rs` and `types/routing/mod.rs`.
- **`Default` impls that invent scenario data (OSR-04, agent B: `types/actions/traffic.rs`).**
  All 18 fabricating impls removed, leaving only the benign `TrafficStopAction` (an empty
  XSD complexType, so `Default` states nothing). Removed: `TrafficSourceAction`,
  `TrafficSinkAction`, `TrafficSwarmAction` (each fabricated a whole child `Position` and/or
  `TrafficDefinition`), `TrafficSignalAction` (a choice — silently picked the
  `TrafficSignalStateAction` branch), `TrafficSignalStateAction` (invented
  `"DefaultSignal"`/`"green"`), `TrafficSignalControllerAction` (invented
  `"DefaultController"`/`"Phase1"`), `TrafficSignalController` (invented
  `"DefaultController"`), `Phase` (invented `"DefaultPhase"`/`30.0`), `TrafficSignalState`
  (invented `"signal_1"`/`"green"`), `TrafficSignalGroupState` (invented `"green"`),
  `TrafficDefinition` (invented `"DefaultTrafficDefinition"` and fabricated whole-child
  `VehicleCategoryDistribution`/`ControllerDistribution` defaults),
  `VehicleCategoryDistribution` (fabricated a 70/20/10 car/truck/van split as if it were
  schema-declared), `ControllerDistribution` (fabricated a whole child `Controller` named
  `"DefaultTrafficController"`), `CentralSwarmObject` (invented `"SwarmCenter"`),
  `TrafficArea`/`Polygon` (fabricated a whole-child rectangle), `RoadRange`/`RoadCursor`
  (invented `"DefaultRoad"`), `Lane` (invented id `0`), `TrafficDistribution`/
  `TrafficDistributionEntry` (fabricated a whole-child `EntityDistribution`),
  `DirectionOfTravelDistribution` (invented `1.0`/`0.0`), `TrafficAreaAction` (fabricated
  whole-child distribution and area). None of the underlying XSD attributes carry a
  `default="…"` — all are `use="required"` (checked against `Schema/OpenSCENARIO.xsd`
  `:1063-1066`, `:1333-1335`, `:1714-1723`, `:1926-1954`, `:2214-2334`). Each gained an
  explicit constructor: most already had one (`::new`/`::rectangle`/`::single_controller`
  etc.); new ones added for `RoadCursor::new`, `Lane::new`, `RoadRange::new`,
  `TrafficDistribution::new`, `TrafficDistributionEntry::new`,
  `DirectionOfTravelDistribution::new`, and `TrafficDefinition::new`. The three
  `TrafficDefinition::with_vehicles`/`with_controllers`/`with_both` convenience
  constructors (unused outside this file except `with_both`) collapsed into one `::new`
  that also stopped inventing the `"DefaultTrafficDefinition"` name, since it depended on
  the now-removed whole-child defaults. `InfrastructureAction`
  (`types/actions/wrappers.rs`) lost its `#[derive(Default)]`, which required
  `TrafficSignalAction: Default` and so silently inherited the same choice-branch
  fabrication; it gained an explicit `InfrastructureAction::new`. Call sites fixed in
  `src/types/actions/wrappers.rs`, `tests/actions_serialization_test.rs` and
  `examples/action_wrappers_demo.rs`. This closes out all fabricating `Default` impls in
  `types/actions/traffic.rs`.
- **`Default` impls that invent scenario data (OSR-04, agent C:
  `types/conditions/{entity,value,spatial}.rs`).** All 26 fabricating impls removed; no
  benign `Default` impls existed in these three files to begin with (every retained `Default`
  in scope was already a container/choice struct with a derived, all-`None`/empty impl, e.g.
  `CollisionCondition`, `ByEntityCondition`'s own field defaults). In `entity.rs` (15):
  `SpeedCondition` (invented `10.0`/`GreaterThan`), `AccelerationCondition` (invented `2.0`),
  `StandStillCondition` (invented `1.0`), `CollisionTarget` (invented `ObjectType::Vehicle`),
  `OffroadCondition`/`EndOfRoadCondition` (each invented `1.0`), `TimeHeadwayCondition`
  (invented `"DefaultEntity"`/`2.0`/`LessThan`/`true`), `TimeToCollisionCondition` (invented
  `5.0`/`LessThan`/`true` and depended on the fabricated `TimeToCollisionTarget`),
  `TimeToCollisionTarget` (a choice — silently picked the `EntityRef` branch, inventing
  `"DefaultEntity"`), `AngleCondition`/`RelativeAngleCondition` (each invented
  `AngleType::Heading`/`0.0`/`0.1`, the latter also `"DefaultEntity"`),
  `RelativeSpeedCondition` (invented `"DefaultEntity"`/`GreaterThan`/`5.0`),
  `RelativeLaneRange` (invented `from: -1`/`to: 1` — both are optional XSD attributes with no
  `default="…"`), `RelativeClearanceCondition` (invented a whole-child
  `RelativeLaneRange`, `distanceForward: 50.0`, `distanceBackward: 10.0`, `free_space: true`),
  `TraveledDistanceCondition` (invented `100.0`), and `EntityCondition` (the XSD
  `EntityCondition` choice group — silently picked the `Speed` branch). `ByEntityCondition`
  lost its `#[derive(Default)]`: both fields are XSD-required, and `EntityCondition` is
  itself a choice with no "nothing" state, so no non-fabricating default was possible; it
  gained `ByEntityCondition::new` (the file's existing per-condition convenience
  constructors were kept). In `value.rs` (8): `SimulationTimeCondition`, `ParameterCondition`,
  `TimeOfDayCondition` (invented `chrono::Utc::now()` — a *non-deterministic* fabricated
  value), `StoryboardElementStateCondition`, `UserDefinedValueCondition`,
  `TrafficSignalCondition`, `TrafficSignalControllerCondition`, `VariableCondition` (each
  invented a name/ref/rule/value string), and `ByValueCondition` (the XSD `ByValueCondition`
  choice group — silently picked the `SimulationTimeCondition` branch). `ByValueCondition`
  gained per-branch constructors (`::parameter`, `::time_of_day`, `::simulation_time`,
  `::storyboard_element_state`, `::user_defined_value`, `::traffic_signal`,
  `::traffic_signal_controller`, `::variable`), following `RouteRef::direct`/`::catalog` and
  `SpeedActionTarget::absolute`/`::relative`. In `spatial.rs` (3): `ReachPositionCondition`,
  `DistanceCondition`, `RelativeDistanceCondition` — each already had an explicit
  `::new`/builder constructor from an earlier pass, so only the fabricating `Default` impls
  needed removing. None of the underlying XSD attributes carry a `default="…"` — all are
  `use="required"`, checked individually against `Schema/OpenSCENARIO.xsd`. Two call sites
  outside `conditions/`: `Condition::default()` and `ConditionType::default()`
  (`src/types/scenario/triggers.rs`, owned by OSR-04 agent E) called
  `ByValueCondition::default()`; both now call
  `ByValueCondition::simulation_time(SimulationTimeCondition::new(10.0, Rule::GreaterThan))`
  explicitly — the same fabricated content as before, now named rather than defaulted. Agent
  E's own `Default` impls in `triggers.rs` were left in place. This closes out all
  fabricating `Default` impls in `types/conditions/entity.rs`, `types/conditions/value.rs`
  and `types/conditions/spatial.rs`.
- **`Default` impls that invent scenario data (OSR-04, agent D:
  `types/actions/{wrappers,appearance,control,trailer}.rs`).** All fabricating impls removed:
  19 in `wrappers.rs`, 5 in `appearance.rs`, 6 in `control.rs`, 1 in `trailer.rs` — 31 total,
  plus `StoryGlobalAction`'s dependent `#[derive(Default)]` in `types/scenario/story.rs`.
  Benign container/choice defaults (all-`None`/empty, or already-derived) were left alone: 2
  in `wrappers.rs` (`DeleteEntityAction`, `RandomRouteAction`), 3 in `appearance.rs`
  (`SensorReferenceSet`, `LightType`, `AppearanceAction`; `AnimationType`,
  `ComponentAnimation`, `PedestrianAnimation` were already `#[derive(Default)]`), 2 in
  `control.rs` (`ControllerAction`, `OverrideControllerValueAction`), 2 in `trailer.rs`
  (`TrailerAction`, `DisconnectTrailerAction`).
  In `wrappers.rs`: `Action`/`GlobalAction`/`PrivateAction` (each a bare `xsd:choice` enum,
  XSD:705-712/1282-1293/1777-1786 — `Default` silently picked one branch; the enum variants
  are themselves the constructors, so no replacement method was needed), `EntityAction`
  (invented `"defaultEntity"` and picked the delete branch — gained `::add`/`::delete`),
  `TrafficAction` (picked the stop branch — gained `::new(action)`/`::with_name`),
  `NamedAction` (F13, known-broken for serialization — `Default` removed, no replacement),
  `SetMonitorAction`, `VariableAction`, `VariableSetAction`, `VariableAddValueRule`,
  `VariableMultiplyByValueRule`, `ParameterAction`, `ParameterSetAction`,
  `ParameterAddValueRule`, `ParameterMultiplyByValueRule` (each invented a name/ref/value for
  an XSD `use="required"` attribute — gained `::new`), `VariableModifyAction`/
  `ParameterModifyAction` (fabricated a whole-child `Rule` — gained `::new(rule)`, plus
  `VariableModifyRule::add_value`/`::multiply_by_value` and `ModifyRule::add_value`/
  `::multiply_by_value` for the nested choice), `UserDefinedAction` (fabricated a whole-child
  `CustomCommandAction` — gained `::new`), `CustomCommandAction` (invented `"default"` for
  `@type` — gained `::new`).
  In `appearance.rs`: `VisibilityAction` (invented `graphics`/`sensors`/`traffic` all `true`
  for three XSD `use="required"` attributes, XSD:2550-2557 — gained `::new`),
  `LightStateAction` (fabricated a whole-child `LightType`/`LightState`, XSD:1411-1417 —
  gained `::new`), `LightState` (invented `LightMode::On` for `@mode`, XSD:1402-1409 — gained
  `::new`), `AnimationState` (invented `0.0` for `@state`, XSD:754-756 — gained `::new`),
  `SensorReference` (invented `"DefaultSensor"`, XSD:2019-2021 — gained `::new`).
  In `control.rs`: `ActivateControllerAction` (invented `longitudinal`/`lateral: true`,
  `lighting`/`animation: false` — none of its attributes is `use="required"` or carries a
  `default="…"`, XSD:713-721, so the correct default is all-`None`; the hand-written impl was
  replaced with `#[derive(Default)]`, and the fabricated values now live in the pre-existing
  `all_domains`/`movement_only` constructors), `ManualGear` (invented `1` for `@number`,
  XSD:1471-1473), `AutomaticGear` (silently picked `AutomaticGearType::Drive`, XSD:792-794),
  `Brake` (invented `0.0` for `@value`, XSD:815-818), `BrakeInput`/`Gear` (each a schema
  `xsd:group` choice — silently picked one branch, XSD:819-824/1258-1263) — all five already
  had explicit constructors from an earlier pass (`ManualGear::new`, `AutomaticGear::park`
  etc., `Brake::new`, `BrakeInput::percent`/`::force`, `Gear::manual`/`::automatic`), so only
  the fabricating `Default` impls needed removing.
  In `trailer.rs`: `ConnectTrailerAction` (invented `"DefaultTrailer"` for `@trailerRef`,
  XSD:967-969 — gained `::new`).
  `StoryGlobalAction` (`types/scenario/story.rs`) lost its `#[derive(Default)]`: it required
  `wrappers::GlobalAction: Default`, which fabricated a choice branch; the field it wraps is
  `Option<StoryGlobalAction>`, so no default is needed.
  None of the underlying XSD attributes carries a `default="…"` — all are `use="required"` or
  simply optional with no schema default (checked individually against
  `Schema/OpenSCENARIO.xsd`; running total across OSR-03 and every OSR-04 agent so far: no
  genuine schema default found in any file touched by this series).
  Call sites fixed in `tests/actions_serialization_test.rs` (two tests exercising the
  fabricated defaults rewritten to exercise the new constructors instead — same test count as
  before, 742 lib tests, so no coverage was lost),
  `tests/init_action_choices_test.rs`, and within the four owned files' own test modules.
  This closes out all fabricating `Default` impls in `types/actions/wrappers.rs`,
  `types/actions/appearance.rs`, `types/actions/control.rs` and `types/actions/trailer.rs`.
- **`Default` impls that invent scenario data (OSR-04, agent E:
  `types/entities/selection.rs`, `types/geometry/shapes.rs`, `types/scenario/triggers.rs`,
  `types/basic.rs`, `types/positions/road.rs`, `types/scenario/init.rs`).** 25 fabricating
  impls removed, 0 genuinely benign found (every attribute checked against
  `Schema/OpenSCENARIO.xsd` carries neither a schema default nor an optional/`Vec`
  representation that would make invented content unnecessary), plus 6 manual impls that
  fabricated content converted to the crate's existing all-`None`/`Vec::new()` derived-default
  pattern for container/choice types (kept, not counted as removed).
  In `entities/selection.rs`: `EntitySelection` (invented `"DefaultSelection"` for
  `@name`, XSD:1180-1185 — gained no new constructor, `::new` already existed),
  `EntityDistributionEntry` (invented `weight: 1.0` and a whole-child
  `ScenarioObjectTemplate`, XSD:1162-1167 — `::new` already existed),
  `ScenarioObjectTemplate` (fabricated a whole-child `Vehicle::default()`, XSD:2007-2012 —
  the `new_vehicle`/`new_pedestrian`/`new_misc_object`/`with_external_reference`
  constructors already existed), `ExternalObjectReference` (invented `"DefaultObject"` for
  `@name` — `::new` already existed), `ByObjectType`/`ByType` (each silently picked the
  `Vehicle` branch of their respective `ObjectType` attribute — `::new`/`::vehicle` already
  existed). `EntityDistribution`'s fabricating impl (invented a whole-child
  `EntityDistributionEntry`, XSD:1157-1161) was replaced with `#[derive(Default)]`: its
  field is `Vec<EntityDistributionEntry>` with no `minOccurs="0"` in the schema, so an empty
  `Vec` is not schema-valid content on its own, but — consistent with `ConditionGroup` below —
  it states nothing invented, and keeping it (rather than deleting outright) avoids a
  clippy `new_without_default` warning against the type's pre-existing bare `::new()`.
  In `geometry/shapes.rs`: `Center`/`Dimensions` (each invented coordinates — `0.0`/`0.0`/`0.0`
  and a "default car" `2.0`/`4.5`/`1.5` — for `xsd:all` groups of `use="required"` attributes,
  XSD:886-890/1058-1062; `Center` gained `::new`, `Dimensions::new` already existed) and the
  `#[derive(Default)]` on `BoundingBox` that depended on them (removed; `BoundingBox` gained
  `::new(center, dimensions)`, XSD:809-814, no schema default on either child). `Vertex`
  (fabricated a whole-child `Position::default()` for a required field, no benign
  replacement — gained `::new(position)`/`::with_time`). `Shape`'s and `Polyline`'s
  fabricating impls (the former invented `polyline: Some(Polyline::default())` instead of
  the schema-neutral all-`None` choice state; the latter invented `vec![Vertex::default()]`
  instead of the schema-neutral empty `Vec`, despite `Vertex` having `minOccurs="2"`,
  XSD:1733-1737) were both replaced with `#[derive(Default)]`, matching the pattern already
  used for `Position` elsewhere in the crate.
  In `scenario/triggers.rs`: `Condition`/`ConditionType` — flagged explicitly by OSR-04 agent
  C as inherited fabrication — invented a whole-child `ByValueCondition` (via
  `ByValueCondition::simulation_time(...)`, the branch C's fix named but did not remove) and,
  for `ConditionType`, additionally picked the `ByValue` branch of the `Condition` choice
  group, XSD:953-961; both removed with no replacement (`Condition::new` already existed;
  `ConditionType`'s variants are constructed directly). `TriggeringEntities` (invented
  `TriggeringEntitiesRule::Any` for a `use="required"` attribute, XSD:2400-2405 — `::new`/
  `::any`/`::all` already existed). `Trigger`'s and `ConditionGroup`'s fabricating impls
  (the former invented a whole-child `ConditionGroup::default()`; the latter a whole-child
  `Condition::default()`) were both replaced with `#[derive(Default)]`: `Trigger`'s
  `ConditionGroup` is `minOccurs="0"` (XSD:2395-2399, genuinely benign empty `Vec`);
  `ConditionGroup`'s `Condition` has no `minOccurs="0"` (XSD:962-966), so the empty `Vec` is
  not schema-valid alone, but states nothing invented, unlike the impl it replaced.
  In `types/basic.rs`: `ParameterDeclaration` (invented `"DefaultParameter"`/
  `ParameterType::String`/`""`, XSD:1634-1641), `ValueConstraint` (invented
  `Rule::EqualTo`/`"0"`, XSD:2442-2445), `Range` (invented `lowerLimit: 0.0`/
  `upperLimit: 100.0`, XSD:1815-1818), `Directory` (invented an empty `@path`, XSD:1067-1069)
  — all four `use="required"` attributes with no schema default; all four already had
  `::new`-style constructors. `Value<T>`'s serde impls were not touched (F15).
  In `positions/road.rs`: `RelativeRoadPosition`/`RelativeLanePosition` (each invented
  `"DefaultEntity"` plus zeroed deltas for `use="required"` attributes — `::new` already
  existed for both).
  In `scenario/init.rs` (previously unassigned — moved here by the 2026-09-10 grouping
  revision): `Private` (invented `"DefaultEntity"` for `@entityRef`, XSD:1771-1776 — `::new`
  already existed). `LongitudinalAction`'s fabricating impl (invented a whole-child
  `SpeedAction`) was replaced with `#[derive(Default)]`, matching the crate's existing
  treatment of the sibling `PrivateAction`/`GlobalAction` choice groups in the same file —
  `LongitudinalAction` is a bare `xsd:choice` (XSD:1431-1437) modelled as parallel `Option`s,
  and all-`None` states nothing about which branch was chosen.
  Two collateral `#[derive(Default)]` removals in files outside this issue's list, both
  reported per the "say so explicitly" rule: `ControllerCatalogLocation`
  (`src/types/controllers/mod.rs`) required `Directory: Default` and had no constructor or
  call site of its own — it appears to be an unused duplicate of
  `catalogs::locations::ControllerCatalogLocation`, which already has no `Default`. Several
  call sites elsewhere that referenced the removed `Center`/`BoundingBox`/`Range`/`Directory`
  defaults (entity constructors in `types/entities/{vehicle,pedestrian,misc_object}.rs`,
  builder finish/`with_dimensions` methods in `src/builder/entities/{vehicle,pedestrian}.rs`,
  test fixtures, and three example files) were rewritten to state an explicit value rather
  than softened with `unwrap_or_default()`.
  None of the underlying XSD attributes carries a `default="…"` — all are `use="required"` or
  optional with no schema default (checked individually against `Schema/OpenSCENARIO.xsd`;
  running total across OSR-03 and every OSR-04 agent so far: still no genuine schema default
  found in any file touched by this series). `cargo test --features builder,validation`: 742
  lib tests (same count as baseline — no coverage lost), and the conformance harness
  (`report`/`lossy`/`validate`/`builder`) is unchanged from baseline (172/172/172/13, 0
  dropped/invented). `grep -rn 'literal("Default' src/ | wc -l` went from 16 to 9.
  This closes out all fabricating `Default` impls in `types/entities/selection.rs`,
  `types/geometry/shapes.rs`, `types/scenario/triggers.rs`, `types/basic.rs`,
  `types/positions/road.rs` and `types/scenario/init.rs`.
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
