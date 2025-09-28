//! Entity builders for programmatic scenario construction

pub mod vehicle;
pub mod catalog;

pub use vehicle::VehicleBuilder;
pub use catalog::{CatalogVehicleBuilder, CatalogPedestrianBuilder};

use crate::builder::{BuilderError, BuilderResult};
use crate::types::entities::{ScenarioObject, Entities};

/// Collection of entity builders for scenario construction
#[derive(Debug, Default)]
pub struct EntityCollection {
    objects: Vec<ScenarioObject>,
}

impl EntityCollection {
    /// Create a new empty entity collection
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add a scenario object to the collection
    pub fn add_object(&mut self, object: ScenarioObject) {
        self.objects.push(object);
    }
    
    /// Convert to Entities structure
    pub fn into_entities(self) -> Entities {
        Entities {
            scenario_objects: self.objects,
        }
    }
    
    /// Get all objects
    pub fn objects(&self) -> &[ScenarioObject] {
        &self.objects
    }
}