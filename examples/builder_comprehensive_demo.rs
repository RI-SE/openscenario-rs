//! Comprehensive demonstration of the NEW OpenSCENARIO-rs detached builder system
//!
//! This example showcases the NEW detached builder pattern including:
//! - Type-safe scenario construction with unlimited fluent chaining
//! - NEW detached builder pattern for complex scenarios
//! - Multiple acts and maneuvers using detached builders
//! - Parameter support and complete scenario building
//! - XML serialization and analysis
//!
//! Run with: `cargo run --example builder_comprehensive_demo --features builder`

#[cfg(feature = "builder")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use openscenario_rs::{ScenarioBuilder, builder::StoryboardBuilder};
    use openscenario_rs::types::{enums::ParameterType, scenario::triggers::Trigger};

    println!("🚗 OpenSCENARIO-rs NEW Detached Builder Comprehensive Demo");
    println!("============================================================");

    // 1. Create a complete scenario using the NEW detached builder pattern
    println!("\n📋 Building comprehensive scenario with NEW detached builders...");
    
    // Step 1: Create scenario with parameters and basic setup
    let scenario_builder = ScenarioBuilder::new()
        .with_header(
            "Highway Overtaking Scenario with Conditional Behavior", 
            "OpenSCENARIO-rs Builder Demo"
        )
        .add_parameter("initial_speed", ParameterType::Double, "25.0")
        .add_parameter("target_speed", ParameterType::Double, "35.0")
        .add_parameter("overtake_distance", ParameterType::Double, "50.0")
        .with_road_file("highway.xodr")
        .with_entities();
    
    println!("✅ Created scenario with parameters and road network");
    
    // Step 2: Build complex storyboard using NEW detached builder pattern
    let mut storyboard_builder = StoryboardBuilder::new(scenario_builder);
    let mut story_builder = storyboard_builder.add_story_simple("highway_overtaking");
    
    println!("✅ Created storyboard and main story");
    
    // Step 3: Demonstrate multiple acts using detached builders
    println!("\n🔧 Building multiple acts with detached builders...");
    
    // Act 1: Initial acceleration
    let mut act1 = story_builder.create_act("initial_acceleration");
    let mut maneuver1 = act1.create_maneuver("ego_accelerate", "ego");
    let speed_action1 = maneuver1
        .create_speed_action()
        .named("initial_acceleration")
        .to_speed(25.0)
        .with_trigger(Trigger { condition_groups: vec![] });
    
    speed_action1.attach_to_detached(&mut maneuver1)?;
    maneuver1.attach_to_detached(&mut act1);
    act1.attach_to(&mut story_builder);
    println!("✅ Created Act 1: Initial acceleration");
    
    // Act 2: Conditional overtaking  
    let mut act2 = story_builder.create_act("conditional_overtaking");
    let mut maneuver2 = act2.create_maneuver("ego_overtake", "ego");
    let speed_action2 = maneuver2
        .create_speed_action()
        .named("overtake_acceleration")
        .to_speed(35.0)
        .with_trigger(Trigger { condition_groups: vec![] });
    
    speed_action2.attach_to_detached(&mut maneuver2)?;
    maneuver2.attach_to_detached(&mut act2);
    act2.attach_to(&mut story_builder);
    println!("✅ Created Act 2: Conditional overtaking");
    
    // Act 3: Lane change maneuver
    let mut act3 = story_builder.create_act("lane_change");
    let mut maneuver3 = act3.create_maneuver("lane_change_maneuver", "ego"); 
    let teleport_action = maneuver3
        .create_teleport_action()
        .named("lane_change")
        .with_trigger(Trigger { condition_groups: vec![] });
    
    teleport_action.attach_to_detached(&mut maneuver3)?;
    maneuver3.attach_to_detached(&mut act3);
    act3.attach_to(&mut story_builder);
    println!("✅ Created Act 3: Lane change maneuver");
    
    // Step 4: Build the final comprehensive scenario
    println!("\n🔧 Building final comprehensive scenario...");
    let scenario = storyboard_builder.finish().build()?;

    println!("✅ Comprehensive scenario built successfully using NEW detached builders!");
    
    // 2. Demonstrate scenario introspection
    println!("\n🔍 Comprehensive Scenario Analysis:");
    println!("- Description: {}", scenario.file_header.description.as_literal().map_or("N/A", |v| v));
    println!("- Author: {}", scenario.file_header.author.as_literal().map_or("N/A", |v| v));
    
    // Show parameters
    if let Some(params) = &scenario.parameter_declarations {
        println!("- Parameters: {}", params.parameter_declarations.len());
        for param in &params.parameter_declarations {
            if let Some(name) = param.name.as_literal() {
                if let Some(value) = param.value.as_literal() {
                    println!("  • {}: {:?} = {}", name, param.parameter_type, value);
                }
            }
        }
    }
    
    // Show road network
    if let Some(road_network) = &scenario.road_network {
        if let Some(logic_file) = &road_network.logic_file {
            if let Some(filepath) = logic_file.filepath.as_literal() {
                println!("- Road Network: {}", filepath);
            }
        }
    }
    
    if let Some(entities) = &scenario.entities {
        println!("- Entities: {}", entities.scenario_objects.len());
    }
    
    if let Some(storyboard) = &scenario.storyboard {
        println!("- Stories: {}", storyboard.stories.len());
        for story in &storyboard.stories {
            if let Some(name) = story.name.as_literal() {
                println!("  • Story: {}", name);
                println!("    Acts: {}", story.acts.len());
                for act in &story.acts {
                    if let Some(act_name) = act.name.as_literal() {
                        println!("    • Act: {} ({} maneuver groups)", 
                                act_name, act.maneuver_groups.len());
                        for mg in &act.maneuver_groups {
                            println!("      - Maneuvers: {}", mg.maneuvers.len());
                            for maneuver in &mg.maneuvers {
                                if let Some(maneuver_name) = maneuver.name.as_literal() {
                                    println!("        * Maneuver: {} ({} events)", 
                                            maneuver_name, maneuver.events.len());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // 3. Demonstrate serialization
    println!("\n💾 Serializing comprehensive scenario to XML...");
    let xml_output = openscenario_rs::serialize_to_string(&scenario)?;
    
    // Save to file
    let output_file = "examples/comprehensive_scenario_detached.xosc";
    std::fs::write(output_file, &xml_output)?;
    println!("✅ Comprehensive scenario saved to '{}'", output_file);
    
    // Show first few lines
    let lines: Vec<&str> = xml_output.lines().take(15).collect();
    println!("\n📄 Generated XML (first 15 lines):");
    for (i, line) in lines.iter().enumerate() {
        println!("{:2}: {}", i + 1, line);
    }
    println!("... (truncated)");

    println!("\n🎉 Comprehensive demo completed successfully!");
    println!("\n🎯 Key Benefits of NEW Detached Builder Pattern:");
    println!("- ✅ Solved all lifetime variance compilation errors");
    println!("- ✅ Unlimited fluent chaining without constraints");
    println!("- ✅ Perfect separation of concerns in complex scenarios");
    println!("- ✅ Type-safe construction with compile-time validation");
    println!("- ✅ Supports parameters, road networks, and complex storyboards");
    println!("- ✅ Multiple acts and maneuvers with ease");
    println!("- ✅ Complete XML serialization and file output");
    println!("- ✅ Comprehensive scenario introspection");
    
    println!("\n📚 NEW Detached Pattern Features Demonstrated:");
    println!("1. ✅ create_*() methods for unlimited fluent chaining");
    println!("2. ✅ attach_*() methods for controlled composition");
    println!("3. ✅ Multiple acts with complex maneuvers");
    println!("4. ✅ Speed and teleport actions with triggers");
    println!("5. ✅ Parameter support and road network integration");
    println!("6. ✅ Complete scenario building and serialization");

    Ok(())
}

#[cfg(not(feature = "builder"))]
fn main() {
    println!("This example requires the 'builder' feature.");
    println!("Run with: cargo run --example builder_comprehensive_demo --features builder");
}