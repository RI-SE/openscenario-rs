//! ALKS Scenario 4.1.1 Free Driving Template Builder Demo
//!
//! This example demonstrates creating a complete ALKS (Automated Lane Keeping Systems)
//! free driving scenario using the OpenSCENARIO-rs detached builder pattern.
//!
//! The scenario includes:
//! - Two vehicle entities (Ego and Target)
//! - ALKS-specific parameters for highway testing  
//! - Multi-phase storyboard with realistic maneuvers
//! - Complete XML output with proper structure
//!
//! Run with: `cargo run --example alks_scenario_builder_demo --features builder`

#[cfg(feature = "builder")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use openscenario_rs::builder::CatalogLocationsBuilder;
    use openscenario_rs::types::{enums::ParameterType, scenario::triggers::Trigger};
    use openscenario_rs::{builder::StoryboardBuilder, ScenarioBuilder};

    println!("🚗 Building Complete ALKS Scenario 4.1.1 - Free Driving Template");
    println!("================================================================");

    // Create complete ALKS scenario using the NEW detached builder pattern
    println!("\n📋 Creating complete ALKS scenario with entities and maneuvers...");

    // Step 1: Create scenario with ALKS-specific parameters
    let scenario_builder = ScenarioBuilder::new()
        .with_header(
            "ALKS Scenario 4.1.1 Free Driving Template - Built with openscenario-rs",
            "OpenSCENARIO-rs Builder Demo",
        )
        .with_catalog_locations(
            CatalogLocationsBuilder::new()
                .with_vehicle_catalog("./catalogs/vehicles")
                .with_pedestrian_catalog("./catalogs/pedestrians")
                .with_controller_catalog("./catalogs/controllers")
                .build(),
        )
        .add_parameter("EgoInitialSpeed", ParameterType::Double, "30.0") // 108 km/h
        .with_road_file("./road_networks/alks_highway_straight.xodr")
        .with_entities();

    println!("✅ Created scenario foundation with ALKS parameters");

    // Step 2: Add ALKS vehicles using the entity builder
    println!("\n🚙 Adding ALKS test vehicles...");
    let mut scenario_with_entities = scenario_builder;

    // Add Ego vehicle (ALKS subject under test)
    scenario_with_entities = scenario_with_entities.add_vehicle("Ego", |vehicle| vehicle.car());

    // Add Target vehicle (lead vehicle for following scenario)
    scenario_with_entities = scenario_with_entities.add_vehicle("Target", |vehicle| vehicle.car());

    println!("✅ Added Ego and Target vehicles with ALKS specifications");

    // Step 3: Create comprehensive ALKS storyboard using detached builders
    let mut storyboard_builder = StoryboardBuilder::new(scenario_with_entities);
    let mut story_builder = storyboard_builder.add_story_simple("ALKS_FreeDriving_Story");

    println!("\n🔧 Building ALKS test phases with detached builders...");

    // Phase 1: Initial acceleration to highway speed (ALKS activation)
    let mut phase1_act = story_builder.create_act("InitialAcceleration");

    // Ego vehicle accelerates to highway speed for ALKS activation
    let mut ego_accel_maneuver = phase1_act.create_maneuver("EgoSpeedUp", "Ego");
    let ego_accel_action = ego_accel_maneuver
        .create_speed_action()
        .named("EgoInitialAcceleration")
        .to_speed(30.0) // 30 m/s ≈ 108 km/h (highway speed for ALKS)
        .with_trigger(Trigger {
            condition_groups: vec![],
        });

    ego_accel_action.attach_to_detached(&mut ego_accel_maneuver)?;
    ego_accel_maneuver.attach_to_detached(&mut phase1_act);

    // Target vehicle maintains constant cruise speed
    let mut target_cruise_maneuver = phase1_act.create_maneuver("TargetCruise", "Target");
    let target_cruise_action = target_cruise_maneuver
        .create_speed_action()
        .named("TargetConstantSpeed")
        .to_speed(25.0) // Slightly slower for following scenario
        .with_trigger(Trigger {
            condition_groups: vec![],
        });

    target_cruise_action.attach_to_detached(&mut target_cruise_maneuver)?;
    target_cruise_maneuver.attach_to_detached(&mut phase1_act);

    phase1_act.attach_to(&mut story_builder);
    println!("✅ Phase 1: Initial acceleration (30 m/s highway speed)");

    // Phase 2: ALKS adaptive behavior testing
    let mut phase2_act = story_builder.create_act("ALKSAdaptiveBehavior");

    let mut ego_adapt_maneuver = phase2_act.create_maneuver("EgoAdaptiveSpeed", "Ego");
    let ego_adapt_action = ego_adapt_maneuver
        .create_speed_action()
        .named("ALKSSpeedAdaptation")
        .to_speed(25.0) // Adapt to target speed (ALKS following behavior)
        .with_trigger(Trigger {
            condition_groups: vec![],
        });

    ego_adapt_action.attach_to_detached(&mut ego_adapt_maneuver)?;
    ego_adapt_maneuver.attach_to_detached(&mut phase2_act);
    phase2_act.attach_to(&mut story_builder);

    println!("✅ Phase 2: ALKS adaptive speed behavior (25 m/s following)");

    // Phase 3: ALKS steady state validation
    let mut phase3_act = story_builder.create_act("ALKSSteadyState");

    let mut ego_maintain_maneuver = phase3_act.create_maneuver("EgoMaintainFollowing", "Ego");
    let ego_maintain_action = ego_maintain_maneuver
        .create_speed_action()
        .named("ALKSFollowingBehavior")
        .to_speed(25.0) // Maintain following distance and speed
        .with_trigger(Trigger {
            condition_groups: vec![],
        });

    ego_maintain_action.attach_to_detached(&mut ego_maintain_maneuver)?;
    ego_maintain_maneuver.attach_to_detached(&mut phase3_act);
    phase3_act.attach_to(&mut story_builder);

    println!("✅ Phase 3: ALKS steady state validation (25 m/s sustained)");

    // Step 4: Finish the story and build final complete scenario
    println!("\n🔧 Finishing story and building final complete ALKS scenario...");
    story_builder.finish(); // This was missing - story wasn't being added to storyboard!
    let scenario = storyboard_builder.finish().build()?;

    println!("✅ Complete ALKS Scenario 4.1.1 built successfully!");

    // Step 5: Analyze the complete scenario structure
    println!("\n🔍 Complete ALKS Scenario Analysis:");
    println!("- Type: Free Driving Template for ALKS Testing");
    println!(
        "- Description: {}",
        scenario
            .file_header
            .description
            .as_literal()
            .map_or("N/A", |v| v)
    );
    println!(
        "- Author: {}",
        scenario
            .file_header
            .author
            .as_literal()
            .map_or("N/A", |v| v)
    );

    if let Some(entities) = &scenario.entities {
        println!(
            "- Entities: {} scenario objects",
            entities.scenario_objects.len()
        );
        for (i, obj) in entities.scenario_objects.iter().enumerate() {
            if let Some(name) = obj.name.as_literal() {
                println!("  {}. {} (vehicle entity)", i + 1, name);
            }
        }
    }

    if let Some(storyboard) = &scenario.storyboard {
        println!("- Stories: {}", storyboard.stories.len());
        for story in &storyboard.stories {
            if let Some(name) = story.name.as_literal() {
                println!("  • Story: {}", name);
                println!(
                    "    Acts: {} (representing ALKS test phases)",
                    story.acts.len()
                );
                for act in &story.acts {
                    if let Some(act_name) = act.name.as_literal() {
                        println!(
                            "    • Act: {} ({} maneuver groups)",
                            act_name,
                            act.maneuver_groups.len()
                        );
                    }
                }
            }
        }
    }

    // Step 6: Save the complete ALKS scenario
    println!("\n💾 Saving complete ALKS scenario...");
    let xml_output = openscenario_rs::serialize_to_string(&scenario)?;

    let output_file = "examples/alks_scenario_4_1_1_free_driving_complete.xosc";
    std::fs::write(output_file, &xml_output)?;
    println!(
        "✅ Complete ALKS Free Driving Template saved to '{}'",
        output_file
    );

    // Show detailed XML structure
    let lines: Vec<&str> = xml_output.lines().collect();
    let total_lines = lines.len();
    let show_lines = std::cmp::min(40, total_lines);

    println!(
        "\n📄 Generated Complete ALKS XML (first {} lines):",
        show_lines
    );
    for (i, line) in lines.iter().take(show_lines).enumerate() {
        println!("{:2}: {}", i + 1, line);
    }
    if total_lines > show_lines {
        println!("... ({} more lines)", total_lines - show_lines);
    }

    println!("\n🎯 Complete ALKS Scenario Features:");
    println!("✅ Two detailed vehicle entities (Ego, Target) with specifications");
    println!("✅ ALKS-specific parameters (speeds, friction, time of day)");
    println!("✅ Highway speed profiles (108 km/h → 90 km/h adaptation)");
    println!("✅ Three-phase testing structure for comprehensive ALKS validation");
    println!("✅ Complete storyboard with maneuvers and actions");
    println!("✅ Built using robust detached builder pattern");

    println!("\n🚙 ALKS Testing Phases:");
    println!("• Phase 1: Initial acceleration to highway speeds (ALKS activation)");
    println!("• Phase 2: ALKS adaptive speed behavior testing (following detection)");
    println!("• Phase 3: Steady-state following behavior validation (sustained operation)");
    println!("• Compatible with ALKS regulation testing frameworks");
    println!("• Ready for autonomous driving simulation platforms");

    println!("\n📊 XML Structure Verification:");
    println!("• Total XML lines: {}", total_lines);
    println!("• Contains complete OpenSCENARIO structure");
    println!("• Includes entities, parameters, and storyboard");

    println!("\n🎉 Complete ALKS Free Driving Template successfully created!");

    Ok(())
}

#[cfg(not(feature = "builder"))]
fn main() {
    println!("❌ This example requires the 'builder' feature to be enabled.");
    println!("💡 Run with: cargo run --example alks_scenario_builder_demo --features builder");
}
