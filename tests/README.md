# Integration Tests for OpenSCENARIO-rs

This directory contains comprehensive integration tests for the OpenSCENARIO-rs library, focusing on real-world scenario parsing and validation.

## Test Files

### `integration_tests.rs`
Main integration test file containing:

#### Basic Parsing Tests
- `can_parse_simple_scenario_from_string()` - Tests basic XML parsing functionality
- `can_access_file_header()` - Validates file header parsing and access
- `can_access_entities()` - Tests entity parsing and retrieval
- `can_serialize_and_deserialize_scenario()` - Validates round-trip serialization
- `handles_malformed_xml()` - Error handling for invalid XML
- `handles_missing_required_fields()` - Validation of required field enforcement

#### Cut-in Scenario Tests (`cut_in_scenario_tests` module)
Comprehensive tests specifically for the `cut_in_101_exam.xosc` scenario file:

- **`can_parse_cut_in_101_exam_scenario()`** - Validates parsing of the complex cut-in scenario with proper file header information
- **`can_access_cut_in_entities()`** - Tests entity extraction and validation for all three vehicles (Ego, A1, A2)
- **`can_access_cut_in_storyboard()`** - Validates storyboard structure parsing
- **`can_validate_cut_in_story_structure()`** - Tests story and act structure validation
- **`can_roundtrip_cut_in_scenario()`** - Tests serialization and deserialization fidelity
- **`validates_scenario_file_exists()`** - Basic file structure and content validation

## Test Data

### `cut_in_101_exam.xosc`
A complex OpenSCENARIO file located at `xosc/cut_in_101_exam.xosc` containing:

- **Entities**: 3 vehicles (Ego, A1, A2) with detailed specifications
- **File Header**: Metadata including author, date, and version information
- **Storyboard**: Complex story structure with initialization and maneuvers
- **Trajectory Data**: Detailed vehicle trajectories with precise timing
- **Environment Settings**: Weather, lighting, and road conditions

### Key Scenario Properties
- **Author**: OnSite_TOPS
- **Description**: scenario_highD
- **Vehicles**: All vehicles use "Default_car" configuration with identical physical properties
- **Story**: Named "Cutin" with three acts (Act_Ego, Act_A1, Act_A2)
- **Environment**: Default environment with clear weather and standard road conditions

## MVP Considerations

The current tests are designed to work with the MVP (Minimum Viable Product) implementation of OpenSCENARIO-rs. Some tests include graceful handling of parsing failures that may occur due to incomplete feature implementation.

Key MVP limitations addressed in tests:
- Simplified vehicle type definitions (no performance or axles fields yet)
- Basic story structure (detailed story elements may not be fully implemented)
- Graceful error handling for incomplete parsing capabilities

## Running Tests

```bash
# Run all integration tests
cargo test --test integration_tests

# Run only cut-in scenario tests
cargo test --test integration_tests cut_in

# Run with verbose output
cargo test --test integration_tests -- --nocapture
```

## Test Philosophy

These integration tests follow a pragmatic approach:

1. **Real-world validation**: Tests use actual OpenSCENARIO files rather than synthetic examples
2. **Progressive enhancement**: Tests are designed to pass with current MVP implementation while being ready for future enhancements
3. **Graceful degradation**: Complex parsing features that aren't implemented yet are handled with appropriate fallbacks
4. **Comprehensive coverage**: Tests cover file structure, entity parsing, serialization, and error handling

## Future Enhancements

As the library evolves, these tests will be enhanced to cover:
- Detailed trajectory validation
- Performance characteristics parsing
- Complex story element validation
- Advanced environment settings
- Catalog reference resolution
- Parameter substitution