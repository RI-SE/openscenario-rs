//! Lane position builder for road-relative positions

use crate::builder::{BuilderError, BuilderResult};
use crate::types::basic::{Double, OSString};
use crate::types::positions::{Position, LanePosition};
use super::PositionBuilder;

/// Builder for lane positions
#[derive(Debug, Clone, Default)]
pub struct LanePositionBuilder {
    road_id: Option<String>,
    lane_id: Option<String>,
    s: Option<f64>,
    offset: Option<f64>,
}

impl LanePositionBuilder {
    /// Create a new lane position builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the road ID
    pub fn road(mut self, road_id: &str) -> Self {
        self.road_id = Some(road_id.to_string());
        self
    }

    /// Set the lane ID
    pub fn lane(mut self, lane_id: &str) -> Self {
        self.lane_id = Some(lane_id.to_string());
        self
    }

    /// Set the s coordinate (distance along road)
    pub fn s(mut self, s: f64) -> Self {
        self.s = Some(s);
        self
    }

    /// Set the lateral offset from lane center
    pub fn offset(mut self, offset: f64) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Set position for right lane with integer lane ID
    pub fn right_lane(mut self, road_id: &str, lane_number: i32, s: f64) -> Self {
        self.road_id = Some(road_id.to_string());
        self.lane_id = Some(lane_number.to_string());
        self.s = Some(s);
        self.offset = Some(0.0);
        self
    }
}

impl PositionBuilder for LanePositionBuilder {
    fn finish(self) -> BuilderResult<Position> {
        self.validate()?;

        let lane_position = LanePosition {
            road_id: OSString::literal(self.road_id.unwrap()),
            lane_id: OSString::literal(self.lane_id.unwrap()),
            s: Double::literal(self.s.unwrap()),
            offset: Double::literal(self.offset.unwrap()),
            orientation: None,
        };

        let mut position = Position::default();
        position.lane_position = Some(lane_position);
        Ok(position)
    }

    fn validate(&self) -> BuilderResult<()> {
        if self.road_id.is_none() {
            return Err(BuilderError::validation_error("Road ID is required"));
        }
        if self.lane_id.is_none() {
            return Err(BuilderError::validation_error("Lane ID is required"));
        }
        if self.s.is_none() {
            return Err(BuilderError::validation_error("S coordinate is required"));
        }
        if self.offset.is_none() {
            return Err(BuilderError::validation_error("Offset is required"));
        }
        Ok(())
    }
}