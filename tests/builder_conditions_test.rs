#[cfg(feature = "builder")]
mod condition_builder_tests {
    use openscenario_rs::builder::conditions::{TimeConditionBuilder, SpeedConditionBuilder, TriggerBuilder, DistanceConditionBuilder};
    use openscenario_rs::types::{
        positions::{Position, WorldPosition},
        basic::{Double, Value},
        enums::Rule,
    };

    #[test]
    fn test_time_condition_builder() {
        let condition = TimeConditionBuilder::new()
            .at_time(5.0)
            .build()
            .unwrap();
            
        assert!(condition.by_value_condition.is_some());
        let by_value = condition.by_value_condition.unwrap();
        assert!(by_value.simulation_time_condition.is_some());
        
        let time_condition = by_value.simulation_time_condition.unwrap();
        assert_eq!(time_condition.value.as_literal().unwrap(), &5.0);
        assert_eq!(time_condition.rule, Rule::GreaterThan);
    }

    #[test]
    fn test_time_condition_with_custom_rule() {
        let condition = TimeConditionBuilder::new()
            .time_rule(10.0, Rule::LessThan)
            .build()
            .unwrap();
            
        let by_value = condition.by_value_condition.unwrap();
        let time_condition = by_value.simulation_time_condition.unwrap();
        assert_eq!(time_condition.value.as_literal().unwrap(), &10.0);
        assert_eq!(time_condition.rule, Rule::LessThan);
    }

    #[test]
    fn test_speed_condition_builder() {
        let condition = SpeedConditionBuilder::new()
            .for_entity("ego")
            .speed_above(30.0)
            .build()
            .unwrap();
            
        assert!(condition.by_entity_condition.is_some());
        let by_entity = condition.by_entity_condition.unwrap();
        
        // Check triggering entities
        assert_eq!(by_entity.triggering_entities.entity_refs.len(), 1);
        assert_eq!(
            by_entity.triggering_entities.entity_refs[0].entity_ref.as_literal().unwrap(),
            "ego"
        );
        
        // Check entity condition
        match by_entity.entity_condition {
            openscenario_rs::types::conditions::entity::EntityCondition::Speed(speed_condition) => {
                assert_eq!(speed_condition.value.as_literal().unwrap(), &30.0);
                assert_eq!(speed_condition.rule, Rule::GreaterThan);
                assert_eq!(speed_condition.entity_ref, "ego");
            }
            _ => panic!("Expected Speed condition"),
        }
    }

    #[test]
    fn test_speed_condition_below() {
        let condition = SpeedConditionBuilder::new()
            .for_entity("target")
            .speed_below(15.0)
            .build()
            .unwrap();
            
        let by_entity = condition.by_entity_condition.unwrap();
        match by_entity.entity_condition {
            openscenario_rs::types::conditions::entity::EntityCondition::Speed(speed_condition) => {
                assert_eq!(speed_condition.value.as_literal().unwrap(), &15.0);
                assert_eq!(speed_condition.rule, Rule::LessThan);
            }
            _ => panic!("Expected Speed condition"),
        }
    }

    #[test]
    fn test_distance_condition_builder() {
        let position = Position {
            world_position: Some(WorldPosition {
                x: Double::literal(100.0),
                y: Double::literal(200.0),
                z: Some(Double::literal(0.0)),
                h: Some(Double::literal(0.0)),
                p: Some(Double::literal(0.0)),
                r: Some(Double::literal(0.0)),
            }),
            relative_world_position: None,
            road_position: None,
            relative_road_position: None,
            lane_position: None,
            relative_lane_position: None,
            trajectory_position: None,
            geographic_position: None,
            relative_object_position: None,
        };
        
        let condition = DistanceConditionBuilder::new()
            .for_entity("ego")
            .to_position(position)
            .closer_than(10.0)
            .build()
            .unwrap();
            
        assert!(condition.by_entity_condition.is_some());
        let by_entity = condition.by_entity_condition.unwrap();
        
        match by_entity.entity_condition {
            openscenario_rs::types::conditions::entity::EntityCondition::Distance(distance_condition) => {
                assert_eq!(distance_condition.value.as_literal().unwrap(), &10.0);
                assert_eq!(distance_condition.rule, Rule::LessThan);
                assert_eq!(distance_condition.freespace.as_literal().unwrap(), &false);
            }
            _ => panic!("Expected Distance condition"),
        }
    }

    #[test]
    fn test_trigger_builder() {
        let time_condition = TimeConditionBuilder::new()
            .at_time(3.0)
            .build()
            .unwrap();
            
        let trigger = TriggerBuilder::new()
            .add_condition(time_condition)
            .build()
            .unwrap();
            
        assert_eq!(trigger.condition_groups.len(), 1);
        assert_eq!(trigger.condition_groups[0].conditions.len(), 1);
    }

    #[test]
    fn test_trigger_builder_multiple_conditions() {
        let time_condition = TimeConditionBuilder::new()
            .at_time(5.0)
            .build()
            .unwrap();

        let speed_condition = SpeedConditionBuilder::new()
            .for_entity("ego")
            .speed_above(30.0)
            .build()
            .unwrap();
            
        let trigger = TriggerBuilder::new()
            .add_condition_group()
                .add_condition(time_condition)
                .add_condition(speed_condition)
                .finish_group()
            .build()
            .unwrap();
            
        assert_eq!(trigger.condition_groups.len(), 1);
        assert_eq!(trigger.condition_groups[0].conditions.len(), 2);
    }

    #[test]
    fn test_trigger_builder_multiple_groups() {
        let time_condition1 = TimeConditionBuilder::new()
            .at_time(3.0)
            .build()
            .unwrap();

        let time_condition2 = TimeConditionBuilder::new()
            .at_time(10.0)
            .build()
            .unwrap();
            
        let trigger = TriggerBuilder::new()
            .add_condition(time_condition1)
            .add_condition(time_condition2)
            .build()
            .unwrap();
            
        // Each add_condition creates its own group
        assert_eq!(trigger.condition_groups.len(), 2);
        assert_eq!(trigger.condition_groups[0].conditions.len(), 1);
        assert_eq!(trigger.condition_groups[1].conditions.len(), 1);
    }

    #[test]
    fn test_condition_validation_errors() {
        // Time condition without time value
        let result = TimeConditionBuilder::new().build();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Time value is required"));

        // Speed condition without entity reference
        let result = SpeedConditionBuilder::new()
            .speed_above(30.0)
            .build();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Entity reference is required"));

        // Speed condition without speed value
        let result = SpeedConditionBuilder::new()
            .for_entity("ego")
            .build();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Speed value is required"));

        // Distance condition without entity reference
        let position = Position {
            world_position: Some(WorldPosition {
                x: Double::literal(0.0),
                y: Double::literal(0.0),
                z: Some(Double::literal(0.0)),
                h: Some(Double::literal(0.0)),
                p: Some(Double::literal(0.0)),
                r: Some(Double::literal(0.0)),
            }),
            relative_world_position: None,
            road_position: None,
            relative_road_position: None,
            lane_position: None,
            relative_lane_position: None,
            trajectory_position: None,
            geographic_position: None,
            relative_object_position: None,
        };
        
        let result = DistanceConditionBuilder::new()
            .to_position(position)
            .closer_than(10.0)
            .build();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Entity reference is required"));

        // Trigger without conditions
        let result = TriggerBuilder::new().build();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("At least one condition is required"));
    }

    #[test]
    fn test_condition_group_builder_fluent_api() {
        let trigger = TriggerBuilder::new()
            .add_condition_group()
                .time_condition()
                    .at_time(5.0)
                    .finish()
                    .unwrap()
                .speed_condition()
                    .for_entity("ego")
                    .speed_above(25.0)
                    .finish()
                    .unwrap()
                .finish_group()
            .build()
            .unwrap();
            
        assert_eq!(trigger.condition_groups.len(), 1);
        assert_eq!(trigger.condition_groups[0].conditions.len(), 2);
    }

    #[test]
    fn test_condition_delay_attribute() {
        // Test that all condition builders include the required delay attribute
        let time_condition = TimeConditionBuilder::new()
            .at_time(5.0)
            .build()
            .unwrap();
            
        // Verify delay attribute is present with default value of 0.0
        assert!(time_condition.delay.is_some());
        assert_eq!(time_condition.delay.unwrap().as_literal().unwrap(), &0.0);
        
        let speed_condition = SpeedConditionBuilder::new()
            .for_entity("ego")
            .speed_above(30.0)
            .build()
            .unwrap();
            
        // Verify delay attribute is present with default value of 0.0
        assert!(speed_condition.delay.is_some());
        assert_eq!(speed_condition.delay.unwrap().as_literal().unwrap(), &0.0);
    }
}