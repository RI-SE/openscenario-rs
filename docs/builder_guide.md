# OpenSCENARIO-rs Builder System Guide

The OpenSCENARIO-rs builder system provides a type-safe, fluent API for programmatically constructing OpenSCENARIO documents. This guide covers all aspects of using the builder system effectively.

## Table of Contents

1. [Quick Start](#quick-start)
2. [Core Concepts](#core-concepts)
3. [Type States](#type-states)
4. [Entity Builders](#entity-builders)
5. [Action Builders](#action-builders)
6. [Condition Builders](#condition-builders)
7. [Storyboard Construction](#storyboard-construction)
8. [Parameter Support](#parameter-support)
9. [Catalog Integration](#catalog-integration)
10. [Error Handling](#error-handling)
11. [Best Practices](#best-practices)
12. [Examples](#examples)

## Quick Start

Enable the builder feature in your `Cargo.toml`:

```toml
[dependencies]
openscenario-rs = { version = "0.1", features = ["builder"] }
```

Create a simple scenario:

```rust
use openscenario_rs::ScenarioBuilder;

let scenario = ScenarioBuilder::new()
    .with_header("My First Scenario", "Your Name")
    .with_entities()
        .add_vehicle("ego")
            .car()
            .finish()
    .build()?;
```

## Core Concepts

### Type-Safe Construction

The builder system uses Rust's type system to enforce correct construction order:

- **Compile-time validation**: Invalid operations are caught at compile time
- **State transitions**: Each step unlocks new methods while preventing invalid ones
- **Zero-cost abstractions**: No runtime overhead for type safety

### Fluent API

The builder provides a natural, readable syntax:

```rust
ScenarioBuilder::new()
    .with_header("Highway Test", "Author")
    .add_parameter("speed", ParameterType::Double, "25.0")
    .with_entities()
        .add_vehicle("ego")
            .car()
            .with_dimensions(4.5, 1.8, 1.4)
            .finish()
    .with_storyboard()
        .add_story("main")
            .add_act("acceleration")
                .add_maneuver("speed_up", "ego")
                    .add_speed_action()
                        .to_speed(30.0)
                        .finish()?
                    .finish()
                .finish()
            .finish()
        .finish()
    .build()?
```

## Type States

The builder progresses through several type states that enforce correct construction order:

### State Diagram

```
Empty → HasHeader → HasEntities → Complete
  ↓         ↓            ↓           ↓
 new()  add_param()  add_vehicle()  build()
       with_catalog() add_pedestrian()
       with_road()   with_storyboard()
```

### State Descriptions

- **Empty**: Initial state, must call `with_header()` first
- **HasHeader**: Can add parameters, catalogs, road networks
- **HasEntities**: Can add entities and build storyboard
- **Complete**: Ready to build final OpenSCENARIO document

## Entity Builders

### Vehicle Builder

Create vehicles with different configurations:

```rust
// Basic car
.add_vehicle("ego")
    .car()
    .finish()

// Custom truck
.add_vehicle("truck1")
    .truck()
    .with_dimensions(8.0, 2.5, 3.0)
    .finish()

// Car with custom properties
.add_vehicle("sports_car")
    .car()
    .with_dimensions(4.2, 1.9, 1.3)
    .finish()
```

### Catalog Vehicle Builder

Reference vehicles from catalogs:

```rust
.add_catalog_vehicle("ego")
    .from_catalog("VehicleCatalog")
    .entry_name("PassengerCar")
    .finish()
```

### Pedestrian Builder

Add pedestrian entities:

```rust
.add_pedestrian("pedestrian1")
    .with_mass(75.0)
    .finish()
```

## Action Builders

### Speed Actions

Control vehicle speed:

```rust
.add_speed_action()
    .named("accelerate")
    .to_speed(30.0)  // Absolute speed
    .finish()?

.add_speed_action()
    .named("slow_down")
    .change_by(-5.0)  // Relative speed change
    .finish()?
```

### Teleport Actions

Instantly move entities:

```rust
.add_teleport_action()
    .named("lane_change")
    .to()
        .world_position(100.0, -3.5, 0.0)
    .finish()?

.add_teleport_action()
    .named("highway_entry")
    .to()
        .lane_position("highway", "1", 50.0)
    .finish()?
```

### Lane Change Actions

Perform lane changes:

```rust
.add_lane_change_action()
    .named("overtake")
    .target_lane_offset(-1)  // Move to left lane
    .dynamics()
        .max_lateral_acc(2.0)
        .finish()
    .finish()?
```

## Condition Builders

### Time Conditions

Trigger events based on simulation time:

```rust
.triggered_by()
    .time_condition(5.0)  // After 5 seconds
    .finish()
```

### Speed Conditions

Trigger based on entity speed:

```rust
.triggered_by()
    .speed_condition("ego", 25.0)  // When ego exceeds 25 m/s
    .finish()
```

### Distance Conditions

Trigger based on distance to positions:

```rust
.triggered_by()
    .distance_condition("ego")
        .to_position(world_position(100.0, 0.0, 0.0))
        .closer_than(10.0)
    .finish()
```

### Complex Triggers

Combine multiple conditions:

```rust
.triggered_by()
    .add_condition_group()
        .time_condition()
            .at_time(3.0)
            .finish()?
        .speed_condition()
            .for_entity("ego")
            .speed_above(20.0)
            .finish()?
        .finish_group()
    .build()?
```

## Storyboard Construction

### Stories, Acts, and Maneuvers

Build complex scenario behavior:

```rust
.with_storyboard()
    .add_story("highway_scenario")
        .add_act("initial_phase")
            .add_maneuver("ego_acceleration", "ego")
                .add_speed_action()
                    .to_speed(25.0)
                    .triggered_by()
                        .time_condition(1.0)
                        .finish()
                    .finish()?
                .finish()
            .finish()
        
        .add_act("overtaking_phase")
            .add_maneuver("ego_overtake", "ego")
                .add_lane_change_action()
                    .target_lane_offset(-1)
                    .triggered_by()
                        .speed_condition("target", 20.0)
                        .finish()
                    .finish()?
                .finish()
            .finish()
        .finish()
    .finish()
```

### Event Sequencing

Control when events occur:

```rust
.add_maneuver("complex_behavior", "ego")
    // First action: accelerate after 2 seconds
    .add_speed_action()
        .named("initial_acceleration")
        .to_speed(30.0)
        .triggered_by()
            .time_condition(2.0)
            .finish()
        .finish()?
    
    // Second action: lane change when close to target
    .add_lane_change_action()
        .named("overtake_maneuver")
        .target_lane_offset(-1)
        .triggered_by()
            .distance_condition("ego")
                .to_entity("target")
                .closer_than(20.0)
            .finish()
        .finish()?
    .finish()
```

## Parameter Support

### Adding Parameters

Make scenarios configurable:

```rust
.with_header("Configurable Scenario", "Author")
.add_parameter("initial_speed", ParameterType::Double, "25.0")
.add_parameter("target_lane", ParameterType::String, "1")
.add_parameter("weather_condition", ParameterType::String, "sunny")
```

### Using Parameters

Reference parameters in values:

```rust
.add_speed_action()
    .to_speed_parameter("initial_speed")  // Use ${initial_speed}
    .finish()?
```

### Parameter Validation

Validate parameter constraints:

```rust
.add_parameter_with_constraints("speed", ParameterType::Double, "25.0")
    .range(0.0, 50.0)
    .finish()
```

## Catalog Integration

### Catalog Locations

Specify where to find catalogs:

```rust
.with_catalog_locations()
    .vehicle_catalog("./catalogs/VehicleCatalog.xosc")
    .pedestrian_catalog("./catalogs/PedestrianCatalog.xosc")
    .finish()
```

### Using Catalog Entities

Reference entities from catalogs:

```rust
.add_catalog_vehicle("ego")
    .from_catalog("VehicleCatalog")
    .entry_name("BMW_X5")
    .parameter_assignments()
        .assign("color", "blue")
        .assign("license_plate", "ABC-123")
        .finish()
    .finish()
```

## Error Handling

### Builder Errors

The builder system provides detailed error messages:

```rust
match scenario_result {
    Ok(scenario) => println!("Scenario built successfully"),
    Err(BuilderError::MissingField { field, suggestion }) => {
        println!("Missing {}: {}", field, suggestion);
    }
    Err(BuilderError::ValidationError { message, suggestion }) => {
        println!("Validation failed: {} ({})", message, suggestion);
    }
    Err(e) => println!("Other error: {}", e),
}
```

### Validation

Validate scenarios before building:

```rust
let builder = ScenarioBuilder::new()
    .with_header("Test", "Author")
    .with_entities();

// Validate current state
builder.validate()?;

// Continue building...
let scenario = builder.build()?;
```

## Best Practices

### 1. Use Type States Effectively

Let the type system guide you:

```rust
// Good: Type system prevents invalid operations
let builder = ScenarioBuilder::new();
// builder.add_vehicle("ego");  // Compile error - no header yet

let builder = builder.with_header("Test", "Author");
// builder.build();  // Compile error - no entities yet

let builder = builder.with_entities();
// Now can add vehicles and build
```

### 2. Handle Errors Appropriately

Use `?` operator for clean error propagation:

```rust
fn build_scenario() -> Result<OpenScenario, BuilderError> {
    let scenario = ScenarioBuilder::new()
        .with_header("Test", "Author")
        .with_entities()
            .add_vehicle("ego")
                .car()
                .finish()
        .with_storyboard()
            .add_story("main")
                .add_act("test")
                    .add_maneuver("action", "ego")
                        .add_speed_action()
                            .to_speed(30.0)
                            .finish()?  // Propagate action builder errors
                        .finish()
                    .finish()
                .finish()
            .finish()
        .build()?;  // Propagate scenario builder errors
    
    Ok(scenario)
}
```

### 3. Use Meaningful Names

Choose descriptive names for entities and events:

```rust
.add_vehicle("ego_vehicle")  // Not just "ego"
.add_vehicle("target_vehicle")
.add_vehicle("oncoming_truck")

.add_speed_action()
    .named("highway_acceleration")  // Descriptive event names
    .to_speed(30.0)
    .finish()?
```

### 4. Structure Complex Scenarios

Break complex scenarios into logical phases:

```rust
.with_storyboard()
    .add_story("highway_overtaking_scenario")
        .add_act("setup_phase")
            // Initial positioning and speeds
            .finish()
        
        .add_act("approach_phase")
            // Approach target vehicle
            .finish()
        
        .add_act("overtaking_phase")
            // Execute overtaking maneuver
            .finish()
        
        .add_act("completion_phase")
            // Return to original lane
            .finish()
        .finish()
```

### 5. Use Parameters for Reusability

Make scenarios configurable:

```rust
.add_parameter("ego_initial_speed", ParameterType::Double, "25.0")
.add_parameter("target_speed", ParameterType::Double, "20.0")
.add_parameter("overtaking_speed", ParameterType::Double, "35.0")
.add_parameter("safety_distance", ParameterType::Double, "10.0")
```

## Examples

### Complete Highway Scenario

```rust
use openscenario_rs::{ScenarioBuilder, types::enums::ParameterType};

fn highway_overtaking_scenario() -> Result<OpenScenario, BuilderError> {
    ScenarioBuilder::new()
        .with_header("Highway Overtaking Scenario", "OpenSCENARIO-rs")
        
        // Parameters for configurability
        .add_parameter("ego_speed", ParameterType::Double, "25.0")
        .add_parameter("target_speed", ParameterType::Double, "20.0")
        .add_parameter("overtake_speed", ParameterType::Double, "35.0")
        
        // Road network
        .with_road_file("highway_3_lanes.xodr")
        
        // Entities
        .with_entities()
            .add_vehicle("ego")
                .car()
                .with_dimensions(4.5, 1.8, 1.4)
                .finish()
            
            .add_vehicle("target")
                .car()
                .finish()
            
            .add_vehicle("oncoming")
                .truck()
                .finish()
        
        // Scenario behavior
        .with_storyboard()
            .add_story("overtaking_story")
                .add_act("initial_driving")
                    .add_maneuver("ego_cruise", "ego")
                        .add_speed_action()
                            .named("initial_cruise")
                            .to_speed_parameter("ego_speed")
                            .triggered_by()
                                .time_condition(1.0)
                                .finish()
                            .finish()?
                        .finish()
                    
                    .add_maneuver("target_cruise", "target")
                        .add_speed_action()
                            .named("target_cruise")
                            .to_speed_parameter("target_speed")
                            .triggered_by()
                                .time_condition(0.5)
                                .finish()
                            .finish()?
                        .finish()
                    .finish()
                
                .add_act("overtaking")
                    .add_maneuver("ego_overtake", "ego")
                        .add_speed_action()
                            .named("accelerate_for_overtake")
                            .to_speed_parameter("overtake_speed")
                            .triggered_by()
                                .distance_condition("ego")
                                    .to_entity("target")
                                    .closer_than(30.0)
                                .finish()
                            .finish()?
                        
                        .add_lane_change_action()
                            .named("move_to_left_lane")
                            .target_lane_offset(-1)
                            .triggered_by()
                                .speed_condition("ego", 30.0)
                                .finish()
                            .finish()?
                        
                        .add_lane_change_action()
                            .named("return_to_right_lane")
                            .target_lane_offset(1)
                            .triggered_by()
                                .distance_condition("ego")
                                    .to_entity("target")
                                    .farther_than(50.0)
                                .finish()
                            .finish()?
                        .finish()
                    .finish()
                .finish()
            .finish()
        .build()
}
```

### Urban Intersection Scenario

```rust
fn urban_intersection_scenario() -> Result<OpenScenario, BuilderError> {
    ScenarioBuilder::new()
        .with_header("Urban Intersection Scenario", "Traffic Engineer")
        
        .add_parameter("approach_speed", ParameterType::Double, "15.0")
        .add_parameter("stop_distance", ParameterType::Double, "5.0")
        
        .with_road_file("urban_intersection.xodr")
        
        .with_entities()
            .add_vehicle("ego")
                .car()
                .finish()
            
            .add_pedestrian("pedestrian1")
                .with_mass(75.0)
                .finish()
            
            .add_vehicle("cross_traffic")
                .car()
                .finish()
        
        .with_storyboard()
            .add_story("intersection_approach")
                .add_act("approach_phase")
                    .add_maneuver("ego_approach", "ego")
                        .add_speed_action()
                            .named("approach_intersection")
                            .to_speed_parameter("approach_speed")
                            .triggered_by()
                                .time_condition(0.5)
                                .finish()
                            .finish()?
                        
                        .add_speed_action()
                            .named("stop_for_pedestrian")
                            .to_speed(0.0)
                            .triggered_by()
                                .distance_condition("ego")
                                    .to_entity("pedestrian1")
                                    .closer_than_parameter("stop_distance")
                                .finish()
                            .finish()?
                        .finish()
                    .finish()
                
                .add_act("crossing_phase")
                    .add_maneuver("pedestrian_cross", "pedestrian1")
                        .add_teleport_action()
                            .named("cross_street")
                            .to()
                                .world_position(50.0, 10.0, 0.0)
                            .triggered_by()
                                .speed_condition("ego", 0.1)  // When ego stops
                                .finish()
                            .finish()?
                        .finish()
                    
                    .add_maneuver("ego_proceed", "ego")
                        .add_speed_action()
                            .named("proceed_through_intersection")
                            .to_speed_parameter("approach_speed")
                            .triggered_by()
                                .distance_condition("pedestrian1")
                                    .to_position(world_position(50.0, 10.0, 0.0))
                                    .closer_than(2.0)  // When pedestrian reaches other side
                                .finish()
                            .finish()?
                        .finish()
                    .finish()
                .finish()
            .finish()
        .build()
}
```

## Advanced Features

### Custom Validation

Implement custom validation logic:

```rust
impl ScenarioBuilder<HasEntities> {
    fn validate_entity_count(&self) -> Result<(), BuilderError> {
        if let Some(entities) = &self.data.entities {
            if entities.scenario_objects.is_empty() {
                return Err(BuilderError::validation_error(
                    "At least one entity is required"
                ));
            }
        }
        Ok(())
    }
}
```

### Performance Optimization

For large scenarios, consider:

```rust
// Pre-allocate collections
let mut builder = ScenarioBuilder::new()
    .with_header("Large Scenario", "Author")
    .with_entities();

// Add many entities efficiently
for i in 0..100 {
    builder = builder
        .add_vehicle(&format!("vehicle_{}", i))
            .car()
            .finish();
}

let scenario = builder.build()?;
```

### Integration with External Tools

Export scenarios for external validation:

```rust
let scenario = build_my_scenario()?;

// Serialize to XML
let xml = openscenario_rs::serialize_to_string(&scenario)?;

// Save to file
std::fs::write("scenario.xosc", xml)?;

// Validate with external tool
std::process::Command::new("openscenario-validator")
    .arg("scenario.xosc")
    .status()?;
```

## Troubleshooting

### Common Issues

1. **Compilation Errors**: Usually indicate incorrect state transitions
   ```rust
   // Error: Cannot add vehicle before header
   ScenarioBuilder::new().add_vehicle("ego");  // ❌
   
   // Correct: Set header first
   ScenarioBuilder::new()
       .with_header("Test", "Author")
       .with_entities()
       .add_vehicle("ego");  // ✅
   ```

2. **Runtime Errors**: Check for missing required fields
   ```rust
   // Error: Missing speed value
   .add_speed_action()
       .finish()?;  // ❌ No speed set
   
   // Correct: Set speed value
   .add_speed_action()
       .to_speed(30.0)
       .finish()?;  // ✅
   ```

3. **Validation Errors**: Ensure logical consistency
   ```rust
   // Error: Invalid entity reference
   .add_maneuver("test", "nonexistent_entity");  // ❌
   
   // Correct: Use existing entity
   .add_maneuver("test", "ego");  // ✅
   ```

### Debug Tips

1. **Enable Debug Logging**:
   ```rust
   env_logger::init();
   log::debug!("Building scenario...");
   ```

2. **Validate Incrementally**:
   ```rust
   let builder = ScenarioBuilder::new()
       .with_header("Test", "Author");
   builder.validate()?;  // Check each step
   
   let builder = builder.with_entities();
   builder.validate()?;
   ```

3. **Use Type Annotations**:
   ```rust
   let builder: ScenarioBuilder<HasHeader> = ScenarioBuilder::new()
       .with_header("Test", "Author");
   ```

This guide provides comprehensive coverage of the OpenSCENARIO-rs builder system. For more examples and advanced usage patterns, see the `examples/` directory in the repository.