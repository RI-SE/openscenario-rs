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

    /// `CatalogEntityBuilder` resolves a `VehicleCatalogReference` against the vehicle catalog
    /// directory named in its `CatalogLocations` (XSD `VehicleCatalogLocation`, `Directory`):
    /// an absolute directory from `new()`, a relative one against `with_base_path`. Without
    /// locations it refuses rather than guessing a directory.
    #[test]
    fn catalog_entity_builder_resolves_vehicle_reference_from_its_locations() {
        use openscenario_rs::builder::CatalogEntityBuilder;

        let root = tempfile::TempDir::new().unwrap();
        let vehicles = root.path().join("Vehicles");
        std::fs::create_dir(&vehicles).unwrap();
        std::fs::write(
            vehicles.join("VehicleCatalog.xosc"),
            r#"<?xml version="1.0"?>
<OpenSCENARIO>
  <FileHeader author="t" date="2024-01-01T00:00:00" description="d" revMajor="1" revMinor="3"/>
  <Catalog name="VehicleCatalog">
    <Vehicle name="Sedan" vehicleCategory="car">
      <BoundingBox><Center x="1.4" y="0" z="0.9"/><Dimensions width="1.8" length="4.5" height="1.5"/></BoundingBox>
      <Performance maxSpeed="60" maxAcceleration="5" maxDeceleration="9"/>
      <Axles>
        <FrontAxle maxSteering="0.5" wheelDiameter="0.6" trackWidth="1.6" positionX="2.8" positionZ="0.3"/>
        <RearAxle maxSteering="0" wheelDiameter="0.6" trackWidth="1.6" positionX="0" positionZ="0.3"/>
      </Axles>
    </Vehicle>
  </Catalog>
</OpenSCENARIO>"#,
        )
        .unwrap();
        let reference = VehicleCatalogReferenceBuilder::new()
            .from_catalog("VehicleCatalog")
            .entry("Sedan")
            .build()
            .unwrap();

        let rows = [
            (
                "new + absolute directory",
                CatalogEntityBuilder::new().with_catalog_locations(
                    CatalogLocationsBuilder::new()
                        .with_vehicle_catalog(vehicles.to_str().unwrap())
                        .build(),
                ),
            ),
            (
                "with_base_path + relative directory",
                CatalogEntityBuilder::with_base_path(root.path()).with_catalog_locations(
                    CatalogLocationsBuilder::new()
                        .with_vehicle_catalog("Vehicles")
                        .build(),
                ),
            ),
        ];
        for (row, mut builder) in rows {
            let vehicle = builder
                .resolve_vehicle_reference(&reference)
                .unwrap_or_else(|e| panic!("{row}: {e}"));
            assert_eq!(vehicle.name.as_literal().unwrap(), "Sedan", "{row}");
            assert_eq!(
                vehicle.bounding_box.dimensions.length.as_literal(),
                Some(&4.5),
                "{row}"
            );
        }

        let error = CatalogEntityBuilder::new()
            .resolve_vehicle_reference(&reference)
            .expect_err("no locations set");
        assert!(
            error.to_string().contains("Catalog locations not set"),
            "{error}"
        );
    }
}
