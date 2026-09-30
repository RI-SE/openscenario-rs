//! Wire form of every `ByValueCondition` branch (XSD:837-847).
//!
//! Each row builds the condition with its public constructor and the `ByValueCondition`
//! branch wrapper, then asserts that the schema-valid XML parses to exactly that value and
//! that the value serializes back to exactly that XML. A wrong element or attribute name, a
//! constructor that stores the wrong field, or a wrapper that picks the wrong branch fails
//! the row.

use chrono::{TimeZone, Utc};
use openscenario_rs::types::conditions::{
    ByValueCondition, ParameterCondition, SimulationTimeCondition, StoryboardElementStateCondition,
    TimeOfDayCondition, TrafficSignalCondition, TrafficSignalControllerCondition,
    UserDefinedValueCondition, VariableCondition,
};
use openscenario_rs::types::enums::{Rule, StoryboardElementState, StoryboardElementType};

fn assert_wire(xml: &str, expected: &ByValueCondition) {
    let parsed: ByValueCondition =
        quick_xml::de::from_str(xml).unwrap_or_else(|e| panic!("failed to parse {xml}: {e}"));
    assert_eq!(&parsed, expected, "parsed value differs for {xml}");
    let emitted = quick_xml::se::to_string(expected).expect("serialize ByValueCondition");
    assert_eq!(emitted, xml);
}

#[test]
fn by_value_condition_branches_match_xsd_wire_form() {
    // XSD:1629-1633
    assert_wire(
        r#"<ByValueCondition><ParameterCondition parameterRef="speedLimit" rule="equalTo" value="30"/></ByValueCondition>"#,
        &ByValueCondition::parameter(ParameterCondition::new("speedLimit", Rule::EqualTo, "30")),
    );
    // XSD:2169-2172
    assert_wire(
        r#"<ByValueCondition><TimeOfDayCondition dateTime="2024-06-01T12:30:00Z" rule="greaterThan"/></ByValueCondition>"#,
        &ByValueCondition::time_of_day(TimeOfDayCondition::new(
            Utc.with_ymd_and_hms(2024, 6, 1, 12, 30, 0).unwrap(),
            Rule::GreaterThan,
        )),
    );
    // XSD:2039-2042
    assert_wire(
        r#"<ByValueCondition><SimulationTimeCondition value="5.5" rule="lessThan"/></ByValueCondition>"#,
        &ByValueCondition::simulation_time(SimulationTimeCondition::new(5.5, Rule::LessThan)),
    );
    // XSD:2119-2123
    assert_wire(
        r#"<ByValueCondition><StoryboardElementStateCondition storyboardElementRef="CutInAct" state="runningState" storyboardElementType="act"/></ByValueCondition>"#,
        &ByValueCondition::storyboard_element_state(StoryboardElementStateCondition::new(
            "CutInAct",
            StoryboardElementState::RunningState,
            StoryboardElementType::Act,
        )),
    );
    // XSD:2437-2441
    assert_wire(
        r#"<ByValueCondition><UserDefinedValueCondition name="rainIntensity" rule="greaterOrEqual" value="0.8"/></ByValueCondition>"#,
        &ByValueCondition::user_defined_value(UserDefinedValueCondition::new(
            "rainIntensity",
            Rule::GreaterOrEqual,
            "0.8",
        )),
    );
    // XSD:2254-2257
    assert_wire(
        r#"<ByValueCondition><TrafficSignalCondition name="Signal1" state="red"/></ByValueCondition>"#,
        &ByValueCondition::traffic_signal(TrafficSignalCondition::new("Signal1", "red")),
    );
    // XSD:2275-2278
    assert_wire(
        r#"<ByValueCondition><TrafficSignalControllerCondition trafficSignalControllerRef="Controller1" phase="stop"/></ByValueCondition>"#,
        &ByValueCondition::traffic_signal_controller(TrafficSignalControllerCondition::new(
            "Controller1",
            "stop",
        )),
    );
    // XSD:2466-2470
    assert_wire(
        r#"<ByValueCondition><VariableCondition variableRef="lapCount" rule="notEqualTo" value="3"/></ByValueCondition>"#,
        &ByValueCondition::variable(VariableCondition::new("lapCount", Rule::NotEqualTo, "3")),
    );
}
