//! Basic geometric shapes for OpenSCENARIO

use crate::types::basic::Double;
use crate::types::positions::Position;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Three-dimensional bounding box for entity spatial extents
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
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

impl Center {
    /// Calculate 3D Euclidean distance to another center point
    pub fn distance_to(&self, other: &Center) -> Result<f64> {
        let params = HashMap::new();
        let x1 = self.x.resolve(&params)?;
        let y1 = self.y.resolve(&params)?;
        let z1 = self.z.resolve(&params)?;
        
        let x2 = other.x.resolve(&params)?;
        let y2 = other.y.resolve(&params)?;
        let z2 = other.z.resolve(&params)?;
        
        let dx = x2 - x1;
        let dy = y2 - y1;
        let dz = z2 - z1;
        
        Ok((dx * dx + dy * dy + dz * dz).sqrt())
    }

    /// Calculate 3D Euclidean distance to another center point with parameter resolution
    pub fn distance_to_with_params(&self, other: &Center, params: &HashMap<String, String>) -> Result<f64> {
        let x1 = self.x.resolve(params)?;
        let y1 = self.y.resolve(params)?;
        let z1 = self.z.resolve(params)?;
        
        let x2 = other.x.resolve(params)?;
        let y2 = other.y.resolve(params)?;
        let z2 = other.z.resolve(params)?;
        
        let dx = x2 - x1;
        let dy = y2 - y1;
        let dz = z2 - z1;
        
        Ok((dx * dx + dy * dy + dz * dz).sqrt())
    }
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
            width: crate::types::basic::Value::literal(2.0), // Default car width
            length: crate::types::basic::Value::literal(4.5), // Default car length
            height: crate::types::basic::Value::literal(1.5), // Default car height
        }
    }
}

impl BoundingBox {
    /// Calculate the volume of the bounding box
    pub fn volume(&self) -> Result<f64> {
        let params = HashMap::new();
        let width = self.dimensions.width.resolve(&params)?;
        let length = self.dimensions.length.resolve(&params)?;
        let height = self.dimensions.height.resolve(&params)?;
        Ok(width * length * height)
    }

    /// Calculate the volume of the bounding box with parameter resolution
    pub fn volume_with_params(&self, params: &HashMap<String, String>) -> Result<f64> {
        let width = self.dimensions.width.resolve(params)?;
        let length = self.dimensions.length.resolve(params)?;
        let height = self.dimensions.height.resolve(params)?;
        Ok(width * length * height)
    }

    /// Check if a point is contained within this bounding box
    pub fn contains_point(&self, x: f64, y: f64, z: f64) -> Result<bool> {
        let params = HashMap::new();
        let center_x = self.center.x.resolve(&params)?;
        let center_y = self.center.y.resolve(&params)?;
        let center_z = self.center.z.resolve(&params)?;
        
        let width = self.dimensions.width.resolve(&params)?;
        let length = self.dimensions.length.resolve(&params)?;
        let height = self.dimensions.height.resolve(&params)?;

        let half_width = width / 2.0;
        let half_length = length / 2.0;
        let half_height = height / 2.0;

        Ok(x >= center_x - half_length && x <= center_x + half_length &&
           y >= center_y - half_width && y <= center_y + half_width &&
           z >= center_z - half_height && z <= center_z + half_height)
    }

    /// Check if a point is contained within this bounding box with parameter resolution
    pub fn contains_point_with_params(&self, x: f64, y: f64, z: f64, params: &HashMap<String, String>) -> Result<bool> {
        let center_x = self.center.x.resolve(params)?;
        let center_y = self.center.y.resolve(params)?;
        let center_z = self.center.z.resolve(params)?;
        
        let width = self.dimensions.width.resolve(params)?;
        let length = self.dimensions.length.resolve(params)?;
        let height = self.dimensions.height.resolve(params)?;

        let half_width = width / 2.0;
        let half_length = length / 2.0;
        let half_height = height / 2.0;

        Ok(x >= center_x - half_length && x <= center_x + half_length &&
           y >= center_y - half_width && y <= center_y + half_width &&
           z >= center_z - half_height && z <= center_z + half_height)
    }

    /// Calculate the distance from a point to the nearest surface of the bounding box
    pub fn distance_to_point(&self, x: f64, y: f64, z: f64, params: &HashMap<String, String>) -> Result<f64> {
        let center_x = self.center.x.resolve(params)?;
        let center_y = self.center.y.resolve(params)?;
        let center_z = self.center.z.resolve(params)?;
        
        let width = self.dimensions.width.resolve(params)?;
        let length = self.dimensions.length.resolve(params)?;
        let height = self.dimensions.height.resolve(params)?;

        let half_width = width / 2.0;
        let half_length = length / 2.0;
        let half_height = height / 2.0;

        // Calculate distance to each face
        let dx = (x - center_x).abs() - half_length;
        let dy = (y - center_y).abs() - half_width;
        let dz = (z - center_z).abs() - half_height;

        // If point is inside, return negative distance to nearest surface
        if dx <= 0.0 && dy <= 0.0 && dz <= 0.0 {
            // Return the maximum (least negative) of dx, dy, dz, which is the closest surface
            Ok(dx.max(dy).max(dz))
        } else {
            // Point is outside, calculate Euclidean distance to nearest corner/edge/face
            let dx_pos = dx.max(0.0);
            let dy_pos = dy.max(0.0);
            let dz_pos = dz.max(0.0);
            Ok((dx_pos * dx_pos + dy_pos * dy_pos + dz_pos * dz_pos).sqrt())
        }
    }

    /// Check if this bounding box intersects with another
    pub fn intersects(&self, other: &BoundingBox, params: &HashMap<String, String>) -> Result<bool> {
        let self_center_x = self.center.x.resolve(params)?;
        let self_center_y = self.center.y.resolve(params)?;
        let self_center_z = self.center.z.resolve(params)?;
        let self_width = self.dimensions.width.resolve(params)?;
        let self_length = self.dimensions.length.resolve(params)?;
        let self_height = self.dimensions.height.resolve(params)?;

        let other_center_x = other.center.x.resolve(params)?;
        let other_center_y = other.center.y.resolve(params)?;
        let other_center_z = other.center.z.resolve(params)?;
        let other_width = other.dimensions.width.resolve(params)?;
        let other_length = other.dimensions.length.resolve(params)?;
        let other_height = other.dimensions.height.resolve(params)?;

        // Check for separation along each axis
        let x_overlap = (self_center_x - other_center_x).abs() <= (self_length + other_length) / 2.0;
        let y_overlap = (self_center_y - other_center_y).abs() <= (self_width + other_width) / 2.0;
        let z_overlap = (self_center_z - other_center_z).abs() <= (self_height + other_height) / 2.0;

        Ok(x_overlap && y_overlap && z_overlap)
    }
}

impl Dimensions {
    /// Create dimensions for a standard vehicle (alias for car)
    pub fn vehicle_default() -> Self {
        Self::new(2.0, 4.5, 1.8)
    }

    /// Create dimensions for a pedestrian
    pub fn pedestrian_default() -> Self {
        Self::new(0.6, 0.6, 1.8)
    }

    /// Create dimensions for a truck
    pub fn truck_default() -> Self {
        Self::new(2.5, 12.0, 3.5)
    }

    /// Create new dimensions with specified values
    pub fn new(width: f64, length: f64, height: f64) -> Self {
        Self {
            width: crate::types::basic::Value::literal(width),
            length: crate::types::basic::Value::literal(length),
            height: crate::types::basic::Value::literal(height),
        }
    }

    /// Create dimensions for a standard car
    pub fn car() -> Self {
        Self {
            width: crate::types::basic::Value::literal(1.8),
            length: crate::types::basic::Value::literal(4.5),
            height: crate::types::basic::Value::literal(1.5),
        }
    }

    /// Create dimensions for a truck
    pub fn truck() -> Self {
        Self {
            width: crate::types::basic::Value::literal(2.5),
            length: crate::types::basic::Value::literal(12.0),
            height: crate::types::basic::Value::literal(3.8),
        }
    }

    /// Create dimensions for a bus
    pub fn bus() -> Self {
        Self {
            width: crate::types::basic::Value::literal(2.5),
            length: crate::types::basic::Value::literal(12.0),
            height: crate::types::basic::Value::literal(3.2),
        }
    }

    /// Create dimensions for a motorcycle
    pub fn motorcycle() -> Self {
        Self {
            width: crate::types::basic::Value::literal(0.8),
            length: crate::types::basic::Value::literal(2.2),
            height: crate::types::basic::Value::literal(1.3),
        }
    }

    /// Create dimensions for a pedestrian
    pub fn pedestrian() -> Self {
        Self {
            width: crate::types::basic::Value::literal(0.6),
            length: crate::types::basic::Value::literal(0.6),
            height: crate::types::basic::Value::literal(1.8),
        }
    }

    /// Calculate the footprint area (width * length)
    pub fn footprint_area(&self, params: &HashMap<String, String>) -> Result<f64> {
        let width = self.width.resolve(params)?;
        let length = self.length.resolve(params)?;
        Ok(width * length)
    }

    /// Scale dimensions by a factor
    pub fn scale(&self, factor: f64, params: &HashMap<String, String>) -> Result<Self> {
        let width = self.width.resolve(params)? * factor;
        let length = self.length.resolve(params)? * factor;
        let height = self.height.resolve(params)? * factor;
        
        Ok(Self {
            width: crate::types::basic::Value::literal(width),
            length: crate::types::basic::Value::literal(length),
            height: crate::types::basic::Value::literal(height),
        })
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
                relative_road_position: None,
                lane_position: None,
                relative_lane_position: None,
                trajectory_position: None,
                geographic_position: None,
                relative_object_position: None,
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
                    position: Position::default(),
                },
                Vertex {
                    time: crate::types::basic::Value::literal(0.04),
                    position: Position::default(),
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
                    position: Position::default(),
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

    #[test]
    fn test_bounding_box_volume() {
        use std::collections::HashMap;
        
        let bbox = BoundingBox {
            center: Center::default(),
            dimensions: Dimensions {
                width: crate::types::basic::Value::literal(2.0),
                length: crate::types::basic::Value::literal(4.0),
                height: crate::types::basic::Value::literal(1.5),
            },
        };

        let volume = bbox.volume().unwrap();
        assert_eq!(volume, 12.0); // 2.0 * 4.0 * 1.5
    }

    #[test]
    fn test_bounding_box_contains_point() {
        use std::collections::HashMap;
        
        let bbox = BoundingBox {
            center: Center {
                x: crate::types::basic::Value::literal(0.0),
                y: crate::types::basic::Value::literal(0.0),
                z: crate::types::basic::Value::literal(0.0),
            },
            dimensions: Dimensions {
                width: crate::types::basic::Value::literal(2.0),
                length: crate::types::basic::Value::literal(4.0),
                height: crate::types::basic::Value::literal(1.5),
            },
        };

        // Point inside
        assert!(bbox.contains_point(0.0, 0.0, 0.0).unwrap());
        assert!(bbox.contains_point(1.9, 0.9, 0.7).unwrap());
        
        // Point outside
        assert!(!bbox.contains_point(2.1, 0.0, 0.0).unwrap());
        assert!(!bbox.contains_point(0.0, 1.1, 0.0).unwrap());
        assert!(!bbox.contains_point(0.0, 0.0, 0.8).unwrap());
    }

    #[test]
    fn test_bounding_box_distance_to_point() {
        use std::collections::HashMap;
        
        let bbox = BoundingBox {
            center: Center::default(),
            dimensions: Dimensions {
                width: crate::types::basic::Value::literal(2.0),
                length: crate::types::basic::Value::literal(4.0),
                height: crate::types::basic::Value::literal(2.0),
            },
        };

        let params = HashMap::new();
        
        // Point inside should have negative distance
        let distance = bbox.distance_to_point(0.0, 0.0, 0.0, &params).unwrap();
        assert!(distance < 0.0);
        
        // Point outside
        let distance = bbox.distance_to_point(3.0, 0.0, 0.0, &params).unwrap();
        assert!(distance > 0.0);
        assert!((distance - 1.0).abs() < 0.001); // Should be 1.0 unit away
    }

    #[test]
    fn test_bounding_box_intersection() {
        use std::collections::HashMap;
        
        let bbox1 = BoundingBox {
            center: Center::default(),
            dimensions: Dimensions {
                width: crate::types::basic::Value::literal(2.0),
                length: crate::types::basic::Value::literal(2.0),
                height: crate::types::basic::Value::literal(2.0),
            },
        };

        let bbox2 = BoundingBox {
            center: Center {
                x: crate::types::basic::Value::literal(1.0),
                y: crate::types::basic::Value::literal(0.0),
                z: crate::types::basic::Value::literal(0.0),
            },
            dimensions: Dimensions {
                width: crate::types::basic::Value::literal(2.0),
                length: crate::types::basic::Value::literal(2.0),
                height: crate::types::basic::Value::literal(2.0),
            },
        };

        let bbox3 = BoundingBox {
            center: Center {
                x: crate::types::basic::Value::literal(3.0),
                y: crate::types::basic::Value::literal(0.0),
                z: crate::types::basic::Value::literal(0.0),
            },
            dimensions: Dimensions {
                width: crate::types::basic::Value::literal(2.0),
                length: crate::types::basic::Value::literal(2.0),
                height: crate::types::basic::Value::literal(2.0),
            },
        };

        let params = HashMap::new();
        
        // bbox1 and bbox2 should intersect
        assert!(bbox1.intersects(&bbox2, &params).unwrap());
        
        // bbox1 and bbox3 should not intersect
        assert!(!bbox1.intersects(&bbox3, &params).unwrap());
    }

    #[test]
    fn test_dimensions_presets() {
        // Test car dimensions
        let car = Dimensions::car();
        assert_eq!(car.width.as_literal().unwrap(), &1.8);
        assert_eq!(car.length.as_literal().unwrap(), &4.5);
        assert_eq!(car.height.as_literal().unwrap(), &1.5);

        // Test truck dimensions
        let truck = Dimensions::truck();
        assert_eq!(truck.width.as_literal().unwrap(), &2.5);
        assert_eq!(truck.length.as_literal().unwrap(), &12.0);
        assert_eq!(truck.height.as_literal().unwrap(), &3.8);

        // Test pedestrian dimensions
        let pedestrian = Dimensions::pedestrian();
        assert_eq!(pedestrian.width.as_literal().unwrap(), &0.6);
        assert_eq!(pedestrian.length.as_literal().unwrap(), &0.6);
        assert_eq!(pedestrian.height.as_literal().unwrap(), &1.8);
    }

    #[test]
    fn test_dimensions_footprint_area() {
        use std::collections::HashMap;
        
        let dims = Dimensions::car();
        let params = HashMap::new();
        let area = dims.footprint_area(&params).unwrap();
        assert_eq!(area, 8.1); // 1.8 * 4.5
    }

    #[test]
    fn test_dimensions_scale() {
        use std::collections::HashMap;
        
        let dims = Dimensions::car();
        let params = HashMap::new();
        let scaled = dims.scale(2.0, &params).unwrap();
        
        assert_eq!(scaled.width.as_literal().unwrap(), &3.6); // 1.8 * 2.0
        assert_eq!(scaled.length.as_literal().unwrap(), &9.0); // 4.5 * 2.0
        assert_eq!(scaled.height.as_literal().unwrap(), &3.0); // 1.5 * 2.0
    }

    #[test]
    fn test_bounding_box_with_parameters() {
        use std::collections::HashMap;
        
        let bbox = BoundingBox {
            center: Center::default(),
            dimensions: Dimensions {
                width: crate::types::basic::Value::parameter("vehicle_width".to_string()),
                length: crate::types::basic::Value::parameter("vehicle_length".to_string()),
                height: crate::types::basic::Value::literal(1.5),
            },
        };

        let mut params = HashMap::new();
        params.insert("vehicle_width".to_string(), "2.0".to_string());
        params.insert("vehicle_length".to_string(), "4.0".to_string());

        let volume = bbox.volume_with_params(&params).unwrap();
        assert_eq!(volume, 12.0); // 2.0 * 4.0 * 1.5
    }

    #[test]
    fn test_center_distance_to() {
        let center1 = Center {
            x: crate::types::basic::Value::literal(0.0),
            y: crate::types::basic::Value::literal(0.0),
            z: crate::types::basic::Value::literal(0.0),
        };

        let center2 = Center {
            x: crate::types::basic::Value::literal(3.0),
            y: crate::types::basic::Value::literal(4.0),
            z: crate::types::basic::Value::literal(0.0),
        };

        let distance = center1.distance_to(&center2).unwrap();
        assert_eq!(distance, 5.0); // 3-4-5 triangle
    }

    #[test]
    fn test_center_distance_to_3d() {
        let center1 = Center {
            x: crate::types::basic::Value::literal(1.0),
            y: crate::types::basic::Value::literal(2.0),
            z: crate::types::basic::Value::literal(3.0),
        };

        let center2 = Center {
            x: crate::types::basic::Value::literal(4.0),
            y: crate::types::basic::Value::literal(6.0),
            z: crate::types::basic::Value::literal(8.0),
        };

        let distance = center1.distance_to(&center2).unwrap();
        let expected = ((3.0_f64 * 3.0) + (4.0 * 4.0) + (5.0 * 5.0)).sqrt(); // sqrt(9 + 16 + 25) = sqrt(50)
        assert!((distance - expected).abs() < 0.001);
    }

    #[test]
    fn test_dimensions_new_defaults() {
        // Test vehicle_default
        let vehicle_dims = Dimensions::vehicle_default();
        assert_eq!(vehicle_dims.width.as_literal().unwrap(), &2.0);
        assert_eq!(vehicle_dims.length.as_literal().unwrap(), &4.5);
        assert_eq!(vehicle_dims.height.as_literal().unwrap(), &1.8);

        // Test pedestrian_default
        let pedestrian_dims = Dimensions::pedestrian_default();
        assert_eq!(pedestrian_dims.width.as_literal().unwrap(), &0.6);
        assert_eq!(pedestrian_dims.length.as_literal().unwrap(), &0.6);
        assert_eq!(pedestrian_dims.height.as_literal().unwrap(), &1.8);

        // Test truck_default
        let truck_dims = Dimensions::truck_default();
        assert_eq!(truck_dims.width.as_literal().unwrap(), &2.5);
        assert_eq!(truck_dims.length.as_literal().unwrap(), &12.0);
        assert_eq!(truck_dims.height.as_literal().unwrap(), &3.5);
    }

    #[test]
    fn test_dimensions_new() {
        let dims = Dimensions::new(1.5, 3.0, 2.0);
        assert_eq!(dims.width.as_literal().unwrap(), &1.5);
        assert_eq!(dims.length.as_literal().unwrap(), &3.0);
        assert_eq!(dims.height.as_literal().unwrap(), &2.0);
    }

    #[test]
    fn test_bounding_box_volume_no_params() {
        let bbox = BoundingBox {
            center: Center::default(),
            dimensions: Dimensions::new(2.0, 4.0, 1.5),
        };

        let volume = bbox.volume().unwrap();
        assert_eq!(volume, 12.0); // 2.0 * 4.0 * 1.5
    }

    #[test]
    fn test_bounding_box_contains_point_no_params() {
        let bbox = BoundingBox {
            center: Center::default(),
            dimensions: Dimensions::new(2.0, 4.0, 1.5),
        };

        // Point inside
        assert!(bbox.contains_point(0.0, 0.0, 0.0).unwrap());
        assert!(bbox.contains_point(1.9, 0.9, 0.7).unwrap());
        
        // Point outside
        assert!(!bbox.contains_point(2.1, 0.0, 0.0).unwrap());
        assert!(!bbox.contains_point(0.0, 1.1, 0.0).unwrap());
        assert!(!bbox.contains_point(0.0, 0.0, 0.8).unwrap());
    }
}
