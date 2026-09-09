# User guide

`openscenario-rs` reads, writes and constructs [OpenSCENARIO](https://www.asam.net/standards/detail/openscenario/)
files. This guide walks through the library from installation to catalog resolution. It
assumes familiarity with the standard itself and describes only what the crate actually does.

For exhaustive signatures see the [API reference](api_reference.md); for the schema mapping,
the [type system guide](type_system_guide.md).

## Installation

```toml
[dependencies]
openscenario-rs = "0.4.0"
```

Neither optional feature is on by default:

```toml
openscenario-rs = { version = "0.4.0", features = ["builder", "validation"] }
```

`builder` enables programmatic construction; `validation` enables XSD schema validation and
the `xosc-validate` binary. The crate requires Rust **1.90** or later.

## Three kinds of document

The `.xosc` extension covers three different document shapes, and the crate models all of them
with the same `OpenScenario` root type. Every field but the header is optional, and which
fields are populated is what distinguishes one shape from another:

```rust
use openscenario_rs::{parse_file, OpenScenarioDocumentType};

let document = parse_file("scenario.xosc")?;

match document.document_type() {
    OpenScenarioDocumentType::Scenario => {
        // entities and a storyboard are both present
        if let Some(entities) = &document.entities {
            println!("{} entities", entities.scenario_objects.len());
        }
    }
    OpenScenarioDocumentType::ParameterVariation => {
        println!("parameter variation file");
    }
    OpenScenarioDocumentType::Catalog => {
        println!("catalog file");
    }
    OpenScenarioDocumentType::Unknown => {
        println!("none of the above");
    }
}
```

The predicates `is_scenario()`, `is_parameter_variation()` and `is_catalog()` are available
where a single check reads better than a match.

## Parsing and serializing

```rust
use openscenario_rs::{parse_file, parse_str, serialize_str};

let from_disk = parse_file("scenario.xosc")?;
let from_memory = parse_str(xml)?;

let xml = serialize_str(&from_disk)?;
```

The file-oriented forms in `parser::xml` add `serialize_to_file`, and catalog documents have
their own set: `parse_catalog_file`, `parse_catalog_str`, `serialize_catalog_to_string`,
`serialize_catalog_to_file`.

Serialization prepends the XML declaration and pretty-prints the output, so a document written
back to disk is readable rather than one long line.

### Validated parsing

Each parse function has a `_validated` sibling that runs a structural pre-check first:

```rust
use openscenario_rs::parser::xml::parse_from_file_validated;

let document = parse_from_file_validated("scenario.xosc")?;
```

Be clear on what this buys you. The pre-check confirms the input is non-empty, begins with
`<?xml` or `<`, and contains the substring `OpenSCENARIO`. It turns a confusing serde error
into a clear one when someone hands you a JSON file. It is **not** schema validation, and a
file containing only `<OpenSCENARIO/>` passes it. For real conformance checking see
[validation_guide.md](validation_guide.md).

## Reading a scenario

Entities are `ScenarioObject`s, and the entity kind is a flat set of `Option` fields rather
than an enum:

```rust
if let Some(entities) = &document.entities {
    for object in &entities.scenario_objects {
        let name = object.name.as_literal().map_or("<parameterized>", |v| v);

        if let Some(vehicle) = &object.vehicle {
            println!("{name}: vehicle, category {:?}", vehicle.vehicle_category);
        } else if let Some(pedestrian) = &object.pedestrian {
            println!("{name}: pedestrian, mass {}", pedestrian.mass);
        } else if let Some(catalog_ref) = &object.entity_catalog_reference {
            println!("{name}: from catalog");
        }
    }
}
```

Two details catch people out. `ScenarioObject::name` is an `OSString`, not an
`Option<OSString>` – the schema requires it. And the catalog reference field is named
`entity_catalog_reference`, not `catalog_reference`.

## Values, parameters and expressions

Nearly every attribute in OpenSCENARIO may hold a literal, a `${parameter}` reference, or a
`${expression}`. The crate models this once, in `Value<T>`, rather than per attribute:

```rust
pub enum Value<T> { Literal(T), Parameter(String), Expression(String) }
```

You will meet it through its aliases: `OSString`, `Double`, `Int`, `UnsignedInt`,
`UnsignedShort`, `Boolean` and `DateTime`. Construction takes an owned value, and a parameter
name is given **bare**, without the `${…}` braces:

```rust
use openscenario_rs::types::basic::Double;

let literal = Double::literal(25.0);
let parameter = Double::parameter("vehicle_speed".to_string());
let expression = Double::expression("vehicle_speed + 10".to_string());
```

Inspection is by three accessors, each returning `None` for the other two variants. There is
no `is_parameter()` predicate:

```rust
if let Some(speed) = literal.as_literal() { /* &f64 */ }
if let Some(name) = parameter.as_parameter() { /* &str */ }
if let Some(expr) = expression.as_expression() { /* &str */ }
```

### Resolution

Resolution takes a plain `HashMap<String, String>`:

```rust
use std::collections::HashMap;

let mut params = HashMap::new();
params.insert("vehicle_speed".to_string(), "30.0".to_string());

let speed: f64 = parameter.resolve(&params)?;
```

A parameter absent from the map, or a substituted value that does not parse into `T`, produces
`Error::ParameterError`. The parameters declared by a document itself are extracted with:

```rust
use openscenario_rs::catalog::extract_scenario_parameters;

let params = extract_scenario_parameters(&document.parameter_declarations);
```

Arithmetic expressions are evaluated separately:

```rust
use openscenario_rs::evaluate_expression;

let result: f64 = evaluate_expression("vehicle_speed * 2 + 5", &params)?;
```

`examples/expression_demo.rs` runs through all three value kinds.

```bash
cargo run --example expression_demo
```

## Catalogs

Catalogs hold reusable vehicles, pedestrians, controllers, trajectories, routes and
environments, referenced from a scenario by catalog name and entry name. `CatalogManager`
loads and resolves them.

```rust
use openscenario_rs::CatalogManager;

let mut manager = CatalogManager::with_base_path("./scenarios");

// Parameters declared by the scenario are needed to resolve parameterized catalog names
manager.set_global_parameters(params)?;

if let Some(locations) = &document.catalog_locations {
    manager.discover_and_load_catalogs(locations)?;
}
```

Resolving a reference needs both the reference and the catalog location it should be looked up
in:

```rust
let resolved = manager.resolve_vehicle_reference(&reference, &vehicle_location)?;

let vehicle = &resolved.entity;
println!("resolved from {}", resolved.metadata.catalog_path);
```

`ResolvedCatalog<T>` carries the entity alongside `ResolutionMetadata`, which records the
catalog file it came from, the entry name and every parameter substituted during resolution.
The path is `metadata.catalog_path`, not a field on the resolved value itself.

`resolve_controller_reference` and `resolve_pedestrian_reference` follow the same shape.
Circular references are detected during resolution rather than being followed until the stack
runs out.

### Building a reference

`CatalogReference<T>` carries a private type marker, so it cannot be built from a struct
literal outside its module. Use the constructor:

```rust
use openscenario_rs::types::catalogs::references::CatalogReference;

let reference = CatalogReference::new("VehicleCatalog".to_string(), "car_white".to_string());
```

`ParameterAssignment` and `ParameterAssignments`, which carry the values a reference passes
into a parameterized catalog entry, live in `types::catalogs::references` alongside it.

### Catalog files

A catalog file is a full OpenSCENARIO document whose root carries a `Catalog` element, and the
crate parses it as such:

```rust
use openscenario_rs::parse_catalog_file;

let catalog = parse_catalog_file("catalogs/vehicles.xosc")?;

println!("{}: {} entries", catalog.catalog_name(), catalog.entity_count());
for name in catalog.entity_names() {
    println!("  {name}");
}
```

`CatalogFile` also offers `vehicles()`, `controllers()`, `pedestrians()` and the corresponding
`find_*` lookups.

## Constructing scenarios

With the `builder` feature:

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
```

The builder uses a typestate so that a document missing its header or entities does not
compile. `CatalogLocations`, `RoadNetwork` and the storyboard are required too, but by the
schema rather than the type system, so `build()` rejects them at runtime. The empty forms
above are what a scenario with no catalogs and no road file should pass. The
[builder guide](builder_guide.md) covers entities, init actions, storyboards and the detached
style.

## Validation

Four layers exist and they answer different questions. Briefly:

- the **structural pre-check** described above rejects obvious non-documents;
- **XSD schema validation** (`validation` feature) is the one that answers whether a file
  conforms to OpenSCENARIO 1.3;
- **semantic validation** checks cross-references and constraints in a parsed document;
- **per-type `validate()`** checks a single value or choice group.

The schema validator in brief:

```rust
use openscenario_rs::validation::XsdValidator;

let mut validator = XsdValidator::from_schema_file("Schema/OpenSCENARIO.xsd")?;
let errors = validator.validate_file("scenario.xosc")?;

if errors.is_empty() {
    println!("valid");
}
```

An empty vector means valid; `Err` is reserved for input that is not well-formed XML at all.
The full treatment is in [validation_guide.md](validation_guide.md).

From the command line:

```bash
cargo run --bin xosc-validate --features validation -- scenario.xosc
```

## Error handling

Everything fallible returns `openscenario_rs::Result<T>`, aliasing a single `Error` enum. The
variants carry structured context rather than a formatted string, so a caller can react to the
specific failure:

```rust
use openscenario_rs::Error;

match parse_file("scenario.xosc") {
    Ok(document) => { /* ... */ }
    Err(Error::FileNotFound { path }) => eprintln!("no such file: {path}"),
    Err(Error::ParameterNotFound { param, available }) => {
        eprintln!("unknown parameter {param}; known: {available:?}");
    }
    Err(Error::XmlParseError(e)) => eprintln!("malformed XML: {e}"),
    Err(e) => eprintln!("{e}"),
}
```

`Error::EntityNotFound` and `Error::ParameterNotFound` both carry the available names, which
is usually what you want to print. See the [API reference](api_reference.md) for the full list
of variants and their constructor helpers.

## Command-line tools

```bash
cargo run --bin scenario_analyzer -- scenario.xosc
cargo run --bin xosc-validate --features validation -- scenario.xosc
```

`xosc-validate` takes `--schema`, `--format human|json|junit`, `--recursive` and `--quiet`, and
exits non-zero on failure so it can be gated in CI without parsing its output.

## Examples

The `examples/` directory holds runnable programs for each area of the crate:

| Example | Shows |
|---|---|
| `parse` | A general-purpose parse-and-inspect tool |
| `expression_demo` | Literals, parameters and expressions |
| `routing_demo` | Routes, waypoints and trajectories |
| `spatial_conditions_demo`, `simple_spatial_demo` | Spatial conditions |
| `motion_conditions_demo`, `byvalue_conditions_demo` | Condition families |
| `vehicle_components_demo`, `vehicle_axles_demo`, `bounding_box_demo` | Vehicle structure |
| `action_wrappers_demo` | The action wrapper hierarchy |
| `cut_in_scenario_demo`, `alks_scenario_4_1_1_comprehensive` | Complete worked scenarios, built programmatically |

```bash
cargo run --example expression_demo
cargo run --example cut_in_scenario_demo --features builder
```

One caveat: `basic_parsing` reads a hardcoded path that is not in the repository and panics.

## Known limitations

The crate targets OpenSCENARIO 1.3 as defined by the bundled `Schema/OpenSCENARIO.xsd`.

One conformance gap is open and worth knowing about before you hit it: every enumeration in
the schema is a union that also admits a parameter, so `vehicleCategory="${cat}"` is valid
across 75 attributes. A bare Rust enum cannot represent that, and the crate currently rejects
such files. The full conformance ledger, including which parts of the schema the test corpus
never exercises, is in [xsd_gaps.md](xsd_gaps.md).
