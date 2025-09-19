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

use crate::types::basic::{Double, Int, OSString};
use serde::{Deserialize, Serialize};

/// Orientation definition for positions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Orientation {
    /// Heading angle (rotation around z-axis)
    #[serde(rename = "@h", skip_serializing_if = "Option::is_none")]
    pub h: Option<Double>,

    /// Pitch angle (rotation around y-axis)
    #[serde(rename = "@p", skip_serializing_if = "Option::is_none")]
    pub p: Option<Double>,

    /// Roll angle (rotation around x-axis)
    #[serde(rename = "@r", skip_serializing_if = "Option::is_none")]
    pub r: Option<Double>,
}

/// Road-based position definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoadPosition {
    /// Road ID reference
    #[serde(rename = "@roadId")]
    pub road_id: OSString,

    /// S-coordinate along the reference line
    #[serde(rename = "@s")]
    pub s: Double,

    /// T-coordinate (lateral offset from reference line)
    #[serde(rename = "@t")]
    pub t: Double,

    /// Orientation relative to s-direction
    #[serde(rename = "Orientation", skip_serializing_if = "Option::is_none")]
    pub orientation: Option<Orientation>,
}

/// Lane-based position definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LanePosition {
    /// Road ID reference
    #[serde(rename = "@roadId")]
    pub road_id: OSString,

    /// Lane ID reference
    #[serde(rename = "@laneId")]
    pub lane_id: OSString,

    /// S-coordinate along the reference line
    #[serde(rename = "@s")]
    pub s: Double,

    /// Offset from lane center
    #[serde(rename = "@offset")]
    pub offset: Double,

    /// Orientation relative to lane direction
    #[serde(rename = "Orientation", skip_serializing_if = "Option::is_none")]
    pub orientation: Option<Orientation>,
}

/// Relative road position relative to an entity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelativeRoadPosition {
    /// Reference entity for relative positioning
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,

    /// Delta S-coordinate along the reference line
    #[serde(rename = "@ds")]
    pub ds: Double,

    /// Delta T-coordinate (lateral offset from reference line)
    #[serde(rename = "@dt")]
    pub dt: Double,

    /// Orientation relative to reference entity
    #[serde(rename = "Orientation", skip_serializing_if = "Option::is_none")]
    pub orientation: Option<Orientation>,
}

/// Relative lane position relative to an entity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelativeLanePosition {
    /// Reference entity for relative positioning
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,

    /// Delta lane ID (relative lane offset)
    #[serde(rename = "@dLane")]
    pub d_lane: Int,

    /// Delta S-coordinate along the reference line
    #[serde(rename = "@ds")]
    pub ds: Double,

    /// Offset from lane center
    #[serde(rename = "@offset")]
    pub offset: Double,

    /// Orientation relative to lane direction
    #[serde(rename = "Orientation", skip_serializing_if = "Option::is_none")]
    pub orientation: Option<Orientation>,
}

/// Road coordinate system definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoadCoordinate {
    /// S-coordinate along the reference line
    #[serde(rename = "@s")]
    pub s: Double,

    /// T-coordinate (lateral offset from reference line)  
    #[serde(rename = "@t")]
    pub t: Double,

    /// Height coordinate (optional)
    #[serde(rename = "@h", skip_serializing_if = "Option::is_none")]
    pub h: Option<Double>,
}

/// Lane coordinate system definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LaneCoordinate {
    /// S-coordinate along the lane reference line
    #[serde(rename = "@s")]
    pub s: Double,

    /// Offset from lane center line
    #[serde(rename = "@offset")]
    pub offset: Double,

    /// Height coordinate (optional)
    #[serde(rename = "@h", skip_serializing_if = "Option::is_none")]
    pub h: Option<Double>,
}

impl RoadPosition {
    /// Create a new road position
    pub fn new(road_id: String, s: f64, t: f64) -> Self {
        Self {
            road_id: OSString::literal(road_id),
            s: Double::literal(s),
            t: Double::literal(t),
            orientation: None,
        }
    }

    /// Create a road position with orientation
    pub fn with_orientation(road_id: String, s: f64, t: f64, orientation: Orientation) -> Self {
        Self {
            road_id: OSString::literal(road_id),
            s: Double::literal(s),
            t: Double::literal(t),
            orientation: Some(orientation),
        }
    }
}

impl LanePosition {
    /// Create a new lane position
    pub fn new(road_id: String, lane_id: String, s: f64, offset: f64) -> Self {
        Self {
            road_id: OSString::literal(road_id),
            lane_id: OSString::literal(lane_id),
            s: Double::literal(s),
            offset: Double::literal(offset),
            orientation: None,
        }
    }

    /// Create a lane position with orientation
    pub fn with_orientation(
        road_id: String,
        lane_id: String,
        s: f64,
        offset: f64,
        orientation: Orientation,
    ) -> Self {
        Self {
            road_id: OSString::literal(road_id),
            lane_id: OSString::literal(lane_id),
            s: Double::literal(s),
            offset: Double::literal(offset),
            orientation: Some(orientation),
        }
    }
}

impl RelativeRoadPosition {
    /// Create a new relative road position
    pub fn new(entity_ref: String, ds: f64, dt: f64) -> Self {
        Self {
            entity_ref: OSString::literal(entity_ref),
            ds: Double::literal(ds),
            dt: Double::literal(dt),
            orientation: None,
        }
    }

    /// Create a relative road position with orientation
    pub fn with_orientation(
        entity_ref: String,
        ds: f64,
        dt: f64,
        orientation: Orientation,
    ) -> Self {
        Self {
            entity_ref: OSString::literal(entity_ref),
            ds: Double::literal(ds),
            dt: Double::literal(dt),
            orientation: Some(orientation),
        }
    }
}

impl RelativeLanePosition {
    /// Create a new relative lane position
    pub fn new(entity_ref: String, d_lane: i32, ds: f64, offset: f64) -> Self {
        Self {
            entity_ref: OSString::literal(entity_ref),
            d_lane: Int::literal(d_lane),
            ds: Double::literal(ds),
            offset: Double::literal(offset),
            orientation: None,
        }
    }

    /// Create a relative lane position with orientation
    pub fn with_orientation(
        entity_ref: String,
        d_lane: i32,
        ds: f64,
        offset: f64,
        orientation: Orientation,
    ) -> Self {
        Self {
            entity_ref: OSString::literal(entity_ref),
            d_lane: Int::literal(d_lane),
            ds: Double::literal(ds),
            offset: Double::literal(offset),
            orientation: Some(orientation),
        }
    }
}

impl RoadCoordinate {
    /// Create a new road coordinate
    pub fn new(s: f64, t: f64) -> Self {
        Self {
            s: Double::literal(s),
            t: Double::literal(t),
            h: None,
        }
    }

    /// Create a new road coordinate with height
    pub fn with_height(s: f64, t: f64, h: f64) -> Self {
        Self {
            s: Double::literal(s),
            t: Double::literal(t),
            h: Some(Double::literal(h)),
        }
    }

    /// Create coordinate along road center
    pub fn center_line(s: f64) -> Self {
        Self::new(s, 0.0)
    }

    /// Create coordinate with lateral offset
    pub fn with_offset(s: f64, t: f64) -> Self {
        Self::new(s, t)
    }
}

impl LaneCoordinate {
    /// Create a new lane coordinate
    pub fn new(s: f64, offset: f64) -> Self {
        Self {
            s: Double::literal(s),
            offset: Double::literal(offset),
            h: None,
        }
    }

    /// Create a new lane coordinate with height
    pub fn with_height(s: f64, offset: f64, h: f64) -> Self {
        Self {
            s: Double::literal(s),
            offset: Double::literal(offset),
            h: Some(Double::literal(h)),
        }
    }

    /// Create coordinate at lane center
    pub fn center_line(s: f64) -> Self {
        Self::new(s, 0.0)
    }

    /// Create coordinate with offset from center
    pub fn with_offset(s: f64, offset: f64) -> Self {
        Self::new(s, offset)
    }
}

impl Orientation {
    /// Create a new orientation with heading only
    pub fn heading(h: f64) -> Self {
        Self {
            h: Some(Double::literal(h)),
            p: None,
            r: None,
        }
    }

    /// Create a new orientation with all angles
    pub fn new(h: f64, p: f64, r: f64) -> Self {
        Self {
            h: Some(Double::literal(h)),
            p: Some(Double::literal(p)),
            r: Some(Double::literal(r)),
        }
    }
}

impl Default for RoadCoordinate {
    fn default() -> Self {
        Self {
            s: Double::literal(0.0),
            t: Double::literal(0.0),
            h: None,
        }
    }
}

impl Default for LaneCoordinate {
    fn default() -> Self {
        Self {
            s: Double::literal(0.0),
            offset: Double::literal(0.0),
            h: None,
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

impl Default for RelativeRoadPosition {
    fn default() -> Self {
        Self {
            entity_ref: OSString::literal("DefaultEntity".to_string()),
            ds: Double::literal(0.0),
            dt: Double::literal(0.0),
            orientation: None,
        }
    }
}

impl Default for RelativeLanePosition {
    fn default() -> Self {
        Self {
            entity_ref: OSString::literal("DefaultEntity".to_string()),
            d_lane: Int::literal(0),
            ds: Double::literal(0.0),
            offset: Double::literal(0.0),
            orientation: None,
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

    #[test]
    fn test_relative_road_position_creation() {
        let pos = RelativeRoadPosition::new("EgoVehicle".to_string(), 10.0, -2.0);

        assert_eq!(pos.entity_ref.as_literal().unwrap(), "EgoVehicle");
        assert_eq!(pos.ds.as_literal().unwrap(), &10.0);
        assert_eq!(pos.dt.as_literal().unwrap(), &-2.0);
        assert!(pos.orientation.is_none());
    }

    #[test]
    fn test_relative_road_position_with_orientation() {
        let orientation = Orientation::heading(0.5);
        let pos =
            RelativeRoadPosition::with_orientation("EgoVehicle".to_string(), 5.0, 1.5, orientation);

        assert_eq!(pos.entity_ref.as_literal().unwrap(), "EgoVehicle");
        assert_eq!(pos.ds.as_literal().unwrap(), &5.0);
        assert_eq!(pos.dt.as_literal().unwrap(), &1.5);
        assert!(pos.orientation.is_some());
        assert_eq!(
            pos.orientation.unwrap().h.unwrap().as_literal().unwrap(),
            &0.5
        );
    }

    #[test]
    fn test_relative_lane_position_creation() {
        let pos = RelativeLanePosition::new("EgoVehicle".to_string(), -1, 15.0, 0.5);

        assert_eq!(pos.entity_ref.as_literal().unwrap(), "EgoVehicle");
        assert_eq!(pos.d_lane, Int::literal(-1));
        assert_eq!(pos.ds.as_literal().unwrap(), &15.0);
        assert_eq!(pos.offset.as_literal().unwrap(), &0.5);
        assert!(pos.orientation.is_none());
    }

    #[test]
    fn test_relative_lane_position_with_orientation() {
        let orientation = Orientation::new(1.57, 0.0, 0.0);
        let pos = RelativeLanePosition::with_orientation(
            "EgoVehicle".to_string(),
            1,
            20.0,
            -1.0,
            orientation,
        );

        assert_eq!(pos.entity_ref.as_literal().unwrap(), "EgoVehicle");
        assert_eq!(pos.d_lane, Int::literal(1));
        assert_eq!(pos.ds.as_literal().unwrap(), &20.0);
        assert_eq!(pos.offset.as_literal().unwrap(), &-1.0);
        assert!(pos.orientation.is_some());
        let orient = pos.orientation.unwrap();
        assert_eq!(orient.h.unwrap().as_literal().unwrap(), &1.57);
        assert_eq!(orient.p.unwrap().as_literal().unwrap(), &0.0);
        assert_eq!(orient.r.unwrap().as_literal().unwrap(), &0.0);
    }

    #[test]
    fn test_relative_road_position_serialization() {
        let pos = RelativeRoadPosition::new("EgoVehicle".to_string(), 10.0, -2.0);
        let xml = quick_xml::se::to_string(&pos).unwrap();

        assert!(xml.contains("RelativeRoadPosition"));
        assert!(xml.contains("entityRef=\"EgoVehicle\""));
        assert!(xml.contains("ds=\"10\""));
        assert!(xml.contains("dt=\"-2\""));
    }

    #[test]
    fn test_relative_lane_position_serialization() {
        let pos = RelativeLanePosition::new("EgoVehicle".to_string(), -1, 15.0, 0.5);
        let xml = quick_xml::se::to_string(&pos).unwrap();

        assert!(xml.contains("RelativeLanePosition"));
        assert!(xml.contains("entityRef=\"EgoVehicle\""));
        assert!(xml.contains("dLane=\"-1\""));
        assert!(xml.contains("ds=\"15\""));
        assert!(xml.contains("offset=\"0.5\""));
    }

    #[test]
    fn test_relative_position_defaults() {
        let rel_road = RelativeRoadPosition::default();
        assert_eq!(rel_road.entity_ref.as_literal().unwrap(), "DefaultEntity");
        assert_eq!(rel_road.ds.as_literal().unwrap(), &0.0);
        assert_eq!(rel_road.dt.as_literal().unwrap(), &0.0);

        let rel_lane = RelativeLanePosition::default();
        assert_eq!(rel_lane.entity_ref.as_literal().unwrap(), "DefaultEntity");
        assert_eq!(rel_lane.d_lane, Int::literal(0));
        assert_eq!(rel_lane.ds.as_literal().unwrap(), &0.0);
        assert_eq!(rel_lane.offset.as_literal().unwrap(), &0.0);
    }
}
