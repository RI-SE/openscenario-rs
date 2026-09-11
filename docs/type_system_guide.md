# Type system guide

This document describes how `openscenario-rs` maps the OpenSCENARIO 1.3 XSD onto Rust types:
the `Value<T>` wrapper that makes every attribute parameterizable, the serde conventions that
distinguish XML attributes from child elements, the two idioms used for schema choice groups,
and the deliberate policies on optionality and `Default`.

It is written for contributors adding or correcting types. For the conformance ledger, which
records what the corpus proves and which gaps remain open, see [xsd_gaps.md](xsd_gaps.md).

## The governing principle

The schema is the specification, and the Rust types follow it rather than approximating it.
Three consequences follow, and they explain most of what looks unusual in `src/types/`:

- A field the schema marks optional is `Option<T>` in Rust. A field it marks required is not.
- No default is invented beyond what the schema defines.
- A type that has no counterpart in the XSD does not belong in the crate, however convenient
  it might be.

## `Value<T>`: literals, parameters, and expressions

Almost every attribute in OpenSCENARIO can carry a literal value, a `${parameter}` reference,
or a `${expression}` in place of the value. The type system models this uniformly rather than
per-attribute (`src/types/basic.rs:33`):

```rust
pub enum Value<T> {
    Literal(T),
    Parameter(String),
    Expression(String),
}
```

The generic parameter is the resolved type, so a speed is a `Value<f64>` whether the file
carries `30.0` or `${initialSpeed}`. The named aliases (`src/types/basic.rs:219`) are what you
will actually see in the type definitions:

| Alias | Expands to | XSD type |
|---|---|---|
| `OSString` | `Value<String>` | `String` |
| `Double` | `Value<f64>` | `Double` |
| `Int` | `Value<i32>` | `Int` |
| `UnsignedInt` | `Value<u32>` | `UnsignedInt` |
| `UnsignedShort` | `Value<u16>` | `UnsignedShort` |
| `Boolean` | `Value<bool>` | `Boolean` |
| `DateTime` | `Value<chrono::DateTime<Utc>>` | `DateTime` |

### Constructing and inspecting

The constructors take an owned value, so a parameter name is a `String` and, importantly, it
is the **bare** name without the `${…}` wrapper – the braces are wire syntax, not part of the
name:

```rust
use openscenario_rs::types::basic::{Double, OSString};

let literal = Double::literal(27.8);
let from_param = Double::parameter("initialSpeed".to_string());
let computed = Double::expression("initialSpeed * 1.1".to_string());
let name = OSString::literal("ego".to_string());
```

Inspection is by the three `as_*` accessors, each returning `None` for the other two variants
(`src/types/basic.rs:82`). There is no `is_parameter()` predicate; match on the `Option`
instead:

```rust
if let Some(speed) = literal.as_literal() {
    println!("literal speed: {speed}");
}
if let Some(param) = from_param.as_parameter() {
    println!("depends on parameter: {param}");
}
```

Resolution takes a plain parameter map and requires `T: FromStr + Clone`:

```rust
use std::collections::HashMap;

let mut params = HashMap::new();
params.insert("initialSpeed".to_string(), "27.8".to_string());

let resolved: f64 = from_param.resolve(&params)?;
```

`resolve` returns `Error::ParameterError` when the name is absent from the map or when the
substituted text does not parse into `T`.

### Serde behavior worth knowing

`Value<T>` carries hand-written `Deserialize` and `Serialize` impls
(`src/types/basic.rs:132` and `:188`) rather than derived ones, and their behavior is not
obvious from the type alone:

- **Deserialization reads a string first**, then classifies it. `${name}` becomes `Parameter`
  when the inner text is a valid parameter name containing none of `+-*/%()`; otherwise it
  becomes `Expression`. A bare `$name` is also accepted as a parameter reference, falling back
  to a literal parse when the name is not valid. Anything else goes through `str::parse::<T>()`
  and is an error on failure.
- **An empty string targeting an `f64` is an error**, not a zero. An empty attribute in a
  source file is a defect, and silently reading it as `0.0` would fabricate a value the file
  never stated.
- **Serialization collapses `Parameter` and `Expression` back to `${…}`.** Both round-trip to
  the same wire syntax, which is correct: the distinction is a parsing convenience, not
  something the schema expresses. `Display` follows the same rules.

## Attributes and elements

XSD attributes and child elements are distinguished by the serde rename, following quick-xml's
convention. An attribute takes an `@` prefix; an element does not:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioObject {
    #[serde(rename = "@name")]
    pub name: OSString,                                  // XML attribute

    #[serde(rename = "Vehicle", skip_serializing_if = "Option::is_none")]
    pub vehicle: Option<Vehicle>,                        // optional child element

    #[serde(rename = "ObjectController", default, skip_serializing_if = "Vec::is_empty")]
    pub object_controller: Vec<ObjectController>,        // repeated child element
}
```

A missing `@` is the single most common cause of a struct that compiles, parses without
complaint, and silently drops every attribute it was supposed to read. The three-line pattern
above is the whole convention:

- schema-optional → `Option<T>` with `skip_serializing_if = "Option::is_none"`;
- repeated (`maxOccurs` above one) → `Vec<T>` with `default`, usually plus
  `skip_serializing_if = "Vec::is_empty"`;
- required → the bare type, with no `default`.

The `skip_serializing_if` attributes are not cosmetic. Without them, an absent optional field
serializes as an empty element or attribute, and the result is schema-invalid output – the
crate emits something the file never contained.

## Choice groups

An XSD `choice` admits exactly one of several branches. Two idioms coexist in the crate, and
which one applies depends on how the choice appears in the schema.

### Parallel `Option` fields

Stated as the crate convention at `src/types/scenario/init.rs:51`. Every branch is an
`Option` field, and a hand-written `validate()` enforces the exactly-one rule that the type
system cannot:

```rust
pub struct GlobalAction {
    #[serde(rename = "EnvironmentAction", default, skip_serializing_if = "Option::is_none")]
    pub environment_action: Option<EnvironmentAction>,
    #[serde(rename = "EntityAction", default, skip_serializing_if = "Option::is_none")]
    pub entity_action: Option<EntityAction>,
    // ... five further branches
}
```

Such types carry two companion methods by convention: `validate() -> Result<(), String>`,
which rejects zero or several populated branches, and `get_action_type() -> Option<&str>`,
which names the populated one.

### `#[serde(flatten)]` over an externally tagged enum

Used where the choice is the entire content of an element, in
`types/actions/{movement,control,traffic,wrappers}.rs`, `types/routing` and elsewhere.
Here one rule governs everything:

> **The enum variant name must equal the XSD element name.**

Not the Rust type name of the payload, and not the schema's `complexType` name. XSD type
names and XML element names are different namespaces, and they diverge often –
`TransitionDynamics` appears on the wire as `<SpeedActionDynamics>`. Naming a variant after
the type produces a struct that compiles and emits an element the schema has never heard of.

`tests/choice_flatten_roundtrip_test.rs` pins a round trip for every such site and asserts the
emitted tag. A new flattened choice belongs in that test.

## Enumerations

All 37 XSD enumeration simple types have a same-named Rust enum in `src/types/enums.rs`. They
are plain unit enums, with each variant renamed to its schema value:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rule {
    #[serde(rename = "greaterThan")]
    GreaterThan,
    #[serde(rename = "lessThan")]
    LessThan,
    #[serde(rename = "equalTo")]
    EqualTo,
    // ...
}
```

Variant-level drift against the schema is common and cheap to check; method 5 in
[xsd_gaps.md](xsd_gaps.md) describes the sweep. As of pass 4 all 37 match exactly in both
directions.

Each of those 37 enumerations is an `xsd:union` carrying a `parameter` member, which makes
`vehicleCategory="$cat"` schema-valid across 75 attributes. A bare Rust enum cannot represent
that, so every enum-typed attribute is wrapped: **90 fields across 25 files** hold `Value<E>`
(54 required) or `Option<Value<E>>` (36 optional — absent and present-but-parameterized are
different states and are not collapsed).

```rust
#[serde(rename = "@vehicleCategory")]
pub vehicle_category: Value<VehicleCategory>,     // required

#[serde(rename = "@role", default, skip_serializing_if = "Option::is_none")]
pub role: Option<Value<Role>>,                    // optional
```

This is not a stringly-typed escape hatch. `Value`'s `Deserialize` falls through to
`s.parse::<T>()` for anything without a `$` sigil, so `vehicleCategory="spaceship"` is still a
hard parse error.

**Mind the sigil.** The schema defines two productions (`Schema/OpenSCENARIO.xsd:4-13`):
`parameter` is `[$][A-Za-z_][A-Za-z0-9_]*` — unbraced — and `expression` is `[$][{]…[\}]`.
Every scalar union (`Double`, `Int`, `Boolean`, …) lists both members, so both spellings
validate there; **all 37 enumeration unions list `parameter` alone**. `vehicleCategory="${cat}"`
is therefore schema-invalid. `Value::Parameter` serializes as `$name` accordingly — valid on
every union in the schema — while `Value::Expression` keeps `${…}`. Deserialization accepts
either spelling, so documents written the other way still parse.

Builder setters keep taking the bare enum and wrap internally, so the common case needs no
migration; each has a parallel `*_param(&str)` for the parameter form:

```rust
vehicle.with_category(VehicleCategory::Truck)   // vehicleCategory="truck"
vehicle.with_category_param("cat")              // vehicleCategory="$cat"
```

The analysis is in [xsd_gaps.md](xsd_gaps.md).

## The `Default` policy

`Default` sorts into **three** categories, not two. The first two were written down from the
start; the third was discovered six times in the field before it was named here, and once it was
*created* by a cleanup pass that was removing instances of it.

**1. Fabricates content — remove.** A `TeleportAction` whose `Default` invents a world position
at the origin produces a document that parses cleanly and describes something nobody wrote, which
is worse than a compile error. If a type needs every field to say anything at all, it should
require every field. Give it an explicit `::new` (or per-branch constructors, for a choice) and
let the compiler point at every call site that was relying on the invented value.

**2. States nothing, and the schema permits that — keep.** A container or choice struct whose
`Default` is all-`None` or an empty `Vec`, *where the schema allows the empty form to appear*.
`ParameterDeclarations` is the model case: its only child is `minOccurs="0"`
(`Schema/OpenSCENARIO.xsd:1642-1646`), so `ParameterDeclarations::default()` serializes to XML
that validates. Nothing is invented and nothing is claimed.

**3. Schema-invalid empty — do not keep silently.** An empty `Vec` or an all-`None` choice invents
nothing, but that does not make it benign. It is only benign when the schema permits the empty
form. When it does not, `T::default()` constructs a value that *cannot* be serialized into
schema-valid XML — a different defect from category 1, and one no round-trip test will catch,
because nothing in the corpus constructs such a value and serializes it.

```xml
<xsd:complexType name="ConditionGroup">
  <xsd:sequence><xsd:element name="Condition" type="Condition" maxOccurs="unbounded"/></xsd:sequence>
</xsd:complexType>          <!-- no minOccurs ⇒ minOccurs=1: at least one Condition required -->

<xsd:complexType name="Polyline">
  <xsd:sequence><xsd:element name="Vertex" type="Vertex" minOccurs="2" maxOccurs="unbounded"/></xsd:sequence>
</xsd:complexType>          <!-- at least TWO vertices required -->
```

**The check to run, before deriving or keeping a `Default` on any container or choice type:**

1. Open `Schema/OpenSCENARIO.xsd` and find the type's declaration.
2. For each child element, read its `minOccurs`. **An absent `minOccurs` means `1`, not `0`** —
   that is the trap, and it is how every instance of this category got in.
3. Empty is schema-valid only if *every* child is `minOccurs="0"`. If any child is required, the
   type belongs in category 3: no `Default` — give it a constructor that takes the required
   children.
4. For an `xsd:choice`, the same rule applies to the choice particle itself. A choice without
   `minOccurs="0"` must select a branch, so an all-`None` value is invalid however many `Option`
   fields it has. `PrivateAction` (`Schema/OpenSCENARIO.xsd:1777-1791`) is a bare `xsd:choice`
   with no `minOccurs="0"` override, so a Rust struct mirroring it gets per-branch constructors,
   not a derived `Default`.
5. `#[derive(Default)]` counts. A derive on a struct with a required non-`Option`, non-`Vec` field
   fabricates that field's own `Default` silently — see the third detector below.

"It states nothing" is a claim about the schema, not about the Rust struct. Verify it against the
schema or do not make it.

### Detecting violations

Three detectors, none of which subsumes the others. Run all three:

| # | detector | finds |
|---|---|---|
| 1 | `grep -rn "^impl Default for" src/` | hand-written impls |
| 2 | `grep -rn 'literal("Default' src/` | fabricated name strings (`"DefaultVehicle"`, …) |
| 3 | `#[derive(Default)]` on a struct with a required (non-`Option`, non-`Vec`) field | derives that fabricate a field silently — **invisible to 1 and 2** |

Detector 3 is not a grep; it needs the struct body. It was added late, after ten agents had run
only the two textual ones, and it immediately found a class neither could see — including one
instance the campaign itself had just introduced. The lesson generalizes: before calling any sweep
complete, ask what shape the detector cannot represent.

### Enforcement history

**The policy is stated and not yet fully enforced.** A sweep found roughly 124 hand-written
`Default` impls across the crate that fabricate content (a name, a coordinate, a whole nested
action) rather than stating nothing. OSR-03 removed a first, verified-small tier — twelve
call sites across `GeographicPosition`, `EntityRef`, `Story`/`Act`/`ManeuverGroup`/`Maneuver`,
`Event`, and `CatalogTimeOfDay` — replacing each with an explicit `::new`/constructor that
requires the caller to say what they mean instead of inheriting an invented value. `Route`,
`Waypoint` and `RouteRef` were scoped for the same pass but left in place: removing
`RouteRef::default()` broke `#[derive(Default)]` on `AssignRouteAction`
(`src/types/actions/movement.rs`) and `RouteRefElement` (`src/types/positions/route.rs`),
both outside OSR-03's file scope.

OSR-04 (agent A, `src/types/actions/movement.rs`, `src/types/positions/route.rs`,
`src/types/routing/mod.rs`) resolved that carve-out and removed 21 further fabricating impls
in those three files (`Route`, `Waypoint`, `RouteRef`; `PositionOfCurrentEntity`,
`PositionInRoadCoordinates`, `PositionInLaneCoordinates`; and 15 in `movement.rs` including
`FollowTrajectoryAction`, `SynchronizeAction`, `FinalSpeed`/`AbsoluteSpeed`/
`RelativeSpeedToMaster`, and the `LaneOffset*` family), giving each an explicit constructor.
A further 13 fabricating impls in `movement.rs` (`TransitionDynamics`, `SpeedActionTarget`,
`AbsoluteTargetSpeed`, `Trajectory`, `TrajectoryFollowingMode`, `TrajectoryRef`,
`LaneChangeTarget`, `RelativeTargetLane`, `LateralAction`, `LongitudinalAction`,
`LongitudinalDistanceAction`, `SpeedProfileAction`, `SpeedProfileEntry`) could not be removed
within OSR-04 agent A's file-disjoint scope: their `Default`/`#[derive(Default)]` was required
by call sites in `src/types/scenario/init.rs`, `src/types/actions/wrappers.rs`, and shared
top-level `tests/*.rs` files that no OSR-04 agent owned under the original partition.

OSR-04 agent A′, working under the revised removal-target partition (agents own *types*, not
files, and may edit whatever the compiler points them at to fix call sites), removed all 13.
Each now has an explicit constructor instead: `TransitionDynamics::new`,
`SpeedActionTarget::absolute`/`::relative`, `AbsoluteTargetSpeed::new`, `Trajectory::new`,
`TrajectoryFollowingMode::new`, `TrajectoryRef::with_trajectory`/`::with_catalog_reference`/
`::from_catalog`, `LaneChangeTarget::relative`/`::absolute`, `RelativeTargetLane::new`,
`LateralAction::lane_change`/`::lane_offset`/`::lateral_distance`,
`LongitudinalAction::speed`/`::longitudinal_distance`/`::speed_profile`,
`LongitudinalDistanceAction::new`, `SpeedProfileAction::new`, and `SpeedProfileEntry::new`. The
`#[derive(Default)]` on `SpeedAction` and `LaneChangeAction` — which depended on these — was
also removed, each gaining an explicit `::new`. Call sites were fixed in
`src/types/scenario/init.rs`, `src/types/positions/mod.rs`, `src/types/positions/trajectory.rs`,
`examples/action_wrappers_demo.rs`, and `tests/xsd_validation_test.rs`,
`tests/advanced_positions_test.rs`, `tests/actions_serialization_test.rs`.

OSR-04 agent B (`src/types/actions/traffic.rs`) removed all 18 fabricating impls in that file,
leaving only the benign `TrafficStopAction` (an empty XSD complexType). Removed:
`TrafficSourceAction`, `TrafficSinkAction`, `TrafficSwarmAction` (each fabricated a whole-child
`Position`/`TrafficDefinition`), `TrafficSignalAction` (a choice — silently picked the
`TrafficSignalStateAction` branch), `TrafficSignalStateAction`, `TrafficSignalControllerAction`,
`TrafficSignalController`, `Phase`, `TrafficSignalState`, `TrafficSignalGroupState` (each
invented a name/id/state string), `TrafficDefinition`, `VehicleCategoryDistribution`,
`ControllerDistribution` (each fabricated a whole-child distribution or nested `Controller`),
`CentralSwarmObject`, `TrafficArea`/`Polygon` (a whole-child rectangle), `RoadRange`/
`RoadCursor`, `Lane`, `TrafficDistribution`/`TrafficDistributionEntry`,
`DirectionOfTravelDistribution`, `TrafficAreaAction`. Each gained an explicit constructor
(`::new` or a named-branch constructor for the choice); `InfrastructureAction`
(`src/types/actions/wrappers.rs`) lost its `#[derive(Default)]`, which had transitively
required the removed `TrafficSignalAction: Default`, and gained `InfrastructureAction::new`.

OSR-04 agent C (`src/types/conditions/{entity,value,spatial}.rs`) removed all 26 fabricating
impls across those three files. In `entity.rs`: `SpeedCondition`, `AccelerationCondition`,
`StandStillCondition`, `CollisionTarget`, `OffroadCondition`, `EndOfRoadCondition`,
`TimeHeadwayCondition`, `TimeToCollisionCondition`, `TimeToCollisionTarget` (a choice —
silently picked the `EntityRef` branch), `AngleCondition`, `RelativeSpeedCondition`,
`RelativeLaneRange`, `RelativeClearanceCondition`, `RelativeAngleCondition`,
`TraveledDistanceCondition`, and `EntityCondition` (the `EntityCondition` XSD choice group —
silently picked the `Speed` branch). `ByEntityCondition` lost its `#[derive(Default)]`, since
both of its fields are XSD-required and one (`EntityCondition`) is itself a choice with no
"nothing" state; it gained `ByEntityCondition::new` plus the pre-existing named condition
constructors. In `value.rs`: `SimulationTimeCondition`, `ParameterCondition`,
`TimeOfDayCondition`, `StoryboardElementStateCondition`, `UserDefinedValueCondition`,
`TrafficSignalCondition`, `TrafficSignalControllerCondition`, `VariableCondition`, and
`ByValueCondition` (the `ByValueCondition` XSD choice group — silently picked the
`SimulationTimeCondition` branch). `ByValueCondition` gained per-branch constructors
(`::parameter`, `::time_of_day`, `::simulation_time`, `::storyboard_element_state`,
`::user_defined_value`, `::traffic_signal`, `::traffic_signal_controller`, `::variable`),
following `RouteRef::direct`/`::catalog` and `SpeedActionTarget::absolute`/`::relative`. In
`spatial.rs`: `ReachPositionCondition`, `DistanceCondition`, `RelativeDistanceCondition` — each
already had an explicit `::new`/builder constructor, so only the fabricating `Default` impls
were removed. Two call sites outside agent C's files needed fixing: `Condition::default()` and
`ConditionType::default()` in `src/types/scenario/triggers.rs` (owned by OSR-04 agent E) called
`ByValueCondition::default()`; both were updated to call
`ByValueCondition::simulation_time(SimulationTimeCondition::new(10.0, Rule::GreaterThan))`
explicitly instead — the same fabricated `SimulationTimeCondition` content as before, now named
rather than defaulted. Agent E's own `Default` impls in that file were left untouched.

OSR-04 agent D (`src/types/actions/{wrappers,appearance,control,trailer}.rs`) removed all 31
fabricating impls across those four files. In `wrappers.rs` (19): `Action`, `GlobalAction`,
`PrivateAction` (each a bare `xsd:choice` enum — `Default` silently picked one branch; the enum
variants are themselves the constructors, no replacement method needed), `EntityAction`
(gained `::add`/`::delete`), `TrafficAction` (gained `::new`/`::with_name`), `NamedAction`
(F13, known-broken for serialization — removed, no replacement), `SetMonitorAction`,
`VariableAction`, `VariableSetAction`, `VariableAddValueRule`, `VariableMultiplyByValueRule`,
`ParameterAction`, `ParameterSetAction`, `ParameterAddValueRule`,
`ParameterMultiplyByValueRule` (each invented a name/ref/value — gained `::new`),
`VariableModifyAction`/`ParameterModifyAction` (fabricated a whole-child `Rule` — gained
`::new`, plus `VariableModifyRule::add_value`/`::multiply_by_value` and
`ModifyRule::add_value`/`::multiply_by_value` for the nested choice), `UserDefinedAction`
(fabricated a whole-child `CustomCommandAction`), `CustomCommandAction` (invented `"default"`
for `@type`). In `appearance.rs` (5): `VisibilityAction`, `LightStateAction`, `LightState`,
`AnimationState`, `SensorReference` — each gained `::new`. In `control.rs` (6):
`ActivateControllerAction` (none of its attributes is `use="required"` or has a
`default="…"`, so the fabricated `true`/`false` values were wrong even for a `Default` — it
is now `#[derive(Default)]`, all-`None`, and the values live in the pre-existing
`all_domains`/`movement_only` constructors), `ManualGear`, `AutomaticGear`, `Brake`,
`BrakeInput`, `Gear` — all five already had explicit constructors from an earlier pass. In
`trailer.rs` (1): `ConnectTrailerAction` (invented `"DefaultTrailer"` — gained `::new`).
`StoryGlobalAction` (`src/types/scenario/story.rs`) lost its `#[derive(Default)]`, which had
transitively required the removed `wrappers::GlobalAction: Default`; the field it wraps is
already `Option<StoryGlobalAction>`, so no default was needed.

OSR-04 agent E (`entities/selection.rs`, `positions/road.rs`, `geometry/shapes.rs`,
`scenario/triggers.rs`, `basic.rs`, `scenario/init.rs`) removed all 25 fabricating impls in
those six files. `entities/selection.rs`: `EntitySelection`, `EntityDistributionEntry`,
`ScenarioObjectTemplate`, `ExternalObjectReference`, `ByObjectType`, `ByType` removed with no
replacement (each already had, or gained, an explicit `::new`-style constructor);
`EntityDistribution`'s fabricating impl (a whole-child `EntityDistributionEntry`) was replaced
with `#[derive(Default)]` — its `Vec` field has no `minOccurs="0"` so the empty result is not
schema-valid content alone, but it states nothing invented. `geometry/shapes.rs`: `Center`/
`Dimensions` removed (`Center` gained `::new`); the `#[derive(Default)]` on `BoundingBox` that
depended on them was removed too, replaced with `BoundingBox::new(center, dimensions)`.
`Vertex` (fabricated a whole-child `Position`) removed, gained `::new`/`::with_time`. `Shape`'s
and `Polyline`'s fabricating impls were replaced with `#[derive(Default)]` (all-`None`/empty
`Vec`, matching `Position`'s existing treatment elsewhere). `scenario/triggers.rs`:
`Condition`/`ConditionType` — the whole-child fabrication OSR-04 agent C flagged but could not
remove — deleted with no replacement. `TriggeringEntities` removed (already had `::new`/
`::any`/`::all`). `Trigger`'s and `ConditionGroup`'s fabricating impls (each invented a
whole-child) were replaced with `#[derive(Default)]`, consistent with the container/choice
policy. `basic.rs`: `ParameterDeclaration`, `ValueConstraint`, `Range`, `Directory` removed
(all four already had `::new`-style constructors); `Value<T>`'s serde impls untouched (F15).
`positions/road.rs`: `RelativeRoadPosition`/`RelativeLanePosition` removed (both already had
`::new`). `scenario/init.rs`: `Private` removed (already had `::new`); `LongitudinalAction`'s
fabricating impl was replaced with `#[derive(Default)]`, matching the sibling `PrivateAction`/
`GlobalAction` choice groups in the same file. One collateral `#[derive(Default)]` removal
outside this issue's file list: `ControllerCatalogLocation` (`src/types/controllers/mod.rs`),
an apparently-unused duplicate of `catalogs::locations::ControllerCatalogLocation`.

OSR-04 agent G (`entities/vehicle.rs`, `scenario/{monitors,variables,story}.rs`,
`positions/{relative,mod}.rs`, `builder/**` outside `builder/conditions/*`) closed the coverage
gap agent E's file set left. `Vehicle::default()` (invented name `"DefaultVehicle"`, a full
bounding box and performance figures), `MonitorDeclaration::default()` (`"DefaultMonitor"`),
`VariableDeclaration::default()` (`"DefaultVariable"`), `RelativeObjectPosition::default()` and
`positions::mod::RelativeWorldPosition::default()` (both `"DefaultEntity"`) were removed with no
replacement; each type already had, or gained, an explicit `::new`-style constructor.
`StoryAction`/`StoryPrivateAction` (`scenario/story.rs`) — the whole-child fabricators OSR-03
deliberately deferred — were also removed with no replacement `Default`: `StoryAction`'s
`@name` is `use="required"` with no schema default, and `StoryPrivateAction` mirrors XSD
`PrivateAction` (`:1777-1791`), a bare `xsd:choice` with no `minOccurs="0"` override, so an
all-`None` value is *also* not schema-valid — the F16 trap, avoided here by giving both types
explicit per-branch constructors (`StoryAction::private`, `StoryPrivateAction::{longitudinal,
visibility, teleport}`) instead of a container-style derived default. In `src/builder/`:
`VisibilityActionBuilder`'s `Default` (fabricating "fully visible" for three XSD-required,
schema-default-less attributes) was removed and inlined into its own `new()`, silencing the
resulting `clippy::new_without_default` rather than reintroducing the impl.
`add_global_environment_action`/`add_default_environment_action` (`builder/init/{actions,
private}.rs`), which invented `Environment::name = "DefaultEnvironment"` on every call, now take
an explicit `name: &str`, threaded up through `InitActionBuilder::{with_default_environment,
for_single_vehicle, for_multiple_vehicles}` and `InitActionBuilderForStoryboard`.
`CatalogEntityBuilder`'s and `ScenarioBuilder<Empty>`'s `Default` impls (`builder/catalog.rs`,
`builder/scenario.rs`) were left in place — both simply delegate to their own `::new()` and
invent nothing.

OSR-04 agent F closed the last item in this issue's own scope: the seven builder `Default`
impls in `src/builder/conditions/{entity,value,spatial}.rs`
(`AccelerationConditionBuilder`, `EnhancedSpeedConditionBuilder`,
`TraveledDistanceConditionBuilder`; `SpeedConditionBuilder`, `ParameterConditionBuilder`,
`VariableConditionBuilder`; `RelativeDistanceConditionBuilder`) each fabricated a `Rule` (or, for
`RelativeDistanceConditionBuilder`, also a `freespace` bool and a `RelativeDistanceType`) that
the XSD marks `use="required"` with no `default="…"` — confirmed by reading
`Schema/OpenSCENARIO.xsd:1843-1851` for the latter. All seven `rule`/`freespace`/
`relative_distance_type` fields became `Option<…>`, set together with the value in every
existing setter (`*_above`/`*_below`/`*_equals`, `closer_than`/`farther_than`,
`longitudinal`/`lateral`), with `build()` erroring if a setter was never called.
`TraveledDistanceConditionBuilder`'s `rule` field was dropped outright — `TraveledDistanceCondition`
(`Schema/OpenSCENARIO.xsd`) carries only `@value`; the field was never read by `build()`, so its
fabricated default was dead code, not just an unused constructor argument. All seven structs now
derive `Default` (every field `Option`, so the derived impl states nothing) instead of hand-writing
one — that keeps `new()` `clippy::new_without_default`-clean without an `#[allow]`. `src/builder/
conditions/*` has zero fabricating `Default` impls as of this change.

**The Default policy stated above is *not* fully enforced.** `grep -rn "^impl Default for" src/`
found 44 hand-written impls before OSR-08 (down from 51 before this issue's seven were removed —
OSR-03/OSR-04 never claimed to cover every file in `src/`). Manual review of all 44 sorted them
into 15 legitimately benign and 29 that fabricate content the policy forbids.

**OSR-08 agent H removed all 18 of those 29 that live in `src/types/distributions/{deterministic,
mod,stochastic}.rs` and `src/types/entities/axles.rs`** — the two directories F4's original sweep
missed entirely. `grep -rn "^impl Default for" src/` now finds **26**. What was removed and why,
per the classification method used across this series:

- `deterministic.rs` (9 impls, all removed): `DeterministicParameterDistribution` and
  `DeterministicSingleParameterDistributionType` are xsd:choice groups whose every variant
  requires real content — no member "states nothing". `DeterministicSingleParameterDistribution`,
  `DistributionSet`, `DistributionSetElement`, `DistributionRange`, `ValueSetDistribution`,
  `ParameterValueSet`, and `ParameterAssignment` all either fabricated a required attribute value
  (a parameter name, a step width, a distribution value) or an empty/single-element `Vec` that
  isn't schema-valid either — `Schema/OpenSCENARIO.xsd` gives `DistributionSet.Element`,
  `ValueSetDistribution.ParameterValueSet`, and `ParameterValueSet.ParameterAssignment` no
  `minOccurs="0"` (all `maxOccurs="unbounded"` with implicit `minOccurs="1"`), so this is the F16
  trap, not the benign case. Each now has an explicit `::new()` requiring the caller to supply
  real content.
- `mod.rs` (6 impls, all removed): `ParameterValueDistribution::default()` fabricated a whole
  nested `Deterministic` distribution tree plus a fake `"default.xosc"` scenario file — the
  "fabricating a whole child" case with no defensible replacement (`new_deterministic`/
  `new_stochastic` already existed). `DistributionDefinition`,
  `DistributionDefinitionGroup`, `DeterministicParameterDistributionGroup`, and
  `DeterministicSingleParameterDistributionTypeGroup` are xsd:choice wrappers with the same
  no-empty-variant problem as above. `UserDefinedDistribution::default()` invented
  `content`/`type` literal `"default"` for two fields both `use="required"`; it now has a
  `::new(content, distribution_type)`. Two structs (`DeterministicMultiParameterDistributionTypeGroup`,
  `ParameterValueDistributionDefinitionGroup`) also lost a *derived* `Default` (not counted in the
  44/26, since the grep is for hand-written `impl Default for`) once their single required field's
  type stopped implementing it.
- `stochastic.rs` (1 impl, removed): `Stochastic::default()` fabricated `numberOfTestRuns: 1`
  where `Schema/OpenSCENARIO.xsd:2085` marks the attribute `use="required"` with no schema
  default, and defaulted `distributions` to an empty `Vec` where `StochasticDistribution` also
  has no `minOccurs="0"` — another F16 instance. Replaced with `::new(number_of_test_runs, first,
  rest)`.
- `entities/axles.rs` (2 impls, removed): `Axles`/`Axle::default()` returned `Self::car()`/
  `Self::rear_car()` — fixed vehicle geometry nobody specified, invented from five separately
  `use="required"` attributes with no schema default. The named presets (`car()`, `truck()`,
  `trailer()`, `motorcycle()`, `front_car()`, `rear_car()`, …) remain as explicit constructors;
  only the silent `Default`/`::default()` path was removed.

**OSR-08 agent I closed the two stragglers and the rest of the catalog subtree.**
`src/types/conditions/entity.rs:613` (`SpeedCondition::default()`, inventing `value: 10.0, rule:
GreaterThan`) and `src/types/positions/trajectory.rs` (`Trajectory::default()`, whose `Polyline`
defaulted to zero vertices — F16: `Schema/OpenSCENARIO.xsd`'s `Polyline` requires `minOccurs="2"`,
so that value could never serialize to schema-valid XML) were both removed with an explicit
`::new()` in their place; `SpeedCondition::new` already existed, `Trajectory::new` is new and
mirrors the identical constructor already present on the sibling
`actions::movement::Trajectory`.

The catalog subtree contributed nine more removals, all fabricating a required field with no
XSD `default="…"`, each replaced with an explicit constructor: `CatalogRoute`/`RouteWaypoint`
(`types/catalogs/routes.rs` — name `"DefaultCatalogRoute"` and a fabricated origin waypoint with
`RouteStrategy::Fastest`), `CatalogController`/`ControllerProperty`
(`types/catalogs/controllers.rs` — name/type and a fake `"defaultProperty"`/`"defaultValue"`
pair; `ControllerProperties`'s own `#[derive(Default)]` stays, since XSD `Properties` has every
child at `minOccurs="0"`), `CatalogFog` (`types/catalogs/environments.rs` — a fabricated 100km
`visualRange`; `CatalogFog::new(visual_range)` replaces it, `CatalogWeather`'s own
`#[derive(Default)]` stays as all-`Option`), `CatalogFile`/`CatalogContent`
(`types/catalogs/files.rs` — the issue's namesake offender, `"DefaultCatalog"`/
`"openscenario-rs"`), `ParameterAssignment` (`types/catalogs/references.rs` — fabricated
`"defaultParam"`/`"defaultValue"`; sibling `ParameterAssignments`' `#[derive(Default)]` stays,
XSD `minOccurs="0"`), and `CatalogTrajectory` (`types/catalogs/trajectories.rs` — the same
double fabrication as `positions::trajectory::Trajectory`: an invented name *and* an
`F16`-invalid zero-vertex `Polyline`).

Removing `CatalogContent`'s `Default` surfaced two **derived** defaults (not counted by
`grep -rn "^impl Default for"`, which only sees hand-written impls) that had been silently
piggy-backing on it: `catalogs::mod::Catalog`/`CatalogDefinition` and
`scenario::storyboard::CatalogDefinition` each wrapped a required `CatalogContent` field and
would otherwise still fabricate `"DefaultCatalog"` through the derive even with the hand-written
impl gone. Both lost their `#[derive(Default)]`; the storyboard one gained a `::new()` it never
had.

`src/catalog/{mod,loader,resolver,parameters}.rs`, `src/parser/{validation,choice_groups}.rs`,
`src/builder/{scenario,catalog}.rs`, and `src/types/controllers/mod.rs` were all **verified, not
assumed**: every remaining impl there was read against its type's shape, not just its file's
prior "done" status.

**`grep -rn "^impl Default for" src/` now finds 15 — every one reviewed individually, none
fabricating:**

- `catalog/loader.rs` `CatalogLoader`, `catalog/mod.rs` `CatalogManager`, `catalog/resolver.rs`
  `CatalogManager`/`CatalogResolver`, `catalog/parameters.rs` `ParameterSubstitutionEngine`,
  `builder/catalog.rs` `CatalogEntityBuilder`, `builder/scenario.rs`
  `ScenarioBuilder<Empty>` — all pure delegations to a non-fabricating `::new()`; no field is
  invented, only empty collections/`None`/a fixed regex pattern used purely as an implementation
  detail (`ParameterSubstitutionEngine`'s parameter-placeholder regex), not scenario content.
- `parser/validation.rs` `ValidationConfig`/`ScenarioValidator`/`ValidationResult`,
  `parser/choice_groups.rs` `ChoiceGroupRegistry` — non-XSD tooling types (validation feature
  toggles, a zero-field parser registry, an empty validation-run result). These describe the
  crate's own machinery, not `.xosc` content, so the policy's rationale (a default that
  "describes something nobody wrote" in a *scenario*) does not apply to them.
- `types/catalogs/locations.rs` `CatalogLocations` — delegates to `::new()`, all eight catalog
  slots `None`; XSD's `CatalogLocations` model group has every child `minOccurs="0"`.
- `types/controllers/mod.rs` `ObjectController` — all-`None` (`name`/`controller`/
  `catalog_reference`); XSD `ObjectController` makes `@name` optional and its `xsd:choice` body
  has no required branch, so all-`None` is a schema-valid "nothing selected" choice container.
- `types/actions/traffic.rs` `TrafficStopAction` — a zero-field struct (`Self {}`); confirmed
  benign survivor from OSR-04 agent B.
- `types/actions/control.rs` `AssignControllerAction` — all seven fields `Option`, a `xsd:choice`
  plus independently-optional activation flags; confirmed benign survivor from OSR-04 agent D.

**A textual grep still undercounts.** A manual sweep for `#[derive(Default)]` on structs with a
required (non-`Option`, non-`Vec`) field — the same shape `grep -rn "^impl Default for"` cannot
see — turned up one more confirmed live fabrication, **outside every file this issue or OSR-04
assigned to any agent**: `src/types/scenario/story.rs:249` derives `Default` for `Actors`, whose
`@selectTriggeringEntities` is `use="required"` in `Schema/OpenSCENARIO.xsd` (`:727`) with no
schema `default="…"`. The derive fabricates `false`, and it is not dead code —
`src/types/scenario/story.rs:365` calls `Actors::default()` live. Every other struct the same
sweep flagged (`TeleportAction`, `AcquirePositionAction`, `AddEntityAction` — each wrapping a
`Position`, itself an all-`None` choice; `ScenarioDefinition` in both `types/scenario/mod.rs` and
`types/scenario/storyboard.rs`, `Init`, `Storyboard` — each a required child whose own `Default`
bottoms out in all-`None`/empty-and-schema-valid state; `ValidationContext`, non-XSD tooling) was
individually checked and is benign for the reason given.

**The honest claim: the policy is enforced everywhere this campaign has looked, and it has now
looked everywhere in `src/` — but "everywhere" surfaced one more fabricating derive the day the
last hand-written impl was checked off, in a file nobody had assigned.** Ten agents across
OSR-03, OSR-04, and OSR-08 removed 174 fabricating impls (hand-written and derived) with zero
harness regressions; the crate's `Default` policy is real and it holds for every impl this report
lists. It is not, today, true that *no* fabricating `Default` remains — `Actors::default()` is
one, named above, unfixed, because `types/scenario/story.rs` was never on this or any prior
issue's file list. The lesson this issue exists to teach held once more: state the count you
verified, not the count you assume, and do not write "and N others."

## A trap: unknown fields are silent

Nothing in the crate uses `#[serde(deny_unknown_fields)]`. serde ignores unrecognized XML
attributes and elements by default, so a field the Rust types do not model is dropped at parse
with no diagnostic, and an unmodeled choice branch deserializes into a struct with every field
`None`.

This is why a green round-trip test is weaker evidence than it looks: the same data is dropped
on both passes and the comparison still succeeds. The corpus reported 172/172 for a long time
while discarding route positions, vehicle light states and global actions. The `lossy` gate
described in [xsd_gaps.md](xsd_gaps.md) is what makes first-parse loss observable, and it is
the check to run after adding a type.

## Serialization patterns worth copying

Three patterns recur, each fixing a class of schema-invalid output.

**Skip optional fields explicitly.** `LaneOffsetAction::target_lane_offset`
(`src/types/actions/movement.rs:316`) is optional in the schema; without
`skip_serializing_if` it serialized as an empty attribute and the output failed validation.

**Hand-write `Serialize` when the derive cannot express the shape.** `EntityCondition`
(`src/types/conditions/entity.rs:368`) serializes through `SerializeMap` because the derived
implementation emitted a wrapper element the schema does not define.

**Deserialize optional numerics through a helper.** `deserialize_optional_double`
(`src/types/actions/movement.rs:23`) distinguishes an absent attribute from an empty one,
which the blanket `Option<Double>` path does not.

## Cross-cutting traits

Two traits in `src/types/mod.rs` cut across the type tree:

```rust
pub trait Validate {
    fn validate(&self, ctx: &ValidationContext) -> crate::Result<()>;
}

pub trait Resolve<T> {
    fn resolve(&self, ctx: &ParameterContext) -> crate::Result<T>;
}
```

`Value<T>` has blanket impls of both. Note that the `Validate::validate` here, which takes a
`&ValidationContext` and returns `crate::Result<()>`, is distinct from the inherent
`validate() -> Result<(), String>` on choice structs described above. See
[validation_guide.md](validation_guide.md) for how the validation layers relate.

## Adding a type

1. Read the XSD declaration. Note every attribute, every child element, and the `minOccurs` /
   `maxOccurs` on each.
2. Write the struct with `@`-prefixed renames for attributes and plain renames for elements,
   applying the optionality rules above.
3. If it is a choice, pick the idiom that matches how the choice appears, and name flattened
   variants for their **elements**.
4. Before writing `#[derive(Default)]` or `impl Default`, run the three-category check in
   [The `Default` policy](#the-default-policy) — including detector 3, which no grep performs:
   a derive on a struct with a required non-`Option`, non-`Vec` field fabricates that field.
5. Add a round-trip test. If the type is a flattened choice, add it to
   `tests/choice_flatten_roundtrip_test.rs`.
6. Run the conformance gates. Their locations and what each one catches are in
   [xsd_gaps.md](xsd_gaps.md).
