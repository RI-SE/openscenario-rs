//! OpenSCENARIO type system
//!
//! This module organizes all OpenSCENARIO data types into logical categories,
//! provides common traits and utilities, and establishes the foundation for
//! the complete type system covering 347+ specification types.

use std::collections::HashMap;

// Core type modules
pub mod basic;
pub mod enums;

// Entity and object types
pub mod entities;
pub mod geometry;

// Scenario structure
pub mod scenario;

// Actions and conditions (to be expanded)
pub mod actions;
pub mod conditions;
pub mod positions;

// Environment and world modeling (to be expanded)
pub mod environment;

// Advanced features (to be expanded)
pub mod catalogs;
pub mod controllers;
pub mod distributions;

// Road network types
pub mod road;

// Routing and navigation types
pub mod routing;

// Re-export commonly used types for convenience
pub use basic::{
    Boolean, Directory, Double, Int, OSString, ParameterDeclaration, ParameterDeclarations, Range,
    UnsignedInt, UnsignedShort, Value, ValueConstraint, ValueConstraintGroup,
};
pub use enums::{
    AngleType, AutomaticGearType, ColorType, ConditionEdge, ControllerType, DirectionalDimension,
    DynamicsDimension, DynamicsShape, FractionalCloudCover, LightMode, MiscObjectCategory,
    ObjectType, ParameterType, PedestrianCategory, PedestrianGestureType, PedestrianMotionType,
    PrecipitationType, Priority, Role, RouteStrategy, RoutingAlgorithm, Rule,
    TriggeringEntitiesRule, VehicleCategory, VehicleComponentType, VehicleLightType, Wetness,
};
pub use environment::{Environment, Fog, Precipitation, RoadCondition, Sun, TimeOfDay, Weather};
pub use scenario::init::{
    Actions, EnvironmentAction, GlobalAction, Init, LongitudinalAction, Private, PrivateActionType,
    PrivateActionWrapper,
};
pub use scenario::storyboard::{FileHeader, OpenScenario, OpenScenarioCategory, ScenarioDefinition, CatalogDefinition, Storyboard};

// Re-export distribution types
pub use distributions::{
    DeterministicMultiParameterDistribution,
    DeterministicMultiParameterDistributionType,
    // Deterministic types
    DeterministicParameterDistribution,
    DeterministicSingleParameterDistribution,
    DeterministicSingleParameterDistributionType,
    DistributionDefinition,
    DistributionRange,
    DistributionSampler,
    DistributionSet,
    DistributionSetElement,
    Histogram,
    HistogramBin,
    LogNormalDistribution,
    NormalDistribution,
    ParameterAssignment,
    ParameterValueDistribution,
    ParameterValueSet,
    PoissonDistribution,
    ProbabilityDistributionSet,
    ProbabilityDistributionSetElement,
    // Stochastic types
    StochasticDistribution,
    StochasticDistributionType,
    UniformDistribution,
    UserDefinedDistribution,
    ValidateDistribution,
    ValueSetDistribution,
};

// Re-export controller types
pub use controllers::{
    ActivateControllerAction, Controller, ControllerAssignment, ControllerDistribution,
    ControllerProperties, ObjectController, OverrideControllerValueAction,
};

// Re-export catalog location types
pub use catalogs::locations::{
    CatalogLocations, ControllerCatalogLocation, EnvironmentCatalogLocation,
    ManeuverCatalogLocation, MiscObjectCatalogLocation, PedestrianCatalogLocation,
    RouteCatalogLocation, TrajectoryCatalogLocation, VehicleCatalogLocation,
};

// Re-export condition types
pub use conditions::{
    AccelerationCondition, ByEntityCondition, ByValueCondition, DistanceCondition, ReachPositionCondition,
    RelativeDistanceCondition, SimulationTimeCondition, SpeedCondition, StandStillCondition,
};

// Re-export routing types
pub use routing::{Route, RouteRef, Waypoint};

// Re-export entity types
pub use entities::{Vehicle, Pedestrian, Entities, ScenarioObject, Axle, Axles};

/// Common trait for types that support validation
///
/// This trait will be implemented by all major OpenSCENARIO types to enable
/// comprehensive validation of scenario documents.
pub trait Validate {
    /// Validate this object using the provided validation context
    fn validate(&self, ctx: &ValidationContext) -> crate::Result<()>;
}

/// Common trait for types that support parameter resolution
///
/// This trait enables resolution of ${parameter} references to their actual values
/// using a parameter context containing the parameter definitions.
pub trait Resolve<T> {
    /// Resolve any parameters in this object using the provided context
    fn resolve(&self, ctx: &ParameterContext) -> crate::Result<T>;
}

/// Context for validation operations
///
/// Contains references to entities, catalogs, and other global state needed
/// for validating cross-references and constraints.
#[derive(Debug, Default)]
pub struct ValidationContext {
    /// Registry of all entities in the scenario for reference validation
    pub entities: HashMap<std::string::String, EntityRef>,
    /// Available catalog entries for catalog reference validation  
    pub catalogs: HashMap<std::string::String, CatalogRef>,
    /// Validation settings and options
    pub strict_mode: bool,
}

impl ValidationContext {
    /// Create a new validation context
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable strict validation mode
    pub fn with_strict_mode(mut self) -> Self {
        self.strict_mode = true;
        self
    }

    /// Add an entity to the validation registry
    pub fn add_entity(&mut self, name: std::string::String, entity_ref: EntityRef) {
        self.entities.insert(name, entity_ref);
    }
}

/// Context for parameter resolution
///
/// Contains the parameter values and scope information needed to resolve
/// ${parameter} references to their actual values.
#[derive(Debug, Default)]
pub struct ParameterContext {
    /// Current parameter values by name
    pub parameters: HashMap<std::string::String, std::string::String>,
    /// Parameter declaration scope (for nested parameter sets)
    pub scope: Vec<std::string::String>,
}

impl ParameterContext {
    /// Create a new parameter context
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a parameter value
    pub fn with_parameter(mut self, name: std::string::String, value: std::string::String) -> Self {
        self.parameters.insert(name, value);
        self
    }

    /// Get a parameter value by name
    pub fn get(&self, name: &str) -> Option<&str> {
        self.parameters.get(name).map(|s| s.as_str())
    }
}

/// Reference to an entity in the scenario
#[derive(Debug, Clone)]
pub struct EntityRef {
    /// Entity name
    pub name: std::string::String,
    /// Entity type
    pub object_type: ObjectType,
}

/// Reference to a catalog entry  
#[derive(Debug, Clone)]
pub struct CatalogRef {
    /// Catalog name
    pub catalog: std::string::String,
    /// Entry name within the catalog
    pub entry: std::string::String,
}

// Default implementations for basic types
impl<T> Validate for Value<T> {
    fn validate(&self, _ctx: &ValidationContext) -> crate::Result<()> {
        // Basic Value<T> types don't need validation beyond their own constraints
        Ok(())
    }
}

impl<T: Clone> Resolve<T> for Value<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    fn resolve(&self, ctx: &ParameterContext) -> crate::Result<T> {
        self.resolve(&ctx.parameters)
    }
}
