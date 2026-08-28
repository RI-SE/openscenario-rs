//! Route-relative position types
//!
//! Models XSD `RoutePosition` (:1968-1974) and its supporting types:
//! - `InRoutePosition` (:1323-1329) — choice of `FromCurrentEntity` |
//!   `FromRoadCoordinates` | `FromLaneCoordinates`
//! - `PositionOfCurrentEntity` (:1761-1763)
//! - `PositionInRoadCoordinates` (:1757-1760)
//! - `PositionInLaneCoordinates` (:1752-1756)
//!
//! `RouteRef` and `Route` are modeled once in `crate::types::routing` and are
//! reused here rather than duplicated.

use crate::types::basic::{Double, OSString};
use crate::types::positions::road::Orientation;
use crate::types::routing::RouteRef;
use serde::{Deserialize, Serialize};

/// Position given by the entity that is currently being acted upon.
///
/// XSD `PositionOfCurrentEntity` (:1761-1763): required `@entityRef`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename = "FromCurrentEntity")]
pub struct PositionOfCurrentEntity {
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,
}

impl Default for PositionOfCurrentEntity {
    fn default() -> Self {
        Self {
            entity_ref: OSString::literal("DefaultEntity".to_string()),
        }
    }
}

/// Position along a route expressed in road coordinates.
///
/// XSD `PositionInRoadCoordinates` (:1757-1760): required `@pathS`, required `@t`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename = "FromRoadCoordinates")]
pub struct PositionInRoadCoordinates {
    #[serde(rename = "@pathS")]
    pub path_s: Double,
    #[serde(rename = "@t")]
    pub t: Double,
}

impl Default for PositionInRoadCoordinates {
    fn default() -> Self {
        Self {
            path_s: Double::literal(0.0),
            t: Double::literal(0.0),
        }
    }
}

/// Position along a route expressed in lane coordinates.
///
/// XSD `PositionInLaneCoordinates` (:1752-1756): required `@laneId` (String),
/// optional `@laneOffset` (Double), required `@pathS` (Double).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename = "FromLaneCoordinates")]
pub struct PositionInLaneCoordinates {
    #[serde(rename = "@laneId")]
    pub lane_id: OSString,
    /// XSD: optional attribute
    #[serde(
        rename = "@laneOffset",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub lane_offset: Option<Double>,
    #[serde(rename = "@pathS")]
    pub path_s: Double,
}

impl Default for PositionInLaneCoordinates {
    fn default() -> Self {
        Self {
            lane_id: OSString::literal("-1".to_string()),
            lane_offset: None,
            path_s: Double::literal(0.0),
        }
    }
}

/// Position within a route.
///
/// XSD `InRoutePosition` (:1323-1329) is a choice; modeled as parallel
/// `Option` fields following the dominant pattern in this crate.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename = "InRoutePosition")]
pub struct InRoutePosition {
    #[serde(rename = "FromCurrentEntity", skip_serializing_if = "Option::is_none")]
    pub from_current_entity: Option<PositionOfCurrentEntity>,
    #[serde(
        rename = "FromRoadCoordinates",
        skip_serializing_if = "Option::is_none"
    )]
    pub from_road_coordinates: Option<PositionInRoadCoordinates>,
    #[serde(
        rename = "FromLaneCoordinates",
        skip_serializing_if = "Option::is_none"
    )]
    pub from_lane_coordinates: Option<PositionInLaneCoordinates>,
}

impl InRoutePosition {
    /// Create an `InRoutePosition` from the current entity.
    pub fn from_current_entity(entity_ref: impl Into<String>) -> Self {
        Self {
            from_current_entity: Some(PositionOfCurrentEntity {
                entity_ref: OSString::literal(entity_ref.into()),
            }),
            ..Default::default()
        }
    }

    /// Create an `InRoutePosition` from road coordinates.
    pub fn from_road_coordinates(path_s: Double, t: Double) -> Self {
        Self {
            from_road_coordinates: Some(PositionInRoadCoordinates { path_s, t }),
            ..Default::default()
        }
    }

    /// Create an `InRoutePosition` from lane coordinates.
    pub fn from_lane_coordinates(
        lane_id: OSString,
        path_s: Double,
        lane_offset: Option<Double>,
    ) -> Self {
        Self {
            from_lane_coordinates: Some(PositionInLaneCoordinates {
                lane_id,
                lane_offset,
                path_s,
            }),
            ..Default::default()
        }
    }
}

/// Element wrapper carrying the `RouteRef` choice.
///
/// XSD `RouteRef` (:1975-1980) is a choice of `Route` | `CatalogReference`,
/// already modeled as `crate::types::routing::RouteRef`. As an *element* the
/// choice has to sit behind a named wrapper, so this struct only re-hosts the
/// existing enum via `flatten` (the same shape `AssignRouteAction` uses).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename = "RouteRef")]
pub struct RouteRefElement {
    #[serde(flatten)]
    pub route_ref: RouteRef,
}

/// Position defined relative to a route.
///
/// XSD `RoutePosition` (:1968-1974) is an `xsd:all` of `RouteRef` (required),
/// `Orientation` (optional) and `InRoutePosition` (required).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename = "RoutePosition")]
pub struct RoutePosition {
    #[serde(rename = "RouteRef")]
    pub route_ref: RouteRefElement,
    /// XSD: optional element
    #[serde(
        rename = "Orientation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub orientation: Option<Orientation>,
    #[serde(rename = "InRoutePosition")]
    pub in_route_position: InRoutePosition,
}

impl RoutePosition {
    /// Create a `RoutePosition` from a route reference and in-route position.
    pub fn new(route_ref: RouteRef, in_route_position: InRoutePosition) -> Self {
        Self {
            route_ref: RouteRefElement { route_ref },
            orientation: None,
            in_route_position,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::routing::CatalogReference;

    #[test]
    fn test_in_route_position_from_lane_coordinates_roundtrip() {
        // Real corpus shape, with a `$param` value for pathS.
        let xml = r#"<InRoutePosition><FromLaneCoordinates laneId="-1" pathS="$_Ego_initS" laneOffset="0"/></InRoutePosition>"#;
        let parsed: InRoutePosition = quick_xml::de::from_str(xml).unwrap();
        let lane = parsed.from_lane_coordinates.as_ref().unwrap();
        assert_eq!(lane.lane_id.as_literal().unwrap(), "-1");
        assert!(
            lane.path_s.as_literal().is_none(),
            "pathS must stay a parameter reference"
        );
        assert_eq!(
            lane.lane_offset.as_ref().unwrap().as_literal().unwrap(),
            &0.0
        );
        assert!(parsed.from_road_coordinates.is_none());
        assert!(parsed.from_current_entity.is_none());

        let ser = quick_xml::se::to_string(&parsed).unwrap();
        assert!(ser.contains("_Ego_initS"), "serialized: {ser}");
        let back: InRoutePosition = quick_xml::de::from_str(&ser).unwrap();
        assert_eq!(parsed, back);
    }

    #[test]
    fn test_in_route_position_from_road_coordinates_roundtrip() {
        let xml =
            r#"<InRoutePosition><FromRoadCoordinates pathS="12.5" t="-1.75"/></InRoutePosition>"#;
        let parsed: InRoutePosition = quick_xml::de::from_str(xml).unwrap();
        let road = parsed.from_road_coordinates.as_ref().unwrap();
        assert_eq!(road.path_s.as_literal().unwrap(), &12.5);
        assert_eq!(road.t.as_literal().unwrap(), &-1.75);

        let ser = quick_xml::se::to_string(&parsed).unwrap();
        let back: InRoutePosition = quick_xml::de::from_str(&ser).unwrap();
        assert_eq!(parsed, back);
    }

    #[test]
    fn test_in_route_position_from_current_entity_roundtrip() {
        let xml = r#"<InRoutePosition><FromCurrentEntity entityRef="Ego"/></InRoutePosition>"#;
        let parsed: InRoutePosition = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(
            parsed
                .from_current_entity
                .as_ref()
                .unwrap()
                .entity_ref
                .as_literal()
                .unwrap(),
            "Ego"
        );

        let ser = quick_xml::se::to_string(&parsed).unwrap();
        let back: InRoutePosition = quick_xml::de::from_str(&ser).unwrap();
        assert_eq!(parsed, back);
    }

    #[test]
    fn test_lane_coordinates_optional_lane_offset_omitted() {
        let xml = r#"<FromLaneCoordinates laneId="1" pathS="3.0"/>"#;
        let parsed: PositionInLaneCoordinates = quick_xml::de::from_str(xml).unwrap();
        assert!(parsed.lane_offset.is_none());
        let ser = quick_xml::se::to_string(&parsed).unwrap();
        assert!(!ser.contains("laneOffset"), "serialized: {ser}");
    }

    #[test]
    fn test_route_position_catalog_reference_roundtrip() {
        // Real corpus shape.
        let xml = r#"<RoutePosition><RouteRef><CatalogReference catalogName="RouteCatalog" entryName="EgoRoute"/></RouteRef><InRoutePosition><FromLaneCoordinates laneId="-1" pathS="$_Ego_initS" laneOffset="0"/></InRoutePosition></RoutePosition>"#;
        let parsed: RoutePosition = quick_xml::de::from_str(xml).unwrap();
        match &parsed.route_ref.route_ref {
            RouteRef::Catalog(CatalogReference {
                catalog_name,
                entry_name,
                ..
            }) => {
                assert_eq!(catalog_name.as_literal().unwrap(), "RouteCatalog");
                assert_eq!(entry_name.as_literal().unwrap(), "EgoRoute");
            }
            other => panic!("expected catalog route ref, got {other:?}"),
        }
        assert!(parsed.orientation.is_none());
        assert!(parsed.in_route_position.from_lane_coordinates.is_some());

        let ser = quick_xml::se::to_string(&parsed).unwrap();
        assert!(ser.contains("CatalogReference"), "serialized: {ser}");
        let back: RoutePosition = quick_xml::de::from_str(&ser).unwrap();
        assert_eq!(parsed, back);
    }

    #[test]
    fn test_route_position_inside_position_roundtrip() {
        use crate::types::positions::Position;
        let xml = r#"<Position><RoutePosition><RouteRef><CatalogReference catalogName="RouteCatalog" entryName="EgoRoute"/></RouteRef><InRoutePosition><FromRoadCoordinates pathS="$initS" t="0"/></InRoutePosition></RoutePosition></Position>"#;
        let parsed: Position = quick_xml::de::from_str(xml).unwrap();
        let rp = parsed
            .route_position
            .as_ref()
            .expect("RoutePosition must parse");
        assert!(rp.in_route_position.from_road_coordinates.is_some());
        assert!(parsed.world_position.is_none());

        let ser = quick_xml::se::to_string(&parsed).unwrap();
        let back: Position = quick_xml::de::from_str(&ser).unwrap();
        assert_eq!(parsed, back);
    }
}
