#[cfg(feature = "builder")]
mod parameter_builder_tests {
    use openscenario_rs::builder::{
        ParameterDeclarationsBuilder, ParameterizedValueBuilder, ScenarioBuilder,
    };
    use openscenario_rs::types::basic::Value;
    use openscenario_rs::types::enums::ParameterType;

    use openscenario_rs::types::catalogs::locations::CatalogLocations;
    use openscenario_rs::types::road::RoadNetwork;
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
        assert_eq!(
            vehicle_param.parameter_type,
            Value::Literal(ParameterType::String)
        );
        assert_eq!(vehicle_param.value.to_string(), "ego");

        // Check double parameter
        let speed_param = &params.parameter_declarations[1];
        assert_eq!(speed_param.name.to_string(), "initial_speed");
        assert_eq!(
            speed_param.parameter_type,
            Value::Literal(ParameterType::Double)
        );
        assert_eq!(speed_param.value.to_string(), "25");

        // Check boolean parameter
        let logging_param = &params.parameter_declarations[3];
        assert_eq!(logging_param.name.to_string(), "enable_logging");
        assert_eq!(
            logging_param.parameter_type,
            Value::Literal(ParameterType::Boolean)
        );
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
    fn test_scenario_builder_with_parameters() {
        let params = ParameterDeclarationsBuilder::new()
            .add_string_parameter("ego_vehicle", "sedan")
            .add_double_parameter("target_speed", 30.0)
            .add_int_parameter("target_lane", 1)
            .build();

        let scenario = ScenarioBuilder::new()
            .with_header("Parameterized Test", "Test Author")
            // Required of a scenario document by the XSD, even when empty.
            .with_catalog_locations(CatalogLocations::default())
            .with_road_network(RoadNetwork::default())
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
        assert_eq!(
            ego_param.parameter_type,
            Value::Literal(ParameterType::String)
        );

        let speed_param = &param_decls.parameter_declarations[1];
        assert_eq!(speed_param.name.to_string(), "target_speed");
        assert_eq!(
            speed_param.parameter_type,
            Value::Literal(ParameterType::Double)
        );

        let lane_param = &param_decls.parameter_declarations[2];
        assert_eq!(lane_param.name.to_string(), "target_lane");
        assert_eq!(
            lane_param.parameter_type,
            Value::Literal(ParameterType::Int)
        );
    }

    #[test]
    fn test_parameter_utils() {
        use openscenario_rs::builder::parameters::utils;

        // Test parameter reference creation
        assert_eq!(utils::parameter_ref("speed"), "${speed}");
        assert_eq!(utils::parameter_ref("vehicle_name"), "${vehicle_name}");

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
}
