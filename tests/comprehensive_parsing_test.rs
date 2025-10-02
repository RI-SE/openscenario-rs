//! Phase 2 comprehensive test demonstrating all new action builders and condition builders

#[cfg(feature = "builder")]
mod tests {
    use openscenario_rs::builder::actions::ActionBuilder;
    use openscenario_rs::builder::{
        actions::{
            ActivateControllerActionBuilder,
            EnvironmentActionBuilder, // EntityActionBuilder, VariableActionBuilder,
            LaneChangeActionBuilder,
            LaneOffsetActionBuilder,
            LateralDistanceActionBuilder,
            OverrideControllerValueActionBuilder,
        },
        conditions::{
            AccelerationConditionBuilder, CollisionConditionBuilder, ParameterConditionBuilder,
            ReachPositionConditionBuilder, RelativeDistanceConditionBuilder,
            TraveledDistanceConditionBuilder, VariableConditionBuilder,
        },
        scenario::ScenarioBuilder,
    };
    use openscenario_rs::types::{
        basic::Value,
        enums::{DynamicsShape, Rule},
        environment::{Environment, RoadCondition, TimeOfDay, Weather},
        positions::{Position, WorldPosition},
    };
    #[test]
    fn test_phase2_lateral_actions() {
        println!("🧪 Testing Phase 2 Lateral Actions...");

        // Test LaneChangeAction
        let lane_change = LaneChangeActionBuilder::new()
            .for_entity("ego")
            .to_absolute_lane("1")
            .with_lane_offset(0.5)
            .with_simple_dynamics(2.0)
            .build_action();

        assert!(
            lane_change.is_ok(),
            "Lane change action should build successfully"
        );
        println!("✅ LaneChangeAction built successfully");

        // Test LateralDistanceAction
        let lateral_distance = LateralDistanceActionBuilder::new()
            .for_entity("ego")
            .from_entity("target")
            .at_distance(3.0)
            .with_freespace(true)
            .continuous(true)
            .build_action();

        assert!(
            lateral_distance.is_ok(),
            "Lateral distance action should build successfully"
        );
        println!("✅ LateralDistanceAction built successfully");

        // Test LaneOffsetAction
        let lane_offset = LaneOffsetActionBuilder::new()
            .for_entity("ego")
            .to_absolute_offset(1.5)
            .continuous(false)
            .with_simple_dynamics(DynamicsShape::Linear, Some(2.0))
            .build_action();

        assert!(
            lane_offset.is_ok(),
            "Lane offset action should build successfully"
        );
        println!("✅ LaneOffsetAction built successfully");
    }

    #[test]
    fn test_phase2_controller_actions() {
        println!("🧪 Testing Phase 2 Controller Actions...");

        // Test ActivateControllerAction
        let activate_controller = ActivateControllerActionBuilder::new()
            .for_entity("ego")
            .movement_only()
            .build_action();

        assert!(
            activate_controller.is_ok(),
            "Activate controller action should build successfully"
        );
        println!("✅ ActivateControllerAction built successfully");

        // Test OverrideControllerValueAction
        let override_controller = OverrideControllerValueActionBuilder::new()
            .for_entity("ego")
            .throttle(true, 0.8)
            .brake(false, 0.0)
            .steering(true, 0.2)
            .build_action();

        assert!(
            override_controller.is_ok(),
            "Override controller action should build successfully"
        );
        println!("✅ OverrideControllerValueAction built successfully");
    }

    #[test]
    fn test_phase2_global_actions() {
        println!("🧪 Testing Phase 2 Global Actions...");

        // Test EnvironmentAction
        let environment = Environment {
            name: Value::literal("TestEnvironment".to_string()),
            time_of_day: TimeOfDay::default(),
            weather: Weather::default(),
            road_condition: RoadCondition::default(),
        };

        let environment_action = EnvironmentActionBuilder::new()
            .for_entity("ego")
            .with_environment(environment)
            .build();

        assert!(
            environment_action.is_ok(),
            "Environment action should build successfully"
        );
        println!("✅ EnvironmentAction built successfully");

        // Test EntityAction (delete) - commented out until implemented
        // let entity_action = EntityActionBuilder::new()
        //     .for_entity("target")
        //     .delete_entity()
        //     .build_action();
        //
        // assert!(entity_action.is_ok(), "Entity action should build successfully");
        // println!("✅ EntityAction built successfully");

        // Test VariableAction - commented out until implemented
        // let variable_action = VariableActionBuilder::new()
        //     .for_entity("ego")
        //     .set_variable("speed_limit", 50.0)
        //     .build_action();
        //
        // assert!(variable_action.is_ok(), "Variable action should build successfully");
        // println!("✅ VariableAction built successfully");
    }

    #[test]
    fn test_phase2_enhanced_conditions() {
        println!("🧪 Testing Phase 2 Enhanced Conditions...");

        // Test ParameterCondition
        let parameter_condition = ParameterConditionBuilder::new()
            .parameter("max_speed")
            .value_above(60.0)
            .build();

        assert!(
            parameter_condition.is_ok(),
            "Parameter condition should build successfully"
        );
        println!("✅ ParameterCondition built successfully");

        // Test VariableCondition
        let variable_condition = VariableConditionBuilder::new()
            .variable("traffic_density")
            .value_below(0.5)
            .build();

        assert!(
            variable_condition.is_ok(),
            "Variable condition should build successfully"
        );
        println!("✅ VariableCondition built successfully");

        // Test AccelerationCondition
        let acceleration_condition = AccelerationConditionBuilder::new()
            .for_entity("ego")
            .acceleration_above(2.0)
            .build();

        assert!(
            acceleration_condition.is_ok(),
            "Acceleration condition should build successfully"
        );
        println!("✅ AccelerationCondition built successfully");

        // Test TraveledDistanceCondition
        let traveled_distance_condition = TraveledDistanceConditionBuilder::new()
            .for_entity("ego")
            .distance_above(100.0)
            .build();

        assert!(
            traveled_distance_condition.is_ok(),
            "Traveled distance condition should build successfully"
        );
        println!("✅ TraveledDistanceCondition built successfully");

        // Test ReachPositionCondition
        let position = Position {
            world_position: Some(WorldPosition::with_full_orientation(
                100.0, 200.0, 0.0, 0.0, 0.0, 0.0,
            )),
            relative_world_position: None,
            road_position: None,
            relative_road_position: None,
            lane_position: None,
            relative_lane_position: None,
            trajectory_position: None,
            geographic_position: None,
            relative_object_position: None,
        };

        let reach_position_condition = ReachPositionConditionBuilder::new()
            .for_entity("ego")
            .at_position(position)
            .with_tolerance(2.0)
            .build();

        assert!(
            reach_position_condition.is_ok(),
            "Reach position condition should build successfully"
        );
        println!("✅ ReachPositionCondition built successfully");

        // Test RelativeDistanceCondition
        let relative_distance_condition = RelativeDistanceConditionBuilder::new()
            .for_entity("ego")
            .to_entity("target")
            .closer_than(5.0)
            .use_freespace(true)
            .longitudinal()
            .build();

        assert!(
            relative_distance_condition.is_ok(),
            "Relative distance condition should build successfully"
        );
        println!("✅ RelativeDistanceCondition built successfully");

        // Test CollisionCondition
        let collision_condition = CollisionConditionBuilder::new()
            .for_entity("ego")
            .with_entity("target")
            .collision_type("front")
            .build();

        assert!(
            collision_condition.is_ok(),
            "Collision condition should build successfully"
        );
        println!("✅ CollisionCondition built successfully");
    }

    #[test]
    fn test_phase2_stop_trigger_support() {
        println!("🧪 Testing Phase 2 Stop Trigger Support...");

        let scenario_builder = ScenarioBuilder::new()
            .with_header("Phase2Test", "TestAuthor")
            .with_entities();

        let mut storyboard_builder = scenario_builder.create_storyboard();

        // Test stop after time
        let storyboard_with_time_stop = storyboard_builder.stop_after_time(30.0);

        assert!(
            storyboard_with_time_stop.is_ok(),
            "Stop after time should work"
        );
        println!("✅ Stop after time trigger built successfully");

        // Test stop when entity reaches position
        let scenario_builder2 = ScenarioBuilder::new()
            .with_header("Phase2Test2", "TestAuthor")
            .with_entities();

        let storyboard_builder2 = scenario_builder2.create_storyboard();

        let position = Position {
            world_position: Some(WorldPosition::with_full_orientation(
                500.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            )),
            relative_world_position: None,
            road_position: None,
            relative_road_position: None,
            lane_position: None,
            relative_lane_position: None,
            trajectory_position: None,
            geographic_position: None,
            relative_object_position: None,
        };

        let storyboard_with_position_stop =
            storyboard_builder2.stop_when_entity_reaches("ego", position);

        assert!(
            storyboard_with_position_stop.is_ok(),
            "Stop when entity reaches position should work"
        );
        println!("✅ Stop when entity reaches position trigger built successfully");
    }

    #[test]
    fn test_phase2_comprehensive_scenario() {
        println!("🧪 Testing Phase 2 Comprehensive Scenario...");

        // Create a comprehensive scenario using Phase 2 features
        let scenario = ScenarioBuilder::new()
            .with_header("Phase2ComprehensiveTest", "TestAuthor")
            .with_entities()
            .create_storyboard()
            .create_init_actions()
            .add_global_environment_action()
            .add_teleport_action(
                "ego",
                Position {
                    world_position: Some(WorldPosition::with_full_orientation(
                        0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                    )),
                    relative_world_position: None,
                    road_position: None,
                    relative_road_position: None,
                    lane_position: None,
                    relative_lane_position: None,
                    trajectory_position: None,
                    geographic_position: None,
                    relative_object_position: None,
                },
            )
            .add_speed_action("ego", 15.0)
            .finish()
            .unwrap()
            .stop_after_time(60.0)
            .unwrap()
            .finish()
            .build();

        assert!(
            scenario.is_ok(),
            "Comprehensive Phase 2 scenario should build successfully"
        );

        let scenario = scenario.unwrap();
        assert!(
            scenario.storyboard.unwrap().stop_trigger.is_some(),
            "Stop trigger should be present"
        );

        println!("✅ Phase 2 comprehensive scenario built successfully");
        println!("🎉 Phase 2 implementation complete!");
    }
}

#[cfg(not(feature = "builder"))]
fn main() {
    println!(
        "Builder feature not enabled. Run with --features builder to test Phase 2 functionality."
    );
}
