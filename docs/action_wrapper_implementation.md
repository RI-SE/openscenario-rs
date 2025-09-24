# Action Wrapper Types Implementation

## Overview

This document describes the implementation of missing Action wrapper types for the OpenSCENARIO-rs project. These wrapper types organize individual action implementations according to the OpenSCENARIO XSD schema structure.

## Implemented Types

### 1. Core Action Wrapper Types

#### `CoreAction` (Main Action Wrapper)
```rust
pub enum CoreAction {
    GlobalAction(CoreGlobalAction),
    UserDefinedAction(CoreUserDefinedAction),
    PrivateAction(CorePrivateAction),
}
```
- Matches XSD `Action` type with choice of GlobalAction, UserDefinedAction, or PrivateAction
- Provides the main entry point for all action types

#### `CoreGlobalAction` (Global Action Wrapper)
```rust
pub enum CoreGlobalAction {
    EnvironmentAction(CoreEnvironmentAction),
    EntityAction(EntityAction),
    InfrastructureAction(InfrastructureAction),
    SetMonitorAction(SetMonitorAction),
    ParameterAction(ParameterAction), // deprecated
    TrafficAction(TrafficAction),
    VariableAction(VariableAction),
}
```
- Wraps all global-level actions that affect the entire scenario
- Includes entity management, traffic control, and infrastructure actions

#### `CorePrivateAction` (Private Action Wrapper)
```rust
pub enum CorePrivateAction {
    LongitudinalAction(LongitudinalAction),
    LateralAction(LateralAction),
    VisibilityAction(VisibilityAction),
    SynchronizeAction(SynchronizeAction),
    ActivateControllerAction(ActivateControllerAction),
    ControllerAction(ControllerAction),
    TeleportAction(TeleportAction),
    RoutingAction(RoutingAction),
    AppearanceAction(AppearanceAction),
    TrailerAction(TrailerAction),
}
```
- Wraps all entity-specific actions
- Includes movement, appearance, and control actions

### 2. Specialized Wrapper Types

#### `EntityAction` (Entity Management)
```rust
pub struct EntityAction {
    pub entity_ref: OSString,
    pub action: EntityActionChoice,
}

pub enum EntityActionChoice {
    AddEntityAction(AddEntityAction),
    DeleteEntityAction(DeleteEntityAction),
}
```
- Manages entity lifecycle (add/delete)
- Includes entity reference for targeting

#### `TrafficAction` (Traffic Management)
```rust
pub struct TrafficAction {
    pub traffic_name: Option<OSString>,
    pub action: TrafficActionChoice,
}

pub enum TrafficActionChoice {
    TrafficSourceAction(TrafficSourceAction),
    TrafficSinkAction(TrafficSinkAction),
    TrafficSwarmAction(TrafficSwarmAction),
    TrafficAreaAction(TrafficAreaAction),
    TrafficStopAction(TrafficStopAction),
}
```
- Manages traffic flow and behavior
- Optional traffic name for identification

#### `InfrastructureAction` (Infrastructure Control)
```rust
pub struct InfrastructureAction {
    pub traffic_signal_action: TrafficSignalAction,
}
```
- Controls infrastructure elements like traffic signals
- Currently focused on traffic signal management

### 3. Individual Action Types

#### Entity Actions
- **`AddEntityAction`**: Adds new entities to the scenario with position
- **`DeleteEntityAction`**: Removes entities from the scenario (empty per XSD)

#### Override Actions (XSD Compliant Names)
- **`OverrideBrakeAction`**: Override brake control with force/input
- **`OverrideThrottleAction`**: Override throttle with value and rate limits
- **`OverrideSteeringWheelAction`**: Override steering with torque limits
- **`OverrideGearAction`**: Override gear selection (manual/automatic)
- **`OverrideParkingBrakeAction`**: Override parking brake control
- **`OverrideClutchAction`**: Override clutch control with rate limits

### 4. Placeholder Types

The following types are implemented as placeholders for future development:
- `CoreEnvironmentAction`
- `SetMonitorAction`
- `ParameterAction` (deprecated)
- `VariableAction`
- `CoreUserDefinedAction`
- `CustomCommandAction`

## Key Features

### XSD Schema Compliance
- All wrapper types match the OpenSCENARIO XSD schema structure
- Proper XML element naming with `#[serde(rename_all = "PascalCase")]`
- Correct choice/sequence patterns from XSD

### Serialization Support
- Full serde serialization/deserialization support
- JSON and XML compatible (via quick-xml)
- Proper attribute handling with `@` prefix

### Type Safety
- Strong typing prevents invalid action combinations
- Compile-time validation of action structure
- Clear separation between global and private actions

### Default Implementations
- All types implement `Default` trait
- Sensible default values for testing and initialization
- No-panic guarantee for default construction

## Usage Examples

### Creating a Private Action
```rust
let teleport_action = TeleportAction::default();
let private_action = CorePrivateAction::TeleportAction(teleport_action);
let core_action = CoreAction::PrivateAction(private_action);
```

### Creating a Global Action
```rust
let traffic_action = TrafficAction {
    traffic_name: Some(OSString::from("highway_traffic")),
    action: TrafficActionChoice::TrafficSourceAction(TrafficSourceAction::default()),
};
let global_action = CoreGlobalAction::TrafficAction(traffic_action);
let core_action = CoreAction::GlobalAction(global_action);
```

### Creating Override Actions
```rust
let brake_override = OverrideBrakeAction {
    active: Boolean::from(true),
    value: Some(Double::from(0.8)),
    brake_input: None,
};
```

## Testing

### Test Coverage
- Comprehensive test suite in `tests/action_wrapper_types_test.rs`
- Serialization/deserialization tests for all wrapper types
- Default implementation tests
- Individual action type tests

### Demo Application
- Example application in `examples/action_wrappers_demo.rs`
- Demonstrates all wrapper types and their usage
- Shows proper construction patterns

## Integration Notes

### Naming Convention
- Used "Core" prefix to avoid conflicts with existing scenario-specific types
- Maintains compatibility with existing codebase
- Clear distinction between wrapper types and individual actions

### Backward Compatibility
- Existing individual action types remain unchanged
- Legacy action names preserved alongside XSD-compliant names
- No breaking changes to existing APIs

### Future Work
- Consolidate with scenario-specific action types
- Implement placeholder action types (Environment, Monitor, Variable)
- Add validation logic for action constraints
- Enhance XML serialization with proper namespaces

## Impact on Schema Coverage

This implementation significantly improves OpenSCENARIO schema coverage by adding:
- 6 main wrapper types (CoreAction, CoreGlobalAction, etc.)
- 2 entity management actions (Add/Delete)
- 6 XSD-compliant override actions
- Proper hierarchical action organization

The action type coverage should increase from 65% to approximately 75-80% with these additions, bringing the project much closer to full OpenSCENARIO compliance.