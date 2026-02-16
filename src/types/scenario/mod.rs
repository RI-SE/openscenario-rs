//! Scenario structure types

pub mod init;
pub mod monitors;
pub mod story;
pub mod storyboard;
pub mod triggers;
pub mod variables;

// Re-export main types for convenience
pub use init::{
    Actions, EnvironmentAction, GlobalAction, Init, LongitudinalAction, LongitudinalActionType,
    Private, PrivateAction,
};
pub use monitors::{MonitorDeclaration, MonitorDeclarations};
pub use story::{Act, Actors, EntityRef, Event, Maneuver, ManeuverGroup, ScenarioStory};
pub use storyboard::{FileHeader, OpenScenario, Storyboard};
pub use variables::{VariableDeclaration, VariableDeclarations};

// Import necessary types for ScenarioDefinition group
use crate::types::basic::ParameterDeclarations;
use crate::types::catalogs::locations::CatalogLocations;
use crate::types::entities::Entities;
use crate::types::road::RoadNetwork;
use serde::{Deserialize, Serialize};

/// ScenarioDefinition group - XSD group wrapper for scenario sequence
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[derive(Default)]
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
    pub catalog_locations: CatalogLocations,
    #[serde(rename = "RoadNetwork")]
    pub road_network: RoadNetwork,
    #[serde(rename = "Entities")]
    pub entities: Entities,
    #[serde(rename = "Storyboard")]
    pub storyboard: Storyboard,
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

    /// Convert from existing OpenScenario structure (only works for scenario definitions)
    pub fn from_open_scenario(scenario: &OpenScenario) -> Option<Self> {
        if scenario.is_scenario() {
            Some(Self {
                parameter_declarations: scenario.parameter_declarations.clone(),
                variable_declarations: scenario.variable_declarations.clone(),
                monitor_declarations: scenario.monitor_declarations.clone(),
                catalog_locations: scenario.catalog_locations.clone().unwrap_or_default(),
                road_network: scenario.road_network.clone().unwrap_or_default(),
                entities: scenario.entities.clone().unwrap_or_default(),
                storyboard: scenario.storyboard.clone().unwrap_or_default(),
            })
        } else {
            None
        }
    }
}

// Additional export for the group type (already available in scope)
