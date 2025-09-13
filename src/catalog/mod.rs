//! Catalog system for OpenSCENARIO
//!
//! This module provides comprehensive catalog management for OpenSCENARIO scenarios:
//! - Loading and parsing catalog files from Directory paths
//! - Resolving catalog references within scenarios
//! - Full integration of catalog content into scenario structures
//!
//! The catalog system enables modular scenario design by allowing reusable
//! components (vehicles, controllers, routes, etc.) to be defined in separate
//! files and referenced from main scenarios.

use crate::types::basic::Directory;

use crate::types::catalogs::references::{VehicleCatalogReference, ControllerCatalogReference, PedestrianCatalogReference};
use crate::types::catalogs::locations::{VehicleCatalogLocation, ControllerCatalogLocation, PedestrianCatalogLocation, CatalogLocations};
use crate::types::entities::{vehicle::Vehicle};
use crate::types::controllers::Controller;

pub mod loader;
pub mod resolver;
pub mod parameters;

// Re-export key types for convenience
pub use loader::CatalogLoader;
pub use resolver::{CatalogResolver, ResolvedCatalog};
pub use parameters::ParameterSubstitutionEngine;

/// Trait for types that can be loaded from catalog directories
pub trait CatalogLocation {
    /// The type of catalog this location points to
    type CatalogType;
    
    /// Load the catalog from the directory path
    async fn load_catalog(&self) -> Result<Self::CatalogType, crate::error::Error>;
    
    /// Get the directory path for this catalog location
    fn directory(&self) -> &Directory;
}

/// Trait for catalog types that can resolve references
pub trait ResolvableCatalog {
    /// The type of entities this catalog contains
    type EntityType;
    
    /// Resolve a catalog reference to the actual entity
    fn resolve_reference(&self, reference_name: &str) -> Result<&Self::EntityType, crate::error::Error>;
    
    /// Get all available entity names in this catalog
    fn entity_names(&self) -> Vec<String>;
}

/// Trait for scenario types that can have their catalog references resolved
pub trait ScenarioResolver: Sized {
    /// Resolve all catalog references in this scenario
    async fn resolve_all_catalogs(self) -> Result<Self, crate::error::Error>;
}

/// Main catalog manager that coordinates loading and resolution
pub struct CatalogManager {
    loader: CatalogLoader,
    resolver: CatalogResolver,
    parameter_engine: ParameterSubstitutionEngine,
}

impl CatalogManager {
    /// Create a new catalog manager
    pub fn new() -> Self {
        Self {
            loader: CatalogLoader::new(),
            resolver: CatalogResolver::new(),
            parameter_engine: ParameterSubstitutionEngine::new(),
        }
    }

    /// Create a catalog manager with a specific base path for relative path resolution
    pub fn with_base_path<P: AsRef<std::path::Path>>(base_path: P) -> Self {
        Self {
            loader: CatalogLoader::with_base_path(base_path),
            resolver: CatalogResolver::new(),
            parameter_engine: ParameterSubstitutionEngine::new(),
        }
    }

    /// Load a catalog from a directory, using cache if available
    pub async fn load_catalog<T: CatalogLocation>(&mut self, location: &T) -> Result<T::CatalogType, crate::error::Error> {
        // Implementation will be added in loader module
        location.load_catalog().await
    }

    /// Resolve a vehicle catalog reference to an actual vehicle
    pub async fn resolve_vehicle_reference(
        &mut self, 
        reference: &VehicleCatalogReference,
        location: &VehicleCatalogLocation
    ) -> Result<ResolvedCatalog<Vehicle>, crate::error::Error> {
        use crate::types::catalogs::entities::CatalogEntity;
        
        // Start resolution tracking for circular dependency detection
        let reference_key = format!("vehicle:{}:{}", 
            reference.catalog_name.as_literal().unwrap_or(&"unknown".to_string()), 
            reference.entry_name.as_literal().unwrap_or(&"unknown".to_string()));
        self.resolver.begin_resolution(&reference_key)?;

        // Load catalog files from the location
        let catalog_vehicles = self.loader.load_vehicle_catalogs(&location.directory)?;
        
        // Find the specific vehicle
        let entry_name = reference.entry_name.as_literal()
            .ok_or_else(|| crate::error::Error::catalog_error("Cannot resolve parameterized entry names yet"))?;
            
        let catalog_vehicle = catalog_vehicles.iter()
            .find(|v| v.entity_name() == entry_name)
            .ok_or_else(|| crate::error::Error::catalog_error(&format!(
                "Vehicle '{}' not found in catalog", entry_name
            )))?;

        // Resolve parameters if any
        let mut parameters = std::collections::HashMap::new();
        if let Some(assignments) = &reference.parameter_assignments {
            for assignment in assignments.iter() {
                let resolved_name = assignment.parameter_ref.as_literal()
                    .ok_or_else(|| crate::error::Error::catalog_error("Cannot resolve parameterized parameter names"))?;
                let resolved_value = assignment.value.as_literal()
                    .ok_or_else(|| crate::error::Error::catalog_error("Cannot resolve parameterized parameter values"))?;
                parameters.insert(resolved_name.clone(), resolved_value.clone());
            }
        }

        // Convert catalog vehicle to scenario vehicle
        let resolved_vehicle = catalog_vehicle.clone().into_scenario_entity(parameters.clone())?;

        // End resolution tracking
        self.resolver.end_resolution(&reference_key);

        Ok(ResolvedCatalog::with_parameters(
            resolved_vehicle,
            "catalog_path".to_string(), // TODO: Get actual path
            entry_name.clone(),
            parameters,
        ))
    }

    /// Resolve a controller catalog reference to an actual controller
    pub async fn resolve_controller_reference(
        &mut self, 
        reference: &ControllerCatalogReference,
        location: &ControllerCatalogLocation
    ) -> Result<ResolvedCatalog<Controller>, crate::error::Error> {
        use crate::types::catalogs::entities::CatalogEntity;
        
        // Start resolution tracking
        let reference_key = format!("controller:{}:{}", 
            reference.catalog_name.as_literal().unwrap_or(&"unknown".to_string()), 
            reference.entry_name.as_literal().unwrap_or(&"unknown".to_string()));
        self.resolver.begin_resolution(&reference_key)?;

        // Load catalog files from the location
        let catalog_controllers = self.loader.load_controller_catalogs(&location.directory)?;
        
        // Find the specific controller
        let entry_name = reference.entry_name.as_literal()
            .ok_or_else(|| crate::error::Error::catalog_error("Cannot resolve parameterized entry names yet"))?;
            
        let catalog_controller = catalog_controllers.iter()
            .find(|c| c.entity_name() == entry_name)
            .ok_or_else(|| crate::error::Error::catalog_error(&format!(
                "Controller '{}' not found in catalog", entry_name
            )))?;

        // Resolve parameters
        let mut parameters = std::collections::HashMap::new();
        if let Some(assignments) = &reference.parameter_assignments {
            for assignment in assignments.iter() {
                let resolved_name = assignment.parameter_ref.as_literal()
                    .ok_or_else(|| crate::error::Error::catalog_error("Cannot resolve parameterized parameter names"))?;
                let resolved_value = assignment.value.as_literal()
                    .ok_or_else(|| crate::error::Error::catalog_error("Cannot resolve parameterized parameter values"))?;
                parameters.insert(resolved_name.clone(), resolved_value.clone());
            }
        }

        // Convert catalog controller to scenario controller
        let resolved_controller = catalog_controller.clone().into_scenario_entity(parameters.clone())?;

        // End resolution tracking
        self.resolver.end_resolution(&reference_key);

        Ok(ResolvedCatalog::with_parameters(
            resolved_controller,
            "catalog_path".to_string(), // TODO: Get actual path
            entry_name.clone(),
            parameters,
        ))
    }

    /// Resolve a pedestrian catalog reference to an actual pedestrian
    pub async fn resolve_pedestrian_reference(
        &mut self, 
        reference: &PedestrianCatalogReference,
        location: &PedestrianCatalogLocation
    ) -> Result<ResolvedCatalog<crate::types::entities::pedestrian::Pedestrian>, crate::error::Error> {
        use crate::types::catalogs::entities::CatalogEntity;
        
        // Start resolution tracking
        let reference_key = format!("pedestrian:{}:{}", 
            reference.catalog_name.as_literal().unwrap_or(&"unknown".to_string()), 
            reference.entry_name.as_literal().unwrap_or(&"unknown".to_string()));
        self.resolver.begin_resolution(&reference_key)?;

        // Load catalog files from the location
        let catalog_pedestrians = self.loader.load_pedestrian_catalogs(&location.directory)?;
        
        // Find the specific pedestrian
        let entry_name = reference.entry_name.as_literal()
            .ok_or_else(|| crate::error::Error::catalog_error("Cannot resolve parameterized entry names yet"))?;
            
        let catalog_pedestrian = catalog_pedestrians.iter()
            .find(|p| p.entity_name() == entry_name)
            .ok_or_else(|| crate::error::Error::catalog_error(&format!(
                "Pedestrian '{}' not found in catalog", entry_name
            )))?;

        // Resolve parameters
        let mut parameters = std::collections::HashMap::new();
        if let Some(assignments) = &reference.parameter_assignments {
            for assignment in assignments.iter() {
                let resolved_name = assignment.parameter_ref.as_literal()
                    .ok_or_else(|| crate::error::Error::catalog_error("Cannot resolve parameterized parameter names"))?;
                let resolved_value = assignment.value.as_literal()
                    .ok_or_else(|| crate::error::Error::catalog_error("Cannot resolve parameterized parameter values"))?;
                parameters.insert(resolved_name.clone(), resolved_value.clone());
            }
        }

        // Convert catalog pedestrian to scenario pedestrian
        let resolved_pedestrian = catalog_pedestrian.clone().into_scenario_entity(parameters.clone())?;

        // End resolution tracking
        self.resolver.end_resolution(&reference_key);

        Ok(ResolvedCatalog::with_parameters(
            resolved_pedestrian,
            "catalog_path".to_string(), // TODO: Get actual path
            entry_name.clone(),
            parameters,
        ))
    }

    /// Discover and load all catalogs from catalog locations
    pub async fn discover_and_load_catalogs(&mut self, locations: &CatalogLocations) -> Result<(), crate::error::Error> {
        // Discover vehicle catalogs
        if let Some(vehicle_location) = &locations.vehicle_catalog {
            if let Ok(files) = self.loader.discover_catalog_files(&vehicle_location.directory) {
                for file_path in files {
                    let _catalog = self.loader.load_and_parse_catalog_file(&file_path)?;
                    // Catalog loaded and validated successfully
                }
            }
        }

        // Discover controller catalogs
        if let Some(controller_location) = &locations.controller_catalog {
            if let Ok(files) = self.loader.discover_catalog_files(&controller_location.directory) {
                for file_path in files {
                    let _catalog = self.loader.load_and_parse_catalog_file(&file_path)?;
                    // Catalog loaded and validated successfully
                }
            }
        }

        // Discover pedestrian catalogs
        if let Some(pedestrian_location) = &locations.pedestrian_catalog {
            if let Ok(files) = self.loader.discover_catalog_files(&pedestrian_location.directory) {
                for file_path in files {
                    let _catalog = self.loader.load_and_parse_catalog_file(&file_path)?;
                    // Catalog loaded and validated successfully
                }
            }
        }

        // Additional catalog types can be added here

        Ok(())
    }



    /// Get access to the parameter engine for custom parameter operations
    pub fn parameter_engine(&mut self) -> &mut ParameterSubstitutionEngine {
        &mut self.parameter_engine
    }

    /// Set global parameters that will be used for all catalog resolutions
    pub fn set_global_parameters(&mut self, parameters: std::collections::HashMap<String, String>) -> Result<(), crate::error::Error> {
        self.parameter_engine.set_parameters(parameters)
    }
}

impl Default for CatalogManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_catalog_manager_creation() {
        let _manager = CatalogManager::new();
        // Manager created successfully
    }

    #[test]
    fn test_catalog_manager_with_base_path() {
        let _manager = CatalogManager::with_base_path("/tmp");
        // Manager with base path created successfully
    }

    #[test]
    fn test_catalog_manager_default() {
        let _manager = CatalogManager::default();
        // Default manager created successfully
    }

    #[test]
    fn test_catalog_manager_parameter_engine() {
        let mut manager = CatalogManager::new();
        
        // Set global parameters
        let mut params = std::collections::HashMap::new();
        params.insert("GlobalParam".to_string(), "GlobalValue".to_string());
        manager.set_global_parameters(params).unwrap();
        
        // Check parameter engine has the parameter
        assert_eq!(manager.parameter_engine().get_parameter("GlobalParam").unwrap(), "GlobalValue");
    }
}