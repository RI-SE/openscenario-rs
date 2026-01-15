#[cfg(feature = "builder")]
mod catalog_builder_tests {
    use openscenario_rs::builder::{
        CatalogEntityBuilder, CatalogLocationsBuilder, PedestrianCatalogReferenceBuilder,
        ScenarioBuilder, VehicleCatalogReferenceBuilder,
    };

    #[test]
    fn test_catalog_locations_builder() {
        let locations = CatalogLocationsBuilder::new()
            .with_vehicle_catalog("./catalogs/vehicles")
            .with_pedestrian_catalog("./catalogs/pedestrians")
            .with_controller_catalog("./catalogs/controllers")
            .build();

        assert!(locations.vehicle_catalog.is_some());
        assert!(locations.pedestrian_catalog.is_some());
        assert!(locations.controller_catalog.is_some());

        let vehicle_catalog = locations.vehicle_catalog.unwrap();
        assert_eq!(
            vehicle_catalog.directory.path.to_string(),
            "./catalogs/vehicles"
        );
    }

    #[test]
    fn test_vehicle_catalog_reference_builder() {
        let reference = VehicleCatalogReferenceBuilder::new()
            .from_catalog("vehicle_catalog")
            .entry("sedan")
            .with_parameter("color", "red")
            .with_parameter("engine_power", "150")
            .build()
            .unwrap();

        assert_eq!(reference.catalog_name.to_string(), "vehicle_catalog");
        assert_eq!(reference.entry_name.to_string(), "sedan");
        assert!(reference.parameter_assignments.is_some());

        let params = reference.parameter_assignments.unwrap();
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn test_pedestrian_catalog_reference_builder() {
        let reference = PedestrianCatalogReferenceBuilder::new()
            .from_catalog("pedestrian_catalog")
            .entry("adult_male")
            .with_parameter("height", "1.8")
            .build()
            .unwrap();

        assert_eq!(reference.catalog_name.to_string(), "pedestrian_catalog");
        assert_eq!(reference.entry_name.to_string(), "adult_male");
        assert!(reference.parameter_assignments.is_some());
    }

    #[test]
    fn test_catalog_entity_builder_creation() {
        let _builder = CatalogEntityBuilder::new();
        let _builder_with_path = CatalogEntityBuilder::with_base_path("/tmp");

        let locations = CatalogLocationsBuilder::new()
            .with_vehicle_catalog("./catalogs/vehicles")
            .build();

        let _builder_with_locations = CatalogEntityBuilder::new().with_catalog_locations(locations);
    }

    #[test]
    fn test_scenario_builder_with_catalog_locations() {
        let locations = CatalogLocationsBuilder::new()
            .with_vehicle_catalog("./catalogs/vehicles")
            .with_pedestrian_catalog("./catalogs/pedestrians")
            .build();

        let scenario = ScenarioBuilder::new()
            .with_header("Catalog Test", "Test Author")
            .with_catalog_locations(locations)
            .with_entities()
            .build()
            .unwrap();

        assert!(scenario.catalog_locations.is_some());
        let catalog_locations = scenario.catalog_locations.unwrap();
        assert!(catalog_locations.vehicle_catalog.is_some());
        assert!(catalog_locations.pedestrian_catalog.is_some());
    }

    #[test]
    fn test_catalog_pedestrian_with_all_fields() {
        use openscenario_rs::types::basic::Value;
        use openscenario_rs::types::catalogs::entities::CatalogPedestrian;

        let catalog_pedestrian = CatalogPedestrian {
            name: "TestPedestrian".to_string(),
            pedestrian_category: Value::Literal("pedestrian".to_string()),
            mass: Value::Literal("75.0".to_string()),
            role: Some(Value::Literal("civil".to_string())),
            model3d: Some("./models/ped.glb".to_string()),
            bounding_box: openscenario_rs::types::geometry::BoundingBox::default(),
            properties: None,
            parameter_declarations: None,
        };

        assert_eq!(catalog_pedestrian.name, "TestPedestrian");
        assert_eq!(catalog_pedestrian.mass.as_literal().unwrap(), "75.0");
        assert!(catalog_pedestrian.role.is_some());
        assert!(catalog_pedestrian.model3d.is_some());
    }

    #[test]
    fn test_catalog_pedestrian_resolution_with_parameters() {
        use openscenario_rs::types::basic::Value;
        use openscenario_rs::types::catalogs::entities::{CatalogEntity, CatalogPedestrian};
        use std::collections::HashMap;

        let catalog_pedestrian = CatalogPedestrian {
            name: "ParamPedestrian".to_string(),
            pedestrian_category: Value::Literal("pedestrian".to_string()),
            mass: Value::Literal("75.0".to_string()),
            role: Some(Value::Literal("civil".to_string())),
            model3d: Some("./models/ped.glb".to_string()),
            bounding_box: openscenario_rs::types::geometry::BoundingBox::default(),
            properties: None,
            parameter_declarations: None,
        };

        let resolved = catalog_pedestrian
            .into_scenario_entity(HashMap::new())
            .unwrap();

        assert_eq!(resolved.name.as_literal().unwrap(), "ParamPedestrian");
        assert_eq!(resolved.mass.as_literal().unwrap(), &75.0);
        assert_eq!(
            resolved.role.unwrap(),
            openscenario_rs::types::enums::Role::Civil
        );
        assert_eq!(resolved.model3d, Some("./models/ped.glb".to_string()));
    }

    #[test]
    fn test_catalog_pedestrian_mass_parameter_resolution() {
        use openscenario_rs::types::basic::Value;
        use openscenario_rs::types::catalogs::entities::{CatalogEntity, CatalogPedestrian};
        use std::collections::HashMap;

        let catalog_pedestrian = CatalogPedestrian {
            name: "MassParamPedestrian".to_string(),
            pedestrian_category: Value::Literal("pedestrian".to_string()),
            mass: Value::Literal("80.0".to_string()), // Parameterized mass
            role: Some(Value::Literal("police".to_string())),
            model3d: None,
            bounding_box: openscenario_rs::types::geometry::BoundingBox::default(),
            properties: None,
            parameter_declarations: None,
        };

        let resolved = catalog_pedestrian
            .into_scenario_entity(HashMap::new())
            .unwrap();

        assert_eq!(resolved.mass.as_literal().unwrap(), &80.0);
        assert_eq!(
            resolved.role.unwrap(),
            openscenario_rs::types::enums::Role::Police
        );
    }

    // Note: Integration tests with actual catalog resolution would require
    // mock catalog files and are better suited for integration test suites
}
