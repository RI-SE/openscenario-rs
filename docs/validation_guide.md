# Validation guide

"Valid" means several different things in this crate, and confusing them is the fastest way to
trust a result that was never checked. Four layers exist, they answer different questions, and
only one of them consults the XSD. This document describes what each one actually does.

| Layer | Question it answers | Consults the schema |
|---|---|---|
| Structural pre-check | Is this plausibly an OpenSCENARIO document? | no |
| XSD schema validation | Does this document conform to OpenSCENARIO 1.3? | **yes** |
| Semantic validation | Do the references and constraints hold? | no |
| Per-type `validate()` | Is this one value or choice group well formed? | no |

## Structural pre-check

`parser::xml::validate_xml_structure` and `validate_catalog_xml_structure` are the cheapest
check and the weakest. They confirm the input is non-empty, that it begins with `<?xml` or
`<`, and that the text contains the substring `OpenSCENARIO`
(`src/parser/xml.rs:279`). That is the whole of it.

They exist to turn a confusing serde error into a clear one when someone passes a JSON file or
an empty buffer. The `*_validated` parse functions run the pre-check first:

```rust
use openscenario_rs::parser::xml::{parse_from_file_validated, parse_from_str_validated};

let scenario = parse_from_file_validated("scenario.xosc")?;
```

Do not read a successful pre-check as schema conformance. A file containing the single line
`<OpenSCENARIO/>` passes it.

## XSD schema validation

This is the layer that answers whether a document conforms to OpenSCENARIO 1.3. It lives in
`src/validation.rs` behind the `validation` cargo feature and is backed by libxml validating
against the bundled `Schema/OpenSCENARIO.xsd`.

```toml
[dependencies]
openscenario-rs = { version = "0.3.2", features = ["validation"] }
```

```rust
use openscenario_rs::validation::XsdValidator;

let mut validator = XsdValidator::from_schema_file("Schema/OpenSCENARIO.xsd")?;
let errors = validator.validate_file("scenario.xosc")?;

if errors.is_empty() {
    println!("valid");
} else {
    for error in &errors {
        println!("{error}");
    }
}
```

The return type carries the distinction that matters: **an empty `Vec` means the document is
valid**, a non-empty one lists the schema violations, and `Err` is reserved for input that is
not well-formed XML at all – a case the schema cannot even be applied to. Reading `Ok` as
"valid" without checking the vector is the mistake this API shape is trying to prevent.

The full surface:

```rust
impl XsdValidator {
    pub fn from_schema_file<P: AsRef<Path>>(xsd_path: P) -> Result<Self>;
    pub fn from_schema_str(xsd_content: &str) -> Result<Self>;
    pub fn validate_str(&mut self, xml: &str) -> Result<Vec<ValidationError>>;
    pub fn validate_file<P: AsRef<Path>>(&mut self, path: P) -> Result<Vec<ValidationError>>;
    pub fn validate_document(&mut self, document: &libxml::tree::Document) -> Vec<ValidationError>;
}
```

Each diagnostic carries an optional line and column, the trimmed libxml message, and a coarse
`error_type` produced by `classify_error_type`, one of `ElementNotAllowed`,
`MissingRequired`, `InvalidValue`, `TypeMismatch` or `ValidationError`.

Two implementation details are worth knowing. The schema is parsed once when the validator is
constructed and the resulting context is reused, so validating a corpus does not re-parse the
XSD per file – construct one validator and loop. The validating methods take `&mut self`
because libxml's `validate_document` does; this is not a design choice the crate could avoid.

`validate_str` exists specifically so that serialized in-memory output can be validated
without a temporary file, which is what the out-of-tree round-trip harness uses to check the
crate's own output rather than only its input.

### The `xosc-validate` binary

The same validator is available from the command line, and the binary requires the feature:

```bash
cargo run --bin xosc-validate --features validation -- scenario.xosc
```

| Flag | Default | Meaning |
|---|---|---|
| `-s`, `--schema <PATH>` | `Schema/OpenSCENARIO.xsd` | Schema to validate against |
| `-f`, `--format <FMT>` | `human` | `human`, `json` or `junit` |
| `-r`, `--recursive` | off | Validate every `.xosc` under the given directories |
| `-q`, `--quiet` | off | Print the summary only |

It exits `0` on success, `1` on validation errors, `2` for a missing file, `3` for a schema
problem and `99` on an internal error, so it can be gated on `$?` in CI without parsing its
output.

## Semantic validation

Schema conformance says nothing about whether an `entityRef` names an entity that exists, or
whether a declared parameter is ever used. That is `parser::validation::ScenarioValidator`,
which walks a parsed document and reports cross-reference and constraint problems.

```rust
use openscenario_rs::parser::validation::{ScenarioValidator, ValidationConfig};

let scenario = openscenario_rs::parse_file("scenario.xosc")?;

let config = ValidationConfig {
    strict_mode: true,
    ..Default::default()
};
let mut validator = ScenarioValidator::with_config(config);
let result = validator.validate_scenario(&scenario);

if !result.is_valid() {
    println!("{}", result.summary());
    for error in &result.errors {
        println!("{}: {}", error.location, error.message);
    }
}
```

`ValidationConfig` carries `strict_mode` (treat warnings as failures), `validate_references`,
`validate_constraints`, `validate_semantics`, `max_errors` and `use_cache`, and has a
`Default`. `ValidationResult` separates `errors` from `warnings` and offers `is_valid()`
(no errors), `is_clean()` (no errors and no warnings), `total_issues()` and `summary()`.

Errors are categorized as `MissingRequired`, `InvalidReference`, `ConstraintViolation`,
`SemanticError`, `TypeMismatch` or `ParameterError`; warnings as `Deprecated`, `Suspicious`,
`Performance` or `BestPractice`. Each carries a `location` and, where one can be given, a
`suggestion`.

## Per-type validation

Two smaller mechanisms operate on individual values rather than whole documents.

**The `Validate` trait** (`src/types/mod.rs:120`) is implemented per type and takes a
`ValidationContext` holding the known entities and parameters:

```rust
use openscenario_rs::types::{EntityRef, ObjectType, Validate, ValidationContext};

let mut ctx = ValidationContext::new().with_strict_mode();
ctx.add_entity(
    "ego".to_string(),
    EntityRef { name: "ego".to_string(), object_type: ObjectType::Vehicle },
);

value.validate(&ctx)?;
```

Note that `with_strict_mode` is chainable but `add_entity` is not – it takes `&mut self` and
two arguments.

**Choice-group `validate()`** is an inherent method, not the trait, and returns
`Result<(), String>`. Types modeling an XSD choice as parallel `Option` fields (`GlobalAction`,
`PrivateAction`, `LongitudinalAction`, `ObjectController` and others) use it to enforce the
exactly-one rule the Rust type system cannot express, alongside a `get_action_type()` that
names the populated branch:

```rust
global_action.validate()?;                       // Err if zero or several branches are set
let kind = global_action.get_action_type();      // Some("EnvironmentAction")
```

The two share a name and nothing else. See [type_system_guide.md](type_system_guide.md) for
why both exist.

## Builder validation

With the `builder` feature, `src/builder/validation.rs` checks a scenario under construction –
before it has become a document that any of the layers above could inspect.

```rust
use openscenario_rs::builder::ValidationContextBuilder;

let ctx = ValidationContextBuilder::new()
    .with_standard_rules()
    .with_entity("ego", "vehicle")
    .with_parameter("initialSpeed", "27.8")
    .build();

ctx.validate_scenario(&scenario)?;
```

The two reference rules scan the serialized document rather than walking the typed tree. That
is deliberate: a reference can appear in `Init`, in actors, in triggering entities, in a
condition target, or in an action nested several levels down, and a typed walk grows a silent
gap every time a new variant is modelled. Scanning what the document actually emits is complete
by construction. Parameterized references (`entityRef="${target}"`) are skipped, since they
resolve at run time.

This is the only layer that checks cross-references at all. XSD validation cannot express them,
so a document naming an entity or parameter that was never declared is schema-valid and
silently wrong.

`with_standard_rules` installs the four shipped rules:
`EntityReferenceValidationRule`, `ParameterReferenceValidationRule`,
`CatalogReferenceValidationRule` and `StoryboardStructureValidationRule`. Custom rules
implement `BuilderValidationRule` and are added with `with_rule`. Types implementing
`BuilderValidatable` validate themselves against a context.

## A note on three types called `ValidationError`

The name collides three ways, and the compiler error when you import the wrong one is not
especially helpful:

| Path | What it is |
|---|---|
| `openscenario_rs::Error::ValidationError` | a variant of the crate's error enum |
| `openscenario_rs::parser::validation::ValidationError` | a semantic-validation finding |
| `openscenario_rs::validation::ValidationError` | a libxml schema diagnostic |

Import the one matching the layer you are working in, and alias it if two are in scope.

## Which layer to use

For most work the answer is the XSD validator, and it is the only layer that can tell you
whether a file conforms to the standard. Reach for the others when the question is genuinely
different: the pre-check for a fast rejection of obvious garbage, the semantic validator when
the document is already known to conform and you want to know whether it makes sense, and the
per-type methods when you are constructing values programmatically and want to fail early.

For what the crate's own conformance is currently known to be, including which parts of the
schema the test corpus never exercises, see [xsd_gaps.md](xsd_gaps.md).
