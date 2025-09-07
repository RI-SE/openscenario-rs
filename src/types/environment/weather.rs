//! Weather condition types for environmental simulation
//!
//! This file contains:
//! - Weather definition with atmospheric conditions and precipitation
//! - Sun positioning and lighting conditions with intensity and angles
//! - Fog conditions with visibility parameters
//! - Precipitation types (rain, snow, dry) with intensity specifications
//!
//! Contributes to project by:
//! - Enabling complete weather setup for OpenSCENARIO scenarios
//! - Supporting realistic lighting and atmospheric conditions
//! - Providing weather-dependent scenario validation
//! - Facilitating integration with weather simulation systems

use serde::{Deserialize, Serialize};
use crate::types::basic::Double;

/// Weather conditions container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weather {
    #[serde(rename = "@cloudState")]
    pub cloud_state: String, // "free", "cloudy", "overcast", "rainy"
    #[serde(rename = "Sun")]
    pub sun: Sun,
    #[serde(rename = "Fog")]
    pub fog: Fog,
    #[serde(rename = "Precipitation")]
    pub precipitation: Precipitation,
}

/// Sun lighting conditions and positioning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sun {
    #[serde(rename = "@intensity")]
    pub intensity: Double,
    #[serde(rename = "@azimuth")]
    pub azimuth: Double,
    #[serde(rename = "@elevation")]
    pub elevation: Double,
}

/// Fog visibility conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fog {
    #[serde(rename = "@visualRange")]
    pub visual_range: Double,
}

/// Precipitation conditions and intensity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Precipitation {
    #[serde(rename = "@precipitationType")]
    pub precipitation_type: String, // "dry", "rain", "snow"
    #[serde(rename = "@intensity")]
    pub intensity: Double,
}

impl Default for Weather {
    fn default() -> Self {
        Self {
            cloud_state: "free".to_string(),
            sun: Sun::default(),
            fog: Fog::default(),
            precipitation: Precipitation::default(),
        }
    }
}

impl Default for Sun {
    fn default() -> Self {
        Self {
            intensity: Double::literal(1.0),
            azimuth: Double::literal(0.0),
            elevation: Double::literal(1.571), // ~π/2 radians (90 degrees)
        }
    }
}

impl Default for Fog {
    fn default() -> Self {
        Self {
            visual_range: Double::literal(100000.0), // 100km clear visibility
        }
    }
}

impl Default for Precipitation {
    fn default() -> Self {
        Self {
            precipitation_type: "dry".to_string(),
            intensity: Double::literal(0.0),
        }
    }
}