//! Whole-document parsing of the repository's own fixtures: `tests/data/simple_scenario.xosc`,
//! `tests/data/expressions_scenario.xosc` and `tests/data/cut_in_101_exam.xosc` (a real
//! exported scenario whose numeric attributes are written in scientific notation).
//!
//! Each test asserts parsed values read from the fixture, and the two round trips assert that
//! the reparsed document equals the parsed one, not only that it parses again.

use openscenario_rs::parse_str;
use openscenario_rs::types::actions::movement::{RoutingActionChoice, SpeedActionTargetChoice};
use openscenario_rs::types::basic::Value;
use openscenario_rs::types::enums::{PedestrianCategory, Rule, VehicleCategory};
use openscenario_rs::types::scenario::init::{LongitudinalActionChoice, PrivateActionChoice};
use openscenario_rs::types::scenario::story::{StoryActionChoice, StoryPrivateActionChoice};
use openscenario_rs::types::OpenScenario;
use std::fs;

/// Helper function to extract entities and storyboard from OpenScenario
fn assert_scenario_document(
    scenario: &OpenScenario,
) -> (
    &openscenario_rs::types::Entities,
    &openscenario_rs::types::Storyboard,
) {
    assert!(scenario.is_scenario(), "Expected scenario document");
    (
        scenario
            .entities
            .as_ref()
            .expect("Scenario should have entities"),
        scenario
            .storyboard
            .as_ref()
            .expect("Scenario should have storyboard"),
    )
}

#[test]
fn can_parse_simple_scenario_from_string() {
    let xml = include_str!("data/simple_scenario.xosc");
    let scenario = parse_str(xml).unwrap();

    let header = &scenario.file_header;
    assert_eq!(header.author.as_literal().unwrap(), "OpenSCENARIO-rs");
    assert_eq!(header.date.as_literal().unwrap(), "2024-01-01T00:00:00");
    assert_eq!(
        header.description.as_literal().unwrap(),
        "Simple test scenario for parsing validation"
    );
    assert_eq!(header.rev_major.as_literal().unwrap(), &1);
    assert_eq!(header.rev_minor.as_literal().unwrap(), &2);

    let (entities, _storyboard) = assert_scenario_document(&scenario);
    assert_eq!(entities.scenario_objects.len(), 2);
}

#[test]
fn can_access_entities() {
    let xml = include_str!("data/simple_scenario.xosc");
    let scenario = parse_str(xml).unwrap();

    let entities = scenario
        .entities
        .as_ref()
        .expect("Scenario should have entities");
    assert_eq!(entities.scenario_objects.len(), 2);

    // Find the ego vehicle
    let ego = entities.find_object("Ego").unwrap();
    assert_eq!(ego.get_name(), Some("Ego"));

    if let Some(vehicle) = ego.vehicle() {
        assert_eq!(vehicle.name.as_literal().unwrap(), "EgoVehicle");
        assert_eq!(
            vehicle.vehicle_category,
            Value::Literal(VehicleCategory::Car)
        );

        // Check bounding box
        assert_eq!(vehicle.bounding_box.center.x.as_literal().unwrap(), &0.0);
        assert_eq!(
            vehicle.bounding_box.dimensions.width.as_literal().unwrap(),
            &2.0
        );
        assert_eq!(
            vehicle.bounding_box.dimensions.length.as_literal().unwrap(),
            &4.5
        );
    } else {
        panic!("Expected vehicle for Ego");
    }

    // Find the pedestrian
    let ped = entities.find_object("Pedestrian1").unwrap();
    assert_eq!(ped.get_name(), Some("Pedestrian1"));

    if let Some(pedestrian) = ped.pedestrian() {
        assert_eq!(pedestrian.name.as_literal().unwrap(), "TestPedestrian");
        assert_eq!(
            pedestrian.pedestrian_category,
            Value::Literal(PedestrianCategory::Pedestrian)
        );

        // Check bounding box
        assert_eq!(
            pedestrian.bounding_box.center.x.as_literal().unwrap(),
            &10.0
        );
        assert_eq!(pedestrian.bounding_box.center.y.as_literal().unwrap(), &2.0);
        assert_eq!(
            pedestrian
                .bounding_box
                .dimensions
                .height
                .as_literal()
                .unwrap(),
            &1.8
        );
    } else {
        panic!("Expected pedestrian");
    }
}

/// Serializing a parsed document and parsing the output again must reproduce the same value.
#[test]
fn can_serialize_and_deserialize_scenario() {
    let xml = include_str!("data/simple_scenario.xosc");
    let original = parse_str(xml).unwrap();

    let serialized_xml = openscenario_rs::serialize_str(&original).unwrap();
    let roundtrip = parse_str(&serialized_xml).unwrap();

    assert_eq!(roundtrip, original);
}

/// A `FileHeader` naming only `@author` lacks four attributes the XSD requires (`date`,
/// `description`, `revMajor`, `revMinor`); the parse error must name the first of them rather
/// than fail somewhere later in an otherwise complete document.
#[test]
fn handles_missing_required_fields() {
    let incomplete_xml = r#"<?xml version="1.0"?>
    <OpenSCENARIO>
      <FileHeader author="Test"/>
      <CatalogLocations/>
      <RoadNetwork/>
      <Entities/>
      <Storyboard><Init><Actions/></Init><StopTrigger/></Storyboard>
    </OpenSCENARIO>"#;

    let message = parse_str(incomplete_xml).unwrap_err().to_string();
    assert!(
        message.contains("missing field `@date`"),
        "expected a missing-attribute error, got: {message}"
    );
}

/// `${...}` attribute values parse as expressions, not as a failed `xsd:double`.
#[test]
fn can_parse_scenario_with_expressions() {
    let xml = include_str!("data/expressions_scenario.xosc");
    let scenario = parse_str(xml).expect("expressions_scenario.xosc must parse");

    let entities = scenario
        .entities
        .as_ref()
        .expect("Scenario should have entities");
    assert_eq!(entities.scenario_objects.len(), 1);

    let vehicle = entities
        .find_object("Ego")
        .and_then(|ego| ego.vehicle())
        .expect("Ego is a vehicle");
    let dimensions = &vehicle.bounding_box.dimensions;
    assert_eq!(
        dimensions.width,
        Value::Expression("vehicle_width".to_string())
    );
    assert_eq!(
        dimensions.length,
        Value::Expression("vehicle_length".to_string())
    );
    assert_eq!(
        dimensions.height,
        Value::Expression("vehicle_height".to_string())
    );
    assert_eq!(vehicle.bounding_box.center.x, Value::Literal(0.0));
}

// Integration tests for the cut_in_101_exam.xosc scenario
mod cut_in_scenario_tests {
    use super::*;

    fn cut_in() -> OpenScenario {
        let xml = fs::read_to_string("tests/data/cut_in_101_exam.xosc")
            .expect("Failed to read cut_in_101_exam.xosc file");
        parse_str(&xml).expect("cut_in_101_exam.xosc must parse successfully")
    }

    #[test]
    fn can_parse_cut_in_101_exam_scenario() {
        let scenario = cut_in();

        assert_eq!(
            scenario.file_header.author.as_literal().unwrap(),
            "OnSite_TOPS"
        );
        assert_eq!(scenario.file_header.rev_major.as_literal().unwrap(), &1);
        assert_eq!(scenario.file_header.rev_minor.as_literal().unwrap(), &0);
        assert_eq!(
            scenario.file_header.date.as_literal().unwrap(),
            "2021-11-02T16:20:00"
        );
    }

    #[test]
    fn can_access_cut_in_entities() {
        let scenario = cut_in();
        let entities = scenario
            .entities
            .as_ref()
            .expect("Scenario should have entities");

        // Should have 3 entities: Ego, A1, A2
        assert_eq!(entities.scenario_objects.len(), 3);

        let vehicle = entities
            .find_object("Ego")
            .and_then(|ego| ego.vehicle())
            .expect("Expected vehicle for Ego");
        assert_eq!(vehicle.name.as_literal().unwrap(), "Default_car");
        assert_eq!(
            vehicle.vehicle_category,
            Value::Literal(VehicleCategory::Car)
        );

        // Written as `1.5000000000000000e+00`, `9.0000000000000002e-01` and so on.
        assert_eq!(vehicle.bounding_box.center.x.as_literal().unwrap(), &1.5);
        assert_eq!(vehicle.bounding_box.center.y.as_literal().unwrap(), &0.0);
        assert_eq!(vehicle.bounding_box.center.z.as_literal().unwrap(), &0.9);
        assert_eq!(
            vehicle.bounding_box.dimensions.width.as_literal().unwrap(),
            &2.1
        );
        assert_eq!(
            vehicle.bounding_box.dimensions.length.as_literal().unwrap(),
            &4.5
        );
        assert_eq!(
            vehicle.bounding_box.dimensions.height.as_literal().unwrap(),
            &1.8
        );

        assert_eq!(
            entities.find_object("A1").and_then(|o| o.get_name()),
            Some("A1")
        );
        assert_eq!(
            entities.find_object("A2").and_then(|o| o.get_name()),
            Some("A2")
        );
    }

    /// The `Init` section: one global environment action, then one `Private` block per entity,
    /// each holding a speed action followed by a teleport.
    #[test]
    fn can_access_cut_in_storyboard() {
        let scenario = cut_in();
        let init = &scenario
            .storyboard
            .as_ref()
            .expect("Scenario should have storyboard")
            .init;

        assert_eq!(init.actions.global_actions.len(), 1);

        let refs: Vec<&str> = init
            .actions
            .private_actions
            .iter()
            .map(|p| p.entity_ref.as_literal().unwrap().as_str())
            .collect();
        assert_eq!(refs, ["Ego", "A1", "A2"]);

        for private in &init.actions.private_actions {
            let kinds: Vec<&str> = private
                .private_actions
                .iter()
                .map(|a| a.action_type())
                .collect();
            assert_eq!(kinds, ["LongitudinalAction", "TeleportAction"]);
        }

        let PrivateActionChoice::LongitudinalAction(longitudinal) =
            &init.actions.private_actions[0].private_actions[0].action
        else {
            panic!("Ego's first init action is a LongitudinalAction");
        };
        let LongitudinalActionChoice::SpeedAction(speed) = &longitudinal.action else {
            panic!("expected a SpeedAction");
        };
        let SpeedActionTargetChoice::AbsoluteTargetSpeed(target) =
            &speed.speed_action_target.target
        else {
            panic!("expected an absolute target speed");
        };
        // `value="2.3307059523850903e+01"`
        assert_eq!(target.value, Value::Literal(2.3307059523850903e+01));
    }

    #[test]
    fn can_validate_cut_in_story_structure() {
        let scenario = cut_in();
        let storyboard = scenario
            .storyboard
            .as_ref()
            .expect("Scenario should have storyboard");

        let story = &storyboard.stories[0];
        assert_eq!(story.name.as_literal().unwrap(), "Cutin");

        let act = &story.acts[0];
        assert_eq!(act.name.as_literal().unwrap(), "Act_Ego");

        let group = &act.maneuver_groups[0];
        assert_eq!(group.name.as_literal().unwrap(), "Sequence_Ego");
    }

    /// Serializing the parsed file and parsing the output again must reproduce the same value,
    /// including every scientific-notation coordinate.
    #[test]
    fn can_roundtrip_cut_in_scenario() {
        let scenario = cut_in();

        let serialized_xml =
            openscenario_rs::serialize_str(&scenario).expect("Serialization must succeed");
        let roundtrip_scenario =
            parse_str(&serialized_xml).expect("Roundtrip parsing must succeed");

        assert_eq!(roundtrip_scenario, scenario);
    }

    /// Ego's `Trajectory_Ego` polyline: 125 vertices whose coordinates are written as
    /// `9.2884257876425379e-04`-style literals, which must parse to the value written.
    #[test]
    fn can_access_trajectory_vertices_with_scientific_notation() {
        let scenario = cut_in();
        let storyboard = scenario
            .storyboard
            .as_ref()
            .expect("Scenario should have storyboard");
        let event = &storyboard.stories[0].acts[0].maneuver_groups[0].maneuvers[0].events[0];

        let StoryActionChoice::PrivateAction(private) = &event.actions[0].action else {
            panic!("expected a private action");
        };
        let StoryPrivateActionChoice::RoutingAction(routing) = &private.action else {
            panic!("expected a RoutingAction");
        };
        let RoutingActionChoice::FollowTrajectoryAction(follow) = &routing.routing_choice else {
            panic!("expected a FollowTrajectoryAction");
        };
        let trajectory = follow.trajectory.as_ref().expect("inline Trajectory");
        assert_eq!(trajectory.name.as_literal().unwrap(), "Trajectory_Ego");

        let polyline = trajectory.shape.as_polyline().expect("Polyline shape");
        assert_eq!(polyline.vertices.len(), 125);

        let first = polyline.vertices[0]
            .position
            .world_position()
            .expect("WorldPosition");
        assert_eq!(first.h, Some(Value::Literal(9.2884257876425379e-04)));

        let second = &polyline.vertices[1];
        assert_eq!(second.time, Some(Value::Literal(0.04)));
        let second = second.position.world_position().expect("WorldPosition");
        assert_eq!(second.x, Value::Literal(9.3177232986101521e-01));
        assert_eq!(second.y, Value::Literal(8.6547006258363979e-04));
    }
}

#[test]
fn can_validate_trajectory_following_modes() {
    let xml = fs::read_to_string("tests/data/cut_in_101_exam.xosc")
        .expect("Failed to read cut_in_101_exam.xosc file");

    let scenario = parse_str(&xml).expect("cut_in_101_exam.xosc must parse successfully");

    // cut_in_101_exam.xosc uses followingMode="follow"
    let mut routing_actions_found = 0;

    // RoutingActions are in the Story structure, not Init - look in stories
    let storyboard = scenario
        .storyboard
        .as_ref()
        .expect("Scenario should have storyboard");
    for story in &storyboard.stories {
        for act in &story.acts {
            for maneuver_group in &act.maneuver_groups {
                for maneuver in &maneuver_group.maneuvers {
                    for event in &maneuver.events {
                        let actions = &event.actions;
                        for action in actions {
                            if let StoryActionChoice::PrivateAction(private_action) = &action.action
                            {
                                if let StoryPrivateActionChoice::RoutingAction(routing) =
                                    &private_action.action
                                {
                                    if let RoutingActionChoice::FollowTrajectoryAction(
                                        follow_action,
                                    ) = &routing.routing_choice
                                    {
                                        routing_actions_found += 1;

                                        use openscenario_rs::types::enums::FollowingMode;
                                        assert_eq!(
                                            follow_action.trajectory_following_mode.following_mode,
                                            Value::Literal(FollowingMode::Follow),
                                            "cut_in_101_exam.xosc uses followingMode='follow'"
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // One FollowTrajectoryAction per vehicle: Ego, A1, A2.
    assert_eq!(routing_actions_found, 3);
}

/// The first event's start trigger:
/// `<Condition name="" delay="0" conditionEdge="rising">` over
/// `<SimulationTimeCondition value="0.03" rule="greaterThan"/>`.
#[test]
fn tdd_can_parse_simulation_time_conditions() {
    use openscenario_rs::types::conditions::value::ByValueConditionChoice;
    use openscenario_rs::types::enums::ConditionEdge;
    use openscenario_rs::types::scenario::triggers::ConditionChoice;

    let xml = fs::read_to_string("tests/data/cut_in_101_exam.xosc")
        .expect("Failed to read cut_in_101_exam.xosc file");
    let scenario = parse_str(&xml).expect("cut_in_101_exam.xosc must parse successfully");

    let storyboard = scenario
        .storyboard
        .as_ref()
        .expect("Scenario should have storyboard");
    let trigger = storyboard.stories[0].acts[0].maneuver_groups[0].maneuvers[0].events[0]
        .start_trigger
        .as_ref()
        .expect("Events should have StartTrigger parsed");
    assert_eq!(trigger.condition_groups.len(), 1);

    let condition = &trigger.condition_groups[0].conditions[0];
    assert_eq!(condition.name.as_literal().unwrap(), "");
    assert_eq!(condition.delay, Value::Literal(0.0));
    assert_eq!(
        condition.condition_edge,
        Value::Literal(ConditionEdge::Rising)
    );

    let ConditionChoice::ByValueCondition(by_value) = &condition.choice else {
        panic!("Expected ByValue condition type");
    };
    let ByValueConditionChoice::SimulationTimeCondition(sim_time) = &by_value.choice else {
        panic!("Expected SimulationTimeCondition");
    };
    assert_eq!(sim_time.value, Value::Literal(0.03));
    assert_eq!(sim_time.rule, Value::Literal(Rule::GreaterThan));
}
