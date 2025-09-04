//! Entity type module for all scenario objects and participants
//!
//! This file contains:
//! - Base entity traits and common entity behaviors
//! - Entity lifecycle management and state tracking
//! - Entity reference resolution and validation
//! - Entity selection and filtering utilities
//! - Common entity properties and bounding box calculations
//!
//! Contributes to project by:
//! - Organizing 12+ entity types into logical categories
//! - Providing consistent interface for all entity types
//! - Enabling polymorphic entity handling and management
//! - Supporting entity relationships and interactions
//! - Facilitating entity validation and constraint checking

// TODO: Declare entity submodules
// TODO: pub mod vehicle;     // Vehicle entity definitions
// TODO: pub mod pedestrian;  // Pedestrian entity definitions
// TODO: pub mod misc_object; // Miscellaneous object definitions (later)

// TODO: Re-export entity types for MVP (Week 3)
// TODO: pub use vehicle::Vehicle;
// TODO: pub use pedestrian::Pedestrian;

// TODO: Define ScenarioObject wrapper for all entity types
// TODO: #[derive(Debug, Clone, Serialize, Deserialize)]
// TODO: pub struct ScenarioObject { 
//   #[serde(rename = "@name")] pub name: String,
//   #[serde(flatten)] pub entity: EntityObject 
// }

// TODO: Define EntityObject enum for polymorphic entities
// TODO: #[derive(Debug, Clone, Serialize, Deserialize)]
// TODO: #[serde(tag = "type")]
// TODO: pub enum EntityObject { Vehicle(Vehicle), Pedestrian(Pedestrian) }

// TODO: Define Entities collection container
// TODO: #[derive(Debug, Clone, Serialize, Deserialize)]
// TODO: pub struct Entities { 
//   #[serde(rename = "ScenarioObject", default)] 
//   pub scenario_objects: Vec<ScenarioObject> 
// }

// TODO: Define EntityRef for entity references
// TODO: #[derive(Debug, Clone, Serialize, Deserialize)]
// TODO: pub struct EntityRef { #[serde(rename = "@entityRef")] pub entity_ref: String }