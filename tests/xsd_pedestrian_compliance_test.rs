#[cfg(feature = "builder")]
mod xsd_compliance_tests {
    use openscenario_rs::types::enums::{PedestrianCategory, Role};
    use openscenario_rs::ScenarioBuilder;

    #[test]
    fn test_xsd_required_fields() {
        let scenario = ScenarioBuilder::new()
            .with_header("Test", "Author")
            .with_entities()
            .add_pedestrian("ped1", |p| p.pedestrian().with_mass(75.0).finish())
            .build()
            .unwrap();

        let xml = quick_xml::se::to_string(&scenario).unwrap();

        // Verify ALL XSD-required fields are present
        assert!(xml.contains("name=\"ped1\""));
        assert!(xml.contains("mass=\"75\""));
        assert!(xml.contains("pedestrianCategory=\"pedestrian\""));
        assert!(xml.contains("BoundingBox"));
    }

    #[test]
    fn test_xsd_optional_fields() {
        let scenario = ScenarioBuilder::new()
            .with_header("Test", "Author")
            .with_entities()
            .add_pedestrian("ped1", |p| {
                p.pedestrian()
                    .with_mass(75.0)
                    .with_role(Role::Police)
                    .with_model3d("./models/police.glb")
                    .finish()
            })
            .build()
            .unwrap();

        let xml = quick_xml::se::to_string(&scenario).unwrap();

        // Verify optional fields are serialized when present
        assert!(xml.contains("role=\"police\""));
        assert!(xml.contains("model3d=\"./models/police.glb\""));
    }
}
