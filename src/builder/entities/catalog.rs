//! Catalog-aware entity builders
//!
//! This module extends the entity builder system with catalog reference support,
//! allowing entities to be loaded from external catalog files.

use crate::builder::catalog::{
    CatalogEntityBuilder, PedestrianCatalogReferenceBuilder, VehicleCatalogReferenceBuilder,
};
use crate::builder::{
    scenario::HasEntities, scenario::ScenarioBuilder, BuilderError, BuilderResult,
};
use crate::types::{basic::OSString, entities::ScenarioObject};

/// Vehicle builder that can load from catalogs
pub struct CatalogVehicleBuilder<'parent> {
    parent: &'parent mut ScenarioBuilder<HasEntities>,
    name: String,
    catalog_builder: Option<CatalogEntityBuilder>,
}

impl<'parent> CatalogVehicleBuilder<'parent> {
    /// Create a new catalog vehicle builder
    pub fn new(parent: &'parent mut ScenarioBuilder<HasEntities>, name: &str) -> Self {
        Self {
            parent,
            name: name.to_string(),
            catalog_builder: None,
        }
    }

    /// Set the catalog entity builder for resolution
    pub fn with_catalog_builder(mut self, catalog_builder: CatalogEntityBuilder) -> Self {
        self.catalog_builder = Some(catalog_builder);
        self
    }

    /// Load vehicle from catalog reference
    pub fn from_catalog(
        self,
        catalog_name: &str,
        entry_name: &str,
    ) -> CatalogVehicleReferenceBuilder<'parent> {
        CatalogVehicleReferenceBuilder::new(self, catalog_name, entry_name)
    }

    /// Finish and add to scenario (for direct vehicle creation)
    pub fn finish(self) -> &'parent mut ScenarioBuilder<HasEntities> {
        // This would be used for direct vehicle creation without catalog
        // For now, we require catalog usage
        self.parent
    }
}

/// Builder for vehicle catalog references
pub struct CatalogVehicleReferenceBuilder<'parent> {
    parent: CatalogVehicleBuilder<'parent>,
    reference_builder: VehicleCatalogReferenceBuilder,
}

impl<'parent> CatalogVehicleReferenceBuilder<'parent> {
    /// Create a new catalog vehicle reference builder
    pub fn new(
        parent: CatalogVehicleBuilder<'parent>,
        catalog_name: &str,
        entry_name: &str,
    ) -> Self {
        let reference_builder = VehicleCatalogReferenceBuilder::new()
            .from_catalog(catalog_name)
            .entry(entry_name);

        Self {
            parent,
            reference_builder,
        }
    }

    /// Add a parameter assignment
    pub fn with_parameter(mut self, name: &str, value: &str) -> Self {
        self.reference_builder = self.reference_builder.with_parameter(name, value);
        self
    }

    /// Finish and resolve the catalog reference
    pub fn finish(self) -> BuilderResult<&'parent mut ScenarioBuilder<HasEntities>> {
        // Build the catalog reference
        let reference = self.reference_builder.build()?;

        // Resolve the vehicle from catalog
        let mut catalog_builder = self.parent.catalog_builder.ok_or_else(|| {
            BuilderError::validation_error(
                "Catalog builder not set. Use .with_catalog_builder() first",
            )
        })?;

        let vehicle = catalog_builder.resolve_vehicle_reference(&reference)?;

        // Create scenario object
        let scenario_object = ScenarioObject {
            name: OSString::literal(self.parent.name.clone()),
            catalog_reference: None, // TODO: Store catalog reference if needed
            vehicle: Some(vehicle),
            pedestrian: None,
            object_controller: None,
        };

        // Add to parent's entities
        if let Some(ref mut entities) = self.parent.parent.data.entities {
            entities.scenario_objects.push(scenario_object);
        }

        Ok(self.parent.parent)
    }
}

/// Pedestrian builder that can load from catalogs
pub struct CatalogPedestrianBuilder<'parent> {
    parent: &'parent mut ScenarioBuilder<HasEntities>,
    name: String,
    catalog_builder: Option<CatalogEntityBuilder>,
}

impl<'parent> CatalogPedestrianBuilder<'parent> {
    /// Create a new catalog pedestrian builder
    pub fn new(parent: &'parent mut ScenarioBuilder<HasEntities>, name: &str) -> Self {
        Self {
            parent,
            name: name.to_string(),
            catalog_builder: None,
        }
    }

    /// Set the catalog entity builder for resolution
    pub fn with_catalog_builder(mut self, catalog_builder: CatalogEntityBuilder) -> Self {
        self.catalog_builder = Some(catalog_builder);
        self
    }

    /// Load pedestrian from catalog reference
    pub fn from_catalog(
        self,
        catalog_name: &str,
        entry_name: &str,
    ) -> CatalogPedestrianReferenceBuilder<'parent> {
        CatalogPedestrianReferenceBuilder::new(self, catalog_name, entry_name)
    }

    /// Finish and add to scenario (for direct pedestrian creation)
    pub fn finish(self) -> &'parent mut ScenarioBuilder<HasEntities> {
        // This would be used for direct pedestrian creation without catalog
        // For now, we require catalog usage
        self.parent
    }
}

/// Builder for pedestrian catalog references
pub struct CatalogPedestrianReferenceBuilder<'parent> {
    parent: CatalogPedestrianBuilder<'parent>,
    reference_builder: PedestrianCatalogReferenceBuilder,
}

impl<'parent> CatalogPedestrianReferenceBuilder<'parent> {
    /// Create a new catalog pedestrian reference builder
    pub fn new(
        parent: CatalogPedestrianBuilder<'parent>,
        catalog_name: &str,
        entry_name: &str,
    ) -> Self {
        let reference_builder = PedestrianCatalogReferenceBuilder::new()
            .from_catalog(catalog_name)
            .entry(entry_name);

        Self {
            parent,
            reference_builder,
        }
    }

    /// Add a parameter assignment
    pub fn with_parameter(mut self, name: &str, value: &str) -> Self {
        self.reference_builder = self.reference_builder.with_parameter(name, value);
        self
    }

    /// Finish and resolve the catalog reference
    pub fn finish(self) -> BuilderResult<&'parent mut ScenarioBuilder<HasEntities>> {
        // Build the catalog reference
        let reference = self.reference_builder.build()?;

        // Resolve the pedestrian from catalog
        let mut catalog_builder = self.parent.catalog_builder.ok_or_else(|| {
            BuilderError::validation_error(
                "Catalog builder not set. Use .with_catalog_builder() first",
            )
        })?;

        let pedestrian = catalog_builder.resolve_pedestrian_reference(&reference)?;

        // Create scenario object
        let scenario_object = ScenarioObject {
            name: OSString::literal(self.parent.name.clone()),
            catalog_reference: None, // TODO: Store catalog reference if needed
            vehicle: None,
            pedestrian: Some(pedestrian),
            object_controller: None,
        };

        // Add to parent's entities
        if let Some(ref mut entities) = self.parent.parent.data.entities {
            entities.scenario_objects.push(scenario_object);
        }

        Ok(self.parent.parent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::catalog::CatalogLocationsBuilder;

    #[test]
    fn test_catalog_vehicle_builder_creation() {
        // This test would require a mock scenario builder
        // For now, just test that the types compile
    }

    #[test]
    fn test_catalog_pedestrian_builder_creation() {
        // This test would require a mock scenario builder
        // For now, just test that the types compile
    }
}
