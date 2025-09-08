//! Entity definitions for OpenSCENARIO scenarios

use serde::{Deserialize, Serialize};
use crate::types::basic::OSString;

pub mod vehicle;
pub mod pedestrian;

// Re-export entity types
pub use vehicle::{Vehicle, Properties};
pub use pedestrian::Pedestrian;

/// Union type for all entity objects
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum EntityObject {
    /// Vehicle entity
    Vehicle(Vehicle),
    /// Pedestrian entity  
    Pedestrian(Pedestrian),
    // TODO: Add MiscellaneousObject later
    // MiscellaneousObject(MiscObject),
}

/// Wrapper for scenario objects containing entity information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScenarioObject {
    /// Name of the scenario object (used for references)
    #[serde(rename = "@name")]
    pub name: OSString,
    
    /// The actual entity object (Vehicle, Pedestrian, etc.)
    #[serde(flatten)]
    pub entity_object: EntityObject,
    
    /// Object controller configuration (optional)
    #[serde(rename = "ObjectController")]
    pub object_controller: ObjectController,
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
            entity_object: EntityObject::Vehicle(vehicle),
            object_controller: ObjectController::default(),
        }
    }
    
    /// Create a new scenario object with a pedestrian
    pub fn new_pedestrian(name: String, pedestrian: Pedestrian) -> Self {
        Self {
            name: crate::types::basic::Value::literal(name),
            entity_object: EntityObject::Pedestrian(pedestrian),
            object_controller: ObjectController::default(),
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
        self.scenario_objects.iter()
            .find(|obj| obj.get_name() == Some(name))
    }
}

/// Object controller for controlling entity behavior
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectController {
    /// Optional controller properties
    #[serde(rename = "Properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Properties>,
}



impl Default for ObjectController {
    fn default() -> Self {
        Self {
            properties: None,
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scenario_object_creation() {
        let vehicle = Vehicle::default();
        let obj = ScenarioObject::new_vehicle("TestVehicle".to_string(), vehicle);
        
        assert_eq!(obj.get_name(), Some("TestVehicle"));
        
        match &obj.entity_object {
            EntityObject::Vehicle(v) => {
                assert_eq!(v.name.as_literal().unwrap(), "DefaultVehicle");
            },
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