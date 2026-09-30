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
            .add_global_environment_action("Environment")
            .add_teleport_action(vehicle_name, position)
            .add_speed_action(vehicle_name, 30.0)
            .build()
            .unwrap();

        // The init actions below reference `vehicle_name`, so the entity has to be declared.
        Self::create()
            .add_vehicle(vehicle_name, |vehicle| vehicle.car())
            .with_storyboard(|storyboard| storyboard.with_init_actions(init))
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
            .add_global_environment_action("Environment")
            .add_teleport_action("ego", ego_position)
            .add_speed_action("ego", 30.0)
            .add_teleport_action("target", target_position)
            .add_speed_action("target", 25.0)
            .build()
            .unwrap();

        Self::create()
            .add_vehicle("ego", |vehicle| vehicle.car())
            .add_vehicle("target", |vehicle| vehicle.car())
            .with_storyboard(|storyboard| storyboard.with_init_actions(init))
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
            .add_global_environment_action("Environment")
            .add_teleport_action("Ego", ego_position)
            .add_speed_action("Ego", 16.67) // 60 km/h
            .add_teleport_action("TargetVehicle", target_position)
            .add_speed_action("TargetVehicle", 13.89) // 50 km/h
            .build()
            .unwrap();

        Self::create_with_header("ALKS Scenario", "openscenario-rs")
            .add_vehicle("Ego", |vehicle| vehicle.car())
            .add_vehicle("TargetVehicle", |vehicle| vehicle.car())
            .with_storyboard(|storyboard| storyboard.with_init_actions(init))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Each template builds (the builder's entity-reference rule passes), declares exactly the
    /// entities its init actions name, and gives each of them a teleport and a speed action.
    /// The shipped templates once referenced entities they never declared.
    #[test]
    fn every_template_builds_and_declares_the_entities_its_init_names() {
        let cases: [(ScenarioBuilder<Complete>, &str, &[&str]); 3] = [
            (
                BasicScenarioTemplate::single_vehicle("ego"),
                "Basic Scenario",
                &["ego"],
            ),
            (
                BasicScenarioTemplate::two_vehicle_scenario(),
                "Basic Scenario",
                &["ego", "target"],
            ),
            (
                BasicScenarioTemplate::alks_template(),
                "ALKS Scenario",
                &["Ego", "TargetVehicle"],
            ),
        ];

        for (builder, description, names) in cases {
            let scenario = builder
                .build()
                .unwrap_or_else(|e| panic!("{description} {names:?} must build: {e}"));
            assert_eq!(
                scenario.file_header.description.as_literal().unwrap(),
                description
            );

            let declared: Vec<&str> = scenario
                .entities
                .as_ref()
                .unwrap()
                .scenario_objects
                .iter()
                .filter_map(|o| o.get_name())
                .collect();
            assert_eq!(declared, names);

            let init = &scenario.storyboard.as_ref().unwrap().init.actions;
            assert_eq!(init.global_actions.len(), 1, "{names:?}");
            let privates: Vec<(&str, Vec<&str>)> = init
                .private_actions
                .iter()
                .map(|p| {
                    (
                        p.entity_ref.as_literal().unwrap().as_str(),
                        p.private_actions.iter().map(|a| a.action_type()).collect(),
                    )
                })
                .collect();
            let expected: Vec<(&str, Vec<&str>)> = names
                .iter()
                .map(|n| (*n, vec!["TeleportAction", "LongitudinalAction"]))
                .collect();
            assert_eq!(privates, expected);
        }
    }
}
