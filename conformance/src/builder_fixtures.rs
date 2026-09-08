//! The builder's corpus.
//!
//! The parser harness enumerates 172 real `.xosc` files because the parser's input is *files*.
//! The builder's input is *code*, so its corpus has to be a set of builder programs. Those
//! programs already existed, scattered across `main/tests/*_builders_test.rs`,
//! `main/examples/builder_*.rs`, and `main/src/builder/templates/basic.rs`, where nothing ever
//! serialized their output. This module collects them behind one registry so
//! [`crate::check_builder_fixture`] can put every one of them through the same round-trip,
//! fidelity, and schema checks the corpus binaries apply to parsed files.
//!
//! Fixtures are construction only — the demos' `println!` narration is dropped. Add a new one
//! here rather than writing a standalone test, so it picks up all three checks for free.

use openscenario_rs::builder::{BuilderResult, CatalogLocationsBuilder, StoryboardBuilder};
use openscenario_rs::types::catalogs::locations::CatalogLocations;
use openscenario_rs::types::enums::ParameterType;
use openscenario_rs::types::road::RoadNetwork;
use openscenario_rs::types::scenario::triggers::Trigger;
use openscenario_rs::types::OpenScenario;
use openscenario_rs::ScenarioBuilder;

/// One builder program, named for reporting.
pub struct BuilderFixture {
    pub name: &'static str,
    pub build: fn() -> BuilderResult<OpenScenario>,
}

/// Every builder program the harness knows about, in a stable order.
pub fn fixtures() -> Vec<BuilderFixture> {
    vec![
        BuilderFixture {
            name: "minimal",
            build: minimal,
        },
        BuilderFixture {
            name: "header_and_entities_only",
            build: header_and_entities_only,
        },
        BuilderFixture {
            name: "with_parameters",
            build: with_parameters,
        },
        BuilderFixture {
            name: "explicit_empty_init",
            build: explicit_empty_init,
        },
        BuilderFixture {
            name: "one_vehicle",
            build: one_vehicle,
        },
        BuilderFixture {
            name: "one_pedestrian",
            build: one_pedestrian,
        },
        BuilderFixture {
            name: "template_single_vehicle",
            build: template_single_vehicle,
        },
        BuilderFixture {
            name: "template_two_vehicle",
            build: template_two_vehicle,
        },
        BuilderFixture {
            name: "template_alks",
            build: template_alks,
        },
        BuilderFixture {
            name: "story_with_speed_action",
            build: story_with_speed_action,
        },
        BuilderFixture {
            name: "cut_in_detached",
            build: cut_in_detached,
        },
        BuilderFixture {
            name: "comprehensive_overtaking",
            build: comprehensive_overtaking,
        },
        BuilderFixture {
            name: "alks_4_1_1_free_driving",
            build: alks_4_1_1_free_driving,
        },
    ]
}

/// The scenario `main/tests/scenario_builder_test.rs:8` builds.
fn minimal() -> BuilderResult<OpenScenario> {
    ScenarioBuilder::new()
        .with_header("Test", "Author")
        .with_catalog_locations(CatalogLocations::default())
        .with_road_network(RoadNetwork::default())
        .with_entities()
        .with_storyboard(|storyboard| storyboard)
        .build()
}

/// `main/examples/builder_basic_demo.rs:18` — header and entities, plus the required catalog
/// locations, road network, and (empty) storyboard the XSD demands.
fn header_and_entities_only() -> BuilderResult<OpenScenario> {
    ScenarioBuilder::new()
        .with_header("Basic Highway Scenario", "Builder Demo")
        .with_catalog_locations(CatalogLocations::default())
        .with_road_network(RoadNetwork::default())
        .with_entities()
        .with_storyboard(|storyboard| storyboard)
        .build()
}

/// `main/tests/scenario_builder_test.rs:23`.
fn with_parameters() -> BuilderResult<OpenScenario> {
    ScenarioBuilder::new()
        .with_header("Test", "Author")
        .add_parameter("initial_speed", ParameterType::Double, "25.0")
        .add_parameter("target_lane", ParameterType::Int, "1")
        .with_catalog_locations(CatalogLocations::default())
        .with_road_network(RoadNetwork::default())
        .with_entities()
        .with_storyboard(|storyboard| storyboard)
        .build()
}

/// `main/tests/complete_scenario_builder_test.rs:41` — an `Init` with no actions at all.
fn explicit_empty_init() -> BuilderResult<OpenScenario> {
    use openscenario_rs::types::scenario::init::{Actions, Init};

    let empty_init = Init {
        actions: Actions {
            global_actions: vec![],
            user_defined_actions: vec![],
            private_actions: vec![],
        },
    };

    ScenarioBuilder::new()
        .with_header("Test", "Author")
        .with_catalog_locations(CatalogLocations::default())
        .with_road_network(RoadNetwork::default())
        .with_entities()
        .with_storyboard(|storyboard| storyboard.with_init_actions(empty_init))
        .build()
}

/// `main/tests/vehicle_builders_test.rs` — a single declared vehicle.
fn one_vehicle() -> BuilderResult<OpenScenario> {
    ScenarioBuilder::new()
        .with_header("Vehicle Test", "Author")
        .with_catalog_locations(CatalogLocations::default())
        .with_road_network(RoadNetwork::default())
        .with_entities()
        .add_vehicle("ego", |v| v.car())
        .with_storyboard(|storyboard| storyboard)
        .build()
}

/// `main/tests/pedestrian_builder_test.rs:9`.
fn one_pedestrian() -> BuilderResult<OpenScenario> {
    ScenarioBuilder::new()
        .with_header("Test", "Author")
        .with_catalog_locations(CatalogLocations::default())
        .with_road_network(RoadNetwork::default())
        .with_entities()
        .add_pedestrian("ped1", |p| p.pedestrian().with_mass(75.0).finish())
        .with_storyboard(|storyboard| storyboard)
        .build()
}

/// `BasicScenarioTemplate::single_vehicle` (`main/src/builder/templates/basic.rs:27`).
fn template_single_vehicle() -> BuilderResult<OpenScenario> {
    use openscenario_rs::builder::templates::BasicScenarioTemplate;
    BasicScenarioTemplate::single_vehicle("ego").build()
}

/// `BasicScenarioTemplate::two_vehicle_scenario` (`.../basic.rs:46`).
fn template_two_vehicle() -> BuilderResult<OpenScenario> {
    use openscenario_rs::builder::templates::BasicScenarioTemplate;
    BasicScenarioTemplate::two_vehicle_scenario().build()
}

/// `BasicScenarioTemplate::alks_template` (`.../basic.rs:70`).
fn template_alks() -> BuilderResult<OpenScenario> {
    use openscenario_rs::builder::templates::BasicScenarioTemplate;
    BasicScenarioTemplate::alks_template().build()
}

/// `main/tests/complete_scenario_builder_test.rs:7` — the nested closure API.
fn story_with_speed_action() -> BuilderResult<OpenScenario> {
    ScenarioBuilder::new()
        .with_header("Highway Test", "Test Author")
        .with_catalog_locations(CatalogLocations::default())
        .with_road_network(RoadNetwork::default())
        .with_entities()
        .add_vehicle("ego", |vehicle| vehicle.car())
        .add_vehicle("target", |vehicle| vehicle.car())
        .with_storyboard(|storyboard| {
            storyboard.add_story("main_story", |story| {
                story.add_act("acceleration_act", |act| {
                    act.add_maneuver("speed_up", "ego", |maneuver| {
                        maneuver
                            .add_speed_action(|speed| speed.named("accelerate").to_speed(30.0))
                            .unwrap()
                    })
                })
            })
        })
        .build()
}

/// `main/examples/cut_in_scenario_demo.rs:32` — the detached builder pattern.
fn cut_in_detached() -> BuilderResult<OpenScenario> {
    let scenario_builder = ScenarioBuilder::new()
        .with_header("Cut-in scenario", "OpenSCENARIO-rs Builder Demo")
        .with_catalog_locations(CatalogLocations::default())
        .with_road_network(RoadNetwork::default())
        .with_entities();

    let mut storyboard_builder = StoryboardBuilder::new(scenario_builder);
    let mut story_builder = storyboard_builder.add_story_simple("Cutin");

    let mut act = story_builder.create_act("Act_Ego");
    let mut maneuver = act.create_maneuver("Sequence_Ego", "Ego");
    let speed = maneuver
        .create_speed_action()
        .named("EgoSpeed")
        .to_speed(29.0)
        .with_trigger(Trigger {
            condition_groups: vec![],
        });

    speed.attach_to_detached(&mut maneuver)?;
    maneuver.attach_to_detached(&mut act);
    act.attach_to(&mut story_builder);

    storyboard_builder.finish().build()
}

/// `main/examples/builder_comprehensive_demo.rs:24` — parameters, a road file, three acts.
fn comprehensive_overtaking() -> BuilderResult<OpenScenario> {
    let scenario_builder = ScenarioBuilder::new()
        .with_header(
            "Highway Overtaking Scenario with Conditional Behavior",
            "OpenSCENARIO-rs Builder Demo",
        )
        .add_parameter("initial_speed", ParameterType::Double, "25.0")
        .add_parameter("target_speed", ParameterType::Double, "35.0")
        .add_parameter("overtake_distance", ParameterType::Double, "50.0")
        .with_catalog_locations(CatalogLocations::default())
        .with_road_file("highway.xodr")
        .with_entities();

    let mut storyboard_builder = StoryboardBuilder::new(scenario_builder);
    let mut story_builder = storyboard_builder.add_story_simple("highway_overtaking");

    for (act_name, maneuver_name, action_name, speed) in [
        (
            "initial_acceleration",
            "ego_accelerate",
            "initial_acceleration",
            25.0,
        ),
        (
            "conditional_overtaking",
            "ego_overtake",
            "overtake_acceleration",
            35.0,
        ),
    ] {
        let mut act = story_builder.create_act(act_name);
        let mut maneuver = act.create_maneuver(maneuver_name, "ego");
        let action = maneuver
            .create_speed_action()
            .named(action_name)
            .to_speed(speed)
            .with_trigger(Trigger {
                condition_groups: vec![],
            });
        action.attach_to_detached(&mut maneuver)?;
        maneuver.attach_to_detached(&mut act);
        act.attach_to(&mut story_builder);
    }

    // The third act uses a teleport rather than a speed action.
    let mut act3 = story_builder.create_act("lane_change");
    let mut maneuver3 = act3.create_maneuver("lane_change_maneuver", "ego");
    let teleport = maneuver3
        .create_teleport_action()
        .named("lane_change")
        .to()
        .world_position(100.0, 0.0, 0.0)
        .with_trigger(Trigger {
            condition_groups: vec![],
        });
    teleport.attach_to_detached(&mut maneuver3)?;
    maneuver3.attach_to_detached(&mut act3);
    act3.attach_to(&mut story_builder);

    storyboard_builder.finish().build()
}

/// `main/examples/alks_scenario_4_1_1_comprehensive.rs` — the repo's most complete builder
/// program, and a hand-written reconstruction of a scenario that also exists in the corpus as
/// `corpus/logical_scenarios/alks_scenario_4_1_1_free_driving_variation.xosc`. That pairing
/// is what the `--coverage` mode of the `builder` binary compares against.
pub fn alks_4_1_1_free_driving() -> BuilderResult<OpenScenario> {
    let mut scenario_builder = ScenarioBuilder::new()
        .with_header(
            "ALKS Scenario 4.1.1 Free Driving - Professional Implementation",
            "OpenSCENARIO-rs Comprehensive Builder Demo",
        )
        .with_catalog_locations(
            CatalogLocationsBuilder::new()
                .with_vehicle_catalog("./catalogs/vehicles/alks_vehicles")
                .with_pedestrian_catalog("./catalogs/pedestrians")
                .with_controller_catalog("./catalogs/controllers/alks_controllers")
                .build(),
        )
        .add_parameter("EgoInitialSpeed", ParameterType::Double, "30.0")
        .add_parameter("TargetCruiseSpeed", ParameterType::Double, "25.0")
        .add_parameter("FinalTestSpeed", ParameterType::Double, "23.0")
        .add_parameter("FollowingDistance", ParameterType::Double, "50.0")
        .add_parameter("ALKSActivationTime", ParameterType::Double, "5.0")
        .add_parameter("TestDuration", ParameterType::Double, "120.0")
        .add_parameter("WeatherCondition", ParameterType::String, "Clear")
        .add_parameter("RoadFriction", ParameterType::Double, "0.8")
        .add_parameter("TimeOfDay", ParameterType::String, "Noon")
        .with_road_file("./road_networks/alks_highway_straight_3lane.xodr")
        .with_entities();

    for (name, dims, perf) in [
        ("Ego", (4.5, 1.8, 1.4), (180.0, 4.0, 9.0)),
        ("Target", (4.2, 1.7, 1.3), (160.0, 3.5, 8.5)),
        ("Background", (4.0, 1.6, 1.2), (140.0, 3.0, 7.5)),
    ] {
        scenario_builder = scenario_builder.add_vehicle(name, |v| {
            v.car()
                .with_dimensions(dims.0, dims.1, dims.2)
                .with_performance(perf.0, perf.1, perf.2)
        });
    }

    let mut storyboard_builder = StoryboardBuilder::new(scenario_builder);
    let mut story_builder = storyboard_builder.add_story_simple("ALKS_FreeDriving_TestSequence");

    // Three acts: highway entry, adaptive following, steady-state validation.
    let acts: [(&str, &[(&str, &str, &str, f64)]); 3] = [
        (
            "Act1_ALKSActivation",
            &[
                ("EgoHighwayEntry", "Ego", "InitialAcceleration", 30.0),
                ("TargetEstablishCruise", "Target", "TargetCruiseSpeed", 25.0),
                (
                    "BackgroundTrafficFlow",
                    "Background",
                    "BackgroundCruise",
                    27.0,
                ),
            ],
        ),
        (
            "Act2_AdaptiveFollowing",
            &[
                ("EgoALKSEngagement", "Ego", "ALKSSpeedAdaptation", 25.0),
                ("TargetSpeedVariation", "Target", "TargetSpeedChange", 23.0),
            ],
        ),
        (
            "Act3_SteadyStateValidation",
            &[
                ("EgoSteadyFollowing", "Ego", "ALKSSteadyState", 23.0),
                ("TargetSteadyCruise", "Target", "TargetSteadySpeed", 23.0),
            ],
        ),
    ];

    for (act_name, maneuvers) in acts {
        let mut act = story_builder.create_act(act_name);
        for (maneuver_name, entity, action_name, speed) in maneuvers {
            let mut maneuver = act.create_maneuver(maneuver_name, entity);
            let action = maneuver
                .create_speed_action()
                .named(action_name)
                .to_speed(*speed)
                .with_trigger(Trigger {
                    condition_groups: vec![],
                });
            action.attach_to_detached(&mut maneuver)?;
            maneuver.attach_to_detached(&mut act);
        }
        act.attach_to(&mut story_builder);
    }

    story_builder.finish();
    storyboard_builder.stop_after_time(120.0)?.finish().build()
}
