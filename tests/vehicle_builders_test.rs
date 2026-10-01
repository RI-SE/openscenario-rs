#[cfg(feature = "builder")]
mod vehicle_builder_tests {
    use openscenario_rs::types::basic::Value;
    use openscenario_rs::types::catalogs::locations::CatalogLocations;
    use openscenario_rs::types::enums::VehicleCategory;
    use openscenario_rs::types::road::RoadNetwork;
    use openscenario_rs::ScenarioBuilder;

    /// `add_vehicle` files each configured vehicle under the scenario-object name it was
    /// given; the presets themselves are covered by the builder's unit tests.
    #[test]
    fn test_multiple_vehicles() {
        let scenario = ScenarioBuilder::new()
            .with_header("Multi Vehicle Test", "Test Author")
            // Required of a scenario document by the XSD, even when empty.
            .with_catalog_locations(CatalogLocations::default())
            .with_road_network(RoadNetwork::default())
            .with_entities()
            .add_vehicle("ego", |v| v.car())
            .add_vehicle("truck1", |v| v.truck())
            .with_storyboard(|storyboard| storyboard)
            .build()
            .unwrap();

        let entities = scenario.entities.unwrap();
        assert_eq!(entities.scenario_objects.len(), 2);
        for (name, category) in [
            ("ego", VehicleCategory::Car),
            ("truck1", VehicleCategory::Truck),
        ] {
            let vehicle = entities
                .find_object(name)
                .and_then(|o| o.vehicle())
                .unwrap_or_else(|| panic!("no vehicle named {name}"));
            assert_eq!(vehicle.vehicle_category, Value::Literal(category), "{name}");
        }
    }

    /// `add_vehicle_mut` hands back an attached builder: its setters reach the vehicle and
    /// `finish` files it in the scenario under the given name.
    #[test]
    fn add_vehicle_mut_attaches_the_configured_vehicle() {
        let mut builder = ScenarioBuilder::new()
            .with_header("Attached Vehicle Test", "Test Author")
            .with_catalog_locations(CatalogLocations::default())
            .with_road_network(RoadNetwork::default())
            .with_entities();
        builder
            .add_vehicle_mut("lorry")
            .truck()
            .with_dimensions(12.0, 2.5, 3.8)
            .with_performance(90.0, 2.0, 6.0)
            .finish();
        let scenario = builder.with_storyboard(|s| s).build().unwrap();

        let vehicle = scenario
            .entities
            .unwrap()
            .find_object("lorry")
            .and_then(|o| o.vehicle())
            .cloned()
            .expect("finish() must add the vehicle to the scenario");
        assert_eq!(
            vehicle.vehicle_category,
            Value::Literal(VehicleCategory::Truck)
        );
        assert_eq!(
            vehicle.bounding_box.dimensions.length.as_literal(),
            Some(&12.0)
        );
        assert_eq!(vehicle.performance.max_speed.as_literal(), Some(&90.0));
    }
}
