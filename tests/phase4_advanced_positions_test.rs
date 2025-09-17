//! Tests for Phase 4 Advanced Positions: TrajectoryPosition, GeographicPosition, RelativeObjectPosition

use openscenario_rs::types::{
    positions::{
        Position, 
        trajectory::TrajectoryPosition,
        world::GeographicPosition,
        relative::RelativeObjectPosition
    },
    basic::{Double, OSString},
};

#[test]
fn test_trajectory_position_new() {
    let position = TrajectoryPosition::new(100.0);
    assert_eq!(position.s, Double::literal(100.0));
    assert_eq!(position.t, None);
    assert_eq!(position.orientation, None);
}

#[test]
fn test_trajectory_position_with_offset() {
    let position = TrajectoryPosition::with_offset(150.0, 5.0);
    assert_eq!(position.s, Double::literal(150.0));
    assert_eq!(position.t, Some(Double::literal(5.0)));
    assert_eq!(position.orientation, None);
}

#[test]
fn test_trajectory_position_at_distance() {
    let position = TrajectoryPosition::at_distance(200.0, -2.5);
    assert_eq!(position.s, Double::literal(200.0));
    assert_eq!(position.t, Some(Double::literal(-2.5)));
    assert_eq!(position.orientation, None);
}

#[test]
fn test_trajectory_position_default() {
    let position = TrajectoryPosition::default();
    assert_eq!(position.s, Double::literal(0.0));
    assert_eq!(position.t, None);
    assert_eq!(position.orientation, None);
}

#[test]
fn test_geographic_position_new() {
    let position = GeographicPosition::new(37.7749, -122.4194); // San Francisco
    assert_eq!(position.latitude, Double::literal(37.7749));
    assert_eq!(position.longitude, Double::literal(-122.4194));
    assert_eq!(position.height, None);
    assert_eq!(position.orientation, None);
}

#[test]
fn test_geographic_position_with_height() {
    let position = GeographicPosition::with_height(40.7128, -74.0060, 10.0); // NYC with height
    assert_eq!(position.latitude, Double::literal(40.7128));
    assert_eq!(position.longitude, Double::literal(-74.0060));
    assert_eq!(position.height, Some(Double::literal(10.0)));
    assert_eq!(position.orientation, None);
}

#[test]
fn test_geographic_position_default() {
    let position = GeographicPosition::default();
    assert_eq!(position.latitude, Double::literal(0.0));
    assert_eq!(position.longitude, Double::literal(0.0));
    assert_eq!(position.height, None);
    assert_eq!(position.orientation, None);
}

#[test]
fn test_relative_object_position_new() {
    let position = RelativeObjectPosition::new("lead_vehicle", 10.0, 2.0);
    assert_eq!(position.entity_ref, OSString::literal("lead_vehicle".to_string()));
    assert_eq!(position.dx, Double::literal(10.0));
    assert_eq!(position.dy, Double::literal(2.0));
    assert_eq!(position.dz, None);
    assert_eq!(position.orientation, None);
}

#[test]
fn test_relative_object_position_with_z() {
    let position = RelativeObjectPosition::with_z("target", 5.0, -1.5, 0.8);
    assert_eq!(position.entity_ref, OSString::literal("target".to_string()));
    assert_eq!(position.dx, Double::literal(5.0));
    assert_eq!(position.dy, Double::literal(-1.5));
    assert_eq!(position.dz, Some(Double::literal(0.8)));
    assert_eq!(position.orientation, None);
}

#[test]
fn test_relative_object_position_behind() {
    let position = RelativeObjectPosition::behind("leader", 20.0);
    assert_eq!(position.entity_ref, OSString::literal("leader".to_string()));
    assert_eq!(position.dx, Double::literal(-20.0));
    assert_eq!(position.dy, Double::literal(0.0));
    assert_eq!(position.dz, None);
}

#[test]
fn test_relative_object_position_ahead() {
    let position = RelativeObjectPosition::ahead("follower", 15.0);
    assert_eq!(position.entity_ref, OSString::literal("follower".to_string()));
    assert_eq!(position.dx, Double::literal(15.0));
    assert_eq!(position.dy, Double::literal(0.0));
    assert_eq!(position.dz, None);
}

#[test]
fn test_relative_object_position_left_of() {
    let position = RelativeObjectPosition::left_of("reference", 3.5);
    assert_eq!(position.entity_ref, OSString::literal("reference".to_string()));
    assert_eq!(position.dx, Double::literal(0.0));
    assert_eq!(position.dy, Double::literal(3.5));
    assert_eq!(position.dz, None);
}

#[test]
fn test_relative_object_position_right_of() {
    let position = RelativeObjectPosition::right_of("reference", 2.8);
    assert_eq!(position.entity_ref, OSString::literal("reference".to_string()));
    assert_eq!(position.dx, Double::literal(0.0));
    assert_eq!(position.dy, Double::literal(-2.8));
    assert_eq!(position.dz, None);
}

#[test]
fn test_relative_object_position_default() {
    let position = RelativeObjectPosition::default();
    assert_eq!(position.entity_ref, OSString::literal("DefaultEntity".to_string()));
    assert_eq!(position.dx, Double::literal(0.0));
    assert_eq!(position.dy, Double::literal(0.0));
    assert_eq!(position.dz, None);
    assert_eq!(position.orientation, None);
}

#[test]
fn test_position_constructors() {
    let trajectory_pos = TrajectoryPosition::new(75.0);
    let geographic_pos = GeographicPosition::new(48.8566, 2.3522); // Paris
    let relative_pos = RelativeObjectPosition::behind("vehicle1", 25.0);
    
    // Test Position construction
    let pos1 = Position {
        world_position: None,
        relative_world_position: None,
        road_position: None,
        relative_road_position: None,
        lane_position: None,
        relative_lane_position: None,
        trajectory_position: Some(trajectory_pos.clone()),
        geographic_position: None,
        relative_object_position: None,
    };
    
    assert_eq!(pos1.trajectory_position, Some(trajectory_pos));
    assert_eq!(pos1.geographic_position, None);
    assert_eq!(pos1.relative_object_position, None);
}

#[test]
fn test_advanced_positions_serialization() {
    let trajectory_pos = TrajectoryPosition::with_offset(100.0, 2.5);
    let geographic_pos = GeographicPosition::with_height(52.5200, 13.4050, 35.0); // Berlin
    let relative_pos = RelativeObjectPosition::left_of("ego", 4.2);
    
    // Test that they can be serialized and deserialized
    let trajectory_serialized = serde_json::to_string(&trajectory_pos).unwrap();
    let trajectory_deserialized: TrajectoryPosition = serde_json::from_str(&trajectory_serialized).unwrap();
    assert_eq!(trajectory_pos, trajectory_deserialized);
    
    let geographic_serialized = serde_json::to_string(&geographic_pos).unwrap();
    let geographic_deserialized: GeographicPosition = serde_json::from_str(&geographic_serialized).unwrap();
    assert_eq!(geographic_pos, geographic_deserialized);
    
    let relative_serialized = serde_json::to_string(&relative_pos).unwrap();
    let relative_deserialized: RelativeObjectPosition = serde_json::from_str(&relative_serialized).unwrap();
    assert_eq!(relative_pos, relative_deserialized);
}

#[test]
fn test_phase4_positions_completeness() {
    // Test that all Phase 4 position types are accessible
    let _trajectory = TrajectoryPosition::new(0.0);
    let _geographic = GeographicPosition::new(0.0, 0.0);
    let _relative_object = RelativeObjectPosition::new("entity", 0.0, 0.0);
    
    // Verify they have all expected methods
    assert_eq!(_trajectory.s, Double::literal(0.0));
    assert_eq!(_geographic.latitude, Double::literal(0.0));
    assert_eq!(_relative_object.dx, Double::literal(0.0));
}

#[test]
fn test_complex_positioning_scenarios() {
    // Complex trajectory positioning
    let complex_trajectory = TrajectoryPosition::with_offset(500.0, -1.8);
    
    // GPS coordinates with height
    let complex_geographic = GeographicPosition::at_coordinates(
        59.9311, 10.7594, 12.0, 1.2 // Oslo
    );
    
    // Multi-offset relative positioning
    let complex_relative = RelativeObjectPosition::at_offset("convoy_leader", 8.5, 3.0, 0.5);
    
    // Verify all complex scenarios work
    assert_eq!(complex_trajectory.s, Double::literal(500.0));
    assert_eq!(complex_geographic.latitude, Double::literal(59.9311));
    assert_eq!(complex_relative.entity_ref, OSString::literal("convoy_leader".to_string()));
}