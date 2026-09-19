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

- **A third conformance corpus source, and an expected-failure manifest.** The corpus now also
  fetches the Eclipse openpass
  [`openscenario1_engine`](https://gitlab.eclipse.org/eclipse/openpass/openscenario1_engine)
  scenario data, pinned at `f51968308e464fd8ebdbe5aea6323209d186c70c` and licensed **EPL-2.0**
  (the two pre-existing sources are MPL-2.0). Like them it is fetched on demand into gitignored
  `conformance/corpus/` and carries its own upstream `LICENSE`; nothing third-party is vendored.
  `find conformance/corpus -name '*.xosc' | wc -l` goes **172 → 212**, and the round-trip tests
  `conformance/build.rs` generates go **172 → 210** (`cargo test -p
  openscenario-roundtrip-harness`: 217 passed, 0 failed). Schema coverage goes from **156 of 294**
  element declarations to **175 present in the corpus**, of which **165 are reached by a file that
  a gate actually passes** — the two figures are not interchangeable and
  [docs/xsd_gaps.md](docs/xsd_gaps.md) states which is which. The import also added
  `conformance/expectations.toml`, which records the three corpus files whose expected outcome is
  not "passes everything": two deliberately invalid upstream fixtures and one schema-valid file
  this crate cannot yet parse. Exemptions are per gate, and each entry asserts both a premise
  about the input and that the named gates still genuinely fail, so a stale exemption breaks the
  build instead of silently protecting nothing. Gate results immediately after this import (later
  entries in this section record how the harness and these counts changed as further gates
  landed): `report` 210 passed / 0 failed / 2 excluded / 212 total; `lossy` 210 lossless /
  0 lossy / 0 skipped / 2 excluded / 212 total with 0 dropped and 0 invented items; `validate`
  209 schema-valid / 0 schema-invalid / 0 skipped / 3 excluded / 212 total with 0 validation
  errors. No change to `openscenario-rs`'s public API.
- **A fourth conformance gate, `validate-input`, and character content visible to the fidelity
  gate.** `profile()` (`conformance/src/xml_profile.rs`), which reduces a document to a multiset
  of element/attribute keys for the `lossy` gate, previously handled only element start/end/empty
  events and discarded text nodes, so `lossy` — the gate whose job is "nothing was dropped,
  nothing was invented" — could not see character content being dropped. It now emits
  `path/to/Element#text` keys, counted per node rather than by value, so serialization is free to
  change escaping or whitespace without tripping the gate. That alone only made the loss visible
  from one side: a schema-invalid input whose invalid part this crate does not model could still
  be parsed, have the invalid content silently dropped, and pass `validate` on output because
  something was removed. The new `validate-input` gate closes that by checking each corpus file
  as it sits on disk against `Schema/OpenSCENARIO.xsd`, without parsing it through this crate, so
  its verdict is a fact about the corpus rather than about the code. `bash scripts/gate.sh` now
  runs **eleven** stages, not ten. Of the 212 corpus files, exactly three fail input validation,
  all upstream openpass fixtures, and `conformance/expectations.toml` records why each is exempt
  from which gates. Gate results: `lossy` 209 lossless / 0 lossy / 0 skipped / 3 excluded /
  212 total; `validate` 209 schema-valid / 0 schema-invalid / 0 skipped / 3 excluded / 212 total;
  `validate-input` 209 input-schema-valid / 0 input-schema-invalid / 0 skipped / 3 excluded /
  212 total. No change to `openscenario-rs`'s public API.
- **The generated round-trip test suite now fails loudly, instead of passing over nothing, when
  it goes stale.** `conformance/build.rs` regenerates one round-trip test per corpus file at
  build time, and cargo's `rerun-if-changed` on the corpus directory tracks only the directory's
  own mtime. Moving the corpus aside and back — as when re-fetching it by hand — preserves that
  mtime, so cargo would not rerun the build script and a stale, empty generated test file could
  survive undetected: the `tests/generated.rs` target would run nothing while the package's other
  test targets kept passing, so `cargo test -p openscenario-roundtrip-harness` and the rest of the
  gate stayed green. The generated file now also records the
  corpus file count the build script saw, and a new check in `conformance/tests/generated.rs`
  compares that count against the corpus on disk at run time, failing with a recovery hint
  (`touch conformance/build.rs`) if they disagree. `cargo test -p openscenario-roundtrip-harness`
  reports 225 passed, 0 failed. No change to `openscenario-rs`'s public API.
- **`FromStr`/`Display` on all 37 `src/types/enums.rs` enumerations.** Nine enums
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
  was removed — this change only adds impls and refactors existing ones into the macro; the
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
- **`MinVec<T, MIN>` in `types::basic`, a vector that carries its schema lower bound in its
  type.** Several `xsd:sequence` particles in `Schema/OpenSCENARIO.xsd` declare `minOccurs`
  above one: `Vertex` and `Waypoint` require two children, `Position` inside `Polygon`
  requires three. `Vec<T>` cannot state such a bound and no serde attribute states it either,
  so dropping `#[serde(default)]` rejects only the empty case and a `<Polyline>` holding a
  single `<Vertex>` still parses and is written back out in the same schema-invalid shape.
  `MinVec` checks the length after deserializing a `Vec<T>` and keeps its inner field
  private, with no `DerefMut` and no `Default` for `MIN > 0`. Hence a value shorter than
  `MIN` is unconstructable rather than merely detectable, which closes the serialization
  side as well as the parse side. The type is additive: no field changes type in this
  release entry, and the fields that will adopt it are converted separately.

### Changed

Breaking, unless noted.

- **`RoutingAction`, `StoryAction` and `StoryPrivateAction` now hold a choice enum instead of
  parallel `Option` fields.** XSD `RoutingAction` (`:1981-1988`), `Action` (`:705-712`) and
  `PrivateAction` (`:1777-1791`) are each a bare `xsd:choice`, so `minOccurs` and `maxOccurs`
  both default to 1 and exactly one branch is required. Modeled as parallel `Option` fields,
  all three accepted a document naming no branch and a document naming two, with no
  diagnostic in either direction: `<RoutingAction/>` parsed into an all-`None` value, and
  `<RoutingAction><RandomRouteAction/><AcquirePositionAction>…</AcquirePositionAction></RoutingAction>`
  parsed into a value carrying both branches. The branch now sits in an externally tagged enum behind
  `#[serde(rename = "$value")]`, hence serde rejects zero branches with
  ``missing field `$value` `` and two with ``duplicate field `$value` ``, and no
  hand-written check is involved.

  This breaks field access and struct literals. `RoutingAction`'s four `Option` fields become
  one `routing_choice: RoutingActionChoice`; `StoryAction`'s three become one
  `action: StoryActionChoice` beside the unchanged `name`; and `StoryPrivateAction`'s ten
  become one `action: StoryPrivateActionChoice`. The three new public enums name their
  variants after the XSD element names. `RoutingAction::with_assign_route`,
  `::with_trajectory`, `StoryAction::private`, `StoryPrivateAction::longitudinal`,
  `::visibility` and `::teleport` keep their signatures. `StoryPrivateAction::empty`, a
  private helper that built the all-`None` value, is gone with the shape it built.

  The conversion also made `convert_private_action_to_story` in `src/builder/` total. It
  previously mapped six of the ten `PrivateAction` branches and returned an empty
  `StoryPrivateAction` for the other four, so an activate-controller, controller, appearance
  or trailer action built through that path was replaced by an action naming nothing. All ten
  branches now map to their counterpart.

- **`AppearanceAction`, `LightType`, `Color`, `AnimationType` and `ComponentAnimation` now hold
  a choice enum instead of parallel `Option` fields.** XSD `AppearanceAction`
  (`Schema/OpenSCENARIO.xsd:765-770`), `LightType` (`:1418-1423`), `Color` (`:929-935`),
  `AnimationType` (`:757-763`) and `ComponentAnimation` (`:947-952`) are each a bare
  `xsd:choice`, so exactly one branch is required. Modeled as parallel `Option` fields, all
  five accepted a document naming no branch and a document naming two, keeping both rather
  than reporting an error. Each now carries its branch in an externally tagged enum behind
  `#[serde(rename = "$value")]`, so serde rejects zero branches with
  ``missing field `$value` `` and two with ``duplicate field `$value` ``.

  This breaks field access and struct literals. `AppearanceAction`'s two `Option` fields
  become one `choice: AppearanceActionChoice`; `LightType`'s two become
  `choice: LightTypeChoice`; `Color`'s two become `choice: ColorChoice` beside the unchanged
  `color_type`; `AnimationType`'s four become `choice: AnimationTypeChoice`; and
  `ComponentAnimation`'s two become `choice: ComponentAnimationChoice`. The five new public
  enums name their variants after the XSD element names. `AppearanceAction::new`,
  `LightType::new`, `Color::new`, `ComponentAnimation::new`, and `AnimationType`'s
  `::component`/`::pedestrian`/`::file`/`::user_defined` and `ComponentAnimation`'s
  `::vehicle`/`::user_defined` replace the removed all-`None` constructors.
  `AppearanceAction::empty` and the private `AnimationType::empty`, each a helper that built
  the all-`None` value, are gone with the shape they built.

- **`TrailerAction` and `Trailer` now hold a choice enum instead of parallel `Option`
  fields.** XSD `TrailerAction` (`Schema/OpenSCENARIO.xsd:2342-2347`) is a bare `xsd:choice`
  of `ConnectTrailerAction` | `DisconnectTrailerAction`, and `Trailer` (`:2336-2341`) is a
  bare `xsd:choice` of an inline `Trailer` (type `ScenarioObject`) or a `TrailerRef`.
  Neither carries `minOccurs`/`maxOccurs`, so exactly one branch is required. Modeled as
  parallel `Option` fields, both accepted a document naming no branch and a document naming
  two, with no diagnostic in either direction. Each now carries its branch in an externally
  tagged enum behind `#[serde(rename = "$value")]`, so serde rejects zero branches with
  ``missing field `$value` `` and two with ``duplicate field `$value` ``.

  This breaks field access and struct literals. `TrailerAction`'s two `Option` fields become
  one `choice: TrailerActionChoice`; `Trailer`'s two become one `choice: TrailerChoice`. Three
  sibling types in the same traffic-and-trailer group were checked and left unconverted:
  `TrafficSourceAction` and `TrafficSwarmAction` (`:2300-2314`, `:2317-2334`) are each an
  `xsd:all` of required attributes and optional elements, not a choice, so their parallel
  `Option` fields already state the schema correctly; `TrafficStopAction` (`:2315-2316`) is
  an empty complex type with no fields at all. `Vehicle` (`:2498-2514`), which holds a
  `Trailer` among five other optional elements, is likewise `xsd:all` and was left as it was.

- **`Property`'s `@name` and `@value` now accept parameter references.** XSD `Property`
  (`Schema/OpenSCENARIO.xsd:1809-1812`) types both attributes as the schema's `String`, a
  union whose second member is the parameter production, so `<Property name="maxSpeed"
  value="$speedLimit"/>` is schema-valid. `entities::vehicle::Property` held both as plain
  Rust `String`, so a document naming a parameter there parsed and round-tripped
  byte-identically while carrying the literal text `$speedLimit` as data; nothing
  substituted it and nothing errored. This was reported as a known narrowing under
  *Removed*, above, when the three divergent `Properties` types were folded onto this one;
  it is fixed here rather than left standing. Both fields now hold `OSString`
  (`Value<String>`), so
  `Value<T>::resolve` (`src/types/basic.rs`) — the same path every other parameterizable
  attribute goes through — resolves them too.

  This breaks direct field access and struct literals: `property.name` and `property.value`
  are `OSString` rather than `String`, so a struct literal must supply
  `OSString::literal("...".to_string())` and a comparison against a `&str` becomes
  `property.value.as_literal().map(String::as_str)`.

- **90 enum-typed attributes now accept parameter references.** All 37 enumeration
  `simpleType`s in `Schema/OpenSCENARIO.xsd` are `xsd:union`s whose second member is
  `<xsd:restriction base="parameter"/>`, so `<Vehicle vehicleCategory="$cat">` is
  schema-valid. The crate modelled each of the 75 XSD attributes so declared as a bare Rust
  enum and rejected all of them at parse. Those **90 fields across 25 files** (54 required,
  36 optional) now hold `Value<E>` and `Option<Value<E>>` respectively; the catalog twins
  moved in lockstep with their scenario counterparts. `Value<T>` was extended rather than
  duplicated, so `Resolve<T>` (`src/types/mod.rs`) and
  `ParameterSubstitutionEngine::resolve_value` (`src/catalog/parameters.rs`) work on enums
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

- **`Value::Parameter` now serializes as `$name`, not `${name}`.** The schema
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
- **`clippy::large_enum_variant` is now allowed crate-wide, from `src/lib.rs`, instead of at
  three individual enum sites.** Every choice group modeled as an externally tagged enum has
  variant sizes set by the schema types its branches carry, not by a design choice in this
  crate, so the lint's population here is exactly the set of enums that mirror an
  `xsd:choice`. Boxing the larger branches would add an indirection the wire format does not
  ask for, purely to satisfy a lint that is measuring the schema rather than the code. The
  crate-level allow removes three local ones and covers every choice enum still to be
  converted, rather than requiring one per site. `cargo clippy --features builder,validation
  --all-targets` goes **197 → 191**. This changes no behavior; `bash scripts/gate.sh` stays
  green across all eleven stages, and the test total is unchanged at 1559.
- **`CONTRIBUTING.md` no longer presents parallel `Option` fields or `#[serde(flatten)]` as
  acceptable ways to model a choice group.** Both idioms are described along with the failure
  each one produces, and the guide now names the externally tagged enum behind `$value` as the
  one shape to write. The pointer to a single dedicated round-trip test file is replaced with
  the same neighbor-file rule the guide already states for every other test.

### Removed

- **Two divergent `Properties` types, folded into the one the schema actually describes.**
  These are breaking changes.
  - **`types::catalogs::controllers::ControllerProperties`** and its paired
    **`ControllerProperty`** — `ControllerProperties` declared only a `Property` child, so a
    `<Properties>` element carrying a `<File>` or `<CustomContent>` child parsed without error
    while silently dropping both; `ControllerProperty` modeled `@value` as a parameterizable
    `Value<String>`, a capability the surviving type does not carry (see the note below).
    **Surviving type: `types::entities::vehicle::Properties`**, whose `Property`, `File` and
    `CustomContent` fields are each a `Vec` with `#[serde(default)]`, matching XSD `Properties`
    (`Schema/OpenSCENARIO.xsd:1802-1808`), which places all three at `minOccurs="0"
    maxOccurs="unbounded"`. `CatalogController::properties` now holds this type directly.
  - **`types::controllers::ControllerProperties`** — a second type of the same name, reachable
    only through the crate's public re-export and not through any parsing path inside the
    crate, whose `Property` field lacked `#[serde(default)]`. XSD `Properties` permits an empty
    element, so an absent `Property` list should deserialize to an empty vector; instead it
    was a parse error. **Surviving type: the same `types::entities::vehicle::Properties`**,
    which `types::controllers::Controller::properties` already held.
  - A caller who set a catalog controller's property value to a parameter reference
    (`$speedLimit`) lost that ability: `entities::vehicle::Property::value` was a plain
    `String`. No file in the conformance corpus exercises a parameterized `Property` value, so
    the loss was not visible to any gate. Fixed under *Changed*, below: both `Property`
    attributes now hold `OSString`.

- **The typed entity catalog reference, and the accessors and constructors built on it.**
  These are breaking changes.
  - **`types::entities::ScenarioEntityReference`** — an `#[serde(untagged)]` enum over
    `CatalogReference<CatalogVehicle>` and `CatalogReference<CatalogPedestrian>`, which
    silently resolved every parsed reference to the vehicle variant; see the entry under
    *Fixed* below. **Surviving type: `types::entities::EntityCatalogReference`**, which is
    untyped, as the schema's element is.
  - **`ScenarioObject::new_vehicle_catalog_reference`** and
    **`ScenarioObject::new_pedestrian_catalog_reference`** — the entity kind is not part of
    a catalog reference, so the two constructors said something the document cannot say.
    **Surviving constructor: `ScenarioObject::new_catalog_reference`.**
  - **`ScenarioObject::vehicle_catalog_reference`** and
    **`ScenarioObject::pedestrian_catalog_reference`** — the first returned `Some` for every
    parsed reference and the second for none of them. **Surviving accessor:
    `ScenarioObject::catalog_reference`**, which returns the reference without claiming a
    kind.
  - **The `Serialize` and `Deserialize` derives on `types::scenario::triggers::ConditionType`**
    — the enum is a constructor argument for `Condition`, not a serde field, since XSD
    `Condition` places the choice directly in the element and `Condition` models it with
    parallel `Option` fields. The derives were untagged, hence they presented the same
    guess-by-trial-order shape as a sanctioned pattern. The enum itself remains, and
    `Condition::new` still takes it.

- **Duplicate and dead types.** Each entry names the surviving twin:
  - **`catalog::resolver::CatalogManager`** — a second, shadowing `CatalogManager` distinct
    from the canonical `catalog::CatalogManager` (`src/catalog/mod.rs`), which is what
    `lib.rs`'s `pub use catalog::{… CatalogManager …}` has always resolved to. The
    duplicate's only consumers were three dead-weight accessors on `CatalogResolver`
    (`with_catalog_manager`, `set_catalog_manager`, `catalog_manager`) and the private
    `catalog_manager` field they wrote to; nothing ever read it. Those are removed with it.
    **Surviving twin: `openscenario_rs::catalog::CatalogManager`.**
  - **`catalog::resolver::CatalogResolvable<T>`** — a second `CatalogResolvable` trait with
    **zero** impls crate-wide. **Surviving twin:
    `types::catalogs::references::CatalogResolvable`**, which is implemented for
    `CatalogReference<T>`.
  - **`types::positions::trajectory::Trajectory`** (and its `positions::Trajectory`
    re-export) — a dead duplicate with no consumers outside its own `impl` and unit tests,
    which also modeled the required `@closed` attribute as a plain Rust `bool` where
    `Schema/OpenSCENARIO.xsd:2361` declares it XSD type `Boolean`, a union that admits
    `$param`. **Surviving twin: `types::actions::movement::Trajectory`**, which uses
    `Boolean` and is what `TrajectoryRef` already boxes.
  - **`types::actions::ValidateAction`** — a trait with zero impls; its only mention in the
    tree was a commented-out line. **Surviving twin: `types::mod::Validate`.**
  - **`types::catalogs::Catalog` / `types::catalogs::CatalogDefinition`** — both `pub` and
    reachable, so removing them is breaking for any downstream consumer that named either path
    directly, even though nothing inside this crate did. `Catalog` wrapped `CatalogContent`
    behind `#[serde(flatten)]` for no reason the schema requires (see **Fixed**), and
    `CatalogDefinition` only wrapped `Catalog`. **Surviving twin:
    `types::scenario::storyboard::CatalogDefinition`**, which already held `CatalogContent`
    directly and is what `OpenScenario::catalog` has always used.
  - **`types::distributions::deterministic::DeterministicParameterDistribution`** — an
    externally-tagged enum naming its branches `Single`/`Multi`, where
    `Schema/OpenSCENARIO.xsd:1039-1044` declares the same choice group under the element names
    `DeterministicSingleParameterDistribution`/`DeterministicMultiParameterDistribution`.
    **Surviving twin: `types::distributions::DeterministicParameterDistributionGroup`**, which
    already names both branches correctly and was defined for the same XSD group. Reachable
    only through its own constructors, an unused iterator (`Deterministic::all_distributions`,
    removed with it) and unit tests; no parsed field held it.
  - **`types::scenario::init::LongitudinalActionType`** — a second, incomplete model of XSD
    `LongitudinalAction` (`Schema/OpenSCENARIO.xsd:1431-1437`), a three-way choice of
    `SpeedAction`, `LongitudinalDistanceAction` and `SpeedProfileAction`. The enum named only
    the first branch. **Surviving twin: `types::scenario::init::LongitudinalAction`**, the
    struct already held by `Private.longitudinal_action`, which models all three branches as
    parallel `Option` fields. `LongitudinalActionType` had no consumer beyond its own
    definition and re-export.
- **Five public types with no schema counterpart**: `controllers::ControllerDistribution`,
  `controllers::ActivateControllerAction`, `controllers::ControllerAssignment`,
  `positions::RoadCoordinate` and `positions::LaneCoordinate`. The `controllers` module now
  exports `Controller` and `ObjectController` only; its own `ControllerProperties` was removed
  in a later pass (see below).
- **`Condition` and `ConditionWrapper`**, neither of which the schema defines.
- **Non-schema extension fields** on `ActivateControllerAction` and `SpeedCondition`, and a
  broader sweep of fields and types with no schema counterpart.
- **`Default` impls that invent scenario data (first tier).** `Default` was
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
- **`Default` impls that invent scenario data (`types/actions/movement.rs`,
  `types/positions/route.rs`, `types/routing/mod.rs`).** 21 further fabricating impls
  removed, including the `Route`/`Waypoint`/`RouteRef` carve-out the first tier could not
  complete:
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
  `SpeedProfileEntry`) could not be removed while this change stayed inside those three files:
  their `Default` was required by call sites in `src/types/scenario/init.rs`,
  `src/types/actions/wrappers.rs`, and shared `tests/*.rs` files.
  `TeleportAction` and `AcquirePositionAction` keep their derived `Default`: both wrap a
  single `Position`, whose own `Default` is an all-`None` choice
  with no branch selected, so it states nothing rather than fabricating a position.
- **`Default` impls that invent scenario data (the 13 impls the entry above could not
  finish).** Fixing call sites wherever the compiler points, rather than staying inside one
  file set, removes all 13:
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
- **`Default` impls that invent scenario data (`types/actions/traffic.rs`).**
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
- **`Default` impls that invent scenario data
  (`types/conditions/{entity,value,spatial}.rs`).** All 26 fabricating impls removed; no
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
  (`src/types/scenario/triggers.rs`) called
  `ByValueCondition::default()`; both now call
  `ByValueCondition::simulation_time(SimulationTimeCondition::new(10.0, Rule::GreaterThan))`
  explicitly — the same fabricated content as before, now named rather than defaulted. The
  other `Default` impls in `triggers.rs` are dealt with in a later entry. This closes out all
  fabricating `Default` impls in `types/conditions/entity.rs`, `types/conditions/value.rs`
  and `types/conditions/spatial.rs`.
- **`Default` impls that invent scenario data
  (`types/actions/{wrappers,appearance,control,trailer}.rs`).** All fabricating impls removed:
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
  `NamedAction` (serialization known-broken at the time — `Default` removed, no replacement),
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
  `Schema/OpenSCENARIO.xsd`; running total across every pass so far: no genuine schema default
  found in any file touched).
  Call sites fixed in `tests/actions_serialization_test.rs` (two tests exercising the
  fabricated defaults rewritten to exercise the new constructors instead — same test count as
  before, 742 lib tests, so no coverage was lost),
  `tests/init_action_choices_test.rs`, and within the four owned files' own test modules.
  This closes out all fabricating `Default` impls in `types/actions/wrappers.rs`,
  `types/actions/appearance.rs`, `types/actions/control.rs` and `types/actions/trailer.rs`.
- **`Default` impls that invent scenario data (`types/entities/selection.rs`, `types/geometry/shapes.rs`, `types/scenario/triggers.rs`,
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
  In `scenario/triggers.rs`: `Condition`/`ConditionType` — flagged by the conditions pass as
  inherited fabrication — invented a whole-child `ByValueCondition` (via
  `ByValueCondition::simulation_time(...)`, the branch that fix named but did not remove) and,
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
  `::new`-style constructors. `Value<T>`'s serde impls were not touched.
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
  Two collateral `#[derive(Default)]` removals in files outside that list, reported here
  explicitly: `ControllerCatalogLocation`
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
  running total across every pass so far: still no genuine schema default found in any file
  touched). `cargo test --features builder,validation`: 742
  lib tests (same count as baseline — no coverage lost), and the conformance harness
  (`report`/`lossy`/`validate`/`builder`) is unchanged from baseline (172/172/172/13, 0
  dropped/invented). `grep -rn 'literal("Default' src/ | wc -l` went from 16 to 9.
  This closes out all fabricating `Default` impls in `types/entities/selection.rs`,
  `types/geometry/shapes.rs`, `types/scenario/triggers.rs`, `types/basic.rs`,
  `types/positions/road.rs` and `types/scenario/init.rs`.
- **`Default` impls that invent scenario data (coverage gap — 10 fabricating impls in files no
  prior pass covered, 2026-09-10).** All 10 removed, 0
  benign found.
  `types/entities/vehicle.rs`: `Vehicle` (invented `"DefaultVehicle"` for `@name`, a fabricated
  `BoundingBox`/`Performance`, XSD:2007-2012 `use="required"` name with no schema default —
  `new_car`/`new_truck`/`new_motorcycle` already existed as non-fabricating constructors).
  `types/scenario/monitors.rs`: `MonitorDeclaration` (invented `"DefaultMonitor"`/`false`,
  XSD `MonitorDeclaration` — both attributes `use="required"` — `::new` already existed).
  `types/scenario/variables.rs`: `VariableDeclaration` (invented `"DefaultVariable"`/
  `ParameterType::String`/`""` — `::new` and the `string_variable`/`int_variable`/
  `double_variable`/`bool_variable` helpers already existed).
  `types/positions/relative.rs`: `RelativeObjectPosition` (invented `"DefaultEntity"`/`0.0`/
  `0.0`, XSD `RelativeObjectPosition` — `::new` already existed).
  `types/positions/mod.rs`: `RelativeWorldPosition` (invented `"DefaultEntity"`/`0.0`/`0.0`,
  XSD:1910-1922 — gained `::new(entity_ref, dx, dy)`, no constructor previously existed).
  `types/scenario/story.rs`: `StoryAction`/`StoryPrivateAction` — the whole-child fabricators
  the first tier deliberately deferred (`StoryAction` invented `"DefaultAction"` for a `use="required"`
  `@name` with no schema default, XSD `Action` :705-712, plus a fabricated
  `StoryPrivateAction::default()` child; that impl in turn invented a whole `LongitudinalAction`
  child). Both removed with **no** replacement `Default`: `StoryPrivateAction` mirrors XSD
  `PrivateAction` (:1777-1791), a bare `xsd:choice` with no `minOccurs="0"` override, so an
  all-`None` derived default would *also* be schema-invalid (the schema-invalid-empty trap —
  not repeated here).
  Both types gained explicit constructors instead: `StoryAction::private(name,
  private_action)`; `StoryPrivateAction::{longitudinal, visibility, teleport}`, one per branch
  actually exercised by existing call sites.
  In `src/builder/` (outside `builder/conditions/*`, which the next entry handles):
  `VisibilityActionBuilder` (`builder/actions/visibility.rs`) invented "fully visible" —
  `graphics`/`sensors`/`traffic` all `true` — for three XSD-required attributes with no
  schema default (`Schema/OpenSCENARIO.xsd:2554-2556`); the `Default` impl was removed and its
  body inlined into the pre-existing `new()`, with `clippy::new_without_default` silenced by
  `#[allow]` rather than reintroducing the impl — the point was to stop the value being
  reachable via `..Default::default()`/a derive bound, not to change what `new()` returns.
  `add_global_environment_action` (`builder/init/actions.rs`,
  `builder/storyboard/story.rs`) and `add_default_environment_action` — renamed
  `add_named_environment_action` — (`builder/init/private.rs`) invented
  `Environment::name = "DefaultEnvironment"` on every call (XSD `Environment` :1186-1194,
  `@name` `use="required"`, no schema default); both now take an explicit `name: &str`.
  `Environment::new(name)` was added (`types/environment/mod.rs`) as the non-fabricating
  constructor these methods, and `InitActionBuilder::{with_default_environment,
  for_single_vehicle, for_multiple_vehicles}` (`builder/init/mod.rs`), now build on.
  `CatalogEntityBuilder::default()`/`ScenarioBuilder<Empty>::default()`
  (`builder/catalog.rs`, `builder/scenario.rs`) were reviewed and kept: both simply delegate
  to their own `::new()` and invent nothing beyond what `::new()` already does.
  None of the underlying XSD attributes carries a `default="…"` (checked individually;
  running total across every pass: still no genuine schema default found anywhere). `cargo test --features builder,validation`: 742 lib tests (same
  count as baseline), and the conformance harness (`report`/`lossy`/`validate`/`builder`) is
  unchanged from baseline (172/172/172/13, 0 dropped/invented).
  `grep -rn 'literal("Default' src/ | wc -l` went from 9 to 1 — the one survivor
  (`types/conditions/spatial.rs:345`) sits in an already-closed file and was left alone here.
  This closes the coverage gap: every fabricating `Default` impl in `src/` is now gone
  **except** the seven `Default` impls in `src/builder/conditions/*`
  (`AccelerationConditionBuilder`, `EnhancedSpeedConditionBuilder`,
  `TraveledDistanceConditionBuilder`, `SpeedConditionBuilder`, `ParameterConditionBuilder`,
  `VariableConditionBuilder`, `RelativeDistanceConditionBuilder`), which the next entry
  handles.
- **`Default` impls that invent scenario data
  (`src/builder/conditions/{entity,value,spatial}.rs`).** All seven removed, 0
  benign found in scope. `AccelerationConditionBuilder`, `EnhancedSpeedConditionBuilder`
  (`entity.rs`), `SpeedConditionBuilder`, `ParameterConditionBuilder`,
  `VariableConditionBuilder` (`value.rs`) each fabricated a `Rule` (`GreaterThan`/`EqualTo`,
  no XSD default on the attribute) as their builder's implicit starting rule; their `rule`
  field became `Option<Value<Rule>>`, set by every existing `*_above`/`*_below`/`*_equals`
  setter, with `build()` erroring `"Rule is required"` if none was ever called (unreachable
  through the crate's own call sites and tests, all of which call one of those setters, but
  a real safety net rather than an invented one). `TraveledDistanceConditionBuilder`'s `rule`
  field was deleted outright rather than made optional: `TraveledDistanceCondition`
  (`Schema/OpenSCENARIO.xsd`) has only a `@value` attribute, so the field was never read by
  `build()` — its fabricated `Rule::GreaterThan` default was dead code, not a real default.
  `RelativeDistanceConditionBuilder` (`spatial.rs`) fabricated three values at once — `rule:
  LessThan`, `freespace: true`, `relative_distance_type: Cartesian` — all three now
  `Option<…>`, each required at `build()`; `Schema/OpenSCENARIO.xsd:1843-1851`
  (`RelativeDistanceCondition`) marks `freespace`, `relativeDistanceType` and `rule` all
  `use="required"` with no `default="…"` on any of them, so none had a schema-sanctioned value
  to fall back to. A `cartesian()` setter was added alongside the pre-existing
  `longitudinal()`/`lateral()` so every enum branch is reachable without a fabricated starting
  value. All seven structs now derive `Default` (every remaining field is `Option`, so the
  derived impl states nothing) rather than hand-writing one — `new()` stays
  `clippy::new_without_default`-clean with no `#[allow]` needed. `grep -rn 'literal("Default'
  src/ | wc -l`: unchanged at 1 (the one survivor, `types/conditions/spatial.rs:345`, is
  an already-closed file and was left alone). `cargo build --features builder,validation --all-targets`:
  clean, no fallout beyond the files just described. `cargo test --features builder,validation`:
  742 lib tests (unchanged). `cargo clippy --features builder,validation --all-targets`: 178
  warnings (unchanged from baseline; no `new_without_default` introduced). Conformance harness
  (`report`/`lossy`/`validate`/`builder`) unchanged from baseline: 172/172/172/13,
  0 dropped/invented.

  **Correction to the coverage-gap entry above.** "Every fabricating `Default` impl in `src/`
  is now gone except the seven in `src/builder/conditions/*`" was wrong the moment it was
  written — those seven were never the *only* gap, only the only gap *inside the file sets the
  passes so far had covered*. A full manual review of `grep -rn "^impl Default for" src/` (44 impls
  remain after this entry's seven are removed, down from 51) finds **29 that still fabricate
  content**, all outside every file covered so far: `src/catalog/`,
  `src/types/catalogs/{references,files,environments,controllers,trajectories,routes}.rs`,
  `src/types/distributions/{mod,deterministic,stochastic}.rs`, one straggler in
  `src/types/positions/trajectory.rs` (`Trajectory::default()`'s `Polyline` shape defaults to
  zero vertices — schema-invalid on its own terms, since `Polyline` requires
  `minOccurs="2"`), and one in `src/types/conditions/entity.rs:613`
  (`SpeedCondition::default()` — a miss in the otherwise-complete pass over that file,
  inventing `value: 10.0, rule: GreaterThan`). Representative: `CatalogFile::default()`
  → `"DefaultCatalog"`; `Axles`/`Axle::default()` → fixed `Self::car()`/`Self::rear_car()`
  geometry nobody specified; `Stochastic::default()` → `numberOfTestRuns: 1`
  (`Schema/OpenSCENARIO.xsd:2085`, `use="required"`, no schema default);
  `ParameterValueDistribution::default()` fabricates an entire nested `Deterministic`
  distribution tree. The other 15 of the 44 are legitimately benign (all-`None`/empty, or
  delegate to a `new()` that invents nothing, or aren't XSD-backed scenario content at all).
  See `docs/type_system_guide.md`'s `Default`-policy section for the full breakdown — it no
  longer claims full enforcement, and says so plainly rather than rounding up. This is scoped
  as separate work, not folded in here.
- **`Default` impls that invent scenario data (`src/types/distributions/
  {deterministic,mod,stochastic}.rs`, `src/types/entities/axles.rs` — the coverage gap the
  original file sweep missed entirely).** All 18 hand-written impls there removed: `xsd:choice` groups
  that defaulted to a specific variant with invented data; distribution containers defaulting a
  required `Vec` to empty or to one fake element where the XSD gives the child no
  `minOccurs="0"` (`DistributionSet.Element`, `ValueSetDistribution.ParameterValueSet`,
  `ParameterValueSet.ParameterAssignment`, `Stochastic.StochasticDistribution` — schema-invalid
  empty);
  `Stochastic::default()` inventing `numberOfTestRuns: 1` for a `use="required"` attribute with
  no schema default; `Axles`/`Axle::default()` picking fixed `car()`/`rear_car()` geometry
  nobody specified. Each gained an explicit `::new()`. `grep -rn "^impl Default for" src/ | wc
  -l`: 44 → 26. Harness and lib tests at baseline (172/172/172/13, 742 tests, 178 clippy
  warnings).
- **`Default` impls that invent scenario data (the two stragglers plus the entire catalog
  subtree).** `src/types/conditions/
  entity.rs:613` (`SpeedCondition::default()`, inventing `value: 10.0, rule: GreaterThan` — a
  miss inside an otherwise-complete file) and `src/types/positions/trajectory.rs`
  (`Trajectory::default()`, whose `Polyline` defaulted to zero vertices — schema-invalid empty,
  since `Schema/OpenSCENARIO.xsd`'s `Polyline` requires `minOccurs="2"`) were both removed with
  an explicit `::new()`. Nine more removed across `src/types/catalogs/{routes,controllers,
  environments,files,references,trajectories}.rs`: `CatalogRoute`/`RouteWaypoint` (fabricated
  name and an origin waypoint with `RouteStrategy::Fastest`), `CatalogController`/
  `ControllerProperty` (fabricated name/type and a `"defaultProperty"`/`"defaultValue"` pair),
  `CatalogFog` (a fabricated 100km `visualRange`), `CatalogFile`/`CatalogContent` (the
  `literal("Default` grep's namesake — `"DefaultCatalog"`/`"openscenario-rs"`),
  `ParameterAssignment` (fabricated `"defaultParam"`/`"defaultValue"`), and `CatalogTrajectory`
  (name plus the same zero-vertex
  `Polyline` as the `trajectory.rs` straggler). Removing `CatalogContent`'s `Default` surfaced
  two derived defaults piggy-backing on it — `catalogs::mod::Catalog`/`CatalogDefinition` and
  `scenario::storyboard::CatalogDefinition` — which lost `#[derive(Default)]` as well (not
  counted by the hand-written-impl grep). `src/catalog/{mod,loader,resolver,parameters}.rs`,
  `src/parser/{validation,choice_groups}.rs`, `src/builder/{scenario,catalog}.rs`, and
  `src/types/controllers/mod.rs` were verified rather than assumed benign; all 15 impls that
  remain there are individually justified in `docs/type_system_guide.md`.
  `grep -rn "^impl Default for" src/ | wc -l`: 26 → 15. Harness at baseline (172/172/172/13),
  742 lib tests, clippy 178 warnings, all unchanged.

  **This closes out the running tally, and the claim is deliberately not "full
  enforcement."** The passes above removed 174 fabricating `Default` impls (hand-written and
  derived) with zero harness regressions. A final manual sweep for
  `#[derive(Default)]` on structs with a required field — the shape the hand-written-impl grep
  cannot see — found one more confirmed live fabrication outside every file covered so far:
  `src/types/scenario/story.rs:249`'s `Actors` derives `Default` over
  a required `@selectTriggeringEntities` boolean with no XSD default, fabricating `false`, and
  `Actors::default()` is called live at `story.rs:365`. It is reported, not fixed, because
  `types/scenario/story.rs` was in none of the file sets covered. See `docs/type_system_guide.md` for
  every survivor's individual justification.
- Divergent duplicate types, folded into their canonical definitions.
- **Three of the crate's four `CatalogReference` types, folded into the one the schema
  actually declares.** This is a breaking change.
  - **`types::routing::CatalogReference`** — a route-specific struct whose
    `parameter_assignments` field lacked `#[serde(default)]`, unlike its three siblings.
    **Surviving type: `types::catalogs::references::CatalogReference<CatalogRoute>`**, now
    re-exported as `types::routing::CatalogReference` so `RouteRef::Catalog` and its call
    sites are unaffected.
  - **`types::scenario::story::CatalogReference`** — a maneuver-specific struct that named its
    `ParameterAssignments` field through the `types::controllers` re-export rather than the
    type's own module, a naming divergence with no behavioral effect since both paths resolve
    to `types::catalogs::references::ParameterAssignments`. **Surviving type:
    `types::catalogs::references::CatalogReference<CatalogManeuver>`**, re-exported as
    `types::scenario::story::CatalogReference`.
  - `Schema/OpenSCENARIO.xsd:879-885` declares one `CatalogReference` complexType, reused at
    ten element positions across the schema. `CatalogRoute` and `CatalogManeuver` already
    implement `CatalogEntity` for their catalog-loading paths, so both positions fold into the
    existing generic `CatalogReference<T>` rather than gaining a second, type-specific copy.
    `types::entities::EntityCatalogReference` is untyped by design (OSS-04) and is unaffected:
    the `EntityObject` choice group it models resolves to five different element kinds, so no
    single `T` could describe it. Four types before this change, two after: the generic
    `CatalogReference<T>` and the untyped `EntityCatalogReference`.

### Fixed

- **Three override actions accepted a document carrying two branches of a choice group, and
  discarded the second.** `OverrideBrakeAction`, `OverrideGearAction` and
  `OverrideParkingBrakeAction` (`types/actions/control.rs`) held their choice group behind
  `#[serde(flatten, skip_serializing_if = "Option::is_none")]`. Earlier passes through the
  crate's `flatten` sites enumerated them with `grep -rn 'serde(flatten)' src/`, which does not
  match an attribute that carries a second argument, hence these three were never counted and
  the claim that no attribute site remained was incomplete. `flatten` buffers an element's
  children into a map through `deserialize_any` before any child is interpreted, and a map does
  not record how often a key occurred. XSD `OverrideBrakeAction`, `OverrideGearAction` and
  `OverrideParkingBrakeAction` each wrap their group with `minOccurs="0"`, so zero branches and
  one branch are both valid while two are not, yet
  `<OverrideBrakeAction active="true"><BrakePercent value="0.5"/><BrakeForce value="9"/></OverrideBrakeAction>`
  parsed and kept only `BrakePercent`. No corpus file exercises any of the three, so no gate
  reported it. Each now hosts its choice enum behind `$value`, which reads the branch from the
  live reader and rejects the second with ``duplicate field `$value` `` while still accepting
  the empty element the schema allows. Field names and types are unchanged, hence this is a fix
  rather than a breaking change. `grep -rn 'flatten' src/` now returns only prose.
- **`UserDefinedDistribution` rejected the empty text body that the schema permits.** XSD
  `UserDefinedDistribution` is a `simpleContent` extension of `xsd:string`, which admits the
  empty string, so `<UserDefinedDistribution type="t"/>` is schema-valid and `xmllint` confirms
  it. The `$text` field carried no `#[serde(default)]`, hence that document failed with
  ``missing field `$text` ``. The field now defaults, and an absent text body reads as an empty
  string.
- **`CatalogManeuver` fabricated an empty maneuver from a document that declares no event.** XSD
  `Maneuver` declares `<Event>` with `maxOccurs="unbounded"` and no `minOccurs`, which defaults
  to one, so at least one event is required. The field carried `#[serde(default,
  skip_serializing_if = "Vec::is_empty")]`, so `<Maneuver name="m"/>` parsed into a maneuver
  with no events instead of failing. Both attributes are removed, and such a document is now
  rejected. `types/scenario/story.rs`'s `Maneuver`, which models the same schema type, was
  already correct. Note that an in-memory value holding an empty `events` still serializes to a
  `<Maneuver>` carrying no `<Event>`; a `Vec` cannot express "at least one", so that side of the
  rule is not enforceable by serde attributes.
- **No stochastic parameter distribution parsed at all, in either of the two shapes that hold a
  sequence.** `StochasticDistribution` (`types/distributions/stochastic.rs`) still carried
  `#[serde(flatten)]` around its seven-way `StochasticDistributionType` choice; the earlier pass
  through the crate's `flatten` sites missed this one. Two of the seven branches hold a `Vec`
  directly below the choice, `ProbabilityDistributionSet.elements` and `Histogram.bins`, and
  `flatten` buffers an element's children into a map through `deserialize_any` before the enum
  picks a variant, so a `Vec<T>` replayed out of that buffer failed with `invalid type: map,
  expected a sequence`, even with a single child element. Neither branch is exercised by the
  conformance corpus, so this was unreachable for as long as the type has existed with no gate
  reporting it. The wrapper now hosts its choice enum behind `$value`, the mechanism already
  used at `types/catalogs/trajectories.rs:45` and applied to the other fourteen sites in earlier
  releases; `grep -rn 'serde(flatten)' src/` now returns no attribute sites anywhere in the
  crate. The public field types and names are unchanged, hence this is a fix rather than a
  breaking change; only the rejection wording moves from `no variant of enum
  StochasticDistributionType found in flattened data` to serde's ``missing field `$value` `` and
  ``duplicate field `$value` ``. A new test file parses one and two `<Element>` children of
  `ProbabilityDistributionSet` and one and two `<Bin>` children of `Histogram`, asserts a
  byte-exact round trip including the sibling `@parameterName` attribute, and asserts the
  zero-branch and two-branch rejections. `bash scripts/gate.sh` passes all eleven stages, with
  the test total going **1513 → 1521**; clippy stays at **197**, and `report`, `lossy`,
  `validate` and `validate-input` are unchanged at 211 / 210 / 210 / 209 passing.

- **Nine more choice wrappers parse a repeated child again, and the choice-group macro is gone.**
  `VariableAction`, `VariableModifyRule`, `ParameterAction` and `ModifyRule`
  (`types/actions/wrappers.rs`), `TrafficSignalAction` (`types/actions/traffic.rs`) and
  `LaneChangeTarget`, `LaneOffsetTarget`, `LateralAction` and `FinalSpeed`
  (`types/actions/movement.rs`) still carried `#[serde(flatten)]`. Each parsed correctly today
  only because nothing in its current XSD subtree carries `maxOccurs > 1`; that immunity was a
  property of the schema as it stands, not of the construct, so any of the nine would have
  reintroduced the `invalid type: map, expected a sequence` failure the moment a repeated
  element was added beneath it. All nine now host their choice enum behind `$value`, the same
  mechanism the previous release established.
  For five of the nine — `TrafficSignalAction`, `LaneChangeTarget`, `LaneOffsetTarget`,
  `LateralAction` and `FinalSpeed` — the change also fixes a defect that was live rather than
  latent. A document giving two branches of the choice, which the schema forbids, was accepted
  under `flatten`: the first branch was kept and the second discarded without an error. Probed
  against the previous release, `<LaneChangeTarget>` holding both `<AbsoluteTargetLane>` and
  `<RelativeTargetLane>` parsed as the absolute target alone. Serde now rejects it with
  ``duplicate field `$value` ``.
  `EntityAction`, `TrafficAction`, `GlobalActionElement` and `PrivateActionElement` moved with
  them, off the private parallel-`Option` wire representation `choice_wrapper_repr!` generated
  for each. That macro reconstructed, by hand, the exactly-one-branch guarantee `$value` already
  supplies structurally: roughly 130 lines deleted for no loss of behavior. The public field
  names and types on all thirteen wrappers are unchanged, hence this is a fix rather than a
  breaking change; only the rejection wording moved from the macro's own message to serde's
  ``missing field `$value` `` and ``duplicate field `$value` ``.
  `tests/osr12_choice_wrapper_sequences_test.rs` gained a byte-exact round trip and the
  zero-branch and two-branch rejections for each of the thirteen wrappers.
  `bash scripts/gate.sh` passes all eleven stages, with the test total going **1495 → 1513** and
  `report`, `lossy`, `validate` and `validate-input` unchanged at 211 / 210 / 210 / 209 passing;
  clippy stays at **197**.

- **A controller's `<Properties>` element parses when empty, and no longer drops `<File>` or
  `<CustomContent>` children.** XSD `Properties` declares `Property`, `File` and
  `CustomContent` all at `minOccurs="0" maxOccurs="unbounded"`, so an empty `<Properties/>` is
  schema-valid and every child kind should survive a parse. Two of the crate's three
  `Properties` models disagreed: one declared only `Property` and dropped the other two
  children silently, since serde ignores element tags a struct does not name; the other
  additionally lacked `#[serde(default)]` on its lone `Vec`, turning an absent `<Property>`
  list into a parse error rather than an empty vector. Both are removed (see **Removed**) in
  favor of the one model that already had this right, `types::entities::vehicle::Properties`.
  `tests/properties_consolidation_test.rs` parses an empty `<Properties/>` through both a
  scenario `Controller` and a catalog `CatalogController`, and asserts a byte-identical round
  trip of a `<Properties>` carrying all three child kinds through `CatalogController`; before
  this change the test did not compile, since `catalogs::controllers::ControllerProperties`
  had no `files` or `custom_content` field to assert against. `bash scripts/gate.sh` passes all
  eleven stages, with the test total going **1493 → 1495**; clippy stays at **197**.

- **Entity catalog references no longer resolve to the wrong kind of entity.** `ScenarioObject`
  and `ScenarioObjectTemplate` held their `<CatalogReference>` child in
  `ScenarioEntityReference`, an `#[serde(untagged)]` enum with a `CatalogReference<CatalogVehicle>`
  variant and a `CatalogReference<CatalogPedestrian>` one. The type parameter carries no
  serialized data, so both variants deserialize from exactly the same two attributes and the
  same optional child, and an untagged enum takes the first variant that matches. Every catalog
  reference in every document was therefore parsed as a vehicle reference, and
  `pedestrian_catalog_reference()` could never return `Some` for a parsed document. The
  serialized bytes are identical either way, hence neither the round-trip gate nor the lossy
  gate could see it: the parsed document described the wrong thing while every conformance
  measure stayed green. This was silent data corruption rather than a parse failure, which is
  the worse of the two, since a parse failure is loud and a caller works around it.
  XSD `EntityObject` (`:1168-1176`) offers one `<CatalogReference>` element for every kind of
  entity, and XSD `CatalogReference` (`:879-885`) carries only `catalogName`, `entryName` and an
  optional `ParameterAssignments`. Thus the document does not record which kind of entity the
  named entry describes; that is knowable only once the referenced catalog file is read, and no
  deserializer can recover it. The field now holds the untyped `EntityCatalogReference`, and the
  entity kind is settled at catalog-resolution time instead of guessed while parsing.
  `tests/entity_catalog_reference_test.rs` pins the parsed content, a byte-exact round trip of
  the source document, and the survival of the `ParameterAssignments` sequence below the
  reference. `bash scripts/gate.sh` passes all eleven stages, with the test total going
  **1490 → 1493** and `report`, `lossy`, `validate` and `validate-input` unchanged at
  211 / 210 / 210 / 209 passing.

- **Inline routes, speed profiles and story-level global actions parse again.** Four element
  wrappers that host an `xsd:choice` still carried `#[serde(flatten)]`: `AssignRouteAction`
  (`types/actions/movement.rs`), `RouteRefElement` (`types/positions/route.rs`),
  `LongitudinalAction` (`types/actions/movement.rs`) and the story-level `GlobalAction` wrapper
  (`types/scenario/story.rs`). `#[serde(flatten)]` makes serde buffer an element's children into
  a `Content` map through `deserialize_any`. quick-xml cannot know at that point that a child
  will later be read back as a sequence, so a `Vec<T>` replayed out of that buffer failed with
  `invalid type: map, expected a sequence`, even when the element occurred only once. XSD `Route`
  requires `minOccurs="2"` `Waypoint` children, hence **no conformant inline route could be
  parsed at all**, by either of the two paths that reach one. `SpeedProfileAction` carries a
  sequence of `SpeedProfileEntry`, and the story-level `<TrafficAction>` reaches `RoadRange` with
  its two required `RoadCursor` children, so both of those branches were unreadable as well.
  Each wrapper now hosts its choice enum behind the field name `$value`, which takes the element
  name from the serialized type so that variant names become element tags and the branch is read
  from the live reader. Serde then enforces the choice cardinality structurally: a wrapper with
  no branch is rejected with ``missing field `$value` `` and one with two branches with
  ``duplicate field `$value` ``, so exactly-one is a property of the type rather than a
  convention about its use. The public field types and names are unchanged, hence this is a fix
  rather than a breaking change. `tests/choice_wrapper_value_sequences_test.rs` pins each site
  with a byte-exact round trip and with the zero-branch and two-branch rejections.
  `bash scripts/gate.sh` passes all eleven stages, with the test total going **1476 → 1488** and
  `report`, `lossy`, `validate` and `validate-input` unchanged at 211 / 210 / 210 / 209 passing.

- **A `<Catalog>` element with more than one entry of the same kind parses again.** `Catalog`
  wrapped `CatalogContent` behind `#[serde(flatten)]`, but `Catalog` in
  `Schema/OpenSCENARIO.xsd` is an `xsd:sequence` of `minOccurs="0" maxOccurs="unbounded"`
  elements plus a required `@name` attribute, and `CatalogContent` already modeled that sequence
  directly. The wrapper was therefore not expressing a choice at all; it only bought the same
  `deserialize_any` buffering that breaks a sequence below `flatten` elsewhere in this release,
  so a `<Catalog name="c">` with two `<Vehicle>` children failed with
  `invalid type: map, expected a sequence`. The corpus never exercised this path: catalog files
  load through `CatalogFile`, which holds `CatalogContent` directly and never passed through
  `Catalog`. `Catalog` and its matching `CatalogDefinition` are removed rather than reshaped —
  see **Removed** below. `bash scripts/gate.sh` passes all eleven stages, with the test total
  going 1488 → 1490 and `report`, `lossy` and `validate` unchanged at 211 / 210 / 210 passing.

- **Sequences below a choice wrapper parse again — an entire branch of `<TrafficAction>` was
  unreachable.** `#[serde(flatten)]` makes serde buffer an element's children into a `Content`
  map through `deserialize_any`. quick-xml cannot know at that point that a child will later be
  read back as a sequence, so a `Vec<T>` replayed out of that buffer failed with
  `invalid type: map, expected a sequence` — **even when the element occurred only once.** Any
  field deserialized as a sequence, anywhere below a flattened choice wrapper, was therefore
  unparseable. For `TrafficAction` this was not a corner case: XSD `RoadRange` requires
  `minOccurs="2"` `RoadCursor` children and `TrafficDistribution` requires at least one
  `TrafficDistributionEntry`, so **no conformant `TrafficAreaAction` could be written that dodged
  it.** `EntityAction`, `TrafficAction`, `GlobalActionElement` and `PrivateActionElement` now keep
  their public externally-tagged enum — which makes "no branch" and "two branches"
  unrepresentable — but move the serde wire format to a private parallel-`Option` representation
  joined by `#[serde(try_from/into)]`. The `TryFrom` is the choice-group `validate()` the crate
  convention asks for, except that it runs at parse time and cannot be forgotten; it rejects
  all-`None` and multiple-`Some` with the same pair of messages `GlobalAction::validate` returns.
  The public field types are unchanged, so this is a fix rather than a breaking change.
  `VariableAction`, `VariableModifyRule`, `ParameterAction` and `ModifyRule` keep the flatten
  construct: their entire XSD subtree contains no element with `maxOccurs > 1`, so nothing below
  them can ever deserialize as a sequence. `tests/osr12_choice_wrapper_sequences_test.rs` pins
  that reasoning, so adding a `Vec` under one of them fails loudly.
  `conformance/corpus/openscenario1-engine/.../traffic_area_action_test_scenario.xosc` — the only
  corpus file containing a `<TrafficAction>`, and the sole supplier of ten element declarations
  including `RoadCursor`, `RoadRange`, `TrafficArea` and `EntityDistribution` — now passes;
  `report` goes **210 passed / 2 excluded → 211 passed / 1 excluded** and its `crate-defect` entry
  in `conformance/expectations.toml` is deleted.

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
- **`DistributionDefinition` and `DistributionDefinitionGroup` could not serialize their
  `Deterministic` branch, and could not have deserialized the schema's own element names
  either.** XSD group `DistributionDefinition` (`Schema/OpenSCENARIO.xsd:1086-1091`) is a choice
  between the `Deterministic` element (type `Deterministic`, an unbounded sequence) and the
  `Stochastic` element (type `Stochastic`). Both Rust enums instead typed the `Deterministic`
  branch as `DeterministicParameterDistribution`, the *different* group nested one level down,
  inside `<Deterministic>`'s own sequence, choosing between a single item and a multi item.
  Nesting one externally-tagged enum inside another does not serialize under quick-xml: it
  failed with `Unsupported("cannot serialize enum newtype variant ...")` before emitting any
  tag, and deserializing the schema's actual inner element name
  (`DeterministicSingleParameterDistribution`) failed with `unknown variant`, since the group's
  own variants are named `Single`/`Multi`. Both enums now hold `Deterministic`/`Stochastic`
  directly. `DistributionDefinition` also carried a third `UserDefined` variant absent from the
  schema's choice; it is removed (see **Removed**, `DeterministicParameterDistribution`, for the
  wrongly named group this leaves behind). Neither enum is reachable from any parsed document
  today — the crate's real `<Deterministic>` support is the hand-written `Serialize`/
  `Deserialize` pair on `Deterministic` itself — so this closes a latent gap rather than a live
  one. `tests/distribution_definition_element_names_test.rs` pins the schema element name for
  both enums and asserts the pre-fix `Unsupported`/`unknown variant` failures against the
  previous shape. `bash scripts/gate.sh` passes all eleven stages; the test total goes
  **1556 → 1559**; clippy stays at **197**; `report`, `lossy`, `validate` and `validate-input`
  are unchanged at 211 / 210 / 210 / 209 passing.
- **The conformance gates are runnable from a fresh checkout.** They previously lived in an
  untracked sibling directory that CONTRIBUTING.md and docs/xsd_gaps.md described but a clone
  of this repository did not contain. The repo is now a Cargo workspace; `conformance` is a
  member crate holding the corpus binaries (`report`, `lossy`, `validate`, `builder`) that were
  previously external, and `scripts/fetch-corpus.sh` fetches the (still unvendored, third-party)
  corpus on demand. No change to `openscenario-rs`'s public API.
- **Nine required, repeated child elements fabricated an empty list when absent.**
  `UsedArea.position`, `SpeedProfileAction.entries`, `VehicleRoleDistribution.entries`,
  `VehicleCategoryDistribution.entries`, `ControllerDistribution.entries`,
  `ClothoidSpline.segments`, `Nurbs.control_points`, `Nurbs.knots` and `Polyline.vertices` each
  carried `#[serde(default)]` on a `Vec` whose XSD element has `minOccurs` of one or two, so a
  document missing the required children, such as `<Polyline/>`, parsed into an empty vector
  instead of failing, and the crate went on to re-emit the same schema-invalid element on
  serialization. `default` on a `Vec` is the correct attribute only when `minOccurs="0"`; here it
  substituted a fabricated empty collection for a required one. All nine now carry no `default`,
  so the missing-children document is rejected with `missing field` naming the element, for
  example ``missing field `Vertex` `` for `Polyline`. This closes the zero-children case only;
  the `minOccurs="2"` bound on `UsedArea.position`, `Nurbs.control_points`, `Nurbs.knots` and
  `Polyline.vertices` still admits one child where the schema requires two, because no serde
  attribute expresses a lower bound above one, and that gap is tracked separately. A new test
  file parses each of the nine types from its empty element and asserts the exact `missing
  field` text, one assertion per test. `bash scripts/gate.sh` passes all eleven stages; the test
  total goes **1559 → 1568**; clippy stays at **191**; `report`, `lossy`, `validate` and
  `validate-input` are unchanged at 211 / 210 / 210 / 209 passing. No change to
  `openscenario-rs`'s public API.

### Known gaps

**`<TrafficAction>` does not deserialize.** The corpus expansion above brought the first corpus
file containing one, and the crate rejects it with `invalid type: map, expected a sequence`. It is
the one `crate-defect` entry in `conformance/expectations.toml`, and ten element declarations are
present in the corpus but unexercised solely because of it.

The parameterized-enumeration gap listed here through pass 4 is closed — see *90 enum-typed
attributes now accept parameter references* under **Changed**.
See [docs/xsd_gaps.md](docs/xsd_gaps.md) for the full ledger.

## [0.4.2], [0.4.1], [0.4.0] - shipped without changelog entries

These three versions are live and immutable on crates.io. No release notes were written when
they shipped, and none are reconstructed here. Everything below is transcribed from `git` and
from the crates.io API; there is deliberately no `Added`/`Changed`/`Fixed` breakdown, because
no such record exists and inventing one would be worse than the gap. For what a release
actually contains, run `git log --oneline` over its commit range.

Sources: publication dates are `versions[].created_at` (UTC) from
`https://crates.io/api/v1/crates/openscenario-rs`. The commit each release was built from is
the `git.sha1` that `cargo publish` recorded in the published crate's own
`.cargo_vcs_info.json`, read out of the `.crate` tarball served by crates.io.

| Version | Published (UTC) | crates.io recorded | Tagged at | Commit range |
|---|---|---|---|---|
| [0.4.2] | 2026-09-16 | `334d35e` | `334d35e` | `be28848..334d35e` |
| [0.4.1] | 2026-09-09 | `67b76b1` | `be28848` | `5ae051d..be28848` |
| [0.4.0] | 2026-09-09 | `b6af9c1` | `5ae051d` | `cade907..5ae051d` |

Why two rows are tagged somewhere other than what crates.io recorded: this repository's history
was rewritten (the `backup/*-pre-rewrite` branches predate the rewrite), so two of those SHAs
are no longer reachable from `main`.

- `67b76b1` is in the repository but not on `main`. The commit `be28848` on `main` has a
  byte-identical tree (`git rev-parse 67b76b1^{tree} be28848^{tree}`), and every file in the
  published 0.4.1 `.crate` matches it, so `v0.4.1` is tagged at `be28848`.
- `b6af9c1` is not in this repository at all: `git cat-file -t b6af9c1` fails, and the SHA
  appears in no pack, no loose object, no reflog and no `refs/original/*` entry. It is
  **not lost**, only absent here — GitHub still serves it, because it retains commits that a
  force-push made unreachable. All three of
  `gh api repos/RI-SE/openscenario-rs/commits/b6af9c1f9807df2a3c8b92810665dad5f3ced011`,
  `https://github.com/RI-SE/openscenario-rs/commit/b6af9c1f9807df2a3c8b92810665dad5f3ced011`
  and `https://github.com/RI-SE/openscenario-rs/archive/b6af9c1f9807df2a3c8b92810665dad5f3ced011.tar.gz`
  resolve, the last one with the complete source tree — and the commit itself can be fetched
  straight back into a clone by its full SHA, which is the recipe at the end of the next
  section. Treat that retention as a convenience, not as an archive guarantee: the published
  `.crate` on crates.io is the only immutable copy.
  `5ae051d` is the surviving equivalent — same author timestamp to the second, same subject,
  and the two trees differ in exactly one file, `docs/xsd_gaps.md`, which `b6af9c1` and the
  published 0.4.0 `.crate` both contain and the rewritten commit does not. That file was lost
  in the rewrite and later recovered by `09dc499`. (`Cargo.lock` also differs between the
  tarball and any commit, because `cargo package` regenerates it.) `v0.4.0` is tagged at
  `5ae051d`.
- The 0.4.2 bump commit is `6195199`; the merge commit `334d35e` two commits later is what was
  actually published, and it also carries `version = "0.4.2"`. They package identically — the
  only difference between them, `tools/scenario_converter.rs`, is not included in the crate —
  so `v0.4.2` is tagged at `334d35e`, the SHA crates.io recorded.
- The lower bound of the 0.4.0 range is `cade907`, the surviving equivalent of the commit 0.3.2
  was published from. crates.io recorded 0.3.2 as `0474f61`, which is also absent from this
  repository; `cade907` has the same author timestamp, the same subject and a tree with **zero**
  differing files. An earlier revision of this entry named `d9ac1ba` and called the bound
  approximate — `d9ac1ba` is two commits further back and four files away, and was wrong.
  The 0.3.x provenance table below has the derivation.

These tags are local and retroactive. They carry no GitHub Releases, because there are no notes
to put in them.

## [0.3.2] and earlier

No changelog was kept before this release. What can still be established is provenance: which
commit each of the three 0.3.x releases was built from, and where that commit went.

**Every 0.3.x release was published from a commit the history rewrite dropped.** None of the
three SHAs `cargo publish` recorded resolve in this repository — `git cat-file -t` fails on each,
and none of them is in a pack, a loose object, a reflog or a `refs/original/*` entry. Each does
have a surviving counterpart on `main`, identified by matching the author timestamp to the second
and the subject line, and confirmed by diffing the GitHub source archive for the recorded SHA
against `git archive` of the counterpart. For all three the trees are identical — no file differs.

| Version | Published (UTC) | crates.io recorded | Resolves here | Surviving equivalent on `main` | Files differing |
|---|---|---|---|---|---|
| 0.3.0 | 2026-04-22 | `1dbb5ac` | no | `0163980` "corrected license name" | 0 |
| 0.3.1 | 2026-05-07 | `444f142` | no | `5d8a860` "bump" | 0 |
| [0.3.2] | 2026-06-10 | `0474f61` | no | `cade907` "fix(catalog): fix ParameterDeclaration serde mapping and add wrapper type" | 0 |

0.4.0's recorded commit `b6af9c1` is absent for the same reason and is covered in the 0.4.x
section above; 0.4.1's recorded `67b76b1` is in the repository but reachable only from the
`backup/*-pre-rewrite` branches, not from `main` or any tag. So of the six published versions,
five were built from a commit that is not on `main` today — the four this repository does not
have at all, plus 0.4.1's — and only 0.4.2's `334d35e` is still an ancestor of `main`.

All four absent SHAs are still served by the GitHub remote: as a web page, as
`gh api repos/RI-SE/openscenario-rs/commits/<sha>`, as
`https://github.com/RI-SE/openscenario-rs/archive/<sha>.tar.gz`, and to `git fetch origin
<sha>`, which — because the remote serves any object it holds, reachable from a branch or not
— brings the commit itself back rather than a snapshot of its tree. Shallow and full fetches
both work. **The SHA must be written out in full**; the abbreviations in the tables above are
rejected with `fatal: couldn't find remote ref`.

To reproduce any of this, or to recover one of the commits:

```sh
# what the published artifact says it was built from
curl -sL https://static.crates.io/crates/openscenario-rs/openscenario-rs-0.3.2.crate \
  | tar -xzO openscenario-rs-0.3.2/.cargo_vcs_info.json

# whether that commit is in this repository
git cat-file -t 0474f61

# the recorded commit's tree, from the remote, against the surviving equivalent
curl -sL https://github.com/RI-SE/openscenario-rs/archive/<full-sha>.tar.gz | tar -xz
git archive --format=tar --prefix=survivor/ cade907 | tar -x
diff -rq openscenario-rs-<full-sha> survivor

# recover the commit itself, into a scratch clone rather than into this repository
git init -q recovered && cd recovered
git remote add origin https://github.com/RI-SE/openscenario-rs.git
git fetch --depth=1 origin 0474f61b23b411ee1e139d394269aad455fdacab
git log --oneline -1 FETCH_HEAD
```

Nothing has been retagged, re-published or yanked to close this gap; the record is the fix.

[Unreleased]: https://github.com/RI-SE/openscenario-rs/compare/v0.4.2...HEAD
[0.4.2]: https://github.com/RI-SE/openscenario-rs/compare/v0.4.1...v0.4.2
[0.4.1]: https://github.com/RI-SE/openscenario-rs/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/RI-SE/openscenario-rs/tree/v0.4.0
[0.3.2]: https://crates.io/crates/openscenario-rs/0.3.2
