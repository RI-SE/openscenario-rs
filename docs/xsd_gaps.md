# Conformance vs the bundled OpenSCENARIO XSD

This document tracks the relationship between `openscenario-rs`'s Rust types and
`Schema/OpenSCENARIO.xsd`. It records what the test suite does and does not prove,
the audit methods that find gaps, and the history of what has been fixed.

**Current state (pass 5, 2026-09):** the corpus round-trips with nothing dropped,
nothing invented, and every output schema-valid. The parameterized-enumeration gap that
pass 4 identified is **closed** — see *Fixed in pass 5* below. No structural conformance
gap is currently open.

## Repository layout

The gates below live in a workspace member, not a sibling crate:

```
openscenario-rs/            # workspace root
├── Cargo.toml               # [workspace] members = ["conformance"]
├── src/                     # this crate, openscenario-rs
└── conformance/              # openscenario-roundtrip-harness – the corpus and the four gates
```

The harness depends on this crate by path
(`openscenario-rs = { path = "..", features = ["validation"] }`), so it always exercises
the working tree rather than a published version. Every `cargo run -p openscenario-roundtrip-harness
--bin …` command on this page is run from the repo root. The corpus itself is not vendored (it is
third-party MPL-2.0 content and this repo is GPL-3.0-only); fetch it once with
`bash scripts/fetch-corpus.sh`. Nothing in the main crate is required to have the corpus present;
the bundled `xosc-validate` binary validates individual files without it.

## What the harness proves, and what it does not

Four independent checks run against the corpus in `conformance/corpus`. All four exit
non-zero on failure, so they can be gated on `$?` rather than by reading output. Only the
first three run in `scripts/hooks/pre-push`; `builder` does not, because it exercises
`src/builder/` rather than the parser and has no corpus fixtures of its own — run it by
hand for any change under `src/builder/`.

| Binary | Compares | Catches |
|---|---|---|
| `cargo run -p openscenario-roundtrip-harness --bin report` | `xml1` vs `xml2` | parse failures, and instability across serialization |
| `cargo run -p openscenario-roundtrip-harness --bin lossy` | **original file** vs `xml1` | data dropped or invented on the *first* parse |
| `cargo run -p openscenario-roundtrip-harness --bin validate` | `xml1` vs **the XSD** | schema-invalid output: invented fields, mis-ordered sequences, empty choice groups |
| `cargo run -p openscenario-roundtrip-harness --bin builder` (not in the pre-push gate) | every builder fixture, through the same three questions | the builder API producing non-schema-valid or lossy output, using code as the corpus instead of files |

### The corpus covers about half the schema

This bounds every green result above, and is the most important caveat on this
page. The corpus is two narrow scenario families and exercises **156 of the
schema's 294 element declarations – 53%**. It contains no `<WorldPosition>` at
all, and no `Animation*`, `TrafficDistribution`, `EntitySelection`, `Nurbs`,
`Clothoid` or `MonitorDeclaration`.

So "0 dropped items" means *no reachable data loss*, not *no data loss*. Types
added from schema reading alone – much of the light, animation and traffic
distribution work in pass 3 – have no corpus coverage, and for several of them
no unit test either. For those, the schema and a careful reading are the only
checks that have ever run.

The 138 unexercised elements are a known, deliberate gap. Closing it means
authoring synthetic fixtures, which no pass has done yet.

The `report` check alone is not sufficient, and this is worth understanding before
trusting a green run. `check_roundtrip` (`conformance/src/lib.rs`) parses, serializes to
`xml1`, reparses, serializes to `xml2`, and asserts `xml1 == xml2` – a round-trip
*fixed point*. serde ignores unknown XML attributes and elements by default, so any
field the Rust types do not model is dropped on the first parse, dropped identically
on the second, and the comparison still succeeds.

For a long time the corpus reported 172/172 while silently discarding route
positions, vehicle light states, steady-state targets and global actions. **A green
`report` run means "stable", not "lossless".** `lossy` is what makes the difference
observable:

```
pass 3 start:   151 lossless, 21 lossy   380 dropped items, 0 invented
pass 3 end:     172 lossless,  0 lossy     0 dropped items, 0 invented
pass 4:         172 schema-valid, 0 invalid, 0 validation errors
```

The `validate` gate found nothing on its first run. That is a real result, not a
gate that cannot fail: the 172 outputs were independently cross-checked with
`xmllint`, and the gate was confirmed to report 172 invalid when an element is
injected into the serialized output. Note that injecting into an *input* file
proves nothing – the crate drops unknown elements at parse, so input tampering
never reaches the output. Its value is as a regression guard.

Both must stay green. If you add a type and `lossy` does not move, that is expected
when the corpus does not exercise it – but it also means the corpus cannot validate
your work, and unit tests are the only check.

## Audit methods

These found the pass-3 gaps and are cheap to re-run. Note the shell caveat: `grep -rl`
and `tr` are shadowed by a `trash-restore` alias in some setups, so prefix with
`command`.

1. **Lossiness report**: `cargo run -p openscenario-roundtrip-harness --bin lossy -- -v`. The only check that
   sees first-parse data loss. Ranks dropped paths corpus-wide, so the highest-value
   gap is always at the top. This is the single most useful tool here.
2. **Corpus attributes vs serde renames**: the set of `attr="` names appearing in the
   corpus, minus the `#[serde(rename = "@…")]` names in `src/`. Anything left is either
   XML plumbing or a real gap. At the start of pass 3 it returned `laneOffset`, `mode`,
   `pathS`, `vehicleLightType`; all four were genuine. It now returns only `encoding`,
   `version`, `xsi`, `noNamespaceSchemaLocation`.
3. **Type-name diff**: XSD `complexType` names vs Rust `pub struct`/`pub enum` names.
   Went from 37 unmatched to 5, all of which are deliberate Rust-side renames whose wire
   names are correct: `GeoPosition`→`GeographicPosition`, `InitActions`→`Actions`,
   `None`→`NoneElement`, `Story`→`ScenarioStory`,
   `TimeToCollisionConditionTarget`→`TimeToCollisionTarget`.
   **Trap:** XSD *type* names are not XML *element* names (`TransitionDynamics` appears
   as `<SpeedActionDynamics>`), so never diff type names against corpus element names.
4. **Orphaned enums**: an enum in `enums.rs` referenced by no field is a strong signal
   that a whole struct cluster is missing. `ColorType`, `LightMode`, `VehicleLightType`,
   `VehicleComponentType`, `PedestrianGestureType` and `PedestrianMotionType` were all
   orphaned, and all six pointed at the unmodelled light and animation subtrees. This
   sweep now returns nothing.
5. **Enumeration variants**: all 37 XSD enumeration `simpleType`s have a same-named Rust
   enum, so only variant-level drift is possible. It is common and worth re-checking:
   pass 3 found nine wrong enums, one of which (`VehicleLightType`) had only 3 of 13
   values right. As of pass 4 all 37 match exactly, in both directions.
6. **Simple-type *structure*, not just variants** – added in pass 4, after a variant-level
   sweep came back perfectly clean while a real gap sat one level up. Every one of those
   37 enumerations is an `xsd:union` with a `parameter` member, which no bare Rust enum can
   hold; pass 5 closed that by wrapping all 90 such fields in `Value<E>`. When checking a
   `simpleType`, read what it is (union? restriction? what base?) *and which members it
   lists* before comparing the values inside it — the enumeration unions admit `parameter`
   but not `expression`, and reading only "it is a union" was enough to get the sigil wrong.
7. **Output validation**: `cargo run -p openscenario-roundtrip-harness --bin validate`. The only check that
   consults the schema. Catches invented fields, mis-ordered `xsd:sequence` children and
   empty choice groups, none of which the other two gates can see. When testing that this
   gate works, corrupt the *serialized output*, never an input file: unknown elements are
   dropped at parse and never reach the output.

## Open gaps

None currently known. The parameterized-enumeration gap that stood here through pass 4 was
closed in pass 5; the entry moved to *Fixed in pass 5* below.

## Known deviations (deliberate)

- Five types carry Rust names that differ from their XSD type name, listed in method 3
  above. Their **wire** names are correct; only the Rust identifiers differ.
- `Value<T>` wraps every parameterizable attribute — scalar **and** enumeration since
  pass 5 — so `$param` (and, on the scalar unions, `${expr}`) references survive
  round-trips. This has no XSD counterpart – it is how the crate represents the schema's
  `parameter` union member.
- **Parameter references are emitted unbraced.** The schema defines `parameter` as
  `[$][A-Za-z_][A-Za-z0-9_]*` and `expression` as `[$][{]…[\}]` (two separate
  `simpleType`s, lines 4–13). Every scalar union lists both members, but all 37
  enumeration unions list `parameter` alone, so `vehicleCategory="${cat}"` is
  schema-*invalid* while `vehicleCategory="$cat"` is valid. `Value::Parameter` therefore
  serializes as `$name`, which validates on every union in the schema;
  `Value::Expression` keeps the braced form. Deserialization accepts either spelling.
- Deprecated-but-schema-valid attributes are modelled rather than dropped
  (`@alongRoute`, `@velocity`, `Pedestrian.@model`, `cartesianDistance`,
  `RelativeSpeedToMaster` and friends), since real files still emit them.

## Fixed in pass 5 (2026-09)

**Parameterized enumeration attributes — closed.** This was pass 4's one open gap and the
largest known conformance defect: every one of the schema's 37 enumeration `simpleType`s is
an `xsd:union` whose second member is `<xsd:restriction base="parameter"/>`, so
`<Vehicle vehicleCategory="$cat">` is schema-valid, and the crate — modelling each of the
**75 XSD attributes** so declared as a bare Rust enum — rejected all of them outright. In
Rust terms that was **90 fields across 25 files** (54 bare, 36 `Option<>`; the count exceeds
75 because the catalog twins duplicate several). The pass-4 ledger counted 89 — the 90th,
`VehicleRoleDistributionEntry::role` in `src/types/actions/traffic.rs`, was written
fully-qualified as `crate::types::enums::Role` and had escaped every previous sweep, which is
its own small lesson about grepping for type names. All 90 now hold `Value<E>` /
`Option<Value<E>>`, migrated in one change so that no half-migrated state exists and the
catalog twins moved in lockstep with their scenario counterparts.

The fix reuses what was already written rather than adding a parallel wrapper: `Value<T>`'s
serde impls work for any `T: FromStr + Display`, and pass 5's predecessor folded all 37
enums into the `osc_enum!` macro so each has a verified `FromStr`/`Display` generated from
the same variant→wire-name table as its `#[serde(rename)]`. Extending `Value<T>` also lights
up `Resolve<T>` and `CatalogParameterSubstitution::resolve_value` for enums for free.

`Value<E>` is not a stringly-typed escape hatch: deserialization falls through to
`s.parse::<T>()` for anything without a `$` sigil, so `vehicleCategory="spaceship"` is still
a hard parse error, pinned by a test.

**The `parameter` sigil was wrong on output, and only the enums exposed it.** Reading the
XSD to write the fixture surfaced a second defect the issue had not anticipated:
`Value::Parameter` serialized as `${name}`, but `${…}` matches the schema's `expression`
production, not `parameter`. The scalar unions list both members, which is why the braced
form had validated for four passes; the enumeration unions list `parameter` alone, so the
braced form would have produced schema-invalid XML on every attribute this pass migrated —
with a *green* round trip, because `Value<T>` serializes and deserializes through the same
string. `Value::Parameter` now emits `$name`, which is valid on every union in the schema;
`Value::Expression` is unchanged. `src/builder/validation.rs`'s undeclared-parameter rule,
which scanned only for `${`, was widened to both spellings.

**Why no gate caught any of this, and what now does.** Zero corpus files parameterize an
enum attribute, so `report`, `lossy` and `validate` were all green on code that rejected
schema-valid input — the coverage caveat above, made concrete for a second time. A
variant-level audit could not see it either: all 37 enums match their XSD enumerations
exactly, and the gap sat one level up in the union wrapper. The corpus is fetched and
gitignored, so it cannot be extended. The fixtures therefore live in `tests/data/`:
`parameterized_enums.xosc` (a required bare attribute, an optional one, a condition `@rule`,
a `@coordinateSystem`) and `parameterized_enums_catalog.xosc`, both confirmed schema-valid
with `xmllint --schema` and both failing to parse on the pre-change tree.
`conformance/tests/parameterized_enums.rs` drives them through the same round-trip and
schema-validation questions the corpus binaries ask, so the gates cover this class going
forward; `tests/parameterized_enum_test.rs` pins parse, resolve, and the escape-hatch guard.

All four corpus binaries were byte-identical before and after the migration (172/172 on
`report`, `lossy` and `validate`; 13/13 on `builder`), which is the evidence that routing
every literal enum value through `Value::Literal` and `Display` is lossless.

## Fixed in pass 4 (2026-08)

Pass 4 started from a saturated position: both existing gates read zero, so neither
could find anything more. The work was to build an instrument that could, then audit
both directions of the XSD↔code mapping exhaustively.

**A third gate.** `XsdValidator` was hoisted out of `tools/xosc_validate.rs` into
`src/validation.rs` behind a `validation` feature – which `lib.rs` had advertised for
some time while `Cargo.toml` never defined it. The harness now validates the crate's own
serialized output against the schema, closing the one bug class neither other gate can
see even in principle: `report` compares output to output, `lossy` compares output to
input, and neither consults the schema. `lossy` and `validate` also now exit non-zero on
failure, so "verify by exit code" is true rather than aspirational.

**The `<Init>` subtree was dropping actions silently.** `init::GlobalAction` modelled 1
of the 7 branches XSD line 1282 declares; `init::PrivateAction` modelled 8 of 10. With no
`deny_unknown_fields` and every branch an `Option`, an
`<Init><Actions><GlobalAction><EntityAction>` deserialized to an all-`None` struct and
re-serialized as an empty `<GlobalAction/>`, violating the choice group: silent loss on
input and invalid output. Both now cover every branch, reusing the types
`actions::wrappers` already had right. No gate moved – the corpus does not exercise these
branches – so 13 round-trip tests are the only evidence the fix works.

**Five types modelled nothing in the schema, and two were the ones publicly exported.**
`controllers::ControllerDistribution` required an attribute and a child that XSD line 990
does not declare, so a valid element could not deserialize into it;
`controllers::ActivateControllerAction` required `@controllerRef`, which line 714 marks
optional and deprecated, and omitted five more attributes;
`controllers::ControllerAssignment` and its `@targetEntity` appear nowhere in the schema.
Correct versions of the first two already existed in `actions::`, and the document tree
already used them – the damage was `types/mod.rs` re-exporting the broken copies and
shadowing them for downstream users. `positions::RoadCoordinate`/`LaneCoordinate` went
too: no XSD type of either name, and nothing held one.

**Nine attributes were stringly typed where the schema names an enumeration**, so invalid
values round-tripped silently. `RelativeWorldPosition` gained the optional `Orientation`
child from line 1912 – the sole missing member across all 287 complex types.

**Fifteen tests were deleted** because they constructed the removed types, pinning
invented shapes rather than testing behaviour. This is the third pass to find tests of
that kind.

**Audit results, both directions.** XSD→Rust: all 287 complexTypes have a representation,
every `maxOccurs="unbounded"` maps to a `Vec`, no required member is an `Option`, no
attribute is missing its `@` prefix. Rust→XSD, across 1,264 serde renames: zero invented
attributes on correctly-named types, zero invented enum variants, zero over-loose
optionality, zero cardinality violations. Both directions are now closed **except** the
parameter-union gap documented above, which neither audit could see.

## Fixed in pass 3 (2026-08)

A full schema-vs-code audit found ~90 gaps in four categories. All are fixed across 14
commits; the corpus went from 380 dropped items to 0.

**Corpus-visible data loss (the 380 items).**
- `Position` was missing the `RoutePosition` branch entirely, so six corpus files parsed
  a route position into an all-`None` `Position`. Added, with `InRoutePosition` and its
  three coordinate types. Largest single fix: 290 items.
- `LightStateAction` and `AnimationAction` were empty structs `{}` while reachable from
  the tree; four corpus files set vehicle lights that round-tripped to nothing. Both
  clusters modelled in full (25 items).
- `StoryAction` modelled only `PrivateAction` of a three-way choice, dropping
  `GlobalAction` from events – a `DeleteEntityAction` and a `VariableAction` in the
  corpus (22 items).
- `AbsoluteSpeed`/`RelativeSpeedToMaster` omitted the `SteadyState` group (32 items).
- `CatalogWeather` modelled `fractionalCloudCover` as an element where the schema
  declares an attribute, and omitted `Wind`/`DomeImage` (11 items).

**Parse failures on schema-valid XML.** Nine enums disagreed with their `simpleType`
(`CoordinateSystem` lacked `world`; `AutomaticGearType` used `park/reverse/neutral/drive`
for `n/p/r/d`; `VehicleLightType` had 3 of 13 values right). Wrong element and attribute
names: `ManualGear.@gear` for `@number`, `Histogram`'s `HistogramBin` for `Bin`,
`SpeedProfileAction`'s `Entry` for `SpeedProfileEntry`, `GeographicPosition` for
`GeoPosition`, and `ByObjectType`/`ByType` with their attributes swapped. Invented
required fields on `TrafficStopAction`, `SetMonitorAction`, `MonitorDeclaration`,
`ExternalObjectReference`. `ValueConstraint` modelled attributes as elements.
`UserDefinedDistribution` did not bind its text body.

**Structural.** `VariableAction`/`ParameterAction` named their choice branches for types
instead of elements and flattened away the required `Rule` wrapper. `ControllerAction`
hoisted the six override children up a level and renamed them, so
`OverrideControllerValueAction` could not round-trip. `TrafficArea` was fabricated
(`<Vertex x y z>`), where the schema has `Polygon | RoadRange+`; `TrafficAreaAction`
could not parse a valid element at all. `EntitySelection`, `SelectedEntities` and
`ScenarioObjectTemplate` were rebuilt to their schema shapes, and `EntitySelection` wired
into `Entities`. The whole traffic-distribution subtree was added. `CustomCommandAction`
and `EnvironmentAction` were empty placeholders. `RoutingAction` regained
`AcquirePositionAction`/`RandomRouteAction`.

**Broken feature.** The four typed catalog loaders deserialized into root elements that
do not exist (`<ControllerCatalog revMajor=…>`), so they could not read any valid catalog
file; their directory wrappers hid the failure behind `if let Ok(…)` and returned empty
maps. They now parse the real `OpenSCENARIO`/`Catalog` document, and genuine parse errors
propagate instead of vanishing.

**Invented fields removed** (strict XSD purity, continuing the pass-2 precedent):
catalog waypoint extras and `LaneConstraints`, the catalog `RouteRef`,
`CatalogEnvironment.road_network`, `StochasticDistribution.@randomSeed`,
`ValueSetDistribution.@assignmentAuthor`, `RandomRouteAction`'s attributes,
`EntitySelection.ByName`, `ScenarioObjectTemplate`'s `TemplateProperties`,
`FollowRouteAction` (absent from the schema entirely) and the invented
`controllers::OverrideControllerValueAction`. Six divergent duplicate types were deleted
in favour of their correct counterparts.

Three tests were found to be **pinning bugs rather than testing behaviour**: one
asserting a `CustomCommandAction` without its required attribute, one preserving
`FollowRouteAction`'s wire format while its own comment noted the type does not exist,
and twelve asserting the invented catalog root shapes.

## Fixed in passes 1–2 (2026-08)

Corrections to earlier revisions of this document are marked (†).

- `controllers::ParameterAssignments`/`ParameterAssignment` had no serde renames, failing
  on every `<ParameterAssignments>` under a scenario `CatalogReference`. Four
  near-duplicate definitions were consolidated into `catalogs::references`.
- `Actors.entity_refs` and `ManeuverGroup.maneuvers` gained `#[serde(default)]`;
  `TriggeringEntities` was deliberately left strict (the XSD requires ≥1).
- `TrajectoryRef` in `positions/` was an `@trajectory` attribute struct; it is now the
  XSD choice, and `TrajectoryPosition` gained its required child.
- `AngleType` had invented variants `relative`/`absolute`; the XSD defines
  `heading`/`pitch`/`roll`.
- Placeholder `CatalogEnvironment`/`CatalogTrajectory`/`CatalogRoute` were removed so
  inline catalog entries stopped dropping weather, shape and waypoint data.
- `CatalogMiscObject` gained its required attributes († an earlier revision claimed this
  was already fixed); `CatalogManeuver` gained its `Event` sequence.
- Scenario-level `MiscObject`, `ExternalObjectReference` wiring, and the recursive
  `Trailer` element on `Vehicle` were added.
- Invented `Default` impls removed: `Controller::default()`, `OpenScenario::default()`,
  `Pedestrian::default()` († an earlier revision claimed this was already removed), and
  `SpeedCondition`'s "DefaultEntity".
- `ActivateControllerAction.parameter_assignments` and `SpeedCondition.entity_ref`
  removed as non-schema extensions.
- `ParameterDeclarationsBlock` replaced by `basic::ParameterDeclarations` across the
  catalog entity types, fixing silent loss of `<ConstraintGroup>` children.
- `CatalogEntity::ResolvedType` is no longer a `String` stub for any impl.
- `ScenarioObject.object_controller` became `Vec<ObjectController>` per the XSD's
  `maxOccurs="unbounded"`.
