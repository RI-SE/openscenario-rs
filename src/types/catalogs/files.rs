//! Catalog file structure definitions
//!
//! This module contains types that represent the structure of catalog files,
//! matching the XML schema used in OpenSCENARIO catalog files.

use super::entities::{
    CatalogController, CatalogEnvironment, CatalogManeuver, CatalogMiscObject, CatalogPedestrian,
    CatalogRoute, CatalogTrajectory, CatalogVehicle,
};
use crate::types::basic::Value;
use crate::FileHeader;
use serde::{Deserialize, Serialize};

use crate::types::basic::OSString;

/// Represents a complete catalog file as loaded from .xosc files
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "OpenSCENARIO")]
pub struct CatalogFile {
    /// File header with metadata
    #[serde(rename = "FileHeader")]
    pub file_header: FileHeader,

    /// The main catalog content
    #[serde(rename = "Catalog")]
    pub catalog: CatalogContent,
}

/// Container for catalog entities of various types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CatalogContent {
    /// Name of this catalog
    #[serde(rename = "@name")]
    pub name: OSString,

    /// Vehicle entities in this catalog
    #[serde(rename = "Vehicle", skip_serializing_if = "Vec::is_empty", default)]
    pub vehicles: Vec<CatalogVehicle>,

    /// Controller entities in this catalog
    #[serde(rename = "Controller", skip_serializing_if = "Vec::is_empty", default)]
    pub controllers: Vec<CatalogController>,

    /// Pedestrian entities in this catalog
    #[serde(rename = "Pedestrian", skip_serializing_if = "Vec::is_empty", default)]
    pub pedestrians: Vec<CatalogPedestrian>,

    /// Miscellaneous object entities in this catalog
    #[serde(rename = "MiscObject", skip_serializing_if = "Vec::is_empty", default)]
    pub misc_objects: Vec<CatalogMiscObject>,

    /// Environment entities in this catalog
    #[serde(rename = "Environment", skip_serializing_if = "Vec::is_empty", default)]
    pub environments: Vec<CatalogEnvironment>,

    /// Maneuver entities in this catalog
    #[serde(rename = "Maneuver", skip_serializing_if = "Vec::is_empty", default)]
    pub maneuvers: Vec<CatalogManeuver>,

    /// Trajectory entities in this catalog
    #[serde(rename = "Trajectory", skip_serializing_if = "Vec::is_empty", default)]
    pub trajectories: Vec<CatalogTrajectory>,

    /// Route entities in this catalog
    #[serde(rename = "Route", skip_serializing_if = "Vec::is_empty", default)]
    pub routes: Vec<CatalogRoute>,
}

impl CatalogFile {
    /// Create a new empty catalog file
    pub fn new(name: String, author: String, description: String) -> Self {
        Self {
            file_header: FileHeader {
                rev_major: Value::Literal(1),
                rev_minor: Value::Literal(3),
                date: Value::Literal("2024-01-01T00:00:00".to_string()),
                description: Value::Literal(description),
                author: Value::Literal(author),
            },
            catalog: CatalogContent {
                name: Value::Literal(name),
                vehicles: Vec::new(),
                controllers: Vec::new(),
                pedestrians: Vec::new(),
                misc_objects: Vec::new(),
                environments: Vec::new(),
                maneuvers: Vec::new(),
                trajectories: Vec::new(),
                routes: Vec::new(),
            },
        }
    }

    /// Get the catalog name
    pub fn catalog_name(&self) -> &OSString {
        &self.catalog.name
    }

    /// Get all vehicles in this catalog
    pub fn vehicles(&self) -> &[CatalogVehicle] {
        &self.catalog.vehicles
    }

    /// Get all controllers in this catalog
    pub fn controllers(&self) -> &[CatalogController] {
        &self.catalog.controllers
    }

    /// Get all pedestrians in this catalog
    pub fn pedestrians(&self) -> &[CatalogPedestrian] {
        &self.catalog.pedestrians
    }

    /// Find a vehicle by name
    pub fn find_vehicle(&self, name: &str) -> Option<&CatalogVehicle> {
        self.catalog.vehicles.iter().find(|v| v.name == name)
    }

    /// Find a controller by name
    pub fn find_controller(&self, name: &str) -> Option<&CatalogController> {
        self.catalog.controllers.iter().find(|c| c.name == name)
    }

    /// Find a pedestrian by name
    pub fn find_pedestrian(&self, name: &str) -> Option<&CatalogPedestrian> {
        self.catalog.pedestrians.iter().find(|p| p.name == name)
    }
}

impl CatalogContent {
    /// Create a new empty catalog content
    pub fn new(name: String) -> Self {
        Self {
            name: Value::Literal(name),
            vehicles: Vec::new(),
            controllers: Vec::new(),
            pedestrians: Vec::new(),
            misc_objects: Vec::new(),
            environments: Vec::new(),
            maneuvers: Vec::new(),
            trajectories: Vec::new(),
            routes: Vec::new(),
        }
    }

    /// Add a vehicle to this catalog
    pub fn add_vehicle(&mut self, vehicle: CatalogVehicle) {
        self.vehicles.push(vehicle);
    }

    /// Add a controller to this catalog
    pub fn add_controller(&mut self, controller: CatalogController) {
        self.controllers.push(controller);
    }

    /// Add a pedestrian to this catalog
    pub fn add_pedestrian(&mut self, pedestrian: CatalogPedestrian) {
        self.pedestrians.push(pedestrian);
    }

    /// Get the total number of entities in this catalog
    pub fn entity_count(&self) -> usize {
        self.vehicles.len()
            + self.controllers.len()
            + self.pedestrians.len()
            + self.misc_objects.len()
            + self.environments.len()
            + self.maneuvers.len()
            + self.trajectories.len()
            + self.routes.len()
    }

    /// Get all entity names in this catalog
    pub fn entity_names(&self) -> Vec<String> {
        let mut names = Vec::new();

        names.extend(self.vehicles.iter().map(|v| v.name.clone()));
        names.extend(self.controllers.iter().map(|c| c.name.clone()));
        names.extend(self.pedestrians.iter().map(|p| p.name.clone()));
        names.extend(self.misc_objects.iter().map(|m| m.name.clone()));
        names.extend(self.environments.iter().map(|e| e.name.clone()));
        names.extend(self.maneuvers.iter().map(|m| m.name.clone()));
        names.extend(self.trajectories.iter().map(|t| t.name.clone()));
        names.extend(self.routes.iter().map(|r| r.name.clone()));

        names
    }
}

impl Default for CatalogFile {
    fn default() -> Self {
        Self::new(
            "DefaultCatalog".to_string(),
            "openscenario-rs".to_string(),
            "Default catalog file".to_string(),
        )
    }
}

impl Default for CatalogContent {
    fn default() -> Self {
        Self::new("DefaultCatalog".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalog_file_creation() {
        let catalog = CatalogFile::new(
            "TestCatalog".to_string(),
            "TestAuthor".to_string(),
            "Test Description".to_string(),
        );

        assert_eq!(catalog.catalog_name().as_literal().unwrap(), "TestCatalog");
        assert_eq!(
            catalog.file_header.author.as_literal().unwrap(),
            "TestAuthor"
        );
        assert_eq!(catalog.catalog.entity_count(), 0);
    }

    #[test]
    fn test_catalog_content_entity_management() {
        let content = CatalogContent::new("TestCatalog".to_string());

        // Initially empty
        assert_eq!(content.entity_count(), 0);
        assert_eq!(content.entity_names().len(), 0);
        assert_eq!(content.name.as_literal().unwrap(), "TestCatalog");
    }

    #[test]
    fn test_catalog_file_default() {
        let catalog = CatalogFile::default();
        assert_eq!(
            catalog.catalog_name().as_literal().unwrap(),
            "DefaultCatalog"
        );
        assert_eq!(
            catalog.file_header.author.as_literal().unwrap(),
            "openscenario-rs"
        );
    }

    #[test]
    fn test_catalog_content_default() {
        let content = CatalogContent::default();
        assert_eq!(content.name.as_literal().unwrap(), "DefaultCatalog");
        assert_eq!(content.entity_count(), 0);
    }
}
