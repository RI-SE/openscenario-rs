use openscenario_rs::parser::xml::parse_from_str;
use openscenario_rs::types::basic::Value;
use openscenario_rs::types::enums::Priority;
use openscenario_rs::types::scenario::story::{Event, StoryAction, StoryPrivateAction};

#[test]
fn test_event_multiple_actions_struct() {
    // Test that Event can now hold multiple actions
    let event = Event {
        name: Value::literal("MultiActionEvent".to_string()),
        maximum_execution_count: Some(Value::literal(1)),
        priority: Some(Priority::Override),
        actions: vec![
            StoryAction {
                name: Value::literal("Action1".to_string()),
                private_action: Some(StoryPrivateAction::default()),
            },
            StoryAction {
                name: Value::literal("Action2".to_string()),
                private_action: Some(StoryPrivateAction::default()),
            },
        ],
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
    if let Some(private_action) = &event.actions[0].private_action {
        assert!(
            private_action.longitudinal_action.is_some(),
            "First action should be longitudinal"
        );
    }

    // Verify the second action is a visibility action
    if let Some(private_action) = &event.actions[1].private_action {
        assert!(
            private_action.visibility_action.is_some(),
            "Second action should be visibility"
        );
    }
}

#[test]
fn test_event_default_has_single_action() {
    let event = Event::default();
    assert_eq!(
        event.actions.len(),
        1,
        "Default event should have one action"
    );
    assert_eq!(event.actions[0].name.as_literal().unwrap(), "DefaultAction");
}

#[test]
fn test_event_serialization_with_multiple_actions() {
    let event = Event {
        name: Value::literal("TestEvent".to_string()),
        maximum_execution_count: None,
        priority: Some(Priority::Parallel),
        actions: vec![
            StoryAction {
                name: Value::literal("FirstAction".to_string()),
                private_action: None,
            },
            StoryAction {
                name: Value::literal("SecondAction".to_string()),
                private_action: None,
            },
        ],
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

