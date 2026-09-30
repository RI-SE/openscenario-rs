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
        match action {
            openscenario_rs::types::actions::wrappers::PrivateAction::LongitudinalAction(long_action) => {
                match long_action.longitudinal_action_choice {
                    openscenario_rs::types::actions::movement::LongitudinalActionChoice::SpeedAction(speed_action) => {
                        let abs_target = speed_action
                            .speed_action_target
                            .target
                            .as_absolute()
                            .expect("Expected AbsoluteTargetSpeed branch");
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

        match action {
            openscenario_rs::types::actions::wrappers::PrivateAction::TeleportAction(
                teleport_action,
            ) => {
                let world = teleport_action
                    .position
                    .world_position()
                    .expect("expected the WorldPosition branch");
                assert_eq!(world.x.as_literal(), Some(&100.0));
                assert_eq!(world.y.as_literal(), Some(&200.0));
            }
            _ => panic!("Expected TeleportAction"),
        }
    }
}
