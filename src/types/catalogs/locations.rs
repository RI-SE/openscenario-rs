//! Catalog location types for OpenSCENARIO
//!
//! This file contains all 8 catalog location types that reference Directory paths:
//! - VehicleCatalogLocation
//! - ControllerCatalogLocation  
//! - PedestrianCatalogLocation
//! - MiscObjectCatalogLocation
//! - EnvironmentCatalogLocation
//! - ManeuverCatalogLocation
//! - TrajectoryCatalogLocation
//! - RouteCatalogLocation

use crate::types::basic::Directory;

use serde::{Deserialize, Serialize};

/// Location specification for vehicle catalogs
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VehicleCatalogLocation {
    /// Directory containing vehicle catalog files
    #[serde(rename = "Directory")]
    pub directory: Directory,
}

impl VehicleCatalogLocation {
    /// Create a new vehicle catalog location
    pub fn new(directory: Directory) -> Self {
        Self { directory }
    }

    /// Create a vehicle catalog location from a path string
    pub fn from_path(path: String) -> Self {
        Self::new(Directory::new(path))
    }
}

/// Location specification for controller catalogs
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ControllerCatalogLocation {
    /// Directory containing controller catalog files
    #[serde(rename = "Directory")]
    pub directory: Directory,
}

impl ControllerCatalogLocation {
    /// Create a new controller catalog location
    pub fn new(directory: Directory) -> Self {
        Self { directory }
    }

    /// Create a controller catalog location from a path string
    pub fn from_path(path: String) -> Self {
        Self::new(Directory::new(path))
    }
}

/// Location specification for pedestrian catalogs
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PedestrianCatalogLocation {
    /// Directory containing pedestrian catalog files
    #[serde(rename = "Directory")]
    pub directory: Directory,
}

impl PedestrianCatalogLocation {
    /// Create a new pedestrian catalog location
    pub fn new(directory: Directory) -> Self {
        Self { directory }
    }

    /// Create a pedestrian catalog location from a path string
    pub fn from_path(path: String) -> Self {
        Self::new(Directory::new(path))
    }
}

/// Location specification for miscellaneous object catalogs
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MiscObjectCatalogLocation {
    /// Directory containing miscellaneous object catalog files
    #[serde(rename = "Directory")]
    pub directory: Directory,
}

impl MiscObjectCatalogLocation {
    /// Create a new miscellaneous object catalog location
    pub fn new(directory: Directory) -> Self {
        Self { directory }
    }

    /// Create a miscellaneous object catalog location from a path string
    pub fn from_path(path: String) -> Self {
        Self::new(Directory::new(path))
    }
}

/// Location specification for environment catalogs
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EnvironmentCatalogLocation {
    /// Directory containing environment catalog files
    #[serde(rename = "Directory")]
    pub directory: Directory,
}

impl EnvironmentCatalogLocation {
    /// Create a new environment catalog location
    pub fn new(directory: Directory) -> Self {
        Self { directory }
    }

    /// Create an environment catalog location from a path string
    pub fn from_path(path: String) -> Self {
        Self::new(Directory::new(path))
    }
}

/// Location specification for maneuver catalogs
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ManeuverCatalogLocation {
    /// Directory containing maneuver catalog files
    #[serde(rename = "Directory")]
    pub directory: Directory,
}

impl ManeuverCatalogLocation {
    /// Create a new maneuver catalog location
    pub fn new(directory: Directory) -> Self {
        Self { directory }
    }

    /// Create a maneuver catalog location from a path string
    pub fn from_path(path: String) -> Self {
        Self::new(Directory::new(path))
    }
}

/// Location specification for trajectory catalogs
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TrajectoryCatalogLocation {
    /// Directory containing trajectory catalog files
    #[serde(rename = "Directory")]
    pub directory: Directory,
}

impl TrajectoryCatalogLocation {
    /// Create a new trajectory catalog location
    pub fn new(directory: Directory) -> Self {
        Self { directory }
    }

    /// Create a trajectory catalog location from a path string
    pub fn from_path(path: String) -> Self {
        Self::new(Directory::new(path))
    }
}

/// Location specification for route catalogs
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RouteCatalogLocation {
    /// Directory containing route catalog files
    #[serde(rename = "Directory")]
    pub directory: Directory,
}

impl RouteCatalogLocation {
    /// Create a new route catalog location
    pub fn new(directory: Directory) -> Self {
        Self { directory }
    }

    /// Create a route catalog location from a path string
    pub fn from_path(path: String) -> Self {
        Self::new(Directory::new(path))
    }
}

/// Container for all catalog locations as specified in CatalogLocations element
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CatalogLocations {
    /// Vehicle catalog location
    #[serde(rename = "VehicleCatalog", skip_serializing_if = "Option::is_none")]
    pub vehicle_catalog: Option<VehicleCatalogLocation>,

    /// Controller catalog location
    #[serde(rename = "ControllerCatalog", skip_serializing_if = "Option::is_none")]
    pub controller_catalog: Option<ControllerCatalogLocation>,

    /// Pedestrian catalog location
    #[serde(rename = "PedestrianCatalog", skip_serializing_if = "Option::is_none")]
    pub pedestrian_catalog: Option<PedestrianCatalogLocation>,

    /// Miscellaneous object catalog location
    #[serde(rename = "MiscObjectCatalog", skip_serializing_if = "Option::is_none")]
    pub misc_object_catalog: Option<MiscObjectCatalogLocation>,

    /// Environment catalog location
    #[serde(rename = "EnvironmentCatalog", skip_serializing_if = "Option::is_none")]
    pub environment_catalog: Option<EnvironmentCatalogLocation>,

    /// Maneuver catalog location
    #[serde(rename = "ManeuverCatalog", skip_serializing_if = "Option::is_none")]
    pub maneuver_catalog: Option<ManeuverCatalogLocation>,

    /// Trajectory catalog location
    #[serde(rename = "TrajectoryCatalog", skip_serializing_if = "Option::is_none")]
    pub trajectory_catalog: Option<TrajectoryCatalogLocation>,

    /// Route catalog location
    #[serde(rename = "RouteCatalog", skip_serializing_if = "Option::is_none")]
    pub route_catalog: Option<RouteCatalogLocation>,
}

impl CatalogLocations {
    /// Create an empty catalog locations container
    pub fn new() -> Self {
        Self {
            vehicle_catalog: None,
            controller_catalog: None,
            pedestrian_catalog: None,
            misc_object_catalog: None,
            environment_catalog: None,
            maneuver_catalog: None,
            trajectory_catalog: None,
            route_catalog: None,
        }
    }

    /// Check if any catalog locations are specified
    pub fn has_catalogs(&self) -> bool {
        self.vehicle_catalog.is_some()
            || self.controller_catalog.is_some()
            || self.pedestrian_catalog.is_some()
            || self.misc_object_catalog.is_some()
            || self.environment_catalog.is_some()
            || self.maneuver_catalog.is_some()
            || self.trajectory_catalog.is_some()
            || self.route_catalog.is_some()
    }

    /// Count the number of specified catalog locations
    pub fn catalog_count(&self) -> usize {
        let mut count = 0;
        if self.vehicle_catalog.is_some() {
            count += 1;
        }
        if self.controller_catalog.is_some() {
            count += 1;
        }
        if self.pedestrian_catalog.is_some() {
            count += 1;
        }
        if self.misc_object_catalog.is_some() {
            count += 1;
        }
        if self.environment_catalog.is_some() {
            count += 1;
        }
        if self.maneuver_catalog.is_some() {
            count += 1;
        }
        if self.trajectory_catalog.is_some() {
            count += 1;
        }
        if self.route_catalog.is_some() {
            count += 1;
        }
        count
    }
}

impl Default for CatalogLocations {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vehicle_catalog_location() {
        let location = VehicleCatalogLocation::from_path("/catalogs/vehicles".to_string());
        assert_eq!(
            location.directory.path.as_literal().unwrap(),
            "/catalogs/vehicles"
        );

        let directory = Directory::new("./vehicles".to_string());
        let location2 = VehicleCatalogLocation::new(directory.clone());
        assert_eq!(&location2.directory, &directory);
    }

    #[test]
    fn test_controller_catalog_location() {
        let location = ControllerCatalogLocation::from_path("/catalogs/controllers".to_string());
        assert_eq!(
            location.directory.path.as_literal().unwrap(),
            "/catalogs/controllers"
        );
    }

    #[test]
    fn test_catalog_locations_container() {
        let mut locations = CatalogLocations::new();
        assert!(!locations.has_catalogs());
        assert_eq!(locations.catalog_count(), 0);

        locations.vehicle_catalog =
            Some(VehicleCatalogLocation::from_path("/vehicles".to_string()));
        locations.controller_catalog = Some(ControllerCatalogLocation::from_path(
            "/controllers".to_string(),
        ));

        assert!(locations.has_catalogs());
        assert_eq!(locations.catalog_count(), 2);
    }

    #[test]
    fn test_all_catalog_location_types() {
        // Test creation of all 8 catalog location types
        let vehicle = VehicleCatalogLocation::from_path("/vehicles".to_string());
        let controller = ControllerCatalogLocation::from_path("/controllers".to_string());
        let pedestrian = PedestrianCatalogLocation::from_path("/pedestrians".to_string());
        let misc_object = MiscObjectCatalogLocation::from_path("/misc_objects".to_string());
        let environment = EnvironmentCatalogLocation::from_path("/environments".to_string());
        let maneuver = ManeuverCatalogLocation::from_path("/maneuvers".to_string());
        let trajectory = TrajectoryCatalogLocation::from_path("/trajectories".to_string());
        let route = RouteCatalogLocation::from_path("/routes".to_string());

        // Verify all are created correctly
        assert_eq!(vehicle.directory.path.as_literal().unwrap(), "/vehicles");
        assert_eq!(
            controller.directory.path.as_literal().unwrap(),
            "/controllers"
        );
        assert_eq!(
            pedestrian.directory.path.as_literal().unwrap(),
            "/pedestrians"
        );
        assert_eq!(
            misc_object.directory.path.as_literal().unwrap(),
            "/misc_objects"
        );
        assert_eq!(
            environment.directory.path.as_literal().unwrap(),
            "/environments"
        );
        assert_eq!(maneuver.directory.path.as_literal().unwrap(), "/maneuvers");
        assert_eq!(
            trajectory.directory.path.as_literal().unwrap(),
            "/trajectories"
        );
        assert_eq!(route.directory.path.as_literal().unwrap(), "/routes");
    }
}
