//! Catalog content types: the entries a catalog file holds, and the `CatalogReference`
//! a scenario uses to name one. Loading and resolution live in [`crate::catalog`].
pub mod entities;
pub mod environments;
pub mod files;
pub mod locations;
pub mod references;
pub mod routes;
pub mod trajectories;

// Re-export catalog types with explicit imports to avoid ambiguity
// Catalog entity types (from entities module - these are the primary definitions)
pub use entities::{
    CatalogAxles, CatalogEntity, CatalogFrontAxle, CatalogManeuver, CatalogMiscObject,
    CatalogPedestrian, CatalogPerformance, CatalogRearAxle, CatalogVehicle,
};

pub use entities::CatalogController;

// Fully-featured catalog entry types live in their specialized modules; these are
// the definitions used by `CatalogContent`, so re-export them from there.
pub use environments::CatalogEnvironment;
pub use routes::CatalogRoute;
pub use trajectories::CatalogTrajectory;

// Supporting types from specialized modules
pub use environments::{
    CatalogFog, CatalogPrecipitation, CatalogRoadCondition, CatalogSun, CatalogTimeOfDay,
    CatalogWeather,
};
pub use routes::RouteWaypoint;
pub use trajectories::{
    CatalogClothoid, CatalogNurbs, CatalogPolyline, CatalogTrajectoryShape, CatalogVertex,
};

// File and location types
pub use files::{CatalogContent, CatalogFile};
pub use locations::{
    CatalogLocations, ControllerCatalogLocation, EnvironmentCatalogLocation,
    ManeuverCatalogLocation, MiscObjectCatalogLocation, PedestrianCatalogLocation,
    RouteCatalogLocation, TrajectoryCatalogLocation, VehicleCatalogLocation,
};
pub use references::{
    CatalogReference, ControllerCatalogReference, ParameterAssignment, PedestrianCatalogReference,
    VehicleCatalogReference,
};
