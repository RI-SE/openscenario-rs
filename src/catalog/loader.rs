//! Catalog loading and caching functionality
//!
//! This module handles:
//! - Loading catalog files from directory paths
//! - Parsing XML catalog content
//! - Caching loaded catalogs for performance
//! - File system operations for catalog discovery

use crate::error::{Error, Result};
use crate::parser::xml::{parse_catalog_from_file, parse_catalog_from_str};
use crate::types::basic::Directory;
use crate::types::catalogs::entities::{CatalogController, CatalogPedestrian, CatalogVehicle};
use crate::types::catalogs::files::CatalogFile;
use std::fs;
use std::path::{Path, PathBuf};

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
        let path_str = directory.path.as_literal().ok_or_else(|| {
            Error::invalid_value(
                "directory.path",
                "parameterized path",
                "literal path required",
            )
        })?;

        let dir_path = self.resolve_path(path_str)?;

        if !dir_path.exists() {
            return Err(Error::directory_not_found(&dir_path.to_string_lossy()));
        }

        if !dir_path.is_dir() {
            return Err(Error::invalid_value(
                "directory.path",
                &dir_path.to_string_lossy(),
                "path must be a directory",
            ));
        }

        let mut catalog_files = Vec::new();

        for entry in fs::read_dir(&dir_path)
            .map_err(|e| Error::file_read_error(&dir_path.to_string_lossy(), &e.to_string()))?
        {
            let entry = entry
                .map_err(|e| Error::file_read_error(&dir_path.to_string_lossy(), &e.to_string()))?;

            let path = entry.path();

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
            return Err(Error::file_not_found(&path.to_string_lossy()));
        }

        fs::read_to_string(path)
            .map_err(|e| Error::file_read_error(&path.to_string_lossy(), &e.to_string()))
    }

    /// Load and parse a catalog file into a CatalogFile structure
    pub fn load_and_parse_catalog_file<P: AsRef<Path>>(&self, file_path: P) -> Result<CatalogFile> {
        let path = file_path.as_ref();

        if !path.exists() {
            return Err(Error::catalog_error(&format!(
                "Catalog file does not exist: {}",
                path.display()
            )));
        }

        parse_catalog_from_file(path).map_err(|e| {
            e.with_context(&format!("Failed to parse catalog file: {}", path.display()))
        })
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
    pub fn load_controller_catalogs(
        &self,
        directory: &Directory,
    ) -> Result<Vec<CatalogController>> {
        let catalog_files = self.discover_catalog_files(directory)?;
        let mut controllers = Vec::new();

        for file_path in catalog_files {
            let catalog = self.load_and_parse_catalog_file(&file_path)?;
            controllers.extend(catalog.controllers().iter().cloned());
        }

        Ok(controllers)
    }

    /// Load all pedestrian catalogs from a directory
    pub fn load_pedestrian_catalogs(
        &self,
        directory: &Directory,
    ) -> Result<Vec<CatalogPedestrian>> {
        let catalog_files = self.discover_catalog_files(directory)?;
        let mut pedestrians = Vec::new();

        for file_path in catalog_files {
            let catalog = self.load_and_parse_catalog_file(&file_path)?;
            pedestrians.extend(catalog.pedestrians().iter().cloned());
        }

        Ok(pedestrians)
    }

    /// Find a specific entity in a catalog file
    pub fn find_entity_in_catalog<P: AsRef<Path>>(
        &self,
        file_path: P,
        entity_name: &str,
    ) -> Result<Option<String>> {
        let catalog = self.load_and_parse_catalog_file(file_path)?;

        // Check if any entity with the given name exists
        let entity_names = catalog.catalog.entity_names();
        if entity_names.contains(&entity_name.to_string()) {
            Ok(Some(entity_name.to_string()))
        } else {
            Ok(None)
        }
    }

    /// Load a specific controller catalog from a file
    pub fn load_controller_catalog<P: AsRef<Path>>(
        &self,
        file_path: P,
    ) -> Result<crate::types::catalogs::controllers::ControllerCatalog> {
        let path = file_path.as_ref();
        if !path.exists() {
            return Err(Error::file_not_found(&path.to_string_lossy()));
        }

        let xml_content = fs::read_to_string(path)
            .map_err(|e| Error::file_read_error(&path.to_string_lossy(), &e.to_string()))?;

        quick_xml::de::from_str(&xml_content)
            .map_err(|e| Error::parse_error(&path.to_string_lossy(), &e.to_string()))
    }

    /// Load a specific trajectory catalog from a file
    pub fn load_trajectory_catalog<P: AsRef<Path>>(
        &self,
        file_path: P,
    ) -> Result<crate::types::catalogs::trajectories::TrajectoryCatalog> {
        let path = file_path.as_ref();
        if !path.exists() {
            return Err(Error::file_not_found(&path.to_string_lossy()));
        }

        let xml_content = fs::read_to_string(path)
            .map_err(|e| Error::file_read_error(&path.to_string_lossy(), &e.to_string()))?;

        quick_xml::de::from_str(&xml_content)
            .map_err(|e| Error::parse_error(&path.to_string_lossy(), &e.to_string()))
    }

    /// Load a specific route catalog from a file
    pub fn load_route_catalog<P: AsRef<Path>>(
        &self,
        file_path: P,
    ) -> Result<crate::types::catalogs::routes::RouteCatalog> {
        let path = file_path.as_ref();
        if !path.exists() {
            return Err(Error::file_not_found(&path.to_string_lossy()));
        }

        let xml_content = fs::read_to_string(path)
            .map_err(|e| Error::file_read_error(&path.to_string_lossy(), &e.to_string()))?;

        quick_xml::de::from_str(&xml_content)
            .map_err(|e| Error::parse_error(&path.to_string_lossy(), &e.to_string()))
    }

    /// Load a specific environment catalog from a file
    pub fn load_environment_catalog<P: AsRef<Path>>(
        &self,
        file_path: P,
    ) -> Result<crate::types::catalogs::environments::EnvironmentCatalog> {
        let path = file_path.as_ref();
        if !path.exists() {
            return Err(Error::file_not_found(&path.to_string_lossy()));
        }

        let xml_content = fs::read_to_string(path)
            .map_err(|e| Error::file_read_error(&path.to_string_lossy(), &e.to_string()))?;

        quick_xml::de::from_str(&xml_content)
            .map_err(|e| Error::parse_error(&path.to_string_lossy(), &e.to_string()))
    }

    /// Load all controller catalogs from a directory and return them as a hashmap
    pub fn load_controller_catalogs_from_directory(
        &self,
        directory: &Directory,
    ) -> Result<
        std::collections::HashMap<String, crate::types::catalogs::controllers::ControllerCatalog>,
    > {
        let catalog_files = self.discover_catalog_files(directory)?;
        let mut catalogs = std::collections::HashMap::new();

        for file_path in catalog_files {
            if let Ok(catalog) = self.load_controller_catalog(&file_path) {
                let catalog_name = file_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string();
                catalogs.insert(catalog_name, catalog);
            }
        }

        Ok(catalogs)
    }

    /// Load all trajectory catalogs from a directory and return them as a hashmap
    pub fn load_trajectory_catalogs_from_directory(
        &self,
        directory: &Directory,
    ) -> Result<
        std::collections::HashMap<String, crate::types::catalogs::trajectories::TrajectoryCatalog>,
    > {
        let catalog_files = self.discover_catalog_files(directory)?;
        let mut catalogs = std::collections::HashMap::new();

        for file_path in catalog_files {
            if let Ok(catalog) = self.load_trajectory_catalog(&file_path) {
                let catalog_name = file_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string();
                catalogs.insert(catalog_name, catalog);
            }
        }

        Ok(catalogs)
    }

    /// Load all route catalogs from a directory and return them as a hashmap
    pub fn load_route_catalogs_from_directory(
        &self,
        directory: &Directory,
    ) -> Result<std::collections::HashMap<String, crate::types::catalogs::routes::RouteCatalog>>
    {
        let catalog_files = self.discover_catalog_files(directory)?;
        let mut catalogs = std::collections::HashMap::new();

        for file_path in catalog_files {
            if let Ok(catalog) = self.load_route_catalog(&file_path) {
                let catalog_name = file_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string();
                catalogs.insert(catalog_name, catalog);
            }
        }

        Ok(catalogs)
    }

    /// Load all environment catalogs from a directory and return them as a hashmap
    pub fn load_environment_catalogs_from_directory(
        &self,
        directory: &Directory,
    ) -> Result<
        std::collections::HashMap<String, crate::types::catalogs::environments::EnvironmentCatalog>,
    > {
        let catalog_files = self.discover_catalog_files(directory)?;
        let mut catalogs = std::collections::HashMap::new();

        for file_path in catalog_files {
            if let Ok(catalog) = self.load_environment_catalog(&file_path) {
                let catalog_name = file_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string();
                catalogs.insert(catalog_name, catalog);
            }
        }

        Ok(catalogs)
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
            Ok(std::env::current_dir()?.join(path))
        }
    }
}

impl Default for CatalogLoader {
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
        assert_eq!(
            loader_with_path.base_path.as_ref().unwrap(),
            Path::new("/tmp")
        );
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
        assert!(files[0]
            .file_name()
            .unwrap()
            .to_string_lossy()
            .starts_with("catalog"));
        assert!(files[1]
            .file_name()
            .unwrap()
            .to_string_lossy()
            .starts_with("catalog"));

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
