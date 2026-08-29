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

One structural gap remains open. Each of those 37 enumerations is an `xsd:union` carrying a
`parameter` member, which makes `vehicleCategory="${cat}"` schema-valid across 75 attributes.
A bare Rust enum cannot represent that, so parameterized enumeration attributes are currently
rejected. The analysis is in [xsd_gaps.md](xsd_gaps.md).

## The `Default` policy

`Default` is implemented only where a default states nothing. Container and choice structs
that default to all-`None` or empty (`Actions`, `GlobalAction`, `PrivateAction`,
`EnvironmentAction`, `ParameterDeclarations`) keep theirs.

Impls that would fabricate scenario content were removed deliberately. A `TeleportAction`
whose `Default` invents a world position at the origin produces a document that parses
cleanly and describes something nobody wrote, which is worse than a compile error. If a type
needs every field to say anything at all, it should require every field.

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
4. Add a round-trip test. If the type is a flattened choice, add it to
   `tests/choice_flatten_roundtrip_test.rs`.
5. Run the conformance gates. Their locations and what each one catches are in
   [xsd_gaps.md](xsd_gaps.md).
