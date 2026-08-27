//! Entity definitions for OpenSCENARIO scenarios

use crate::types::basic::OSString;
use crate::types::controllers::ObjectController;
use serde::{Deserialize, Serialize};

pub mod axles;
pub mod misc_object;
pub mod pedestrian;
pub mod selection;
pub mod vehicle;

// Re-export entity types
pub use axles::{Axle, Axles};
pub use misc_object::MiscObject;
pub use pedestrian::Pedestrian;
pub use selection::{
    ByName, ByObjectType, ByType, EntityDistribution, EntityDistributionEntry, EntitySelection,
    ExternalObjectReference, ScenarioObjectTemplate, SelectedEntities, TemplateProperties,
    TemplateProperty,
};
pub use vehicle::{Properties, Trailer, Vehicle};

/// Union type for all entity objects
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum EntityObject {
    /// Vehicle entity
    Vehicle(Box<Vehicle>),
    /// Pedestrian entity
    Pedestrian(Box<Pedestrian>),
    /// Miscellaneous object entity
    MiscObject(Box<MiscObject>),
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

    /// Miscellaneous object entity (optional)
    #[serde(
        rename = "MiscObject",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub misc_object: Option<MiscObject>,

    /// External object reference (optional)
    #[serde(
        rename = "ExternalObjectReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub external_object_reference: Option<ExternalObjectReference>,

    /// Entity catalog reference (vehicle or pedestrian)
    ///
    /// References a vehicle or pedestrian from an external catalog.
    /// Mutually exclusive with direct vehicle/pedestrian definitions.
    #[serde(rename = "CatalogReference", skip_serializing_if = "Option::is_none")]
    pub entity_catalog_reference: Option<ScenarioEntityReference>,

    /// Object controller configuration (optional, may occur multiple times)
    #[serde(
        rename = "ObjectController",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub object_controller: Vec<ObjectController>,
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
            misc_object: None,
            external_object_reference: None,
            entity_catalog_reference: None,
            object_controller: Vec::new(),
        }
    }

    /// Create a new scenario object with a pedestrian
    pub fn new_pedestrian(name: String, pedestrian: Pedestrian) -> Self {
        Self {
            name: crate::types::basic::Value::literal(name),
            vehicle: None,
            pedestrian: Some(pedestrian),
            misc_object: None,
            external_object_reference: None,
            entity_catalog_reference: None,
            object_controller: Vec::new(),
        }
    }

    /// Create a new scenario object with a miscellaneous object
    pub fn new_misc_object(name: String, misc_object: MiscObject) -> Self {
        Self {
            name: crate::types::basic::Value::literal(name),
            vehicle: None,
            pedestrian: None,
            misc_object: Some(misc_object),
            external_object_reference: None,
            entity_catalog_reference: None,
            object_controller: Vec::new(),
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
            misc_object: None,
            external_object_reference: None,
            entity_catalog_reference: Some(ScenarioEntityReference::Vehicle(catalog_reference)),
            object_controller: Vec::new(),
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
            misc_object: None,
            external_object_reference: None,
            entity_catalog_reference: Some(ScenarioEntityReference::Pedestrian(catalog_reference)),
            object_controller: Vec::new(),
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
        } else if let Some(pedestrian) = &self.pedestrian {
            Some(EntityObject::Pedestrian(Box::new(pedestrian.clone())))
        } else {
            self.misc_object
                .as_ref()
                .map(|misc_object| EntityObject::MiscObject(Box::new(misc_object.clone())))
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
    fn test_scenario_object_misc_object_roundtrip() {
        let misc = MiscObject::new(
            "Barrier1".to_string(),
            100.0,
            crate::types::enums::MiscObjectCategory::Barrier,
        );
        let obj = ScenarioObject::new_misc_object("Barrier1".to_string(), misc);

        assert!(obj.misc_object.is_some());
        assert!(obj.vehicle.is_none());
        assert!(obj.pedestrian.is_none());

        match obj.get_entity_object() {
            Some(EntityObject::MiscObject(m)) => {
                assert_eq!(m.name.as_literal().unwrap(), "Barrier1");
            }
            _ => panic!("Expected misc object"),
        }

        let xml = quick_xml::se::to_string(&obj).unwrap();
        assert!(xml.contains("MiscObject"));
        assert!(xml.contains("miscObjectCategory=\"barrier\""));

        let deserialized: ScenarioObject = quick_xml::de::from_str(&xml).unwrap();
        assert!(deserialized.misc_object.is_some());
        assert_eq!(
            deserialized
                .misc_object
                .unwrap()
                .name
                .as_literal()
                .unwrap(),
            "Barrier1"
        );
    }

    #[test]
    fn test_scenario_object_multiple_object_controllers_roundtrip() {
        use crate::types::catalogs::references::ControllerCatalogReference;
        use crate::types::controllers::Controller;
        use crate::types::enums::ControllerType;

        let mut obj =
            ScenarioObject::new_vehicle("TestVehicle".to_string(), Vehicle::default());

        obj.object_controller.push(ObjectController::with_controller(Controller::new(
            "InlineController".to_string(),
            ControllerType::Movement,
        )));
        obj.object_controller
            .push(ObjectController::with_catalog_reference(
                ControllerCatalogReference::new(
                    "ControllerCatalog".to_string(),
                    "CatalogedController".to_string(),
                ),
            ));

        assert_eq!(obj.object_controller.len(), 2);

        let xml = quick_xml::se::to_string(&obj).unwrap();
        assert_eq!(xml.matches("<ObjectController").count(), 2);

        let deserialized: ScenarioObject = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(deserialized.object_controller.len(), 2);
        assert!(deserialized.object_controller[0].controller.is_some());
        assert_eq!(
            deserialized.object_controller[0]
                .controller
                .as_ref()
                .unwrap()
                .name
                .as_literal()
                .unwrap(),
            "InlineController"
        );
        assert!(deserialized.object_controller[1]
            .catalog_reference
            .is_some());
        assert_eq!(
            deserialized.object_controller[1]
                .catalog_reference
                .as_ref()
                .unwrap()
                .entry_name
                .as_literal()
                .unwrap(),
            "CatalogedController"
        );
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
