//! Weather condition types for environmental simulation
//!
//! This file contains:
//! - Weather definition with atmospheric conditions and precipitation
//! - Sun positioning and lighting conditions with intensity and angles
//! - Fog conditions with visibility parameters
//! - Precipitation types (rain, snow, dry) with intensity specifications
//!
use crate::types::basic::Double;
use serde::{Deserialize, Serialize};

/// Weather conditions container
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Sun {
    #[serde(rename = "@intensity")]
    pub intensity: Double,
    #[serde(rename = "@azimuth")]
    pub azimuth: Double,
    #[serde(rename = "@elevation")]
    pub elevation: Double,
}

/// Fog visibility conditions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Fog {
    #[serde(rename = "@visualRange")]
    pub visual_range: Double,
}

/// Precipitation conditions and intensity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weather_default_is_clear() {
        let w = Weather::default();
        assert_eq!(w.cloud_state, "free");
        assert_eq!(w.precipitation.precipitation_type, "dry");
        assert_eq!(w.precipitation.intensity.as_literal(), Some(&0.0));
        assert_eq!(w.fog.visual_range.as_literal(), Some(&100000.0));
    }

    #[test]
    fn test_sun_default_noon_position() {
        let s = Sun::default();
        assert_eq!(s.intensity.as_literal(), Some(&1.0));
        assert_eq!(s.elevation.as_literal(), Some(&1.571));
    }

    #[test]
    fn test_weather_xml_roundtrip() {
        let w = Weather::default();
        let xml = quick_xml::se::to_string(&w).unwrap();
        let deserialized: Weather = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(w, deserialized);
    }

    #[test]
    fn test_custom_rain_weather() {
        let w = Weather {
            cloud_state: "rainy".to_string(),
            sun: Sun {
                intensity: Double::literal(0.3),
                azimuth: Double::literal(3.14),
                elevation: Double::literal(0.5),
            },
            fog: Fog {
                visual_range: Double::literal(500.0),
            },
            precipitation: Precipitation {
                precipitation_type: "rain".to_string(),
                intensity: Double::literal(0.8),
            },
        };
        assert_eq!(w.precipitation.precipitation_type, "rain");
        assert_eq!(w.precipitation.intensity.as_literal(), Some(&0.8));
        assert_eq!(w.fog.visual_range.as_literal(), Some(&500.0));
    }
}
