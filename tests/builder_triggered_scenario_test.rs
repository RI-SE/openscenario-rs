#[cfg(feature = "builder")]
mod triggered_scenario_tests {
    use openscenario_rs::ScenarioBuilder;

    #[test]
    fn test_scenario_with_triggered_events() {
        let mut scenario_builder = ScenarioBuilder::new()
            .with_header("Triggered Test", "Test Author")
            .with_entities();
            
        scenario_builder = scenario_builder.add_vehicle("ego", |v| v.car());
            
        let mut storyboard_builder = scenario_builder.with_storyboard();
        let mut story_builder = storyboard_builder.add_story_simple("triggered_story");
        let mut act_builder = story_builder.add_act("triggered_act");
        let mut maneuver_builder = act_builder.add_maneuver("speed_maneuver", "ego");
        
        maneuver_builder.add_speed_action()
            .named("delayed_acceleration")
            .to_speed(30.0)
            .triggered_by()
                .time_condition(5.0)
                .finish()
            .finish()
            .unwrap();
            
        let scenario = maneuver_builder
            .finish()
            .finish()
            .finish()
            .finish()
            .build()
            .unwrap();
            
        // Verify trigger structure
        let storyboard = scenario.storyboard.unwrap();
        let story = &storyboard.stories[0];
        let act = &story.acts[0];
        let maneuver = &act.maneuver_groups[0].maneuvers[0];
        let event = &maneuver.events[0];
        
        assert!(event.start_trigger.is_some());
        let trigger = event.start_trigger.as_ref().unwrap();
        assert_eq!(trigger.condition_groups.len(), 1);
        assert_eq!(trigger.condition_groups[0].conditions.len(), 1);
        
        // Verify the condition is a time condition
        let condition = &trigger.condition_groups[0].conditions[0];
        assert!(condition.by_value_condition.is_some());
        let by_value = condition.by_value_condition.as_ref().unwrap();
        assert!(by_value.simulation_time_condition.is_some());
        
        let time_condition = by_value.simulation_time_condition.as_ref().unwrap();
        assert_eq!(time_condition.value.as_literal().unwrap(), &5.0);
    }

    #[test]
    fn test_scenario_with_speed_triggered_events() {
        let mut scenario_builder = ScenarioBuilder::new()
            .with_header("Speed Triggered Test", "Test Author")
            .with_entities();
            
        scenario_builder = scenario_builder.add_vehicle("ego", |v| v.car());
            
        let mut storyboard_builder = scenario_builder.with_storyboard();
        let mut story_builder = storyboard_builder.add_story_simple("speed_story");
        let mut act_builder = story_builder.add_act("speed_act");
        let mut maneuver_builder = act_builder.add_maneuver("brake_maneuver", "ego");
        
        maneuver_builder.add_speed_action()
            .named("emergency_brake")
            .to_speed(0.0)
            .triggered_by()
                .speed_condition("ego", 50.0)
                .finish()
            .finish()
            .unwrap();
            
        let scenario = maneuver_builder
            .finish()
            .finish()
            .finish()
            .finish()
            .build()
            .unwrap();
            
        // Verify trigger structure
        let storyboard = scenario.storyboard.unwrap();
        let story = &storyboard.stories[0];
        let act = &story.acts[0];
        let maneuver = &act.maneuver_groups[0].maneuvers[0];
        let event = &maneuver.events[0];
        
        assert!(event.start_trigger.is_some());
        let trigger = event.start_trigger.as_ref().unwrap();
        assert_eq!(trigger.condition_groups.len(), 1);
        assert_eq!(trigger.condition_groups[0].conditions.len(), 1);
        
        // Verify the condition is a speed condition
        let condition = &trigger.condition_groups[0].conditions[0];
        assert!(condition.by_entity_condition.is_some());
        let by_entity = condition.by_entity_condition.as_ref().unwrap();
        
        match &by_entity.entity_condition {
            openscenario_rs::types::conditions::entity::EntityCondition::Speed(speed_condition) => {
                assert_eq!(speed_condition.value.as_literal().unwrap(), &50.0);
                assert_eq!(speed_condition.entity_ref.as_literal().unwrap(), "ego");
            }
            _ => panic!("Expected Speed condition"),
        }
    }

    #[test]
    fn test_scenario_with_multiple_conditions() {
        let mut scenario_builder = ScenarioBuilder::new()
            .with_header("Multi Condition Test", "Test Author")
            .with_entities();
            
        scenario_builder = scenario_builder.add_vehicle("ego", |v| v.car());
            
        let mut storyboard_builder = scenario_builder.with_storyboard();
        let mut story_builder = storyboard_builder.add_story_simple("multi_story");
        let mut act_builder = story_builder.add_act("multi_act");
        let mut maneuver_builder = act_builder.add_maneuver("complex_maneuver", "ego");
        
        // Create a trigger with multiple conditions (AND logic)
        let time_condition = openscenario_rs::builder::conditions::TimeConditionBuilder::new()
            .at_time(3.0)
            .build()
            .unwrap();
            
        let speed_condition = openscenario_rs::builder::conditions::SpeedConditionBuilder::new()
            .for_entity("ego")
            .speed_above(25.0)
            .build()
            .unwrap();
            
        let trigger = openscenario_rs::builder::conditions::TriggerBuilder::new()
            .add_condition_group()
                .add_condition(time_condition)
                .add_condition(speed_condition)
                .finish_group()
            .build()
            .unwrap();
        
        maneuver_builder.add_speed_action()
            .named("complex_action")
            .to_speed(40.0)
            .with_trigger(trigger)
            .finish()
            .unwrap();
            
        let scenario = maneuver_builder
            .finish()
            .finish()
            .finish()
            .finish()
            .build()
            .unwrap();
            
        // Verify trigger structure
        let storyboard = scenario.storyboard.unwrap();
        let story = &storyboard.stories[0];
        let act = &story.acts[0];
        let maneuver = &act.maneuver_groups[0].maneuvers[0];
        let event = &maneuver.events[0];
        
        assert!(event.start_trigger.is_some());
        let trigger = event.start_trigger.as_ref().unwrap();
        assert_eq!(trigger.condition_groups.len(), 1);
        assert_eq!(trigger.condition_groups[0].conditions.len(), 2); // Both time and speed conditions
    }

    #[test]
    fn test_scenario_with_default_trigger() {
        let mut scenario_builder = ScenarioBuilder::new()
            .with_header("Default Trigger Test", "Test Author")
            .with_entities();
            
        scenario_builder = scenario_builder.add_vehicle("ego", |v| v.car());
            
        let mut storyboard_builder = scenario_builder.with_storyboard();
        let mut story_builder = storyboard_builder.add_story_simple("default_story");
        let mut act_builder = story_builder.add_act("default_act");
        let mut maneuver_builder = act_builder.add_maneuver("default_maneuver", "ego");
        
        // Add action without explicit trigger (should get default trigger)
        maneuver_builder.add_speed_action()
            .named("immediate_action")
            .to_speed(20.0)
            .finish()
            .unwrap();
            
        let scenario = maneuver_builder
            .finish()
            .finish()
            .finish()
            .finish()
            .build()
            .unwrap();
            
        // Verify default trigger structure
        let storyboard = scenario.storyboard.unwrap();
        let story = &storyboard.stories[0];
        let act = &story.acts[0];
        let maneuver = &act.maneuver_groups[0].maneuvers[0];
        let event = &maneuver.events[0];
        
        assert!(event.start_trigger.is_some());
        let trigger = event.start_trigger.as_ref().unwrap();
        assert_eq!(trigger.condition_groups.len(), 1);
        
        // Should have a default time condition at t=0
        let condition = &trigger.condition_groups[0].conditions[0];
        assert!(condition.by_value_condition.is_some());
    }
}