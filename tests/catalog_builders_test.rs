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

    // Note: Integration tests with actual catalog resolution would require
    // mock catalog files and are better suited for integration test suites
}
