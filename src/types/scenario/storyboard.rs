//! Storyboard and main scenario structure types

use crate::types::basic::{OSString, UnsignedShort, ParameterDeclarations};
use crate::types::entities::Entities;
use crate::types::catalogs::files::CatalogContent;
use crate::types::distributions::ParameterValueDistribution;
use crate::types::scenario::variables::VariableDeclarations;
use crate::types::scenario::monitors::MonitorDeclarations;
use serde::{Deserialize, Serialize};

/// Root OpenSCENARIO document structure supporting all document types
/// This represents the flattened XSD group structure where OpenScenarioCategory 
/// is a choice between ScenarioDefinition, CatalogDefinition, and ParameterValueDistributionDefinition groups
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "OpenSCENARIO")]
pub struct OpenScenario {
    #[serde(rename = "FileHeader")]
    pub file_header: FileHeader,
    
    // ScenarioDefinition group elements (optional - present for scenario documents)
    #[serde(
        rename = "ParameterDeclarations",
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_declarations: Option<ParameterDeclarations>,

    #[serde(
        rename = "VariableDeclarations", 
        skip_serializing_if = "Option::is_none"
    )]
    pub variable_declarations: Option<VariableDeclarations>,

    #[serde(
        rename = "MonitorDeclarations",
        skip_serializing_if = "Option::is_none"
    )]
    pub monitor_declarations: Option<MonitorDeclarations>,

    #[serde(
        rename = "CatalogLocations",
        skip_serializing_if = "Option::is_none"
    )]
    pub catalog_locations: Option<crate::types::catalogs::locations::CatalogLocations>,

    #[serde(
        rename = "RoadNetwork",
        skip_serializing_if = "Option::is_none"
    )]
    pub road_network: Option<crate::types::road::RoadNetwork>,

    #[serde(
        rename = "Entities",
        skip_serializing_if = "Option::is_none"
    )]
    pub entities: Option<Entities>,

    #[serde(
        rename = "Storyboard",
        skip_serializing_if = "Option::is_none"
    )]
    pub storyboard: Option<Storyboard>,

    // ParameterValueDistributionDefinition group elements (optional - present for parameter variation documents)
    #[serde(
        rename = "ParameterValueDistribution",
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_value_distribution: Option<ParameterValueDistribution>,

    // CatalogDefinition group elements (optional - present for catalog documents)
    #[serde(
        rename = "Catalog",
        skip_serializing_if = "Option::is_none"
    )]
    pub catalog: Option<CatalogDefinition>,
}

impl OpenScenario {
    /// Determine the document type based on which elements are present
    pub fn document_type(&self) -> OpenScenarioDocumentType {
        if self.entities.is_some() && self.storyboard.is_some() {
            OpenScenarioDocumentType::Scenario
        } else if self.parameter_value_distribution.is_some() {
            OpenScenarioDocumentType::ParameterVariation
        } else if self.catalog.is_some() {
            OpenScenarioDocumentType::Catalog
        } else {
            OpenScenarioDocumentType::Unknown
        }
    }

    /// Check if this is a scenario document
    pub fn is_scenario(&self) -> bool {
        matches!(self.document_type(), OpenScenarioDocumentType::Scenario)
    }

    /// Check if this is a parameter variation document
    pub fn is_parameter_variation(&self) -> bool {
        matches!(self.document_type(), OpenScenarioDocumentType::ParameterVariation)
    }

    /// Check if this is a catalog document
    pub fn is_catalog(&self) -> bool {
        matches!(self.document_type(), OpenScenarioDocumentType::Catalog)
    }
}

/// OpenSCENARIO document types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenScenarioDocumentType {
    /// Concrete scenario with entities and storyboard
    Scenario,
    /// Parameter variation document
    ParameterVariation,
    /// Catalog document
    Catalog,
    /// Unknown or invalid document type
    Unknown,
}

/// Scenario definition containing concrete scenario elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioDefinition {
    #[serde(
        rename = "ParameterDeclarations",
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_declarations: Option<ParameterDeclarations>,

    #[serde(
        rename = "VariableDeclarations", 
        skip_serializing_if = "Option::is_none"
    )]
    pub variable_declarations: Option<VariableDeclarations>,

    #[serde(
        rename = "MonitorDeclarations",
        skip_serializing_if = "Option::is_none"
    )]
    pub monitor_declarations: Option<MonitorDeclarations>,

    #[serde(rename = "CatalogLocations")]
    pub catalog_locations: crate::types::catalogs::locations::CatalogLocations,

    #[serde(rename = "RoadNetwork")]
    pub road_network: crate::types::road::RoadNetwork,

    #[serde(rename = "Entities")]
    pub entities: Entities,

    #[serde(rename = "Storyboard")]
    pub storyboard: Storyboard,
}

/// Catalog definition for catalog files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogDefinition {
    #[serde(rename = "Catalog")]
    pub catalog: CatalogContent,
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    /// Default creates a concrete scenario document
    fn default() -> Self {
        Self {
            file_header: FileHeader {
                author: crate::types::basic::Value::literal("Unknown".to_string()),
                date: crate::types::basic::Value::literal("1970-01-01T00:00:00".to_string()),
                description: crate::types::basic::Value::literal("".to_string()),
                rev_major: crate::types::basic::Value::literal(1),
                rev_minor: crate::types::basic::Value::literal(0),
            },
            // Scenario elements
            parameter_declarations: Some(ParameterDeclarations::default()),
            variable_declarations: None,
            monitor_declarations: None,
            catalog_locations: Some(crate::types::catalogs::locations::CatalogLocations::default()),
            road_network: Some(crate::types::road::RoadNetwork::default()),
            entities: Some(Entities::default()),
            storyboard: Some(Storyboard::default()),
            // Parameter variation elements
            parameter_value_distribution: None,
            // Catalog elements
            catalog: None,
        }
    }
}

impl Default for ScenarioDefinition {
    fn default() -> Self {
        Self {
            parameter_declarations: None,
            variable_declarations: None,
            monitor_declarations: None,
            catalog_locations: crate::types::catalogs::locations::CatalogLocations::default(),
            road_network: crate::types::road::RoadNetwork::default(),
            entities: Entities::default(),
            storyboard: Storyboard::default(),
        }
    }
}

impl Default for CatalogDefinition {
    fn default() -> Self {
        Self {
            catalog: CatalogContent::default(),
        }
    }
}


