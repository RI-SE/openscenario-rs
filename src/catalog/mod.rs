//! Catalog system for OpenSCENARIO
//!
//! This module provides comprehensive catalog management for OpenSCENARIO scenarios:
//! - Loading and parsing catalog files from Directory paths
//! - Caching catalog data for performance optimization
//! - Resolving catalog references within scenarios
//! - Full integration of catalog content into scenario structures
//!
//! The catalog system enables modular scenario design by allowing reusable
//! components (vehicles, controllers, routes, etc.) to be defined in separate
//! files and referenced from main scenarios.

use crate::error::{Error, Result};
use crate::types::basic::Directory;
use std::collections::HashMap;
use std::path::Path;

pub mod loader;
pub mod resolver;

// Re-export key types for convenience
pub use loader::{CatalogLoader, CatalogCache};
pub use resolver::{CatalogResolver, ResolvedCatalog};

/// Trait for types that can be loaded from catalog directories
pub trait CatalogLocation {
    /// The type of catalog this location points to
    type CatalogType;
    
    /// Load the catalog from the directory path
    async fn load_catalog(&self) -> Result<Self::CatalogType>;
    
    /// Get the directory path for this catalog location
    fn directory(&self) -> &Directory;
}

/// Trait for catalog types that can resolve references
pub trait ResolvableCatalog {
    /// The type of entities this catalog contains
    type EntityType;
    
    /// Resolve a catalog reference to the actual entity
    fn resolve_reference(&self, reference_name: &str) -> Result<&Self::EntityType>;
    
    /// Get all available entity names in this catalog
    fn entity_names(&self) -> Vec<String>;
}

/// Trait for scenario types that can have their catalog references resolved
pub trait ScenarioResolver: Sized {
    /// Resolve all catalog references in this scenario
    async fn resolve_all_catalogs(self) -> Result<Self>;
}

/// Main catalog manager that coordinates loading, caching, and resolution
pub struct CatalogManager {
    cache: CatalogCache,
    loader: CatalogLoader,
}

impl CatalogManager {
    /// Create a new catalog manager
    pub fn new() -> Self {
        Self {
            cache: CatalogCache::new(),
            loader: CatalogLoader::new(),
        }
    }

    /// Load a catalog from a directory, using cache if available
    pub async fn load_catalog<T: CatalogLocation>(&mut self, location: &T) -> Result<T::CatalogType> {
        // Implementation will be added in loader module
        location.load_catalog().await
    }

    /// Clear the catalog cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> (usize, usize) {
        self.cache.stats()
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
        let manager = CatalogManager::new();
        let (cached_items, total_size) = manager.cache_stats();
        assert_eq!(cached_items, 0);
        assert_eq!(total_size, 0);
    }

    #[test]
    fn test_catalog_manager_default() {
        let manager = CatalogManager::default();
        let (cached_items, total_size) = manager.cache_stats();
        assert_eq!(cached_items, 0);
        assert_eq!(total_size, 0);
    }
}