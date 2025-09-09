//! Road-based position types for highway and street positioning
//!
//! This file contains:
//! - RoadPosition for road-relative coordinates (road ID, s, t)
//! - RelativeRoadPosition for entity-relative road positioning
//! - LanePosition for lane-specific positioning with offsets
//! - RelativeLanePosition for lane-relative positioning
//! - Road network integration and coordinate validation
//!
//! Contributes to project by:
//! - Supporting OpenDRIVE and road network integration
//! - Providing natural positioning for automotive scenarios
//! - Enabling lane-aware positioning and lane change operations
//! - Facilitating road-following and path planning algorithms
//! - Supporting both absolute and relative road-based positioning

use serde::{Deserialize, Serialize};
use crate::types::basic::Value;

/// Orientation definition for positions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Orientation {
    /// Heading angle (rotation around z-axis)
    #[serde(rename = "@h", skip_serializing_if = "Option::is_none")]
    pub h: Option<Value<f64>>,
    
    /// Pitch angle (rotation around y-axis)
    #[serde(rename = "@p", skip_serializing_if = "Option::is_none")]
    pub p: Option<Value<f64>>,
    
    /// Roll angle (rotation around x-axis)
    #[serde(rename = "@r", skip_serializing_if = "Option::is_none")]
    pub r: Option<Value<f64>>,
}

/// Road-based position definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoadPosition {
    /// Road ID reference
    #[serde(rename = "@roadId")]
    pub road_id: Value<String>,
    
    /// S-coordinate along the reference line
    #[serde(rename = "@s")]
    pub s: Value<f64>,
    
    /// T-coordinate (lateral offset from reference line)
    #[serde(rename = "@t")]
    pub t: Value<f64>,
    
    /// Orientation relative to s-direction
    #[serde(rename = "Orientation", skip_serializing_if = "Option::is_none")]
    pub orientation: Option<Orientation>,
}

/// Lane-based position definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LanePosition {
    /// Road ID reference
    #[serde(rename = "@roadId")]
    pub road_id: Value<String>,
    
    /// Lane ID reference
    #[serde(rename = "@laneId")]
    pub lane_id: Value<String>,
    
    /// S-coordinate along the reference line
    #[serde(rename = "@s")]
    pub s: Value<f64>,
    
    /// Offset from lane center
    #[serde(rename = "@offset")]
    pub offset: Value<f64>,
    
    /// Orientation relative to lane direction
    #[serde(rename = "Orientation", skip_serializing_if = "Option::is_none")]
    pub orientation: Option<Orientation>,
}

impl RoadPosition {
    /// Create a new road position
    pub fn new(road_id: String, s: f64, t: f64) -> Self {
        Self {
            road_id: Value::literal(road_id),
            s: Value::literal(s),
            t: Value::literal(t),
            orientation: None,
        }
    }
    
    /// Create a road position with orientation
    pub fn with_orientation(road_id: String, s: f64, t: f64, orientation: Orientation) -> Self {
        Self {
            road_id: Value::literal(road_id),
            s: Value::literal(s),
            t: Value::literal(t),
            orientation: Some(orientation),
        }
    }
}

impl LanePosition {
    /// Create a new lane position
    pub fn new(road_id: String, lane_id: String, s: f64, offset: f64) -> Self {
        Self {
            road_id: Value::literal(road_id),
            lane_id: Value::literal(lane_id),
            s: Value::literal(s),
            offset: Value::literal(offset),
            orientation: None,
        }
    }
    
    /// Create a lane position with orientation
    pub fn with_orientation(
        road_id: String, 
        lane_id: String, 
        s: f64, 
        offset: f64, 
        orientation: Orientation
    ) -> Self {
        Self {
            road_id: Value::literal(road_id),
            lane_id: Value::literal(lane_id),
            s: Value::literal(s),
            offset: Value::literal(offset),
            orientation: Some(orientation),
        }
    }
}

impl Orientation {
    /// Create a new orientation with heading only
    pub fn heading(h: f64) -> Self {
        Self {
            h: Some(Value::literal(h)),
            p: None,
            r: None,
        }
    }
    
    /// Create a new orientation with all angles
    pub fn new(h: f64, p: f64, r: f64) -> Self {
        Self {
            h: Some(Value::literal(h)),
            p: Some(Value::literal(p)),
            r: Some(Value::literal(r)),
        }
    }
}

impl Default for Orientation {
    fn default() -> Self {
        Self {
            h: None,
            p: None,
            r: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_road_position_creation() {
        let pos = RoadPosition::new("0".to_string(), 100.0, 2.5);
        
        assert_eq!(pos.road_id.as_literal().unwrap(), "0");
        assert_eq!(pos.s.as_literal().unwrap(), &100.0);
        assert_eq!(pos.t.as_literal().unwrap(), &2.5);
        assert!(pos.orientation.is_none());
    }
    
    #[test]
    fn test_lane_position_creation() {
        let pos = LanePosition::new("0".to_string(), "-4".to_string(), 5.0, 0.0);
        
        assert_eq!(pos.road_id.as_literal().unwrap(), "0");
        assert_eq!(pos.lane_id.as_literal().unwrap(), "-4");
        assert_eq!(pos.s.as_literal().unwrap(), &5.0);
        assert_eq!(pos.offset.as_literal().unwrap(), &0.0);
        assert!(pos.orientation.is_none());
    }
    
    #[test]
    fn test_orientation_creation() {
        let orientation = Orientation::heading(1.57);
        
        assert_eq!(orientation.h.as_ref().unwrap().as_literal().unwrap(), &1.57);
        assert!(orientation.p.is_none());
        assert!(orientation.r.is_none());
    }
    
    #[test]
    fn test_road_position_with_orientation() {
        let orientation = Orientation::heading(0.5);
        let pos = RoadPosition::with_orientation("1".to_string(), 50.0, -1.0, orientation);
        
        assert_eq!(pos.road_id.as_literal().unwrap(), "1");
        assert_eq!(pos.s.as_literal().unwrap(), &50.0);
        assert_eq!(pos.t.as_literal().unwrap(), &-1.0);
        assert!(pos.orientation.is_some());
        assert_eq!(
            pos.orientation.unwrap().h.unwrap().as_literal().unwrap(), 
            &0.5
        );
    }
    
    #[test]
    fn test_lane_position_serialization() {
        let pos = LanePosition::new("0".to_string(), "-4".to_string(), 5.0, 0.0);
        let xml = quick_xml::se::to_string(&pos).unwrap();
        
        assert!(xml.contains("LanePosition"));
        assert!(xml.contains("roadId=\"0\""));
        assert!(xml.contains("laneId=\"-4\""));
        assert!(xml.contains("s=\"5\""));
        assert!(xml.contains("offset=\"0\""));
    }
}