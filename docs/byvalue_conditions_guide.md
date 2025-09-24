# ByValueCondition Implementation Guide

This document describes the complete implementation of ByValueCondition types in OpenSCENARIO-rs, covering all 7 condition types from the OpenSCENARIO specification.

## Overview

The `ByValueCondition` struct supports all value-based conditions that can trigger scenario events based on parameter values, time conditions, and system states. This implementation provides 100% coverage of the ByValueCondition types defined in the OpenSCENARIO XSD schema.

## Implemented Condition Types

### 1. SimulationTimeCondition
Triggers based on simulation time elapsed.

```rust
use openscenario_rs::types::{
    basic::Double,
    conditions::SimulationTimeCondition,
    enums::Rule,
};

let condition = SimulationTimeCondition {
    value: Double::literal(15.5),
    rule: Rule::GreaterThan,
};
```

**XML Representation:**
```xml
<SimulationTimeCondition value="15.5" rule="greaterThan"/>
```

### 2. ParameterCondition
Triggers based on scenario parameter values.

```rust
use openscenario_rs::types::{
    basic::OSString,
    conditions::ParameterCondition,
    enums::Rule,
};

let condition = ParameterCondition {
    parameter_ref: OSString::literal("vehicleSpeed".to_string()),
    rule: Rule::GreaterThan,
    value: OSString::literal("50".to_string()),
};
```

**XML Representation:**
```xml
<ParameterCondition parameterRef="vehicleSpeed" rule="greaterThan" value="50"/>
```

### 3. TimeOfDayCondition
Triggers based on absolute time of day.

```rust
use openscenario_rs::types::{
    basic::DateTime,
    conditions::TimeOfDayCondition,
    enums::Rule,
};

let condition = TimeOfDayCondition {
    date_time: DateTime::literal(chrono::Utc::now()),
    rule: Rule::GreaterThan,
};
```

**XML Representation:**
```xml
<TimeOfDayCondition dateTime="2023-06-15T14:30:00Z" rule="greaterThan"/>
```

### 4. StoryboardElementStateCondition
Triggers based on storyboard element states (acts, events, maneuvers, etc.).

```rust
use openscenario_rs::types::{
    basic::OSString,
    conditions::StoryboardElementStateCondition,
    enums::{StoryboardElementState, StoryboardElementType},
};

let condition = StoryboardElementStateCondition {
    storyboard_element_ref: OSString::literal("overtakeManeuver".to_string()),
    state: StoryboardElementState::CompleteState,
    storyboard_element_type: StoryboardElementType::Maneuver,
};
```

**XML Representation:**
```xml
<StoryboardElementStateCondition 
    storyboardElementRef="overtakeManeuver" 
    statee="completeState" 
    storyboardElementType="maneuver"/>
```

### 5. UserDefinedValueCondition
Triggers based on custom user-defined conditions.

```rust
use openscenario_rs::types::{
    basic::OSString,
    conditions::UserDefinedValueCondition,
    enums::Rule,
};

let condition = UserDefinedValueCondition {
    name: OSString::literal("weatherCondition".to_string()),
    rule: Rule::EqualTo,
    value: OSString::literal("rainy".to_string()),
};
```

**XML Representation:**
```xml
<UserDefinedValueCondition name="weatherCondition" rule="equalTo" value="rainy"/>
```

### 6. TrafficSignalCondition
Triggers based on traffic signal states.

```rust
use openscenario_rs::types::{
    basic::OSString,
    conditions::TrafficSignalCondition,
};

let condition = TrafficSignalCondition {
    name: OSString::literal("intersection_main".to_string()),
    state: OSString::literal("green".to_string()),
};
```

**XML Representation:**
```xml
<TrafficSignalCondition name="intersection_main" statee="green"/>
```

### 7. TrafficSignalControllerCondition
Triggers based on traffic signal controller phases.

```rust
use openscenario_rs::types::{
    basic::OSString,
    conditions::TrafficSignalControllerCondition,
};

let condition = TrafficSignalControllerCondition {
    traffic_signal_controller_ref: OSString::literal("controller_1".to_string()),
    phase: OSString::literal("phase_2".to_string()),
};
```

**XML Representation:**
```xml
<TrafficSignalControllerCondition 
    trafficSignalControllerRef="controller_1" 
    phase="phase_2"/>
```

### 8. VariableCondition
Triggers based on scenario variable values.

```rust
use openscenario_rs::types::{
    basic::OSString,
    conditions::VariableCondition,
    enums::Rule,
};

let condition = VariableCondition {
    variable_ref: OSString::literal("fuelLevel".to_string()),
    rule: Rule::LessThan,
    value: OSString::literal("10".to_string()),
};
```

**XML Representation:**
```xml
<VariableCondition variableRef="fuelLevel" rule="lessThan" value="10"/>
```

## ByValueCondition Container

The `ByValueCondition` struct acts as a container for all these condition types, following the XSD choice pattern where exactly one condition type should be specified:

```rust
use openscenario_rs::types::conditions::ByValueCondition;

let condition = ByValueCondition {
    parameter: Some(ParameterCondition { /* ... */ }),
    time_of_day: None,
    simulation_time: None,
    storyboard_element_state: None,
    user_defined_value: None,
    traffic_signal: None,
    traffic_signal_controller: None,
    variable: None,
};
```

## Parameter References

All string-based fields support parameter references using the `${parameterName}` syntax:

```rust
let condition = ParameterCondition {
    parameter_ref: OSString::parameter("dynamicParam".to_string()),
    rule: Rule::EqualTo,
    value: OSString::parameter("dynamicValue".to_string()),
};
```

## Rule Types

All conditions that support comparison rules use the `Rule` enum:

- `EqualTo` - Exact equality
- `GreaterThan` - Strictly greater than
- `LessThan` - Strictly less than  
- `GreaterOrEqual` - Greater than or equal to
- `LessOrEqual` - Less than or equal to
- `NotEqualTo` - Not equal to

## Storyboard Element States

The `StoryboardElementStateCondition` supports all element states:

- `CompleteState` - Element has completed
- `EndTransition` - Element is ending
- `RunningState` - Element is currently running
- `SkipTransition` - Element is being skipped
- `StandbyState` - Element is on standby
- `StartTransition` - Element is starting
- `StopTransition` - Element is stopping

## Storyboard Element Types

The `StoryboardElementStateCondition` supports all element types:

- `Act` - Story act
- `Action` - Individual action
- `Event` - Story event
- `Maneuver` - Vehicle maneuver
- `ManeuverGroup` - Group of maneuvers
- `Story` - Complete story

## Testing

Comprehensive tests are provided in:
- `tests/byvalue_conditions_test.rs` - Unit tests for all condition types
- `tests/byvalue_integration_test.rs` - Integration tests for XML serialization
- `examples/byvalue_conditions_demo.rs` - Usage examples

Run tests with:
```bash
cargo test byvalue
```

Run the demo with:
```bash
cargo run --example byvalue_conditions_demo
```

## Implementation Notes

1. **Type Safety**: All conditions use strongly-typed enums and structs
2. **Parameter Support**: Full support for `${parameter}` references
3. **XML Compliance**: Serde annotations ensure correct XML serialization
4. **Default Values**: All types provide sensible defaults for testing
5. **Feature Gates**: DateTime support requires the `chrono` feature (enabled by default)

## Migration from Previous Versions

If upgrading from a version that only supported `SimulationTimeCondition`, update your code to use the new condition types as needed. The existing `SimulationTimeCondition` usage remains unchanged.

## Future Enhancements

- Expression evaluation for parameter references
- Validation of parameter types and ranges
- Runtime condition evaluation framework
- Builder pattern for complex condition construction