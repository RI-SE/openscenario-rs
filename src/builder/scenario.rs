//! Core scenario builder for programmatic scenario construction
//!
//! This module provides the main [`ScenarioBuilder`] type that enables type-safe,
//! fluent construction of OpenSCENARIO documents. The builder uses compile-time
//! state validation to ensure scenarios are constructed in the correct order.
//!
//! # Type States
//!
//! The builder progresses through several type states:
//! - [`Empty`] → [`HasHeader`] → [`HasEntities`] → [`Complete`]
//!
//! Each state transition unlocks new methods while preventing invalid operations.
//!
//! # Example
//!
//! ```rust
//! use openscenario_rs::types::catalogs::locations::CatalogLocations;
//! use openscenario_rs::types::road::RoadNetwork;
//! use openscenario_rs::ScenarioBuilder;
//!
//! // CatalogLocations and RoadNetwork are required of a scenario document by the XSD.
//! let scenario = ScenarioBuilder::new()
//!     .with_header("Highway Test", "Test Author")
//!     .with_catalog_locations(CatalogLocations::default())
//!     .with_road_network(RoadNetwork::default())
//!     .with_entities()
//!         .add_vehicle("ego", |v| v.car())
//!     .with_storyboard(|storyboard| {
//!         storyboard
//!     })
//!     .build()
//!     .unwrap();
//! ```

use super::validation::ValidationContextBuilder;
use super::{BuilderError, BuilderResult};
use crate::types::{
    basic::{OSString, ParameterDeclaration, ParameterDeclarations, UnsignedShort},
    catalogs::locations::CatalogLocations,
    entities::Entities,
    enums::ParameterType,
    road::RoadNetwork,
    scenario::storyboard::{FileHeader, OpenScenario, Storyboard},
};
use std::marker::PhantomData;

/// Initial state - scenario builder has just been created
#[derive(Debug)]
pub struct Empty;

/// Header has been set - can now add optional components like catalogs and parameters
#[derive(Debug)]
pub struct HasHeader;

/// Entities have been initialized - can now add entities and build storyboard
#[derive(Debug)]
pub struct HasEntities;

/// Scenario is complete with storyboard - ready to build final document
#[derive(Debug)]
pub struct Complete;

/// Type-safe scenario builder with compile-time state validation
///
/// The `ScenarioBuilder` uses the type system to enforce correct construction order.
/// Each state transition is validated at compile time, preventing runtime errors
/// from incomplete or incorrectly ordered scenario construction.
///
/// # Type Parameters
///
/// - `S`: The current state of the builder (Empty, HasHeader, HasEntities, or Complete)
///
/// # State Transitions
///
/// ```text
/// Empty --with_header()--> HasHeader --with_entities()--> HasEntities --with_storyboard()--> Complete
///   |                         |                              |                                    |
///   new()                     add_parameter()                add_vehicle()                       build()
///                             with_catalog_locations()       add_pedestrian()
///                             with_road_network()
/// ```
pub struct ScenarioBuilder<S> {
    _state: PhantomData<S>,
    pub(crate) data: PartialScenarioData,
}

#[derive(Debug, Default)]
pub(crate) struct PartialScenarioData {
    pub(crate) file_header: Option<FileHeader>,
    pub(crate) parameter_declarations: Option<ParameterDeclarations>,
    pub(crate) catalog_locations: Option<CatalogLocations>,
    pub(crate) road_network: Option<RoadNetwork>,
    pub(crate) entities: Option<Entities>,
    pub(crate) storyboard: Option<Storyboard>,
}

// Implementation for Empty state (starting point)
impl ScenarioBuilder<Empty> {
    /// Create a new scenario builder in the initial Empty state
    ///
    /// This is the entry point for all scenario construction. The builder starts
    /// in the Empty state and must progress through the required states to build
    /// a valid OpenSCENARIO document.
    ///
    /// # Example
    ///
    /// ```rust
    /// use openscenario_rs::ScenarioBuilder;
    ///
    /// let builder = ScenarioBuilder::new();
    /// // Must call .with_header() next
    /// ```
    pub fn new() -> Self {
        Self {
            _state: PhantomData,
            data: PartialScenarioData::default(),
        }
    }

    /// Set file header information and transition to HasHeader state
    ///
    /// The file header contains essential metadata about the scenario including
    /// description, author, and creation timestamp. The revision defaults to **1.3**, the
    /// version of the standard this crate targets and validates against; override it with
    /// [`ScenarioBuilder::with_revision`] when writing for an older consumer.
    ///
    /// # Arguments
    ///
    /// * `description` - Human-readable description of the scenario
    /// * `author` - Name of the scenario author/creator
    ///
    /// # Returns
    ///
    /// A `ScenarioBuilder<HasHeader>` that can accept optional components like
    /// parameters, catalogs, and road networks before adding entities.
    ///
    /// # Example
    ///
    /// ```rust
    /// use openscenario_rs::ScenarioBuilder;
    ///
    /// let builder = ScenarioBuilder::new()
    ///     .with_header("Highway overtaking scenario", "John Doe");
    /// // Can now add parameters, catalogs, or entities
    /// ```
    pub fn with_header(mut self, description: &str, author: &str) -> ScenarioBuilder<HasHeader> {
        let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();

        self.data.file_header = Some(FileHeader {
            rev_major: UnsignedShort::literal(1),
            rev_minor: UnsignedShort::literal(3),
            date: OSString::literal(now),
            description: OSString::literal(description.to_string()),
            author: OSString::literal(author.to_string()),
            license: None,
            properties: None,
        });

        ScenarioBuilder {
            _state: PhantomData,
            data: self.data,
        }
    }
}

// Implementation for HasHeader state
impl ScenarioBuilder<HasHeader> {
    /// Override the OpenSCENARIO revision recorded in the file header
    ///
    /// [`ScenarioBuilder::with_header`] defaults to 1.3, which is what this crate targets.
    /// Set it lower only when the consumer requires an earlier revision; note that the
    /// document is still built from the 1.3 type model, so declaring an older revision does
    /// not restrict what the builder emits.
    ///
    /// # Example
    ///
    /// ```rust
    /// use openscenario_rs::ScenarioBuilder;
    ///
    /// let builder = ScenarioBuilder::new()
    ///     .with_header("Legacy consumer", "Author")
    ///     .with_revision(1, 0);
    /// ```
    pub fn with_revision(mut self, major: u16, minor: u16) -> Self {
        if let Some(header) = &mut self.data.file_header {
            header.rev_major = UnsignedShort::literal(major);
            header.rev_minor = UnsignedShort::literal(minor);
        }
        self
    }

    /// Add parameter declarations to the scenario
    ///
    /// Parameters allow scenarios to be configurable and reusable. This method
    /// accepts a complete `ParameterDeclarations` structure with multiple parameters.
    ///
    /// # Arguments
    ///
    /// * `params` - Complete parameter declarations structure
    ///
    /// # Example
    ///
    /// ```rust
    /// use openscenario_rs::{ScenarioBuilder, types::basic::ParameterDeclarations};
    ///
    /// let params = ParameterDeclarations::default(); // Build your parameters
    /// let builder = ScenarioBuilder::new()
    ///     .with_header("Test", "Author")
    ///     .with_parameters(params);
    /// ```
    pub fn with_parameters(mut self, params: ParameterDeclarations) -> Self {
        self.data.parameter_declarations = Some(params);
        self
    }

    /// Add a single parameter declaration (convenience method)
    ///
    /// This is a convenience method for adding individual parameters without
    /// constructing the full `ParameterDeclarations` structure manually.
    /// Multiple calls to this method will accumulate parameters.
    ///
    /// # Arguments
    ///
    /// * `name` - Parameter name (used in `${name}` references)
    /// * `param_type` - Type of the parameter (Double, Integer, String, etc.)
    /// * `value` - Default value for the parameter
    ///
    /// # Example
    ///
    /// ```rust
    /// use openscenario_rs::{ScenarioBuilder, types::enums::ParameterType};
    ///
    /// let builder = ScenarioBuilder::new()
    ///     .with_header("Test", "Author")
    ///     .add_parameter("initial_speed", ParameterType::Double, "25.0")
    ///     .add_parameter("target_lane", ParameterType::String, "1");
    /// ```
    pub fn add_parameter(mut self, name: &str, param_type: ParameterType, value: &str) -> Self {
        let mut params = self.data.parameter_declarations.take().unwrap_or_default();

        params.parameter_declarations.push(ParameterDeclaration {
            name: OSString::literal(name.to_string()),
            parameter_type: param_type,
            value: OSString::literal(value.to_string()),
            constraint_groups: Vec::new(),
        });

        self.data.parameter_declarations = Some(params);
        self
    }

    /// Add catalog locations (optional)
    pub fn with_catalog_locations(mut self, locations: CatalogLocations) -> Self {
        self.data.catalog_locations = Some(locations);
        self
    }

    /// Add road network (optional for minimal scenarios)
    pub fn with_road_network(mut self, network: RoadNetwork) -> Self {
        self.data.road_network = Some(network);
        self
    }

    /// Set road network from OpenDRIVE file
    pub fn with_road_file(mut self, file_path: &str) -> Self {
        self.data.road_network = Some(RoadNetwork {
            logic_file: Some(crate::types::road::LogicFile {
                filepath: OSString::literal(file_path.to_string()),
            }),
            scene_graph_file: None,
            traffic_signals: None,
            used_area: None,
        });
        self
    }

    /// Initialize entities and progress to HasEntities state
    pub fn with_entities(mut self) -> ScenarioBuilder<HasEntities> {
        self.data.entities = Some(Entities::new());

        ScenarioBuilder {
            _state: PhantomData,
            data: self.data,
        }
    }
}

// Implementation for HasEntities state
impl ScenarioBuilder<HasEntities> {
    /// Add a vehicle entity using closure-based configuration
    pub fn add_vehicle<F>(mut self, name: &str, config: F) -> Self
    where
        F: FnOnce(
            crate::builder::entities::DetachedVehicleBuilder,
        ) -> crate::builder::entities::DetachedVehicleBuilder,
    {
        let vehicle_builder = crate::builder::entities::DetachedVehicleBuilder::new(name);
        let configured_builder = config(vehicle_builder);
        let vehicle_object = configured_builder.build();

        // Add to entities
        if let Some(ref mut entities) = self.data.entities {
            entities.add_object(vehicle_object);
        }

        self
    }

    /// Add a vehicle entity (legacy method for backward compatibility)
    pub fn add_vehicle_mut(&mut self, name: &str) -> crate::builder::entities::VehicleBuilder<'_> {
        crate::builder::entities::VehicleBuilder::new(self, name)
    }

    /// Add a vehicle from catalog
    pub fn add_catalog_vehicle(
        &mut self,
        name: &str,
    ) -> crate::builder::entities::catalog::CatalogVehicleBuilder<'_> {
        crate::builder::entities::catalog::CatalogVehicleBuilder::new(self, name)
    }

    /// Add a pedestrian entity using closure-based configuration
    pub fn add_pedestrian<F>(mut self, name: &str, config: F) -> Self
    where
        F: FnOnce(
            crate::builder::entities::DetachedPedestrianBuilder,
        ) -> crate::builder::entities::DetachedPedestrianBuilder,
    {
        let pedestrian_builder = crate::builder::entities::DetachedPedestrianBuilder::new(name);
        let configured_builder = config(pedestrian_builder);
        let pedestrian_object = configured_builder.build();

        // Add to entities
        if let Some(ref mut entities) = self.data.entities {
            entities.add_object(pedestrian_object);
        }

        self
    }

    /// Add a pedestrian from catalog
    pub fn add_catalog_pedestrian(
        &mut self,
        name: &str,
    ) -> crate::builder::entities::catalog::CatalogPedestrianBuilder<'_> {
        crate::builder::entities::catalog::CatalogPedestrianBuilder::new(self, name)
    }

    /// Configure storyboard using closure-based pattern
    pub fn with_storyboard<F>(self, config: F) -> ScenarioBuilder<Complete>
    where
        F: FnOnce(
            crate::builder::storyboard::StoryboardBuilder,
        ) -> crate::builder::storyboard::StoryboardBuilder,
    {
        let storyboard_builder = crate::builder::storyboard::StoryboardBuilder::new(self);
        let configured_builder = config(storyboard_builder);
        configured_builder.finish()
    }

    /// Start building the storyboard (legacy method)
    pub fn with_storyboard_mut(self) -> crate::builder::storyboard::StoryboardBuilder {
        crate::builder::storyboard::StoryboardBuilder::new(self)
    }

    /// Create a storyboard builder (alias for with_storyboard_mut)
    pub fn create_storyboard(self) -> crate::builder::storyboard::StoryboardBuilder {
        crate::builder::storyboard::StoryboardBuilder::new(self)
    }

    /// Build the final OpenScenario document
    pub fn build(self) -> BuilderResult<OpenScenario> {
        build_scenario(self.data)
    }
}

/// Assembles and validates the final document.
///
/// `ScenarioBuilder<HasEntities>` and `ScenarioBuilder<Complete>` both expose `build()` and both
/// produce exactly the same document, so the assembly lives here rather than being duplicated in
/// each impl.
///
/// Every scenario is run through [`BuilderValidationContext`] before it is returned. Those rules
/// – entity references resolve, the storyboard hierarchy is populated – catch the semantic
/// mistakes XSD validation cannot see, and returning `Err` here is the difference between the
/// caller learning about a broken scenario at `build()` and shipping invalid XML.
///
/// # Required elements
///
/// The output type is [`OpenScenario`], the flattened union of the schema's three document
/// kinds, so every field it carries is `Option`. A *scenario* document is narrower than that:
/// the XSD group `ScenarioDefinition` (`Schema/OpenSCENARIO.xsd:1989`) declares
/// `CatalogLocations` and `RoadNetwork` without `minOccurs="0"`, which makes both required
/// alongside `Entities` and `Storyboard`. Nothing in `OpenScenario` enforces that, so this
/// function does: omitting either one produced schema-invalid XML with no error at all.
///
/// They are rejected rather than defaulted. An empty-but-present `CatalogLocations` would
/// satisfy the validator while stating something the caller never wrote, which is the
/// invented-default pattern the type model removed elsewhere.
fn build_scenario(data: PartialScenarioData) -> BuilderResult<OpenScenario> {
    let file_header = data
        .file_header
        .ok_or_else(|| BuilderError::missing_field("file_header", ".with_header()"))?;

    let entities = data
        .entities
        .ok_or_else(|| BuilderError::missing_field("entities", ".with_entities()"))?;

    let storyboard = data
        .storyboard
        .ok_or_else(|| BuilderError::missing_field("storyboard", ".with_storyboard()"))?;

    // The suggestions name the empty-but-present form deliberately: the schema requires both
    // elements, but every child of each is optional, so a scenario with no catalogs and no road
    // file still has to emit them.
    let catalog_locations = data.catalog_locations.ok_or_else(|| {
        BuilderError::missing_field(
            "catalog_locations",
            ".with_catalog_locations(CatalogLocations::default())",
        )
    })?;

    let road_network = data.road_network.ok_or_else(|| {
        BuilderError::missing_field(
            "road_network",
            ".with_road_file(path) or .with_road_network(RoadNetwork::default())",
        )
    })?;

    let scenario = OpenScenario {
        file_header,
        parameter_declarations: data.parameter_declarations,
        variable_declarations: None,
        monitor_declarations: None,
        catalog_locations: Some(catalog_locations),
        road_network: Some(road_network),
        entities: Some(entities),
        storyboard: Some(storyboard),
        parameter_value_distribution: None,
        catalog: None,
    };

    // The rules resolve entity references against what the scenario actually declares, so the
    // context is seeded from the assembled document rather than from builder state.
    let mut context = ValidationContextBuilder::new().with_standard_rules();
    if let Some(entities) = &scenario.entities {
        for object in &entities.scenario_objects {
            context = context.with_entity(&object.name.to_string(), "ScenarioObject");
        }
    }
    context.build().validate_scenario(&scenario)?;

    Ok(scenario)
}

// Implementation for Complete state (final scenarios with storyboard)
impl ScenarioBuilder<Complete> {
    /// Create a Complete state builder from existing data (internal use)
    pub(crate) fn from_data_complete(data: PartialScenarioData) -> Self {
        Self {
            _state: PhantomData,
            data,
        }
    }

    /// Build the final scenario (same as HasEntities but with Complete state)
    pub fn build(self) -> BuilderResult<OpenScenario> {
        build_scenario(self.data)
    }
}

impl Default for ScenarioBuilder<Empty> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::types::catalogs::locations::CatalogLocations;
    use crate::types::road::RoadNetwork;

    /// The smallest chain that builds a schema-valid scenario document.
    ///
    /// `CatalogLocations` and `RoadNetwork` are set to their empty forms rather than omitted:
    /// the XSD requires both elements of a scenario document, and every child of each is
    /// optional, so this is the minimum a document can state and still conform.
    fn minimal_builder() -> ScenarioBuilder<Complete> {
        ScenarioBuilder::new()
            .with_header("Test Scenario", "Test Author")
            .with_catalog_locations(CatalogLocations::default())
            .with_road_network(RoadNetwork::default())
            .with_entities()
            .with_storyboard(|storyboard| storyboard)
    }

    #[test]
    fn test_minimal_scenario_builder() {
        let scenario = minimal_builder().build().unwrap();

        // Verify basic structure
        if let crate::types::basic::Value::Literal(desc) = &scenario.file_header.description {
            assert_eq!(desc, "Test Scenario");
        } else {
            panic!("Description should be literal");
        }

        assert!(scenario.entities.is_some());
        assert!(scenario.storyboard.is_some());
        assert!(scenario.catalog_locations.is_some());
        assert!(scenario.road_network.is_some());
    }

    /// The header defaults to the revision this crate targets, not to 1.0.
    #[test]
    fn header_defaults_to_revision_1_3() {
        let scenario = minimal_builder().build().unwrap();

        assert_eq!(scenario.file_header.rev_major.as_literal().unwrap(), &1);
        assert_eq!(scenario.file_header.rev_minor.as_literal().unwrap(), &3);
    }

    #[test]
    fn with_revision_overrides_the_default() {
        let scenario = ScenarioBuilder::new()
            .with_header("Legacy consumer", "Test Author")
            .with_revision(1, 0)
            .with_catalog_locations(CatalogLocations::default())
            .with_road_network(RoadNetwork::default())
            .with_entities()
            .with_storyboard(|storyboard| storyboard)
            .build()
            .unwrap();

        assert_eq!(scenario.file_header.rev_minor.as_literal().unwrap(), &0);
    }

    /// Regression: omitting either element used to serialize to schema-invalid XML silently.
    /// The XSD group `ScenarioDefinition` requires both, so `build()` has to refuse.
    #[test]
    fn build_rejects_missing_catalog_locations() {
        let err = ScenarioBuilder::new()
            .with_header("Test Scenario", "Test Author")
            .with_road_network(RoadNetwork::default())
            .with_entities()
            .with_storyboard(|storyboard| storyboard)
            .build()
            .unwrap_err();

        assert!(
            matches!(&err, BuilderError::MissingField { field, .. } if field == "catalog_locations"),
            "expected a missing catalog_locations error, got {err:?}"
        );
    }

    #[test]
    fn build_rejects_missing_road_network() {
        let err = ScenarioBuilder::new()
            .with_header("Test Scenario", "Test Author")
            .with_catalog_locations(CatalogLocations::default())
            .with_entities()
            .with_storyboard(|storyboard| storyboard)
            .build()
            .unwrap_err();

        assert!(
            matches!(&err, BuilderError::MissingField { field, .. } if field == "road_network"),
            "expected a missing road_network error, got {err:?}"
        );
    }

    /// The whole point of the change: the minimal document now validates against the schema.
    #[cfg(feature = "validation")]
    #[test]
    fn minimal_scenario_is_schema_valid() {
        let scenario = minimal_builder().build().unwrap();
        let xml = crate::serialize_to_string(&scenario).unwrap();

        let mut validator =
            crate::validation::XsdValidator::from_schema_file("Schema/OpenSCENARIO.xsd").unwrap();
        let errors = validator.validate_str(&xml).unwrap();

        assert!(
            errors.is_empty(),
            "expected schema-valid output, got {errors:?}"
        );
    }
}
