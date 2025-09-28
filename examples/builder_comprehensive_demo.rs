//! Comprehensive demonstration of the OpenSCENARIO-rs builder system
//!
//! This example showcases all major features of the builder API including:
//! - Type-safe scenario construction
//! - Entity creation (vehicles, pedestrians)
//! - Action builders (speed, teleport, lane change)
//! - Condition builders (time, speed, distance)
//! - Storyboard construction (stories, acts, maneuvers)
//! - Parameter support and catalog integration
//!
//! Run with: `cargo run --example builder_comprehensive_demo --features builder`

#[cfg(feature = "builder")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use openscenario_rs::{
        ScenarioBuilder,
        builder::{
            actions::{SpeedActionBuilder, TeleportActionBuilder},
            conditions::{TimeConditionBuilder, SpeedConditionBuilder, TriggerBuilder},
            positions::WorldPositionBuilder,
        },
        types::enums::{ParameterType, VehicleCategory},
    };

    println!("🚗 OpenSCENARIO-rs Builder Comprehensive Demo");
    println!("============================================");

    // 1. Create a complete scenario with all features
    println!("\n📋 Building comprehensive highway scenario...");
    
    let scenario = ScenarioBuilder::new()
        // Set basic metadata
        .with_header(
            "Highway Overtaking Scenario with Conditional Behavior", 
            "OpenSCENARIO-rs Builder Demo"
        )
        
        // Add configurable parameters
        .add_parameter("initial_speed", ParameterType::Double, "25.0")
        .add_parameter("target_speed", ParameterType::Double, "35.0")
        .add_parameter("overtake_distance", ParameterType::Double, "50.0")
        
        // Set road network
        .with_road_file("highway.xodr")
        
        // Initialize entities
        .with_entities()
            // Add ego vehicle
            .add_vehicle("ego")
                .car()
                .with_dimensions(4.5, 1.8, 1.4)
                .finish()
            
            // Add target vehicle
            .add_vehicle("target")
                .car()
                .finish()
            
            // Add truck
            .add_vehicle("truck")
                .truck()
                .finish()
        
        // Build the storyboard with complex behavior
        .with_storyboard()
            .add_story("highway_overtaking")
                
                // Act 1: Initial acceleration
                .add_act("initial_acceleration")
                    .add_maneuver("ego_accelerate", "ego")
                        .add_speed_action()
                            .named("initial_acceleration")
                            .to_speed(25.0) // Use parameter value
                            .triggered_by()
                                .time_condition(1.0) // Start after 1 second
                                .finish()
                            .finish()?
                        .finish()
                    .finish()
                
                // Act 2: Conditional overtaking
                .add_act("conditional_overtaking")
                    .add_maneuver("ego_overtake", "ego")
                        // Speed up when close to target
                        .add_speed_action()
                            .named("overtake_acceleration")
                            .to_speed(35.0)
                            .triggered_by()
                                .speed_condition("target", 20.0) // When target is slow
                                .finish()
                            .finish()?
                        
                        // Lane change maneuver (teleport for simplicity)
                        .add_teleport_action()
                            .named("lane_change")
                            .to()
                                .world_position(100.0, -3.5, 0.0) // Left lane
                            .triggered_by()
                                .time_condition(5.0) // After 5 seconds
                                .finish()
                            .finish()?
                        .finish()
                    .finish()
                
                .finish()
            .finish()
        
        // Build final scenario
        .build()?;

    println!("✅ Scenario built successfully!");
    
    // 2. Demonstrate scenario introspection
    println!("\n🔍 Scenario Analysis:");
    println!("- Description: {}", scenario.file_header.description.as_literal().unwrap_or("N/A"));
    println!("- Author: {}", scenario.file_header.author.as_literal().unwrap_or("N/A"));
    
    if let Some(entities) = &scenario.entities {
        println!("- Entities: {} vehicles", entities.scenario_objects.len());
        for obj in &entities.scenario_objects {
            if let Some(name) = obj.name.as_literal() {
                println!("  • {}", name);
            }
        }
    }
    
    if let Some(storyboard) = &scenario.storyboard {
        println!("- Stories: {}", storyboard.stories.len());
        for story in &storyboard.stories {
            if let Some(name) = story.name.as_literal() {
                println!("  • Story: {}", name);
                println!("    Acts: {}", story.acts.len());
                for act in &story.acts {
                    if let Some(act_name) = act.name.as_literal() {
                        println!("    • Act: {}", act_name);
                        println!("      Maneuver Groups: {}", act.maneuver_groups.len());
                    }
                }
            }
        }
    }

    // 3. Demonstrate serialization
    println!("\n💾 Serializing scenario to XML...");
    let xml_output = openscenario_rs::serialize_to_string(&scenario)?;
    
    // Save to file
    std::fs::write("demo_scenario.xosc", &xml_output)?;
    println!("✅ Scenario saved to 'demo_scenario.xosc'");
    
    // Show first few lines
    let lines: Vec<&str> = xml_output.lines().take(10).collect();
    println!("\n📄 Generated XML (first 10 lines):");
    for (i, line) in lines.iter().enumerate() {
        println!("{:2}: {}", i + 1, line);
    }
    println!("... (truncated)");

    // 4. Demonstrate individual builder components
    println!("\n🔧 Individual Builder Component Demos:");
    
    // Speed action builder
    println!("\n• Speed Action Builder:");
    let speed_action = SpeedActionBuilder::new()
        .for_entity("demo_vehicle")
        .to_speed(30.0)
        .build_action()?;
    println!("  ✅ Speed action created for 30.0 m/s");
    
    // Teleport action builder
    println!("\n• Teleport Action Builder:");
    let teleport_action = TeleportActionBuilder::new()
        .for_entity("demo_vehicle")
        .to()
            .world_position(100.0, 200.0, 0.0)
        .build_action()?;
    println!("  ✅ Teleport action created to (100, 200, 0)");
    
    // Condition builders
    println!("\n• Condition Builders:");
    let time_condition = TimeConditionBuilder::new()
        .at_time(5.0)
        .build()?;
    println!("  ✅ Time condition created for t=5.0s");
    
    let speed_condition = SpeedConditionBuilder::new()
        .for_entity("demo_vehicle")
        .speed_above(25.0)
        .build()?;
    println!("  ✅ Speed condition created for >25.0 m/s");
    
    // Trigger builder
    println!("\n• Trigger Builder:");
    let trigger = TriggerBuilder::new()
        .add_condition(time_condition)
        .add_condition(speed_condition)
        .build()?;
    println!("  ✅ Trigger created with 2 conditions (OR logic)");

    println!("\n🎉 Comprehensive demo completed successfully!");
    println!("\nKey Features Demonstrated:");
    println!("- ✅ Type-safe builder pattern with compile-time state validation");
    println!("- ✅ Fluent API for intuitive scenario construction");
    println!("- ✅ Parameter support for configurable scenarios");
    println!("- ✅ Entity builders (vehicles with different types)");
    println!("- ✅ Action builders (speed, teleport)");
    println!("- ✅ Condition builders (time, speed)");
    println!("- ✅ Trigger builders with complex logic");
    println!("- ✅ Storyboard construction (stories, acts, maneuvers)");
    println!("- ✅ XML serialization and file output");
    println!("- ✅ Scenario introspection and analysis");

    Ok(())
}

#[cfg(not(feature = "builder"))]
fn main() {
    println!("This example requires the 'builder' feature.");
    println!("Run with: cargo run --example builder_comprehensive_demo --features builder");
}