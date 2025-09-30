//! Catalog system module for reusable scenario components
//!
//! This file contains:
//! - Base catalog traits and catalog management utilities
//! - Catalog reference resolution and dependency tracking
//! - Catalog validation and consistency checking
//! - Cross-catalog reference handling and circular dependency detection
//! - Catalog versioning and compatibility management
//!
//! Contributes to project by:
//! - Organizing catalog system into logical component categories
//! - Providing consistent framework for catalog reference resolution
//! - Supporting modular scenario design through reusable components
//! - Enabling catalog validation and dependency management
//! - Facilitating catalog evolution and version compatibility

pub mod controllers;
pub mod entities;
pub mod environments;
pub mod files;
pub mod locations;
pub mod references;
pub mod routes;
pub mod trajectories;

// Re-export catalog types with explicit imports to avoid ambiguity
// Main catalog containers
pub use controllers::ControllerCatalog;
pub use environments::EnvironmentCatalog;
pub use routes::RouteCatalog;
pub use trajectories::TrajectoryCatalog;

// Catalog entity types (from entities module - these are the primary definitions)
pub use entities::{
    CatalogEntity, CatalogVehicle, CatalogPedestrian, 
    CatalogMiscObject, CatalogManeuver,
    CatalogPerformance, CatalogAxles, CatalogFrontAxle, CatalogRearAxle, ParameterDefinition,
};

// Qualified re-exports to avoid conflicts - use entities module as primary
pub use entities::CatalogController;
pub use entities::CatalogEnvironment;
pub use entities::CatalogRoute;
pub use entities::CatalogTrajectory;

// Supporting types from specialized modules
pub use environments::{CatalogTimeOfDay, CatalogWeather, CatalogSun, CatalogFog, CatalogPrecipitation, CatalogRoadCondition};
pub use trajectories::{CatalogPolyline, CatalogVertex, CatalogClothoid, CatalogNurbs, CatalogTrajectoryShape};

// File and location types
pub use files::{CatalogFile, CatalogContent};
pub use locations::{
    VehicleCatalogLocation, ControllerCatalogLocation, PedestrianCatalogLocation,
    MiscObjectCatalogLocation, EnvironmentCatalogLocation, ManeuverCatalogLocation,
    TrajectoryCatalogLocation, RouteCatalogLocation, CatalogLocations,
};
pub use references::{CatalogReference, VehicleCatalogReference, ControllerCatalogReference, PedestrianCatalogReference, ParameterAssignment};

// Import necessary types for catalog groups
use serde::{Deserialize, Serialize};

/// Catalog type - container for all catalog entities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Catalog {
    #[serde(flatten)]
    pub content: CatalogContent,
}

/// CatalogDefinition group - XSD group wrapper for catalog sequence
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CatalogDefinition {
    #[serde(rename = "Catalog")]
    pub catalog: Catalog,
}

impl Default for Catalog {
    fn default() -> Self {
        Self {
            content: CatalogContent::default(),
        }
    }
}

impl Default for CatalogDefinition {
    fn default() -> Self {
        Self {
            catalog: Catalog::default(),
        }
    }
}

impl Catalog {
    /// Create new catalog with name
    pub fn new(name: String) -> Self {
        Self {
            content: CatalogContent::new(name),
        }
    }

    /// Create from existing catalog content
    pub fn from_content(content: CatalogContent) -> Self {
        Self { content }
    }

    /// Get catalog name
    pub fn name(&self) -> &str {
        self.content.name.as_literal().map_or("Unknown", |s| s)
    }
}

impl CatalogDefinition {
    /// Create new catalog definition
    pub fn new(catalog: Catalog) -> Self {
        Self { catalog }
    }

    /// Create with catalog name
    pub fn with_name(name: String) -> Self {
        Self {
            catalog: Catalog::new(name),
        }
    }

    /// Create from catalog content
    pub fn from_content(content: CatalogContent) -> Self {
        Self {
            catalog: Catalog::from_content(content),
        }
    }
}
