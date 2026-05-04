//! Relative position builder for entity-relative positions

use super::PositionBuilder;
use crate::builder::{BuilderError, BuilderResult};
use crate::types::basic::{Double, Int, OSString};
use crate::types::positions::{Position, RelativeLanePosition, RelativeWorldPosition};

/// Builder for relative positions
#[derive(Debug, Clone, Default)]
pub struct RelativePositionBuilder {
    entity_ref: Option<String>,
    // World offset
    dx: Option<f64>,
    dy: Option<f64>,
    dz: Option<f64>,
    // Lane offset
    ds: Option<f64>,
    d_lane: Option<i32>,
    offset: Option<f64>,
    position_type: RelativePositionType,
}

#[derive(Debug, Clone, Default)]
enum RelativePositionType {
    #[default]
    World,
    Lane,
}

impl RelativePositionBuilder {
    /// Create a new relative position builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the reference entity
    pub fn to_entity(mut self, entity_ref: &str) -> Self {
        self.entity_ref = Some(entity_ref.to_string());
        self
    }

    /// Set world coordinate offsets
    pub fn world_offset(mut self, dx: f64, dy: f64, dz: f64) -> Self {
        self.dx = Some(dx);
        self.dy = Some(dy);
        self.dz = Some(dz);
        self.position_type = RelativePositionType::World;
        self
    }

    /// Set lane coordinate offsets
    pub fn lane_offset(mut self, ds: f64, offset: f64) -> Self {
        self.ds = Some(ds);
        self.d_lane = Some(0);
        self.offset = Some(offset);
        self.position_type = RelativePositionType::Lane;
        self
    }
}

impl PositionBuilder for RelativePositionBuilder {
    fn finish(self) -> BuilderResult<Position> {
        self.validate()?;

        let mut position = Position::default();

        match self.position_type {
            RelativePositionType::World => {
                let relative_world_position = RelativeWorldPosition {
                    entity_ref: OSString::literal(self.entity_ref.unwrap()),
                    dx: Double::literal(self.dx.unwrap()),
                    dy: Double::literal(self.dy.unwrap()),
                    dz: Double::literal(self.dz.unwrap()),
                };
                position.relative_world_position = Some(relative_world_position);
            }
            RelativePositionType::Lane => {
                let relative_lane_position = RelativeLanePosition {
                    entity_ref: OSString::literal(self.entity_ref.unwrap()),
                    ds: Double::literal(self.ds.unwrap()),
                    d_lane: Int::literal(self.d_lane.unwrap()),
                    offset: Double::literal(self.offset.unwrap()),
                    orientation: None,
                };
                position.relative_lane_position = Some(relative_lane_position);
            }
        }

        Ok(position)
    }

    fn validate(&self) -> BuilderResult<()> {
        if self.entity_ref.is_none() {
            return Err(BuilderError::validation_error(
                "Entity reference is required",
            ));
        }

        match self.position_type {
            RelativePositionType::World => {
                if self.dx.is_none() || self.dy.is_none() || self.dz.is_none() {
                    return Err(BuilderError::validation_error(
                        "World offsets (dx, dy, dz) are required",
                    ));
                }
            }
            RelativePositionType::Lane => {
                if self.ds.is_none() || self.offset.is_none() {
                    return Err(BuilderError::validation_error(
                        "Lane offsets (ds, offset) are required",
                    ));
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_offset_builds_relative_world_position() {
        let pos = RelativePositionBuilder::new()
            .to_entity("ego")
            .world_offset(10.0, 5.0, 0.0)
            .finish()
            .unwrap();
        let rwp = pos.relative_world_position.unwrap();
        assert_eq!(rwp.entity_ref.as_literal(), Some(&"ego".to_string()));
        assert_eq!(rwp.dx.as_literal(), Some(&10.0));
        assert_eq!(rwp.dy.as_literal(), Some(&5.0));
    }

    #[test]
    fn test_lane_offset_builds_relative_lane_position() {
        let pos = RelativePositionBuilder::new()
            .to_entity("lead")
            .lane_offset(20.0, 0.5)
            .finish()
            .unwrap();
        let rlp = pos.relative_lane_position.unwrap();
        assert_eq!(rlp.entity_ref.as_literal(), Some(&"lead".to_string()));
        assert_eq!(rlp.ds.as_literal(), Some(&20.0));
        assert_eq!(rlp.offset.as_literal(), Some(&0.5));
    }

    #[test]
    fn test_missing_entity_ref_fails() {
        let result = RelativePositionBuilder::new()
            .world_offset(1.0, 2.0, 3.0)
            .finish();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Entity reference"));
    }

    #[test]
    fn test_default_type_is_world_and_fails_without_offsets() {
        let result = RelativePositionBuilder::new()
            .to_entity("ego")
            .finish();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("World offsets"));
    }
}
