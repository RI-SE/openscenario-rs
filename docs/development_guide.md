# Development Guide

This document provides essential patterns, troubleshooting guidance, and architectural insights for working with the openscenario-rs codebase.

## Type System Patterns

### Value<T> Type Usage

The library uses `Value<T>` types (e.g., `Double`, `OSString`) to support parameterization in OpenSCENARIO files. Understanding the correct usage patterns is crucial:

**Constructor Parameters vs Struct Fields**:
- **Constructor parameters**: Use concrete types (`f64`, `String`) for ergonomics
- **Struct fields**: Use `Value<T>` types (`Double`, `OSString`) for parameterization support
- **Conversion**: Automatic via `::literal()` constructors

```rust
// ✅ Correct: Constructor takes f64
impl TrafficSwarmAction {
    pub fn new(velocity: f64) -> Self {
        Self {
            velocity: Some(Double::literal(velocity)), // Wrapped in Value<T>
            // ...
        }
    }
}

// ✅ Correct: Field uses Value<T> type
pub struct TrafficSwarmAction {
    pub velocity: Option<Double>, // Value<f64> wrapper
}
```

**Comparison Patterns**:
When testing or comparing `Value<T>` types, use `.as_literal()` to extract wrapped values:

```rust
// ✅ Correct: Extract literal value for comparison
assert_eq!(action.velocity.as_ref().unwrap().as_literal(), Some(&10.0));

// ❌ Incorrect: Direct comparison fails
// assert_eq!(action.velocity, Some(10.0));
```

**Common Value<T> Types**:
- `Double` = `Value<f64>`
- `OSString` = `Value<String>` 
- `UnsignedInt` = `Value<u32>`
- `PositiveDouble` = `Value<f64>` (with validation)

## XML Serialization Patterns

### Optional Field Handling

Optional fields in structs require careful serialization handling to prevent empty attribute issues:

```rust
// ✅ Correct: Skip serialization of None values
#[derive(Serialize, Deserialize)]
pub struct TrafficSwarmAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub velocity: Option<Double>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_definition: Option<TrafficDefinition>,
}
```

**Problem**: Without `skip_serializing_if`, optional fields serialize as empty attributes (`velocity=""`), which fail to deserialize back to `Value<T>` types.

**Solution**: Always use `skip_serializing_if = "Option::is_none"` for optional `Value<T>` fields.

## Common Compilation Fixes

### Value<T> Type Mismatches

**Issue**: Direct assignment of raw values to `Value<T>` fields
```rust
// ❌ Incorrect
action.velocity = Some(10.0);

// ✅ Correct
action.velocity = Some(Double::literal(10.0));
```

**Issue**: Direct comparison of `Value<T>` with raw values
```rust
// ❌ Incorrect
assert_eq!(action.velocity, Some(10.0));

// ✅ Correct
assert_eq!(action.velocity.as_ref().unwrap().as_literal(), Some(&10.0));
```

**Issue**: OSString comparison issues
```rust
// ❌ Incorrect
assert_eq!(road.name, "Highway1");

// ✅ Correct
assert_eq!(road.name.as_literal(), Some(&"Highway1".to_string()));
```

### Float Literal Syntax

**Issue**: Invalid float literals in tests
```rust
// ❌ Incorrect
let value = 10.;

// ✅ Correct
let value = 10.0;
```

## Testing Patterns

### XML Round-Trip Tests

When writing XML serialization tests, ensure proper handling of optional fields:

```rust
#[test]
fn test_xml_round_trip() {
    let original = TrafficSwarmAction {
        velocity: Some(Double::literal(10.0)),
        traffic_definition: None, // Will be skipped in serialization
    };
    
    let xml = quick_xml::se::to_string(&original).unwrap();
    let deserialized: TrafficSwarmAction = quick_xml::de::from_str(&xml).unwrap();
    
    // Use as_literal() for Value<T> comparisons
    assert_eq!(
        deserialized.velocity.as_ref().unwrap().as_literal(),
        Some(&10.0)
    );
}
```

## Schema Compliance

### XSD Compliance Patterns

OpenSCENARIO-rs achieves 95%+ XSD validation compliance. Follow these patterns for maintaining compliance:

#### Optional Attribute Pattern

```rust
// ✅ Correct: XSD-compliant optional attribute
#[derive(Serialize, Deserialize)]
pub struct LaneChangeAction {
    #[serde(
        rename = "@targetLaneOffset",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_double"
    )]
    pub target_lane_offset: Option<Double>,
}

// ❌ Incorrect: Serializes empty string for None values
#[serde(rename = "@targetLaneOffset", default)]
pub target_lane_offset: Option<Double>,
```

#### Custom Deserializer for Empty Strings

```rust
fn deserialize_optional_double<'de, D>(deserializer: D) -> Result<Option<Double>, D::Error>
where
    D: Deserializer<'de>,
{
    // Handle empty strings gracefully for backward compatibility
    // Empty strings become None, valid values become Some(Double)
}
```

#### XSD Choice Group Implementation

```rust
// ✅ Correct: Custom serialization for choice groups
impl Serialize for EntityCondition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        match self {
            EntityCondition::SpeedCondition(condition) => {
                map.serialize_entry("SpeedCondition", condition)?;
            }
            // ... other variants
        }
        map.end()
    }
}
```

### Verifying OpenSCENARIO XSD Compliance

When modifying serialization behavior, always verify against the official schema:

1. Check if attributes are required (`use="required"`) or optional
2. Verify element occurrence constraints (`minOccurs`, `maxOccurs`)
3. Note deprecated elements/attributes in the schema
4. Ensure serialization matches schema expectations
5. **Test XSD validation**: Use tools to validate generated XML

**XSD Validation Testing**:
```bash
# Use examples to validate XSD compliance
cargo run --example test_lane_change_serialization
```

**Common XSD Issues**:
- Empty attributes (`targetLaneOffset=""`) instead of omission
- Incorrect choice group structure (flat vs nested elements)
- Attributes serialized as elements or vice versa

## Troubleshooting Checklist

### Test Compilation Failures

1. **Value<T> type mismatches**:
   - Check if raw values are assigned to `Value<T>` fields
   - Use `::literal()` constructors for wrapping
   - Use `.as_literal()` for comparisons

2. **XML serialization issues**:
   - Add `skip_serializing_if = "Option::is_none"` to optional fields
   - Verify schema compliance for serialization changes

3. **Float syntax errors**:
   - Ensure all float literals end with `.0` (e.g., `10.0` not `10.`)

4. **String comparison issues**:
   - Use `.as_literal()` for `OSString` comparisons
   - Convert string literals to `String` when needed

### Architecture Validation

When uncertain about type usage patterns:

1. **Search existing code**: Look for similar implementations in the codebase
2. **Check constructor patterns**: See how other structs handle `Value<T>` conversion
3. **Verify test patterns**: Follow established testing conventions
4. **Consult schema**: Ensure changes align with OpenSCENARIO XSD requirements

## Recent Fixes Applied

### XSD Validation Compliance (Latest - Resolved)

**Problem**: XSD validation failures due to improper XML serialization patterns.

**Root Cause**: 
- `LaneChangeAction` serialized empty `targetLaneOffset=""` instead of omitting attribute
- `EntityCondition` enum didn't match XSD choice group structure requirements
- Various attribute vs element serialization inconsistencies

**Solution**: Implemented comprehensive XSD compliance patterns:

**Files Modified**:
- `src/types/actions/movement.rs` - Added `skip_serializing_if` to `targetLaneOffset`
- `src/types/conditions/entity.rs` - Custom `Serialize` for choice groups
- `tests/xsd_validation_fixes_test.rs` - Comprehensive XSD compliance tests
- `examples/test_lane_change_serialization.rs` - Validation example
- `docs/xsd_validation_fixes.md` - Complete implementation guide

**Key Patterns**:
- Use `skip_serializing_if = "Option::is_none"` for all optional XML attributes
- Implement custom `Serialize` for XSD choice groups using `SerializeMap`
- Use custom deserializers for graceful empty string handling
- Always use `@` prefix for XML attributes in serde annotations

### TrafficSwarmAction XML Serialization (Resolved)

**Problem**: `test_xml_round_trip_traffic_swarm` failing due to empty attribute serialization.

**Root Cause**: Optional `Value<T>` fields serialized as empty attributes, failing deserialization.

**Solution**: Added `skip_serializing_if = "Option::is_none"` to optional fields in `TrafficSwarmAction`.

**Files Modified**:
- `src/types/actions/traffic.rs` - Main fix + test updates
- `src/types/actions/control.rs` - Value<T> comparison fixes
- `src/types/actions/movement.rs` - Raw value assignment fixes
- `src/types/road.rs` - OSString comparison fixes
- `src/types/positions/road.rs` - Multiple Value<T> fixes
- `tests/phase1_position_types_test.rs` - Float literal and type fixes
- `tests/integration_tests.rs` & `tests/integration_tests_fixed.rs` - Minor fixes

**Key Pattern**: Always use `skip_serializing_if = "Option::is_none"` for optional `Value<T>` fields to prevent empty attribute serialization issues.

## Contributing Guidelines

1. **Follow existing patterns**: Study neighboring files before implementing
2. **Test thoroughly**: Include XML round-trip tests for serializable types
3. **Verify schema compliance**: Ensure changes align with OpenSCENARIO XSD
4. **Use proper types**: Follow `Value<T>` usage patterns consistently
5. **Document significant changes**: Update this guide when discovering new patterns