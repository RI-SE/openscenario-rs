//! Basic geometric shapes for OpenSCENARIO

use serde::{Deserialize, Serialize};
use crate::types::basic::{Double};
use crate::types::positions::Position;

/// Three-dimensional bounding box for entity spatial extents
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub struct BoundingBox {
    /// Center position of the bounding box
    #[serde(rename = "Center")]
    pub center: Center,
    /// Dimensions of the bounding box
    #[serde(rename = "Dimensions")]
    pub dimensions: Dimensions,
}

/// Three-dimensional center point
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Center {
    /// X coordinate
    #[serde(rename = "@x")]
    pub x: Double,
    /// Y coordinate  
    #[serde(rename = "@y")]
    pub y: Double,
    /// Z coordinate
    #[serde(rename = "@z")]
    pub z: Double,
}

/// Three-dimensional dimensions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dimensions {
    /// Width (y-axis extent)
    #[serde(rename = "@width")]
    pub width: Double,
    /// Length (x-axis extent)
    #[serde(rename = "@length")]
    pub length: Double,
    /// Height (z-axis extent)
    #[serde(rename = "@height")]
    pub height: Double,
}


impl Default for Center {
    fn default() -> Self {
        Self {
            x: crate::types::basic::Value::literal(0.0),
            y: crate::types::basic::Value::literal(0.0),
            z: crate::types::basic::Value::literal(0.0),
        }
    }
}

impl Default for Dimensions {
    fn default() -> Self {
        Self {
            width: crate::types::basic::Value::literal(2.0),  // Default car width
            length: crate::types::basic::Value::literal(4.5), // Default car length
            height: crate::types::basic::Value::literal(1.5), // Default car height
        }
    }
}

/// Shape definition for trajectories and paths
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Shape {
    #[serde(rename = "Polyline", skip_serializing_if = "Option::is_none")]
    pub polyline: Option<Polyline>,
    // Other shape types can be added later as optional fields
}

/// Polyline shape with time-positioned vertices
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Polyline {
    #[serde(rename = "Vertex", default)]
    pub vertices: Vec<Vertex>,
}

/// Trajectory vertex with time and position
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vertex {
    #[serde(rename = "@time")]
    pub time: Double,
    #[serde(rename = "Position")]
    pub position: Position,
}

impl Default for Shape {
    fn default() -> Self {
        Self {
            polyline: Some(Polyline::default()),
        }
    }
}

impl Default for Polyline {
    fn default() -> Self {
        Self {
            vertices: vec![Vertex::default()],
        }
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            time: crate::types::basic::Value::literal(0.0),
            position: Position::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounding_box_default() {
        let bbox = BoundingBox::default();
        
        // Center should be at origin
        assert_eq!(bbox.center.x.as_literal().unwrap(), &0.0);
        assert_eq!(bbox.center.y.as_literal().unwrap(), &0.0);
        assert_eq!(bbox.center.z.as_literal().unwrap(), &0.0);
        
        // Default dimensions should be car-like
        assert_eq!(bbox.dimensions.width.as_literal().unwrap(), &2.0);
        assert_eq!(bbox.dimensions.length.as_literal().unwrap(), &4.5);
        assert_eq!(bbox.dimensions.height.as_literal().unwrap(), &1.5);
    }

    #[test]
    fn test_trajectory_vertex() {
        use crate::types::positions::{Position, WorldPosition};
        
        let vertex = Vertex {
            time: crate::types::basic::Value::literal(0.04),
            position: Position {
                world_position: Some(WorldPosition::default()),
                relative_world_position: None,
                road_position: None,
                lane_position: None,
            },
        };
        
        assert_eq!(vertex.time.as_literal().unwrap(), &0.04);
    }

    #[test]
    fn test_polyline_structure() {
        use crate::types::positions::{Position, WorldPosition};
        
        let polyline = Polyline {
            vertices: vec![
                Vertex {
                    time: crate::types::basic::Value::literal(0.0),
                    position: Position {
                        world_position: Some(WorldPosition::default()),
                        relative_world_position: None,
                        road_position: None,
                        lane_position: None,
                    },
                },
                Vertex {
                    time: crate::types::basic::Value::literal(0.04),
                    position: Position {
                        world_position: Some(WorldPosition::default()),
                        relative_world_position: None,
                        road_position: None,
                        lane_position: None,
                    },
                },
            ],
        };
        
        assert_eq!(polyline.vertices.len(), 2);
        assert_eq!(polyline.vertices[0].time.as_literal().unwrap(), &0.0);
        assert_eq!(polyline.vertices[1].time.as_literal().unwrap(), &0.04);
    }

    #[test]
    fn test_shape_serialization() {
        use crate::types::positions::{Position, WorldPosition};
        
        let shape = Shape {
            polyline: Some(Polyline {
                vertices: vec![Vertex {
                    time: crate::types::basic::Value::literal(1.0),
                    position: Position {
                        world_position: Some(WorldPosition::default()),
                        relative_world_position: None,
                        road_position: None,
                        lane_position: None,
                    },
                }],
            }),
        };
        
        let xml = quick_xml::se::to_string(&shape).unwrap();
        assert!(xml.contains("<Polyline"));
        assert!(xml.contains("time=\"1\""));
    }

    #[test]
    fn test_bounding_box_serialization() {
        let bbox = BoundingBox {
            center: Center {
                x: crate::types::basic::Value::literal(1.0),
                y: crate::types::basic::Value::literal(2.0),
                z: crate::types::basic::Value::literal(0.5),
            },
            dimensions: Dimensions {
                width: crate::types::basic::Value::literal(1.8),
                length: crate::types::basic::Value::literal(4.2),
                height: crate::types::basic::Value::literal(1.4),
            },
        };
        
        // Test that serialization works
        let xml = quick_xml::se::to_string(&bbox).unwrap();
        assert!(xml.contains("x=\"1\""));
        assert!(xml.contains("width=\"1.8\""));
    }
}