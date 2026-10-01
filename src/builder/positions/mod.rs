//! Builders for each `Position` branch: world, relative, lane, and road.

pub mod lane;
pub mod relative;
pub mod world;

pub use lane::LanePositionBuilder;
pub use relative::RelativePositionBuilder;
pub use world::WorldPositionBuilder;

use crate::builder::BuilderResult;
use crate::types::positions::Position;

/// Trait for position builders that can be finished and converted to Position
pub trait PositionBuilder {
    /// Finish building the position and return it
    fn finish(self) -> BuilderResult<Position>;

    /// Validate the position configuration
    fn validate(&self) -> BuilderResult<()>;
}

/// Unified position builder interface for dynamic position type selection
pub struct UnifiedPositionBuilder;

/// Enum representing different position types
pub enum PositionType {
    World(WorldPositionBuilder),
    Relative(RelativePositionBuilder),
    Lane(LanePositionBuilder),
}

impl UnifiedPositionBuilder {
    /// Create a new world position builder
    pub fn world() -> WorldPositionBuilder {
        WorldPositionBuilder::new()
    }

    /// Create a new relative position builder
    pub fn relative() -> RelativePositionBuilder {
        RelativePositionBuilder::new()
    }

    /// Create a new lane position builder
    pub fn lane() -> LanePositionBuilder {
        LanePositionBuilder::new()
    }
}
