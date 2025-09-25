//! Motion Conditions Demo - Phase 2 Implementation
//!
//! This example demonstrates the usage of AccelerationCondition and StandStillCondition
//! for motion-based scenario triggering in OpenSCENARIO files.

use openscenario_rs::types::conditions::entity::{
    AccelerationCondition, ByEntityCondition, StandStillCondition, EntityCondition,
};
use openscenario_rs::types::enums::{DirectionalDimension, Rule};
use openscenario_rs::types::scenario::triggers::TriggeringEntities;

fn main() {
    println!("=== Motion Conditions Demo ===\n");

    // 1. Basic AccelerationCondition examples
    println!("1. AccelerationCondition Examples:");

    let high_acceleration = AccelerationCondition::greater_than(5.0);
    println!("   High acceleration trigger: > 5.0 m/s²");
    println!(
        "   Rule: {:?}, Value: {:?}",
        high_acceleration.rule, high_acceleration.value
    );

    let low_acceleration = AccelerationCondition::less_than(1.0);
    println!("   Low acceleration trigger: < 1.0 m/s²");
    println!(
        "   Rule: {:?}, Value: {:?}",
        low_acceleration.rule, low_acceleration.value
    );

    // 2. Directional AccelerationCondition examples
    println!("\n2. Directional AccelerationCondition Examples:");

    let longitudinal_accel = AccelerationCondition::longitudinal(3.0, Rule::GreaterThan);
    println!("   Longitudinal acceleration: > 3.0 m/s²");
    println!("   Direction: {:?}", longitudinal_accel.direction);

    let lateral_accel = AccelerationCondition::lateral(2.0, Rule::LessThan);
    println!("   Lateral acceleration: < 2.0 m/s²");
    println!("   Direction: {:?}", lateral_accel.direction);

    let vertical_accel = AccelerationCondition::vertical(1.5, Rule::EqualTo);
    println!("   Vertical acceleration: = 1.5 m/s²");
    println!("   Direction: {:?}", vertical_accel.direction);

    // 3. StandStillCondition examples
    println!("\n3. StandStillCondition Examples:");

    let short_standstill = StandStillCondition::new(2.0);
    println!("   Short standstill: 2.0 seconds");
    println!("   Duration: {:?}", short_standstill.duration);

    let long_standstill = StandStillCondition::with_duration(10.0);
    println!("   Long standstill: 10.0 seconds");
    println!("   Duration: {:?}", long_standstill.duration);

    // 4. Using conditions in ByEntityCondition enum
    println!("\n4. ByEntityCondition Integration:");

    let triggering_entities = TriggeringEntities::default();
    let conditions = vec![
        ByEntityCondition::acceleration(triggering_entities.clone(), 4.0, Rule::GreaterThan),
        ByEntityCondition::standstill(triggering_entities.clone(), 3.0),
        ByEntityCondition::acceleration_with_direction(
            triggering_entities.clone(),
            2.5,
            Rule::LessThan,
            DirectionalDimension::Lateral,
        ),
    ];

    for (i, condition) in conditions.iter().enumerate() {
        match &condition.entity_condition {
            EntityCondition::Acceleration(acc) => {
                println!(
                    "   Condition {}: Acceleration {:?} {:?}",
                    i + 1,
                    acc.rule,
                    acc.value
                );
                if let Some(dir) = &acc.direction {
                    println!("      Direction: {:?}", dir);
                }
            }
            EntityCondition::StandStill(standstill) => {
                println!(
                    "   Condition {}: StandStill for {:?} seconds",
                    i + 1,
                    standstill.duration
                );
            }
            _ => println!("   Condition {}: Other condition type", i + 1),
        }
    }

    // 5. Real-world scenario examples
    println!("\n5. Real-World Scenario Examples:");

    // Emergency braking detection
    let emergency_braking = AccelerationCondition::longitudinal(-8.0, Rule::LessThan);
    println!("   Emergency braking: longitudinal acceleration < -8.0 m/s²");

    // Sharp turn detection
    let sharp_turn = AccelerationCondition::lateral(4.0, Rule::GreaterThan);
    println!("   Sharp turn: lateral acceleration > 4.0 m/s²");

    // Traffic jam detection
    let traffic_jam = StandStillCondition::with_duration(30.0);
    println!("   Traffic jam: standstill for 30.0 seconds");

    // Parking completion
    let parking_complete = StandStillCondition::with_duration(5.0);
    println!("   Parking complete: standstill for 5.0 seconds");

    // 6. Demonstrate serialization capability
    println!("\n6. Serialization Example:");

    let complex_condition = AccelerationCondition::new(3.5, Rule::GreaterOrEqual)
        .with_direction(DirectionalDimension::Longitudinal);

    match serde_json::to_string_pretty(&complex_condition) {
        Ok(json) => println!("   Serialized AccelerationCondition:\n{}", json),
        Err(e) => println!("   Serialization error: {}", e),
    }

    println!("\n=== Demo Complete ===");
    println!("Phase 2: Motion Conditions successfully implemented!");
    println!("- AccelerationCondition: ✅");
    println!("- StandStillCondition: ✅");
    println!("- ByEntityCondition integration: ✅");
    println!("- Builder methods: ✅");
    println!("- Serialization support: ✅");
}
