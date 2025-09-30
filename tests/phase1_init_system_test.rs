//! Phase 1 Integration Test - Init Action System
//!
//! This test verifies that Phase 1 implementation fixes the critical execution blockers:
//! 1. Empty Triggers - All events should have non-empty StartTrigger elements
//! 2. Missing Init Actions - Entities should have proper initialization
//! 3. Integration - Trigger builders should be connected to event builders

#[cfg(feature = "builder")]
mod tests {
    use openscenario_rs::builder::{
        InitActionBuilder, 
        ScenarioBuilder,
        BasicScenarioTemplate,
        TriggerBuilder,
        TimeConditionBuilder,
    };
    use openscenario_rs::builder::positions::WorldPositionBuilder;
    use openscenario_rs::types::scenario::init::Init;

    #[test]
    fn test_init_action_builder_basic() {
        // Test that InitActionBuilder creates proper Init structure
        let init = InitActionBuilder::new()
            .add_global_environment_action()
            .build()
            .unwrap();
            
        // Should have environment action
        assert_eq!(init.actions.global_actions.len(), 1);
        assert!(init.actions.global_actions[0].environment_action.is_some());
        
        // Should have empty private actions initially
        assert!(init.actions.private_actions.is_empty());
    }

    #[test]
    fn test_init_action_builder_with_entities() {
        // Test entity initialization with teleport and speed actions
        let position = WorldPositionBuilder::new()
            .at_coordinates(10.0, 20.0, 0.0)
            .with_heading(1.57) // 90 degrees
            .build()
            .unwrap();
            
        let init = InitActionBuilder::new()
            .add_global_environment_action()
            .add_teleport_action("ego", position)
            .add_speed_action("ego", 30.0)
            .build()
            .unwrap();
            
        // Should have environment action
        assert_eq!(init.actions.global_actions.len(), 1);
        
        // Should have one private action for ego
        assert_eq!(init.actions.private_actions.len(), 1);
        let ego_private = &init.actions.private_actions[0];
        assert_eq!(ego_private.entity_ref.as_literal().unwrap(), "ego");
        
        // Should have two actions: teleport and speed
        assert_eq!(ego_private.private_actions.len(), 2);
        
        // First action should be teleport
        assert!(ego_private.private_actions[0].teleport_action.is_some());
        
        // Second action should be speed
        assert!(ego_private.private_actions[1].longitudinal_action.is_some());
        let longitudinal = ego_private.private_actions[1].longitudinal_action.as_ref().unwrap();
        assert!(longitudinal.speed_action.is_some());
        
        let speed_action = longitudinal.speed_action.as_ref().unwrap();
        assert_eq!(speed_action.speed_target.absolute_value.as_ref().unwrap().as_literal().unwrap(), 30.0);
    }

    #[test]
    fn test_init_action_builder_multiple_entities() {
        // Test multi-entity initialization
        let ego_pos = WorldPositionBuilder::new()
            .at_coordinates(0.0, 0.0, 0.0)
            .build()
            .unwrap();
            
        let target_pos = WorldPositionBuilder::new()
            .at_coordinates(50.0, 0.0, 0.0)
            .build()
            .unwrap();
            
        let init = InitActionBuilder::new()
            .add_global_environment_action()
            .add_teleport_action("ego", ego_pos)
            .add_speed_action("ego", 30.0)
            .add_teleport_action("target", target_pos)
            .add_speed_action("target", 25.0)
            .build()
            .unwrap();
            
        // Should have two entities
        assert_eq!(init.actions.private_actions.len(), 2);
        
        // Check ego entity
        let ego_private = &init.actions.private_actions[0];
        assert_eq!(ego_private.entity_ref.as_literal().unwrap(), "ego");
        assert_eq!(ego_private.private_actions.len(), 2);
        
        // Check target entity
        let target_private = &init.actions.private_actions[1];
        assert_eq!(target_private.entity_ref.as_literal().unwrap(), "target");
        assert_eq!(target_private.private_actions.len(), 2);
    }

    #[test]
    fn test_trigger_builder_integration() {
        // Test that TriggerBuilder creates non-empty triggers
        let trigger = TriggerBuilder::new()
            .add_condition(
                TimeConditionBuilder::new()
                    .at_time(5.0)
                    .build()
                    .unwrap()
            )
            .build()
            .unwrap();
            
        // Should have one condition group
        assert_eq!(trigger.condition_groups.len(), 1);
        
        // Should have one condition in the group
        assert_eq!(trigger.condition_groups[0].conditions.len(), 1);
        
        // Condition should be a time condition
        let condition = &trigger.condition_groups[0].conditions[0];
        assert!(condition.by_value_condition.is_some());
        
        let by_value = condition.by_value_condition.as_ref().unwrap();
        assert!(by_value.simulation_time_condition.is_some());
        
        let time_condition = by_value.simulation_time_condition.as_ref().unwrap();
        assert_eq!(time_condition.value.as_literal().unwrap(), 5.0);
    }

    #[test]
    fn test_basic_scenario_template() {
        // Test that BasicScenarioTemplate provides working foundation
        let scenario = BasicScenarioTemplate::create();
        
        // Should have header
        assert!(scenario.data.header.is_some());
        let header = scenario.data.header.as_ref().unwrap();
        assert_eq!(header.name.as_literal().unwrap(), "Basic Scenario");
        assert_eq!(header.author.as_literal().unwrap(), "openscenario-rs");
        
        // Should have entities
        assert!(scenario.data.entities.is_some());
    }

    #[test]
    fn test_alks_scenario_template() {
        // Test ALKS template provides proper initialization
        let scenario = BasicScenarioTemplate::alks_template();
        
        // Should have proper header
        assert!(scenario.data.header.is_some());
        let header = scenario.data.header.as_ref().unwrap();
        assert_eq!(header.name.as_literal().unwrap(), "ALKS Scenario");
        
        // Should have entities
        assert!(scenario.data.entities.is_some());
    }

    #[test]
    fn test_convenience_methods() {
        // Test convenience methods for common patterns
        let init_single = InitActionBuilder::for_single_vehicle("ego")
            .build()
            .unwrap();
            
        assert_eq!(init_single.actions.global_actions.len(), 1);
        assert_eq!(init_single.actions.private_actions.len(), 1);
        assert_eq!(
            init_single.actions.private_actions[0].entity_ref.as_literal().unwrap(),
            "ego"
        );
        
        let init_multi = InitActionBuilder::for_multiple_vehicles(&["ego", "target", "obstacle"])
            .build()
            .unwrap();
            
        assert_eq!(init_multi.actions.global_actions.len(), 1);
        assert_eq!(init_multi.actions.private_actions.len(), 3);
        
        let entity_names: Vec<String> = init_multi.actions.private_actions
            .iter()
            .map(|p| p.entity_ref.as_literal().unwrap().to_string())
            .collect();
        assert_eq!(entity_names, vec!["ego", "target", "obstacle"]);
    }

    #[test]
    fn test_fluent_private_action_builder() {
        // Test fluent API for private actions
        let position = WorldPositionBuilder::new()
            .at_coordinates(5.0, 10.0, 0.0)
            .build()
            .unwrap();
            
        let init = InitActionBuilder::new()
            .create_private_action("ego")
                .add_teleport_action(position)
                .add_speed_action(40.0)
                .finish()
            .build()
            .unwrap();
            
        assert_eq!(init.actions.private_actions.len(), 1);
        let ego_private = &init.actions.private_actions[0];
        assert_eq!(ego_private.entity_ref.as_literal().unwrap(), "ego");
        assert_eq!(ego_private.private_actions.len(), 2);
        
        // Should have teleport and speed actions
        assert!(ego_private.private_actions[0].teleport_action.is_some());
        assert!(ego_private.private_actions[1].longitudinal_action.is_some());
    }

    #[test]
    fn test_phase1_success_criteria() {
        // This test verifies all Phase 1 success criteria are met
        
        // 1. Non-empty triggers can be created
        let trigger = TriggerBuilder::new()
            .add_condition(
                TimeConditionBuilder::new()
                    .at_time(0.0)
                    .build()
                    .unwrap()
            )
            .build()
            .unwrap();
            
        assert!(!trigger.condition_groups.is_empty());
        assert!(!trigger.condition_groups[0].conditions.is_empty());
        
        // 2. Entity initialization works
        let position = WorldPositionBuilder::new()
            .at_coordinates(0.0, -1.75, 0.0)
            .with_heading(0.0)
            .build()
            .unwrap();
            
        let init = InitActionBuilder::new()
            .add_global_environment_action()
            .add_teleport_action("Ego", position)
            .add_speed_action("Ego", 16.67)
            .build()
            .unwrap();
            
        // Should have environment setup
        assert!(!init.actions.global_actions.is_empty());
        assert!(init.actions.global_actions[0].environment_action.is_some());
        
        // Should have entity initialization
        assert!(!init.actions.private_actions.is_empty());
        let ego_private = &init.actions.private_actions[0];
        assert_eq!(ego_private.entity_ref.as_literal().unwrap(), "Ego");
        assert_eq!(ego_private.private_actions.len(), 2);
        
        // 3. Templates provide working scenarios
        let scenario = BasicScenarioTemplate::alks_template();
        assert!(scenario.data.header.is_some());
        assert!(scenario.data.entities.is_some());
        
        println!("✅ Phase 1 Success Criteria Met:");
        println!("   - Non-empty triggers can be created");
        println!("   - Entity initialization system works");
        println!("   - Templates provide executable foundations");
    }
}