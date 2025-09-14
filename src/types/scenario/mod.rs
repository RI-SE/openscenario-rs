//! Scenario structure types

pub mod init;
pub mod storyboard;
pub mod story;
pub mod triggers;
pub mod variables;
pub mod monitors;

// Re-export main types for convenience
pub use init::{Init, Actions, GlobalAction, EnvironmentAction, Private, PrivateActionWrapper, PrivateActionType, LongitudinalAction, LongitudinalActionType};
pub use storyboard::{OpenScenario, FileHeader, Storyboard};
pub use story::{
    ScenarioStory, Act, ManeuverGroup, Maneuver, Event, Actors, EntityRef,
};
pub use variables::{VariableDeclarations, VariableDeclaration};
pub use monitors::{MonitorDeclarations, MonitorDeclaration};

// Import necessary types for ScenarioDefinition group
use crate::types::basic::ParameterDeclarations;
use crate::types::catalogs::locations::CatalogLocations;
use crate::types::entities::Entities;
use crate::types::road::RoadNetwork;
use serde::{Deserialize, Serialize};

/// ScenarioDefinition group - XSD group wrapper for scenario sequence
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScenarioDefinition {
    #[serde(rename = "ParameterDeclarations", skip_serializing_if = "Option::is_none")]
    pub parameter_declarations: Option<ParameterDeclarations>,
    #[serde(rename = "VariableDeclarations", skip_serializing_if = "Option::is_none")]  
    pub variable_declarations: Option<VariableDeclarations>,
    #[serde(rename = "MonitorDeclarations", skip_serializing_if = "Option::is_none")]
    pub monitor_declarations: Option<MonitorDeclarations>,
    #[serde(rename = "CatalogLocations")]
    pub catalog_locations: CatalogLocations,
    #[serde(rename = "RoadNetwork")]
    pub road_network: RoadNetwork,
    #[serde(rename = "Entities")]
    pub entities: Entities,
    #[serde(rename = "Storyboard")]
    pub storyboard: Storyboard,
}

impl Default for ScenarioDefinition {
    fn default() -> Self {
        Self {
            parameter_declarations: None,
            variable_declarations: None,
            monitor_declarations: None,
            catalog_locations: CatalogLocations::default(),
            road_network: RoadNetwork::default(),
            entities: Entities::default(),
            storyboard: Storyboard::default(),
        }
    }
}

impl ScenarioDefinition {
    /// Create new scenario definition with required elements
    pub fn new(
        catalog_locations: CatalogLocations,
        road_network: RoadNetwork,
        entities: Entities,
        storyboard: Storyboard,
    ) -> Self {
        Self {
            parameter_declarations: None,
            variable_declarations: None,
            monitor_declarations: None,
            catalog_locations,
            road_network,
            entities,
            storyboard,
        }
    }

    /// Create scenario definition with parameters
    pub fn with_parameters(mut self, parameters: ParameterDeclarations) -> Self {
        self.parameter_declarations = Some(parameters);
        self
    }

    /// Create scenario definition with variables
    pub fn with_variables(mut self, variables: VariableDeclarations) -> Self {
        self.variable_declarations = Some(variables);
        self
    }

    /// Create scenario definition with monitors
    pub fn with_monitors(mut self, monitors: MonitorDeclarations) -> Self {
        self.monitor_declarations = Some(monitors);
        self
    }

    /// Convert from existing OpenScenario structure
    pub fn from_open_scenario(scenario: &OpenScenario) -> Option<Self> {
        Some(Self {
            parameter_declarations: scenario.parameter_declarations.clone(),
            variable_declarations: None, // OpenScenario doesn't have this yet
            monitor_declarations: None,  // OpenScenario doesn't have this yet
            catalog_locations: scenario.catalog_locations.clone()?,
            road_network: scenario.road_network.clone()?,
            entities: scenario.entities.clone(),
            storyboard: scenario.storyboard.clone(),
        })
    }
}

// Additional export for the group type (already available in scope)