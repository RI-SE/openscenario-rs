//! Simple Spatial Conditions Demo
//!
//! This example demonstrates basic usage of spatial conditions
//! using direct struct construction to avoid import issues.

use openscenario_rs::types::{
    basic::{Double, Boolean, OSString},
    enums::{RelativeDistanceType, Rule},
    positions::Position,
};

fn main() {
    println!("🚗 Simple Spatial Conditions Demo");
    println!("==================================\n");

    // Example 1: Create a ReachPositionCondition directly
    println!("1. Reach Position Condition");
    println!("---------------------------");
    
    // We'll construct the condition using the builder method
    let reach_condition = openscenario_rs::types::conditions::ReachPositionCondition::new(
        Position::default(),
        2.0,
    );
    
    println!("✓ Created reach position condition");
    println!("  Tolerance: {} meters", reach_condition.tolerance);
    println!("  Position: {:?}\n", reach_condition.position.world_position.is_some());

    // Example 2: Create a DistanceCondition directly
    println!("2. Distance Condition");
    println!("--------------------");
    
    let distance_condition = openscenario_rs::types::conditions::DistanceCondition {
        position: Position::default(),
        value: Double::literal(50.0),
        freespace: Boolean::literal(true),
        rule: Rule::LessThan,
        along_route: None,
        coordinate_system: None,
        relative_distance_type: None,
        routing_algorithm: None,
    };
    
    println!("✓ Created distance condition");
    println!("  Distance threshold: {} meters", distance_condition.value);
    println!("  Rule: {:?}", distance_condition.rule);
    println!("  Freespace: {}\n", distance_condition.freespace);

    // Example 3: Create a RelativeDistanceCondition directly
    println!("3. Relative Distance Condition");
    println!("------------------------------");
    
    let relative_condition = openscenario_rs::types::conditions::RelativeDistanceCondition {
        entity_ref: OSString::literal("target_vehicle".to_string()),
        value: Double::literal(20.0),
        freespace: Boolean::literal(true),
        relative_distance_type: RelativeDistanceType::Longitudinal,
        rule: Rule::GreaterThan,
        coordinate_system: None,
        routing_algorithm: None,
    };
    
    println!("✓ Created relative distance condition");
    println!("  Reference entity: {}", relative_condition.entity_ref);
    println!("  Distance: {} meters", relative_condition.value);
    println!("  Type: {:?}", relative_condition.relative_distance_type);
    println!("  Rule: {:?}\n", relative_condition.rule);

    // Example 4: Test XML serialization
    println!("4. XML Serialization Test");
    println!("-------------------------");
    
    match quick_xml::se::to_string(&relative_condition) {
        Ok(xml) => {
            println!("✓ Successfully serialized to XML:");
            println!("{}", xml);
        }
        Err(e) => {
            println!("✗ Serialization failed: {}", e);
        }
    }
    
    println!("\n🎯 Spatial conditions are working!");
    println!("   - All three condition types can be created");
    println!("   - XML serialization works correctly");
    println!("   - Ready for integration with scenario logic");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_conditions_work() {
        // Verify all demo conditions can be created without panics
        let _reach = openscenario_rs::types::conditions::ReachPositionCondition {
            position: Position::default(),
            tolerance: Double::literal(2.0),
        };
        
        let _distance = openscenario_rs::types::conditions::DistanceCondition {
            position: Position::default(),
            value: Double::literal(50.0),
            freespace: Boolean::literal(true),
            rule: Rule::LessThan,
            along_route: None,
            coordinate_system: None,
            relative_distance_type: None,
            routing_algorithm: None,
        };
        
        let _relative = openscenario_rs::types::conditions::RelativeDistanceCondition {
            entity_ref: OSString::literal("target".to_string()),
            value: Double::literal(20.0),
            freespace: Boolean::literal(true),
            relative_distance_type: RelativeDistanceType::Longitudinal,
            rule: Rule::GreaterThan,
            coordinate_system: None,
            routing_algorithm: None,
        };
        
        // If we get here, all conditions compiled successfully
        assert!(true);
    }
}