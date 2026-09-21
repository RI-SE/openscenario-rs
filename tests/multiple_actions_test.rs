use openscenario_rs::parser::xml::parse_from_str;
use openscenario_rs::types::actions::VisibilityAction;
use openscenario_rs::types::basic::MinVec;
use openscenario_rs::types::basic::Value;
use openscenario_rs::types::enums::Priority;
use openscenario_rs::types::scenario::story::{
    Event, StoryAction, StoryActionChoice, StoryPrivateAction, StoryPrivateActionChoice,
};

#[test]
fn test_event_multiple_actions_struct() {
    // Test that Event can now hold multiple actions
    let event = Event {
        name: Value::literal("MultiActionEvent".to_string()),
        maximum_execution_count: Some(Value::literal(1)),
        priority: Value::Literal(Priority::Override),
        actions: MinVec::new(vec![
            StoryAction::private(
                "Action1",
                StoryPrivateAction::visibility(VisibilityAction::new(true, true, true)),
            ),
            StoryAction::private(
                "Action2",
                StoryPrivateAction::visibility(VisibilityAction::new(true, true, true)),
            ),
        ])
        .unwrap(),
        start_trigger: None,
    };

    assert_eq!(event.actions.len(), 2);
    assert_eq!(event.actions[0].name.as_literal().unwrap(), "Action1");
    assert_eq!(event.actions[1].name.as_literal().unwrap(), "Action2");
}

#[test]
fn test_event_multiple_actions_xml_parsing() {
    let xml_content = include_str!("data/multiple_actions_scenario.xosc");

    let scenario = parse_from_str(xml_content).expect("Should parse successfully");

    // Navigate to the event with multiple actions
    let story = &scenario.storyboard.unwrap().stories[0];
    let act = &story.acts[0];
    let maneuver_group = &act.maneuver_groups[0];
    let maneuver = &maneuver_group.maneuvers[0];
    let event = &maneuver.events[0];

    // Verify the event has multiple actions
    assert_eq!(event.actions.len(), 2, "Event should have 2 actions");
    assert_eq!(event.actions[0].name.as_literal().unwrap(), "SpeedAction");
    assert_eq!(
        event.actions[1].name.as_literal().unwrap(),
        "VisibilityAction"
    );

    // Verify the first action is a speed action
    let StoryActionChoice::PrivateAction(first) = &event.actions[0].action else {
        panic!("First action should be a PrivateAction");
    };
    assert!(
        matches!(
            first.action,
            StoryPrivateActionChoice::LongitudinalAction(_)
        ),
        "First action should be longitudinal"
    );

    // Verify the second action is a visibility action
    let StoryActionChoice::PrivateAction(second) = &event.actions[1].action else {
        panic!("Second action should be a PrivateAction");
    };
    assert!(
        matches!(second.action, StoryPrivateActionChoice::VisibilityAction(_)),
        "Second action should be visibility"
    );
}

#[test]
fn test_event_new_requires_the_actions_it_will_perform() {
    // Event has no Default impl (it used to fabricate a name, a
    // Priority::Overwrite, and a whole StoryAction nobody wrote), and its actions are
    // now a constructor parameter rather than a list the caller fills afterwards. The
    // XSD declares Action with the default minOccurs="1", so there is no valid state in
    // which a freshly built event has none.
    let event = Event::new(
        "MyEvent",
        Priority::Override,
        vec![StoryAction::private(
            "Action1",
            StoryPrivateAction::visibility(VisibilityAction::new(true, true, true)),
        )],
    )
    .expect("one action satisfies the schema minimum");
    assert_eq!(event.actions.len(), 1);
    assert_eq!(event.actions[0].name.as_literal().unwrap(), "Action1");
}

#[test]
fn test_event_new_refuses_an_event_with_no_action() {
    assert!(Event::new("MyEvent", Priority::Override, Vec::new()).is_err());
}

#[test]
fn test_event_serialization_with_multiple_actions() {
    let event = Event {
        name: Value::literal("TestEvent".to_string()),
        maximum_execution_count: None,
        priority: Value::Literal(Priority::Parallel),
        actions: MinVec::new(vec![
            StoryAction::private(
                "FirstAction",
                StoryPrivateAction::visibility(VisibilityAction::new(true, true, true)),
            ),
            StoryAction::private(
                "SecondAction",
                StoryPrivateAction::visibility(VisibilityAction::new(true, true, true)),
            ),
        ])
        .unwrap(),
        start_trigger: None,
    };

    let serialized = quick_xml::se::to_string(&event).expect("Serialization should succeed");

    // Check that both actions are serialized
    assert!(
        serialized.contains("FirstAction"),
        "Should contain first action name"
    );
    assert!(
        serialized.contains("SecondAction"),
        "Should contain second action name"
    );

    // Check that Action appears twice (once for each action)
    let action_count = serialized.matches("<Action").count();
    assert_eq!(
        action_count, 2,
        "Should have 2 Action elements in serialized XML"
    );
}
