#[cfg(feature = "builder")]
mod builder_tests {
    use openscenario_rs::types::basic::Value;
    use openscenario_rs::types::enums::ParameterType;
    use openscenario_rs::ScenarioBuilder;

    #[test]
    fn test_minimal_scenario_creation() {
        let scenario = ScenarioBuilder::new()
            .with_header("Test", "Author")
            .with_entities()
            .with_storyboard(|storyboard| {
                // Minimal storyboard with default init
                storyboard
            })
            .build()
            .unwrap();

        assert!(scenario.entities.is_some());
        assert!(scenario.storyboard.is_some());
    }

    #[test]
    fn test_parameter_support() {
        let scenario = ScenarioBuilder::new()
            .with_header("Test", "Author")
            .add_parameter("initial_speed", ParameterType::Double, "25.0")
            .add_parameter("target_lane", ParameterType::Int, "1")
            .with_entities()
            .with_storyboard(|storyboard| {
                // Minimal storyboard with default init
                storyboard
            })
            .build()
            .unwrap();

        assert!(scenario.parameter_declarations.is_some());
        let params = scenario.parameter_declarations.unwrap();
        assert_eq!(params.parameter_declarations.len(), 2);

        // Check first parameter
        let speed_param = &params.parameter_declarations[0];
        if let Value::Literal(name) = &speed_param.name {
            assert_eq!(name, "initial_speed");
        } else {
            panic!("Parameter name should be literal");
        }
        assert_eq!(speed_param.parameter_type, ParameterType::Double);
    }
}
