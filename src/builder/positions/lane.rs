//! Lane position builder for road-relative positions

use super::PositionBuilder;
use crate::builder::{BuilderError, BuilderResult};
use crate::types::basic::{Double, OSString};
use crate::types::positions::{LanePosition, Position};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_lane_position_builds() {
        let pos = LanePositionBuilder::new()
            .road("1")
            .lane("-1")
            .s(50.0)
            .offset(0.0)
            .finish()
            .unwrap();
        let lp = pos.lane_position.unwrap();
        assert_eq!(lp.road_id.as_literal(), Some(&"1".to_string()));
        assert_eq!(lp.lane_id.as_literal(), Some(&"-1".to_string()));
        assert_eq!(lp.s.as_literal(), Some(&50.0));
    }

    #[test]
    fn test_missing_road_id_fails_validation() {
        let result = LanePositionBuilder::new()
            .lane("-1")
            .s(50.0)
            .offset(0.0)
            .finish();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Road ID"));
    }

    #[test]
    fn test_missing_s_coordinate_fails() {
        let result = LanePositionBuilder::new()
            .road("1")
            .lane("-1")
            .offset(0.0)
            .finish();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("S coordinate"));
    }

    #[test]
    fn test_right_lane_helper_sets_all_fields() {
        let pos = LanePositionBuilder::new()
            .right_lane("road1", -2, 100.0)
            .finish()
            .unwrap();
        let lp = pos.lane_position.unwrap();
        assert_eq!(lp.lane_id.as_literal(), Some(&"-2".to_string()));
        assert_eq!(lp.s.as_literal(), Some(&100.0));
        assert_eq!(lp.offset.as_literal(), Some(&0.0));
    }
}
