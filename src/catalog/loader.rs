//! Catalog loading and caching functionality
//!
//! This module handles:
//! - Loading catalog files from directory paths
//! - Parsing XML catalog content
//! - Caching loaded catalogs for performance
//! - File system operations for catalog discovery

use crate::error::{Error, Result};
use crate::types::basic::Directory;
use crate::types::catalogs::files::CatalogFile;
use crate::types::catalogs::entities::{CatalogVehicle, CatalogController, CatalogPedestrian};
use crate::parser::xml::{parse_catalog_from_file, parse_catalog_from_str};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use std::sync::{Arc, RwLock};

/// Catalog file loader that handles file system operations
pub struct CatalogLoader {
    /// Base path for resolving relative catalog paths
    base_path: Option<PathBuf>,
}

impl CatalogLoader {
    /// Create a new catalog loader
    pub fn new() -> Self {
        Self { base_path: None }
    }

    /// Create a catalog loader with a specific base path
    pub fn with_base_path<P: AsRef<Path>>(base_path: P) -> Self {
        Self {
            base_path: Some(base_path.as_ref().to_path_buf()),
        }
    }

    /// Set the base path for resolving relative paths
    pub fn set_base_path<P: AsRef<Path>>(&mut self, base_path: P) {
        self.base_path = Some(base_path.as_ref().to_path_buf());
    }

    /// Discover all .xosc files in a directory
    pub fn discover_catalog_files(&self, directory: &Directory) -> Result<Vec<PathBuf>> {
        // For now, assume we have a literal path. Parameter resolution will be handled later.
        let path_str = directory.path.as_literal()
            .ok_or_else(|| Error::catalog_error("Cannot discover files with parameterized paths"))?;

        let dir_path = self.resolve_path(path_str)?;
        
        if !dir_path.exists() {
            return Err(Error::catalog_error(&format!("Directory does not exist: {}", dir_path.display())));
        }

        if !dir_path.is_dir() {
            return Err(Error::catalog_error(&format!("Path is not a directory: {}", dir_path.display())));
        }

        let mut catalog_files = Vec::new();
        
        for entry in fs::read_dir(&dir_path)
            .map_err(|e| Error::catalog_error(&format!("Failed to read directory {}: {}", dir_path.display(), e)))?
        {
            let entry = entry
                .map_err(|e| Error::catalog_error(&format!("Failed to read directory entry: {}", e)))?;
            
            let path = entry.path();
            
            // Check if it's a .xosc file
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "xosc" {
                        catalog_files.push(path);
                    }
                }
            }
        }

        catalog_files.sort();
        Ok(catalog_files)
    }

    /// Load and parse a catalog file
    pub fn load_catalog_file<P: AsRef<Path>>(&self, file_path: P) -> Result<String> {
        let path = file_path.as_ref();
        
        if !path.exists() {
            return Err(Error::catalog_error(&format!("Catalog file does not exist: {}", path.display())));
        }

        fs::read_to_string(path)
            .map_err(|e| Error::catalog_error(&format!("Failed to read catalog file {}: {}", path.display(), e)))
    }

    /// Load and parse a catalog file into a CatalogFile structure
    pub fn load_and_parse_catalog_file<P: AsRef<Path>>(&self, file_path: P) -> Result<CatalogFile> {
        let path = file_path.as_ref();
        
        if !path.exists() {
            return Err(Error::catalog_error(&format!("Catalog file does not exist: {}", path.display())));
        }

        parse_catalog_from_file(path)
            .map_err(|e| e.with_context(&format!("Failed to parse catalog file: {}", path.display())))
    }

    /// Load and parse a catalog from XML string
    pub fn parse_catalog_from_string(&self, xml: &str) -> Result<CatalogFile> {
        parse_catalog_from_str(xml)
            .map_err(|e| e.with_context("Failed to parse catalog from string"))
    }

    /// Load all vehicle catalogs from a directory
    pub fn load_vehicle_catalogs(&self, directory: &Directory) -> Result<Vec<CatalogVehicle>> {
        let catalog_files = self.discover_catalog_files(directory)?;
        let mut vehicles = Vec::new();

        for file_path in catalog_files {
            let catalog = self.load_and_parse_catalog_file(&file_path)?;
            vehicles.extend(catalog.vehicles().iter().cloned());
        }

        Ok(vehicles)
    }

    /// Load all controller catalogs from a directory
    pub fn load_controller_catalogs(&self, directory: &Directory) -> Result<Vec<CatalogController>> {
        let catalog_files = self.discover_catalog_files(directory)?;
        let mut controllers = Vec::new();

        for file_path in catalog_files {
            let catalog = self.load_and_parse_catalog_file(&file_path)?;
            controllers.extend(catalog.controllers().iter().cloned());
        }

        Ok(controllers)
    }

    /// Load all pedestrian catalogs from a directory
    pub fn load_pedestrian_catalogs(&self, directory: &Directory) -> Result<Vec<CatalogPedestrian>> {
        let catalog_files = self.discover_catalog_files(directory)?;
        let mut pedestrians = Vec::new();

        for file_path in catalog_files {
            let catalog = self.load_and_parse_catalog_file(&file_path)?;
            pedestrians.extend(catalog.pedestrians().iter().cloned());
        }

        Ok(pedestrians)
    }

    /// Find a specific entity in a catalog file
    pub fn find_entity_in_catalog<P: AsRef<Path>>(&self, file_path: P, entity_name: &str) -> Result<Option<String>> {
        let catalog = self.load_and_parse_catalog_file(file_path)?;
        
        // Check if any entity with the given name exists
        let entity_names = catalog.catalog.entity_names();
        if entity_names.contains(&entity_name.to_string()) {
            Ok(Some(entity_name.to_string()))
        } else {
            Ok(None)
        }
    }

    /// Resolve a path relative to the base path if needed
    fn resolve_path(&self, path: &str) -> Result<PathBuf> {
        let path = Path::new(path);
        
        if path.is_absolute() {
            Ok(path.to_path_buf())
        } else if let Some(base) = &self.base_path {
            Ok(base.join(path))
        } else {
            // Use current directory as base
            Ok(std::env::current_dir()
                .map_err(|e| Error::catalog_error(&format!("Failed to get current directory: {}", e)))?
                .join(path))
        }
    }
}

impl Default for CatalogLoader {
    fn default() -> Self {
        Self::new()
    }
}

/// Cache for loaded catalogs to improve performance
pub struct CatalogCache {
    /// Cached raw catalog XML data, keyed by resolved file path
    xml_cache: Arc<RwLock<HashMap<PathBuf, String>>>,
    /// Cached parsed catalog objects, keyed by resolved file path
    parsed_cache: Arc<RwLock<HashMap<PathBuf, CatalogFile>>>,
    /// Maximum cache size (number of entries per cache)
    max_size: usize,
}

impl CatalogCache {
    /// Create a new catalog cache
    pub fn new() -> Self {
        Self::with_capacity(100)
    }

    /// Create a catalog cache with specific capacity
    pub fn with_capacity(max_size: usize) -> Self {
        Self {
            xml_cache: Arc::new(RwLock::new(HashMap::new())),
            parsed_cache: Arc::new(RwLock::new(HashMap::new())),
            max_size,
        }
    }

    /// Get a cached XML catalog by path
    pub fn get_xml(&self, path: &Path) -> Option<String> {
        let cache = self.xml_cache.read().unwrap();
        cache.get(path).cloned()
    }

    /// Get a cached parsed catalog by path
    pub fn get_parsed(&self, path: &Path) -> Option<CatalogFile> {
        let cache = self.parsed_cache.read().unwrap();
        cache.get(path).cloned()
    }

    /// Insert an XML catalog into the cache
    pub fn insert_xml<P: AsRef<Path>>(&self, path: P, content: String) {
        let path = path.as_ref().to_path_buf();
        let mut cache = self.xml_cache.write().unwrap();
        
        // Simple eviction: if we're at capacity, remove the first entry
        if cache.len() >= self.max_size {
            if let Some(first_key) = cache.keys().next().cloned() {
                cache.remove(&first_key);
            }
        }
        
        cache.insert(path, content);
    }

    /// Insert a parsed catalog into the cache
    pub fn insert_parsed<P: AsRef<Path>>(&self, path: P, catalog: CatalogFile) {
        let path = path.as_ref().to_path_buf();
        let mut cache = self.parsed_cache.write().unwrap();
        
        // Simple eviction: if we're at capacity, remove the first entry
        if cache.len() >= self.max_size {
            if let Some(first_key) = cache.keys().next().cloned() {
                cache.remove(&first_key);
            }
        }
        
        cache.insert(path, catalog);
    }

    /// Insert a catalog into the cache (legacy method for backward compatibility)
    pub fn insert<P: AsRef<Path>>(&self, path: P, content: String) {
        self.insert_xml(path, content);
    }

    /// Get a cached catalog by path (legacy method for backward compatibility)
    pub fn get(&self, path: &Path) -> Option<String> {
        self.get_xml(path)
    }

    /// Clear both caches
    pub fn clear(&self) {
        let mut xml_cache = self.xml_cache.write().unwrap();
        let mut parsed_cache = self.parsed_cache.write().unwrap();
        xml_cache.clear();
        parsed_cache.clear();
    }

    /// Get cache statistics (number of entries, total content size)
    pub fn stats(&self) -> (usize, usize) {
        let xml_cache = self.xml_cache.read().unwrap();
        let parsed_cache = self.parsed_cache.read().unwrap();
        let xml_count = xml_cache.len();
        let parsed_count = parsed_cache.len();
        let total_size = xml_cache.values().map(|s| s.len()).sum();
        (xml_count + parsed_count, total_size)
    }
}

impl Default for CatalogCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_catalog_loader_creation() {
        let loader = CatalogLoader::new();
        assert!(loader.base_path.is_none());

        let loader_with_path = CatalogLoader::with_base_path("/tmp");
        assert_eq!(loader_with_path.base_path.as_ref().unwrap(), Path::new("/tmp"));
    }

    #[test]
    fn test_catalog_cache() {
        let cache = CatalogCache::new();
        let (count, size) = cache.stats();
        assert_eq!(count, 0);
        assert_eq!(size, 0);

        // Insert and retrieve
        let path = Path::new("/test/path.xosc");
        let content = "test content".to_string();
        cache.insert(&path, content.clone());
        
        let retrieved = cache.get(&path).unwrap();
        assert_eq!(retrieved, content);
        
        let (count, size) = cache.stats();
        assert_eq!(count, 1);
        assert_eq!(size, content.len());
    }

    #[test]
    fn test_discover_catalog_files() -> Result<()> {
        // Create a temporary directory with some test files
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        // Create test files
        fs::write(dir_path.join("catalog1.xosc"), "catalog1 content")?;
        fs::write(dir_path.join("catalog2.xosc"), "catalog2 content")?;
        fs::write(dir_path.join("not_catalog.txt"), "not a catalog")?;

        let directory = Directory::new(dir_path.to_string_lossy().to_string());
        let loader = CatalogLoader::new();

        let files = loader.discover_catalog_files(&directory)?;
        assert_eq!(files.len(), 2);

        // Files should be sorted
        assert!(files[0].file_name().unwrap().to_string_lossy().starts_with("catalog"));
        assert!(files[1].file_name().unwrap().to_string_lossy().starts_with("catalog"));

        Ok(())
    }

    #[test]
    fn test_load_catalog_file() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test_catalog.xosc");
        let content = "<?xml version=\"1.0\"?>\n<OpenSCENARIO>test</OpenSCENARIO>";
        
        fs::write(&file_path, &content).unwrap();

        let loader = CatalogLoader::new();
        let loaded_content = loader.load_catalog_file(&file_path)?;
        assert_eq!(loaded_content, content);

        Ok(())
    }

    #[test]
    fn test_catalog_cache_parsed() {
        let cache = CatalogCache::new();
        let (count, _) = cache.stats();
        assert_eq!(count, 0);

        // Insert and retrieve parsed catalog
        let path = Path::new("/test/catalog.xosc");
        let catalog = CatalogFile::default();
        cache.insert_parsed(&path, catalog.clone());
        
        let retrieved = cache.get_parsed(&path).unwrap();
        assert_eq!(retrieved.catalog_name(), catalog.catalog_name());
        
        let (count, _) = cache.stats();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_parse_catalog_from_string() {
        let loader = CatalogLoader::new();
        
        // Test with minimal valid catalog XML
        let catalog_xml = r#"<?xml version="1.0"?>
        <OpenSCENARIO>
            <FileHeader author="Test" date="2024-01-01T00:00:00" description="Test" revMajor="1" revMinor="3"/>
            <Catalog name="TestCatalog">
            </Catalog>
        </OpenSCENARIO>"#;
        
        let catalog = loader.parse_catalog_from_string(catalog_xml).unwrap();
        assert_eq!(catalog.catalog_name().as_literal().unwrap(), "TestCatalog");
        assert_eq!(catalog.file_header.author.as_literal().unwrap(), "Test");
        assert_eq!(catalog.catalog.entity_count(), 0);
    }
}