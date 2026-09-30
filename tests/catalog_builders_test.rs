#[cfg(feature = "builder")]
mod catalog_builder_tests {
    use openscenario_rs::builder::{
        CatalogLocationsBuilder, PedestrianCatalogReferenceBuilder, ScenarioBuilder,
        VehicleCatalogReferenceBuilder,
    };
    use openscenario_rs::types::basic::Directory;

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
        assert_eq!(params.assignments.len(), 2);
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
    fn all_eight_catalog_location_kinds_can_be_set() {
        let empty = CatalogLocationsBuilder::new().build();
        assert!(!empty.has_catalogs(), "nothing set must build no location");

        let locations = CatalogLocationsBuilder::new()
            .with_vehicle_catalog("./catalogs/vehicles")
            .with_pedestrian_catalog("./catalogs/pedestrians")
            .with_controller_catalog("./catalogs/controllers")
            .with_misc_object_catalog("./catalogs/misc")
            .with_environment_catalog("./catalogs/environments")
            .with_maneuver_catalog("./catalogs/maneuvers")
            .with_trajectory_catalog("./catalogs/trajectories")
            .with_route_catalog("./catalogs/routes")
            .build();

        assert_eq!(locations.catalog_count(), 8);
        let path = |directory: Option<&Directory>| {
            directory
                .expect("location set")
                .path
                .as_literal()
                .expect("literal path")
                .clone()
        };
        let set = [
            (
                path(locations.vehicle_catalog.as_ref().map(|l| &l.directory)),
                "vehicles",
            ),
            (
                path(locations.pedestrian_catalog.as_ref().map(|l| &l.directory)),
                "pedestrians",
            ),
            (
                path(locations.controller_catalog.as_ref().map(|l| &l.directory)),
                "controllers",
            ),
            (
                path(locations.misc_object_catalog.as_ref().map(|l| &l.directory)),
                "misc",
            ),
            (
                path(locations.environment_catalog.as_ref().map(|l| &l.directory)),
                "environments",
            ),
            (
                path(locations.maneuver_catalog.as_ref().map(|l| &l.directory)),
                "maneuvers",
            ),
            (
                path(locations.trajectory_catalog.as_ref().map(|l| &l.directory)),
                "trajectories",
            ),
            (
                path(locations.route_catalog.as_ref().map(|l| &l.directory)),
                "routes",
            ),
        ];
        for (actual, kind) in set {
            assert_eq!(actual, format!("./catalogs/{kind}"), "{kind} location");
        }
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
            // Required of a scenario document by the XSD even when it names no road file.
            .with_road_network(openscenario_rs::types::road::RoadNetwork::default())
            .with_entities()
            .with_storyboard(|storyboard| {
                // Minimal storyboard with default init
                storyboard
            })
            .build()
            .unwrap();

        assert!(scenario.catalog_locations.is_some());
        let catalog_locations = scenario.catalog_locations.unwrap();
        assert!(catalog_locations.vehicle_catalog.is_some());
        assert!(catalog_locations.pedestrian_catalog.is_some());
    }
}
