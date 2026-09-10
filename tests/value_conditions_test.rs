//! Simple tests for ByValueCondition implementation
//!
//! This test file validates that all the new ByValueCondition types
//! compile and construct correctly via their required-field constructors.

use openscenario_rs::types::basic::OSString;
use openscenario_rs::types::basic::Value;
use openscenario_rs::types::conditions::*;
use openscenario_rs::types::enums::{Rule, StoryboardElementState, StoryboardElementType};

#[test]
fn test_simulation_time_condition() {
    let condition = SimulationTimeCondition::new(10.0, Rule::GreaterThan);
    assert_eq!(condition.rule, Value::Literal(Rule::GreaterThan));
}

#[test]
fn test_parameter_condition() {
    let condition = ParameterCondition::new("defaultParam", Rule::EqualTo, "defaultValue");
    assert_eq!(condition.rule, Value::Literal(Rule::EqualTo));
}

#[test]
fn test_storyboard_element_state_condition() {
    let condition = StoryboardElementStateCondition::new(
        "defaultElement",
        StoryboardElementState::RunningState,
        StoryboardElementType::Story,
    );
    assert_eq!(
        condition.state,
        Value::Literal(StoryboardElementState::RunningState)
    );
    assert_eq!(
        condition.storyboard_element_type,
        Value::Literal(StoryboardElementType::Story)
    );
}

#[test]
fn test_user_defined_value_condition() {
    let condition =
        UserDefinedValueCondition::new("defaultCondition", Rule::EqualTo, "defaultValue");
    assert_eq!(condition.rule, Value::Literal(Rule::EqualTo));
}

#[test]
fn test_traffic_signal_condition() {
    let condition = TrafficSignalCondition::new("defaultSignal", "green");
    if let OSString::Literal(state) = &condition.state {
        assert_eq!(state, "green");
    } else {
        panic!("Expected literal state");
    }
}

#[test]
fn test_traffic_signal_controller_condition() {
    let condition = TrafficSignalControllerCondition::new("defaultController", "phase1");
    if let OSString::Literal(phase) = &condition.phase {
        assert_eq!(phase, "phase1");
    } else {
        panic!("Expected literal phase");
    }
}

#[test]
fn test_variable_condition() {
    let condition = VariableCondition::new("defaultVariable", Rule::EqualTo, "defaultValue");
    assert_eq!(condition.rule, Value::Literal(Rule::EqualTo));
}

#[test]
fn test_byvalue_condition_simulation_time_branch() {
    let condition =
        ByValueCondition::simulation_time(SimulationTimeCondition::new(10.0, Rule::GreaterThan));

    // Should have simulation time condition on the branch we asked for
    assert!(condition.simulation_time_condition.is_some());

    // All others should be None — `ByValueCondition` is an XSD `xsd:choice`, so exactly one
    // branch is populated.
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
    let parameter_condition = ByValueCondition::parameter(ParameterCondition {
        parameter_ref: OSString::literal("testParam".to_string()),
        rule: Value::Literal(Rule::GreaterThan),
        value: OSString::literal("10".to_string()),
    });
    let variable_condition = ByValueCondition::variable(VariableCondition {
        variable_ref: OSString::literal("testVar".to_string()),
        rule: Value::Literal(Rule::LessThan),
        value: OSString::literal("5".to_string()),
    });

    // Verify each branch is set correctly and the other branches stay empty
    assert!(parameter_condition.parameter_condition.is_some());
    assert!(parameter_condition.variable_condition.is_none());
    assert!(variable_condition.variable_condition.is_some());
    assert!(variable_condition.parameter_condition.is_none());

    if let Some(param_cond) = &parameter_condition.parameter_condition {
        assert_eq!(param_cond.rule, Value::Literal(Rule::GreaterThan));
    }
    if let Some(var_cond) = &variable_condition.variable_condition {
        assert_eq!(var_cond.rule, Value::Literal(Rule::LessThan));
    }
}

#[cfg(feature = "chrono")]
#[test]
fn test_time_of_day_condition_with_chrono() {
    let condition = TimeOfDayCondition::new(chrono::Utc::now(), Rule::GreaterThan);
    assert_eq!(condition.rule, Rule::GreaterThan);
    // Should have a valid DateTime
    use openscenario_rs::types::basic::DateTime;
    if let DateTime::Literal(_) = condition.date_time {
        // Should be a valid datetime
    } else {
        panic!("Expected literal datetime");
    }
}
