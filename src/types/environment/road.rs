//! Road condition and infrastructure types
//!
//! This file contains:
//! - RoadCondition with friction scale factors for surface properties
//! - Road surface properties affecting vehicle dynamics
//!
//! Contributes to project by:
//! - Supporting realistic road surface simulation and vehicle dynamics
//! - Enabling friction-dependent scenario validation
//! - Providing road condition setup for OpenSCENARIO scenarios
//! - Supporting variable road conditions for weather-dependent testing

use crate::types::basic::Double;
use serde::{Deserialize, Serialize};

/// Road surface conditions affecting vehicle dynamics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoadCondition {
    #[serde(rename = "@frictionScaleFactor")]
    pub friction_scale_factor: Double,
}

impl Default for RoadCondition {
    fn default() -> Self {
        Self {
            friction_scale_factor: Double::literal(1.0), // Normal dry road conditions
        }
    }
}

