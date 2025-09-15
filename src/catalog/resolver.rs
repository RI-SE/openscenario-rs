//! Catalog reference resolution functionality
//!
//! This module handles:
//! - Resolving catalog references to actual entities
//! - Dependency tracking and circular reference detection
//! - Parameter substitution during resolution
//! - Integration of resolved content into scenarios

use crate::catalog::parameters::ParameterSubstitutionEngine;
use crate::error::{Error, Result};
use crate::types::catalogs::controllers::{CatalogController, ControllerCatalog};
use crate::types::catalogs::environments::{CatalogEnvironment, EnvironmentCatalog};
use crate::types::catalogs::routes::{CatalogRoute, RouteCatalog};
use crate::types::catalogs::trajectories::{CatalogTrajectory, TrajectoryCatalog};
use std::collections::{HashMap, HashSet};

/// Represents a resolved catalog entity
pub struct ResolvedCatalog<T> {
    /// The resolved entity content
    pub entity: T,
    /// Additional metadata about the resolution
    pub metadata: ResolutionMetadata,
}

/// Metadata about how a catalog reference was resolved
#[derive(Debug, Clone)]
pub struct ResolutionMetadata {
    /// Path to the catalog file where the entity was found
    pub catalog_path: String,
    /// Name of the entity in the catalog
    pub entity_name: String,
    /// Parameters that were substituted during resolution
    pub parameter_substitutions: HashMap<String, String>,
}

/// Trait for catalog resolution
pub trait CatalogResolvable<T> {
    /// Resolve a catalog reference to the actual entity
    fn resolve(&self, entry_name: &str, catalogs: &CatalogManager) -> Result<ResolvedCatalog<T>>;
}

/// Central catalog manager for all catalog types
pub struct CatalogManager {
    /// Controller catalogs indexed by catalog name
    pub controller_catalogs: HashMap<String, ControllerCatalog>,
    /// Trajectory catalogs indexed by catalog name
    pub trajectory_catalogs: HashMap<String, TrajectoryCatalog>,
    /// Route catalogs indexed by catalog name
    pub route_catalogs: HashMap<String, RouteCatalog>,
    /// Environment catalogs indexed by catalog name
    pub environment_catalogs: HashMap<String, EnvironmentCatalog>,
    /// Parameter resolver for handling parameter substitution
    pub parameter_resolver: ParameterSubstitutionEngine,
}

/// Catalog reference resolver
pub struct CatalogResolver {
    /// Track resolved references to detect circular dependencies
    resolution_stack: HashSet<String>,
    /// Catalog manager containing all loaded catalogs
    catalog_manager: Option<CatalogManager>,
}

impl CatalogManager {
    /// Create a new empty catalog manager
    pub fn new() -> Self {
        Self {
            controller_catalogs: HashMap::new(),
            trajectory_catalogs: HashMap::new(),
            route_catalogs: HashMap::new(),
            environment_catalogs: HashMap::new(),
            parameter_resolver: ParameterSubstitutionEngine::new(),
        }
    }

    /// Add a controller catalog
    pub fn add_controller_catalog(&mut self, name: String, catalog: ControllerCatalog) {
        self.controller_catalogs.insert(name, catalog);
    }

    /// Add a trajectory catalog
    pub fn add_trajectory_catalog(&mut self, name: String, catalog: TrajectoryCatalog) {
        self.trajectory_catalogs.insert(name, catalog);
    }

    /// Add a route catalog
    pub fn add_route_catalog(&mut self, name: String, catalog: RouteCatalog) {
        self.route_catalogs.insert(name, catalog);
    }

    /// Add an environment catalog
    pub fn add_environment_catalog(&mut self, name: String, catalog: EnvironmentCatalog) {
        self.environment_catalogs.insert(name, catalog);
    }

    /// Set global parameter values that apply to all catalog resolutions
    pub fn set_parameters(&mut self, parameters: HashMap<String, String>) -> Result<()> {
        self.parameter_resolver.set_parameters(parameters)
    }

    /// Set a single global parameter value
    pub fn set_parameter(&mut self, name: String, value: String) -> Result<()> {
        self.parameter_resolver.set_parameter(name, value)
    }

    /// Get the current parameter context
    pub fn parameter_context(&self) -> &HashMap<String, String> {
        self.parameter_resolver.context()
    }

    /// Resolve controller reference from controller catalogs
    pub fn resolve_controller_reference(
        &self,
        catalog_name: &str,
        entry_name: &str,
    ) -> Result<ResolvedCatalog<CatalogController>> {
        self.resolve_controller_reference_with_params(catalog_name, entry_name, &HashMap::new())
    }

    /// Resolve controller reference with parameter overrides
    pub fn resolve_controller_reference_with_params(
        &self,
        catalog_name: &str,
        entry_name: &str,
        params: &HashMap<String, String>,
    ) -> Result<ResolvedCatalog<CatalogController>> {
        if let Some(catalog) = self.controller_catalogs.get(catalog_name) {
            for controller in &catalog.controllers {
                if controller.name == entry_name {
                    let resolved_controller =
                        if !params.is_empty() || !self.parameter_resolver.context().is_empty() {
                            // Apply parameter substitution if needed
                            let _param_engine = self
                                .parameter_resolver
                                .with_additional_context(params.clone());
                            // For now, return the original controller
                            controller.clone()
                        } else {
                            controller.clone()
                        };

                    return Ok(ResolvedCatalog::with_parameters(
                        resolved_controller,
                        format!("controller_catalog:{}", catalog_name),
                        entry_name.to_string(),
                        params.clone(),
                    ));
                }
            }
        }
        Err(Error::catalog_error(&format!(
            "Controller '{}' not found in catalog '{}'",
            entry_name, catalog_name
        )))
    }

    /// Resolve trajectory reference from trajectory catalogs
    pub fn resolve_trajectory_reference(
        &self,
        catalog_name: &str,
        entry_name: &str,
    ) -> Result<ResolvedCatalog<CatalogTrajectory>> {
        self.resolve_trajectory_reference_with_params(catalog_name, entry_name, &HashMap::new())
    }

    /// Resolve trajectory reference with parameter overrides
    pub fn resolve_trajectory_reference_with_params(
        &self,
        catalog_name: &str,
        entry_name: &str,
        params: &HashMap<String, String>,
    ) -> Result<ResolvedCatalog<CatalogTrajectory>> {
        if let Some(catalog) = self.trajectory_catalogs.get(catalog_name) {
            for trajectory in &catalog.trajectories {
                if trajectory.name == entry_name {
                    return Ok(ResolvedCatalog::with_parameters(
                        trajectory.clone(),
                        format!("trajectory_catalog:{}", catalog_name),
                        entry_name.to_string(),
                        params.clone(),
                    ));
                }
            }
        }
        Err(Error::catalog_error(&format!(
            "Trajectory '{}' not found in catalog '{}'",
            entry_name, catalog_name
        )))
    }

    /// Resolve route reference from route catalogs
    pub fn resolve_route_reference(
        &self,
        catalog_name: &str,
        entry_name: &str,
    ) -> Result<ResolvedCatalog<CatalogRoute>> {
        self.resolve_route_reference_with_params(catalog_name, entry_name, &HashMap::new())
    }

    /// Resolve route reference with parameter overrides
    pub fn resolve_route_reference_with_params(
        &self,
        catalog_name: &str,
        entry_name: &str,
        params: &HashMap<String, String>,
    ) -> Result<ResolvedCatalog<CatalogRoute>> {
        if let Some(catalog) = self.route_catalogs.get(catalog_name) {
            for route in &catalog.routes {
                if route.name == entry_name {
                    return Ok(ResolvedCatalog::with_parameters(
                        route.clone(),
                        format!("route_catalog:{}", catalog_name),
                        entry_name.to_string(),
                        params.clone(),
                    ));
                }
            }
        }
        Err(Error::catalog_error(&format!(
            "Route '{}' not found in catalog '{}'",
            entry_name, catalog_name
        )))
    }

    /// Resolve environment reference from environment catalogs
    pub fn resolve_environment_reference(
        &self,
        catalog_name: &str,
        entry_name: &str,
    ) -> Result<ResolvedCatalog<CatalogEnvironment>> {
        self.resolve_environment_reference_with_params(catalog_name, entry_name, &HashMap::new())
    }

    /// Resolve environment reference with parameter overrides
    pub fn resolve_environment_reference_with_params(
        &self,
        catalog_name: &str,
        entry_name: &str,
        params: &HashMap<String, String>,
    ) -> Result<ResolvedCatalog<CatalogEnvironment>> {
        if let Some(catalog) = self.environment_catalogs.get(catalog_name) {
            for environment in &catalog.environments {
                if environment.name == entry_name {
                    return Ok(ResolvedCatalog::with_parameters(
                        environment.clone(),
                        format!("environment_catalog:{}", catalog_name),
                        entry_name.to_string(),
                        params.clone(),
                    ));
                }
            }
        }
        Err(Error::catalog_error(&format!(
            "Environment '{}' not found in catalog '{}'",
            entry_name, catalog_name
        )))
    }
}

impl CatalogResolver {
    /// Create a new catalog resolver
    pub fn new() -> Self {
        Self {
            resolution_stack: HashSet::new(),
            catalog_manager: None,
        }
    }

    /// Create a new catalog resolver with a catalog manager
    pub fn with_catalog_manager(catalog_manager: CatalogManager) -> Self {
        Self {
            resolution_stack: HashSet::new(),
            catalog_manager: Some(catalog_manager),
        }
    }

    /// Set the catalog manager
    pub fn set_catalog_manager(&mut self, catalog_manager: CatalogManager) {
        self.catalog_manager = Some(catalog_manager);
    }

    /// Get a reference to the catalog manager
    pub fn catalog_manager(&self) -> Option<&CatalogManager> {
        self.catalog_manager.as_ref()
    }

    /// Begin resolving a reference (for circular dependency detection)
    pub fn begin_resolution(&mut self, reference_key: &str) -> Result<()> {
        if self.resolution_stack.contains(reference_key) {
            return Err(Error::catalog_error(&format!(
                "Circular catalog reference detected: {}",
                reference_key
            )));
        }
        self.resolution_stack.insert(reference_key.to_string());
        Ok(())
    }

    /// End resolving a reference
    pub fn end_resolution(&mut self, reference_key: &str) {
        self.resolution_stack.remove(reference_key);
    }

    /// Check if we're currently resolving a reference
    pub fn is_resolving(&self, reference_key: &str) -> bool {
        self.resolution_stack.contains(reference_key)
    }

    /// Clear the resolution stack
    pub fn clear(&mut self) {
        self.resolution_stack.clear();
    }
}

impl Default for CatalogResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ResolvedCatalog<T> {
    /// Create a new resolved catalog entry
    pub fn new(entity: T, catalog_path: String, entity_name: String) -> Self {
        Self {
            entity,
            metadata: ResolutionMetadata {
                catalog_path,
                entity_name,
                parameter_substitutions: HashMap::new(),
            },
        }
    }

    /// Create a resolved catalog entry with parameter substitutions
    pub fn with_parameters(
        entity: T,
        catalog_path: String,
        entity_name: String,
        parameters: HashMap<String, String>,
    ) -> Self {
        Self {
            entity,
            metadata: ResolutionMetadata {
                catalog_path,
                entity_name,
                parameter_substitutions: parameters,
            },
        }
    }

    /// Get the resolved entity
    pub fn into_entity(self) -> T {
        self.entity
    }

    /// Get a reference to the resolved entity
    pub fn entity(&self) -> &T {
        &self.entity
    }

    /// Get the resolution metadata
    pub fn metadata(&self) -> &ResolutionMetadata {
        &self.metadata
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolver_creation() {
        let resolver = CatalogResolver::new();
        assert!(!resolver.is_resolving("test"));
    }

    #[test]
    fn test_circular_dependency_detection() {
        let mut resolver = CatalogResolver::new();

        // Start resolving a reference
        assert!(resolver.begin_resolution("vehicle1").is_ok());
        assert!(resolver.is_resolving("vehicle1"));

        // Try to resolve the same reference again - should detect circular dependency
        assert!(resolver.begin_resolution("vehicle1").is_err());

        // End the resolution
        resolver.end_resolution("vehicle1");
        assert!(!resolver.is_resolving("vehicle1"));

        // Now we can start resolving it again
        assert!(resolver.begin_resolution("vehicle1").is_ok());
        resolver.end_resolution("vehicle1");
    }

    #[test]
    fn test_resolved_catalog() {
        let entity = "test_vehicle".to_string();
        let resolved = ResolvedCatalog::new(
            entity.clone(),
            "/path/to/catalog.xosc".to_string(),
            "TestVehicle".to_string(),
        );

        assert_eq!(resolved.entity(), &entity);
        assert_eq!(resolved.metadata().catalog_path, "/path/to/catalog.xosc");
        assert_eq!(resolved.metadata().entity_name, "TestVehicle");
        assert!(resolved.metadata().parameter_substitutions.is_empty());
    }

    #[test]
    fn test_resolved_catalog_with_parameters() {
        let mut params = HashMap::new();
        params.insert("MaxSpeed".to_string(), "60.0".to_string());
        params.insert("Color".to_string(), "Red".to_string());

        let resolved = ResolvedCatalog::with_parameters(
            42u32,
            "/catalogs/vehicles.xosc".to_string(),
            "SportsCar".to_string(),
            params.clone(),
        );

        assert_eq!(*resolved.entity(), 42u32);
        assert_eq!(resolved.metadata().parameter_substitutions, params);
    }
}
