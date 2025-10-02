//! Demonstration of ByValueCondition types
//!
//! This example shows how to create and use all the different
//! ByValueCondition types in OpenSCENARIO-rs.

use openscenario_rs::types::{
    basic::{Double, OSString},
    conditions::{
        ByValueCondition, ParameterCondition, SimulationTimeCondition,
        StoryboardElementStateCondition, TimeOfDayCondition, TrafficSignalCondition,
        TrafficSignalControllerCondition, UserDefinedValueCondition, VariableCondition,
    },
    enums::{Rule, StoryboardElementState, StoryboardElementType},
};

fn main() {
    println!("OpenSCENARIO ByValueCondition Demo");
    println!("==================================");

    // 1. SimulationTimeCondition - trigger when simulation time > 15.5 seconds
    let sim_time_condition = SimulationTimeCondition {
        value: Double::literal(15.5),
        rule: Rule::GreaterThan,
    };
    println!("1. Simulation Time Condition: trigger when time > 15.5s");

    // 2. ParameterCondition - trigger when parameter equals a value
    let param_condition = ParameterCondition {
        parameter_ref: OSString::literal("vehicleSpeed".to_string()),
        rule: Rule::GreaterThan,
        value: OSString::literal("50".to_string()),
    };
    println!("2. Parameter Condition: trigger when vehicleSpeed > 50");

    // 3. TimeOfDayCondition - trigger at specific time of day
    let time_of_day_condition = TimeOfDayCondition::default();
    println!("3. Time of Day Condition: trigger at specific time");

    // 4. StoryboardElementStateCondition - trigger when story element completes
    let storyboard_condition = StoryboardElementStateCondition {
        storyboard_element_ref: OSString::literal("overtakeManeuver".to_string()),
        state: StoryboardElementState::CompleteState,
        storyboard_element_type: StoryboardElementType::Maneuver,
    };
    println!("4. Storyboard Element State: trigger when overtakeManeuver completes");

    // 5. UserDefinedValueCondition - custom condition logic
    let user_defined_condition = UserDefinedValueCondition {
        name: OSString::literal("weatherCondition".to_string()),
        rule: Rule::EqualTo,
        value: OSString::literal("rainy".to_string()),
    };
    println!("5. User Defined Value: trigger when weatherCondition equals 'rainy'");

    // 6. TrafficSignalCondition - trigger on traffic signal state
    let traffic_signal_condition = TrafficSignalCondition {
        name: OSString::literal("intersection_main".to_string()),
        state: OSString::literal("green".to_string()),
    };
    println!("6. Traffic Signal: trigger when intersection_main is green");

    // 7. TrafficSignalControllerCondition - trigger on controller phase
    let traffic_controller_condition = TrafficSignalControllerCondition {
        traffic_signal_controller_ref: OSString::literal("controller_1".to_string()),
        phase: OSString::literal("phase_2".to_string()),
    };
    println!("7. Traffic Signal Controller: trigger when controller_1 enters phase_2");

    // 8. VariableCondition - trigger on variable value
    let variable_condition = VariableCondition {
        variable_ref: OSString::literal("fuelLevel".to_string()),
        rule: Rule::LessThan,
        value: OSString::literal("10".to_string()),
    };
    println!("8. Variable Condition: trigger when fuelLevel < 10");

    // Create a comprehensive ByValueCondition with multiple conditions
    let comprehensive_condition = ByValueCondition {
        parameter_condition: Some(param_condition),
        time_of_day_condition: Some(time_of_day_condition),
        simulation_time_condition: Some(sim_time_condition),
        storyboard_element_state_condition: Some(storyboard_condition),
        user_defined_value_condition: Some(user_defined_condition),
        traffic_signal_condition: Some(traffic_signal_condition),
        traffic_signal_controller_condition: Some(traffic_controller_condition),
        variable_condition: Some(variable_condition),
    };

    println!("\nComprehensive ByValueCondition created with all 8 condition types!");
    println!("This demonstrates the complete implementation of ByValueCondition.");

    // Show parameter reference usage
    let param_ref_condition = ParameterCondition {
        parameter_ref: OSString::parameter("dynamicParam".to_string()),
        rule: Rule::EqualTo,
        value: OSString::parameter("dynamicValue".to_string()),
    };
    println!("\nParameter Reference Example:");
    println!(
        "- Parameter reference: ${{{}}}",
        match param_ref_condition.parameter_ref {
            OSString::Parameter(ref name) => name,
            _ => "literal",
        }
    );

    // Demonstrate different rule types
    println!("\nAvailable Rule Types:");
    let rules = vec![
        ("EqualTo", Rule::EqualTo),
        ("GreaterThan", Rule::GreaterThan),
        ("LessThan", Rule::LessThan),
        ("GreaterOrEqual", Rule::GreaterOrEqual),
        ("LessOrEqual", Rule::LessOrEqual),
        ("NotEqualTo", Rule::NotEqualTo),
    ];

    for (name, rule) in rules {
        let condition = ParameterCondition {
            parameter_ref: OSString::literal("testParam".to_string()),
            rule,
            value: OSString::literal("testValue".to_string()),
        };
        println!("- {}: {:?}", name, condition.rule);
    }

    println!("\nDemo completed successfully!");
}
