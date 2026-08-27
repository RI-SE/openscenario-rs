//! Environment catalog types for OpenSCENARIO reusable environment definitions
//!
//! This module contains catalog-specific environment types that enable reuse of
//! environment configurations across multiple scenarios with parameter substitution.

use crate::types::basic::{Boolean, Double, Int, OSString, ParameterDeclarations, Value};
use crate::types::environment::{
    Environment, Fog, Precipitation, RoadCondition, Sun, TimeOfDay, Weather,
};
use crate::types::enums::{CloudState, FractionalCloudCover, Wetness};
use serde::{Deserialize, Serialize};

/// Environment catalog containing reusable environment definitions
///
/// Represents a collection of environment configurations that can be referenced
/// from scenarios, enabling modular environment design and weather/lighting reuse.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "EnvironmentCatalog")]
pub struct EnvironmentCatalog {
    /// Version information for catalog compatibility
    #[serde(rename = "@revMajor")]
    pub rev_major: Int,

    #[serde(rename = "@revMinor")]
    pub rev_minor: Int,

    /// Collection of environment entries in this catalog
    #[serde(rename = "Environment")]
    pub environments: Vec<CatalogEnvironment>,
}

impl Default for EnvironmentCatalog {
    fn default() -> Self {
        Self {
            rev_major: Value::Literal(1),
            rev_minor: Value::Literal(0),
            environments: Vec::new(),
        }
    }
}

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

    /// Optional road network reference
    #[serde(rename = "RoadNetwork", skip_serializing_if = "Option::is_none")]
    pub road_network: Option<RoadNetworkReference>,
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

    /// Fractional cloud cover (optional)
    #[serde(
        rename = "FractionalCloudCover",
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

/// Reference to a road network file
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "RoadNetwork")]
pub struct RoadNetworkReference {
    /// Path to the road network file (OpenDRIVE format)
    #[serde(rename = "@logicFile")]
    pub logic_file: OSString,

    /// Optional path to visual geometry file
    #[serde(rename = "@sceneGraphFile", skip_serializing_if = "Option::is_none")]
    pub scene_graph_file: Option<OSString>,

    /// Traffic signals reference (optional)
    #[serde(rename = "TrafficSignals", skip_serializing_if = "Option::is_none")]
    pub traffic_signals: Option<TrafficSignalsReference>,
}

/// Reference to traffic signals configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "TrafficSignals")]
pub struct TrafficSignalsReference {
    /// Traffic signal controller configuration file
    #[serde(
        rename = "@trafficSignalControllerFile",
        skip_serializing_if = "Option::is_none"
    )]
    pub controller_file: Option<OSString>,
}

// Implementation methods for catalog environments

impl EnvironmentCatalog {
    /// Creates a new environment catalog with version information
    pub fn new(rev_major: i32, rev_minor: i32) -> Self {
        Self {
            rev_major: Value::Literal(rev_major),
            rev_minor: Value::Literal(rev_minor),
            environments: Vec::new(),
        }
    }

    /// Adds an environment to this catalog
    pub fn add_environment(&mut self, environment: CatalogEnvironment) {
        self.environments.push(environment);
    }

    /// Finds an environment by name in this catalog
    pub fn find_environment(&self, name: &str) -> Option<&CatalogEnvironment> {
        self.environments.iter().find(|e| e.name == name)
    }

    /// Gets all environment names in this catalog
    pub fn environment_names(&self) -> Vec<&str> {
        self.environments.iter().map(|e| e.name.as_str()).collect()
    }
}

impl CatalogEnvironment {
    /// Creates a new catalog environment with the specified name
    pub fn new(name: String) -> Self {
        Self {
            name,
            parameter_declarations: None,
            time_of_day: None,
            weather: None,
            road_condition: None,
            road_network: None,
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
            road_network: None,
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

    /// Sets the road network reference for this environment
    pub fn set_road_network(&mut self, road_network: RoadNetworkReference) {
        self.road_network = Some(road_network);
    }

    /// Converts this catalog environment to a scenario environment
    /// with parameter substitution (placeholder for future implementation)
    pub fn to_scenario_environment(&self) -> Environment {
        let time_of_day = self.time_of_day.as_ref().map(|tod| TimeOfDay {
            animation: Boolean::literal(tod.animation.as_literal().copied().unwrap_or(false)),
            date_time: tod
                .date_time
                .as_literal()
                .unwrap_or(&"2021-01-01T12:00:00".to_string())
                .clone(),
        });

        let weather = self.weather.as_ref().map(|w| {
            let precipitation_type = w.precipitation.as_ref().map(|p| {
                match p
                    .precipitation_type
                    .as_literal()
                    .map(|s| s.as_str())
                    .unwrap_or("dry")
                {
                    "rain" => crate::types::enums::PrecipitationType::Rain,
                    "snow" => crate::types::enums::PrecipitationType::Snow,
                    _ => crate::types::enums::PrecipitationType::Dry,
                }
            });

            Weather {
                cloud_state: None,
                atmospheric_pressure: None,
                temperature: None,
                fractional_cloud_cover: None,
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
                precipitation: w.precipitation.as_ref().map(|p| Precipitation {
                    precipitation_type: precipitation_type
                        .unwrap_or(crate::types::enums::PrecipitationType::Dry),
                    intensity: None,
                    precipitation_intensity: p.intensity.clone(),
                }),
                wind: None,
                dome_image: None,
            }
        });

        let road_condition = self.road_condition.as_ref().map(|rc| RoadCondition {
            friction_scale_factor: Double::literal(
                rc.friction_scale_factor.as_literal().copied().unwrap_or(1.0),
            ),
            wetness: rc.wetness.clone(),
            properties: rc.properties.clone(),
        });

        Environment {
            name: OSString::literal(self.name.clone()),
            parameter_declarations: None,
            time_of_day,
            weather,
            road_condition,
        }
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
        }
    }
}

impl RoadNetworkReference {
    /// Creates a road network reference with the specified logic file
    pub fn new(logic_file: OSString) -> Self {
        Self {
            logic_file,
            scene_graph_file: None,
            traffic_signals: None,
        }
    }

    /// Creates a road network reference with visual geometry
    pub fn with_scene_graph(logic_file: OSString, scene_graph_file: OSString) -> Self {
        Self {
            logic_file,
            scene_graph_file: Some(scene_graph_file),
            traffic_signals: None,
        }
    }

    /// Sets traffic signals configuration
    pub fn set_traffic_signals(&mut self, controller_file: OSString) {
        self.traffic_signals = Some(TrafficSignalsReference {
            controller_file: Some(controller_file),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::basic::ParameterDeclaration;
    use crate::types::enums::ParameterType;

    #[test]
    fn test_environment_catalog_creation() {
        let catalog = EnvironmentCatalog::new(1, 2);

        assert_eq!(catalog.rev_major.as_literal().unwrap(), &1);
        assert_eq!(catalog.rev_minor.as_literal().unwrap(), &2);
        assert!(catalog.environments.is_empty());
    }

    #[test]
    fn test_catalog_environment_creation() {
        let environment = CatalogEnvironment::new("TestEnvironment".to_string());

        assert_eq!(environment.name, "TestEnvironment");
        assert!(environment.parameter_declarations.is_none());
        assert!(environment.time_of_day.is_none());
    }

    #[test]
    fn test_environment_catalog_operations() {
        let mut catalog = EnvironmentCatalog::new(1, 0);
        let env1 = CatalogEnvironment::new("SunnyDay".to_string());
        let env2 = CatalogEnvironment::new("RainyNight".to_string());

        catalog.add_environment(env1);
        catalog.add_environment(env2);

        assert_eq!(catalog.environments.len(), 2);
        assert!(catalog.find_environment("SunnyDay").is_some());
        assert!(catalog.find_environment("RainyNight").is_some());
        assert!(catalog.find_environment("NonExistent").is_none());

        let names = catalog.environment_names();
        assert!(names.contains(&"SunnyDay"));
        assert!(names.contains(&"RainyNight"));
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
    fn test_road_network_reference() {
        let mut road_net = RoadNetworkReference::new(Value::Literal("road.xodr".to_string()));
        road_net.set_traffic_signals(Value::Literal("signals.xml".to_string()));

        assert_eq!(road_net.logic_file.as_literal().unwrap(), "road.xodr");
        assert!(road_net.traffic_signals.is_some());

        let with_graphics = RoadNetworkReference::with_scene_graph(
            Value::Literal("road.xodr".to_string()),
            Value::Literal("road.osgb".to_string()),
        );

        assert_eq!(
            with_graphics
                .scene_graph_file
                .as_ref()
                .unwrap()
                .as_literal()
                .unwrap(),
            "road.osgb"
        );
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
    fn test_environment_serialization() {
        let catalog = EnvironmentCatalog::new(1, 0);

        // Test XML serialization
        let xml_result = quick_xml::se::to_string(&catalog);
        assert!(xml_result.is_ok());

        let xml = xml_result.unwrap();
        assert!(xml.contains("EnvironmentCatalog"));
        assert!(xml.contains("revMajor=\"1\""));
        assert!(xml.contains("revMinor=\"0\""));
    }

    #[test]
    fn test_to_scenario_environment() {
        let mut catalog_env = CatalogEnvironment::new("TestEnvironment".to_string());

        // Set up a sunny day environment
        catalog_env.set_weather(CatalogWeather::sunny());
        catalog_env.set_time_of_day(CatalogTimeOfDay::new(Value::Literal(
            "2021-06-21T12:00:00".to_string(),
        )));

        let scenario_env = catalog_env.to_scenario_environment();

        assert_eq!(scenario_env.name.as_literal().unwrap(), "TestEnvironment");
        assert_eq!(
            scenario_env.time_of_day.as_ref().unwrap().date_time,
            "2021-06-21T12:00:00"
        );
        // cloud_state is not propagated by to_scenario_environment (deprecated field)
        assert!(scenario_env.weather.as_ref().unwrap().cloud_state.is_none());
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

    #[test]
    fn test_defaults() {
        let catalog = EnvironmentCatalog::default();
        let environment = CatalogEnvironment::new("DefaultCatalogEnvironment".to_string());
        let weather = CatalogWeather::default();

        assert_eq!(catalog.rev_major.as_literal().unwrap(), &1);
        assert_eq!(environment.name, "DefaultCatalogEnvironment");
        assert!(environment.time_of_day.is_none());
        assert!(weather.cloud_state.is_none());
        assert!(weather.sun.is_none());
    }
}
