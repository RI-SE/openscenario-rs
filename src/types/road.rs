//! Road network types for OpenSCENARIO
//!
//! This module defines types for road network definitions including
//! logic files and road network references.

use std::env::consts::OS;

use crate::types::basic::OSString;
use serde::{Deserialize, Serialize};

/// Road network definition for scenario
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoadNetwork {
    /// Logic file reference containing road network data
    #[serde(rename = "LogicFile", skip_serializing_if = "Option::is_none")]
    pub logic_file: Option<LogicFile>,

    /// Scene graph file reference (optional)
    #[serde(rename = "SceneGraphFile", skip_serializing_if = "Option::is_none")]
    pub scene_graph_file: Option<SceneGraphFile>,
}

/// Logic file containing road network definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogicFile {
    /// File path to the logic file (typically .xodr)
    #[serde(rename = "@filepath")]
    pub filepath: OSString,
}

/// Scene graph file for visual representation (optional)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SceneGraphFile {
    /// File path to the scene graph file
    #[serde(rename = "@filepath")]
    pub filepath: OSString,
}

impl RoadNetwork {
    /// Create a new road network with a logic file
    pub fn new(logic_file: LogicFile) -> Self {
        Self {
            logic_file: Some(logic_file),
            scene_graph_file: None,
        }
    }

    /// Create a road network from a filepath
    pub fn from_logic_file_path(filepath: String) -> Self {
        Self::new(LogicFile::new(filepath))
    }
}

impl LogicFile {
    /// Create a new logic file reference
    pub fn new(filepath: String) -> Self {
        Self {
            filepath: OSString::literal(filepath),
        }
    }
}

impl SceneGraphFile {
    /// Create a new scene graph file reference
    pub fn new(filepath: String) -> Self {
        Self {
            filepath: OSString::literal(filepath),
        }
    }
}

impl Default for RoadNetwork {
    fn default() -> Self {
        Self {
            logic_file: None,
            scene_graph_file: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_road_network_creation() {
        let logic_file = LogicFile::new("./road_networks/test.xodr".to_string());
        let road_network = RoadNetwork::new(logic_file);

        assert!(road_network.logic_file.is_some());
        assert_eq!(
            road_network
                .logic_file
                .unwrap()
                .filepath
                .as_literal()
                .unwrap(),
            "./road_networks/test.xodr"
        );
    }

    #[test]
    fn test_road_network_from_path() {
        let road_network = RoadNetwork::from_logic_file_path(
            "./road_networks/alks_road_different_curvatures.xodr".to_string(),
        );

        assert!(road_network.logic_file.is_some());
        assert_eq!(
            road_network
                .logic_file
                .unwrap()
                .filepath
                .as_literal()
                .unwrap(),
            "./road_networks/alks_road_different_curvatures.xodr"
        );
    }

    #[test]
    fn test_logic_file_creation() {
        let logic_file = LogicFile::new("test.xodr".to_string());
        assert_eq!(logic_file.filepath.as_literal().unwrap(), "test.xodr");
    }

    #[test]
    fn test_scene_graph_file_creation() {
        let scene_file = SceneGraphFile::new("test.osgb".to_string());
        assert_eq!(scene_file.filepath.as_literal().unwrap(), "test.osgb");
    }

    #[test]
    fn test_road_network_serialization() {
        let road_network = RoadNetwork::from_logic_file_path("test.xodr".to_string());
        let xml = quick_xml::se::to_string(&road_network).unwrap();

        assert!(xml.contains("RoadNetwork"));
        assert!(xml.contains("LogicFile"));
        assert!(xml.contains("filepath=\"test.xodr\""));
    }
}

