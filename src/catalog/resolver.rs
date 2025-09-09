//! Catalog reference resolution functionality
//!
//! This module handles:
//! - Resolving catalog references to actual entities
//! - Dependency tracking and circular reference detection
//! - Parameter substitution during resolution
//! - Integration of resolved content into scenarios

use crate::error::{Error, Result};
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

/// Catalog reference resolver
pub struct CatalogResolver {
    /// Track resolved references to detect circular dependencies
    resolution_stack: HashSet<String>,
}

impl CatalogResolver {
    /// Create a new catalog resolver
    pub fn new() -> Self {
        Self {
            resolution_stack: HashSet::new(),
        }
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