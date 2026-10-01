#[cfg(feature = "builder")]
mod detached_builders_tests {
    use openscenario_rs::builder::storyboard::{DetachedActBuilder, DetachedManeuverBuilder};
    use openscenario_rs::builder::{ScenarioBuilder, StoryboardBuilder};

    use openscenario_rs::types::catalogs::locations::CatalogLocations;
    use openscenario_rs::types::road::RoadNetwork;
    #[test]
    fn test_detached_act_builder_creation() {
        // Test that DetachedActBuilder can be created and used without lifetime issues
        let scenario_builder = ScenarioBuilder::new()
            .with_header("Test", "Author")
            // Required of a scenario document by the XSD, even when empty.
            .with_catalog_locations(CatalogLocations::default())
            .with_road_network(RoadNetwork::default())
            .with_entities();

        let mut storyboard_builder = StoryboardBuilder::new(scenario_builder);
        let mut story_builder = storyboard_builder.add_story_simple("TestStory");

        // Create detached act builder - this should work without lifetime constraints
        let detached_act = story_builder.create_act("act1");

        // Test fluent chaining
        let detached_act =
            detached_act.with_start_trigger(openscenario_rs::types::scenario::triggers::Trigger {
                condition_groups: vec![],
            });

        // An act with no maneuver group is refused: XSD Act declares ManeuverGroup with
        // the default minOccurs="1". This used to succeed and attach an empty act.
        assert!(detached_act.attach_to(&mut story_builder).is_err());
    }

    #[test]
    fn test_detached_maneuver_builder_creation() {
        // Test that DetachedManeuverBuilder can be created and used
        let mut detached_act = DetachedActBuilder::new("act1");

        // Create detached maneuver builder
        let detached_maneuver = detached_act.create_maneuver("maneuver1", "vehicle1");

        // A maneuver with no event is refused: XSD Maneuver declares Event with the
        // default minOccurs="1".
        assert!(detached_maneuver
            .attach_to_detached(&mut detached_act)
            .is_err());
    }

    /// A speed event built on a detached maneuver, attached to a detached act, comes out of
    /// `DetachedActBuilder::build` with the name, trigger, target and actor it was given.
    #[test]
    fn test_perfect_fluent_chaining() {
        use openscenario_rs::types::actions::movement::SpeedActionTargetChoice;
        use openscenario_rs::types::basic::Value;
        use openscenario_rs::types::scenario::init::LongitudinalActionChoice;
        use openscenario_rs::types::scenario::story::{
            StoryActionChoice, StoryPrivateActionChoice,
        };
        use openscenario_rs::types::scenario::triggers::Trigger;

        let mut detached_act = DetachedActBuilder::new("act1");
        let mut detached_maneuver = DetachedManeuverBuilder::new("maneuver1", "vehicle1");

        detached_maneuver
            .create_speed_action()
            .named("speed_event")
            .to_speed(30.0)
            .with_trigger(Trigger {
                condition_groups: vec![],
            })
            .attach_to_detached(&mut detached_maneuver)
            .unwrap();
        detached_maneuver
            .attach_to_detached(&mut detached_act)
            .unwrap();

        let act = detached_act
            .build()
            .expect("one maneuver group satisfies Act");
        let group = &act.maneuver_groups[0];
        assert_eq!(
            group.actors.entity_refs[0].entity_ref,
            Value::literal("vehicle1".to_string())
        );
        let maneuver = &group.maneuvers[0];
        assert_eq!(maneuver.name.as_literal().unwrap(), "maneuver1");

        let event = &maneuver.events[0];
        assert_eq!(event.name.as_literal().unwrap(), "speed_event");
        assert_eq!(
            event.start_trigger,
            Some(Trigger {
                condition_groups: vec![]
            })
        );

        let StoryActionChoice::PrivateAction(private) = &event.actions[0].action else {
            panic!("expected a private action");
        };
        let StoryPrivateActionChoice::LongitudinalAction(longitudinal) = &private.action else {
            panic!("expected a LongitudinalAction");
        };
        let LongitudinalActionChoice::SpeedAction(speed) = &longitudinal.action else {
            panic!("expected a SpeedAction");
        };
        let SpeedActionTargetChoice::AbsoluteTargetSpeed(target) =
            &speed.speed_action_target.target
        else {
            panic!("expected an absolute target speed");
        };
        assert_eq!(target.value, Value::Literal(30.0));
    }

    /// `DetachedActBuilder::attach_to` puts the built act on the story, and the story reaches
    /// the document through `StoryBuilder::finish` and `StoryboardBuilder::finish` (XSD
    /// `Storyboard` :2112 → `Story` :2105 → `Act` :697 → `ManeuverGroup` → `Maneuver` → `Event`).
    #[test]
    fn detached_act_attached_to_story_reaches_the_document() {
        let scenario_builder = ScenarioBuilder::new()
            .with_header("Test", "Author")
            .with_catalog_locations(CatalogLocations::default())
            .with_road_network(RoadNetwork::default())
            .with_entities()
            .add_vehicle("vehicle1", |vehicle| vehicle.car());

        let mut storyboard_builder = StoryboardBuilder::new(scenario_builder);
        let mut story_builder = storyboard_builder.add_story_simple("TestStory");
        let mut detached_act = story_builder.create_act("act1");
        let mut detached_maneuver = detached_act.create_maneuver("maneuver1", "vehicle1");
        detached_maneuver
            .create_speed_action()
            .named("speed_event")
            .to_speed(30.0)
            .attach_to_detached(&mut detached_maneuver)
            .unwrap();
        detached_maneuver
            .attach_to_detached(&mut detached_act)
            .unwrap();
        detached_act.attach_to(&mut story_builder).unwrap();
        story_builder.finish().expect("the story holds one act");

        let scenario = storyboard_builder.finish().build().unwrap();
        let stories = &scenario.storyboard.as_ref().unwrap().stories;
        assert_eq!(stories.len(), 1);
        assert_eq!(stories[0].name.as_literal().unwrap(), "TestStory");
        let acts = stories[0].acts.as_slice();
        assert_eq!(acts.len(), 1);
        assert_eq!(acts[0].name.as_literal().unwrap(), "act1");
        let group = &acts[0].maneuver_groups.as_slice()[0];
        assert_eq!(group.maneuvers[0].name.as_literal().unwrap(), "maneuver1");
        assert_eq!(
            group.maneuvers[0].events[0].name.as_literal().unwrap(),
            "speed_event"
        );
    }
}
