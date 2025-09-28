//! Core scenario builder for programmatic scenario construction

use crate::types::{
    basic::{OSString, UnsignedShort, ParameterDeclarations, ParameterDeclaration},
    scenario::storyboard::{OpenScenario, FileHeader, Storyboard},
    entities::Entities,
    catalogs::locations::CatalogLocations,
    road::RoadNetwork,
    enums::ParameterType,
};
use super::{BuilderError, BuilderResult};
use std::marker::PhantomData;

// Type states for compile-time safety
#[derive(Debug)]
pub struct Empty;

#[derive(Debug)] 
pub struct HasHeader;

#[derive(Debug)]
pub struct HasEntities;

#[derive(Debug)]
pub struct Complete;

/// Type-safe scenario builder with compile-time state validation
pub struct ScenarioBuilder<S> {
    _state: PhantomData<S>,
    pub(crate) data: PartialScenarioData,
}

#[derive(Debug, Default)]
pub(crate) struct PartialScenarioData {
    pub(crate) file_header: Option<FileHeader>,
    pub(crate) parameter_declarations: Option<ParameterDeclarations>,
    pub(crate) catalog_locations: Option<CatalogLocations>,
    pub(crate) road_network: Option<RoadNetwork>, 
    pub(crate) entities: Option<Entities>,
    pub(crate) storyboard: Option<Storyboard>,
}

// Implementation for Empty state (starting point)
impl ScenarioBuilder<Empty> {
    /// Create a new scenario builder
    pub fn new() -> Self {
        Self {
            _state: PhantomData,
            data: PartialScenarioData::default(),
        }
    }
    
    /// Set file header and progress to HasHeader state
    pub fn with_header(mut self, description: &str, author: &str) -> ScenarioBuilder<HasHeader> {
        #[cfg(feature = "chrono")]
        let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();
        #[cfg(not(feature = "chrono"))]
        let now = "2024-01-01T00:00:00".to_string();
        
        self.data.file_header = Some(FileHeader {
            rev_major: UnsignedShort::literal(1),
            rev_minor: UnsignedShort::literal(0), 
            date: OSString::literal(now),
            description: OSString::literal(description.to_string()),
            author: OSString::literal(author.to_string()),
        });
        
        ScenarioBuilder {
            _state: PhantomData,
            data: self.data,
        }
    }
}

// Implementation for HasHeader state
impl ScenarioBuilder<HasHeader> {
    /// Add parameter declarations
    pub fn with_parameters(mut self, params: ParameterDeclarations) -> Self {
        self.data.parameter_declarations = Some(params);
        self
    }
    
    /// Add a single parameter (convenience method)
    pub fn add_parameter(mut self, name: &str, param_type: ParameterType, value: &str) -> Self {
        let mut params = self.data.parameter_declarations.take().unwrap_or_default();
        
        params.parameter_declarations.push(ParameterDeclaration {
            name: OSString::literal(name.to_string()),
            parameter_type,
            value: OSString::literal(value.to_string()),
            constraint_groups: Vec::new(),
        });
        
        self.data.parameter_declarations = Some(params);
        self
    }
    
    /// Add catalog locations (optional)
    pub fn with_catalog_locations(mut self, locations: CatalogLocations) -> Self {
        self.data.catalog_locations = Some(locations);
        self
    }
    
    /// Add road network (optional for minimal scenarios)
    pub fn with_road_network(mut self, network: RoadNetwork) -> Self {
        self.data.road_network = Some(network);
        self
    }
    
    /// Set road network from OpenDRIVE file
    pub fn with_road_file(mut self, file_path: &str) -> Self {
        self.data.road_network = Some(RoadNetwork {
            logic_file: Some(crate::types::road::LogicFile {
                filepath: OSString::literal(file_path.to_string()),
            }),
            scene_graph_file: None,
        });
        self
    }
    
    /// Initialize entities and progress to HasEntities state
    pub fn with_entities(mut self) -> ScenarioBuilder<HasEntities> {
        self.data.entities = Some(Entities::new());
        
        ScenarioBuilder {
            _state: PhantomData,
            data: self.data,
        }
    }
}

// Implementation for HasEntities state  
impl ScenarioBuilder<HasEntities> {
    /// Add a vehicle entity
    pub fn add_vehicle(&mut self, name: &str) -> crate::builder::entities::VehicleBuilder<'_> {
        crate::builder::entities::VehicleBuilder::new(self, name)
    }
    
    /// Build the final OpenScenario document
    pub fn build(self) -> BuilderResult<OpenScenario> {
        let file_header = self.data.file_header
            .ok_or_else(|| BuilderError::missing_field("file_header", ".with_header()"))?;
            
        let entities = self.data.entities
            .ok_or_else(|| BuilderError::missing_field("entities", ".with_entities()"))?;
        
        // Use defaults for optional fields
        let storyboard = self.data.storyboard.unwrap_or_else(|| Storyboard::default());
        
        Ok(OpenScenario {
            file_header,
            parameter_declarations: self.data.parameter_declarations,
            variable_declarations: None, 
            monitor_declarations: None,
            catalog_locations: self.data.catalog_locations,
            road_network: self.data.road_network,
            entities: Some(entities),
            storyboard: Some(storyboard),
            parameter_value_distribution: None,
            catalog: None,
        })
    }
}

impl Default for ScenarioBuilder<Empty> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_minimal_scenario_builder() {
        let scenario = ScenarioBuilder::new()
            .with_header("Test Scenario", "Test Author")
            .with_entities()
            .build()
            .unwrap();
            
        // Verify basic structure
        if let crate::types::basic::Value::Literal(desc) = &scenario.file_header.description {
            assert_eq!(desc, "Test Scenario");
        } else {
            panic!("Description should be literal");
        }
        
        assert!(scenario.entities.is_some());
        assert!(scenario.storyboard.is_some());
    }
}