//! Integration tests for scenario initialization through the public builder API.
//!
//! The per-method `InitActionBuilder` contracts live in the builder's own unit tests
//! (`src/builder/init/`); these cover the default template header and the fluent
//! `create_private_action` chain end to end.

#[cfg(feature = "builder")]
mod tests {
    use openscenario_rs::builder::positions::WorldPositionBuilder;
    use openscenario_rs::builder::{BasicScenarioTemplate, InitActionBuilder, ScenarioTemplate};

    #[test]
    fn test_basic_scenario_template() {
        // Test that BasicScenarioTemplate provides working foundation
        let scenario_builder = BasicScenarioTemplate::create().with_storyboard(|storyboard| {
            // Minimal storyboard with default init
            storyboard
        });
        let scenario = scenario_builder.build().unwrap();

        // Should have header
        let header = &scenario.file_header;
        assert_eq!(header.description.as_literal().unwrap(), "Basic Scenario");
        assert_eq!(header.author.as_literal().unwrap(), "openscenario-rs");

        // Should have entities
        assert!(scenario.entities.is_some());
    }

    // `test_convenience_methods` covered `InitActionBuilder::for_single_vehicle` and
    // `for_multiple_vehicles`, both removed. Each opened a `<Private>` container for a
    // named entity and put no action in it, and the test asserted that count as the
    // expected result. XSD `Private` requires at least one `PrivateAction`, so what the
    // test pinned was the defect.

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
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(init.actions.private_actions.len(), 1);
        let ego_private = &init.actions.private_actions[0];
        assert_eq!(ego_private.entity_ref.as_literal().unwrap(), "ego");
        assert_eq!(ego_private.private_actions.len(), 2);

        // Should have teleport and speed actions
        assert_eq!(
            ego_private.private_actions[0].action_type(),
            "TeleportAction"
        );
        assert_eq!(
            ego_private.private_actions[1].action_type(),
            "LongitudinalAction"
        );
    }
}
