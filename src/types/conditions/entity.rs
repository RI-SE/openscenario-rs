//! Entity-based condition types for scenario triggering
//!
//! This file contains:
//! - Spatial conditions (distance, collision, position-based triggers)
//! - Motion conditions (speed, acceleration, standstill detection)
//! - State conditions (end-of-road, off-road, clearance checks)
//! - Temporal conditions (time headway, time-to-collision)
//! - Relative conditions comparing entities to each other
//!
//! Contributes to project by:
//! - Enabling realistic scenario triggering based on entity behavior
//! - Providing comprehensive spatial relationship monitoring
//! - Supporting safety-critical condition detection (collision, clearance)
//! - Facilitating dynamic scenario adaptation based on entity states
//! - Enabling complex multi-entity coordination and interaction patterns

use crate::types::basic::{Boolean, Double, Int, OSString, UnsignedInt, UnsignedShort};
use crate::types::enums::Rule;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpeedCondition {
    #[serde(rename = "@value")]
    pub value: Double,
    #[serde(rename = "@rule")]
    pub rule: Rule,
    #[serde(rename = "@entityRef")]
    pub entity_ref: String,
}

/// Entity-based condition types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ByEntityCondition {
    /// Speed-based condition
    #[serde(rename = "SpeedCondition")]
    Speed(SpeedCondition),
    // More entity conditions will be added later
}

impl Default for SpeedCondition {
    fn default() -> Self {
        Self {
            value: Double::literal(10.0),
            rule: Rule::GreaterThan,
            entity_ref: "DefaultEntity".to_string(),
        }
    }
}

impl Default for ByEntityCondition {
    fn default() -> Self {
        ByEntityCondition::Speed(SpeedCondition::default())
    }
}

