#[cfg(feature = "builder")]
mod parameter_builder_tests {
    use openscenario_rs::builder::{
        ParameterContext, ParameterDeclarationsBuilder, ParameterizedValueBuilder, ScenarioBuilder,
    };
    use openscenario_rs::types::enums::ParameterType;

    #[test]
    fn test_parameter_declarations_builder() {
        let params = ParameterDeclarationsBuilder::new()
            .add_string_parameter("vehicle_name", "ego")
            .add_double_parameter("initial_speed", 25.0)
            .add_int_parameter("lane_id", 1)
            .add_boolean_parameter("enable_logging", true)
            .add_datetime_parameter("start_time", "2023-01-01T00:00:00")
            .add_unsigned_short_parameter("num_vehicles", 5)
            .add_unsigned_int_parameter("scenario_id", 12345)
            .build();

        assert_eq!(params.parameter_declarations.len(), 7);

        // Check string parameter
        let vehicle_param = &params.parameter_declarations[0];
        assert_eq!(vehicle_param.name.to_string(), "vehicle_name");
        assert_eq!(vehicle_param.parameter_type, ParameterType::String);
        assert_eq!(vehicle_param.value.to_string(), "ego");

        // Check double parameter
        let speed_param = &params.parameter_declarations[1];
        assert_eq!(speed_param.name.to_string(), "initial_speed");
        assert_eq!(speed_param.parameter_type, ParameterType::Double);
        assert_eq!(speed_param.value.to_string(), "25");

        // Check boolean parameter
        let logging_param = &params.parameter_declarations[3];
        assert_eq!(logging_param.name.to_string(), "enable_logging");
        assert_eq!(logging_param.parameter_type, ParameterType::Boolean);
        assert_eq!(logging_param.value.to_string(), "true");
    }

    #[test]
    fn test_parameterized_value_builder() {
        // Test literal value
        let literal_value = ParameterizedValueBuilder::literal(42.0).build();
        match literal_value {
            openscenario_rs::types::basic::Value::Literal(val) => assert_eq!(val, 42.0),
            _ => panic!("Expected literal value"),
        }

        // Test parameter reference
        let param_value = ParameterizedValueBuilder::<f64>::parameter("speed").build();
        match param_value {
            openscenario_rs::types::basic::Value::Parameter(name) => assert_eq!(name, "speed"),
            _ => panic!("Expected parameter reference"),
        }

        // Test expression
        let expr_value = ParameterizedValueBuilder::<f64>::expression("$speed * 2").build();
        match expr_value {
            openscenario_rs::types::basic::Value::Expression(expr) => {
                assert_eq!(expr, "$speed * 2")
            }
            _ => panic!("Expected expression"),
        }
    }

    #[test]
    fn test_parameter_context() {
        let context = ParameterContext::new()
            .add_parameter("speed", "30.0")
            .add_parameter("vehicle", "sedan")
            .add_parameter("lane", "1");

        assert_eq!(context.get_parameter("speed"), Some("30.0"));
        assert_eq!(context.get_parameter("vehicle"), Some("sedan"));
        assert_eq!(context.get_parameter("lane"), Some("1"));
        assert_eq!(context.get_parameter("unknown"), None);

        // Test parameter resolution
        let resolved_speed = context.resolve_parameter("speed").unwrap();
        assert_eq!(resolved_speed, "30.0");

        let resolved_vehicle = context.resolve_parameter("vehicle").unwrap();
        assert_eq!(resolved_vehicle, "sedan");

        // Test error for unknown parameter
        let result = context.resolve_parameter("unknown");
        assert!(result.is_err());
    }

    #[test]
    fn test_parameter_context_with_map() {
        let mut params = std::collections::HashMap::new();
        params.insert("speed".to_string(), "25.0".to_string());
        params.insert("vehicle".to_string(), "truck".to_string());

        let context = ParameterContext::new()
            .with_parameters(params)
            .add_parameter("lane", "2");

        assert_eq!(context.get_parameter("speed"), Some("25.0"));
        assert_eq!(context.get_parameter("vehicle"), Some("truck"));
        assert_eq!(context.get_parameter("lane"), Some("2"));
        assert_eq!(context.parameters().len(), 3);
    }

    #[test]
    fn test_scenario_builder_with_parameters() {
        let params = ParameterDeclarationsBuilder::new()
            .add_string_parameter("ego_vehicle", "sedan")
            .add_double_parameter("target_speed", 30.0)
            .add_int_parameter("target_lane", 1)
            .build();

        let scenario = ScenarioBuilder::new()
            .with_header("Parameterized Test", "Test Author")
            .with_parameters(params)
            .with_entities()
            .with_storyboard(|storyboard| {
                // Minimal storyboard with default init
                storyboard
            })
            .build()
            .unwrap();

        assert!(scenario.parameter_declarations.is_some());
        let param_decls = scenario.parameter_declarations.unwrap();
        assert_eq!(param_decls.parameter_declarations.len(), 3);

        // Verify parameter names and types
        let ego_param = &param_decls.parameter_declarations[0];
        assert_eq!(ego_param.name.to_string(), "ego_vehicle");
        assert_eq!(ego_param.parameter_type, ParameterType::String);

        let speed_param = &param_decls.parameter_declarations[1];
        assert_eq!(speed_param.name.to_string(), "target_speed");
        assert_eq!(speed_param.parameter_type, ParameterType::Double);

        let lane_param = &param_decls.parameter_declarations[2];
        assert_eq!(lane_param.name.to_string(), "target_lane");
        assert_eq!(lane_param.parameter_type, ParameterType::Int);
    }

    #[test]
    fn test_parameter_utils() {
        use openscenario_rs::builder::parameters::utils;

        // Test parameter reference creation
        assert_eq!(utils::parameter_ref("speed"), "${speed}");
        assert_eq!(utils::parameter_ref("vehicle_name"), "${vehicle_name}");

        // Test parameter reference detection
        assert!(utils::is_parameter_ref("${speed}"));
        assert!(utils::is_parameter_ref("${vehicle_name}"));
        assert!(!utils::is_parameter_ref("speed"));
        assert!(!utils::is_parameter_ref("literal_value"));

        // Test parameter name extraction
        assert_eq!(utils::extract_parameter_name("${speed}"), Some("speed"));
        assert_eq!(
            utils::extract_parameter_name("${vehicle_name}"),
            Some("vehicle_name")
        );
        assert_eq!(utils::extract_parameter_name("speed"), None);
        assert_eq!(utils::extract_parameter_name("literal_value"), None);

        // Test parameterized value creation
        let param_string = utils::parameterized_string("vehicle_name");
        match param_string {
            openscenario_rs::types::basic::Value::Parameter(name) => {
                assert_eq!(name, "vehicle_name")
            }
            _ => panic!("Expected parameter reference"),
        }

        let param_double = utils::parameterized_double("speed");
        match param_double {
            openscenario_rs::types::basic::Value::Parameter(name) => assert_eq!(name, "speed"),
            _ => panic!("Expected parameter reference"),
        }
    }

    #[test]
    fn test_empty_parameter_declarations_builder() {
        let params = ParameterDeclarationsBuilder::new().build();
        assert_eq!(params.parameter_declarations.len(), 0);

        let builder = ParameterDeclarationsBuilder::new();
        assert_eq!(builder.len(), 0);
        assert!(builder.is_empty());
    }

    #[test]
    fn test_parameter_context_value_resolution() {
        let context = ParameterContext::new()
            .add_parameter("speed", "30.0")
            .add_parameter("vehicle", "sedan");

        // Test literal value resolution
        let literal_value = openscenario_rs::types::basic::Value::Literal(42.0);
        let resolved = context.resolve_value(&literal_value).unwrap();
        assert_eq!(resolved, "42");

        // Test parameter value resolution
        let param_value: openscenario_rs::types::basic::Value<String> =
            openscenario_rs::types::basic::Value::Parameter("speed".to_string());
        let resolved = context.resolve_value(&param_value).unwrap();
        assert_eq!(resolved, "30.0");

        // Test expression value resolution (returns as-is for now)
        let expr_value: openscenario_rs::types::basic::Value<String> =
            openscenario_rs::types::basic::Value::Expression("$speed * 2".to_string());
        let resolved = context.resolve_value(&expr_value).unwrap();
        assert_eq!(resolved, "$speed * 2");
    }
}
