//! World position builder for absolute coordinate positions

use crate::builder::{BuilderError, BuilderResult};
use crate::types::basic::Double;
use crate::types::positions::{Position, WorldPosition};
use super::PositionBuilder;

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