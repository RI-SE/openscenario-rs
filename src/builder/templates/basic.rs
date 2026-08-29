//! Basic scenario template for common patterns

use super::ScenarioTemplate;
use crate::builder::{
    init::InitActionBuilder,
    positions::WorldPositionBuilder,
    scenario::{Complete, HasEntities, ScenarioBuilder},
};
use crate::types::catalogs::locations::CatalogLocations;
use crate::types::road::RoadNetwork;

/// Basic scenario template providing common initialization patterns
pub struct BasicScenarioTemplate;

impl ScenarioTemplate for BasicScenarioTemplate {
    fn create() -> ScenarioBuilder<HasEntities> {
        Self::create_with_header("Basic Scenario", "openscenario-rs")
    }

    /// Both `CatalogLocations` and `RoadNetwork` are required of a scenario document by the XSD
    /// (`ScenarioDefinition`, `Schema/OpenSCENARIO.xsd:1989`), so they are set here rather than
    /// left for the caller to remember. Every child of each is `minOccurs="0"`, so the empty
    /// form is schema-valid and states nothing: a template that invented a catalog directory or
    /// a road file would be putting words in the caller's mouth. Override either with
    /// `.with_catalog_locations()` or `.with_road_file()` on the returned builder.
    fn create_with_header(name: &str, author: &str) -> ScenarioBuilder<HasEntities> {
        ScenarioBuilder::new()
            .with_header(name, author)
            .with_catalog_locations(CatalogLocations::default())
            .with_road_network(RoadNetwork::default())
            .with_entities()
    }
}

impl BasicScenarioTemplate {
    /// Create a single vehicle scenario with basic initialization
    pub fn single_vehicle(
        vehicle_name: &str,
    ) -> crate::builder::scenario::ScenarioBuilder<crate::builder::scenario::Complete> {
        let position = WorldPositionBuilder::new()
            .at_coordinates(0.0, 0.0, 0.0)
            .build()
            .unwrap();

        let init = InitActionBuilder::new()
            .add_global_environment_action()
            .add_teleport_action(vehicle_name, position)
            .add_speed_action(vehicle_name, 30.0)
            .build()
            .unwrap();

        Self::create().with_storyboard(|storyboard| storyboard.with_init_actions(init))
    }

    /// Create a two-vehicle scenario (ego + target)
    pub fn two_vehicle_scenario() -> ScenarioBuilder<Complete> {
        let ego_position = WorldPositionBuilder::new()
            .at_coordinates(0.0, 0.0, 0.0)
            .build()
            .unwrap();

        let target_position = WorldPositionBuilder::new()
            .at_coordinates(50.0, 0.0, 0.0)
            .build()
            .unwrap();

        let init = InitActionBuilder::new()
            .add_global_environment_action()
            .add_teleport_action("ego", ego_position)
            .add_speed_action("ego", 30.0)
            .add_teleport_action("target", target_position)
            .add_speed_action("target", 25.0)
            .build()
            .unwrap();

        Self::create().with_storyboard(|storyboard| storyboard.with_init_actions(init))
    }

    /// Create an ALKS-style scenario template
    pub fn alks_template() -> ScenarioBuilder<Complete> {
        let ego_position = WorldPositionBuilder::new()
            .at_coordinates(0.0, -1.75, 0.0)
            .with_heading(0.0)
            .build()
            .unwrap();

        let target_position = WorldPositionBuilder::new()
            .at_coordinates(100.0, -1.75, 0.0)
            .with_heading(0.0)
            .build()
            .unwrap();

        let init = InitActionBuilder::new()
            .add_global_environment_action()
            .add_teleport_action("Ego", ego_position)
            .add_speed_action("Ego", 16.67) // 60 km/h
            .add_teleport_action("TargetVehicle", target_position)
            .add_speed_action("TargetVehicle", 13.89) // 50 km/h
            .build()
            .unwrap();

        Self::create_with_header("ALKS Scenario", "openscenario-rs")
            .with_storyboard(|storyboard| storyboard.with_init_actions(init))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_template() {
        let scenario = BasicScenarioTemplate::create();
        assert!(scenario.data.file_header.is_some());
    }

    #[test]
    fn test_single_vehicle_template() {
        let scenario = BasicScenarioTemplate::single_vehicle("ego");
        assert!(scenario.data.file_header.is_some());
        assert!(scenario.data.entities.is_some());
        assert!(scenario.data.storyboard.is_some());
    }

    #[test]
    fn test_two_vehicle_template() {
        let scenario = BasicScenarioTemplate::two_vehicle_scenario();
        assert!(scenario.data.file_header.is_some());
        assert!(scenario.data.entities.is_some());
        assert!(scenario.data.storyboard.is_some());
    }

    #[test]
    fn test_alks_template() {
        let scenario = BasicScenarioTemplate::alks_template();
        assert!(scenario.data.file_header.is_some());
        assert!(scenario.data.entities.is_some());
        assert!(scenario.data.storyboard.is_some());

        // Check header description
        let header = scenario.data.file_header.as_ref().unwrap();
        assert_eq!(header.description.as_literal().unwrap(), "ALKS Scenario");
    }
}
