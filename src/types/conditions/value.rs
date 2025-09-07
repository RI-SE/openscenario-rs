//! Value-based condition types for parameter and time-based triggering
//!
//! This file contains:
//! - Parameter conditions for scenario parameter monitoring
//! - Variable conditions for dynamic variable state checking
//! - Time-based conditions (simulation time, time-of-day)
//! - Storyboard element state conditions for execution flow control
//! - Traffic signal conditions for infrastructure interaction
//! - User-defined custom condition support
//!
//! Contributes to project by:
//! - Enabling scenario control through parameter and variable monitoring
//! - Supporting time-based scenario progression and scheduling
//! - Providing execution flow control through storyboard state tracking
//! - Facilitating infrastructure integration through signal state monitoring
//! - Enabling custom condition logic through extensible user-defined types

use crate::types::{Double};
use crate::types::enums::{Rule};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SimulationTimeCondition {
    #[serde(rename = "@value")]
    pub value: Double,
    #[serde(rename = "@rule")]
    pub rule: Rule,
}

/// Value-based condition types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "PascalCase")]
pub enum ByValueCondition {
    /// Simulation time-based condition
    #[serde(rename = "SimulationTimeCondition")]
    SimulationTime(SimulationTimeCondition),
    // More value conditions will be added later
}

impl Default for SimulationTimeCondition {
    fn default() -> Self {
        Self {
            value: Double::literal(10.0),
            rule: Rule::GreaterThan,
        }
    }
}

impl Default for ByValueCondition {
    fn default() -> Self {
        ByValueCondition::SimulationTime(SimulationTimeCondition::default())
    }
}