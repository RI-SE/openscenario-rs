//! Simple tests for ByValueCondition implementation
//! 
//! This test file validates that all the new ByValueCondition types
//! compile and have correct defaults.

use openscenario_rs::types::conditions::*;
use openscenario_rs::types::basic::OSString;
use openscenario_rs::types::enums::{Rule, StoryboardElementState, StoryboardElementType};

#[test]
fn test_simulation_time_condition() {
    let condition = SimulationTimeCondition::default();
    assert_eq!(condition.rule, Rule::GreaterThan);
}

#[test]
fn test_parameter_condition() {
    let condition = ParameterCondition::default();
    assert_eq!(condition.rule, Rule::EqualTo);
}

#[test]
fn test_storyboard_element_state_condition() {
    let condition = StoryboardElementStateCondition::default();
    assert_eq!(condition.state, StoryboardElementState::RunningState);
    assert_eq!(condition.storyboard_element_type, StoryboardElementType::Story);
}

#[test]
fn test_user_defined_value_condition() {
    let condition = UserDefinedValueCondition::default();
    assert_eq!(condition.rule, Rule::EqualTo);
}

#[test]
fn test_traffic_signal_condition() {
    let condition = TrafficSignalCondition::default();
    // Should have default values set
    if let OSString::Literal(state) = &condition.state {
        assert_eq!(state, "green");
    } else {
        panic!("Expected literal state");
    }
}

#[test]
fn test_traffic_signal_controller_condition() {
    let condition = TrafficSignalControllerCondition::default();
    // Should have default values set
    if let OSString::Literal(phase) = &condition.phase {
        assert_eq!(phase, "phase1");
    } else {
        panic!("Expected literal phase");
    }
}

#[test]
fn test_variable_condition() {
    let condition = VariableCondition::default();
    assert_eq!(condition.rule, Rule::EqualTo);
}

#[test]
fn test_byvalue_condition_default() {
    let condition = ByValueCondition::default();
    
    // Should have simulation time condition by default
    assert!(condition.simulation_time_condition.is_some());
    
    // All others should be None by default
    assert!(condition.parameter_condition.is_none());
    assert!(condition.time_of_day_condition.is_none());
    assert!(condition.storyboard_element_state_condition.is_none());
    assert!(condition.user_defined_value_condition.is_none());
    assert!(condition.traffic_signal_condition.is_none());
    assert!(condition.traffic_signal_controller_condition.is_none());
    assert!(condition.variable_condition.is_none());
}

#[test]
fn test_byvalue_condition_with_specific_conditions() {
    let mut condition = ByValueCondition::default();
    
    // Set a parameter condition
    condition.parameter_condition = Some(ParameterCondition {
        parameter_ref: OSString::literal("testParam".to_string()),
        rule: Rule::GreaterThan,
        value: OSString::literal("10".to_string()),
    });
    
    // Set a variable condition
    condition.variable_condition = Some(VariableCondition {
        variable_ref: OSString::literal("testVar".to_string()),
        rule: Rule::LessThan,
        value: OSString::literal("5".to_string()),
    });
    
    // Verify conditions are set correctly
    assert!(condition.parameter_condition.is_some());
    assert!(condition.variable_condition.is_some());
    
    if let Some(param_cond) = &condition.parameter_condition {
        assert_eq!(param_cond.rule, Rule::GreaterThan);
    }
    
    if let Some(var_cond) = &condition.variable_condition {
        assert_eq!(var_cond.rule, Rule::LessThan);
    }
}

#[cfg(feature = "chrono")]
#[test]
fn test_time_of_day_condition_with_chrono() {
    let condition = TimeOfDayCondition::default();
    assert_eq!(condition.rule, Rule::GreaterThan);
    // Should have a valid DateTime
    use openscenario_rs::types::basic::DateTime;
    if let DateTime::Literal(_) = condition.date_time {
        // Should be a valid datetime
    } else {
        panic!("Expected literal datetime");
    }
}

#[cfg(not(feature = "chrono"))]
#[test]
fn test_time_of_day_condition_without_chrono() {
    let condition = TimeOfDayCondition::default();
    assert_eq!(condition.rule, Rule::GreaterThan);
    // Should have a string datetime
    if let OSString::Literal(date_str) = &condition.date_time {
        assert_eq!(date_str, "2024-01-01T12:00:00Z");
    } else {
        panic!("Expected literal string datetime");
    }
}