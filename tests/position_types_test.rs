//! Integration tests for Phase 1 position types: RelativeRoadPosition and RelativeLanePosition
//!
//! This test file validates:
//! - Correct serialization/deserialization of new position types
//! - Integration with the Position enum
//! - Builder methods and default implementations
//! - XML round-trip compatibility

use openscenario_rs::types::basic::{Double, OSString};
use openscenario_rs::types::positions::{
    Orientation, Position, RelativeLanePosition, RelativeRoadPosition,
};


#[test]
fn test_relative_road_position_xml_roundtrip() {
    let original = RelativeRoadPosition::new("EgoVehicle".to_string(), 10.0, -2.5);

    // Serialize to XML
    let xml = quick_xml::se::to_string(&original).expect("Failed to serialize");
    println!("RelativeRoadPosition XML: {}", xml);

    // Deserialize back
    let deserialized: RelativeRoadPosition =
        quick_xml::de::from_str(&xml).expect("Failed to deserialize");

    // Verify round-trip
    assert_eq!(original, deserialized);
    assert_eq!(deserialized.entity_ref.as_literal().unwrap(), "EgoVehicle");
    assert_eq!(deserialized.ds.as_literal().unwrap(), &10.0);
    assert_eq!(deserialized.dt.as_literal().unwrap(), &-2.5);
}

#[test]
fn test_relative_lane_position_xml_roundtrip() {
    let original = RelativeLanePosition::new("EgoVehicle".to_string(), -1, 15.0, 0.5);

    // Serialize to XML
    let xml = quick_xml::se::to_string(&original).expect("Failed to serialize");
    println!("RelativeLanePosition XML: {}", xml);

    // Deserialize back
    let deserialized: RelativeLanePosition =
        quick_xml::de::from_str(&xml).expect("Failed to deserialize");

    // Verify round-trip
    assert_eq!(original, deserialized);
    assert_eq!(deserialized.entity_ref.as_literal().unwrap(), "EgoVehicle");
    assert_eq!(deserialized.d_lane.as_literal().unwrap(), &-1);
    assert_eq!(deserialized.ds.as_literal().unwrap(), &15.0);
    assert_eq!(deserialized.offset.as_literal().unwrap(), &0.5);
}

#[test]
fn test_relative_road_position_with_orientation() {
    let orientation = Orientation::heading(1.57);
    let pos =
        RelativeRoadPosition::with_orientation("EgoVehicle".to_string(), 5.0, 1.5, orientation);

    // Serialize to XML
    let xml = quick_xml::se::to_string(&pos).expect("Failed to serialize");
    println!("RelativeRoadPosition with orientation XML: {}", xml);

    // Verify orientation is included
    assert!(xml.contains("Orientation"));
    assert!(xml.contains("h=\"1.57\""));

    // Deserialize back
    let deserialized: RelativeRoadPosition =
        quick_xml::de::from_str(&xml).expect("Failed to deserialize");

    assert_eq!(pos, deserialized);
    assert!(deserialized.orientation.is_some());
    assert_eq!(
        deserialized
            .orientation
            .unwrap()
            .h
            .unwrap()
            .as_literal()
            .unwrap(),
        &1.57
    );
}

#[test]
fn test_relative_lane_position_with_orientation() {
    let orientation = Orientation::new(1.57, 0.1, -0.1);
    let pos = RelativeLanePosition::with_orientation(
        "EgoVehicle".to_string(),
        1,
        20.0,
        -1.0,
        orientation,
    );

    // Serialize to XML
    let xml = quick_xml::se::to_string(&pos).expect("Failed to serialize");
    println!("RelativeLanePosition with orientation XML: {}", xml);

    // Verify orientation is included
    assert!(xml.contains("Orientation"));
    assert!(xml.contains("h=\"1.57\""));
    assert!(xml.contains("p=\"0.1\""));
    assert!(xml.contains("r=\"-0.1\""));

    // Deserialize back
    let deserialized: RelativeLanePosition =
        quick_xml::de::from_str(&xml).expect("Failed to deserialize");

    assert_eq!(pos, deserialized);
    assert!(deserialized.orientation.is_some());
    let orient = deserialized.orientation.unwrap();
    assert_eq!(orient.h.unwrap().as_literal().unwrap(), &1.57);
    assert_eq!(orient.p.unwrap().as_literal().unwrap(), &0.1);
    assert_eq!(orient.r.unwrap().as_literal().unwrap(), &-0.1);
}

#[test]
fn test_position_enum_integration() {
    // Test RelativeRoadPosition integration
    let rel_road = RelativeRoadPosition::new("EgoVehicle".to_string(), 10.0, -2.0);
    let position = Position::relative_road(rel_road.clone());

    assert!(position.relative_road_position.is_some());
    assert_eq!(position.relative_road_position.unwrap(), rel_road);
    assert!(position.world_position.is_none());
    assert!(position.relative_world_position.is_none());
    assert!(position.road_position.is_none());
    assert!(position.lane_position.is_none());
    assert!(position.relative_lane_position.is_none());

    // Test RelativeLanePosition integration
    let rel_lane = RelativeLanePosition::new("EgoVehicle".to_string(), -1, 15.0, 0.5);
    let position = Position::relative_lane(rel_lane.clone());

    assert!(position.relative_lane_position.is_some());
    assert_eq!(position.relative_lane_position.unwrap(), rel_lane);
    assert!(position.world_position.is_none());
    assert!(position.relative_world_position.is_none());
    assert!(position.road_position.is_none());
    assert!(position.relative_road_position.is_none());
    assert!(position.lane_position.is_none());
}

#[test]
fn test_position_xml_serialization() {
    // Test Position with RelativeRoadPosition
    let rel_road = RelativeRoadPosition::new("EgoVehicle".to_string(), 10.0, -2.0);
    let position = Position::relative_road(rel_road);

    let xml = quick_xml::se::to_string(&position).expect("Failed to serialize Position");
    println!("Position with RelativeRoadPosition XML: {}", xml);

    assert!(xml.contains("Position"));
    assert!(xml.contains("RelativeRoadPosition"));
    assert!(xml.contains("entityRef=\"EgoVehicle\""));
    assert!(xml.contains("ds=\"10\""));
    assert!(xml.contains("dt=\"-2\""));

    // Test Position with RelativeLanePosition
    let rel_lane = RelativeLanePosition::new("EgoVehicle".to_string(), -1, 15.0, 0.5);
    let position = Position::relative_lane(rel_lane);

    let xml = quick_xml::se::to_string(&position).expect("Failed to serialize Position");
    println!("Position with RelativeLanePosition XML: {}", xml);

    assert!(xml.contains("Position"));
    assert!(xml.contains("RelativeLanePosition"));
    assert!(xml.contains("entityRef=\"EgoVehicle\""));
    assert!(xml.contains("dLane=\"-1\""));
    assert!(xml.contains("ds=\"15\""));
    assert!(xml.contains("offset=\"0.5\""));
}

#[test]
fn test_parameter_support() {
    // Test with parameter references
    let rel_road = RelativeRoadPosition {
        entity_ref: OSString::parameter("TargetEntity".to_string()),
        ds: Double::parameter("RelativeDistance".to_string()),
        dt: Double::literal(-1.5),
        orientation: None,
    };

    // Serialize to XML
    let xml = quick_xml::se::to_string(&rel_road).expect("Failed to serialize");
    println!("RelativeRoadPosition with parameters XML: {}", xml);

    // Verify parameter syntax is preserved
    assert!(xml.contains("entityRef=\"${TargetEntity}\""));
    assert!(xml.contains("ds=\"${RelativeDistance}\""));
    assert!(xml.contains("dt=\"-1.5\""));

    // Deserialize back
    let deserialized: RelativeRoadPosition =
        quick_xml::de::from_str(&xml).expect("Failed to deserialize");

    assert_eq!(rel_road, deserialized);
    assert_eq!(
        deserialized.entity_ref.as_parameter().unwrap(),
        "TargetEntity"
    );
    assert_eq!(deserialized.ds.as_parameter().unwrap(), "RelativeDistance");
    assert_eq!(deserialized.dt.as_literal().unwrap(), &-1.5);
}

#[test]
fn test_default_implementations() {
    let rel_road_default = RelativeRoadPosition::default();
    assert_eq!(
        rel_road_default.entity_ref.as_literal().unwrap(),
        "DefaultEntity"
    );
    assert_eq!(rel_road_default.ds.as_literal().unwrap(), &0.0);
    assert_eq!(rel_road_default.dt.as_literal().unwrap(), &0.0);
    assert!(rel_road_default.orientation.is_none());

    let rel_lane_default = RelativeLanePosition::default();
    assert_eq!(
        rel_lane_default.entity_ref.as_literal().unwrap(),
        "DefaultEntity"
    );
    assert_eq!(rel_lane_default.d_lane.as_literal().unwrap(), &0);
    assert_eq!(rel_lane_default.ds.as_literal().unwrap(), &0.0);
    assert_eq!(rel_lane_default.offset.as_literal().unwrap(), &0.0);
    assert!(rel_lane_default.orientation.is_none());
}

#[test]
fn test_builder_methods() {
    // Test RelativeRoadPosition builders
    let pos1 = RelativeRoadPosition::new("Vehicle1".to_string(), 5.0, 2.0);
    assert_eq!(pos1.entity_ref.as_literal().unwrap(), "Vehicle1");
    assert_eq!(pos1.ds.as_literal().unwrap(), &5.0);
    assert_eq!(pos1.dt.as_literal().unwrap(), &2.0);
    assert!(pos1.orientation.is_none());

    let orientation = Orientation::heading(0.5);
    let pos2 =
        RelativeRoadPosition::with_orientation("Vehicle2".to_string(), 10.0, -1.0, orientation);
    assert_eq!(pos2.entity_ref.as_literal().unwrap(), "Vehicle2");
    assert_eq!(pos2.ds.as_literal().unwrap(), &10.0);
    assert_eq!(pos2.dt.as_literal().unwrap(), &-1.0);
    assert!(pos2.orientation.is_some());

    // Test RelativeLanePosition builders
    let pos3 = RelativeLanePosition::new("Vehicle3".to_string(), 1, 15.0, 0.5);
    assert_eq!(pos3.entity_ref.as_literal().unwrap(), "Vehicle3");
    assert_eq!(pos3.d_lane.as_literal().unwrap(), &1);
    assert_eq!(pos3.ds.as_literal().unwrap(), &15.0);
    assert_eq!(pos3.offset.as_literal().unwrap(), &0.5);
    assert!(pos3.orientation.is_none());

    let orientation2 = Orientation::new(1.57, 0.0, 0.0);
    let pos4 = RelativeLanePosition::with_orientation(
        "Vehicle4".to_string(),
        -2,
        20.0,
        -0.5,
        orientation2,
    );
    assert_eq!(pos4.entity_ref.as_literal().unwrap(), "Vehicle4");
    assert_eq!(pos4.d_lane.as_literal().unwrap(), &-2);
    assert_eq!(pos4.ds.as_literal().unwrap(), &20.0);
    assert_eq!(pos4.offset.as_literal().unwrap(), &-0.5);
    assert!(pos4.orientation.is_some());
}
