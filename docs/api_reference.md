# API reference

A reference to the public surface of `openscenario-rs` 0.3.2. Every signature here is taken
from the source; where a name is ambiguous or collides with another, that is flagged rather
than glossed over.

For narrative introductions see the [user guide](user_guide.md) and
[builder guide](builder_guide.md). For how the types map to the schema, see the
[type system guide](type_system_guide.md).

## Crate metadata

| | |
|---|---|
| Version | 0.3.2 |
| Edition | 2021 |
| MSRV | **1.90** |
| License | GPL-3.0-only |
| Target standard | OpenSCENARIO 1.3 |

## Features

Neither feature is enabled by default.

| Feature | Enables |
|---|---|
| `builder` | `openscenario_rs::builder`, the `ScenarioBuilder` re-export, and the `pedestrian_builder_demo` example |
| `validation` | `openscenario_rs::validation` (`XsdValidator`) and the `xosc-validate` binary |

```toml
[dependencies]
openscenario-rs = { version = "0.3.2", features = ["builder", "validation"] }
```

## Top-level functions

The crate root offers five convenience functions wrapping `parser::xml`:

```rust
pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<OpenScenario>;
pub fn parse_str(xml: &str) -> Result<OpenScenario>;
pub fn parse_catalog_file<P: AsRef<Path>>(path: P) -> Result<CatalogFile>;
pub fn parse_catalog_str(xml: &str) -> Result<CatalogFile>;
pub fn serialize_str(scenario: &OpenScenario) -> Result<String>;
```

## Root re-exports

There is no `prelude` module; the crate root re-exports the following directly.

```rust
pub use error::{Error, Result};
pub use types::scenario::storyboard::{
    FileHeader, OpenScenario, OpenScenarioDocumentType, ScenarioDefinition,
};
pub use parser::xml::{
    parse_catalog_from_file, parse_catalog_from_str, parse_from_file, parse_from_str,
    serialize_catalog_to_file, serialize_catalog_to_string, serialize_to_file,
    serialize_to_string,
};
pub use parser::choice_groups::{
    parse_choice_group, ChoiceGroupParser, ChoiceGroupRegistry, XsdChoiceGroup,
};
pub use expression::evaluate_expression;
pub use catalog::{
    CatalogLoader, CatalogManager, CatalogResolver, ParameterSubstitutionEngine, ResolvedCatalog,
};

#[cfg(feature = "builder")]
pub use builder::ScenarioBuilder;
```

## Modules

| Module | Gate | Contents |
|---|---|---|
| `types` | none | The OpenSCENARIO 1.3 type model |
| `parser` | none | XML deserialization, serialization, semantic validation, choice-group helpers |
| `catalog` | none | Catalog loading, resolution and parameter substitution |
| `expression` | none | `${…}` tokenizing, parsing and evaluation |
| `error` | none | `Error` and `Result` |
| `builder` | `builder` | Typestate scenario construction |
| `validation` | `validation` | libxml-backed XSD schema validation |

## The document root

`OpenScenario` (`src/types/scenario/storyboard.rs:16`) models all three document shapes the
standard defines, so every field but the header is optional:

```rust
pub struct OpenScenario {
    pub file_header: FileHeader,
    pub parameter_declarations: Option<ParameterDeclarations>,
    pub variable_declarations: Option<VariableDeclarations>,
    pub monitor_declarations: Option<MonitorDeclarations>,
    pub catalog_locations: Option<CatalogLocations>,
    pub road_network: Option<RoadNetwork>,
    pub entities: Option<Entities>,
    pub storyboard: Option<Storyboard>,
    pub parameter_value_distribution: Option<ParameterValueDistribution>,
    pub catalog: Option<CatalogDefinition>,
}
```

Which shape a given document is, is determined by inspection rather than by a tag:

```rust
impl OpenScenario {
    pub fn document_type(&self) -> OpenScenarioDocumentType;
    pub fn is_scenario(&self) -> bool;
    pub fn is_parameter_variation(&self) -> bool;
    pub fn is_catalog(&self) -> bool;
}

pub enum OpenScenarioDocumentType {
    Scenario,
    ParameterVariation,
    Catalog,
    Unknown,
}
```

A document counts as `Scenario` when it carries both entities and a storyboard; failing that,
`ParameterVariation` if it carries a parameter value distribution, then `Catalog`, then
`Unknown`.

## Parser

### `parser::xml`

```rust
pub fn parse_from_str(xml: &str) -> Result<OpenScenario>;
pub fn parse_from_file<P: AsRef<Path>>(path: P) -> Result<OpenScenario>;
pub fn parse_from_str_validated(xml: &str) -> Result<OpenScenario>;
pub fn parse_from_file_validated<P: AsRef<Path>>(path: P) -> Result<OpenScenario>;
pub fn serialize_to_string(scenario: &OpenScenario) -> Result<String>;
pub fn serialize_to_file<P: AsRef<Path>>(scenario: &OpenScenario, path: P) -> Result<()>;
pub fn validate_xml_structure(xml: &str) -> Result<()>;

pub fn parse_catalog_from_str(xml: &str) -> Result<CatalogFile>;
pub fn parse_catalog_from_file<P: AsRef<Path>>(path: P) -> Result<CatalogFile>;
pub fn parse_catalog_from_str_validated(xml: &str) -> Result<CatalogFile>;
pub fn parse_catalog_from_file_validated<P: AsRef<Path>>(path: P) -> Result<CatalogFile>;
pub fn serialize_catalog_to_string(catalog: &CatalogFile) -> Result<String>;
pub fn serialize_catalog_to_file<P: AsRef<Path>>(catalog: &CatalogFile, path: P) -> Result<()>;
pub fn validate_catalog_xml_structure(xml: &str) -> Result<()>;
```

Parsing is `quick_xml::de::from_str`. Serialization prepends the XML declaration, runs
`quick_xml::se::to_string`, and pretty-prints the result through `markup_fmt`. The parse and
serialize functions are `#[must_use]`.

The `_validated` variants run `validate_xml_structure` first. That check confirms the input is
non-empty, starts with `<?xml` or `<`, and contains the substring `OpenSCENARIO` – it is
**not** schema validation. For that, see `validation::XsdValidator` below.

### `parser::validation`

```rust
pub struct ScenarioValidator;
impl ScenarioValidator {
    pub fn new() -> Self;
    pub fn with_config(config: ValidationConfig) -> Self;
    pub fn validate_scenario(&mut self, scenario: &OpenScenario) -> ValidationResult;
}

pub struct ValidationConfig {
    pub strict_mode: bool,
    pub validate_references: bool,
    pub validate_constraints: bool,
    pub validate_semantics: bool,
    pub max_errors: usize,
    pub use_cache: bool,
}

pub struct ValidationResult {
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
    pub metrics: ValidationMetrics,
}
impl ValidationResult {
    pub fn is_valid(&self) -> bool;      // no errors
    pub fn is_clean(&self) -> bool;      // no errors and no warnings
    pub fn total_issues(&self) -> usize;
    pub fn summary(&self) -> String;
}
```

Findings are categorized by `ValidationErrorCategory` (`MissingRequired`, `InvalidReference`,
`ConstraintViolation`, `SemanticError`, `TypeMismatch`, `ParameterError`) and
`ValidationWarningCategory` (`Deprecated`, `Suspicious`, `Performance`, `BestPractice`).

### `parser::choice_groups`

```rust
pub trait XsdChoiceGroup { /* ... */ }
pub struct ChoiceGroupParser;
pub struct ChoiceGroupRegistry;
pub fn parse_choice_group(/* ... */);
```

Infrastructure for XSD choice groups, re-exported at the crate root. The module documents
itself as a simplified implementation sufficient for current needs.

## Schema validation (feature `validation`)

```rust
pub struct XsdValidator;
impl XsdValidator {
    pub fn from_schema_file<P: AsRef<Path>>(xsd_path: P) -> Result<Self>;
    pub fn from_schema_str(xsd_content: &str) -> Result<Self>;
    pub fn validate_str(&mut self, xml: &str) -> Result<Vec<ValidationError>>;
    pub fn validate_file<P: AsRef<Path>>(&mut self, path: P) -> Result<Vec<ValidationError>>;
    pub fn validate_document(&mut self, document: &libxml::tree::Document) -> Vec<ValidationError>;
}

pub struct ValidationError {
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub message: String,
    pub error_type: String,
}

pub fn convert_errors(errors: Vec<StructuredError>) -> Vec<ValidationError>;
pub fn classify_error_type(message: &str) -> String;
```

An empty `Vec` means the document is valid. `Err` is returned only when the input is not
well-formed XML – that is, when the schema cannot be applied at all. The schema is parsed once
at construction and reused, so a validator should be built once and looped over.

`classify_error_type` returns one of `ElementNotAllowed`, `MissingRequired`, `InvalidValue`,
`TypeMismatch` or `ValidationError`.

See the [validation guide](validation_guide.md) for how this relates to the other layers.

## Types

### `Value<T>` and its aliases

```rust
pub enum Value<T> { Literal(T), Parameter(String), Expression(String) }

impl<T: Clone> Value<T> {
    pub fn literal(value: T) -> Self;
    pub fn parameter(name: String) -> Self;     // bare name, no ${}
    pub fn expression(expr: String) -> Self;
}

impl<T> Value<T> where T: FromStr + Clone {
    pub fn resolve(&self, params: &HashMap<String, String>) -> Result<T>;
    pub fn as_literal(&self) -> Option<&T>;
    pub fn as_parameter(&self) -> Option<&str>;
    pub fn as_expression(&self) -> Option<&str>;
}
```

Aliases: `OSString`, `Double`, `Int`, `UnsignedInt`, `UnsignedShort`, `Boolean`, `DateTime`.

Free functions in `types::basic`: `parse_parameter_reference`, `is_expression`,
`is_valid_parameter_name`.

### Cross-cutting traits

```rust
pub trait Validate {
    fn validate(&self, ctx: &ValidationContext) -> Result<()>;
}

pub trait Resolve<T> {
    fn resolve(&self, ctx: &ParameterContext) -> Result<T>;
}

pub struct ValidationContext {
    pub entities: HashMap<String, EntityRef>,
    pub catalogs: HashMap<String, CatalogRef>,
    pub strict_mode: bool,
}
impl ValidationContext {
    pub fn new() -> Self;
    pub fn with_strict_mode(self) -> Self;                             // chainable
    pub fn add_entity(&mut self, name: String, entity_ref: EntityRef); // not chainable
}

pub struct ParameterContext {
    pub parameters: HashMap<String, String>,
    pub scope: Vec<String>,
}
```

`Value<T>` carries blanket impls of both traits.

### Enumerations

All 37 XSD enumeration simple types have a same-named Rust enum in `types::enums`, each
variant renamed to its schema value. Those re-exported at `types::` include `AngleType`,
`AutomaticGearType`, `ColorType`, `ConditionEdge`, `ControllerType`, `DirectionalDimension`,
`DynamicsDimension`, `DynamicsShape`, `FractionalCloudCover`, `LightMode`,
`MiscObjectCategory`, `ObjectType`, `ParameterType`, `PedestrianCategory`,
`PedestrianGestureType`, `PedestrianMotionType`, `PrecipitationType`, `Priority`, `Role`,
`RouteStrategy`, `RoutingAlgorithm`, `Rule`, `TriggeringEntitiesRule`, `VehicleCategory`,
`VehicleComponentType`, `VehicleLightType` and `Wetness`. The remainder are reachable at
`types::enums::`.

### Entities

```rust
pub struct ScenarioObject {
    pub name: OSString,                                            // required, not Option
    pub vehicle: Option<Vehicle>,
    pub pedestrian: Option<Pedestrian>,
    pub misc_object: Option<MiscObject>,
    pub external_object_reference: Option<ExternalObjectReference>,
    pub entity_catalog_reference: Option<ScenarioEntityReference>,
    pub object_controller: Vec<ObjectController>,                  // may repeat
}
```

The entity variants are flat `Option` fields; there is no `EntityObject` enum. The catalog
reference field is named `entity_catalog_reference`, not `catalog_reference`.

### Conditions by value

`ByValueCondition` (`src/types/conditions/value.rs:98`) holds **eight** branches as parallel
`Option` fields. The field names are the snake-case condition names in full, each carrying the
`_condition` suffix:

| Field | Element |
|---|---|
| `parameter_condition` | `ParameterCondition` |
| `variable_condition` | `VariableCondition` |
| `time_of_day_condition` | `TimeOfDayCondition` |
| `simulation_time_condition` | `SimulationTimeCondition` |
| `storyboard_element_state_condition` | `StoryboardElementStateCondition` |
| `user_defined_value_condition` | `UserDefinedValueCondition` |
| `traffic_signal_condition` | `TrafficSignalCondition` |
| `traffic_signal_controller_condition` | `TrafficSignalControllerCondition` |

`Rule` takes `EqualTo`, `GreaterThan`, `LessThan`, `GreaterOrEqual`, `LessOrEqual` and
`NotEqualTo`, serializing as `equalTo`, `greaterThan` and so on.

## Catalogs

```rust
pub struct CatalogManager;
impl CatalogManager {
    pub fn new() -> Self;
    pub fn with_base_path<P: AsRef<Path>>(base_path: P) -> Self;
    pub fn load_catalog<T: CatalogLocation>(&mut self, location: &T) -> Result<T::CatalogType>;
    pub fn resolve_vehicle_reference(
        &mut self,
        reference: &VehicleCatalogReference,
        location: &VehicleCatalogLocation,
    ) -> Result<ResolvedCatalog<Vehicle>>;
    pub fn resolve_controller_reference(/* ... */) -> Result<ResolvedCatalog<Controller>>;
    pub fn resolve_pedestrian_reference(/* ... */) -> Result<ResolvedCatalog<Pedestrian>>;
    pub fn discover_and_load_catalogs(&mut self, locations: &CatalogLocations) -> Result<()>;
    pub fn parameter_engine(&mut self) -> &mut ParameterSubstitutionEngine;
    pub fn set_global_parameters(&mut self, parameters: HashMap<String, String>) -> Result<()>;
}

pub struct ResolvedCatalog<T> {
    pub entity: T,
    pub metadata: ResolutionMetadata,
}

pub struct ResolutionMetadata {
    pub catalog_path: String,
    pub entity_name: String,
    pub parameter_substitutions: HashMap<String, String>,
}
```

Free functions in `catalog`: `extract_scenario_parameters(&Option<ParameterDeclarations>) ->
HashMap<String, String>` and `resolve_catalog_reference_simple`.

`CatalogResolver` tracks a resolution stack to detect circular dependencies.
`ParameterSubstitutionEngine` handles substitution into catalog entities.

> **Name collision.** Two different types are called `CatalogManager`. The one re-exported at
> the crate root is `catalog::CatalogManager`, documented above. A second, unrelated
> `CatalogManager` in `catalog::resolver` holds per-kind catalog maps and is not re-exported.
> `openscenario_rs::CatalogManager` always means the first.

### Catalog references

`CatalogReference<T>` (`src/types/catalogs/references.rs`) carries a private `PhantomData`
field, so it cannot be built from a struct literal outside its module. Use the constructors:

```rust
impl<T: CatalogEntity> CatalogReference<T> {
    pub fn new(catalog_name: String, entry_name: String) -> Self;
    pub fn with_parameters(/* ... */) -> Self;
    pub fn get_catalog_name(&self, params: &HashMap<String, String>) -> Result<String>;
    pub fn get_entry_name(&self, params: &HashMap<String, String>) -> Result<String>;
}
```

`ParameterAssignment` and `ParameterAssignments` live in `types::catalogs::references`, not in
`types::basic`.

## Expressions

```rust
pub fn evaluate_expression<T>(expr: &str, params: &HashMap<String, String>) -> Result<T>;
```

The module also exposes `ExpressionParser::{new, parse}` and
`ExpressionEvaluator::{new, evaluate}` for working with the parsed `Expr` tree directly.

## Errors

```rust
pub type Result<T> = std::result::Result<T, Error>;
```

`Error` is a single `thiserror` enum with 26 variants:

| Group | Variants |
|---|---|
| XML and IO | `XmlParseError`, `XmlSerializeError`, `IoError`, `FileNotFound`, `DirectoryNotFound`, `FileReadError`, `FileWriteError` |
| Lookup | `EntityNotFound`, `CatalogEntryNotFound`, `CatalogNotFound`, `ParameterNotFound` |
| Validation | `ValidationError`, `MissingRequiredField`, `InvalidValue`, `OutOfRange`, `TypeMismatch`, `ConstraintViolation` |
| Structure | `InvalidXmlStructure`, `MalformedXml`, `ChoiceGroupError`, `InconsistentState` |
| Parameters and expressions | `ParameterError`, `CircularDependency`, `ParseError`, `ExpressionError` |
| Catalog | `CatalogError` |

Constructor helpers exist for most: `Error::file_not_found`, `entity_not_found`,
`catalog_not_found`, `validation_error`, `missing_field`, `invalid_value`, `out_of_range`,
`type_mismatch`, `parameter_error`, `parameter_not_found`, `invalid_xml`, `malformed_xml`,
`parsing_error`, `circular_dependency`, `parse_error`, `expression_error`,
`constraint_violation`, `catalog_error`, `choice_group_error`.

`Error::with_context(self, &str) -> Self` prefixes the message of a subset of variants. It is
a silent no-op on the rest, so do not rely on it to attach context universally.

> **Name collision.** Three distinct things are called `ValidationError`:
> `Error::ValidationError` (a variant of the enum above),
> `parser::validation::ValidationError` (a semantic-validation finding), and
> `validation::ValidationError` (a libxml schema diagnostic). Alias on import when more than
> one is in scope.

## Builder (feature `builder`)

Entry point and typestate transitions:

```rust
pub struct ScenarioBuilder<S>;
pub struct Empty; pub struct HasHeader; pub struct HasEntities; pub struct Complete;

impl ScenarioBuilder<Empty> {
    pub fn new() -> Self;
    pub fn with_header(self, description: &str, author: &str) -> ScenarioBuilder<HasHeader>;
}

impl ScenarioBuilder<HasHeader> {
    pub fn with_parameters(self, params: ParameterDeclarations) -> Self;
    pub fn add_parameter(self, name: &str, param_type: ParameterType, value: &str) -> Self;
    pub fn with_catalog_locations(self, locations: CatalogLocations) -> Self;
    pub fn with_road_network(self, network: RoadNetwork) -> Self;
    pub fn with_road_file(self, file_path: &str) -> Self;
    pub fn with_entities(self) -> ScenarioBuilder<HasEntities>;
}

impl ScenarioBuilder<HasEntities> {
    pub fn add_vehicle<F>(self, name: &str, config: F) -> Self;      // closure, not a builder
    pub fn add_vehicle_mut(&mut self, name: &str) -> VehicleBuilder<'_>;
    pub fn add_catalog_vehicle(/* ... */) -> CatalogVehicleBuilder<'_>;
    pub fn add_pedestrian<F>(self, name: &str, config: F) -> Self;
    pub fn add_catalog_pedestrian(/* ... */) -> CatalogPedestrianBuilder<'_>;
    pub fn with_storyboard<F>(self, config: F) -> ScenarioBuilder<Complete>;
    pub fn with_storyboard_mut(self) -> StoryboardBuilder;
    pub fn create_storyboard(self) -> StoryboardBuilder;
    pub fn build(self) -> BuilderResult<OpenScenario>;
}

impl ScenarioBuilder<Complete> {
    pub fn build(self) -> BuilderResult<OpenScenario>;
}
```

Note that `add_vehicle` and `add_pedestrian` take a configuring closure and return `Self`; the
builder-returning forms are the `_mut` variants. `with_storyboard` likewise takes a closure and
advances the state to `Complete`.

`build()` enforces the five elements the XSD requires of a scenario document and returns
`BuilderError::MissingField` for any that is absent: `file_header`, `entities`,
`catalog_locations`, `road_network` and `storyboard`. The first two are already guaranteed by
the typestate; the other three are runtime checks. `CatalogLocations::default()` and
`RoadNetwork::default()` are the correct values when a scenario references no catalogs and
names no road file: the elements are required even though all their children are optional.

### Errors

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

### Re-exports at `builder::`

Actions: `ActivateControllerActionBuilder`, `EntityActionBuilder`, `EnvironmentActionBuilder`,
`FollowTrajectoryActionBuilder`, `LaneChangeActionBuilder`, `LaneOffsetActionBuilder`,
`LateralDistanceActionBuilder`, `PolylineBuilder`, `SpeedActionBuilder`,
`TeleportActionBuilder`, `TrajectoryBuilder`, `VariableActionBuilder`, `VertexBuilder`.

Catalog: `CatalogEntityBuilder`, `CatalogLocationsBuilder`,
`PedestrianCatalogReferenceBuilder`, `VehicleCatalogReferenceBuilder`.

Conditions: `AccelerationConditionBuilder`, `CollisionConditionBuilder`,
`ParameterConditionBuilder`, `ReachPositionConditionBuilder`,
`RelativeDistanceConditionBuilder`, `SpeedConditionBuilder`, `TimeConditionBuilder`,
`TraveledDistanceConditionBuilder`, `TriggerBuilder`, `ValueSpeedConditionBuilder`,
`VariableConditionBuilder`.

Entities: `DetachedVehicleBuilder`, `VehicleBuilder`.

Init: `GlobalActionBuilder`, `InitActionBuilder`, `PrivateActionBuilder`.

Parameters: `ParameterContext`, `ParameterDeclarationsBuilder`, `ParameterizedValueBuilder`.

Storyboard: `ActBuilder`, `DetachedActBuilder`, `DetachedFollowTrajectoryActionBuilder`,
`DetachedManeuverBuilder`, `DetachedSpeedActionBuilder`, `DetachedStoryBuilder`,
`ManeuverBuilder`, `StoryBuilder`, `StoryboardBuilder`.

Templates: `BasicScenarioTemplate`, `ScenarioTemplate`.

Validation: `BuilderValidatable`, `BuilderValidationContext`, `ValidationContextBuilder`.

> `PedestrianBuilder`, `CatalogVehicleBuilder`, `CatalogPedestrianBuilder` and
> `DetachedPedestrianBuilder` exist but are **not** re-exported at `builder::`. Import them
> from `openscenario_rs::builder::entities`.

> **Name collision.** `builder::parameters::ParameterContext` is a different type from
> `types::ParameterContext`. The builder one has a private field and no `scope`.

## Binaries

| Binary | Required features | Purpose |
|---|---|---|
| `xosc-validate` | `validation` | Validate `.xosc` files against the XSD |
| `scenario_analyzer` | none | Inspect and report on a scenario file |

```bash
cargo run --bin xosc-validate --features validation -- scenario.xosc
cargo run --bin scenario_analyzer -- scenario.xosc
```
