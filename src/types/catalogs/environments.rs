//! Environment catalog types for OpenSCENARIO reusable environment definitions
//!
//! This module contains catalog-specific environment types that enable reuse of
//! environment configurations across multiple scenarios with parameter substitution.

use crate::types::basic::{Boolean, Double, OSString, ParameterDeclarations, Value};
use crate::types::environment::{
    Environment, Fog, Precipitation, RoadCondition, Sun, TimeOfDay, Weather,
};
use crate::types::enums::{CloudState, FractionalCloudCover, Wetness};
use serde::{Deserialize, Serialize};

/// Environment definition within a catalog
///
/// Extends the base Environment type with catalog-specific functionality
/// including parameter declarations and reusable weather/lighting configurations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Environment")]
pub struct CatalogEnvironment {
    /// Unique name for this environment in the catalog
    #[serde(rename = "@name")]
    pub name: String,

    /// Parameter declarations for this environment
    #[serde(
        rename = "ParameterDeclarations",
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_declarations: Option<ParameterDeclarations>,

    /// Time of day configuration (can be parameterized) — optional per XSD
    #[serde(rename = "TimeOfDay", default, skip_serializing_if = "Option::is_none")]
    pub time_of_day: Option<CatalogTimeOfDay>,

    /// Weather conditions (can be parameterized) — optional per XSD
    #[serde(rename = "Weather", default, skip_serializing_if = "Option::is_none")]
    pub weather: Option<CatalogWeather>,

    /// Road conditions (can be parameterized) — optional per XSD
    #[serde(
        rename = "RoadCondition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub road_condition: Option<CatalogRoadCondition>,
}

/// Time of day configuration with parameterizable properties
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "TimeOfDay")]
pub struct CatalogTimeOfDay {
    /// Whether time animation is enabled (can be parameterized)
    #[serde(rename = "@animation")]
    pub animation: Boolean,

    /// Date and time in ISO 8601 format (can be parameterized)
    #[serde(rename = "@dateTime")]
    pub date_time: OSString,
}

impl Default for CatalogTimeOfDay {
    fn default() -> Self {
        Self {
            animation: Value::Literal(false),
            date_time: Value::Literal("2021-01-01T12:00:00".to_string()),
        }
    }
}

/// Weather conditions with parameterizable atmospheric properties
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename = "Weather")]
pub struct CatalogWeather {
    /// Cloud state (can be parameterized) — deprecated per XSD
    #[serde(
        rename = "@cloudState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[allow(deprecated)]
    pub cloud_state: Option<CloudState>,

    /// Atmospheric pressure in hPa (optional, can be parameterized)
    #[serde(
        rename = "@atmosphericPressure",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub atmospheric_pressure: Option<Double>,

    /// Temperature in Kelvin (optional, can be parameterized)
    #[serde(rename = "@temperature", default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<Double>,

    /// Fractional cloud cover (optional) — XSD `@fractionalCloudCover` attribute
    #[serde(
        rename = "@fractionalCloudCover",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fractional_cloud_cover: Option<FractionalCloudCover>,

    /// Sun lighting conditions
    #[serde(rename = "Sun", default, skip_serializing_if = "Option::is_none")]
    pub sun: Option<CatalogSun>,

    /// Fog conditions
    #[serde(rename = "Fog", default, skip_serializing_if = "Option::is_none")]
    pub fog: Option<CatalogFog>,

    /// Precipitation conditions
    #[serde(
        rename = "Precipitation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub precipitation: Option<CatalogPrecipitation>,

    /// Wind conditions
    #[serde(rename = "Wind", default, skip_serializing_if = "Option::is_none")]
    pub wind: Option<crate::types::environment::weather::Wind>,

    /// Sky dome image reference
    #[serde(rename = "DomeImage", default, skip_serializing_if = "Option::is_none")]
    pub dome_image: Option<crate::types::environment::weather::DomeImage>,
}

/// Sun lighting configuration with parameterizable properties
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Sun")]
pub struct CatalogSun {
    /// Light intensity (0.0-1.0, can be parameterized) — deprecated per XSD
    #[serde(rename = "@intensity", default, skip_serializing_if = "Option::is_none")]
    pub intensity: Option<Double>,

    /// Sun azimuth angle in radians (can be parameterized)
    #[serde(rename = "@azimuth")]
    pub azimuth: Double,

    /// Sun elevation angle in radians (can be parameterized)
    #[serde(rename = "@elevation")]
    pub elevation: Double,

    /// Illuminance in lux (optional, current replacement for `intensity`)
    #[serde(rename = "@illuminance", default, skip_serializing_if = "Option::is_none")]
    pub illuminance: Option<Double>,
}

/// Fog conditions with parameterizable visibility
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Fog")]
pub struct CatalogFog {
    /// Visual range in meters (can be parameterized)
    #[serde(rename = "@visualRange")]
    pub visual_range: Double,

    /// Optional fog bounding box for localized fog
    #[serde(rename = "BoundingBox", skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<crate::types::geometry::BoundingBox>,
}

impl Default for CatalogFog {
    fn default() -> Self {
        Self {
            visual_range: Value::Literal(100000.0), // 100km clear visibility
            bounding_box: None,
        }
    }
}

/// Precipitation conditions with parameterizable intensity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Precipitation")]
pub struct CatalogPrecipitation {
    /// Type of precipitation (can be parameterized)
    #[serde(rename = "@precipitationType")]
    pub precipitation_type: OSString,

    /// Precipitation intensity (0.0-1.0, can be parameterized) — deprecated per XSD
    #[serde(rename = "@intensity", default, skip_serializing_if = "Option::is_none")]
    pub intensity: Option<Double>,

    /// Precipitation intensity (current replacement for `intensity`)
    #[serde(
        rename = "@precipitationIntensity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub precipitation_intensity: Option<Double>,
}

/// Road conditions with parameterizable surface properties
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "RoadCondition")]
pub struct CatalogRoadCondition {
    /// Friction scale factor (can be parameterized)
    #[serde(rename = "@frictionScaleFactor")]
    pub friction_scale_factor: Double,

    /// Optional wetness factor (can be parameterized)
    #[serde(rename = "@wetness", skip_serializing_if = "Option::is_none")]
    pub wetness: Option<Wetness>,

    /// Optional properties
    #[serde(
        rename = "Properties",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub properties: Option<crate::types::entities::vehicle::Properties>,
}

// Implementation methods for catalog environments

impl CatalogEnvironment {
    /// Creates a new catalog environment with the specified name
    pub fn new(name: String) -> Self {
        Self {
            name,
            parameter_declarations: None,
            time_of_day: None,
            weather: None,
            road_condition: None,
        }
    }

    /// Creates a catalog environment with parameter declarations
    pub fn with_parameters(name: String, parameters: ParameterDeclarations) -> Self {
        Self {
            name,
            parameter_declarations: Some(parameters),
            time_of_day: None,
            weather: None,
            road_condition: None,
        }
    }

    /// Sets the time of day for this environment
    pub fn set_time_of_day(&mut self, time_of_day: CatalogTimeOfDay) {
        self.time_of_day = Some(time_of_day);
    }

    /// Sets the weather conditions for this environment
    pub fn set_weather(&mut self, weather: CatalogWeather) {
        self.weather = Some(weather);
    }

    /// Sets the road conditions for this environment
    pub fn set_road_condition(&mut self, road_condition: CatalogRoadCondition) {
        self.road_condition = Some(road_condition);
    }

    /// Converts this catalog environment to a scenario environment, resolving
    /// every parameterizable value against `parameters`.
    pub fn resolve_environment(
        &self,
        parameters: &std::collections::HashMap<String, String>,
    ) -> crate::error::Result<Environment> {
        let time_of_day = self
            .time_of_day
            .as_ref()
            .map(|tod| -> crate::error::Result<TimeOfDay> {
                Ok(TimeOfDay {
                    animation: Boolean::literal(tod.animation.resolve(parameters)?),
                    date_time: tod.date_time.resolve(parameters)?,
                })
            })
            .transpose()?;

        let weather = self
            .weather
            .as_ref()
            .map(|w| -> crate::error::Result<Weather> {
                let precipitation = w
                    .precipitation
                    .as_ref()
                    .map(|p| -> crate::error::Result<Precipitation> {
                        let type_str = p.precipitation_type.resolve(parameters)?;
                        let precipitation_type = match type_str.as_str() {
                            "rain" => crate::types::enums::PrecipitationType::Rain,
                            "snow" => crate::types::enums::PrecipitationType::Snow,
                            "dry" => crate::types::enums::PrecipitationType::Dry,
                            other => {
                                return Err(crate::error::Error::invalid_value(
                                    "precipitationType",
                                    other,
                                    "must be one of: dry, rain, snow",
                                ))
                            }
                        };

                        Ok(Precipitation {
                            precipitation_type,
                            intensity: None,
                            precipitation_intensity: p
                                .precipitation_intensity
                                .clone()
                                .or_else(|| p.intensity.clone()),
                        })
                    })
                    .transpose()?;

                Ok(Weather {
                    cloud_state: w.cloud_state.clone(),
                    atmospheric_pressure: w.atmospheric_pressure.clone(),
                    temperature: w.temperature.clone(),
                    fractional_cloud_cover: w.fractional_cloud_cover.clone(),
                    sun: w.sun.as_ref().map(|s| Sun {
                        intensity: s.intensity.clone(),
                        azimuth: s.azimuth.clone(),
                        elevation: s.elevation.clone(),
                        illuminance: s.illuminance.clone(),
                    }),
                    fog: w.fog.as_ref().map(|f| Fog {
                        visual_range: f.visual_range.clone(),
                        bounding_box: f.bounding_box.clone(),
                    }),
                    precipitation,
                    wind: w.wind.clone(),
                    dome_image: w.dome_image.clone(),
                })
            })
            .transpose()?;

        let road_condition = self
            .road_condition
            .as_ref()
            .map(|rc| -> crate::error::Result<RoadCondition> {
                Ok(RoadCondition {
                    friction_scale_factor: Double::literal(
                        rc.friction_scale_factor.resolve(parameters)?,
                    ),
                    wetness: rc.wetness.clone(),
                    properties: rc.properties.clone(),
                })
            })
            .transpose()?;

        Ok(Environment {
            name: OSString::literal(crate::types::catalogs::entities::resolve_parameter(
                &self.name, parameters,
            )?),
            parameter_declarations: self.parameter_declarations.clone(),
            time_of_day,
            weather,
            road_condition,
        })
    }
}

impl CatalogTimeOfDay {
    /// Creates a time of day with the specified date-time
    pub fn new(date_time: OSString) -> Self {
        Self {
            animation: Value::Literal(false),
            date_time,
        }
    }

    /// Creates an animated time of day
    pub fn with_animation(date_time: OSString, animation: Boolean) -> Self {
        Self {
            animation,
            date_time,
        }
    }
}

impl CatalogWeather {
    /// Creates weather with the specified cloud state
    #[allow(deprecated)]
    pub fn new(cloud_state: CloudState) -> Self {
        Self {
            cloud_state: Some(cloud_state),
            atmospheric_pressure: None,
            temperature: None,
            fractional_cloud_cover: None,
            sun: Some(CatalogSun {
                intensity: Some(Value::Literal(1.0)),
                azimuth: Value::Literal(0.0),
                elevation: Value::Literal(1.571),
                illuminance: None,
            }),
            fog: Some(CatalogFog::default()),
            precipitation: Some(CatalogPrecipitation {
                precipitation_type: Value::Literal("dry".to_string()),
                intensity: Some(Value::Literal(0.0)),
                precipitation_intensity: None,
            }),
            wind: None,
            dome_image: None,
        }
    }

    /// Creates sunny weather conditions
    #[allow(deprecated)]
    pub fn sunny() -> Self {
        Self {
            cloud_state: Some(CloudState::Free),
            atmospheric_pressure: None,
            temperature: None,
            fractional_cloud_cover: None,
            sun: Some(CatalogSun {
                intensity: Some(Value::Literal(1.0)),
                azimuth: Value::Literal(0.0),
                elevation: Value::Literal(1.571),
                illuminance: None,
            }),
            fog: Some(CatalogFog {
                visual_range: Value::Literal(100000.0),
                bounding_box: None,
            }),
            precipitation: Some(CatalogPrecipitation {
                precipitation_type: Value::Literal("dry".to_string()),
                intensity: Some(Value::Literal(0.0)),
                precipitation_intensity: None,
            }),
            wind: None,
            dome_image: None,
        }
    }

    /// Creates rainy weather conditions
    #[allow(deprecated)]
    pub fn rainy(intensity: Double) -> Self {
        Self {
            cloud_state: Some(CloudState::Rainy),
            atmospheric_pressure: None,
            temperature: None,
            fractional_cloud_cover: None,
            sun: Some(CatalogSun {
                intensity: Some(Value::Literal(0.3)),
                azimuth: Value::Literal(0.0),
                elevation: Value::Literal(1.571),
                illuminance: None,
            }),
            fog: Some(CatalogFog {
                visual_range: Value::Literal(5000.0), // Reduced visibility in rain
                bounding_box: None,
            }),
            precipitation: Some(CatalogPrecipitation {
                precipitation_type: Value::Literal("rain".to_string()),
                intensity: Some(intensity),
                precipitation_intensity: None,
            }),
            wind: None,
            dome_image: None,
        }
    }
}

/// Catalog entity integration so `CatalogEnvironment` can be used as the entry type
/// in `CatalogContent` and behind a `CatalogReference`.
impl crate::types::catalogs::entities::CatalogEntity for CatalogEnvironment {
    type ResolvedType = Environment;

    fn into_scenario_entity(
        self,
        parameters: std::collections::HashMap<String, String>,
    ) -> crate::error::Result<Self::ResolvedType> {
        self.resolve_environment(&parameters)
    }

    fn parameter_schema() -> Vec<crate::types::catalogs::entities::ParameterDefinition> {
        vec![
            crate::types::catalogs::entities::ParameterDefinition {
                name: "TimeOfDay".to_string(),
                parameter_type: "String".to_string(),
                default_value: Some("12:00:00".to_string()),
                description: Some("Time of day in HH:MM:SS format".to_string()),
            },
            crate::types::catalogs::entities::ParameterDefinition {
                name: "WeatherCondition".to_string(),
                parameter_type: "String".to_string(),
                default_value: Some("dry".to_string()),
                description: Some("Weather condition (dry, wet, snow, fog)".to_string()),
            },
            crate::types::catalogs::entities::ParameterDefinition {
                name: "RoadCondition".to_string(),
                parameter_type: "String".to_string(),
                default_value: Some("dry".to_string()),
                description: Some("Road surface condition (dry, wet, snow, ice)".to_string()),
            },
        ]
    }

    fn entity_name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::basic::ParameterDeclaration;
    use crate::types::enums::ParameterType;

    #[test]
    fn test_catalog_environment_creation() {
        let environment = CatalogEnvironment::new("TestEnvironment".to_string());

        assert_eq!(environment.name, "TestEnvironment");
        assert!(environment.parameter_declarations.is_none());
        assert!(environment.time_of_day.is_none());
    }

    #[test]
    fn test_catalog_weather_presets() {
        let sunny = CatalogWeather::sunny();
        let rainy = CatalogWeather::rainy(Value::Literal(0.8));

        assert_eq!(sunny.cloud_state, Some(CloudState::Free));
        assert_eq!(
            sunny.sun.as_ref().unwrap().intensity.as_ref().unwrap().as_literal().unwrap(),
            &1.0
        );
        assert_eq!(
            sunny
                .precipitation
                .as_ref()
                .unwrap()
                .precipitation_type
                .as_literal()
                .unwrap(),
            "dry"
        );

        assert_eq!(rainy.cloud_state, Some(CloudState::Rainy));
        assert_eq!(
            rainy
                .precipitation
                .as_ref()
                .unwrap()
                .precipitation_type
                .as_literal()
                .unwrap(),
            "rain"
        );
        assert_eq!(
            rainy
                .precipitation
                .as_ref()
                .unwrap()
                .intensity
                .as_ref()
                .unwrap()
                .as_literal()
                .unwrap(),
            &0.8
        );
    }

    #[test]
    fn test_time_of_day_configuration() {
        let tod1 = CatalogTimeOfDay::new(Value::Parameter("startTime".to_string()));
        let tod2 = CatalogTimeOfDay::with_animation(
            Value::Literal("2021-06-21T06:00:00".to_string()),
            Value::Literal(true),
        );

        assert!(matches!(tod1.date_time, Value::Parameter(_)));
        assert_eq!(tod1.animation.as_literal().unwrap(), &false);

        assert_eq!(tod2.date_time.as_literal().unwrap(), "2021-06-21T06:00:00");
        assert_eq!(tod2.animation.as_literal().unwrap(), &true);
    }

    #[test]
    fn test_environment_with_parameters() {
        let param_decl = ParameterDeclarations {
            parameter_declarations: vec![ParameterDeclaration {
                name: OSString::literal("visibility".to_string()),
                parameter_type: ParameterType::Double,
                value: OSString::literal("10000.0".to_string()),
                constraint_groups: Vec::new(),
            }],
        };

        let mut environment =
            CatalogEnvironment::with_parameters("ParameterizedEnvironment".to_string(), param_decl);

        // Set fog with parameterized visibility
        let fog = CatalogFog {
            visual_range: Value::Parameter("visibility".to_string()),
            bounding_box: None,
        };
        let mut weather = CatalogWeather::default();
        weather.fog = Some(fog);
        environment.set_weather(weather);

        assert_eq!(environment.name, "ParameterizedEnvironment");
        assert!(environment.parameter_declarations.is_some());
        assert!(matches!(
            environment.weather.as_ref().unwrap().fog.as_ref().unwrap().visual_range,
            Value::Parameter(_)
        ));
    }

    #[test]
    fn test_road_condition_parameters() {
        let road_condition = CatalogRoadCondition {
            friction_scale_factor: Value::Parameter("frictionFactor".to_string()),
            wetness: Some(Wetness::Moist),
            properties: None,
        };

        assert!(matches!(
            road_condition.friction_scale_factor,
            Value::Parameter(_)
        ));
        assert_eq!(road_condition.wetness, Some(Wetness::Moist));
    }

    #[test]
    fn test_resolve_environment() {
        let mut catalog_env = CatalogEnvironment::new("TestEnvironment".to_string());

        // Set up a sunny day environment
        catalog_env.set_weather(CatalogWeather::sunny());
        catalog_env.set_time_of_day(CatalogTimeOfDay::new(Value::Literal(
            "2021-06-21T12:00:00".to_string(),
        )));

        let scenario_env = catalog_env
            .resolve_environment(&std::collections::HashMap::new())
            .unwrap();

        assert_eq!(scenario_env.name.as_literal().unwrap(), "TestEnvironment");
        assert_eq!(
            scenario_env.time_of_day.as_ref().unwrap().date_time,
            "2021-06-21T12:00:00"
        );
        assert_eq!(
            scenario_env.weather.as_ref().unwrap().cloud_state,
            Some(CloudState::Free)
        );
        assert_eq!(
            scenario_env
                .weather
                .as_ref()
                .unwrap()
                .sun
                .as_ref()
                .unwrap()
                .intensity
                .as_ref()
                .unwrap()
                .as_literal()
                .unwrap(),
            &1.0
        );
    }

    /// `CatalogEnvironment` resolves to a real scenario `Environment`, with
    /// parameterized values substituted from the assignment map.
    #[test]
    fn test_catalog_environment_into_scenario_entity() {
        use crate::types::catalogs::entities::CatalogEntity;

        let mut catalog_env = CatalogEnvironment::new("Rainy".to_string());
        catalog_env.set_weather(CatalogWeather::rainy(Value::Parameter(
            "rainIntensity".to_string(),
        )));
        catalog_env.set_road_condition(CatalogRoadCondition {
            friction_scale_factor: Value::Parameter("friction".to_string()),
            wetness: Some(Wetness::Moist),
            properties: None,
        });

        let mut parameters = std::collections::HashMap::new();
        parameters.insert("friction".to_string(), "0.6".to_string());

        let resolved = catalog_env.into_scenario_entity(parameters).unwrap();

        assert_eq!(resolved.name.as_literal().unwrap(), "Rainy");
        let weather = resolved.weather.as_ref().unwrap();
        assert_eq!(
            weather
                .precipitation
                .as_ref()
                .unwrap()
                .precipitation_type,
            crate::types::enums::PrecipitationType::Rain
        );
        assert_eq!(weather.fog.as_ref().unwrap().visual_range, Value::Literal(5000.0));

        let road_condition = resolved.road_condition.as_ref().unwrap();
        assert_eq!(
            road_condition.friction_scale_factor.as_literal().unwrap(),
            &0.6
        );
        assert_eq!(road_condition.wetness, Some(Wetness::Moist));
    }

    #[test]
    fn test_defaults() {
        let environment = CatalogEnvironment::new("DefaultCatalogEnvironment".to_string());
        let weather = CatalogWeather::default();

        assert_eq!(environment.name, "DefaultCatalogEnvironment");
        assert!(environment.time_of_day.is_none());
        assert!(weather.cloud_state.is_none());
        assert!(weather.sun.is_none());
    }

    /// Regression: `fractionalCloudCover` is an XSD attribute on `Weather`
    /// (:2564-2578), not a child element. Also verify `Wind`/`DomeImage`
    /// round-trip through the catalog weather type.
    #[test]
    fn test_catalog_weather_fractional_cloud_cover_is_attribute() {
        let xml = r#"<Weather fractionalCloudCover="threeOktas">
    <Wind direction="1.0" speed="5.5"/>
    <DomeImage azimuthOffset="0.2">
        <DomeFile filepath="sky.hdr"/>
    </DomeImage>
</Weather>"#;
        let weather: CatalogWeather = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(
            weather.fractional_cloud_cover,
            Some(FractionalCloudCover::ThreeOktas)
        );
        assert!(weather.wind.is_some());
        assert!(weather.dome_image.is_some());

        let serialized = quick_xml::se::to_string(&weather).unwrap();
        assert!(
            serialized.contains(r#"fractionalCloudCover="threeOktas""#),
            "serialized as attribute: {serialized}"
        );
        assert!(!serialized.contains("<FractionalCloudCover"), "serialized: {serialized}");

        let reparsed: CatalogWeather = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(weather, reparsed);
    }
}
