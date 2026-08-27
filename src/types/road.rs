//! Road network types for OpenSCENARIO
//!
//! This module defines types for road network definitions including
//! logic files and road network references.

use crate::types::actions::traffic::TrafficSignalController;
use crate::types::basic::OSString;
use crate::types::positions::Position;
use serde::{Deserialize, Serialize};

/// Road network definition for scenario
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub struct RoadNetwork {
    /// Logic file reference containing road network data
    #[serde(rename = "LogicFile", skip_serializing_if = "Option::is_none")]
    pub logic_file: Option<LogicFile>,

    /// Scene graph file reference (optional)
    #[serde(rename = "SceneGraphFile", skip_serializing_if = "Option::is_none")]
    pub scene_graph_file: Option<SceneGraphFile>,

    /// Traffic signal controllers defined for this road network (optional)
    #[serde(
        rename = "TrafficSignals",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub traffic_signals: Option<TrafficSignals>,

    /// Region of the road network actually used by the scenario (optional)
    #[serde(rename = "UsedArea", default, skip_serializing_if = "Option::is_none")]
    pub used_area: Option<UsedArea>,
}

/// Wrapper for the set of traffic signal controllers on the road network
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TrafficSignals {
    #[serde(rename = "TrafficSignalController", default)]
    pub traffic_signal_controller: Vec<TrafficSignalController>,
}

/// Region of the road network actually used by the scenario
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UsedArea {
    #[serde(rename = "Position", default)]
    pub position: Vec<Position>,
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
            traffic_signals: None,
            used_area: None,
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_road_network_creation() {
        let logic_file = LogicFile::new("./road_networks/test.xodr".to_string());
        let road_network = RoadNetwork::new(logic_file);

        assert!(road_network.logic_file.is_some());
        assert_eq!(
            road_network.logic_file.unwrap().filepath.as_literal(),
            Some(&"./road_networks/test.xodr".to_string())
        );
    }

    #[test]
    fn test_road_network_from_path() {
        let road_network = RoadNetwork::from_logic_file_path(
            "./road_networks/alks_road_different_curvatures.xodr".to_string(),
        );

        assert!(road_network.logic_file.is_some());
        assert_eq!(
            road_network.logic_file.unwrap().filepath.as_literal(),
            Some(&"./road_networks/alks_road_different_curvatures.xodr".to_string())
        );
    }

    #[test]
    fn test_logic_file_creation() {
        let logic_file = LogicFile::new("test.xodr".to_string());
        assert_eq!(
            logic_file.filepath.as_literal(),
            Some(&"test.xodr".to_string())
        );
    }

    #[test]
    fn test_scene_graph_file_creation() {
        let scene_file = SceneGraphFile::new("test.osgb".to_string());
        assert_eq!(
            scene_file.filepath.as_literal(),
            Some(&"test.osgb".to_string())
        );
    }

    #[test]
    fn test_road_network_serialization() {
        let road_network = RoadNetwork::from_logic_file_path("test.xodr".to_string());
        let xml = quick_xml::se::to_string(&road_network).unwrap();

        assert!(xml.contains("RoadNetwork"));
        assert!(xml.contains("LogicFile"));
        assert!(xml.contains("filepath=\"test.xodr\""));
    }

    #[test]
    fn test_road_network_traffic_signals_and_used_area_roundtrip() {
        use crate::types::positions::{Position, WorldPosition};

        let road_network = RoadNetwork {
            logic_file: Some(LogicFile::new("test.xodr".to_string())),
            scene_graph_file: None,
            traffic_signals: Some(TrafficSignals {
                traffic_signal_controller: vec![TrafficSignalController::new(
                    "intersection_1",
                )],
            }),
            used_area: Some(UsedArea {
                position: vec![Position {
                    world_position: Some(WorldPosition::new(0.0, 0.0)),
                    ..Position::empty()
                }],
            }),
        };

        let xml = quick_xml::se::to_string(&road_network).unwrap();
        let deserialized: RoadNetwork = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(road_network, deserialized);
    }
}
