//! Storyboard and main scenario structure types

use serde::{Deserialize, Serialize};
use crate::types::basic::{OSString, UnsignedShort};
use crate::types::entities::Entities;

/// Root OpenSCENARIO document structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "OpenSCENARIO")]
pub struct OpenScenario {
    #[serde(rename = "FileHeader")]
    pub file_header: FileHeader,
    
    #[serde(rename = "ParameterDeclarations", skip_serializing_if = "Option::is_none")]
    pub parameter_declarations: Option<crate::types::basic::ParameterDeclarations>,
    
    #[serde(rename = "CatalogLocations", skip_serializing_if = "Option::is_none")]
    pub catalog_locations: Option<crate::types::catalogs::locations::CatalogLocations>,
    
    #[serde(rename = "RoadNetwork", skip_serializing_if = "Option::is_none")]
    pub road_network: Option<crate::types::road::RoadNetwork>,
    
    #[serde(rename = "Entities", default)]
    pub entities: Entities,
    
    #[serde(rename = "Storyboard")]
    pub storyboard: Storyboard,
}

/// File header with scenario metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileHeader {
    #[serde(rename = "@author")]
    pub author: OSString,
    #[serde(rename = "@date")]
    pub date: OSString, // Simplified for MVP, will use DateTime later
    #[serde(rename = "@description")]
    pub description: OSString,
    #[serde(rename = "@revMajor")]
    pub rev_major: UnsignedShort,
    #[serde(rename = "@revMinor")]
    pub rev_minor: UnsignedShort,
}

// Entities is now imported from entities module

/// Storyboard structure (simplified for MVP)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Storyboard {
    #[serde(rename = "Init")]
    pub init: Init,
    #[serde(rename = "Story", default)]
    pub stories: Vec<super::story::ScenarioStory>,
    #[serde(rename = "StopTrigger", skip_serializing_if = "Option::is_none")]
    pub stop_trigger: Option<super::triggers::Trigger>,
}

// Init is now imported from init.rs module
pub use super::init::Init;

// Story is now imported from story.rs module

impl Default for Storyboard {
    fn default() -> Self {
        Self {
            init: Init::default(),
            stories: Vec::new(),
            stop_trigger: None,
        }
    }
}

impl Default for OpenScenario {
    fn default() -> Self {
        Self {
            file_header: FileHeader {
                author: crate::types::basic::Value::literal("Unknown".to_string()),
                date: crate::types::basic::Value::literal("1970-01-01T00:00:00".to_string()),
                description: crate::types::basic::Value::literal("".to_string()),
                rev_major: crate::types::basic::Value::literal(1),
                rev_minor: crate::types::basic::Value::literal(0),
            },
            parameter_declarations: None,
            catalog_locations: None,
            road_network: None,
            entities: Entities::default(),
            storyboard: Storyboard::default(),
        }
    }
}