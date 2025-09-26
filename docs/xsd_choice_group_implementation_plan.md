# Complete XSD Choice Group Implementation Plan

## Executive Summary

This document outlines a comprehensive plan to implement proper XSD choice group handling in OpenSCENARIO-rs. The goal is to achieve full schema compliance for all OpenSCENARIO XSD choice groups, starting with the immediate `Deterministic` parsing issue and extending to a complete solution.

## Current Problem Context

### Root Issue
- **XSD Schema**: Choice groups with `maxOccurs="unbounded"` allow mixed element types in any order
- **Serde Limitation**: Cannot handle complex nested XML structures in choice group semantics
- **Immediate Impact**: ALKS scenario parsing fails with "invalid type: map, expected a sequence"

### Failed Approaches
1. `OneOrMany<T>` enum wrapper - fails on complex nested structures
2. Custom `MapAccess` deserializer - still hits serde XML parsing limits
3. Manual XML event parsing - too complex for targeted fix

## Implementation Strategy

### Phase 1: Core Infrastructure (Foundation)
**Timeline**: 1-2 weeks  
**Priority**: Critical

#### 1.1 Choice Group Detection System
```rust
// New module: src/parser/choice_groups.rs

/// Trait for types that represent XSD choice groups
pub trait XsdChoiceGroup {
    type Variant;
    
    /// Get the XML element names that are part of this choice group
    fn choice_element_names() -> &'static [&'static str];
    
    /// Parse a single choice element from XML
    fn parse_choice_element(element_name: &str, xml: &str) -> Result<Self::Variant>;
    
    /// Combine multiple choice variants into the final type
    fn from_choice_variants(variants: Vec<Self::Variant>) -> Result<Self>;
}

/// Registry of all choice groups in the schema
pub struct ChoiceGroupRegistry {
    groups: HashMap<&'static str, Box<dyn XsdChoiceGroupHandler>>,
}

impl ChoiceGroupRegistry {
    pub fn register_deterministic() {
        // Register Deterministic choice group
    }
    
    pub fn register_entity_action() {
        // Register EntityAction choice group
    }
    
    // ... other choice groups
}
```

#### 1.2 Raw XML Event Parser
```rust
// New module: src/parser/xml_events.rs

/// XML event-based parser for choice groups
pub struct ChoiceGroupParser<R: BufRead> {
    reader: Reader<R>,
    buf: Vec<u8>,
}

impl<R: BufRead> ChoiceGroupParser<R> {
    /// Parse a choice group element by delegating to appropriate handlers
    pub fn parse_choice_group<T: XsdChoiceGroup>(
        &mut self,
        container_element: &str
    ) -> Result<T> {
        let mut variants = Vec::new();
        
        loop {
            self.buf.clear();
            match self.reader.read_event_into(&mut self.buf)? {
                Event::Start(e) => {
                    let element_name = std::str::from_utf8(e.name().as_ref())?;
                    
                    if T::choice_element_names().contains(&element_name) {
                        let element_xml = self.read_complete_element(element_name)?;
                        let variant = T::parse_choice_element(element_name, &element_xml)?;
                        variants.push(variant);
                    } else {
                        // Skip unknown elements
                        self.reader.read_to_end_into(e.name(), &mut self.buf)?;
                    }
                }
                Event::End(e) if e.name().as_ref() == container_element.as_bytes() => {
                    break;
                }
                _ => {}
            }
        }
        
        T::from_choice_variants(variants)
    }
    
    fn read_complete_element(&mut self, element_name: &str) -> Result<String> {
        // Implementation to read complete XML element as string
        // for serde deserialization of individual elements
    }
}
```

#### 1.3 Custom Deserialize Integration
```rust
// Integration with serde system
impl<'de> Deserialize<'de> for Deterministic {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use custom XML event parser for choice groups
        // Fall back to standard serde for non-choice elements
        
        use serde::de::{Error, Visitor};
        
        struct DeterministicVisitor;
        
        impl<'de> Visitor<'de> for DeterministicVisitor {
            type Value = Deterministic;
            
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a Deterministic choice group")
            }
            
            fn visit_str<E>(self, xml: &str) -> std::result::Result<Self::Value, E>
            where
                E: Error,
            {
                // Parse using choice group parser
                let mut parser = ChoiceGroupParser::from_str(xml);
                parser.parse_choice_group::<Deterministic>("Deterministic")
                    .map_err(Error::custom)
            }
        }
        
        deserializer.deserialize_str(DeterministicVisitor)
    }
}
```

### Phase 2: Deterministic Choice Group (Priority Fix)
**Timeline**: 3-5 days  
**Priority**: High

#### 2.1 Deterministic Implementation
```rust
// Update: src/types/distributions/deterministic.rs

#[derive(Debug, Clone, PartialEq)]
pub struct Deterministic {
    pub distributions: Vec<DeterministicParameterDistribution>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeterministicParameterDistribution {
    Single(DeterministicSingleParameterDistribution),
    Multi(DeterministicMultiParameterDistribution),
}

impl XsdChoiceGroup for Deterministic {
    type Variant = DeterministicParameterDistribution;
    
    fn choice_element_names() -> &'static [&'static str] {
        &["DeterministicSingleParameterDistribution", "DeterministicMultiParameterDistribution"]
    }
    
    fn parse_choice_element(element_name: &str, xml: &str) -> Result<Self::Variant> {
        match element_name {
            "DeterministicSingleParameterDistribution" => {
                let single: DeterministicSingleParameterDistribution = quick_xml::de::from_str(xml)?;
                Ok(DeterministicParameterDistribution::Single(single))
            }
            "DeterministicMultiParameterDistribution" => {
                let multi: DeterministicMultiParameterDistribution = quick_xml::de::from_str(xml)?;
                Ok(DeterministicParameterDistribution::Multi(multi))
            }
            _ => Err(Error::parsing_error(&format!("Unknown choice element: {}", element_name), 0, 0))
        }
    }
    
    fn from_choice_variants(variants: Vec<Self::Variant>) -> Result<Self> {
        Ok(Deterministic {
            distributions: variants
        })
    }
}

// Maintain backward compatibility
impl Deterministic {
    pub fn single_distributions(&self) -> Vec<&DeterministicSingleParameterDistribution> {
        self.distributions.iter().filter_map(|d| match d {
            DeterministicParameterDistribution::Single(s) => Some(s),
            _ => None,
        }).collect()
    }
    
    pub fn multi_distributions(&self) -> Vec<&DeterministicMultiParameterDistribution> {
        self.distributions.iter().filter_map(|d| match d {
            DeterministicParameterDistribution::Multi(m) => Some(m),
            _ => None,
        }).collect()
    }
}
```

#### 2.2 Testing & Validation
```rust
// New test: tests/deterministic_choice_group_test.rs

#[test]
fn test_alks_scenario_deterministic_parsing() {
    let xml = r#"
    <Deterministic>
        <DeterministicMultiParameterDistribution>
            <ValueSetDistribution>
                <ParameterValueSet>
                    <ParameterAssignment parameterRef="SideVehicle_InitLateralOffset_m" value="-7" />
                    <ParameterAssignment parameterRef="SideVehicle_FinalLateralOffset_m" value="-1.75" />
                </ParameterValueSet>
                <ParameterValueSet>
                    <ParameterAssignment parameterRef="SideVehicle_InitLateralOffset_m" value="7" />
                    <ParameterAssignment parameterRef="SideVehicle_FinalLateralOffset_m" value="1.75" />
                </ParameterValueSet>
            </ValueSetDistribution>
        </DeterministicMultiParameterDistribution>
    </Deterministic>
    "#;
    
    let deterministic: Deterministic = quick_xml::de::from_str(xml).unwrap();
    assert_eq!(deterministic.distributions.len(), 1);
    
    // Verify ALKS scenario file parses
    let scenario = openscenario_rs::parser::xml::parse_from_file(
        "./xosc/alks_scenario_4_6_2_lateral_detection_range_variation.xosc"
    ).unwrap();
    
    assert!(scenario.parameter_value_distribution.is_some());
}

#[test]
fn test_mixed_deterministic_distributions() {
    let xml = r#"
    <Deterministic>
        <DeterministicSingleParameterDistribution parameterName="speed">
            <DistributionSet>
                <Element value="30.0"/>
                <Element value="50.0"/>
            </DistributionSet>
        </DeterministicSingleParameterDistribution>
        <DeterministicMultiParameterDistribution>
            <ValueSetDistribution>
                <ParameterValueSet>
                    <ParameterAssignment parameterRef="x" value="1"/>
                    <ParameterAssignment parameterRef="y" value="2"/>
                </ParameterValueSet>
            </ValueSetDistribution>
        </DeterministicMultiParameterDistribution>
        <DeterministicSingleParameterDistribution parameterName="acceleration">
            <DistributionRange>
                <Range lowerLimit="1.0" upperLimit="5.0"/>
                <StepWidth value="0.5"/>
            </DistributionRange>
        </DeterministicSingleParameterDistribution>
    </Deterministic>
    "#;
    
    let deterministic: Deterministic = quick_xml::de::from_str(xml).unwrap();
    assert_eq!(deterministic.distributions.len(), 3);
    assert_eq!(deterministic.single_distributions().len(), 2);
    assert_eq!(deterministic.multi_distributions().len(), 1);
}
```

### Phase 3: Additional Choice Groups (Scalability)
**Timeline**: 2-3 weeks  
**Priority**: Medium

#### 3.1 Identify All XSD Choice Groups
Audit OpenSCENARIO XSD schema for all choice groups:

1. **Action Choice Groups**:
   - `UserDefinedAction`
   - `PrivateAction` 
   - `EntityAction`
   - `EnvironmentAction`

2. **Condition Choice Groups**:
   - `ConditionGroup`
   - `EntityCondition`
   - `ValueCondition`
   - `TrafficSignalCondition`

3. **Entity Choice Groups**:
   - `EntityObject` (Vehicle, Pedestrian, MiscObject)
   - `CatalogReference` vs direct definitions

4. **Position Choice Groups**:
   - `Position` (WorldPosition, RelativeWorldPosition, etc.)

#### 3.2 Implement High-Priority Choice Groups
```rust
// Example: EntityAction choice group
impl XsdChoiceGroup for EntityAction {
    type Variant = EntityActionVariant;
    
    fn choice_element_names() -> &'static [&'static str] {
        &[
            "UserDefinedAction",
            "PrivateAction", 
            "ControllerAction",
            "AppearanceAction"
        ]
    }
    
    fn parse_choice_element(element_name: &str, xml: &str) -> Result<Self::Variant> {
        match element_name {
            "UserDefinedAction" => {
                let action: UserDefinedAction = quick_xml::de::from_str(xml)?;
                Ok(EntityActionVariant::UserDefined(action))
            }
            "PrivateAction" => {
                let action: PrivateAction = quick_xml::de::from_str(xml)?;
                Ok(EntityActionVariant::Private(action))
            }
            // ... other variants
            _ => Err(Error::parsing_error(&format!("Unknown EntityAction: {}", element_name), 0, 0))
        }
    }
    
    fn from_choice_variants(variants: Vec<Self::Variant>) -> Result<Self> {
        // EntityAction typically contains exactly one choice
        if variants.len() == 1 {
            Ok(EntityAction { variant: variants.into_iter().next().unwrap() })
        } else {
            Err(Error::validation_error("EntityAction", "Must contain exactly one action type"))
        }
    }
}
```

#### 3.3 Choice Group Registry
```rust
// src/parser/choice_groups.rs

lazy_static! {
    static ref CHOICE_GROUP_REGISTRY: ChoiceGroupRegistry = {
        let mut registry = ChoiceGroupRegistry::new();
        
        // Register all choice groups
        registry.register::<Deterministic>("Deterministic");
        registry.register::<EntityAction>("EntityAction");
        registry.register::<EntityCondition>("EntityCondition");
        registry.register::<Position>("Position");
        // ... etc
        
        registry
    };
}

pub fn parse_choice_group<T: XsdChoiceGroup>(container_name: &str, xml: &str) -> Result<T> {
    CHOICE_GROUP_REGISTRY.parse::<T>(container_name, xml)
}
```

### Phase 4: Performance & Optimization
**Timeline**: 1 week  
**Priority**: Low

#### 4.1 Benchmarking
```rust
// benches/choice_group_parsing.rs

#[bench]
fn bench_deterministic_original_vs_choice_group(b: &mut Bencher) {
    let xml = include_str!("../test_data/complex_deterministic.xml");
    
    b.iter(|| {
        let _: Deterministic = quick_xml::de::from_str(xml).unwrap();
    });
}

#[bench]
fn bench_large_scenario_parsing(b: &mut Bencher) {
    let scenario_xml = include_str!("../xosc/alks_scenario_4_6_2_lateral_detection_range_variation.xosc");
    
    b.iter(|| {
        let _: OpenScenario = openscenario_rs::parser::xml::parse_from_str(scenario_xml).unwrap();
    });
}
```

#### 4.2 Optimization Strategies
1. **XML Element Caching**: Cache parsed XML elements for reuse
2. **Lazy Parsing**: Only parse choice elements when accessed
3. **Memory Pool**: Reuse XML parsing buffers
4. **Selective Registration**: Only register choice groups that are actually used

### Phase 5: Documentation & Migration
**Timeline**: 3-5 days  
**Priority**: Medium

#### 5.1 Developer Documentation
```markdown
# XSD Choice Group Handling

## Usage

For types that represent XSD choice groups, implement the `XsdChoiceGroup` trait:

\`\`\`rust
impl XsdChoiceGroup for MyChoiceType {
    type Variant = MyVariant;
    
    fn choice_element_names() -> &'static [&'static str] {
        &["ElementA", "ElementB", "ElementC"]
    }
    
    fn parse_choice_element(element_name: &str, xml: &str) -> Result<Self::Variant> {
        // Parse individual elements
    }
    
    fn from_choice_variants(variants: Vec<Self::Variant>) -> Result<Self> {
        // Combine variants into final type
    }
}
\`\`\`

## Automatic Deserialization

The custom `Deserialize` implementation will automatically use choice group parsing:

\`\`\`rust
let my_type: MyChoiceType = quick_xml::de::from_str(xml)?;
\`\`\`
```

#### 5.2 Migration Guide
```markdown
# Migration Guide: Choice Groups

## Breaking Changes

### Deterministic Struct
- **Before**: `single_distributions: Vec<...>`, `multi_distributions: Vec<...>`
- **After**: `distributions: Vec<DeterministicParameterDistribution>`

### Backward Compatibility
Helper methods maintain API compatibility:
- `deterministic.single_distributions()` - returns filtered view
- `deterministic.multi_distributions()` - returns filtered view

### Migration Steps
1. Update imports if using internal distribution types
2. Use new unified `distributions` field for new code
3. Legacy helper methods continue working
```

## Implementation Timeline

| Phase | Duration | Deliverables | Success Criteria |
|-------|----------|--------------|------------------|
| **Phase 1** | 1-2 weeks | Core infrastructure | Choice group trait system working |
| **Phase 2** | 3-5 days | Deterministic fix | ALKS scenario parsing successful |
| **Phase 3** | 2-3 weeks | All choice groups | Full XSD choice group coverage |
| **Phase 4** | 1 week | Optimization | No performance regression |
| **Phase 5** | 3-5 days | Documentation | Complete developer docs |

**Total Timeline**: 4-6 weeks

## Risk Assessment

### High Risk
- **Serde Integration Complexity**: Custom deserializers may have edge cases
- **Performance Impact**: XML event parsing may be slower than serde
- **Breaking Changes**: API changes may affect existing consumers

### Mitigation Strategies
- **Extensive Testing**: Comprehensive test suite with real OpenSCENARIO files
- **Backward Compatibility**: Maintain helper methods for legacy API
- **Gradual Rollout**: Implement choice groups incrementally
- **Performance Monitoring**: Benchmark at each phase

### Low Risk
- **XSD Compliance**: Schema-driven approach ensures correctness
- **Incremental Implementation**: Can deliver value at each phase
- **Clear Architecture**: Well-defined interfaces and separation of concerns

## Success Metrics

### Phase 2 Success (Immediate)
- ✅ ALKS scenario `alks_scenario_4_6_2_lateral_detection_range_variation.xosc` parses successfully
- ✅ All existing deterministic parsing tests continue to pass
- ✅ Mixed deterministic distributions work correctly

### Full Implementation Success
- ✅ All OpenSCENARIO XSD choice groups implemented
- ✅ Full schema compliance for choice group semantics
- ✅ No performance regression compared to current implementation
- ✅ Comprehensive test coverage for all choice group combinations
- ✅ Complete developer documentation and migration guide

## Alternative Approaches Considered

### 1. Schema Generation Approach
**Pros**: Automatic code generation, perfect XSD compliance  
**Cons**: Complex tooling, loss of manual control, potential code bloat  
**Decision**: Rejected due to complexity and maintenance concerns

### 2. XML-First Parsing
**Pros**: Complete control over XML parsing  
**Cons**: Requires reimplementing all serde functionality  
**Decision**: Rejected as too disruptive

### 3. Serde Extension/Fork
**Pros**: Fixes root issue in serde XML handling  
**Cons**: Maintenance burden, community adoption challenges  
**Decision**: Rejected due to scope and maintenance

## Conclusion

This implementation plan provides a **comprehensive, incremental approach** to achieving full XSD choice group compliance in OpenSCENARIO-rs. The design emphasizes:

- **Immediate Value**: Fix ALKS parsing issue in Phase 2
- **Scalability**: Architecture supports all future choice groups
- **Backward Compatibility**: Existing code continues working
- **Performance**: No significant overhead compared to current approach
- **Maintainability**: Clear interfaces and extensive documentation

The phased approach allows for **early delivery of critical fixes** while building toward **complete schema compliance**.