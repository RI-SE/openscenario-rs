# Distribution System Fixes - OpenSCENARIO-rs

## Problem Summary

The current distribution system in OpenSCENARIO-rs has fundamental schema compliance issues that prevent parsing of valid OpenSCENARIO files, specifically parameter variation documents (`.xosc` files with `ParameterValueDistribution`).

### Root Cause Analysis

**Error**: `invalid type: map, expected a sequence` when parsing ALKS scenario files

**Technical Root Cause**: The Rust `Deterministic` struct incorrectly models the XSD schema structure:

- **XSD Schema**: `Deterministic` → `DeterministicParameterDistribution` (choice group, `maxOccurs="unbounded"`)
- **XSD Schema**: `DeterministicParameterDistribution` = **choice** between `DeterministicSingleParameterDistribution` OR `DeterministicMultiParameterDistribution`
- **Current Rust**: Separate vectors `single_distributions: Vec<>` and `multi_distributions: Vec<>`
- **Problem**: Serde expects array but finds single object when file contains one `<DeterministicMultiParameterDistribution>`

## XSD Schema Structure (Ground Truth)

```xml
<!-- XSD Definition -->
<xsd:complexType name="Deterministic">
    <xsd:sequence>
        <xsd:group ref="DeterministicParameterDistribution" minOccurs="0" maxOccurs="unbounded"/>
    </xsd:sequence>
</xsd:complexType>

<xsd:group name="DeterministicParameterDistribution">
    <xsd:choice>
        <xsd:element name="DeterministicMultiParameterDistribution" type="DeterministicMultiParameterDistribution"/>
        <xsd:element name="DeterministicSingleParameterDistribution" type="DeterministicSingleParameterDistribution"/>
    </xsd:choice>
</xsd:group>
```

## Current Implementation Issues

### 1. Deterministic Struct (❌ Incorrect)

**File**: `src/types/distributions/deterministic.rs`

**Current**:
```rust
pub struct Deterministic {
    #[serde(rename = "DeterministicSingleParameterDistribution", default)]
    pub single_distributions: Vec<DeterministicSingleParameterDistribution>,
    #[serde(rename = "DeterministicMultiParameterDistribution", default)]
    pub multi_distributions: Vec<DeterministicMultiParameterDistribution>,
}
```

**Problem**: Treats single and multi as separate lists instead of unified choice sequence

### 2. DeterministicParameterDistribution Enum (✅ Correct Structure, ❌ Wrong Usage)

**Current**:
```rust
pub enum DeterministicParameterDistribution {
    #[serde(rename = "DeterministicSingleParameterDistribution")]
    Single(DeterministicSingleParameterDistribution),
    #[serde(rename = "DeterministicMultiParameterDistribution")]
    Multi(DeterministicMultiParameterDistribution),
}
```

**Problem**: Enum is correct but not used in `Deterministic` struct

## Proposed Fixes

### Phase 1: Core Schema Compliance Fix

#### 1.1 Fix Deterministic Struct

**File**: `src/types/distributions/deterministic.rs`

**Target Structure**:
```rust
pub struct Deterministic {
    #[serde(rename = "$value", default)]
    pub distributions: Vec<DeterministicParameterDistribution>,
}
```

**Challenges**:
- Serde doesn't handle XSD choice groups with `$value` well
- May need custom serializer/deserializer
- Breaking change for existing API consumers

#### 1.2 Alternative: Custom Serde Implementation

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Deterministic {
    pub distributions: Vec<DeterministicParameterDistribution>,
}

// Custom Serialize/Deserialize implementation to handle XSD choice semantics
```

### Phase 2: Update Consuming Code

#### 2.1 Files Requiring Updates

1. **`src/types/controllers/mod.rs`** - Update default creation
2. **`examples/parse.rs`** - Update distribution analysis  
3. **`src/types/distributions/mod.rs`** - Update validation and utilities
4. **Any test files** - Update test construction

#### 2.2 API Migration Strategy

**Option A: Breaking Change**
- Update `Deterministic` struct directly
- Update all consumers
- Single coordinated change

**Option B: Gradual Migration** 
- Add new unified `distributions()` method
- Deprecate old `single_distributions`/`multi_distributions` fields
- Maintain backward compatibility temporarily

### Phase 3: Validation & Testing

#### 3.1 Test Files to Validate

- ✅ `xosc/alks_scenario_4_6_2_lateral_detection_range_variation.xosc` (Current failing case)
- `xosc/concrete_scenarios/alks_scenario_4_6_2_lateral_detection_range_template.xosc`
- Any other parameter variation files

#### 3.2 Test Cases Needed

1. **Single Distribution Only**: File with just `DeterministicSingleParameterDistribution`
2. **Multi Distribution Only**: File with just `DeterministicMultiParameterDistribution` (current case)
3. **Mixed Distributions**: File with both types
4. **Empty Deterministic**: File with empty `<Deterministic>` element
5. **Multiple of Same Type**: File with multiple `DeterministicMultiParameterDistribution` elements

## Implementation Plan

### Step 1: Custom Serde Implementation (Recommended)

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Deterministic {
    pub distributions: Vec<DeterministicParameterDistribution>,
}

impl<'de> Deserialize<'de> for Deterministic {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Custom logic to handle XSD choice group deserialization
        // Parse XML elements as they appear and convert to enum variants
    }
}

impl Serialize for Deterministic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Custom logic to serialize enum variants as appropriate XML elements
    }
}
```

### Step 2: Backward Compatibility Layer

```rust
impl Deterministic {
    /// Legacy method for backward compatibility
    #[deprecated(note = "Use distributions() instead")]
    pub fn single_distributions(&self) -> Vec<&DeterministicSingleParameterDistribution> {
        self.distributions.iter()
            .filter_map(|d| match d {
                DeterministicParameterDistribution::Single(s) => Some(s),
                _ => None,
            })
            .collect()
    }

    /// Legacy method for backward compatibility  
    #[deprecated(note = "Use distributions() instead")]
    pub fn multi_distributions(&self) -> Vec<&DeterministicMultiParameterDistribution> {
        self.distributions.iter()
            .filter_map(|d| match d {
                DeterministicParameterDistribution::Multi(m) => Some(m),
                _ => None,
            })
            .collect()
    }
}
```

### Step 3: Update Consuming Code

1. **Update controllers/mod.rs**: Use new unified API
2. **Update examples/parse.rs**: Use new unified API or temporary compatibility methods
3. **Update validation**: Use new structure
4. **Update tests**: Use new structure

### Step 4: Validation

1. Test ALKS scenario parsing (should fix the current error)
2. Test all parameter variation scenarios
3. Test serialization round-trip
4. Test backward compatibility (if implemented)

## Risk Assessment

### High Risk
- **Breaking API changes**: Affects all consumers of `Deterministic`
- **Serde complexity**: Custom serialization can be error-prone
- **XML edge cases**: May uncover other schema compliance issues

### Medium Risk  
- **Performance impact**: Custom serde may be slower than derive
- **Testing complexity**: Need comprehensive test coverage for XSD choice semantics

### Low Risk
- **Feature compatibility**: Core OpenSCENARIO functionality unaffected
- **Schema evolution**: Fix aligns with official XSD, future-proof

## Success Criteria

1. ✅ `xosc/alks_scenario_4_6_2_lateral_detection_range_variation.xosc` parses successfully
2. ✅ All existing parameter variation files continue to parse
3. ✅ Serialization round-trip maintains XML structure
4. ✅ No regression in existing scenario parsing
5. ✅ API provides clean, unified access to all distributions

## Timeline Estimate

- **Step 1 (Custom Serde)**: 4-6 hours
- **Step 2 (Compatibility Layer)**: 2-3 hours  
- **Step 3 (Update Consumers)**: 3-4 hours
- **Step 4 (Testing & Validation)**: 2-3 hours
- **Total**: 11-16 hours

## Follow-up Actions

After fixing the `Deterministic` struct:

1. **Audit other distribution types** for similar XSD compliance issues
2. **Review catalog system** for choice group handling
3. **Validate against broader set** of real-world OpenSCENARIO files
4. **Consider schema validation tool** to catch future compliance issues

## References

- **OpenSCENARIO XSD Schema**: `Schema/OpenSCENARIO.xsd`
- **Failing test file**: `xosc/alks_scenario_4_6_2_lateral_detection_range_variation.xosc`
- **Current implementation**: `src/types/distributions/deterministic.rs:8-19`
- **Error reproduction**: `cargo run --example test_alks_parse ./xosc/alks_scenario_4_6_2_lateral_detection_range_variation.xosc`