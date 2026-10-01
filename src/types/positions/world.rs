//! World coordinate position types for absolute positioning

use crate::types::basic::Double;
use serde::{Deserialize, Serialize};

/// Absolute world position with X, Y, Z coordinates and orientation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldPosition {
    /// X coordinate in meters
    #[serde(rename = "@x")]
    pub x: Double,
    /// Y coordinate in meters
    #[serde(rename = "@y")]
    pub y: Double,
    /// Z coordinate in meters (height)
    #[serde(rename = "@z", skip_serializing_if = "Option::is_none")]
    pub z: Option<Double>,
    /// Heading angle in radians
    #[serde(rename = "@h", skip_serializing_if = "Option::is_none")]
    pub h: Option<Double>,
    /// Pitch angle in radians
    #[serde(rename = "@p", skip_serializing_if = "Option::is_none")]
    pub p: Option<Double>,
    /// Roll angle in radians
    #[serde(rename = "@r", skip_serializing_if = "Option::is_none")]
    pub r: Option<Double>,
}

impl WorldPosition {
    /// Create a new WorldPosition with required x, y coordinates
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x: Double::literal(x),
            y: Double::literal(y),
            z: None,
            h: None,
            p: None,
            r: None,
        }
    }

    /// Create a new WorldPosition with x, y, z coordinates
    pub fn with_z(x: f64, y: f64, z: f64) -> Self {
        Self {
            x: Double::literal(x),
            y: Double::literal(y),
            z: Some(Double::literal(z)),
            h: None,
            p: None,
            r: None,
        }
    }

    /// Create a new WorldPosition with x, y, z, h coordinates
    pub fn with_orientation(x: f64, y: f64, z: f64, h: f64) -> Self {
        Self {
            x: Double::literal(x),
            y: Double::literal(y),
            z: Some(Double::literal(z)),
            h: Some(Double::literal(h)),
            p: None,
            r: None,
        }
    }

    /// Create a new WorldPosition with all coordinates
    pub fn with_full_orientation(x: f64, y: f64, z: f64, h: f64, p: f64, r: f64) -> Self {
        Self {
            x: Double::literal(x),
            y: Double::literal(y),
            z: Some(Double::literal(z)),
            h: Some(Double::literal(h)),
            p: Some(Double::literal(p)),
            r: Some(Double::literal(r)),
        }
    }
}

/// Geographic position using latitude/longitude coordinates.
///
/// Corresponds to XSD complexType `GeoPosition`. `latitude`/`longitude` (radians) and
/// `height` are deprecated since OpenSCENARIO 1.2 in favor of `latitudeDeg`/`longitudeDeg`
/// (degrees) and `altitude`. All are optional per the XSD (none use `use="required"`).
/// The constructors write only the degree attributes. A document that uses the deprecated
/// attributes still deserializes into the deprecated fields, without unit conversion.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "GeoPosition")]
pub struct GeographicPosition {
    /// Latitude in radians (deprecated — XSD attribute `latitude`)
    #[serde(rename = "@latitude", default, skip_serializing_if = "Option::is_none")]
    pub latitude: Option<Double>,

    /// Longitude in radians (deprecated — XSD attribute `longitude`)
    #[serde(
        rename = "@longitude",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub longitude: Option<Double>,

    /// Height above a reference surface in meters (deprecated — XSD attribute `height`)
    #[serde(rename = "@height", default, skip_serializing_if = "Option::is_none")]
    pub height: Option<Double>,

    /// Latitude in degrees, range [-90..90] — XSD attribute `latitudeDeg` (current)
    #[serde(
        rename = "@latitudeDeg",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub latitude_deg: Option<Double>,

    /// Longitude in degrees, range [-180..180] — XSD attribute `longitudeDeg` (current)
    #[serde(
        rename = "@longitudeDeg",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub longitude_deg: Option<Double>,

    /// Altitude above the road surface in meters — XSD attribute `altitude` (current)
    #[serde(rename = "@altitude", default, skip_serializing_if = "Option::is_none")]
    pub altitude: Option<Double>,

    /// Vertical road selection — XSD attribute `verticalRoadSelection`
    #[serde(
        rename = "@verticalRoadSelection",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vertical_road_selection: Option<crate::types::basic::Int>,

    /// Orientation in geographic coordinate system
    #[serde(rename = "Orientation", skip_serializing_if = "Option::is_none")]
    pub orientation: Option<crate::types::positions::road::Orientation>,
}

impl GeographicPosition {
    /// Create a geographic position from a latitude and longitude in degrees,
    /// written to `latitudeDeg` and `longitudeDeg`.
    pub fn from_degrees(latitude_deg: f64, longitude_deg: f64) -> Self {
        Self {
            latitude: None,
            longitude: None,
            height: None,
            latitude_deg: Some(Double::literal(latitude_deg)),
            longitude_deg: Some(Double::literal(longitude_deg)),
            altitude: None,
            vertical_road_selection: None,
            orientation: None,
        }
    }

    /// Create a geographic position from a latitude and longitude in degrees and an
    /// altitude in meters, written to `latitudeDeg`, `longitudeDeg` and `altitude`.
    pub fn from_degrees_with_altitude(
        latitude_deg: f64,
        longitude_deg: f64,
        altitude: f64,
    ) -> Self {
        Self {
            altitude: Some(Double::literal(altitude)),
            ..Self::from_degrees(latitude_deg, longitude_deg)
        }
    }

    /// Add orientation to geographic position
    pub fn with_orientation(
        mut self,
        orientation: crate::types::positions::road::Orientation,
    ) -> Self {
        self.orientation = Some(orientation);
        self
    }

    /// Create a geographic position from a latitude and longitude in degrees, an altitude
    /// in meters and a heading in radians. The heading is written to `Orientation/@h`.
    pub fn from_degrees_with_altitude_and_heading(
        latitude_deg: f64,
        longitude_deg: f64,
        altitude: f64,
        heading: f64,
    ) -> Self {
        use crate::types::positions::road::Orientation;

        let orientation = Orientation {
            h: Some(Double::literal(heading)),
            p: None,
            r: None,
            reference_context: None,
        };

        Self::from_degrees_with_altitude(latitude_deg, longitude_deg, altitude)
            .with_orientation(orientation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_position_with_full_orientation() {
        let pos = WorldPosition::with_full_orientation(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        assert_eq!(pos.x.as_literal().unwrap(), &1.0);
        assert_eq!(pos.z.unwrap().as_literal().unwrap(), &3.0);
        assert_eq!(pos.r.unwrap().as_literal().unwrap(), &6.0);
    }

    #[test]
    fn test_world_position_xml_roundtrip() {
        let pos = WorldPosition::new(100.5, -50.3);
        let xml = quick_xml::se::to_string(&pos).unwrap();
        assert!(xml.contains("x=\"100.5\""), "serialized: {xml}");
        assert!(xml.contains("y=\"-50.3\""), "serialized: {xml}");
        // `new` sets only the required attributes; the optional ones stay off the wire.
        for optional in ["z=", "h=", "p=", "r="] {
            assert!(!xml.contains(optional), "serialized: {xml}");
        }
        let deserialized: WorldPosition = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(pos, deserialized);
    }

    #[test]
    fn test_geographic_position_at_coordinates() {
        // (constructor output, latitudeDeg, longitudeDeg, altitude, heading)
        let cases = [
            (
                GeographicPosition::from_degrees(48.137, 11.576),
                "48.137",
                "11.576",
                None,
                None,
            ),
            (
                GeographicPosition::from_degrees_with_altitude(48.0, -11.5, 500.0),
                "48",
                "-11.5",
                Some("500"),
                None,
            ),
            (
                GeographicPosition::from_degrees_with_altitude_and_heading(
                    -33.5, 151.25, 12.5, 1.57,
                ),
                "-33.5",
                "151.25",
                Some("12.5"),
                Some("1.57"),
            ),
        ];
        for (pos, lat, lon, alt, heading) in cases {
            let xml = quick_xml::se::to_string(&pos).unwrap();
            assert!(xml.contains(&format!("latitudeDeg=\"{lat}\"")), "{xml}");
            assert!(xml.contains(&format!("longitudeDeg=\"{lon}\"")), "{xml}");
            match alt {
                Some(alt) => assert!(xml.contains(&format!("altitude=\"{alt}\"")), "{xml}"),
                None => assert!(!xml.contains("altitude="), "{xml}"),
            }
            match heading {
                Some(h) => assert!(xml.contains(&format!("<Orientation h=\"{h}\"/>")), "{xml}"),
                None => assert!(!xml.contains("<Orientation"), "{xml}"),
            }
            // The radian and height attributes are deprecated since 1.2.
            for deprecated in [" latitude=", " longitude=", " height="] {
                assert!(!xml.contains(deprecated), "{xml}");
            }
            let deserialized: GeographicPosition = quick_xml::de::from_str(&xml).unwrap();
            assert_eq!(pos, deserialized);
        }
    }

    #[test]
    fn test_geographic_position_parse_deprecated_and_current_attributes() {
        let literal = |v: f64| Some(Double::literal(v));
        let empty = GeographicPosition {
            latitude: None,
            longitude: None,
            height: None,
            latitude_deg: None,
            longitude_deg: None,
            altitude: None,
            vertical_road_selection: None,
            orientation: None,
        };
        let cases = [
            // Deprecated radian attributes are read as written, without unit conversion.
            (
                r#"<GeoPosition latitude="0.5" longitude="0.2" height="3"/>"#,
                GeographicPosition {
                    latitude: literal(0.5),
                    longitude: literal(0.2),
                    height: literal(3.0),
                    ..empty.clone()
                },
            ),
            (
                r#"<GeoPosition latitudeDeg="1" longitudeDeg="2"/>"#,
                GeographicPosition {
                    latitude_deg: literal(1.0),
                    longitude_deg: literal(2.0),
                    ..empty.clone()
                },
            ),
        ];
        for (xml, expected) in cases {
            let pos: GeographicPosition = quick_xml::de::from_str(xml).unwrap();
            assert_eq!(pos, expected, "{xml}");
        }
    }
}
