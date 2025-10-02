#[cfg(feature = "builder")]
mod action_builder_tests {
    use openscenario_rs::builder::actions::ActionBuilder;
    use openscenario_rs::builder::actions::{SpeedActionBuilder, TeleportActionBuilder};

    #[test]
    fn test_speed_action_builder() {
        let action = SpeedActionBuilder::new()
            .for_entity("ego")
            .to_speed(30.0)
            .build_action()
            .unwrap();

        // Verify the action was built correctly
        match action.action {
            openscenario_rs::types::actions::wrappers::CorePrivateAction::LongitudinalAction(long_action) => {
                match long_action.longitudinal_action_choice {
                    openscenario_rs::types::actions::movement::LongitudinalActionChoice::SpeedAction(speed_action) => {
                        assert!(speed_action.speed_action_target.absolute.is_some());
                        let abs_target = speed_action.speed_action_target.absolute.unwrap();
                        assert_eq!(abs_target.value.as_literal(), Some(&30.0));
                    }
                    _ => panic!("Expected SpeedAction"),
                }
            }
            _ => panic!("Expected LongitudinalAction"),
        }
    }

    #[test]
    fn test_teleport_action_builder() {
        let action = TeleportActionBuilder::new()
            .for_entity("ego")
            .to()
            .world_position(100.0, 200.0, 0.0)
            .build_action()
            .unwrap();

        match action.action {
            openscenario_rs::types::actions::wrappers::CorePrivateAction::TeleportAction(
                teleport_action,
            ) => {
                // Verify position was set correctly
                assert!(teleport_action.position.world_position.is_some());
            }
            _ => panic!("Expected TeleportAction"),
        }
    }
}
