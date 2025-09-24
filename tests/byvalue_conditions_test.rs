//! Comprehensive tests for ByValueCondition types
//!
//! Tests all 7 condition types in ByValueCondition:
//! - ParameterCondition
//! - TimeOfDayCondition  
//! - SimulationTimeCondition
//! - StoryboardElementStateCondition
//! - UserDefinedValueCondition
//! - TrafficSignalCondition
//! - TrafficSignalControllerCondition
//! - VariableCondition

use openscenario_rs::types::{
    basic::{Double, OSString},
    conditions::{
        ByValueCondition, ParameterCondition, SimulationTimeCondition,
        StoryboardElementStateCondition, TimeOfDayCondition, TrafficSignalCondition,
        TrafficSignalControllerCondition, UserDefinedValueCondition, VariableCondition,
    },
    enums::{Rule, StoryboardElementState, StoryboardElementType},
};

#[test]
fn test_simulation_time_condition() {
    let condition = SimulationTimeCondition {
        value: Double::literal(15.5),
        rule: Rule::GreaterThan,
    };

    assert_eq!(condition.value, Double::literal(15.5));
    assert_eq!(condition.rule, Rule::GreaterThan);
}

#[test]
fn test_parameter_condition() {
    let condition = ParameterCondition {
        parameter_ref: OSString::literal("testParam".to_string()),
        rule: Rule::EqualTo,
        value: OSString::literal("expectedValue".to_string()),
    };

    assert_eq!(condition.parameter_ref, OSString::literal("testParam".to_string()));
    assert_eq!(condition.rule, Rule::EqualTo);
    assert_eq!(condition.value, OSString::literal("expectedValue".to_string()));
}

#[test]
fn test_time_of_day_condition() {
    let condition = TimeOfDayCondition {
        date_time: openscenario_rs::types::basic::DateTime::literal(chrono::Utc::now()),
        rule: Rule::GreaterThan,
    };

    assert_eq!(condition.rule, Rule::GreaterThan);
}

#[test]
fn test_storyboard_element_state_condition() {
    let condition = StoryboardElementStateCondition {
        storyboard_element_ref: OSString::literal("myEvent".to_string()),
        state: StoryboardElementState::CompleteState,
        storyboard_element_type: StoryboardElementType::Event,
    };

    assert_eq!(condition.storyboard_element_ref, OSString::literal("myEvent".to_string()));
    assert_eq!(condition.state, StoryboardElementState::CompleteState);
    assert_eq!(condition.storyboard_element_type, StoryboardElementType::Event);
}

#[test]
fn test_user_defined_value_condition() {
    let condition = UserDefinedValueCondition {
        name: OSString::literal("customCheck".to_string()),
        rule: Rule::NotEqualTo,
        value: OSString::literal("false".to_string()),
    };

    assert_eq!(condition.name, OSString::literal("customCheck".to_string()));
    assert_eq!(condition.rule, Rule::NotEqualTo);
    assert_eq!(condition.value, OSString::literal("false".to_string()));
}

#[test]
fn test_traffic_signal_condition() {
    let condition = TrafficSignalCondition {
        name: OSString::literal("intersection_signal_1".to_string()),
        state: OSString::literal("red".to_string()),
    };

    assert_eq!(condition.name, OSString::literal("intersection_signal_1".to_string()));
    assert_eq!(condition.state, OSString::literal("red".to_string()));
}

#[test]
fn test_traffic_signal_controller_condition() {
    let condition = TrafficSignalControllerCondition {
        traffic_signal_controller_ref: OSString::literal("controller_main".to_string()),
        phase: OSString::literal("phase_2".to_string()),
    };

    assert_eq!(condition.traffic_signal_controller_ref, OSString::literal("controller_main".to_string()));
    assert_eq!(condition.phase, OSString::literal("phase_2".to_string()));
}

#[test]
fn test_variable_condition() {
    let condition = VariableCondition {
        variable_ref: OSString::literal("speedLimit".to_string()),
        rule: Rule::LessThan,
        value: OSString::literal("50".to_string()),
    };

    assert_eq!(condition.variable_ref, OSString::literal("speedLimit".to_string()));
    assert_eq!(condition.rule, Rule::LessThan);
    assert_eq!(condition.value, OSString::literal("50".to_string()));
}

#[test]
fn test_by_value_condition_with_parameter() {
    let condition = ByValueCondition {
        parameter: Some(ParameterCondition {
            parameter_ref: OSString::literal("testParam".to_string()),
            rule: Rule::EqualTo,
            value: OSString::literal("testValue".to_string()),
        }),
        time_of_day: None,
        simulation_time: None,
        storyboard_element_state: None,
        user_defined_value: None,
        traffic_signal: None,
        traffic_signal_controller: None,
        variable: None,
    };

    assert!(condition.parameter.is_some());
    assert!(condition.simulation_time.is_none());
    assert!(condition.variable.is_none());
}

#[test]
fn test_by_value_condition_with_multiple_conditions() {
    let condition = ByValueCondition {
        parameter: Some(ParameterCondition::default()),
        time_of_day: Some(TimeOfDayCondition::default()),
        simulation_time: Some(SimulationTimeCondition::default()),
        storyboard_element_state: None,
        user_defined_value: None,
        traffic_signal: None,
        traffic_signal_controller: None,
        variable: None,
    };

    assert!(condition.parameter.is_some());
    assert!(condition.time_of_day.is_some());
    assert!(condition.simulation_time.is_some());
    assert!(condition.storyboard_element_state.is_none());
}

#[test]
fn test_default_implementations() {
    // Test that all condition types have working Default implementations
    let _sim_time = SimulationTimeCondition::default();
    let _param = ParameterCondition::default();
    let _time_of_day = TimeOfDayCondition::default();
    let _storyboard = StoryboardElementStateCondition::default();
    let _user_defined = UserDefinedValueCondition::default();
    let _traffic_signal = TrafficSignalCondition::default();
    let _traffic_controller = TrafficSignalControllerCondition::default();
    let _variable = VariableCondition::default();
    let _by_value = ByValueCondition::default();

    // Verify the default ByValueCondition has simulation_time set
    let default_by_value = ByValueCondition::default();
    assert!(default_by_value.simulation_time.is_some());
    assert!(default_by_value.parameter.is_none());
}

#[test]
fn test_condition_serialization_structure() {
    // Test that the serde attributes are correctly applied
    let condition = ByValueCondition {
        parameter: Some(ParameterCondition {
            parameter_ref: OSString::literal("param1".to_string()),
            rule: Rule::EqualTo,
            value: OSString::literal("value1".to_string()),
        }),
        time_of_day: None,
        simulation_time: None,
        storyboard_element_state: None,
        user_defined_value: None,
        traffic_signal: None,
        traffic_signal_controller: None,
        variable: None,
    };

    // This test verifies the structure compiles and can be created
    // Actual XML serialization testing would require more complex setup
    assert!(condition.parameter.is_some());
}

#[test]
fn test_parameter_references() {
    // Test parameter reference handling
    let param_condition = ParameterCondition {
        parameter_ref: OSString::parameter("dynamicParam".to_string()),
        rule: Rule::GreaterThan,
        value: OSString::parameter("dynamicValue".to_string()),
    };

    match param_condition.parameter_ref {
        OSString::Parameter(ref name) => assert_eq!(name, "dynamicParam"),
        _ => panic!("Expected parameter reference"),
    }

    match param_condition.value {
        OSString::Parameter(ref name) => assert_eq!(name, "dynamicValue"),
        _ => panic!("Expected parameter reference"),
    }
}

#[test]
fn test_all_rule_types() {
    // Test that all Rule enum variants work with conditions
    let rules = vec![
        Rule::EqualTo,
        Rule::GreaterThan,
        Rule::LessThan,
        Rule::GreaterOrEqual,
        Rule::LessOrEqual,
        Rule::NotEqualTo,
    ];

    for rule in rules {
        let condition = ParameterCondition {
            parameter_ref: OSString::literal("test".to_string()),
            rule: rule.clone(),
            value: OSString::literal("test".to_string()),
        };
        assert_eq!(condition.rule, rule);
    }
}

#[test]
fn test_storyboard_element_states() {
    // Test all StoryboardElementState variants
    let states = vec![
        StoryboardElementState::CompleteState,
        StoryboardElementState::EndTransition,
        StoryboardElementState::RunningState,
        StoryboardElementState::SkipTransition,
        StoryboardElementState::StandbyState,
        StoryboardElementState::StartTransition,
        StoryboardElementState::StopTransition,
    ];

    for state in states {
        let condition = StoryboardElementStateCondition {
            storyboard_element_ref: OSString::literal("element".to_string()),
            state: state.clone(),
            storyboard_element_type: StoryboardElementType::Event,
        };
        assert_eq!(condition.state, state);
    }
}

#[test]
fn test_storyboard_element_types() {
    // Test all StoryboardElementType variants
    let types = vec![
        StoryboardElementType::Act,
        StoryboardElementType::Action,
        StoryboardElementType::Event,
        StoryboardElementType::Maneuver,
        StoryboardElementType::ManeuverGroup,
        StoryboardElementType::Story,
    ];

    for element_type in types {
        let condition = StoryboardElementStateCondition {
            storyboard_element_ref: OSString::literal("element".to_string()),
            state: StoryboardElementState::RunningState,
            storyboard_element_type: element_type.clone(),
        };
        assert_eq!(condition.storyboard_element_type, element_type);
    }
}