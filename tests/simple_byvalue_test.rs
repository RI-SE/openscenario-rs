//! Simple test to verify ByValueCondition compiles and works

use openscenario_rs::types::{
    basic::{Double, OSString},
    conditions::{ByValueCondition, SimulationTimeCondition, ParameterCondition},
    enums::Rule,
};

#[test]
fn test_byvalue_condition_basic() {
    // Test that we can create a basic ByValueCondition with simulation time
    let sim_time_condition = SimulationTimeCondition {
        value: Double::literal(10.0),
        rule: Rule::GreaterThan,
    };

    let condition = ByValueCondition {
        parameter: None,
        time_of_day: None,
        simulation_time: Some(sim_time_condition),
        storyboard_element_state: None,
        user_defined_value: None,
        traffic_signal: None,
        traffic_signal_controller: None,
        variable: None,
    };

    assert!(condition.simulation_time.is_some());
    assert!(condition.parameter.is_none());
}

#[test]
fn test_byvalue_condition_parameter() {
    // Test that we can create a ByValueCondition with parameter condition
    let param_condition = ParameterCondition {
        parameter_ref: OSString::literal("testParam".to_string()),
        rule: Rule::EqualTo,
        value: OSString::literal("testValue".to_string()),
    };

    let condition = ByValueCondition {
        parameter: Some(param_condition),
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
}

#[test]
fn test_byvalue_condition_default() {
    let condition = ByValueCondition::default();
    
    // Default should have simulation_time set
    assert!(condition.simulation_time.is_some());
    assert!(condition.parameter.is_none());
}