//! Demonstration of trajectory-based actions using the builder API
//!
//! This example shows how to create scenarios with follow trajectory actions,
//! including building trajectories with polylines and attaching them to maneuvers.
//!
//! Run with:
//! ```bash
//! cargo run --example trajectory_action_demo --features builder
//! ```

use openscenario_rs::builder::{
    DetachedFollowTrajectoryActionBuilder, ScenarioBuilder, StoryboardBuilder,
    TrajectoryBuilder,
};
use openscenario_rs::serialize_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== OpenSCENARIO Trajectory Action Demo ===\n");

    // Create a scenario with basic setup
    let scenario_builder = ScenarioBuilder::new()
        .with_header("Trajectory Following Scenario", "OpenSCENARIO-rs Demo")
        .with_road_file("highway.xodr")
        .with_entities();

    println!("1. Building trajectory with multiple waypoints...");

    // Create a trajectory with multiple vertices
    let ego_trajectory = TrajectoryBuilder::new()
        .name("ego_vehicle_path")
        .closed(false)
        .polyline()
        .add_vertex()
        .time(0.0)
        .world_position(0.0, 0.0, 0.0, 0.0)
        .finish()?
        .add_vertex()
        .time(2.0)
        .world_position(20.0, 5.0, 0.0, 0.1)
        .finish()?
        .add_vertex()
        .time(4.0)
        .world_position(40.0, 15.0, 0.0, 0.3)
        .finish()?
        .add_vertex()
        .time(6.0)
        .world_position(60.0, 20.0, 0.0, 0.5)
        .finish()?
        .add_vertex()
        .time(8.0)
        .world_position(80.0, 20.0, 0.0, 0.0)
        .finish()?
        .finish()
        .build()?;

    println!("   ✓ Created trajectory with 5 waypoints");
    println!("     - Start: (0.0, 0.0) at t=0.0s");
    println!("     - End:   (80.0, 20.0) at t=8.0s");

    // Create a second trajectory for another vehicle
    let target_trajectory = TrajectoryBuilder::new()
        .name("target_vehicle_path")
        .closed(false)
        .polyline()
        .add_vertex()
        .time(0.0)
        .world_position(50.0, 0.0, 0.0, 0.0)
        .finish()?
        .add_vertex()
        .time(5.0)
        .world_position(100.0, 0.0, 0.0, 0.0)
        .finish()?
        .finish()
        .build()?;

    println!("   ✓ Created second trajectory with 2 waypoints\n");

    println!("2. Building storyboard with follow trajectory actions...");

    // Build storyboard using detached builder pattern
    let mut storyboard_builder = StoryboardBuilder::new(scenario_builder);
    let mut story = storyboard_builder.add_story_simple("trajectory_following_story");
    let mut act = story.create_act("trajectory_movement");

    // Create maneuver for ego vehicle
    let mut ego_maneuver = act.create_maneuver("follow_path", "ego_vehicle");

    // Create follow trajectory action for ego using detached builder
    let ego_action = ego_maneuver
        .create_follow_trajectory_action()
        .named("ego_follow_trajectory")
        .with_trajectory(ego_trajectory)
        .following_mode_follow() // Entity follows trajectory timing
        .with_time_trigger(0.0)?;

    // Attach to maneuver
    ego_action.attach_to_detached(&mut ego_maneuver)?;
    println!("   ✓ Created ego vehicle maneuver with follow trajectory action");

    // Create maneuver for target vehicle
    let mut target_maneuver = act.create_maneuver("straight_path", "target_vehicle");

    // Create follow trajectory action for target with position mode
    let target_action = target_maneuver
        .create_follow_trajectory_action()
        .named("target_follow_trajectory")
        .with_trajectory(target_trajectory)
        .following_mode_position() // Entity reaches positions at specified times
        .initial_distance_offset(10.0) // Start 10m ahead on trajectory
        .start_immediately()?;

    // Attach to maneuver
    target_action.attach_to_detached(&mut target_maneuver)?;
    println!("   ✓ Created target vehicle maneuver with position-based following");

    // Assemble the scenario
    ego_maneuver.attach_to_detached(&mut act);
    target_maneuver.attach_to_detached(&mut act);
    act.attach_to(&mut story);

    let scenario = storyboard_builder.finish().build()?;
    println!("   ✓ Completed scenario assembly\n");

    // Serialize to XML
    println!("3. Serializing scenario to XML...");
    let xml = serialize_to_string(&scenario)?;

    println!("   ✓ Generated OpenSCENARIO XML ({} bytes)\n", xml.len());

    // Display key parts of the scenario
    println!("=== Scenario Summary ===");
    println!("File Header:");
    println!("  Author:      {}", scenario.file_header.author.as_literal().unwrap());
    println!(
        "  Description: {}",
        scenario.file_header.description.as_literal().unwrap()
    );

    if let Some(ref storyboard) = scenario.storyboard {
        println!("\nStoryboard Structure:");
        println!("  Stories: {}", storyboard.stories.len());
        for story in &storyboard.stories {
            println!("    Story: {}", story.name.as_literal().unwrap());
            for act in &story.acts {
                println!("      Act: {}", act.name.as_literal().unwrap());
                for maneuver_group in &act.maneuver_groups {
                    println!(
                        "        Maneuver Group with {} maneuvers",
                        maneuver_group.maneuvers.len()
                    );
                    for maneuver in &maneuver_group.maneuvers {
                        println!(
                            "          Maneuver: {} ({} events)",
                            maneuver.name.as_literal().unwrap(),
                            maneuver.events.len()
                        );
                        for event in &maneuver.events {
                            println!("            Event: {}", event.name.as_literal().unwrap());
                            if let Some(ref actions) = event.actions.first() {
                                if let Some(ref private_action) = actions.private_action {
                                    if private_action.routing_action.is_some() {
                                        println!("              Type: FollowTrajectoryAction");
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Write to file
    let output_file = "generated_trajectory_scenario.xosc";
    std::fs::write(output_file, xml)?;
    println!("\n✓ Saved scenario to: {}", output_file);

    println!("\n=== Demo Complete ===");
    println!("\nKey Features Demonstrated:");
    println!("  • TrajectoryBuilder with polyline shapes");
    println!("  • VertexBuilder for time-positioned waypoints");
    println!("  • FollowTrajectoryActionBuilder with two following modes:");
    println!("    - 'follow': Entity follows trajectory timing");
    println!("    - 'position': Entity reaches positions at specified times");
    println!("  • DetachedFollowTrajectoryActionBuilder for unlimited fluent chaining");
    println!("  • Initial distance offset configuration");
    println!("  • Time-based trigger setup");

    Ok(())
}
