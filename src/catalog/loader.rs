//! Loading and caching of catalog files.
//!
//! Catalogs are found under the `Directory` paths declared in `CatalogLocations`.
//! Each file is parsed on first use and cached by path, so repeated references into
//! the same catalog do not re-read it.

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

    /// Load the controller entries from a specific catalog file
    ///
    /// A catalog file is an `OpenSCENARIO` document whose body is a
    /// `Catalog` element (see `CatalogFile`/`CatalogContent`); this parses
    /// that structure and returns whatever `Controller` entries it holds
    /// (empty if the file is a catalog of some other kind).
    pub fn load_controller_catalog<P: AsRef<Path>>(
        &self,
        file_path: P,
    ) -> Result<Vec<CatalogController>> {
        let catalog = self.load_and_parse_catalog_file(file_path)?;
        Ok(catalog.catalog.controllers)
    }

    /// Load the trajectory entries from a specific catalog file
    pub fn load_trajectory_catalog<P: AsRef<Path>>(
        &self,
        file_path: P,
    ) -> Result<Vec<crate::types::catalogs::trajectories::CatalogTrajectory>> {
        let catalog = self.load_and_parse_catalog_file(file_path)?;
        Ok(catalog.catalog.trajectories)
    }

    /// Load the route entries from a specific catalog file
    pub fn load_route_catalog<P: AsRef<Path>>(
        &self,
        file_path: P,
    ) -> Result<Vec<crate::types::catalogs::routes::CatalogRoute>> {
        let catalog = self.load_and_parse_catalog_file(file_path)?;
        Ok(catalog.catalog.routes)
    }

    /// Load the environment entries from a specific catalog file
    pub fn load_environment_catalog<P: AsRef<Path>>(
        &self,
        file_path: P,
    ) -> Result<Vec<crate::types::catalogs::environments::CatalogEnvironment>> {
        let catalog = self.load_and_parse_catalog_file(file_path)?;
        Ok(catalog.catalog.environments)
    }

    /// Load all controller catalogs from a directory and return them as a hashmap
    ///
    /// A catalog directory legitimately mixes catalog kinds (e.g. a vehicle
    /// catalog alongside a controller catalog), so a file that parses fine
    /// but has no `Controller` entries is skipped rather than treated as an
    /// error. A genuine parse failure (malformed XML, not a valid catalog
    /// file at all) is propagated instead of silently discarded.
    pub fn load_controller_catalogs_from_directory(
        &self,
        directory: &Directory,
    ) -> Result<std::collections::HashMap<String, Vec<CatalogController>>> {
        let catalog_files = self.discover_catalog_files(directory)?;
        let mut catalogs = std::collections::HashMap::new();

        for file_path in catalog_files {
            let controllers = self.load_controller_catalog(&file_path)?;
            if controllers.is_empty() {
                continue;
            }
            let catalog_name = file_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();
            catalogs.insert(catalog_name, controllers);
        }

        Ok(catalogs)
    }

    /// Load all trajectory catalogs from a directory and return them as a hashmap
    ///
    /// See `load_controller_catalogs_from_directory` for the error-handling
    /// rationale: files with no `Trajectory` entries are skipped, real parse
    /// errors propagate.
    pub fn load_trajectory_catalogs_from_directory(
        &self,
        directory: &Directory,
    ) -> Result<
        std::collections::HashMap<
            String,
            Vec<crate::types::catalogs::trajectories::CatalogTrajectory>,
        >,
    > {
        let catalog_files = self.discover_catalog_files(directory)?;
        let mut catalogs = std::collections::HashMap::new();

        for file_path in catalog_files {
            let trajectories = self.load_trajectory_catalog(&file_path)?;
            if trajectories.is_empty() {
                continue;
            }
            let catalog_name = file_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();
            catalogs.insert(catalog_name, trajectories);
        }

        Ok(catalogs)
    }

    /// Load all route catalogs from a directory and return them as a hashmap
    ///
    /// See `load_controller_catalogs_from_directory` for the error-handling
    /// rationale: files with no `Route` entries are skipped, real parse
    /// errors propagate.
    pub fn load_route_catalogs_from_directory(
        &self,
        directory: &Directory,
    ) -> Result<std::collections::HashMap<String, Vec<crate::types::catalogs::routes::CatalogRoute>>>
    {
        let catalog_files = self.discover_catalog_files(directory)?;
        let mut catalogs = std::collections::HashMap::new();

        for file_path in catalog_files {
            let routes = self.load_route_catalog(&file_path)?;
            if routes.is_empty() {
                continue;
            }
            let catalog_name = file_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();
            catalogs.insert(catalog_name, routes);
        }

        Ok(catalogs)
    }

    /// Load all environment catalogs from a directory and return them as a hashmap
    ///
    /// See `load_controller_catalogs_from_directory` for the error-handling
    /// rationale: files with no `Environment` entries are skipped, real
    /// parse errors propagate.
    pub fn load_environment_catalogs_from_directory(
        &self,
        directory: &Directory,
    ) -> Result<
        std::collections::HashMap<
            String,
            Vec<crate::types::catalogs::environments::CatalogEnvironment>,
        >,
    > {
        let catalog_files = self.discover_catalog_files(directory)?;
        let mut catalogs = std::collections::HashMap::new();

        for file_path in catalog_files {
            let environments = self.load_environment_catalog(&file_path)?;
            if environments.is_empty() {
                continue;
            }
            let catalog_name = file_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();
            catalogs.insert(catalog_name, environments);
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
        let names: Vec<_> = files
            .iter()
            .map(|f| f.file_name().unwrap().to_string_lossy().into_owned())
            .collect();
        // Only `.xosc` files, sorted.
        assert_eq!(names, ["catalog1.xosc", "catalog2.xosc"]);

        // A relative directory is taken from the loader's base path.
        let rooted = CatalogLoader::with_base_path(dir_path.parent().unwrap());
        let relative = Directory::new(dir_path.file_name().unwrap().to_string_lossy().into_owned());
        assert_eq!(rooted.discover_catalog_files(&relative)?, files);

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

    /// A real controller catalog file is an `OpenSCENARIO` document with a
    /// `FileHeader` and a `Catalog name="..."` body holding `Controller`
    /// entries (XSD `:849-861`, `:1532-1537`) — not a standalone
    /// `<ControllerCatalog>` root, which does not exist in the schema.
    #[test]
    fn test_load_controller_catalog() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("controllers.xosc");
        let xml = r#"<?xml version="1.0"?>
<OpenSCENARIO>
    <FileHeader author="Test" date="2024-01-01T00:00:00" description="Controller catalog" revMajor="1" revMinor="3"/>
    <Catalog name="ControllerCatalog">
        <Controller name="AdaptiveCruiseController" controllerType="movement"/>
    </Catalog>
</OpenSCENARIO>"#;
        fs::write(&file_path, xml).unwrap();

        let loader = CatalogLoader::new();
        let controllers = loader.load_controller_catalog(&file_path)?;

        assert_eq!(controllers.len(), 1);
        assert_eq!(
            controllers[0].name.as_literal().unwrap(),
            "AdaptiveCruiseController"
        );

        Ok(())
    }

    /// A real trajectory catalog file's `Trajectory` entries carry a
    /// `Shape` (XSD `:2364` area references the schema-defined `Trajectory`
    /// complex type used inside a `Catalog`), not a bespoke
    /// `<TrajectoryCatalog>` root.
    #[test]
    fn test_load_trajectory_catalog() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("trajectories.xosc");
        let xml = r#"<?xml version="1.0"?>
<OpenSCENARIO>
    <FileHeader author="Test" date="2024-01-01T00:00:00" description="Trajectory catalog" revMajor="1" revMinor="3"/>
    <Catalog name="TrajectoryCatalog">
        <Trajectory name="StraightPath" closed="false">
            <Shape>
                <Polyline>
                    <Vertex>
                        <Position><WorldPosition x="0" y="0" z="0"/></Position>
                    </Vertex>
                    <Vertex>
                        <Position><WorldPosition x="10" y="0" z="0"/></Position>
                    </Vertex>
                </Polyline>
            </Shape>
        </Trajectory>
    </Catalog>
</OpenSCENARIO>"#;
        fs::write(&file_path, xml).unwrap();

        let loader = CatalogLoader::new();
        let trajectories = loader.load_trajectory_catalog(&file_path)?;

        assert_eq!(trajectories.len(), 1);
        assert_eq!(trajectories[0].name.as_literal().unwrap(), "StraightPath");

        Ok(())
    }

    /// A real route catalog file's `Route` entries carry `Waypoint`
    /// children directly under `Catalog`, not a bespoke `<RouteCatalog>`
    /// root (XSD `:1963` `RouteCatalogLocation` is an unrelated location
    /// type).
    #[test]
    fn test_load_route_catalog() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("routes.xosc");
        let xml = r#"<?xml version="1.0"?>
<OpenSCENARIO>
    <FileHeader author="Test" date="2024-01-01T00:00:00" description="Route catalog" revMajor="1" revMinor="3"/>
    <Catalog name="RouteCatalog">
        <Route name="MainRoute" closed="false">
            <Waypoint routeStrategy="shortest">
                <Position><WorldPosition x="0" y="0" z="0"/></Position>
            </Waypoint>
            <Waypoint routeStrategy="shortest">
                <Position><WorldPosition x="100" y="0" z="0"/></Position>
            </Waypoint>
        </Route>
    </Catalog>
</OpenSCENARIO>"#;
        fs::write(&file_path, xml).unwrap();

        let loader = CatalogLoader::new();
        let routes = loader.load_route_catalog(&file_path)?;

        assert_eq!(routes.len(), 1);
        assert_eq!(routes[0].name.as_literal().unwrap(), "MainRoute");
        assert_eq!(routes[0].waypoints.len(), 2);

        Ok(())
    }

    /// A real environment catalog file's `Environment` entries hold
    /// `Weather`/`TimeOfDay`/`RoadCondition` directly under `Catalog`, not a
    /// bespoke `<EnvironmentCatalog>` root (XSD `:1201`
    /// `EnvironmentCatalogLocation` is an unrelated location type).
    #[test]
    fn test_load_environment_catalog() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("environments.xosc");
        let xml = r#"<?xml version="1.0"?>
<OpenSCENARIO>
    <FileHeader author="Test" date="2024-01-01T00:00:00" description="Environment catalog" revMajor="1" revMinor="3"/>
    <Catalog name="EnvironmentCatalog">
        <Environment name="Sunny">
            <Weather fractionalCloudCover="zeroOktas">
                <Sun azimuth="0" elevation="1.571" illuminance="100000"/>
            </Weather>
        </Environment>
    </Catalog>
</OpenSCENARIO>"#;
        fs::write(&file_path, xml).unwrap();

        let loader = CatalogLoader::new();
        let environments = loader.load_environment_catalog(&file_path)?;

        assert_eq!(environments.len(), 1);
        assert_eq!(environments[0].name.as_literal().unwrap(), "Sunny");

        Ok(())
    }

    /// Directory-level loaders skip files that legitimately parse as some
    /// other catalog kind (no entries of the requested kind) but still
    /// return the ones that do match.
    #[test]
    fn test_load_controller_catalogs_from_directory_skips_other_catalog_kinds() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        fs::write(
            dir_path.join("controllers.xosc"),
            r#"<?xml version="1.0"?>
<OpenSCENARIO>
    <FileHeader author="Test" date="2024-01-01T00:00:00" description="d" revMajor="1" revMinor="3"/>
    <Catalog name="ControllerCatalog">
        <Controller name="Ctrl1"/>
    </Catalog>
</OpenSCENARIO>"#,
        )
        .unwrap();

        // A route catalog living in the same directory - has zero Controller
        // entries and must be skipped, not treated as an error.
        fs::write(
            dir_path.join("routes.xosc"),
            r#"<?xml version="1.0"?>
<OpenSCENARIO>
    <FileHeader author="Test" date="2024-01-01T00:00:00" description="d" revMajor="1" revMinor="3"/>
    <Catalog name="RouteCatalog">
        <Route name="R1" closed="false">
            <Waypoint routeStrategy="shortest">
                <Position><WorldPosition x="0" y="0" z="0"/></Position>
            </Waypoint>
            <Waypoint routeStrategy="shortest">
                <Position><WorldPosition x="10" y="0" z="0"/></Position>
            </Waypoint>
        </Route>
    </Catalog>
</OpenSCENARIO>"#,
        )
        .unwrap();

        let directory = Directory::new(dir_path.to_string_lossy().to_string());
        let loader = CatalogLoader::new();
        let catalogs = loader.load_controller_catalogs_from_directory(&directory)?;

        assert_eq!(catalogs.len(), 1);
        assert_eq!(catalogs["controllers"].len(), 1);
        assert_eq!(
            catalogs["controllers"][0].name.as_literal().unwrap(),
            "Ctrl1"
        );

        Ok(())
    }

    /// A genuinely malformed `.xosc` file must fail loudly rather than be
    /// swallowed as "no matching entries".
    #[test]
    fn test_load_controller_catalogs_from_directory_propagates_parse_errors() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        fs::write(dir_path.join("broken.xosc"), "<not valid xml").unwrap();

        let directory = Directory::new(dir_path.to_string_lossy().to_string());
        let loader = CatalogLoader::new();

        let err = loader
            .load_controller_catalogs_from_directory(&directory)
            .expect_err("a malformed catalog file is an error");
        assert!(err.to_string().contains("broken.xosc"), "{err}");
    }

    /// Regression: catalog files used to be modeled as a standalone
    /// `<ControllerCatalog revMajor=".." revMinor="..">` root element. No
    /// such complexType exists in the schema (only the unrelated
    /// `ControllerCatalogLocation`, XSD `:985`) — a real catalog file is
    /// always an `OpenSCENARIO` document with a `Catalog` body. Confirm the
    /// old invented shape is rejected rather than silently accepted.
    #[test]
    fn test_old_invented_root_shape_does_not_parse_as_catalog_file() {
        let old_shape_xml = r#"<ControllerCatalog revMajor="1" revMinor="0">
    <Controller name="Ctrl1"/>
</ControllerCatalog>"#;

        let loader = CatalogLoader::new();
        assert!(loader.parse_catalog_from_string(old_shape_xml).is_err());
    }

    /// `set_base_path` changes relative-path resolution after construction,
    /// the same way `with_base_path` does at construction time
    /// (`test_discover_catalog_files`'s "relative directory" case above).
    #[test]
    fn test_set_base_path_changes_relative_resolution() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();
        fs::write(dir_path.join("catalog1.xosc"), "catalog1 content")?;

        let mut loader = CatalogLoader::new();
        loader.set_base_path(dir_path.parent().unwrap());
        let relative = Directory::new(dir_path.file_name().unwrap().to_string_lossy().into_owned());

        let files = loader.discover_catalog_files(&relative)?;
        assert_eq!(files, vec![dir_path.join("catalog1.xosc")]);

        Ok(())
    }

    /// A parameterized directory path has no enclosing scope to resolve it
    /// in when discovered on its own (mirrors `CatalogManager::literal`'s
    /// rationale for the whole-document case).
    #[test]
    fn test_discover_catalog_files_rejects_parameterized_path() {
        let directory = Directory::from_parameter("CatalogDir".to_string());
        let loader = CatalogLoader::new();

        let err = loader
            .discover_catalog_files(&directory)
            .expect_err("a parameterized directory path cannot be discovered on its own");
        assert!(err.to_string().contains("directory.path"), "{err}");
    }

    #[test]
    fn test_discover_catalog_files_directory_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let missing = temp_dir.path().join("does_not_exist");
        let directory = Directory::new(missing.to_string_lossy().to_string());
        let loader = CatalogLoader::new();

        let err = loader
            .discover_catalog_files(&directory)
            .expect_err("a missing directory is an error");
        assert!(err.to_string().contains("does_not_exist"), "{err}");
    }

    #[test]
    fn test_discover_catalog_files_rejects_a_file_as_directory() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("not_a_directory.xosc");
        fs::write(&file_path, "content")?;
        let directory = Directory::new(file_path.to_string_lossy().to_string());
        let loader = CatalogLoader::new();

        let err = loader
            .discover_catalog_files(&directory)
            .expect_err("a file is not a directory");
        assert!(err.to_string().contains("must be a directory"), "{err}");

        Ok(())
    }

    #[test]
    fn test_load_catalog_file_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let missing = temp_dir.path().join("missing.xosc");
        let loader = CatalogLoader::new();

        let err = loader
            .load_catalog_file(&missing)
            .expect_err("a missing file is an error");
        assert!(err.to_string().contains("missing.xosc"), "{err}");
    }

    #[test]
    fn test_load_and_parse_catalog_file_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let missing = temp_dir.path().join("missing.xosc");
        let loader = CatalogLoader::new();

        let err = loader
            .load_and_parse_catalog_file(&missing)
            .expect_err("a missing file is an error");
        assert!(err.to_string().contains("missing.xosc"), "{err}");
    }

    /// `load_vehicle_catalogs`, `load_controller_catalogs` and
    /// `load_pedestrian_catalogs` each aggregate their own entity kind
    /// across every `.xosc` file in a directory, not just the first one
    /// found. One row per kind, so a regression confined to one loader's
    /// aggregation doesn't silently leave the other two untested.
    #[test]
    fn test_load_catalogs_aggregate_across_directory_by_kind() -> Result<()> {
        enum Kind {
            Vehicle,
            Controller,
            Pedestrian,
        }

        // Writes the fixture pair for `kind` into `dir_path`; each kind's
        // catalog element differs, so this is the "helper" side of the table.
        fn write_fixtures(dir_path: &Path, kind: &Kind, name_a: &str, name_b: &str) {
            let entry_xml = |name: &str| -> String {
                match kind {
                    Kind::Vehicle => format!(
                        r#"<Vehicle name="{name}" vehicleCategory="car">
            <BoundingBox>
                <Center x="1.0" y="0.0" z="0.8"/>
                <Dimensions width="1.8" length="4.0" height="1.6"/>
            </BoundingBox>
            <Performance maxSpeed="40" maxAcceleration="4" maxDeceleration="6"/>
            <Axles>
                <FrontAxle maxSteering="0.4" wheelDiameter="0.55" trackWidth="1.6" positionX="2.5" positionZ="0.25"/>
                <RearAxle maxSteering="0.0" wheelDiameter="0.55" trackWidth="1.6" positionX="0.0" positionZ="0.25"/>
            </Axles>
        </Vehicle>"#
                    ),
                    Kind::Controller => {
                        format!(r#"<Controller name="{name}" controllerType="movement"/>"#)
                    }
                    Kind::Pedestrian => format!(
                        r#"<Pedestrian name="{name}" pedestrianCategory="pedestrian" mass="75.0">
            <BoundingBox>
                <Center x="0" y="0" z="0"/>
                <Dimensions width="0.5" length="0.5" height="1.8"/>
            </BoundingBox>
        </Pedestrian>"#
                    ),
                }
            };
            let catalog_name = match kind {
                Kind::Vehicle => "VehicleCatalog",
                Kind::Controller => "ControllerCatalog",
                Kind::Pedestrian => "PedestrianCatalog",
            };
            for (file_name, name) in [("a.xosc", name_a), ("b.xosc", name_b)] {
                fs::write(
                    dir_path.join(file_name),
                    format!(
                        r#"<?xml version="1.0"?>
<OpenSCENARIO>
    <FileHeader author="Test" date="2024-01-01T00:00:00" description="d" revMajor="1" revMinor="3"/>
    <Catalog name="{catalog_name}">
        {}
    </Catalog>
</OpenSCENARIO>"#,
                        entry_xml(name)
                    ),
                )
                .unwrap();
            }
        }

        for (label, kind, name_a, name_b) in [
            ("vehicle", Kind::Vehicle, "CarA", "CarB"),
            ("controller", Kind::Controller, "Ctrl1", "Ctrl2"),
            ("pedestrian", Kind::Pedestrian, "PedA", "PedB"),
        ] {
            let temp_dir = TempDir::new().unwrap();
            let dir_path = temp_dir.path();
            write_fixtures(dir_path, &kind, name_a, name_b);

            let directory = Directory::new(dir_path.to_string_lossy().to_string());
            let loader = CatalogLoader::new();

            let names: Vec<String> = match kind {
                Kind::Vehicle => loader
                    .load_vehicle_catalogs(&directory)?
                    .iter()
                    .map(|v| v.name.as_literal().unwrap().clone())
                    .collect(),
                Kind::Controller => loader
                    .load_controller_catalogs(&directory)?
                    .iter()
                    .map(|c| c.name.as_literal().unwrap().clone())
                    .collect(),
                Kind::Pedestrian => loader
                    .load_pedestrian_catalogs(&directory)?
                    .iter()
                    .map(|p| p.name.as_literal().unwrap().clone())
                    .collect(),
            };

            assert_eq!(names, [name_a, name_b], "kind: {label}");
        }

        Ok(())
    }

    #[test]
    fn test_find_entity_in_catalog() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("controllers.xosc");
        let xml = r#"<?xml version="1.0"?>
<OpenSCENARIO>
    <FileHeader author="Test" date="2024-01-01T00:00:00" description="d" revMajor="1" revMinor="3"/>
    <Catalog name="ControllerCatalog">
        <Controller name="Ctrl1" controllerType="movement"/>
    </Catalog>
</OpenSCENARIO>"#;
        fs::write(&file_path, xml).unwrap();

        let loader = CatalogLoader::new();
        assert_eq!(
            loader.find_entity_in_catalog(&file_path, "Ctrl1")?,
            Some("Ctrl1".to_string())
        );
        assert_eq!(
            loader.find_entity_in_catalog(&file_path, "NoSuchEntity")?,
            None
        );

        Ok(())
    }

    /// `load_trajectory_catalogs_from_directory`, `load_route_catalogs_from_directory`
    /// and `load_environment_catalogs_from_directory` all key their hashmap by file
    /// stem the same way `load_controller_catalogs_from_directory` (tested above)
    /// does. One row per kind, so a regression confined to one of the three
    /// doesn't silently leave the other two untested.
    #[test]
    fn test_load_catalogs_from_directory_by_kind() -> Result<()> {
        struct Row {
            kind: &'static str,
            file_name: &'static str,
            xml: &'static str,
            entry_name: &'static str,
        }

        fn assert_single_entry<T>(
            catalogs: &std::collections::HashMap<String, Vec<T>>,
            key: &str,
            kind: &str,
            name_of: impl Fn(&T) -> String,
            expected_name: &str,
        ) {
            assert_eq!(catalogs.len(), 1, "kind: {kind}");
            let entries = catalogs
                .get(key)
                .unwrap_or_else(|| panic!("kind: {kind}: missing hashmap key {key:?}"));
            assert_eq!(entries.len(), 1, "kind: {kind}");
            assert_eq!(name_of(&entries[0]), expected_name, "kind: {kind}");
        }

        let rows = [
            Row {
                kind: "trajectory",
                file_name: "trajectories.xosc",
                xml: r#"<?xml version="1.0"?>
<OpenSCENARIO>
    <FileHeader author="Test" date="2024-01-01T00:00:00" description="d" revMajor="1" revMinor="3"/>
    <Catalog name="TrajectoryCatalog">
        <Trajectory name="StraightPath" closed="false">
            <Shape>
                <Polyline>
                    <Vertex>
                        <Position><WorldPosition x="0" y="0" z="0"/></Position>
                    </Vertex>
                    <Vertex>
                        <Position><WorldPosition x="10" y="0" z="0"/></Position>
                    </Vertex>
                </Polyline>
            </Shape>
        </Trajectory>
    </Catalog>
</OpenSCENARIO>"#,
                entry_name: "StraightPath",
            },
            Row {
                kind: "route",
                file_name: "routes.xosc",
                xml: r#"<?xml version="1.0"?>
<OpenSCENARIO>
    <FileHeader author="Test" date="2024-01-01T00:00:00" description="d" revMajor="1" revMinor="3"/>
    <Catalog name="RouteCatalog">
        <Route name="MainRoute" closed="false">
            <Waypoint routeStrategy="shortest">
                <Position><WorldPosition x="0" y="0" z="0"/></Position>
            </Waypoint>
            <Waypoint routeStrategy="shortest">
                <Position><WorldPosition x="100" y="0" z="0"/></Position>
            </Waypoint>
        </Route>
    </Catalog>
</OpenSCENARIO>"#,
                entry_name: "MainRoute",
            },
            Row {
                kind: "environment",
                file_name: "environments.xosc",
                xml: r#"<?xml version="1.0"?>
<OpenSCENARIO>
    <FileHeader author="Test" date="2024-01-01T00:00:00" description="d" revMajor="1" revMinor="3"/>
    <Catalog name="EnvironmentCatalog">
        <Environment name="Sunny">
            <Weather fractionalCloudCover="zeroOktas">
                <Sun azimuth="0" elevation="1.571" illuminance="100000"/>
            </Weather>
        </Environment>
    </Catalog>
</OpenSCENARIO>"#,
                entry_name: "Sunny",
            },
        ];

        for row in rows {
            let temp_dir = TempDir::new().unwrap();
            let dir_path = temp_dir.path();
            fs::write(dir_path.join(row.file_name), row.xml).unwrap();

            let directory = Directory::new(dir_path.to_string_lossy().to_string());
            let loader = CatalogLoader::new();
            let expected_key = Path::new(row.file_name)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap()
                .to_string();

            match row.kind {
                "trajectory" => {
                    let catalogs = loader.load_trajectory_catalogs_from_directory(&directory)?;
                    assert_single_entry(
                        &catalogs,
                        &expected_key,
                        row.kind,
                        |t| t.name.as_literal().unwrap().clone(),
                        row.entry_name,
                    );
                }
                "route" => {
                    let catalogs = loader.load_route_catalogs_from_directory(&directory)?;
                    assert_single_entry(
                        &catalogs,
                        &expected_key,
                        row.kind,
                        |r| r.name.as_literal().unwrap().clone(),
                        row.entry_name,
                    );
                }
                "environment" => {
                    let catalogs = loader.load_environment_catalogs_from_directory(&directory)?;
                    assert_single_entry(
                        &catalogs,
                        &expected_key,
                        row.kind,
                        |e| e.name.as_literal().unwrap().clone(),
                        row.entry_name,
                    );
                }
                other => unreachable!("unhandled row kind: {other}"),
            }
        }

        Ok(())
    }
}
