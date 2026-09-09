//! Weather condition types for environmental simulation
//!
//! This file contains:
//! - Weather definition with atmospheric conditions and precipitation
//! - Sun positioning and lighting conditions with intensity and angles
//! - Fog conditions with visibility parameters
//! - Precipitation types (rain, snow, dry) with intensity specifications
//!
use crate::types::basic::Double;
use crate::types::enums::{CloudState, FractionalCloudCover, PrecipitationType};
use crate::types::geometry::BoundingBox;
use serde::{Deserialize, Serialize};

/// Weather conditions container
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Weather {
    /// Deprecated cloud state; prefer `fractional_cloud_cover`.
    #[allow(deprecated)]
    #[serde(
        rename = "@cloudState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cloud_state: Option<CloudState>,
    #[serde(
        rename = "@atmosphericPressure",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub atmospheric_pressure: Option<Double>,
    #[serde(
        rename = "@temperature",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub temperature: Option<Double>,
    #[serde(
        rename = "@fractionalCloudCover",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fractional_cloud_cover: Option<FractionalCloudCover>,
    #[serde(rename = "Sun", default, skip_serializing_if = "Option::is_none")]
    pub sun: Option<Sun>,
    #[serde(rename = "Fog", default, skip_serializing_if = "Option::is_none")]
    pub fog: Option<Fog>,
    #[serde(
        rename = "Precipitation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub precipitation: Option<Precipitation>,
    #[serde(rename = "Wind", default, skip_serializing_if = "Option::is_none")]
    pub wind: Option<Wind>,
    #[serde(rename = "DomeImage", default, skip_serializing_if = "Option::is_none")]
    pub dome_image: Option<DomeImage>,
}

/// Sun lighting conditions and positioning
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Sun {
    /// Deprecated sun intensity; prefer `illuminance`.
    #[serde(
        rename = "@intensity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub intensity: Option<Double>,
    #[serde(rename = "@azimuth")]
    pub azimuth: Double,
    #[serde(rename = "@elevation")]
    pub elevation: Double,
    #[serde(
        rename = "@illuminance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub illuminance: Option<Double>,
}

/// Fog visibility conditions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Fog {
    #[serde(rename = "@visualRange")]
    pub visual_range: Double,
    #[serde(
        rename = "BoundingBox",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bounding_box: Option<BoundingBox>,
}

/// Precipitation conditions and intensity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Precipitation {
    #[serde(rename = "@precipitationType")]
    pub precipitation_type: PrecipitationType,
    /// Deprecated intensity; prefer `precipitation_intensity`.
    #[serde(
        rename = "@intensity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub intensity: Option<Double>,
    #[serde(
        rename = "@precipitationIntensity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub precipitation_intensity: Option<Double>,
}

/// Wind conditions (direction in radians, speed in m/s)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Wind {
    #[serde(rename = "@direction")]
    pub direction: Double,
    #[serde(rename = "@speed")]
    pub speed: Double,
}

/// Sky dome image reference
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DomeImage {
    #[serde(rename = "DomeFile")]
    pub dome_file: crate::types::entities::vehicle::File,
    #[serde(
        rename = "@azimuthOffset",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub azimuth_offset: Option<Double>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weather_empty_parses() {
        let xml = r#"<Weather/>"#;
        let w: Weather = quick_xml::de::from_str(xml).unwrap();
        assert!(w.cloud_state.is_none());
        assert!(w.sun.is_none());
        assert!(w.fog.is_none());
        assert!(w.precipitation.is_none());
        assert!(w.wind.is_none());
        assert!(w.dome_image.is_none());
    }

    #[test]
    fn test_weather_all_none_serializes_empty() {
        let w = Weather::default();
        let xml = quick_xml::se::to_string(&w).unwrap();
        assert!(!xml.contains('@'));
        assert!(!xml.contains("cloudState"));
        assert!(!xml.contains("<Sun"));
        assert!(!xml.contains("<Fog"));
        assert!(!xml.contains("<Precipitation"));
        assert!(!xml.contains("<Wind"));
        assert!(!xml.contains("<DomeImage"));
    }

    #[test]
    fn test_weather_roundtrip_fractional_cloud_cover() {
        let w = Weather {
            fractional_cloud_cover: Some(FractionalCloudCover::ThreeOktas),
            ..Default::default()
        };
        let xml = quick_xml::se::to_string(&w).unwrap();
        let deserialized: Weather = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(w, deserialized);
        assert!(deserialized.cloud_state.is_none());
    }

    #[test]
    fn test_weather_roundtrip_wind_and_dome_image() {
        let w = Weather {
            wind: Some(Wind {
                direction: Double::literal(1.0),
                speed: Double::literal(5.5),
            }),
            dome_image: Some(DomeImage {
                dome_file: crate::types::entities::vehicle::File {
                    filepath: "sky.hdr".to_string(),
                },
                azimuth_offset: Some(Double::literal(0.2)),
            }),
            ..Default::default()
        };
        let xml = quick_xml::se::to_string(&w).unwrap();
        let deserialized: Weather = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(w, deserialized);
    }

    #[test]
    #[allow(clippy::approx_constant)] // 3.14 is a sun azimuth scenario value, not an approximation of PI
    fn test_custom_rain_weather() {
        let w = Weather {
            sun: Some(Sun {
                intensity: Some(Double::literal(0.3)),
                azimuth: Double::literal(3.14),
                elevation: Double::literal(0.5),
                illuminance: None,
            }),
            fog: Some(Fog {
                visual_range: Double::literal(500.0),
                bounding_box: None,
            }),
            precipitation: Some(Precipitation {
                precipitation_type: PrecipitationType::Rain,
                intensity: None,
                precipitation_intensity: Some(Double::literal(0.8)),
            }),
            ..Default::default()
        };
        assert_eq!(
            w.precipitation.as_ref().unwrap().precipitation_type,
            PrecipitationType::Rain
        );
        assert_eq!(
            w.precipitation
                .as_ref()
                .unwrap()
                .precipitation_intensity
                .as_ref()
                .unwrap()
                .as_literal(),
            Some(&0.8)
        );
        assert_eq!(
            w.fog.as_ref().unwrap().visual_range.as_literal(),
            Some(&500.0)
        );
    }
}
