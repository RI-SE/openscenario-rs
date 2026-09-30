#[cfg(feature = "builder")]
mod pedestrian_builder_tests {
    use openscenario_rs::types::basic::Value;
    use openscenario_rs::types::catalogs::locations::CatalogLocations;
    use openscenario_rs::types::entities::Pedestrian;
    use openscenario_rs::types::enums::{PedestrianCategory, Role};
    use openscenario_rs::types::road::RoadNetwork;
    use openscenario_rs::ScenarioBuilder;

    /// `add_pedestrian` files each configured pedestrian under the scenario-object name
    /// it was given; the presets themselves are covered by the builder's unit tests.
    #[test]
    fn test_multiple_pedestrians() {
        let scenario = ScenarioBuilder::new()
            .with_header("Test", "Author")
            // Required of a scenario document by the XSD, even when empty.
            .with_catalog_locations(CatalogLocations::default())
            .with_road_network(RoadNetwork::default())
            .with_entities()
            .add_pedestrian("ped1", |p| p.pedestrian().with_mass(75.0).finish())
            .add_pedestrian("wheel1", |p| p.wheelchair().with_mass(85.0).finish())
            .add_pedestrian("dog1", |p| p.animal().with_mass(50.0).finish())
            .with_storyboard(|storyboard| storyboard)
            .build()
            .unwrap();

        let entities = scenario.entities.unwrap();
        assert_eq!(entities.scenario_objects.len(), 3);
        for (name, category, mass) in [
            ("ped1", PedestrianCategory::Pedestrian, 75.0),
            ("wheel1", PedestrianCategory::Wheelchair, 85.0),
            ("dog1", PedestrianCategory::Animal, 50.0),
        ] {
            let ped = entities
                .find_object(name)
                .and_then(|o| o.pedestrian())
                .unwrap_or_else(|| panic!("no pedestrian named {name}"));
            assert_eq!(ped.pedestrian_category, Value::Literal(category), "{name}");
            assert_eq!(ped.mass.as_literal(), Some(&mass), "{name}");
        }
    }

    /// XSD `Pedestrian` (`:1677-1692`): required `@mass`, `@name`, `@pedestrianCategory`,
    /// optional `@model3d` and `@role`, and a required `BoundingBox`.
    #[test]
    fn test_pedestrian_xsd_serialization() {
        let xml = concat!(
            r#"<Pedestrian name="test" pedestrianCategory="pedestrian" mass="75" role="civil" model3d="./model.glb">"#,
            r#"<BoundingBox><Center x="0" y="0" z="0"/><Dimensions width="2" length="4.5" height="1.5"/></BoundingBox>"#,
            r#"</Pedestrian>"#
        );
        let pedestrian = Pedestrian {
            name: Value::literal("test".to_string()),
            pedestrian_category: Value::Literal(PedestrianCategory::Pedestrian),
            mass: openscenario_rs::types::basic::Double::literal(75.0),
            role: Some(Value::Literal(Role::Civil)),
            model: None,
            model3d: Some("./model.glb".to_string()),
            bounding_box: openscenario_rs::types::geometry::BoundingBox::new(
                openscenario_rs::types::geometry::Center::new(0.0, 0.0, 0.0),
                openscenario_rs::types::geometry::Dimensions::new(2.0, 4.5, 1.5),
            ),
            properties: None,
            parameter_declarations: None,
        };

        assert_eq!(quick_xml::se::to_string(&pedestrian).unwrap(), xml);
        assert_eq!(
            quick_xml::de::from_str::<Pedestrian>(xml).unwrap(),
            pedestrian
        );
    }
}
