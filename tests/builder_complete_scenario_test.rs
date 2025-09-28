#[cfg(feature = "builder")]
mod complete_scenario_tests {
    use openscenario_rs::ScenarioBuilder;

    #[test]
    fn test_complete_scenario_with_actions() {
        let scenario = ScenarioBuilder::new()
            .with_header("Highway Test", "Test Author")
            .with_entities()
                .add_vehicle("ego")
                    .car()
                    .finish()
                .add_vehicle("target")
                    .car()
                    .finish()
            .with_storyboard()
                .add_story("main_story")
                    .add_act("acceleration_act")
                        .add_maneuver("speed_up", "ego")
                            .add_speed_action()
                                .named("accelerate")
                                .to_speed(30.0)
                                .finish()
                                .unwrap()
                            .finish()
                        .finish()
                    .finish()
                .finish()
            .build()
            .unwrap();
            
        // Verify complete structure
        assert!(scenario.entities.is_some());
        assert!(scenario.storyboard.is_some());
        
        let storyboard = scenario.storyboard.unwrap();
        assert_eq!(storyboard.stories.len(), 1);
        
        let story = &storyboard.stories[0];
        assert_eq!(story.acts.len(), 1);
        
        let act = &story.acts[0];
        assert_eq!(act.maneuver_groups.len(), 1);
    }

    #[test]
    fn test_minimal_storyboard_creation() {
        let scenario = ScenarioBuilder::new()
            .with_header("Test", "Author")
            .with_entities()
            .with_storyboard()
            .finish()
            .build()
            .unwrap();
            
        assert!(scenario.storyboard.is_some());
        let storyboard = scenario.storyboard.unwrap();
        assert_eq!(storyboard.stories.len(), 0);
        assert!(storyboard.init.actions.global_actions.is_empty());
        assert!(storyboard.init.actions.private_actions.is_empty());
    }
}