#[cfg(feature = "builder")]
mod pedestrian_builder_tests {
    use openscenario_rs::types::enums::{PedestrianCategory, Role};
    use openscenario_rs::ScenarioBuilder;

    #[test]
    fn test_standard_pedestrian_builder() {
        let scenario = ScenarioBuilder::new()
            .with_header("Test", "Author")
            .with_entities()
            .add_pedestrian("ped1", |p| p.pedestrian().with_mass(75.0).finish())
            .build();

        assert!(scenario.is_ok());
        let scenario = scenario.unwrap();
        assert_eq!(scenario.entities.unwrap().scenario_objects.len(), 1);

        let obj = &scenario.entities.unwrap().scenario_objects[0];
        assert!(obj.pedestrian.is_some());

        let ped = obj.pedestrian.as_ref().unwrap();
        assert_eq!(ped.pedestrian_category, PedestrianCategory::Pedestrian);
        assert_eq!(ped.mass.as_literal().unwrap(), &75.0);
    }

    #[test]
    fn test_wheel_pedestrian_builder() {
        let scenario = ScenarioBuilder::new()
            .with_header("Test", "Author")
            .with_entities()
            .add_pedestrian("wheelchair1", |p| {
                p.wheelchair()
                    .with_mass(85.0)
                    .with_role(Role::Civil)
                    .finish()
            })
            .build();

        assert!(scenario.is_ok());
        let scenario = scenario.unwrap();
        let obj = &scenario.entities.unwrap().scenario_objects[0];
        let ped = obj.pedestrian.as_ref().unwrap();

        assert_eq!(ped.pedestrian_category, PedestrianCategory::Wheelchair);
        assert_eq!(ped.mass.as_literal().unwrap(), &85.0);
        assert_eq!(ped.role.unwrap(), Role::Civil);
    }

    #[test]
    fn test_animal_pedestrian_builder() {
        let scenario = ScenarioBuilder::new()
            .with_header("Test", "Author")
            .with_entities()
            .add_pedestrian("dog1", |p| {
                p.animal()
                    .with_mass(50.0)
                    .with_model3d("./models/dog.glb")
                    .finish()
            })
            .build();

        assert!(scenario.is_ok());
        let scenario = scenario.unwrap();
        let obj = &scenario.entities.unwrap().scenario_objects[0];
        let ped = obj.pedestrian.as_ref().unwrap();

        assert_eq!(ped.pedestrian_category, PedestrianCategory::Animal);
        assert_eq!(ped.mass.as_literal().unwrap(), &50.0);
        assert_eq!(ped.model3d.as_ref().unwrap(), "./models/dog.glb");
    }

    #[test]
    fn test_custom_dimensions() {
        let scenario = ScenarioBuilder::new()
            .with_header("Test", "Author")
            .with_entities()
            .add_pedestrian("tall_person", |p| {
                p.pedestrian()
                    .with_mass(90.0)
                    .with_dimensions(0.7, 0.7, 2.0)
                    .finish()
            })
            .build();

        assert!(scenario.is_ok());
        let scenario = scenario.unwrap();
        let obj = &scenario.entities.unwrap().scenario_objects[0];
        let ped = obj.pedestrian.as_ref().unwrap();

        assert_eq!(ped.mass.as_literal().unwrap(), &90.0);
        assert_eq!(
            ped.bounding_box.dimensions.width.as_literal().unwrap(),
            &0.7
        );
        assert_eq!(
            ped.bounding_box.dimensions.height.as_literal().unwrap(),
            &2.0
        );
    }

    #[test]
    fn test_multiple_pedestrians() {
        let scenario = ScenarioBuilder::new()
            .with_header("Test", "Author")
            .with_entities()
            .add_pedestrian("ped1", |p| p.pedestrian().with_mass(75.0).finish())
            .add_pedestrian("wheel1", |p| p.wheelchair().with_mass(85.0).finish())
            .add_pedestrian("dog1", |p| p.animal().with_mass(50.0).finish())
            .build();

        assert!(scenario.is_ok());
        let scenario = scenario.unwrap();
        assert_eq!(scenario.entities.unwrap().scenario_objects.len(), 3);
    }

    #[test]
    fn test_pedestrian_xsd_serialization() {
        use openscenario_rs::types::entities::Pedestrian;

        let pedestrian = Pedestrian {
            name: openscenario_rs::types::basic::Value::literal("test".to_string()),
            pedestrian_category: PedestrianCategory::Pedestrian,
            mass: openscenario_rs::types::basic::Double::literal(75.0),
            role: Some(Role::Civil),
            model3d: Some("./model.glb".to_string()),
            bounding_box: openscenario_rs::types::geometry::BoundingBox::default(),
            properties: None,
            parameter_declarations: None,
        };

        // Test serialization
        let xml = quick_xml::se::to_string(&pedestrian).unwrap();

        // Verify XSD-required fields are present
        assert!(xml.contains("name=\"test\""));
        assert!(xml.contains("mass=\"75\""));
        assert!(xml.contains("pedestrianCategory=\"pedestrian\""));
        assert!(xml.contains("role=\"civil\""));
        assert!(xml.contains("model3d=\"./model.glb\""));
        assert!(xml.contains("BoundingBox"));
    }
}
