#[cfg(feature = "builder")]
mod vehicle_builder_tests {
    use openscenario_rs::ScenarioBuilder;
    use openscenario_rs::types::basic::Value;

    #[test]
    fn test_vehicle_creation() {
        let mut scenario_builder = ScenarioBuilder::new()
            .with_header("Test", "Author")
            .with_entities();
            
        scenario_builder
            .add_vehicle("ego")
            .car()
            .finish();
            
        let scenario = scenario_builder.build().unwrap();
            
        let entities = scenario.entities.unwrap();
        assert_eq!(entities.scenario_objects.len(), 1);
        
        let ego = entities.find_object("ego").unwrap();
        assert!(ego.vehicle.is_some());
        
        let vehicle = ego.vehicle.as_ref().unwrap();
        if let Value::Literal(name) = &vehicle.name {
            assert_eq!(name, "PassengerCar");
        } else {
            panic!("Vehicle name should be literal");
        }
    }

    #[test]
    fn test_multiple_vehicles() {
        let mut scenario_builder = ScenarioBuilder::new()
            .with_header("Multi Vehicle Test", "Test Author")
            .with_entities();
            
        scenario_builder
            .add_vehicle("ego")
            .car()
            .finish()
            .add_vehicle("truck1")
            .truck()
            .finish();
            
        let scenario = scenario_builder.build().unwrap();
            
        let entities = scenario.entities.unwrap();
        assert_eq!(entities.scenario_objects.len(), 2);
        
        let ego = entities.find_object("ego").unwrap();
        assert!(ego.vehicle.is_some());
        
        let truck = entities.find_object("truck1").unwrap();
        assert!(truck.vehicle.is_some());
    }

    #[test]
    fn test_vehicle_with_custom_dimensions() {
        let mut scenario_builder = ScenarioBuilder::new()
            .with_header("Custom Vehicle Test", "Test Author")
            .with_entities();
            
        scenario_builder
            .add_vehicle("custom")
            .car()
            .with_dimensions(5.0, 2.0, 1.8)
            .with_performance(150.0, 5.0, 12.0)
            .finish();
            
        let scenario = scenario_builder.build().unwrap();
            
        let entities = scenario.entities.unwrap();
        assert_eq!(entities.scenario_objects.len(), 1);
        
        let custom = entities.find_object("custom").unwrap();
        assert!(custom.vehicle.is_some());
        
        let vehicle = custom.vehicle.as_ref().unwrap();
        
        // Check dimensions
        if let Value::Literal(length) = &vehicle.bounding_box.dimensions.length {
            assert_eq!(*length, 5.0);
        } else {
            panic!("Length should be literal");
        }
        
        if let Value::Literal(width) = &vehicle.bounding_box.dimensions.width {
            assert_eq!(*width, 2.0);
        } else {
            panic!("Width should be literal");
        }
        
        // Check performance
        if let Value::Literal(max_speed) = &vehicle.performance.max_speed {
            assert_eq!(*max_speed, 150.0);
        } else {
            panic!("Max speed should be literal");
        }
    }
}