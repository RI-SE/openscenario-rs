//! Catalog integration for builder system
//!
//! This module provides catalog reference support for the builder system,
//! enabling entities to be loaded from external catalog files.

use crate::builder::{BuilderError, BuilderResult};
use crate::catalog::CatalogManager;
use crate::types::{
    basic::{OSString, Directory},
    catalogs::{
        locations::{CatalogLocations, VehicleCatalogLocation, PedestrianCatalogLocation, ControllerCatalogLocation},
        references::{VehicleCatalogReference, PedestrianCatalogReference, ControllerCatalogReference},
    },
    entities::vehicle::Vehicle,
    controllers::Controller,
};
use std::collections::HashMap;

/// Builder for catalog locations
#[derive(Debug, Default)]
pub struct CatalogLocationsBuilder {
    vehicle_catalog: Option<VehicleCatalogLocation>,
    pedestrian_catalog: Option<PedestrianCatalogLocation>,
    controller_catalog: Option<ControllerCatalogLocation>,
}

impl CatalogLocationsBuilder {
    /// Create a new catalog locations builder
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set vehicle catalog location
    pub fn with_vehicle_catalog(mut self, directory_path: &str) -> Self {
        self.vehicle_catalog = Some(VehicleCatalogLocation {
            directory: Directory {
                path: OSString::literal(directory_path.to_string()),
            },
        });
        self
    }
    
    /// Set pedestrian catalog location
    pub fn with_pedestrian_catalog(mut self, directory_path: &str) -> Self {
        self.pedestrian_catalog = Some(PedestrianCatalogLocation {
            directory: Directory {
                path: OSString::literal(directory_path.to_string()),
            },
        });
        self
    }
    
    /// Set controller catalog location
    pub fn with_controller_catalog(mut self, directory_path: &str) -> Self {
        self.controller_catalog = Some(ControllerCatalogLocation {
            directory: Directory {
                path: OSString::literal(directory_path.to_string()),
            },
        });
        self
    }
    
    /// Build the catalog locations
    pub fn build(self) -> CatalogLocations {
        CatalogLocations {
            vehicle_catalog: self.vehicle_catalog,
            pedestrian_catalog: self.pedestrian_catalog,
            controller_catalog: self.controller_catalog,
            misc_object_catalog: None,
            environment_catalog: None,
            maneuver_catalog: None,
            trajectory_catalog: None,
            route_catalog: None,
        }
    }
}

/// Builder for vehicle catalog references
#[derive(Debug, Default)]
pub struct VehicleCatalogReferenceBuilder {
    catalog_name: Option<String>,
    entry_name: Option<String>,
    parameters: HashMap<String, String>,
}

impl VehicleCatalogReferenceBuilder {
    /// Create a new vehicle catalog reference builder
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set the catalog name
    pub fn from_catalog(mut self, catalog_name: &str) -> Self {
        self.catalog_name = Some(catalog_name.to_string());
        self
    }
    
    /// Set the entry name
    pub fn entry(mut self, entry_name: &str) -> Self {
        self.entry_name = Some(entry_name.to_string());
        self
    }
    
    /// Add a parameter assignment
    pub fn with_parameter(mut self, name: &str, value: &str) -> Self {
        self.parameters.insert(name.to_string(), value.to_string());
        self
    }
    
    /// Build the catalog reference
    pub fn build(self) -> BuilderResult<VehicleCatalogReference> {
        let catalog_name = self.catalog_name
            .ok_or_else(|| BuilderError::missing_field("catalog_name", ".from_catalog()"))?;
        let entry_name = self.entry_name
            .ok_or_else(|| BuilderError::missing_field("entry_name", ".entry()"))?;
        
        let parameter_assignments = if self.parameters.is_empty() {
            None
        } else {
            Some(self.parameters.into_iter().map(|(name, value)| {
                crate::types::catalogs::references::ParameterAssignment {
                    parameter_ref: OSString::literal(name),
                    value: OSString::literal(value),
                }
            }).collect())
        };
        
        let mut reference = VehicleCatalogReference::new(catalog_name, entry_name);
        reference.parameter_assignments = parameter_assignments;
        Ok(reference)
    }
}

/// Builder for pedestrian catalog references
#[derive(Debug, Default)]
pub struct PedestrianCatalogReferenceBuilder {
    catalog_name: Option<String>,
    entry_name: Option<String>,
    parameters: HashMap<String, String>,
}

impl PedestrianCatalogReferenceBuilder {
    /// Create a new pedestrian catalog reference builder
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set the catalog name
    pub fn from_catalog(mut self, catalog_name: &str) -> Self {
        self.catalog_name = Some(catalog_name.to_string());
        self
    }
    
    /// Set the entry name
    pub fn entry(mut self, entry_name: &str) -> Self {
        self.entry_name = Some(entry_name.to_string());
        self
    }
    
    /// Add a parameter assignment
    pub fn with_parameter(mut self, name: &str, value: &str) -> Self {
        self.parameters.insert(name.to_string(), value.to_string());
        self
    }
    
    /// Build the catalog reference
    pub fn build(self) -> BuilderResult<PedestrianCatalogReference> {
        let catalog_name = self.catalog_name
            .ok_or_else(|| BuilderError::missing_field("catalog_name", ".from_catalog()"))?;
        let entry_name = self.entry_name
            .ok_or_else(|| BuilderError::missing_field("entry_name", ".entry()"))?;
        
        let parameter_assignments = if self.parameters.is_empty() {
            None
        } else {
            Some(self.parameters.into_iter().map(|(name, value)| {
                crate::types::catalogs::references::ParameterAssignment {
                    parameter_ref: OSString::literal(name),
                    value: OSString::literal(value),
                }
            }).collect())
        };
        
        let mut reference = PedestrianCatalogReference::new(catalog_name, entry_name);
        reference.parameter_assignments = parameter_assignments;
        Ok(reference)
    }
}

/// Catalog-aware entity builder that can resolve catalog references
pub struct CatalogEntityBuilder {
    catalog_manager: CatalogManager,
    catalog_locations: Option<CatalogLocations>,
}

impl CatalogEntityBuilder {
    /// Create a new catalog entity builder
    pub fn new() -> Self {
        Self {
            catalog_manager: CatalogManager::new(),
            catalog_locations: None,
        }
    }
    
    /// Create with a base path for relative catalog resolution
    pub fn with_base_path<P: AsRef<std::path::Path>>(base_path: P) -> Self {
        Self {
            catalog_manager: CatalogManager::with_base_path(base_path),
            catalog_locations: None,
        }
    }
    
    /// Set catalog locations
    pub fn with_catalog_locations(mut self, locations: CatalogLocations) -> Self {
        self.catalog_locations = Some(locations);
        self
    }
    
    /// Resolve a vehicle catalog reference to an actual vehicle
    pub fn resolve_vehicle_reference(
        &mut self,
        reference: &VehicleCatalogReference,
    ) -> BuilderResult<Vehicle> {
        let locations = self.catalog_locations.as_ref()
            .ok_or_else(|| BuilderError::validation_error("Catalog locations not set"))?;
        
        let vehicle_location = locations.vehicle_catalog.as_ref()
            .ok_or_else(|| BuilderError::validation_error("Vehicle catalog location not set"))?;
        
        let resolved = self.catalog_manager
            .resolve_vehicle_reference(reference, vehicle_location)
            .map_err(|e| BuilderError::OpenScenarioError(e))?;
        
        Ok(resolved.entity)
    }
    
    /// Resolve a pedestrian catalog reference to an actual pedestrian
    pub fn resolve_pedestrian_reference(
        &mut self,
        reference: &PedestrianCatalogReference,
    ) -> BuilderResult<crate::types::entities::pedestrian::Pedestrian> {
        let locations = self.catalog_locations.as_ref()
            .ok_or_else(|| BuilderError::validation_error("Catalog locations not set"))?;
        
        let pedestrian_location = locations.pedestrian_catalog.as_ref()
            .ok_or_else(|| BuilderError::validation_error("Pedestrian catalog location not set"))?;
        
        let resolved = self.catalog_manager
            .resolve_pedestrian_reference(reference, pedestrian_location)
            .map_err(|e| BuilderError::OpenScenarioError(e))?;
        
        Ok(resolved.entity)
    }
    
    /// Resolve a controller catalog reference to an actual controller
    pub fn resolve_controller_reference(
        &mut self,
        reference: &ControllerCatalogReference,
    ) -> BuilderResult<Controller> {
        let locations = self.catalog_locations.as_ref()
            .ok_or_else(|| BuilderError::validation_error("Catalog locations not set"))?;
        
        let controller_location = locations.controller_catalog.as_ref()
            .ok_or_else(|| BuilderError::validation_error("Controller catalog location not set"))?;
        
        let resolved = self.catalog_manager
            .resolve_controller_reference(reference, controller_location)
            .map_err(|e| BuilderError::OpenScenarioError(e))?;
        
        Ok(resolved.entity)
    }
    
    /// Validate that all catalog locations are accessible
    pub fn validate_catalog_locations(&mut self) -> BuilderResult<()> {
        if let Some(locations) = &self.catalog_locations {
            self.catalog_manager
                .discover_and_load_catalogs(locations)
                .map_err(|e| BuilderError::OpenScenarioError(e))?;
        }
        Ok(())
    }
}

impl Default for CatalogEntityBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalog_locations_builder() {
        let locations = CatalogLocationsBuilder::new()
            .with_vehicle_catalog("./catalogs/vehicles")
            .with_pedestrian_catalog("./catalogs/pedestrians")
            .build();
        
        assert!(locations.vehicle_catalog.is_some());
        assert!(locations.pedestrian_catalog.is_some());
        assert!(locations.controller_catalog.is_none());
    }

    #[test]
    fn test_vehicle_catalog_reference_builder() {
        let reference = VehicleCatalogReferenceBuilder::new()
            .from_catalog("vehicle_catalog")
            .entry("sedan")
            .with_parameter("color", "red")
            .build()
            .unwrap();
        
        assert_eq!(reference.catalog_name.to_string(), "vehicle_catalog");
        assert_eq!(reference.entry_name.to_string(), "sedan");
        assert!(reference.parameter_assignments.is_some());
    }

    #[test]
    fn test_catalog_entity_builder_creation() {
        let _builder = CatalogEntityBuilder::new();
        let _builder_with_path = CatalogEntityBuilder::with_base_path("/tmp");
    }
}