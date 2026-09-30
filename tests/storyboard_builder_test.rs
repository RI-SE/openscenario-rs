//! `StoryboardBuilder`'s stop-trigger shortcuts and the fluent init-action chain, checked on the
//! document `ScenarioBuilder::build` returns.

#![cfg(feature = "builder")]

use openscenario_rs::builder::scenario::{HasEntities, ScenarioBuilder};
use openscenario_rs::types::catalogs::locations::CatalogLocations;
use openscenario_rs::types::positions::{Position, WorldPosition};
use openscenario_rs::types::road::RoadNetwork;

fn with_entities(name: &str) -> ScenarioBuilder<HasEntities> {
    ScenarioBuilder::new()
        .with_header(name, "TestAuthor")
        // Both are required of a scenario document by the XSD.
        .with_catalog_locations(CatalogLocations::default())
        .with_road_network(RoadNetwork::default())
        .with_entities()
}

fn stop_trigger_xml(scenario: &openscenario_rs::types::OpenScenario) -> String {
    let trigger = scenario
        .storyboard
        .as_ref()
        .expect("storyboard")
        .stop_trigger
        .as_ref()
        .expect("the shortcut sets the storyboard's StopTrigger");
    quick_xml::se::to_string_with_root("StopTrigger", trigger).expect("serialize")
}

/// XSD `Condition` (`:953`), `ByValueCondition`, `SimulationTimeCondition` (`:2039`).
#[test]
fn stop_after_time_sets_a_simulation_time_stop_trigger() {
    let scenario = with_entities("StopAfterTime")
        .create_storyboard()
        .stop_after_time(30.0)
        .unwrap()
        .finish()
        .build()
        .unwrap();

    assert_eq!(
        stop_trigger_xml(&scenario),
        concat!(
            r#"<StopTrigger><ConditionGroup>"#,
            r#"<Condition name="TimeCondition" conditionEdge="rising" delay="0">"#,
            r#"<ByValueCondition><SimulationTimeCondition value="30" rule="greaterThan"/></ByValueCondition>"#,
            r#"</Condition></ConditionGroup></StopTrigger>"#,
        )
    );
}

/// XSD `ByEntityCondition` (`:825`), `TriggeringEntities` (`:2400`) and
/// `ReachPositionCondition` (`:1819`, `@tolerance` required, defaulted to 1 by the builder).
#[test]
fn stop_when_entity_reaches_sets_a_reach_position_stop_trigger() {
    let position = Position::world(WorldPosition::with_full_orientation(
        500.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ));
    let scenario = with_entities("StopWhenReached")
        .add_vehicle("ego", |vehicle| vehicle.car())
        .create_storyboard()
        .stop_when_entity_reaches("ego", position)
        .unwrap()
        .finish()
        .build()
        .unwrap();

    assert_eq!(
        stop_trigger_xml(&scenario),
        concat!(
            r#"<StopTrigger><ConditionGroup>"#,
            r#"<Condition name="ReachPositionCondition" conditionEdge="rising" delay="0">"#,
            r#"<ByEntityCondition>"#,
            r#"<TriggeringEntities triggeringEntitiesRule="any"><EntityRef entityRef="ego"/></TriggeringEntities>"#,
            r#"<EntityCondition><ReachPositionCondition tolerance="1">"#,
            r#"<Position><WorldPosition x="500" y="0" z="0" h="0" p="0" r="0"/></Position>"#,
            r#"</ReachPositionCondition></EntityCondition>"#,
            r#"</ByEntityCondition></Condition></ConditionGroup></StopTrigger>"#,
        )
    );
}

/// The fluent chain `create_init_actions` → `finish` → `stop_after_time` → `finish` keeps both
/// what the init chain added and the stop trigger.
#[test]
fn init_action_chain_and_stop_trigger_build_together() {
    let scenario = with_entities("Phase2ComprehensiveTest")
        // The init actions below teleport "ego", so the entity has to be declared.
        .add_vehicle("ego", |vehicle| vehicle.car())
        .create_storyboard()
        .create_init_actions()
        .add_global_environment_action("TestEnvironment")
        .add_teleport_action(
            "ego",
            Position::world(WorldPosition::with_full_orientation(
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            )),
        )
        .unwrap()
        .add_speed_action("ego", 15.0)
        .unwrap()
        .finish()
        .unwrap()
        .stop_after_time(60.0)
        .unwrap()
        .finish()
        .build()
        .expect("Comprehensive scenario should build successfully");

    let storyboard = scenario.storyboard.as_ref().unwrap();
    assert!(
        storyboard.stop_trigger.is_some(),
        "Stop trigger should be present"
    );

    let actions = &storyboard.init.actions;
    assert_eq!(actions.global_actions.len(), 1);
    assert_eq!(actions.private_actions.len(), 1);
    let ego = &actions.private_actions[0];
    assert_eq!(ego.entity_ref.as_literal().unwrap(), "ego");
    let kinds: Vec<&str> = ego
        .private_actions
        .iter()
        .map(|a| a.action_type())
        .collect();
    assert_eq!(kinds, ["TeleportAction", "LongitudinalAction"]);
}
