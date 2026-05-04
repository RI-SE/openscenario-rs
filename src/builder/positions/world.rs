//! World position builder for absolute coordinate positions

use super::PositionBuilder;
use crate::builder::{BuilderError, BuilderResult};
use crate::types::basic::Double;
use crate::types::positions::{Position, WorldPosition};

/// Builder for world positions with absolute coordinates
#[derive(Debug, Clone, Default)]
pub struct WorldPositionBuilder {
    x: Option<f64>,
    y: Option<f64>,
    z: Option<f64>,
    h: Option<f64>,
    p: Option<f64>,
    r: Option<f64>,
}

impl WorldPositionBuilder {
    /// Create a new world position builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the X coordinate (east)
    pub fn x(mut self, x: f64) -> Self {
        self.x = Some(x);
        self
    }

    /// Set the Y coordinate (north)
    pub fn y(mut self, y: f64) -> Self {
        self.y = Some(y);
        self
    }

    /// Set the Z coordinate (up)
    pub fn z(mut self, z: f64) -> Self {
        self.z = Some(z);
        self
    }

    /// Set the heading angle
    pub fn heading(mut self, h: f64) -> Self {
        self.h = Some(h);
        self
    }

    /// Set the pitch angle
    pub fn pitch(mut self, p: f64) -> Self {
        self.p = Some(p);
        self
    }

    /// Set the roll angle
    pub fn roll(mut self, r: f64) -> Self {
        self.r = Some(r);
        self
    }

    /// Set coordinates all at once (x, y, z)
    pub fn at_coordinates(mut self, x: f64, y: f64, z: f64) -> Self {
        self.x = Some(x);
        self.y = Some(y);
        self.z = Some(z);
        self
    }

    /// Set the heading angle (alias for heading method)
    pub fn with_heading(mut self, h: f64) -> Self {
        self.h = Some(h);
        self
    }

    /// Build the world position
    pub fn build(self) -> BuilderResult<Position> {
        self.finish()
    }
}

impl PositionBuilder for WorldPositionBuilder {
    fn finish(self) -> BuilderResult<Position> {
        self.validate()?;

        let world_position = WorldPosition {
            x: Double::literal(self.x.unwrap()),
            y: Double::literal(self.y.unwrap()),
            z: self.z.map(Double::literal),
            h: self.h.map(Double::literal),
            p: self.p.map(Double::literal),
            r: self.r.map(Double::literal),
        };

        let mut position = Position::default();
        position.world_position = Some(world_position);
        Ok(position)
    }

    fn validate(&self) -> BuilderResult<()> {
        if self.x.is_none() {
            return Err(BuilderError::validation_error("X coordinate is required"));
        }
        if self.y.is_none() {
            return Err(BuilderError::validation_error("Y coordinate is required"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimal_world_position_xy_only() {
        let pos = WorldPositionBuilder::new()
            .x(10.0)
            .y(20.0)
            .finish()
            .unwrap();
        let wp = pos.world_position.unwrap();
        assert_eq!(wp.x.as_literal(), Some(&10.0));
        assert_eq!(wp.y.as_literal(), Some(&20.0));
        assert!(wp.z.is_none());
        assert!(wp.h.is_none());
    }

    #[test]
    fn test_missing_x_fails() {
        let result = WorldPositionBuilder::new().y(20.0).finish();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("X coordinate"));
    }

    #[test]
    fn test_missing_y_fails() {
        let result = WorldPositionBuilder::new().x(10.0).finish();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Y coordinate"));
    }

    #[test]
    fn test_at_coordinates_with_heading() {
        let pos = WorldPositionBuilder::new()
            .at_coordinates(1.0, 2.0, 3.0)
            .heading(1.57)
            .finish()
            .unwrap();
        let wp = pos.world_position.unwrap();
        assert_eq!(wp.z.as_ref().unwrap().as_literal(), Some(&3.0));
        assert_eq!(wp.h.as_ref().unwrap().as_literal(), Some(&1.57));
    }

    #[test]
    fn test_build_alias_works_same_as_finish() {
        let pos = WorldPositionBuilder::new()
            .x(5.0)
            .y(6.0)
            .build()
            .unwrap();
        assert!(pos.world_position.is_some());
    }
}
