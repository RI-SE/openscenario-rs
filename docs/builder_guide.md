# Builder guide

The builder API constructs OpenSCENARIO documents programmatically, behind the `builder`
cargo feature. It exists because assembling an `OpenScenario` by hand means populating a deep
tree of nested `Option` fields in the right order, which is tedious and easy to get subtly
wrong.

```toml
[dependencies]
openscenario-rs = { version = "0.3.2", features = ["builder"] }
```

Signatures in this guide are taken from `src/builder/`; the runnable examples referenced
throughout are in `examples/`.

## Quick start

```rust
use openscenario_rs::types::catalogs::locations::CatalogLocations;
use openscenario_rs::types::road::RoadNetwork;
use openscenario_rs::ScenarioBuilder;

let scenario = ScenarioBuilder::new()
    .with_header("Basic Highway Scenario", "Builder Demo")
    .with_catalog_locations(CatalogLocations::default())
    .with_road_network(RoadNetwork::default())
    .with_entities()
    .with_storyboard(|sb| sb)
    .build()?;

let xml = openscenario_rs::serialize_to_string(&scenario)?;
```

Four things are required of a scenario document and `build()` rejects a chain missing any of
them: the header, `CatalogLocations`, `RoadNetwork`, and the storyboard. The last three are
runtime errors rather than compile errors – see [Required elements](#required-elements).

## Typestate

`ScenarioBuilder<S>` carries a phantom state parameter, and the methods available depend on
it. The point is that a document missing its header or its entities is not representable – the
transition methods are the only way forward, so an incomplete scenario fails to compile rather
than failing at runtime.

```
Empty ──with_header──▶ HasHeader ──with_entities──▶ HasEntities ──with_storyboard──▶ Complete
```

`build()` is exposed on both `HasEntities` and `Complete`, but the two implementations are
identical and both enforce the same requirements.

### Required elements

The XSD group `ScenarioDefinition` (`Schema/OpenSCENARIO.xsd:1989`) declares four elements
without `minOccurs="0"`, so a scenario document must carry all four. `build()` returns
`BuilderError::MissingField` for each one it does not find:

| Element | Set it with | Enforced by |
|---|---|---|
| `FileHeader` | `.with_header(description, author)` | the typestate |
| `Entities` | `.with_entities()` | the typestate |
| `CatalogLocations` | `.with_catalog_locations(...)` | `build()`, at runtime |
| `RoadNetwork` | `.with_road_file(path)` or `.with_road_network(...)` | `build()`, at runtime |
| `Storyboard` | `.with_storyboard(...)` | `build()`, at runtime |

A scenario that references no catalogs and names no road file still has to emit both elements,
because every *child* of each is optional but the elements themselves are not. Pass
`CatalogLocations::default()` and `RoadNetwork::default()` for that case: they serialize as
empty elements, which is schema-valid and states nothing.

Omitting either used to produce schema-invalid XML with no error at all, which is what
motivated the check.

| State | Available methods |
|---|---|
| `Empty` | `new`, `with_header` |
| `HasHeader` | `with_revision`, `with_parameters`, `add_parameter`, `with_catalog_locations`, `with_road_network`, `with_road_file`, `with_entities` |
| `HasEntities` | `add_vehicle`, `add_vehicle_mut`, `add_catalog_vehicle`, `add_pedestrian`, `add_catalog_pedestrian`, `with_storyboard`, `with_storyboard_mut`, `create_storyboard`, `build` |
| `Complete` | `build` |

Note the argument order on `with_header`: **description first, then author.**

```rust
pub fn with_header(self, description: &str, author: &str) -> ScenarioBuilder<HasHeader>;
```

## Header, parameters and road network

```rust
use openscenario_rs::types::enums::ParameterType;
use openscenario_rs::ScenarioBuilder;

let builder = ScenarioBuilder::new()
    .with_header("Highway Overtaking", "openscenario-rs")
    .add_parameter("initial_speed", ParameterType::Double, "25.0")
    .add_parameter("target_speed", ParameterType::Double, "35.0")
    .with_road_file("highway.xodr")
    .with_entities();
```

`add_parameter` appends a single declaration; `with_parameters` replaces the whole
`ParameterDeclarations` block. `with_road_file` is shorthand for the common case of a road
network that is one OpenDRIVE file; `with_road_network` takes a full `RoadNetwork`.

The file header records revision **1.3**, the version this crate targets and validates
against. Override it only for a consumer that needs an earlier one:

```rust
let builder = ScenarioBuilder::new()
    .with_header("Legacy consumer", "Author")
    .with_revision(1, 0);
```

Note that the document is still built from the 1.3 type model, so declaring an older revision
does not restrict what the builder emits.

## Catalog locations

`CatalogLocationsBuilder` covers all eight kinds the schema defines, each taking a directory
path:

```rust
use openscenario_rs::builder::CatalogLocationsBuilder;

let locations = CatalogLocationsBuilder::new()
    .with_vehicle_catalog("./catalogs/vehicles")
    .with_pedestrian_catalog("./catalogs/pedestrians")
    .with_controller_catalog("./catalogs/controllers")
    .with_misc_object_catalog("./catalogs/misc")
    .with_environment_catalog("./catalogs/environments")
    .with_maneuver_catalog("./catalogs/maneuvers")
    .with_trajectory_catalog("./catalogs/trajectories")
    .with_route_catalog("./catalogs/routes")
    .build();
```

Every kind is optional, so a builder with nothing set produces an empty `CatalogLocations`,
which is the right value for a scenario that references no catalogs.

## Two builder styles

The crate offers an attached and a detached style, and knowing which one you are in explains
most of the API's shape.

**Attached builders** borrow their parent and hand it back. `add_vehicle` takes a configuring
closure and returns the scenario builder, so the chain never breaks:

```rust
let builder = builder.add_vehicle("ego", |v| v.car().with_dimensions(4.5, 1.8, 1.4));
```

`add_vehicle_mut` is the same thing without the closure, for when the configuration is long
enough that a closure reads badly. It returns a `VehicleBuilder<'_>` borrowing the scenario
builder, and `finish()` returns the borrow:

```rust
let mut builder = builder;
builder.add_vehicle_mut("ego").car().with_performance(200.0, 10.0, 10.0).finish();
```

**Detached builders** own nothing and produce a value. They are the answer when a component is
built once and reused, or built somewhere the parent is not in scope:

```rust
use openscenario_rs::builder::DetachedVehicleBuilder;

let ego = DetachedVehicleBuilder::new("ego").car().build();      // -> ScenarioObject
```

Detached entity builders end in `build()`; detached storyboard builders end in `attach_to`
or `attach_to_detached`, which is how a maneuver assembled in isolation is folded back into an
act. `examples/builder_comprehensive_demo.rs` works through a full scenario in this style.

## Entities

`VehicleBuilder` and `DetachedVehicleBuilder` share a method set:

| Method | Effect |
|---|---|
| `car()` | Passenger-car category, with default dimensions and performance |
| `truck()` | Truck category, likewise |
| `with_dimensions(length, width, height)` | Bounding box |
| `with_performance(max_speed, max_acceleration, max_deceleration)` | Performance block |
| `finish()` / `build()` | Return to the parent, or produce a `ScenarioObject` |

`VehicleBuilder::detached(self)` converts an attached builder into a detached one.

Pedestrians take a parallel set – `pedestrian()`, `wheelchair()`, `animal()`, plus
`with_mass`, `with_dimensions`, `with_role` and `with_model3d`. Note that `with_mass` exists
on the pedestrian builders only; vehicles have no such method.

```rust
use openscenario_rs::builder::entities::DetachedPedestrianBuilder;

let walker = DetachedPedestrianBuilder::new("pedestrian_1")
    .pedestrian()
    .with_mass(80.0)
    .with_dimensions(0.5, 0.6, 1.8);
```

> `PedestrianBuilder`, `DetachedPedestrianBuilder`, `CatalogVehicleBuilder` and
> `CatalogPedestrianBuilder` are **not** re-exported at `openscenario_rs::builder`. Import
> them from `openscenario_rs::builder::entities`. Vehicle builders are re-exported at both
> paths.

## Init actions

The `Init` block sets the world's starting state, and `InitActionBuilder`
(`src/builder/init/`) covers it. The shortcuts handle the common cases directly:

```rust
use openscenario_rs::builder::InitActionBuilder;

let init = InitActionBuilder::new()
    .add_global_environment_action()
    .add_speed_action("ego", 27.8)
    .add_teleport_action("ego", position)
    .build()?;
```

For anything beyond those, `create_private_action(entity_ref)` opens a `PrivateActionBuilder`
scoped to one entity, and `create_global_action()` opens a `GlobalActionBuilder`. Both return
to the parent through `finish()`.

`PrivateActionBuilder` covers teleport, speed, longitudinal distance, speed profile, route
assignment (inline via `add_assign_route_action` and by catalog via
`add_assign_route_catalog`), synchronization, and visibility. Visibility has both the general
form and two shortcuts:

```rust
let init = InitActionBuilder::new()
    .create_private_action("ego")
    .add_speed_action(27.8)
    .make_visible()
    .finish()
    .build()?;
```

`add_action(PrivateActionWrapper)` is the escape hatch for an action the builder does not wrap.

Three constructors cover the standard openings: `InitActionBuilder::with_default_environment`,
`::for_single_vehicle(entity_ref)` and `::for_multiple_vehicles(&[&str])`.

## Storyboard

`with_storyboard` takes a closure and advances the state to `Complete`. Where the storyboard is
complex enough that a single closure becomes unwieldy, `StoryboardBuilder::new` takes the
scenario builder directly and the detached style takes over:

```rust
use openscenario_rs::builder::StoryboardBuilder;

let mut storyboard = StoryboardBuilder::new(scenario_builder);
let mut story = storyboard.add_story_simple("highway_overtaking");

let mut act = story.create_act("initial_acceleration");
let mut maneuver = act.create_maneuver("ego_accelerate", "ego");

let speed_action = maneuver
    .create_speed_action()
    .named("initial_acceleration")
    .to_speed(25.0)
    .with_trigger(trigger);

speed_action.attach_to_detached(&mut maneuver)?;
maneuver.attach_to_detached(&mut act);
act.attach_to(&mut story);
```

The nesting reads bottom-up: an action attaches to a maneuver, a maneuver to an act, an act to
a story. `StoryboardBuilder` also offers stop-trigger shortcuts – `stop_after_time(f64)`,
`stop_when_entity_reaches`, `stop_on_condition` and the general `with_stop_trigger`.

## Conditions and triggers

`TriggerBuilder` assembles condition groups, and each condition family has its own builder:
`SpeedConditionBuilder`, `AccelerationConditionBuilder`, `TimeConditionBuilder`,
`TraveledDistanceConditionBuilder`, `ReachPositionConditionBuilder`,
`RelativeDistanceConditionBuilder`, `CollisionConditionBuilder`, `ParameterConditionBuilder`,
`VariableConditionBuilder` and `ValueSpeedConditionBuilder`. All are re-exported at
`openscenario_rs::builder`.

## Validation during construction

`src/builder/validation.rs` checks a scenario before it becomes a document:

```rust
use openscenario_rs::builder::ValidationContextBuilder;

let ctx = ValidationContextBuilder::new()
    .with_standard_rules()
    .with_entity("ego", "vehicle")
    .with_parameter("initial_speed", "25.0")
    .build();

ctx.validate_scenario(&scenario)?;
```

`with_standard_rules` installs `EntityReferenceValidationRule`,
`ParameterReferenceValidationRule`, `CatalogReferenceValidationRule` and
`StoryboardStructureValidationRule`. Custom rules implement `BuilderValidationRule` and go in
through `with_rule`. Details are in the [validation guide](validation_guide.md).

## Errors

```rust
pub enum BuilderError {
    ValidationError { message: String, suggestion: String },
    MissingField { field: String, suggestion: String },
    InvalidEntityRef { entity: String, available: Vec<String> },
    ConstraintViolation { constraint: String, details: String },
    OpenScenarioError(#[from] crate::error::Error),
}
pub type BuilderResult<T> = std::result::Result<T, BuilderError>;
```

`InvalidEntityRef` carries the available entity names, and the two `suggestion` fields carry
a remedy where the builder can name one – worth surfacing rather than printing the message
alone.

## Templates

`BasicScenarioTemplate` and `ScenarioTemplate` (`src/builder/templates/`) wrap a preconfigured
opening for scenarios that differ only in their details.

## Examples

| Example | Shows |
|---|---|
| `builder_basic_demo` | The minimum: header, required elements, entities, build, serialize |
| `builder_comprehensive_demo` | Detached builders across multiple acts and maneuvers |
| `builder_performance_demo` | Building at volume |
| `pedestrian_builder_demo` | Pedestrian entities |
| `cut_in_scenario_demo` | A cut-in scenario, detached style |
| `alks_scenario_4_1_1_comprehensive` | ALKS Scenario 4.1.1 "Free Driving", end to end |

```bash
cargo run --example builder_basic_demo --features builder
cargo run --example builder_comprehensive_demo --features builder
```

## Choosing a style

Reach for the attached style by default: it is shorter, the borrow checker keeps the
composition honest, and the chain reads in document order. Switch to detached builders when a
component is reused across scenarios, when it is assembled in a function that does not hold the
parent, or when a storyboard has enough nesting that closures stop being readable. The two mix
freely within one scenario.
