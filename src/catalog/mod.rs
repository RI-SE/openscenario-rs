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

use crate::types::catalogs::locations::{
    CatalogLocations, ControllerCatalogLocation, PedestrianCatalogLocation, VehicleCatalogLocation,
};
use crate::types::catalogs::references::{
    ControllerCatalogReference, PedestrianCatalogReference, VehicleCatalogReference,
};
use crate::types::controllers::Controller;
use crate::types::entities::vehicle::Vehicle;

pub mod loader;
pub mod parameters;
pub mod resolver;

// Re-export key types for convenience
pub use loader::CatalogLoader;
pub use parameters::ParameterSubstitutionEngine;
pub use resolver::{CatalogResolver, ResolvedCatalog};

/// Trait for types that can be loaded from catalog directories
pub trait CatalogLocation {
    /// The type of catalog this location points to
    type CatalogType;

    /// Load the catalog from the directory path
    fn load_catalog(&self) -> Result<Self::CatalogType, crate::error::Error>;

    /// Get the directory path for this catalog location
    fn directory(&self) -> &Directory;
}

/// Trait for catalog types that can resolve references
pub trait ResolvableCatalog {
    /// The type of entities this catalog contains
    type EntityType;

    /// Resolve a catalog reference to the actual entity
    fn resolve_reference(
        &self,
        reference_name: &str,
    ) -> Result<&Self::EntityType, crate::error::Error>;

    /// Get all available entity names in this catalog
    fn entity_names(&self) -> Vec<String>;
}

/// Trait for scenario types that can have their catalog references resolved
pub trait ScenarioResolver: Sized {
    /// Resolve all catalog references in this scenario
    fn resolve_all_catalogs(self) -> Result<Self, crate::error::Error>;
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
    pub fn load_catalog<T: CatalogLocation>(
        &mut self,
        location: &T,
    ) -> Result<T::CatalogType, crate::error::Error> {
        // Implementation will be added in loader module
        location.load_catalog()
    }

    /// Resolve a vehicle catalog reference to an actual vehicle
    pub fn resolve_vehicle_reference(
        &mut self,
        reference: &VehicleCatalogReference,
        location: &VehicleCatalogLocation,
    ) -> Result<ResolvedCatalog<Vehicle>, crate::error::Error> {
        use crate::types::catalogs::entities::CatalogEntity;

        // Start resolution tracking for circular dependency detection
        let reference_key = format!(
            "vehicle:{}:{}",
            reference
                .catalog_name
                .as_literal()
                .unwrap_or(&"unknown".to_string()),
            reference
                .entry_name
                .as_literal()
                .unwrap_or(&"unknown".to_string())
        );
        self.resolver.begin_resolution(&reference_key)?;

        // Load catalog files from the location and track file paths
        let catalog_files = self.loader.discover_catalog_files(&location.directory)?;
        let mut catalog_vehicle = None;
        let mut catalog_file_path = String::new();

        // Find the specific vehicle across all catalog files
        let entry_name = reference.entry_name.as_literal().ok_or_else(|| {
            crate::error::Error::catalog_error("Cannot resolve parameterized entry names yet")
        })?;

        for file_path in catalog_files {
            let catalog = self.loader.load_and_parse_catalog_file(&file_path)?;
            for vehicle in catalog.vehicles() {
                if vehicle.entity_name() == entry_name {
                    catalog_vehicle = Some(vehicle.clone());
                    catalog_file_path = file_path.to_string_lossy().to_string();
                    break;
                }
            }
            if catalog_vehicle.is_some() {
                break;
            }
        }

        let catalog_vehicle = catalog_vehicle.ok_or_else(|| {
            crate::error::Error::catalog_error(&format!(
                "Vehicle '{}' not found in catalog",
                entry_name
            ))
        })?;

        // Resolve parameters if any
        let mut parameters = std::collections::HashMap::new();
        if let Some(assignments) = &reference.parameter_assignments {
            for assignment in assignments.iter() {
                let resolved_name = assignment.parameter_ref.as_literal().ok_or_else(|| {
                    crate::error::Error::catalog_error(
                        "Cannot resolve parameterized parameter names",
                    )
                })?;
                let resolved_value = assignment.value.as_literal().ok_or_else(|| {
                    crate::error::Error::catalog_error(
                        "Cannot resolve parameterized parameter values",
                    )
                })?;
                parameters.insert(resolved_name.clone(), resolved_value.clone());
            }
        }

        // Convert catalog vehicle to scenario vehicle
        let resolved_vehicle = catalog_vehicle
            .clone()
            .into_scenario_entity(parameters.clone())?;

        // End resolution tracking
        self.resolver.end_resolution(&reference_key);

        Ok(ResolvedCatalog::with_parameters(
            resolved_vehicle,
            catalog_file_path,
            entry_name.clone(),
            parameters,
        ))
    }

    /// Resolve a controller catalog reference to an actual controller
    pub fn resolve_controller_reference(
        &mut self,
        reference: &ControllerCatalogReference,
        location: &ControllerCatalogLocation,
    ) -> Result<ResolvedCatalog<Controller>, crate::error::Error> {
        use crate::types::catalogs::entities::CatalogEntity;

        // Start resolution tracking
        let reference_key = format!(
            "controller:{}:{}",
            reference
                .catalog_name
                .as_literal()
                .unwrap_or(&"unknown".to_string()),
            reference
                .entry_name
                .as_literal()
                .unwrap_or(&"unknown".to_string())
        );
        self.resolver.begin_resolution(&reference_key)?;

        // Load catalog files from the location and track file paths
        let catalog_files = self.loader.discover_catalog_files(&location.directory)?;
        let mut catalog_controller = None;
        let mut catalog_file_path = String::new();

        // Find the specific controller across all catalog files
        let entry_name = reference.entry_name.as_literal().ok_or_else(|| {
            crate::error::Error::catalog_error("Cannot resolve parameterized entry names yet")
        })?;

        for file_path in catalog_files {
            let catalog = self.loader.load_and_parse_catalog_file(&file_path)?;
            for controller in catalog.controllers() {
                if controller.entity_name() == entry_name {
                    catalog_controller = Some(controller.clone());
                    catalog_file_path = file_path.to_string_lossy().to_string();
                    break;
                }
            }
            if catalog_controller.is_some() {
                break;
            }
        }

        let catalog_controller = catalog_controller.ok_or_else(|| {
            crate::error::Error::catalog_error(&format!(
                "Controller '{}' not found in catalog",
                entry_name
            ))
        })?;

        // Resolve parameters
        let mut parameters = std::collections::HashMap::new();
        if let Some(assignments) = &reference.parameter_assignments {
            for assignment in assignments.iter() {
                let resolved_name = assignment.parameter_ref.as_literal().ok_or_else(|| {
                    crate::error::Error::catalog_error(
                        "Cannot resolve parameterized parameter names",
                    )
                })?;
                let resolved_value = assignment.value.as_literal().ok_or_else(|| {
                    crate::error::Error::catalog_error(
                        "Cannot resolve parameterized parameter values",
                    )
                })?;
                parameters.insert(resolved_name.clone(), resolved_value.clone());
            }
        }

        // Convert catalog controller to scenario controller
        let resolved_controller = catalog_controller
            .clone()
            .into_scenario_entity(parameters.clone())?;

        // End resolution tracking
        self.resolver.end_resolution(&reference_key);

        Ok(ResolvedCatalog::with_parameters(
            resolved_controller,
            catalog_file_path,
            entry_name.clone(),
            parameters,
        ))
    }

    /// Resolve a pedestrian catalog reference to an actual pedestrian
    pub fn resolve_pedestrian_reference(
        &mut self,
        reference: &PedestrianCatalogReference,
        location: &PedestrianCatalogLocation,
    ) -> Result<ResolvedCatalog<crate::types::entities::pedestrian::Pedestrian>, crate::error::Error>
    {
        use crate::types::catalogs::entities::CatalogEntity;

        // Start resolution tracking
        let reference_key = format!(
            "pedestrian:{}:{}",
            reference
                .catalog_name
                .as_literal()
                .unwrap_or(&"unknown".to_string()),
            reference
                .entry_name
                .as_literal()
                .unwrap_or(&"unknown".to_string())
        );
        self.resolver.begin_resolution(&reference_key)?;

        // Load catalog files from the location and track file paths
        let catalog_files = self.loader.discover_catalog_files(&location.directory)?;
        let mut catalog_pedestrian = None;
        let mut catalog_file_path = String::new();

        // Find the specific pedestrian across all catalog files
        let entry_name = reference.entry_name.as_literal().ok_or_else(|| {
            crate::error::Error::catalog_error("Cannot resolve parameterized entry names yet")
        })?;

        for file_path in catalog_files {
            let catalog = self.loader.load_and_parse_catalog_file(&file_path)?;
            for pedestrian in catalog.pedestrians() {
                if pedestrian.entity_name() == entry_name {
                    catalog_pedestrian = Some(pedestrian.clone());
                    catalog_file_path = file_path.to_string_lossy().to_string();
                    break;
                }
            }
            if catalog_pedestrian.is_some() {
                break;
            }
        }

        let catalog_pedestrian = catalog_pedestrian.ok_or_else(|| {
            crate::error::Error::catalog_error(&format!(
                "Pedestrian '{}' not found in catalog",
                entry_name
            ))
        })?;

        // Resolve parameters
        let mut parameters = std::collections::HashMap::new();
        if let Some(assignments) = &reference.parameter_assignments {
            for assignment in assignments.iter() {
                let resolved_name = assignment.parameter_ref.as_literal().ok_or_else(|| {
                    crate::error::Error::catalog_error(
                        "Cannot resolve parameterized parameter names",
                    )
                })?;
                let resolved_value = assignment.value.as_literal().ok_or_else(|| {
                    crate::error::Error::catalog_error(
                        "Cannot resolve parameterized parameter values",
                    )
                })?;
                parameters.insert(resolved_name.clone(), resolved_value.clone());
            }
        }

        // Convert catalog pedestrian to scenario pedestrian
        let resolved_pedestrian = catalog_pedestrian
            .clone()
            .into_scenario_entity(parameters.clone())?;

        // End resolution tracking
        self.resolver.end_resolution(&reference_key);

        Ok(ResolvedCatalog::with_parameters(
            resolved_pedestrian,
            catalog_file_path,
            entry_name.clone(),
            parameters,
        ))
    }

    /// Discover and load all catalogs from catalog locations
    pub fn discover_and_load_catalogs(
        &mut self,
        locations: &CatalogLocations,
    ) -> Result<(), crate::error::Error> {
        // Discover vehicle catalogs
        if let Some(vehicle_location) = &locations.vehicle_catalog {
            if let Ok(files) = self
                .loader
                .discover_catalog_files(&vehicle_location.directory)
            {
                for file_path in files {
                    let _catalog = self.loader.load_and_parse_catalog_file(&file_path)?;
                    // Catalog loaded and validated successfully
                }
            }
        }

        // Discover controller catalogs
        if let Some(controller_location) = &locations.controller_catalog {
            if let Ok(files) = self
                .loader
                .discover_catalog_files(&controller_location.directory)
            {
                for file_path in files {
                    let _catalog = self.loader.load_and_parse_catalog_file(&file_path)?;
                    // Catalog loaded and validated successfully
                }
            }
        }

        // Discover pedestrian catalogs
        if let Some(pedestrian_location) = &locations.pedestrian_catalog {
            if let Ok(files) = self
                .loader
                .discover_catalog_files(&pedestrian_location.directory)
            {
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
    pub fn set_global_parameters(
        &mut self,
        parameters: std::collections::HashMap<String, String>,
    ) -> Result<(), crate::error::Error> {
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
        assert_eq!(
            manager
                .parameter_engine()
                .get_parameter("GlobalParam")
                .unwrap(),
            "GlobalValue"
        );
    }
}

/// Helper function to extract parameters from scenario ParameterDeclarations
pub fn extract_scenario_parameters(
    parameter_declarations: &Option<crate::types::basic::ParameterDeclarations>,
) -> std::collections::HashMap<String, String> {
    let mut parameters = std::collections::HashMap::new();

    if let Some(param_decls) = parameter_declarations {
        for param in &param_decls.parameter_declarations {
            parameters.insert(param.name.to_string(), param.value.to_string());
        }
    }

    parameters
}

/// Simple utility function to resolve catalog references using existing infrastructure
pub fn resolve_catalog_reference_simple(
    catalog_name: &crate::types::basic::OSString,
    entry_name: &crate::types::basic::OSString,
    catalog_locations: &crate::types::catalogs::locations::CatalogLocations,
    scenario_parameters: &std::collections::HashMap<String, String>,
    base_path: &std::path::Path,
) -> crate::error::Result<bool> {
    use crate::parser::xml::parse_catalog_from_file;

    // Step 1: Resolve parameter references in catalog name and entry name
    let resolved_catalog_name = catalog_name.resolve(scenario_parameters)?;
    let resolved_entry_name = entry_name.resolve(scenario_parameters)?;

    // Step 2: Determine catalog file path based on catalog type
    let (catalog_dir, filename) = match resolved_catalog_name.as_str() {
        "vehicle_catalog" => {
            if let Some(vehicle_catalog) = &catalog_locations.vehicle_catalog {
                (
                    vehicle_catalog
                        .directory
                        .path
                        .resolve(scenario_parameters)?,
                    "vehicle_catalog.xosc",
                )
            } else {
                return Err(crate::error::Error::catalog_error(
                    "Vehicle catalog location not specified",
                ));
            }
        }
        "pedestrian_catalog" => {
            if let Some(pedestrian_catalog) = &catalog_locations.pedestrian_catalog {
                (
                    pedestrian_catalog
                        .directory
                        .path
                        .resolve(scenario_parameters)?,
                    "pedestrian_catalog.xosc",
                )
            } else {
                return Err(crate::error::Error::catalog_error(
                    "Pedestrian catalog location not specified",
                ));
            }
        }
        "controller_catalog" => {
            if let Some(controller_catalog) = &catalog_locations.controller_catalog {
                (
                    controller_catalog
                        .directory
                        .path
                        .resolve(scenario_parameters)?,
                    "controller_catalog.xosc",
                )
            } else {
                return Err(crate::error::Error::catalog_error(
                    "Controller catalog location not specified",
                ));
            }
        }
        "misc_object_catalog" => {
            if let Some(misc_object_catalog) = &catalog_locations.misc_object_catalog {
                (
                    misc_object_catalog
                        .directory
                        .path
                        .resolve(scenario_parameters)?,
                    "misc_object_catalog.xosc",
                )
            } else {
                return Err(crate::error::Error::catalog_error(
                    "Misc object catalog location not specified",
                ));
            }
        }
        _ => {
            return Err(crate::error::Error::catalog_error(&format!(
                "Unknown catalog name: {}",
                resolved_catalog_name
            )));
        }
    };

    let catalog_file_path = base_path.join(catalog_dir).join(filename);

    // Step 3: Load catalog file using existing parser
    if !catalog_file_path.exists() {
        return Err(crate::error::Error::catalog_error(&format!(
            "Catalog file not found: {}",
            catalog_file_path.display()
        )));
    }

    let catalog_file = parse_catalog_from_file(&catalog_file_path)?;

    // Step 4: Find the requested entry in the catalog

    let catalog_content = &catalog_file.catalog;

    // Check vehicles
    if !catalog_content.vehicles.is_empty() {
        let found = catalog_content
            .vehicles
            .iter()
            .any(|v| v.name.to_string() == resolved_entry_name);
        if found {
            return Ok(true);
        }
    }

    // Check pedestrians
    if !catalog_content.pedestrians.is_empty() {
        let found = catalog_content
            .pedestrians
            .iter()
            .any(|p| p.name.to_string() == resolved_entry_name);
        if found {
            return Ok(true);
        }
    }

    // Check controllers
    if !catalog_content.controllers.is_empty() {
        let found = catalog_content
            .controllers
            .iter()
            .any(|c| c.name.to_string() == resolved_entry_name);
        if found {
            return Ok(true);
        }
    }

    // Check misc objects
    if !catalog_content.misc_objects.is_empty() {
        let found = catalog_content
            .misc_objects
            .iter()
            .any(|m| m.name.to_string() == resolved_entry_name);
        if found {
            return Ok(true);
        }
    }

    Ok(false)
}
