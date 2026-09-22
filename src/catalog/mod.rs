//! Catalog loading and reference resolution.
//!
//! A catalog holds reusable definitions – vehicles, controllers, routes – in their own
//! file, which a scenario then references by catalog name and entry name. This module
//! loads those files from the `Directory` paths declared in `CatalogLocations` and
//! resolves each reference into the content it names.

use crate::types::basic::{Directory, OSString};

use crate::types::catalogs::entities::CatalogEntity;
use crate::types::catalogs::locations::{
    CatalogLocations, ControllerCatalogLocation, PedestrianCatalogLocation, VehicleCatalogLocation,
};
use crate::types::catalogs::references::{
    CatalogReference, ControllerCatalogReference, PedestrianCatalogReference,
    VehicleCatalogReference,
};
use crate::types::controllers::Controller;
use crate::types::entities::vehicle::Vehicle;
use std::path::{Path, PathBuf};

pub mod loader;
pub mod resolver;

// Re-export key types for convenience
pub use loader::CatalogLoader;
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
///
/// A single reference is resolved by the same code that resolves the references of a whole
/// document in [`crate::parser::resolve`]: the entry is located by catalog name and entry name,
/// and its own declarations, overridden by the reference's assignments, are the only
/// parameters it sees.
pub struct CatalogManager {
    loader: CatalogLoader,
    /// The directory a relative catalog directory is taken from; the current working directory
    /// when unset.
    base_path: Option<PathBuf>,
}

impl CatalogManager {
    /// Create a new catalog manager
    pub fn new() -> Self {
        Self {
            loader: CatalogLoader::new(),
            base_path: None,
        }
    }

    /// Create a catalog manager with a specific base path for relative path resolution
    pub fn with_base_path<P: AsRef<std::path::Path>>(base_path: P) -> Self {
        Self {
            loader: CatalogLoader::with_base_path(&base_path),
            base_path: Some(base_path.as_ref().to_path_buf()),
        }
    }

    /// Load a catalog from a directory, using cache if available
    pub fn load_catalog<T: CatalogLocation>(
        &mut self,
        location: &T,
    ) -> Result<T::CatalogType, crate::error::Error> {
        location.load_catalog()
    }

    /// Resolve a vehicle catalog reference to an actual vehicle
    pub fn resolve_vehicle_reference(
        &mut self,
        reference: &VehicleCatalogReference,
        location: &VehicleCatalogLocation,
    ) -> Result<ResolvedCatalog<Vehicle>, crate::error::Error> {
        self.resolve_reference(reference, "VehicleCatalog", &location.directory)
    }

    /// Resolve a controller catalog reference to an actual controller
    pub fn resolve_controller_reference(
        &mut self,
        reference: &ControllerCatalogReference,
        location: &ControllerCatalogLocation,
    ) -> Result<ResolvedCatalog<Controller>, crate::error::Error> {
        self.resolve_reference(reference, "ControllerCatalog", &location.directory)
    }

    /// Resolve a pedestrian catalog reference to an actual pedestrian
    pub fn resolve_pedestrian_reference(
        &mut self,
        reference: &PedestrianCatalogReference,
        location: &PedestrianCatalogLocation,
    ) -> Result<ResolvedCatalog<crate::types::entities::pedestrian::Pedestrian>, crate::error::Error>
    {
        self.resolve_reference(reference, "PedestrianCatalog", &location.directory)
    }

    /// Resolve `reference` against the catalogs in `directory`, the directory of the
    /// `CatalogLocations` child named `location`, and read the resolved entry as `R`.
    ///
    /// A reference built outside a document has no enclosing scope, so its names and values
    /// must be literal.
    fn resolve_reference<T, R>(
        &self,
        reference: &CatalogReference<T>,
        location: &str,
        directory: &Directory,
    ) -> Result<ResolvedCatalog<R>, crate::error::Error>
    where
        T: CatalogEntity,
        R: serde::de::DeserializeOwned,
    {
        let catalog_name = literal(&reference.catalog_name, "catalogName")?;
        let entry_name = literal(&reference.entry_name, "entryName")?;
        let mut assignments = Vec::new();
        if let Some(list) = &reference.parameter_assignments {
            for assignment in &list.assignments {
                assignments.push((
                    literal(&assignment.parameter_ref, "parameterRef")?,
                    literal(&assignment.value, "value")?,
                ));
            }
        }
        let directory = literal(&directory.path, "Directory path")?;
        let directory = match &self.base_path {
            Some(base) => base.join(&directory),
            None => Path::new(&directory).to_path_buf(),
        };

        let (xml, file) = crate::parser::resolve::resolve_catalog_entry(
            location,
            &directory,
            &catalog_name,
            &entry_name,
            &assignments,
        )?;
        let entity = quick_xml::de::from_str(&xml).map_err(|e| {
            crate::error::Error::catalog_error(&format!(
                "resolved entry `{}` of catalog `{}` could not be read: {}",
                entry_name, catalog_name, e
            ))
        })?;
        Ok(ResolvedCatalog::with_parameters(
            entity,
            file.to_string_lossy().into_owned(),
            entry_name,
            assignments.into_iter().collect(),
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
}

/// The literal text of `value`, or an error naming `attribute` when it is a parameter
/// reference or an expression, which only a document scope could resolve.
fn literal(value: &OSString, attribute: &str) -> Result<String, crate::error::Error> {
    value.as_literal().cloned().ok_or_else(|| {
        crate::error::Error::catalog_error(&format!(
            "{} `{}` is a parameter reference, and a reference resolved on its own has no \
             enclosing scope to resolve it in; resolve the whole document with \
             `parse_file_resolved` instead",
            attribute, value
        ))
    })
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
            .any(|v| v.name.as_literal() == Some(&resolved_entry_name));
        if found {
            return Ok(true);
        }
    }

    // Check pedestrians
    if !catalog_content.pedestrians.is_empty() {
        let found = catalog_content
            .pedestrians
            .iter()
            .any(|p| p.name.as_literal() == Some(&resolved_entry_name));
        if found {
            return Ok(true);
        }
    }

    // Check controllers
    if !catalog_content.controllers.is_empty() {
        let found = catalog_content
            .controllers
            .iter()
            .any(|c| c.name.as_literal() == Some(&resolved_entry_name));
        if found {
            return Ok(true);
        }
    }

    // Check misc objects
    if !catalog_content.misc_objects.is_empty() {
        let found = catalog_content
            .misc_objects
            .iter()
            .any(|m| m.name.as_literal() == Some(&resolved_entry_name));
        if found {
            return Ok(true);
        }
    }

    Ok(false)
}
