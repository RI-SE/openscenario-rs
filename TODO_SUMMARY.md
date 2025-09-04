# OpenSCENARIO-rs TODO Summary

This document summarizes all the TODO items that have been added to project files, organized by implementation phase and priority.

## Phase 1: Foundation MVP (Weeks 1-4)

### Week 1: Core Infrastructure (CRITICAL) ✅ COMPLETED

**src/lib.rs** ✅
- [x] Re-export core types from submodules (Error, Result, OpenScenario, FileHeader)
- [x] Implement high-level convenience functions (parse_file, parse_str, serialize_str, validate_scenario)
- [x] Add feature flag conditional compilation (builder, validation, streaming)
- [x] Add comprehensive library documentation with examples

**src/error.rs** ✅
- [x] Define main error enum with thiserror (XmlParseError, IoError, ValidationError, etc.)
- [x] Define Result type alias
- [x] Add error context helpers (with_context, parameter_error, validation_error, parsing_error)

**Cargo.toml** ✅
- [x] Define package metadata and basic dependencies (serde, quick-xml, thiserror)
- [x] Define feature flags (serde-parsing, validation, builder, expressions, streaming)
- [x] Add development dependencies (proptest, criterion, tempfile)

**src/types/basic.rs** ✅
- [x] Define Value<T> enum for parameter support (Literal, Parameter)
- [x] Implement custom serde deserializer for Value<T> (${param} pattern)
- [x] Define basic OpenSCENARIO type aliases (String, Double, Int, etc.)
- [x] Add parameter validation helpers (parse_parameter_reference, is_valid_parameter_name)
- [x] Add comprehensive unit tests (12 test cases)

**src/types/enums.rs** ✅
- [x] Implement critical enums for MVP (VehicleCategory, PedestrianCategory, ObjectType, Rule, ConditionEdge)
- [x] Add 10 total enums including coordinate system and dynamics enums
- [x] Add serde annotations for XML mapping (#[serde(rename = "...")])
- [x] Implement Display and FromStr traits for string conversion
- [x] Add unit tests for serialization and parsing

**src/parser/xml.rs** ✅
- [x] Implement serde-based parsing functions (parse_from_str, parse_from_file)
- [x] Implement serialization functions (serialize_to_string, serialize_to_file)
- [x] Handle XML parsing errors properly with context
- [x] Add XML validation (validate_xml_structure)
- [x] Add unit tests for validation logic

### Week 2: Basic Scenario Structure (CRITICAL) 🟡 IN PROGRESS

**src/types/scenario/storyboard.rs** ✅
- [x] Define OpenScenario root type with serde annotations
- [x] Define FileHeader with required metadata fields
- [x] Define Storyboard, Init, and InitActions structures (simplified for MVP)

**src/types/entities/mod.rs**
- [ ] Define ScenarioObject wrapper and EntityObject enum
- [ ] Define Entities collection container
- [ ] Define EntityRef for entity references

**src/types/mod.rs** ✅  
- [x] Declare all submodules (basic, enums, entities, geometry, scenario, etc.)
- [x] Re-export core types for convenience (Value, OSString, enums, scenario types)
- [x] Define common traits (Validate, Resolve) and contexts
- [x] Implement ValidationContext and ParameterContext with registries
- [x] Add EntityRef and CatalogRef for cross-references

**tests/integration_tests.rs**
- [ ] Add basic parsing tests (can_parse_simple_scenario, can_access_file_header)
- [ ] Add error handling tests (malformed XML, missing fields)

### Week 3: Basic Entities (HIGH)

**src/types/entities/vehicle.rs**
- [ ] Implement Vehicle struct with serde annotations
- [ ] Include basic fields (name, category, bounding_box)
- [ ] Skip complex fields like Performance, Axles for MVP

**src/types/entities/pedestrian.rs**
- [ ] Implement Pedestrian struct with serde annotations
- [ ] Include basic fields (name, category, bounding_box)

**src/types/geometry/shapes.rs**
- [ ] Implement BoundingBox and supporting geometry types
- [ ] Add Center and Dimensions structs

**tests/integration_tests.rs**
- [ ] Add entity access tests (can_access_entities)

### Week 4: Basic Actions & Conditions (MEDIUM)

**src/types/actions/mod.rs**
- [ ] Define base Action enum and ActionWrapper
- [ ] Re-export action types for MVP

**src/types/actions/movement.rs**
- [ ] Implement SpeedAction and TeleportAction
- [ ] Define supporting types (TransitionDynamics, SpeedActionTarget, etc.)

**src/types/conditions/mod.rs**
- [ ] Define base Condition enum and ConditionWrapper
- [ ] Re-export condition types for MVP

**src/types/conditions/value.rs**
- [ ] Implement SimulationTimeCondition

**src/types/conditions/entity.rs**
- [ ] Implement SpeedCondition

**tests/integration_tests.rs**
- [ ] Add tests for accessing actions and conditions

## Phase 2: Usability MVP (Weeks 5-8)

### Week 5: Expression System (HIGH)
- [ ] Enhance Value<T> deserializer for ${param} pattern
- [ ] Implement parameter resolution logic
- [ ] Add parameter validation helpers

### Week 6: Validation Framework (HIGH)
- [ ] Implement Validate trait and ValidationContext
- [ ] Add basic validations (required fields, entity references)
- [ ] Add validation tests

### Week 7: Builder Pattern (MEDIUM)

**src/builder/scenario.rs**
- [ ] Implement ScenarioBuilder struct and basic methods
- [ ] Add entity building methods (add_vehicle, add_pedestrian)
- [ ] Add builder validation during construction
- [ ] Add convenience methods for common patterns

**tests/integration_tests.rs**
- [ ] Add builder integration tests

### Week 8: Serialization (HIGH)

**src/parser/xml.rs**
- [ ] Implement serde-based serialization functions
- [ ] Add proper XML declaration and formatting

**tests/integration_tests.rs**
- [ ] Add round-trip serialization tests
- [ ] Verify serialized XML is well-formed

## Phase 3: Ecosystem MVP (Weeks 9-12)

### Week 9: Position System
- [ ] Implement WorldPosition, RelativeWorldPosition, LanePosition
- [ ] Add coordinate system transformation utilities

### Week 10: More Actions
- [ ] Implement LaneChangeAction, FollowTrajectoryAction, SynchronizeAction

### Week 11: More Conditions
- [ ] Implement DistanceCondition, CollisionCondition, TimeHeadwayCondition

### Week 12: Geometry Basics
- [ ] Implement Polyline and basic trajectory support
- [ ] Add coordinate transformations

## Phase 4+: Production Ready (Weeks 13+)

### Week 13: Streaming Parser
- [ ] Add streaming parser feature flag
- [ ] Implement manual quick-xml parsing for large files

**src/parser/xml.rs**
- [ ] Add streaming parser module (feature gated)

### Week 15: Documentation & Examples

**examples/basic_parsing.rs**
- [ ] Implement complete basic parsing example
- [ ] Add error handling demonstration
- [ ] Create example OpenSCENARIO file

## TODO Categories Summary

### Critical for MVP (Must Implement First)
- Error handling and Result types
- Basic data types and enums
- Core scenario structure (OpenScenario, FileHeader, Entities)
- Serde-based XML parsing and serialization
- Basic entity types (Vehicle, Pedestrian)

### High Priority (Core Functionality)
- Parameter expression system
- Validation framework
- Basic actions and conditions
- Round-trip serialization testing

### Medium Priority (Enhanced Features)
- Builder pattern for programmatic construction
- Extended action and condition types
- Position system and geometry

### Future Features (Performance & Polish)
- Streaming parser for large files
- Advanced validation rules
- Comprehensive documentation and examples
- Performance optimization and benchmarking

## Current Status Summary (Updated)

### ✅ **COMPLETED (Week 1 - Core Infrastructure)**
- **Complete Error Handling**: Error enum, Result type, context helpers
- **Complete Type System Foundation**: Value<T> enum with parameter support (${param})
- **Complete Enum System**: 10 critical enums with serde + Display/FromStr traits
- **Complete XML Parsing**: Parse/serialize with validation and proper error handling
- **Complete Project Configuration**: Dependencies, features, dev dependencies
- **Complete Public API**: lib.rs with convenience functions and clean re-exports
- **Complete Module Organization**: types/mod.rs with traits, contexts, and module structure
- **Minimal Scenario Structure**: OpenScenario root type for parser compatibility

**Build Status**: ✅ `cargo build --lib` and `cargo test --lib` both succeed  
**Test Status**: ✅ 6 unit tests passing (basic types, enums, parser validation)

### 🎯 **CURRENT PRIORITIES (Week 2 - In Progress)**
- Implement entities system (Vehicle, Pedestrian, ScenarioObject, Entities collection)
- Implement geometry types (BoundingBox, Center, Dimensions)
- Add basic integration tests with real OpenSCENARIO XML
- Create test data and fixtures

### 📝 **SUCCESS METRICS ACHIEVED**
- **Week 1 Goal**: ✅ Core infrastructure compiles and basic types work
- **Public API**: ✅ Clean, documented API with `openscenario_rs::parse_file()` 
- **Type Safety**: ✅ Strong typing with parameter support and validation framework
- **Ready for**: Entity implementation and first working scenario parsing tests

## Implementation Notes

1. **✅ Serde approach working** - XML parsing and serialization infrastructure complete
2. **✅ Error handling robust** - Comprehensive error types with proper context  
3. **✅ Parameter system ready** - Value<T> enum handles ${param} syntax correctly
4. **✅ Public API complete** - Clean, documented API with convenience functions
5. **✅ Module system ready** - Type organization with traits and validation framework
6. **Next**: Implement entities (Vehicle, Pedestrian) and geometry (BoundingBox)
7. **Next**: Create integration tests with real OpenSCENARIO XML files
8. **Ready for**: First complete scenario parsing and validation

**Foundation Status**: Week 1 objectives exceeded - not only do we have core infrastructure, but also complete public API and module organization that was planned for Week 2. The library is ahead of schedule and ready for entity implementation.