#[cfg(feature = "builder")]
mod detached_builders_tests {
    use openscenario_rs::builder::{ScenarioBuilder, StoryboardBuilder};
    use openscenario_rs::builder::storyboard::{DetachedActBuilder, DetachedManeuverBuilder, DetachedSpeedActionBuilder};

    #[test]
    fn test_detached_act_builder_creation() {
        // Test that DetachedActBuilder can be created and used without lifetime issues
        let scenario_builder = ScenarioBuilder::new()
            .with_header("Test", "Author")
            .with_entities();
            
        let mut storyboard_builder = StoryboardBuilder::new(scenario_builder);
        let mut story_builder = storyboard_builder.add_story_simple("TestStory");
        
        // Create detached act builder - this should work without lifetime constraints
        let detached_act = story_builder.create_act("act1");
        
        // Test fluent chaining
        let detached_act = detached_act.with_start_trigger(
            openscenario_rs::types::scenario::triggers::Trigger {
                condition_groups: vec![],
            }
        );
        
        // Attach to story builder
        detached_act.attach_to(&mut story_builder);
        
        // Test passes if no compilation errors occur
        assert!(true);
    }

    #[test]
    fn test_detached_maneuver_builder_creation() {
        // Test that DetachedManeuverBuilder can be created and used
        let mut detached_act = DetachedActBuilder::new("act1");
        
        // Create detached maneuver builder
        let detached_maneuver = detached_act.create_maneuver("maneuver1", "vehicle1");
        
        // Attach to detached act
        detached_maneuver.attach_to_detached(&mut detached_act);
        
        // Test passes if no compilation errors occur
        assert!(true);
    }

    #[test]
    fn test_detached_speed_action_builder_creation() {
        // Test that DetachedSpeedActionBuilder can be created and used
        let mut detached_maneuver = DetachedManeuverBuilder::new("maneuver1", "vehicle1");
        
        // Create detached speed action builder
        let detached_speed = detached_maneuver.create_speed_action();
        
        // Test fluent chaining
        let detached_speed = detached_speed
            .named("speed_event")
            .to_speed(30.0);
        
        // Attach to detached maneuver
        let result = detached_speed.attach_to_detached(&mut detached_maneuver);
        assert!(result.is_ok());
    }

    #[test]
    fn test_complete_detached_workflow() {
        // Test the complete workflow using only detached builders
        let scenario_builder = ScenarioBuilder::new()
            .with_header("Test", "Author")
            .with_entities();
            
        let mut storyboard_builder = StoryboardBuilder::new(scenario_builder);
        let mut story_builder = storyboard_builder.add_story_simple("TestStory");
        
        // Create detached builders
        let mut detached_act = story_builder.create_act("act1");
        let mut detached_maneuver = detached_act.create_maneuver("maneuver1", "vehicle1");
        let detached_speed = detached_maneuver.create_speed_action()
            .named("speed_event")
            .to_speed(30.0);
        
        // Chain the attachments
        detached_speed.attach_to_detached(&mut detached_maneuver).unwrap();
        detached_maneuver.attach_to_detached(&mut detached_act);
        detached_act.attach_to(&mut story_builder);
        
        // Test passes if no compilation errors occur
        assert!(true);
    }

    #[test]
    fn test_perfect_fluent_chaining() {
        // Test that demonstrates perfect fluent chaining without lifetime constraints
        let mut detached_act = DetachedActBuilder::new("act1");
        let mut detached_maneuver = DetachedManeuverBuilder::new("maneuver1", "vehicle1");
        
        // This should compile without any lifetime issues
        let detached_speed = detached_maneuver
            .create_speed_action()
            .named("speed_event")
            .to_speed(30.0)
            .with_trigger(openscenario_rs::types::scenario::triggers::Trigger {
                condition_groups: vec![],
            });
        
        // Attach and verify no compilation errors
        let result = detached_speed.attach_to_detached(&mut detached_maneuver);
        assert!(result.is_ok());
        
        detached_maneuver.attach_to_detached(&mut detached_act);
        
        // Test passes if no compilation errors occur
        assert!(true);
    }
}