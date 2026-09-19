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
    ByObjectType, ByType, EntityDistribution, EntityDistributionEntry, EntitySelection,
    ExternalObjectReference, ScenarioObjectTemplate, SelectedEntities,
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

/// A `<CatalogReference>` standing in for an entity in the `EntityObject` choice group.
///
/// XSD `EntityObject` (`:1168-1176`) offers a single `<CatalogReference>` element, and
/// XSD `CatalogReference` (`:879-885`) carries only `catalogName`, `entryName` and an
/// optional `ParameterAssignments`. The document therefore does not say whether the
/// entry it names is a vehicle, a pedestrian or a miscellaneous object. That is knowable
/// only once the referenced catalog file is read. Hence this reference carries no entity
/// type, and the kind is settled at catalog-resolution time rather than guessed while parsing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "CatalogReference")]
pub struct EntityCatalogReference {
    /// Name of the catalog file holding the entry
    #[serde(rename = "@catalogName")]
    pub catalog_name: OSString,

    /// Name of the entry within that catalog
    #[serde(rename = "@entryName")]
    pub entry_name: OSString,

    /// Parameter assignments applied when the entry is resolved
    #[serde(
        rename = "ParameterAssignments",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_assignments: Option<crate::types::catalogs::references::ParameterAssignments>,
}

impl EntityCatalogReference {
    /// Create a reference to an entry in a catalog, without parameter assignments.
    pub fn new(catalog_name: impl Into<String>, entry_name: impl Into<String>) -> Self {
        Self {
            catalog_name: OSString::literal(catalog_name.into()),
            entry_name: OSString::literal(entry_name.into()),
            parameter_assignments: None,
        }
    }
}

/// The XSD `EntityObject` group (`:1168-1176`): a bare `xsd:choice` of
/// `CatalogReference | Vehicle | Pedestrian | MiscObject |
/// ExternalObjectReference`. Neither the group reference in `ScenarioObject`
/// nor in `ScenarioObjectTemplate` carries `minOccurs="0"`, so exactly one
/// branch is required in both.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum EntityObjectChoice {
    /// Reference to an entry in a catalog
    CatalogReference(EntityCatalogReference),
    /// Vehicle entity
    Vehicle(Vehicle),
    /// Pedestrian entity
    Pedestrian(Pedestrian),
    /// Miscellaneous object entity
    MiscObject(MiscObject),
    /// External object reference
    ExternalObjectReference(ExternalObjectReference),
}

/// Wrapper for scenario objects containing entity information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScenarioObject {
    /// Name of scenario object (used for references)
    #[serde(rename = "@name")]
    pub name: OSString,

    /// The entity definition, exactly one of the `EntityObject` group's branches.
    #[serde(rename = "$value")]
    pub entity: EntityObjectChoice,

    /// Object controller configuration (optional, may occur multiple times)
    #[serde(
        rename = "ObjectController",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub object_controller: Vec<ObjectController>,
}

/// Container for all entities in the scenario
///
/// XSD `Entities` (`:1122-1127`): sequence of `ScenarioObject`
/// (`minOccurs="0" maxOccurs="unbounded"`) followed by `EntitySelection`
/// (`minOccurs="0" maxOccurs="unbounded"`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Entities {
    /// List of scenario objects
    #[serde(rename = "ScenarioObject", default)]
    pub scenario_objects: Vec<ScenarioObject>,

    /// List of entity selections
    #[serde(
        rename = "EntitySelection",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub entity_selections: Vec<EntitySelection>,
}

impl ScenarioObject {
    /// Create a new scenario object with a vehicle
    pub fn new_vehicle(name: String, vehicle: Vehicle) -> Self {
        Self {
            name: crate::types::basic::Value::literal(name),
            entity: EntityObjectChoice::Vehicle(vehicle),
            object_controller: Vec::new(),
        }
    }

    /// Create a new scenario object with a pedestrian
    pub fn new_pedestrian(name: String, pedestrian: Pedestrian) -> Self {
        Self {
            name: crate::types::basic::Value::literal(name),
            entity: EntityObjectChoice::Pedestrian(pedestrian),
            object_controller: Vec::new(),
        }
    }

    /// Create a new scenario object with a miscellaneous object
    pub fn new_misc_object(name: String, misc_object: MiscObject) -> Self {
        Self {
            name: crate::types::basic::Value::literal(name),
            entity: EntityObjectChoice::MiscObject(misc_object),
            object_controller: Vec::new(),
        }
    }

    /// Create a new scenario object referencing a catalog entry
    ///
    /// The document does not record which kind of entity the entry describes, so this
    /// constructor takes no entity type.
    pub fn new_catalog_reference(name: String, catalog_reference: EntityCatalogReference) -> Self {
        Self {
            name: crate::types::basic::Value::literal(name),
            entity: EntityObjectChoice::CatalogReference(catalog_reference),
            object_controller: Vec::new(),
        }
    }

    /// The vehicle, if this object's entity branch is `Vehicle`
    pub fn vehicle(&self) -> Option<&Vehicle> {
        match &self.entity {
            EntityObjectChoice::Vehicle(v) => Some(v),
            _ => None,
        }
    }

    /// A mutable view of the vehicle, if this object's entity branch is `Vehicle`
    pub fn vehicle_mut(&mut self) -> Option<&mut Vehicle> {
        match &mut self.entity {
            EntityObjectChoice::Vehicle(v) => Some(v),
            _ => None,
        }
    }

    /// The pedestrian, if this object's entity branch is `Pedestrian`
    pub fn pedestrian(&self) -> Option<&Pedestrian> {
        match &self.entity {
            EntityObjectChoice::Pedestrian(p) => Some(p),
            _ => None,
        }
    }

    /// A mutable view of the pedestrian, if this object's entity branch is `Pedestrian`
    pub fn pedestrian_mut(&mut self) -> Option<&mut Pedestrian> {
        match &mut self.entity {
            EntityObjectChoice::Pedestrian(p) => Some(p),
            _ => None,
        }
    }

    /// The miscellaneous object, if this object's entity branch is `MiscObject`
    pub fn misc_object(&self) -> Option<&MiscObject> {
        match &self.entity {
            EntityObjectChoice::MiscObject(m) => Some(m),
            _ => None,
        }
    }

    /// The external object reference, if this object's entity branch is `ExternalObjectReference`
    pub fn external_object_reference(&self) -> Option<&ExternalObjectReference> {
        match &self.entity {
            EntityObjectChoice::ExternalObjectReference(r) => Some(r),
            _ => None,
        }
    }

    /// Get the catalog reference if this object is defined by one
    pub fn catalog_reference(&self) -> Option<&EntityCatalogReference> {
        match &self.entity {
            EntityObjectChoice::CatalogReference(r) => Some(r),
            _ => None,
        }
    }

    /// Get the entity object as an enum variant
    pub fn get_entity_object(&self) -> Option<EntityObject> {
        match &self.entity {
            EntityObjectChoice::Vehicle(v) => Some(EntityObject::Vehicle(Box::new(v.clone()))),
            EntityObjectChoice::Pedestrian(p) => {
                Some(EntityObject::Pedestrian(Box::new(p.clone())))
            }
            EntityObjectChoice::MiscObject(m) => {
                Some(EntityObject::MiscObject(Box::new(m.clone())))
            }
            EntityObjectChoice::CatalogReference(_)
            | EntityObjectChoice::ExternalObjectReference(_) => None,
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
        // Vehicle::new_car requires the (inner) vehicle name explicitly
        // (`Vehicle` no longer has a fabricating `Default`), which is deliberately different
        // from the outer `ScenarioObject`'s own name to prove they are independent fields.
        let vehicle = Vehicle::new_car("InnerVehicle".to_string());
        let obj = ScenarioObject::new_vehicle("TestVehicle".to_string(), vehicle);

        assert_eq!(obj.get_name(), Some("TestVehicle"));

        assert!(obj.vehicle().is_some());
        assert!(obj.pedestrian().is_none());

        if let Some(v) = obj.vehicle() {
            assert_eq!(v.name.as_literal().unwrap(), "InnerVehicle");
        }

        match obj.get_entity_object() {
            Some(EntityObject::Vehicle(v)) => {
                assert_eq!(v.name.as_literal().unwrap(), "InnerVehicle");
            }
            _ => panic!("Expected vehicle"),
        }
    }

    #[test]
    fn test_entities_container() {
        let mut entities = Entities::new();

        let vehicle = Vehicle::new_car("TestVehicle".to_string());
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

        assert!(obj.misc_object().is_some());
        assert!(obj.vehicle().is_none());
        assert!(obj.pedestrian().is_none());

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
        assert!(deserialized.misc_object().is_some());
        assert_eq!(
            deserialized
                .misc_object()
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

        let mut obj = ScenarioObject::new_vehicle(
            "TestVehicle".to_string(),
            Vehicle::new_car("TestVehicle".to_string()),
        );

        obj.object_controller
            .push(ObjectController::with_controller(Controller::new(
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
        assert!(deserialized.object_controller[0].controller().is_some());
        assert_eq!(
            deserialized.object_controller[0]
                .controller()
                .unwrap()
                .name
                .as_literal()
                .unwrap(),
            "InlineController"
        );
        assert!(deserialized.object_controller[1]
            .catalog_reference()
            .is_some());
        assert_eq!(
            deserialized.object_controller[1]
                .catalog_reference()
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

        let vehicle = Vehicle::new_car("TestVehicle".to_string());
        let obj = ScenarioObject::new_vehicle("TestVehicle".to_string(), vehicle);
        entities.add_object(obj);

        // Test that serialization works
        let xml = quick_xml::se::to_string(&entities).unwrap();
        assert!(xml.contains("ScenarioObject"));
        assert!(xml.contains("name=\"TestVehicle\""));
    }

    // `ScenarioObject`'s `EntityObject` group (XSD:1168-1176, referenced with
    // no `minOccurs` from XSD:2000-2005) is a required bare choice: neither
    // zero nor two branches is constructible.

    #[test]
    fn test_scenario_object_zero_branches_rejected() {
        let xml = r#"<ScenarioObject name="Ego"/>"#;
        let result: Result<ScenarioObject, _> = quick_xml::de::from_str(xml);
        assert!(
            result.is_err(),
            "ScenarioObject with no EntityObject branch must be rejected"
        );
    }

    #[test]
    fn test_scenario_object_two_branches_rejected() {
        let xml = r#"<ScenarioObject name="Ego"><Vehicle name="car" vehicleCategory="car"><BoundingBox><Center x="0" y="0" z="0"/><Dimensions width="2.0" length="4.5" height="1.8"/></BoundingBox><Performance maxSpeed="10" maxAcceleration="1" maxDeceleration="1"/><Axles><FrontAxle maxSteering="0.5" wheelDiameter="0.6" trackWidth="1.8" positionX="3.1" positionZ="0.3"/><RearAxle maxSteering="0.0" wheelDiameter="0.6" trackWidth="1.8" positionX="0.0" positionZ="0.3"/></Axles></Vehicle><CatalogReference catalogName="Cat" entryName="Entry"/></ScenarioObject>"#;
        let result: Result<ScenarioObject, _> = quick_xml::de::from_str(xml);
        assert!(
            result.is_err(),
            "Vehicle beside CatalogReference is two branches and must be rejected"
        );
    }
}
