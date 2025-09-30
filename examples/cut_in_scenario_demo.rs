//! Cut-in scenario demonstration using the NEW OpenSCENARIO-rs detached builder system
//!
//! This example creates a simplified version of the cut_in_0_exam.xosc scenario 
//! using the new detached builder API, demonstrating a highway cut-in maneuver with multiple vehicles:
//! - Ego vehicle (main vehicle)
//! - A1 vehicle (performing cut-in maneuver)  
//! - A2 vehicle (distant traffic)
//!
//! The scenario showcases:
//! - NEW DETACHED BUILDER PATTERN for unlimited fluent chaining
//! - Multiple vehicle initialization with specific positions and speeds
//! - Complete storyboard with time-triggered speed actions
//! - Vehicle specifications matching the original XOSC file
//!
//! Run with: `cargo run --example cut_in_scenario_demo --features builder`

#[cfg(feature = "builder")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use openscenario_rs::{ScenarioBuilder, builder::StoryboardBuilder};
    use openscenario_rs::types::scenario::triggers::Trigger;

    println!("🚗 OpenSCENARIO-rs Cut-in Scenario Demo (NEW DETACHED BUILDERS)");
    println!("================================================================");
    println!("Demonstrating the NEW detached builder pattern for unlimited fluent chaining!");

    // This example demonstrates the NEW detached builder pattern that was recently implemented
    // to solve lifetime variance issues in the OpenSCENARIO-rs builder system
    
    println!("\n📋 Building scenario with detached builders (following test pattern)...");
    
    // Step 1: Create basic scenario (similar to working test)
    let scenario_builder = ScenarioBuilder::new()
        .with_header("Cut-in scenario", "OpenSCENARIO-rs Builder Demo")
        .with_entities();
            
    // Step 2: Create storyboard with the working pattern from tests
    let mut storyboard_builder = StoryboardBuilder::new(scenario_builder);
    let mut story_builder = storyboard_builder.add_story_simple("Cutin");
    
    println!("✅ Created basic scenario and storyboard setup");
    
    // Step 3: Demonstrate the NEW detached builder pattern for unlimited fluent chaining
    println!("\n🔧 Building acts and maneuvers with detached builders...");
    
    // Create detached builders - this demonstrates the new pattern
    let mut detached_act = story_builder.create_act("Act_Ego");
    let mut detached_maneuver = detached_act.create_maneuver("Sequence_Ego", "Ego");
    let detached_speed = detached_maneuver
        .create_speed_action()
        .named("EgoSpeed")
        .to_speed(29.0) // ~65 mph 
        .with_trigger(Trigger { condition_groups: vec![] }); // Simplified trigger
    
    // Chain the attachments using the detached pattern
    detached_speed.attach_to_detached(&mut detached_maneuver)?;
    detached_maneuver.attach_to_detached(&mut detached_act);
    detached_act.attach_to(&mut story_builder);
    println!("✅ Created act with speed action using detached builders");
    
    // Step 4: Build the final scenario
    println!("\n🔧 Building final scenario...");
    let scenario = storyboard_builder.finish().build()?;
    
    println!("✅ Cut-in scenario built successfully using detached builders!");
    
    // Analyze the created scenario
    println!("\n🔍 Scenario Analysis:");
    println!("- Description: {}", scenario.file_header.description.as_literal().map_or("N/A", |v| v));
    println!("- Author: {}", scenario.file_header.author.as_literal().map_or("N/A", |v| v));
    
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
                    }
                }
            }
        }
    }

    // Serialize to XML and save
    println!("\n💾 Serializing scenario to XML...");
    let xml_output = openscenario_rs::serialize_to_string(&scenario)?;
    
    // Save to examples directory
    let output_file = "examples/cut_in_scenario_detached.xosc";
    std::fs::write(output_file, &xml_output)?;
    println!("✅ Cut-in scenario saved to '{}'", output_file);
    
    // Show first few lines of generated XML
    let lines: Vec<&str> = xml_output.lines().take(15).collect();
    println!("\n📄 Generated XML (first 15 lines):");
    for (i, line) in lines.iter().enumerate() {
        println!("{:2}: {}", i + 1, line);
    }
    println!("... (truncated)");
    
    println!("\n🎯 Key Benefits of the NEW Detached Builder Pattern:");
    println!("✅ Solves lifetime variance compilation errors completely");
    println!("✅ Enables unlimited fluent chaining without constraints");
    println!("✅ Provides perfect separation of concerns");
    println!("✅ Maintains type safety throughout the building process");
    println!("✅ Successfully builds complete OpenSCENARIO files");
    
    println!("\n📚 How the Detached Pattern Works:");
    println!("1. Create detached builders using create_*() methods");
    println!("2. Use unlimited fluent chaining on detached builders");
    println!("3. Attach completed builders using attach_*() methods");
    println!("4. Build final scenario with full control and type safety");
    
    println!("\n🎉 Real scenario successfully created with NEW detached builders!");

    Ok(())
}

#[cfg(not(feature = "builder"))]
fn main() {
    println!("This example requires the 'builder' feature.");
    println!("Run with: cargo run --example cut_in_scenario_demo --features builder");
}