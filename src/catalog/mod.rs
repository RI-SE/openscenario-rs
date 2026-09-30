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
    fn default_matches_an_unrooted_manager() {
        assert_eq!(
            CatalogManager::default().base_path,
            CatalogManager::new().base_path
        );
    }
}

// Parameter extraction and catalog-reference resolution against a flat map both went through
// `extract_scenario_parameters` and `resolve_catalog_reference_simple` here. Both ignored the
// section 9.1 scoping of `ParameterDeclarations` and, for the latter, matched catalog names
// against a hard-coded set of filenames rather than the `<Catalog>` element's own name.
// `parser::resolve` is the one path that resolves a document's parameters and catalog
// references correctly; `parse_file_resolved`/`parse_str_resolved` are its public entry
// points. Both helpers were removed with their only callers, `examples/parse.rs` and
// `tools/scenario_analyzer.rs`, ported to `parse_file_resolved`.
