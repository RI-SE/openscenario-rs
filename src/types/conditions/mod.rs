//! Condition type module for trigger and event management
//!
//! This file contains:
//! - Base condition traits and common condition behaviors
//! - Condition evaluation logic and state management
//! - Trigger edge detection (rising, falling, rising-or-falling)
//! - Condition group logic (AND/OR combinations)
//! - Condition validation and constraint checking
//!
pub mod entity; // Entity-based conditions
pub mod spatial; // Spatial conditions
pub mod value; // Value-based conditions

// Re-export spatial conditions for convenience
pub use spatial::{DistanceCondition, ReachPositionCondition, RelativeDistanceCondition};

// Keep entity-specific re-exports
pub use entity::{
    AccelerationCondition, AngleCondition, ByEntityCondition, CollisionCondition, CollisionTarget,
    EndOfRoadCondition, EntityCondition, OffroadCondition, RelativeAngleCondition,
    RelativeClearanceCondition, RelativeLaneRange, RelativeSpeedCondition, SpeedCondition,
    StandStillCondition, TimeHeadwayCondition, TimeToCollisionCondition, TimeToCollisionTarget,
    TraveledDistanceCondition,
};
pub use value::{
    ByValueCondition, ParameterCondition, SimulationTimeCondition, StoryboardElementStateCondition,
    TimeOfDayCondition, TrafficSignalCondition, TrafficSignalControllerCondition,
    UserDefinedValueCondition, VariableCondition,
};

// The XSD `Condition` (`:953-961`) is modelled by `scenario::triggers::Condition`, which
// carries `@name`/`@conditionEdge`/`@delay` and the full ByEntity/ByValue choice. A local
// `Condition`/`ConditionWrapper` pair used to live here with an internally-tagged
// `#[serde(tag = "type")]` representation and only two variants; it matched no schema type
// and shadowed the real one on import, so it was removed.
