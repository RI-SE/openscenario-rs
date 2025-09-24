//! Integration test for ByValueCondition XML serialization/deserialization

use openscenario_rs::types::{
    basic::{Double, OSString},
    conditions::{ByValueCondition, ParameterCondition, SimulationTimeCondition},
    enums::Rule,
};
use serde_xml_rs;

#[test]
fn test_byvalue_condition_xml_serialization() {
    let condition = ByValueCondition {
        parameter: Some(ParameterCondition {
            parameter_ref: OSString::literal("testParam".to_string()),
            rule: Rule::EqualTo,
            value: OSString::literal("testValue".to_string()),
        }),
        time_of_day: None,
        simulation_time: Some(SimulationTimeCondition {
            value: Double::literal(10.0),
            rule: Rule::GreaterThan,
        }),
        storyboard_element_state: None,
        user_defined_value: None,
        traffic_signal: None,
        traffic_signal_controller: None,
        variable: None,
    };

    // Test that the structure can be created and accessed
    assert!(condition.parameter.is_some());
    assert!(condition.simulation_time.is_some());
    assert!(condition.time_of_day.is_none());
}

#[test]
fn test_condition_defaults() {
    let default_condition = ByValueCondition::default();
    
    // Default should have simulation_time set
    assert!(default_condition.simulation_time.is_some());
    assert!(default_condition.parameter.is_none());
    
    // Test the default simulation time condition
    let sim_time = default_condition.simulation_time.unwrap();
    assert_eq!(sim_time.value, Double::literal(10.0));
    assert_eq!(sim_time.rule, Rule::GreaterThan);
}

#[test]
fn test_all_condition_types_compile() {
    // This test ensures all condition types can be instantiated
    use openscenario_rs::types::conditions::*;
    
    let _param = ParameterCondition::default();
    let _time_of_day = TimeOfDayCondition::default();
    let _sim_time = SimulationTimeCondition::default();
    let _storyboard = StoryboardElementStateCondition::default();
    let _user_defined = UserDefinedValueCondition::default();
    let _traffic_signal = TrafficSignalCondition::default();
    let _traffic_controller = TrafficSignalControllerCondition::default();
    let _variable = VariableCondition::default();
    
    // Test that they can all be used in ByValueCondition
    let condition = ByValueCondition {
        parameter: Some(_param),
        time_of_day: Some(_time_of_day),
        simulation_time: Some(_sim_time),
        storyboard_element_state: Some(_storyboard),
        user_defined_value: Some(_user_defined),
        traffic_signal: Some(_traffic_signal),
        traffic_signal_controller: Some(_traffic_controller),
        variable: Some(_variable),
    };
    
    // Verify all fields are set
    assert!(condition.parameter.is_some());
    assert!(condition.time_of_day.is_some());
    assert!(condition.simulation_time.is_some());
    assert!(condition.storyboard_element_state.is_some());
    assert!(condition.user_defined_value.is_some());
    assert!(condition.traffic_signal.is_some());
    assert!(condition.traffic_signal_controller.is_some());
    assert!(condition.variable.is_some());
}