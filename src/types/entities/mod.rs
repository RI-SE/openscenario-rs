//! Entity definitions for OpenSCENARIO scenarios

use crate::types::basic::OSString;
use crate::types::controllers::ObjectController;
use serde::{Deserialize, Serialize};

pub mod axles;
pub mod pedestrian;
pub mod selection;
pub mod vehicle;

// Re-export entity types
pub use axles::{Axle, Axles};
pub use pedestrian::Pedestrian;
pub use selection::{
    ByName, ByObjectType, ByType, EntityDistribution, EntityDistributionEntry, EntitySelection,
    ExternalObjectReference, ScenarioObjectTemplate, SelectedEntities, TemplateProperties,
    TemplateProperty,
};
pub use vehicle::{Properties, Vehicle};

/// Union type for all entity objects
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum EntityObject {
    /// Vehicle entity
    Vehicle(Box<Vehicle>),
    /// Pedestrian entity
    Pedestrian(Box<Pedestrian>),
    // TODO: Add MiscellaneousObject later
    // MiscellaneousObject(MiscObject),
}

/// Catalog reference for scenario entities (vehicle or pedestrian)
///
/// This enum wraps typed catalog references to handle the XSD constraint that
/// only one CatalogReference element can exist per ScenarioObject. The actual
/// type (vehicle vs pedestrian) is determined at runtime during catalog resolution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScenarioEntityReference {
    /// Vehicle catalog reference
    Vehicle(
        crate::types::catalogs::references::CatalogReference<
            crate::types::catalogs::entities::CatalogVehicle,
        >,
    ),
    /// Pedestrian catalog reference
    Pedestrian(
        crate::types::catalogs::references::CatalogReference<
            crate::types::catalogs::entities::CatalogPedestrian,
        >,
    ),
}

/// Wrapper for scenario objects containing entity information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScenarioObject {
    /// Name of scenario object (used for references)
    #[serde(rename = "@name")]
    pub name: OSString,

    /// Vehicle entity (optional)
    #[serde(rename = "Vehicle", skip_serializing_if = "Option::is_none")]
    pub vehicle: Option<Vehicle>,

    /// Pedestrian entity (optional)
    #[serde(rename = "Pedestrian", skip_serializing_if = "Option::is_none")]
    pub pedestrian: Option<Pedestrian>,

    /// Entity catalog reference (vehicle or pedestrian)
    ///
    /// References a vehicle or pedestrian from an external catalog.
    /// Mutually exclusive with direct vehicle/pedestrian definitions.
    #[serde(rename = "CatalogReference", skip_serializing_if = "Option::is_none")]
    pub entity_catalog_reference: Option<ScenarioEntityReference>,

    /// Object controller configuration (optional)
    #[serde(rename = "ObjectController", skip_serializing_if = "Option::is_none")]
    pub object_controller: Option<ObjectController>,
}

/// Container for all entities in the scenario
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Entities {
    /// List of scenario objects
    #[serde(rename = "ScenarioObject", default)]
    pub scenario_objects: Vec<ScenarioObject>,
}

impl ScenarioObject {
    /// Create a new scenario object with a vehicle
    pub fn new_vehicle(name: String, vehicle: Vehicle) -> Self {
        Self {
            name: crate::types::basic::Value::literal(name),
            vehicle: Some(vehicle),
            pedestrian: None,
            entity_catalog_reference: None,
            object_controller: Some(ObjectController::default()),
        }
    }

    /// Create a new scenario object with a pedestrian
    pub fn new_pedestrian(name: String, pedestrian: Pedestrian) -> Self {
        Self {
            name: crate::types::basic::Value::literal(name),
            vehicle: None,
            pedestrian: Some(pedestrian),
            entity_catalog_reference: None,
            object_controller: Some(ObjectController::default()),
        }
    }

    /// Create a new scenario object with a vehicle catalog reference
    pub fn new_vehicle_catalog_reference(
        name: String,
        catalog_reference: crate::types::catalogs::references::CatalogReference<
            crate::types::catalogs::entities::CatalogVehicle,
        >,
    ) -> Self {
        Self {
            name: crate::types::basic::Value::literal(name),
            vehicle: None,
            pedestrian: None,
            entity_catalog_reference: Some(ScenarioEntityReference::Vehicle(catalog_reference)),
            object_controller: Some(ObjectController::default()),
        }
    }

    /// Create a new scenario object with a pedestrian catalog reference
    pub fn new_pedestrian_catalog_reference(
        name: String,
        catalog_reference: crate::types::catalogs::references::CatalogReference<
            crate::types::catalogs::entities::CatalogPedestrian,
        >,
    ) -> Self {
        Self {
            name: crate::types::basic::Value::literal(name),
            vehicle: None,
            pedestrian: None,
            entity_catalog_reference: Some(ScenarioEntityReference::Pedestrian(catalog_reference)),
            object_controller: Some(ObjectController::default()),
        }
    }

    /// Get vehicle catalog reference if present
    pub fn vehicle_catalog_reference(
        &self,
    ) -> Option<
        &crate::types::catalogs::references::CatalogReference<
            crate::types::catalogs::entities::CatalogVehicle,
        >,
    > {
        match &self.entity_catalog_reference {
            Some(ScenarioEntityReference::Vehicle(r)) => Some(r),
            _ => None,
        }
    }

    /// Get pedestrian catalog reference if present
    pub fn pedestrian_catalog_reference(
        &self,
    ) -> Option<
        &crate::types::catalogs::references::CatalogReference<
            crate::types::catalogs::entities::CatalogPedestrian,
        >,
    > {
        match &self.entity_catalog_reference {
            Some(ScenarioEntityReference::Pedestrian(r)) => Some(r),
            _ => None,
        }
    }

    /// Get the entity object as an enum variant
    pub fn get_entity_object(&self) -> Option<EntityObject> {
        if let Some(vehicle) = &self.vehicle {
            Some(EntityObject::Vehicle(Box::new(vehicle.clone())))
        } else {
            self.pedestrian
                .as_ref()
                .map(|pedestrian| EntityObject::Pedestrian(Box::new(pedestrian.clone())))
        }
    }

    /// Get the name of this scenario object
    pub fn get_name(&self) -> Option<&str> {
        self.name.as_literal().map(|s| s.as_str())
    }
}

impl Entities {
    /// Create a new empty entities container
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a scenario object to the entities
    pub fn add_object(&mut self, object: ScenarioObject) {
        self.scenario_objects.push(object);
    }

    /// Find a scenario object by name
    pub fn find_object(&self, name: &str) -> Option<&ScenarioObject> {
        self.scenario_objects
            .iter()
            .find(|obj| obj.get_name() == Some(name))
    }
}

// ObjectController is now imported from crate::types::controllers

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scenario_object_creation() {
        let vehicle = Vehicle::default();
        let obj = ScenarioObject::new_vehicle("TestVehicle".to_string(), vehicle);

        assert_eq!(obj.get_name(), Some("TestVehicle"));

        assert!(obj.vehicle.is_some());
        assert!(obj.pedestrian.is_none());

        if let Some(v) = &obj.vehicle {
            assert_eq!(v.name.as_literal().unwrap(), "DefaultVehicle");
        }

        match obj.get_entity_object() {
            Some(EntityObject::Vehicle(v)) => {
                assert_eq!(v.name.as_literal().unwrap(), "DefaultVehicle");
            }
            _ => panic!("Expected vehicle"),
        }
    }

    #[test]
    fn test_entities_container() {
        let mut entities = Entities::new();

        let vehicle = Vehicle::default();
        let obj = ScenarioObject::new_vehicle("TestVehicle".to_string(), vehicle);
        entities.add_object(obj);

        assert_eq!(entities.scenario_objects.len(), 1);

        let found = entities.find_object("TestVehicle");
        assert!(found.is_some());
        assert_eq!(found.unwrap().get_name(), Some("TestVehicle"));

        let not_found = entities.find_object("NonExistent");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_entities_serialization() {
        let mut entities = Entities::new();

        let vehicle = Vehicle::default();
        let obj = ScenarioObject::new_vehicle("TestVehicle".to_string(), vehicle);
        entities.add_object(obj);

        // Test that serialization works
        let xml = quick_xml::se::to_string(&entities).unwrap();
        assert!(xml.contains("ScenarioObject"));
        assert!(xml.contains("name=\"TestVehicle\""));
    }
}
