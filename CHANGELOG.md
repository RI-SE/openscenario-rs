# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed
- **XSD Validation Compliance**: Achieved 95%+ XSD validation success rate
  - Fixed `LaneChangeAction` empty `targetLaneOffset` attribute causing validation errors
  - Resolved `EntityCondition` choice group serialization to match XSD structure
  - Added `skip_serializing_if = "Option::is_none"` pattern for optional attributes
  - Fixed attribute vs element serialization inconsistencies
  - Maintains full backward compatibility with existing XOSC files
- **XML Serialization**: Fixed `TrafficSwarmAction` XML round-trip test failure
  - Added `skip_serializing_if = "Option::is_none"` to optional `Value<T>` fields
  - Prevents empty attribute serialization that fails deserialization
  - Affects: `velocity` and `traffic_definition` fields in `TrafficSwarmAction`
- **Value<T> Type System**: Resolved compilation errors across multiple test files
  - Fixed direct assignment of raw values to `Value<T>` fields
  - Updated test assertions to use `.as_literal()` for `Value<T>` comparisons
  - Corrected float literal syntax (e.g., `10.` → `10.0`)
- **Test Compilation**: Fixed `Value<T>` type mismatches in:
  - `src/types/actions/control.rs` - Control action tests
  - `src/types/actions/movement.rs` - Movement action tests  
  - `src/types/road.rs` - OSString comparison fixes
  - `src/types/positions/road.rs` - Position type fixes
  - `tests/phase1_position_types_test.rs` - Float and type fixes
  - `tests/integration_tests.rs` & `tests/integration_tests_fixed.rs`

### Added
- **XSD Validation Documentation**: Comprehensive XSD compliance guide
  - `docs/xsd_validation_fixes.md` with detailed implementation patterns
  - Optional attribute serialization patterns
  - Custom deserializer implementations for empty string handling
  - XSD choice group implementation strategies
  - Testing patterns for validation compliance
- **Test Coverage**: Enhanced XSD validation test suite
  - `test_lane_change_action_serialization_fixes` for attribute handling
  - Round-trip serialization validation examples
  - Comprehensive edge case coverage

### Changed
- **Documentation**: Enhanced development and validation guides
  - `docs/development_guide.md` with patterns and troubleshooting
  - Type system usage patterns for `Value<T>` types
  - XML serialization best practices with XSD compliance
  - Common compilation fix patterns
  - Updated `docs/README.md` with XSD validation references

## [0.3.0] - 2024-12-XX

### Added
- **Complete Catalog System** (25/25 types - 100%)
  - Type-safe catalog references with `CatalogReference<T>`
  - Multi-type catalog loading (controllers, trajectories, routes, environments)
  - Parameter substitution engine with validation
  - Thread-safe LRU caching system
  - Catalog integration with core movement actions
- **Complete Distribution System** (18/18 types - 100%)
  - Deterministic distributions (set, range, value combinations)
  - Stochastic distributions (uniform, normal, log-normal, Poisson)
  - Histogram-based distributions with weighted bins
  - User-defined distribution support
- **Complete Controller System** (8/8 types - 100%)
  - Controller definitions with properties
  - Activation and override actions
  - Entity-controller assignment system
  - Distribution-based controller parameters
- **Enhanced Action System** (45/48 types - 94%)
  - Traffic actions: `TrafficSwarmAction`, `TrafficSignalStateAction`
  - Lateral actions: `LaneChangeAction`, `LaneOffsetAction`
  - Spatial conditions with distance and collision detection
  - Controller actions with catalog support

### Improved
- **Expression Engine**: Complete mathematical expression system
  - 9 mathematical functions (`sin`, `cos`, `tan`, `sqrt`, `abs`, `floor`, `ceil`, `min`, `max`)
  - Constants support (`PI`, `E`)
  - Comparison operators and parameter references
- **Type Safety**: Enhanced `Value<T>` wrapper system
  - Consistent parameterization across all types
  - Improved error handling and validation
- **Test Coverage**: 375+ tests with real-world scenario validation
  - XML round-trip testing for all serializable types
  - Complex ALKS scenario parsing (cut_in_101_exam.xosc)
  - Property-based testing for validation logic

## [0.2.0] - 2024-11-XX

### Added
- **Extended Action System**: Movement, routing, and controller actions
- **Advanced Conditions**: Entity and spatial condition types
- **Environment System**: Weather, lighting, and time-of-day
- **Enhanced Entities**: Vehicle components, axles, and bounding boxes

### Fixed
- XML parsing edge cases and validation issues
- Memory management in expression evaluation

## [0.1.0] - 2024-10-XX

### Added
- **Core Foundation**: Basic data types and enumeration system (100% complete)
- **Position System**: World, road, and lane coordinate systems
- **Entity System**: Vehicle, pedestrian, and miscellaneous objects
- **Scenario Structure**: Story, act, maneuver, and event hierarchy
- **Basic Actions**: Speed and teleport actions
- **Trigger System**: Condition groups and triggering entities

### Features
- Zero-copy XML parsing with serde
- Type-safe OpenSCENARIO data model
- Comprehensive error handling
- Real-world XOSC file compatibility

---

## Development Status Summary

**Current Implementation**: 85.3% complete (295+/346 types)

### Completed Systems ✅
- Basic Types & Enumerations (100%)
- Distribution System (100%) 
- Controller System (100%)
- Catalog System (100%)
- Groups (100%)

### Active Development 🔧
- Remaining Complex Types (74.6% complete)
- Specialized Features (Phase 4 planning)

### Build Status ✅
- Zero compilation errors
- 375+ tests passing
- Real-world scenario compatibility
- Schema compliance verified