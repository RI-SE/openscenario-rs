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
        let value_of = |name: &str| {
            params
                .assignments
                .iter()
                .find(|a| a.parameter_ref.to_string() == name)
                .unwrap_or_else(|| panic!("no assignment for parameterRef {name}"))
                .value
                .to_string()
        };
        assert_eq!(value_of("color"), "red");
        assert_eq!(value_of("engine_power"), "150");
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

        let params = reference.parameter_assignments.unwrap();
        assert_eq!(params.assignments.len(), 1);
        assert_eq!(params.assignments[0].parameter_ref.to_string(), "height");
        assert_eq!(params.assignments[0].value.to_string(), "1.8");
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

    /// `ScenarioBuilder::add_catalog_vehicle` / `add_catalog_pedestrian`
    /// (`src/builder/entities/catalog.rs`) are the public entry points onto
    /// `CatalogVehicleBuilder`/`CatalogPedestrianBuilder`: they resolve a catalog reference
    /// through `CatalogEntityBuilder` and bake the result straight into the scenario's
    /// `Entities` — the XSD's `CatalogReference` choice (`EntityObject` group,
    /// `Schema/OpenSCENARIO.xsd:1949-1957`) never survives to the built document, only the
    /// `Vehicle`/`Pedestrian` it named does. A `ParameterAssignment` on the reference overrides
    /// the entry's own default (ASAM OpenSCENARIO XML 1.3 section 9.5: "the ParameterAssignment
    /// element within CatalogReference may be used to override these defaults"), so asserting
    /// the resolved `mass` instead of the catalog entry's constant default proves the assignment
    /// actually reached the entry rather than being accepted and dropped.
    #[test]
    fn add_catalog_vehicle_and_add_catalog_pedestrian_resolve_with_parameter_assignments() {
        use openscenario_rs::builder::CatalogEntityBuilder;
        use openscenario_rs::types::entities::EntityObjectChoice;
        use openscenario_rs::types::road::RoadNetwork;

        let root = tempfile::TempDir::new().unwrap();
        let vehicles_dir = root.path().join("Vehicles");
        let pedestrians_dir = root.path().join("Pedestrians");
        std::fs::create_dir(&vehicles_dir).unwrap();
        std::fs::create_dir(&pedestrians_dir).unwrap();
        std::fs::write(
            vehicles_dir.join("VehicleCatalog.xosc"),
            r#"<?xml version="1.0"?>
<OpenSCENARIO>
  <FileHeader author="t" date="2024-01-01T00:00:00" description="d" revMajor="1" revMinor="3"/>
  <Catalog name="VehicleCatalog">
    <Vehicle name="Sedan" vehicleCategory="car" mass="$mass">
      <ParameterDeclarations>
        <ParameterDeclaration name="mass" parameterType="double" value="1500"/>
      </ParameterDeclarations>
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
        std::fs::write(
            pedestrians_dir.join("PedestrianCatalog.xosc"),
            r#"<?xml version="1.0"?>
<OpenSCENARIO>
  <FileHeader author="t" date="2024-01-01T00:00:00" description="d" revMajor="1" revMinor="3"/>
  <Catalog name="PedestrianCatalog">
    <Pedestrian name="Walker" pedestrianCategory="pedestrian" mass="$mass">
      <ParameterDeclarations>
        <ParameterDeclaration name="mass" parameterType="double" value="80"/>
      </ParameterDeclarations>
      <BoundingBox><Center x="0" y="0" z="0.9"/><Dimensions width="0.5" length="0.5" height="1.8"/></BoundingBox>
    </Pedestrian>
  </Catalog>
</OpenSCENARIO>"#,
        )
        .unwrap();

        let locations = CatalogLocationsBuilder::new()
            .with_vehicle_catalog(vehicles_dir.to_str().unwrap())
            .with_pedestrian_catalog(pedestrians_dir.to_str().unwrap())
            .build();

        let mut builder = ScenarioBuilder::new()
            .with_header("Catalog Entity Builders Test", "Test Author")
            .with_catalog_locations(locations.clone())
            .with_road_network(RoadNetwork::default())
            .with_entities();

        builder
            .add_catalog_vehicle("Ego")
            .with_catalog_builder(
                CatalogEntityBuilder::new().with_catalog_locations(locations.clone()),
            )
            .from_catalog("VehicleCatalog", "Sedan")
            .with_parameter("mass", "2200")
            .finish()
            .expect("vehicle resolves");

        builder
            .add_catalog_pedestrian("Pedestrian1")
            .with_catalog_builder(CatalogEntityBuilder::new().with_catalog_locations(locations))
            .from_catalog("PedestrianCatalog", "Walker")
            .with_parameter("mass", "95")
            .finish()
            .expect("pedestrian resolves");

        let scenario = builder
            .with_storyboard(|s| s)
            .build()
            .expect("scenario builds");

        let entities = scenario.entities.expect("entities present");
        assert_eq!(entities.scenario_objects.len(), 2, "both entities added");

        let ego = &entities.scenario_objects[0];
        assert_eq!(ego.name.as_literal().unwrap(), "Ego");
        match &ego.entity {
            EntityObjectChoice::Vehicle(vehicle) => {
                assert_eq!(vehicle.name.as_literal().unwrap(), "Sedan");
                assert_eq!(
                    vehicle.mass.as_ref().and_then(|m| m.as_literal()).copied(),
                    Some(2200.0),
                    "with_parameter must override the catalog entry's default mass"
                );
            }
            other => panic!("Ego is not a resolved Vehicle: {other:?}"),
        }

        let pedestrian_object = &entities.scenario_objects[1];
        assert_eq!(pedestrian_object.name.as_literal().unwrap(), "Pedestrian1");
        match &pedestrian_object.entity {
            EntityObjectChoice::Pedestrian(pedestrian) => {
                assert_eq!(pedestrian.name.as_literal().unwrap(), "Walker");
                assert_eq!(
                    pedestrian.mass.as_literal().copied(),
                    Some(95.0),
                    "with_parameter must override the catalog entry's default mass"
                );
            }
            other => panic!("Pedestrian1 is not a resolved Pedestrian: {other:?}"),
        }
    }
}
