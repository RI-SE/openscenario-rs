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
//! use openscenario_rs::ScenarioBuilder;
//!
//! let scenario = ScenarioBuilder::new()
//!     .with_header("Highway Test", "Test Author")
//!     .with_entities()
//!         .add_vehicle("ego")
//!             .car()
//!             .finish()
//!     .with_storyboard()
//!         .add_story("main_story")
//!             .add_act("acceleration_act")
//!                 .add_maneuver("speed_up", "ego")
//!                     .add_speed_action()
//!                         .to_speed(30.0)
//!                         .finish()
//!                         .unwrap()
//!                     .finish()
//!                 .finish()
//!             .finish()
//!         .finish()
//!     .build()
//!     .unwrap();
//! ```

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
    /// description, author, and creation timestamp. This method automatically
    /// sets the OpenSCENARIO version to 1.0 and uses the current timestamp.
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
        #[cfg(feature = "chrono")]
        let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();
        #[cfg(not(feature = "chrono"))]
        let now = "2024-01-01T00:00:00".to_string();

        self.data.file_header = Some(FileHeader {
            rev_major: UnsignedShort::literal(1),
            rev_minor: UnsignedShort::literal(0),
            date: OSString::literal(now),
            description: OSString::literal(description.to_string()),
            author: OSString::literal(author.to_string()),
        });

        ScenarioBuilder {
            _state: PhantomData,
            data: self.data,
        }
    }
}

// Implementation for HasHeader state
impl ScenarioBuilder<HasHeader> {
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
        let file_header = self
            .data
            .file_header
            .ok_or_else(|| BuilderError::missing_field("file_header", ".with_header()"))?;

        let entities = self
            .data
            .entities
            .ok_or_else(|| BuilderError::missing_field("entities", ".with_entities()"))?;

        let storyboard = self
            .data
            .storyboard
            .ok_or_else(|| BuilderError::missing_field("storyboard", ".with_storyboard()"))?;

        Ok(OpenScenario {
            file_header,
            parameter_declarations: self.data.parameter_declarations,
            variable_declarations: None,
            monitor_declarations: None,
            catalog_locations: self.data.catalog_locations,
            road_network: self.data.road_network,
            entities: Some(entities),
            storyboard: Some(storyboard),
            parameter_value_distribution: None,
            catalog: None,
        })
    }
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
        let file_header = self
            .data
            .file_header
            .ok_or_else(|| BuilderError::missing_field("file_header", ".with_header()"))?;

        let entities = self
            .data
            .entities
            .ok_or_else(|| BuilderError::missing_field("entities", ".with_entities()"))?;

        let storyboard = self
            .data
            .storyboard
            .ok_or_else(|| BuilderError::missing_field("storyboard", ".with_storyboard()"))?;

        Ok(OpenScenario {
            file_header,
            parameter_declarations: self.data.parameter_declarations,
            variable_declarations: None,
            monitor_declarations: None,
            catalog_locations: self.data.catalog_locations,
            road_network: self.data.road_network,
            entities: Some(entities),
            storyboard: Some(storyboard),
            parameter_value_distribution: None,
            catalog: None,
        })
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

    #[test]
    fn test_minimal_scenario_builder() {
        let scenario = ScenarioBuilder::new()
            .with_header("Test Scenario", "Test Author")
            .with_entities()
            .build()
            .unwrap();

        // Verify basic structure
        if let crate::types::basic::Value::Literal(desc) = &scenario.file_header.description {
            assert_eq!(desc, "Test Scenario");
        } else {
            panic!("Description should be literal");
        }

        assert!(scenario.entities.is_some());
        assert!(scenario.storyboard.is_some());
    }
}
