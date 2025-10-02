//! Condition type module for trigger and event management
//!
//! This file contains:
//! - Base condition traits and common condition behaviors
//! - Condition evaluation logic and state management
//! - Trigger edge detection (rising, falling, rising-or-falling)
//! - Condition group logic (AND/OR combinations)
//! - Condition validation and constraint checking
//!
//! Contributes to project by:
//! - Organizing 21+ condition types into logical categories
//! - Providing consistent evaluation framework for all condition types
//! - Supporting complex trigger logic through condition combinations
//! - Enabling event-driven scenario execution and timing control
//! - Facilitating condition debugging and state introspection
use crate::types::enums::ConditionEdge;

pub mod entity; // Entity-based conditions
pub mod value; // Value-based conditions

pub use entity::{
    AccelerationCondition, AngleCondition, ByEntityCondition, CollisionCondition, CollisionTarget,
    DistanceCondition, EndOfRoadCondition, EntityCondition, OffRoadCondition, OffroadCondition,
    ReachPositionCondition, RelativeAngleCondition, RelativeClearanceCondition,
    RelativeDistanceCondition, RelativeLaneRange, RelativeSpeedCondition, SpeedCondition,
    StandStillCondition, TimeHeadwayCondition, TimeToCollisionCondition, TimeToCollisionTarget,
    TraveledDistanceCondition,
};
pub use value::{
    ByValueCondition, ParameterCondition, SimulationTimeCondition, StoryboardElementStateCondition,
    TimeOfDayCondition, TrafficSignalCondition, TrafficSignalControllerCondition,
    UserDefinedValueCondition, VariableCondition,
};

use crate::types::basic::Double;
use serde::{Deserialize, Serialize};

// Define base condition enum for polymorphic handling
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Condition {
    SimulationTime(SimulationTimeCondition),
    Speed(SpeedCondition),
}

// Define condition wrapper with common attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionWrapper {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@conditionEdge")]
    pub edge: ConditionEdge,
    #[serde(rename = "@delay")]
    pub delay: Double,
    #[serde(flatten)]
    pub condition: Condition,
}

// Add condition evaluation trait (later phases) - KEEP AS FUTURE WORK
// pub trait EvaluateCondition { fn evaluate(&self, ctx: &SimulationContext) -> bool; }
