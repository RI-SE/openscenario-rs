//! Spatial Conditions Demo
//!
//! This example demonstrates the usage of position and distance-based conditions
//! in OpenSCENARIO scenarios, including:
//! - ReachPositionCondition for waypoint-based triggering
//! - DistanceCondition for proximity-based triggering
//! - RelativeDistanceCondition for inter-entity distance monitoring
//! - Various coordinate systems and measurement types
//! - Real-world autonomous driving scenario examples

use openscenario_rs::types::{
    conditions::{DistanceCondition, ReachPositionCondition, RelativeDistanceCondition},
    enums::{CoordinateSystem, RelativeDistanceType, RoutingAlgorithm, Rule},
    positions::Position,
};

fn main() {
    println!("🚗 OpenSCENARIO Spatial Conditions Demo");
    println!("========================================\n");

    // Example 1: Reach Position Condition
    println!("1. Reach Position Condition");
    println!("---------------------------");
    
    let waypoint_condition = ReachPositionCondition::at_world_position(
        1250.0, // x: 1250m east
        850.0,  // y: 850m north  
        0.0,    // z: ground level
        1.57,   // heading: 90 degrees (facing north)
        2.0,    // tolerance: 2 meters
    );
    
    println!("✓ Vehicle must reach waypoint (1250, 850) within 2m tolerance");
    println!("  Target position: x={}, y={}, heading={} rad", 
             waypoint_condition.position.world_position.as_ref().unwrap().x,
             waypoint_condition.position.world_position.as_ref().unwrap().y,
             waypoint_condition.position.world_position.as_ref().unwrap().h);
    println!("  Tolerance: {} meters\n", waypoint_condition.tolerance);

    // Example 2: Distance Condition for Intersection Approach
    println!("2. Distance Condition - Intersection Approach");
    println!("---------------------------------------------");
    
    let intersection_pos = Position::default(); // Would be set to intersection coordinates
    let intersection_condition = DistanceCondition::less_than(intersection_pos, 50.0, true)
        .with_coordinate_system(CoordinateSystem::Road)
        .with_distance_type(RelativeDistanceType::Cartesian);
    
    println!("✓ Trigger when vehicle is within 50m of intersection");
    println!("  Distance threshold: {} meters", intersection_condition.value);
    println!("  Measurement: Freespace cartesian distance");
    println!("  Coordinate system: Road coordinates\n");

    // Example 3: Relative Distance - Car Following
    println!("3. Relative Distance - Car Following Safety");
    println!("-------------------------------------------");
    
    let following_condition = RelativeDistanceCondition::longitudinal(
        "lead_vehicle",
        20.0,
        true, // freespace distance
        Rule::GreaterThan,
    ).with_coordinate_system(CoordinateSystem::Road);
    
    println!("✓ Maintain safe following distance from lead vehicle");
    println!("  Reference entity: {}", following_condition.entity_ref);
    println!("  Minimum distance: {} meters", following_condition.value);
    println!("  Measurement type: Longitudinal freespace");
    println!("  Rule: Must be greater than threshold\n");

    // Example 4: Relative Distance - Lane Change Safety
    println!("4. Relative Distance - Lane Change Safety");
    println!("-----------------------------------------");
    
    let lane_change_condition = RelativeDistanceCondition::lateral(
        "adjacent_vehicle",
        3.0,
        true, // freespace
        Rule::GreaterThan,
    ).with_coordinate_system(CoordinateSystem::Lane);
    
    println!("✓ Safe lateral clearance for lane change maneuver");
    println!("  Reference entity: {}", lane_change_condition.entity_ref);
    println!("  Minimum clearance: {} meters", lane_change_condition.value);
    println!("  Measurement type: Lateral freespace");
    println!("  Coordinate system: Lane-relative\n");

    // Example 5: Complex Multi-Condition Scenario
    println!("5. Complex Scenario - Highway Merge");
    println!("-----------------------------------");
    
    // Condition 1: Approach merge point
    let merge_approach = ReachPositionCondition::at_world_position(
        2000.0, 500.0, 0.0, 0.0, 5.0
    );
    
    // Condition 2: Safe gap in traffic
    let gap_condition = RelativeDistanceCondition::longitudinal(
        "highway_traffic",
        30.0,
        true,
        Rule::GreaterThan,
    ).with_coordinate_system(CoordinateSystem::Road);
    
    // Condition 3: Minimum speed for merge
    // (This would use SpeedCondition, but we'll describe it)
    
    println!("✓ Highway merge scenario with multiple conditions:");
    println!("  - Approach merge point (2000, 500) within 5m");
    println!("  - Maintain 30m+ gap from highway traffic");
    println!("  - Achieve minimum merge speed (would use SpeedCondition)");
    println!("  - All conditions use road coordinate system for consistency\n");

    // Example 6: XML Serialization Demo
    println!("6. XML Serialization Example");
    println!("----------------------------");
    
    let demo_condition = RelativeDistanceCondition::cartesian(
        "ego_vehicle",
        15.0,
        false, // reference point distance
        Rule::LessOrEqual,
    ).with_coordinate_system(CoordinateSystem::Entity)
     .with_routing_algorithm(RoutingAlgorithm::Shortest);
    
    match quick_xml::se::to_string(&demo_condition) {
        Ok(xml) => {
            println!("✓ Condition serialized to XML:");
            println!("{}", xml);
        }
        Err(e) => {
            println!("✗ Serialization failed: {}", e);
        }
    }
    
    println!("\n🎯 Spatial conditions enable precise scenario control!");
    println!("   - Position-based triggering for waypoint navigation");
    println!("   - Distance monitoring for safety and coordination");
    println!("   - Flexible coordinate systems for different use cases");
    println!("   - Support for both freespace and reference point measurements");
    println!("   - Integration with OpenSCENARIO parameter system");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_conditions_compile() {
        // Verify all demo conditions can be created without panics
        let _waypoint = ReachPositionCondition::at_world_position(1250.0, 850.0, 0.0, 1.57, 2.0);
        let _intersection = DistanceCondition::less_than(Position::default(), 50.0, true);
        let _following = RelativeDistanceCondition::longitudinal("lead", 20.0, true, Rule::GreaterThan);
        let _lane_change = RelativeDistanceCondition::lateral("adjacent", 3.0, true, Rule::GreaterThan);
        
        // If we get here, all conditions compiled successfully
        assert!(true);
    }
}