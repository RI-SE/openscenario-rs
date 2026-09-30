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
}
