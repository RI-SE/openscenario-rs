//! Demonstration of ByValueCondition types
//!
//! This example shows how to create and use all the different
//! ByValueCondition types in OpenSCENARIO-rs.

use openscenario_rs::types::basic::Value;
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
        rule: Value::Literal(Rule::GreaterThan),
    };
    println!("1. Simulation Time Condition: trigger when time > 15.5s");

    // 2. ParameterCondition - trigger when parameter equals a value
    let param_condition = ParameterCondition {
        parameter_ref: OSString::literal("vehicleSpeed".to_string()),
        rule: Value::Literal(Rule::GreaterThan),
        value: OSString::literal("50".to_string()),
    };
    println!("2. Parameter Condition: trigger when vehicleSpeed > 50");

    // 3. TimeOfDayCondition - trigger at a specific time of day
    let time_of_day_condition = TimeOfDayCondition::new(
        chrono::DateTime::parse_from_rfc3339("2024-01-01T12:00:00Z")
            .unwrap()
            .with_timezone(&chrono::Utc),
        Rule::GreaterThan,
    );
    println!("3. Time of Day Condition: trigger after 2024-01-01T12:00:00Z");

    // 4. StoryboardElementStateCondition - trigger when story element completes
    let storyboard_condition = StoryboardElementStateCondition {
        storyboard_element_ref: OSString::literal("overtakeManeuver".to_string()),
        state: Value::Literal(StoryboardElementState::CompleteState),
        storyboard_element_type: Value::Literal(StoryboardElementType::Maneuver),
    };
    println!("4. Storyboard Element State: trigger when overtakeManeuver completes");

    // 5. UserDefinedValueCondition - custom condition logic
    let user_defined_condition = UserDefinedValueCondition {
        name: OSString::literal("weatherCondition".to_string()),
        rule: Value::Literal(Rule::EqualTo),
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
        rule: Value::Literal(Rule::LessThan),
        value: OSString::literal("10".to_string()),
    };
    println!("8. Variable Condition: trigger when fuelLevel < 10");

    // `ByValueCondition` is an `xsd:choice`: exactly one branch is present per instance,
    // never several at once. Each condition built above becomes its own
    // `ByValueCondition`, selecting the matching branch constructor.
    let _parameter = ByValueCondition::parameter(param_condition);
    let _time_of_day = ByValueCondition::time_of_day(time_of_day_condition);
    let _simulation_time = ByValueCondition::simulation_time(sim_time_condition);
    let _storyboard_element_state =
        ByValueCondition::storyboard_element_state(storyboard_condition);
    let _user_defined_value = ByValueCondition::user_defined_value(user_defined_condition);
    let _traffic_signal = ByValueCondition::traffic_signal(traffic_signal_condition);
    let _traffic_signal_controller =
        ByValueCondition::traffic_signal_controller(traffic_controller_condition);
    let _variable = ByValueCondition::variable(variable_condition);

    println!("\nBuilt all 8 ByValueCondition branches, one condition at a time.");
    println!("This demonstrates the complete implementation of ByValueCondition.");

    // Show parameter reference usage
    let param_ref_condition = ParameterCondition {
        parameter_ref: OSString::parameter("dynamicParam".to_string()),
        rule: Value::Literal(Rule::EqualTo),
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
            rule: Value::Literal(rule),
            value: OSString::literal("testValue".to_string()),
        };
        println!("- {}: {:?}", name, condition.rule);
    }

    println!("\nDemo completed successfully!");
}
