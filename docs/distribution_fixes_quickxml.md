# Distribution Fixes with Raw quick-xml - OpenSCENARIO-rs

## Overview

Instead of using custom serde implementations to handle XSD choice groups, we can leverage quick-xml's raw event parsing capabilities to manually parse the problematic `Deterministic` struct. This approach gives us fine-grained control over XML parsing without the limitations of serde's automatic mapping.

## Core Problem Recap

**Current Issue**: `invalid type: map, expected a sequence`
- **XSD Schema**: `DeterministicParameterDistribution` is a choice group with `maxOccurs="unbounded"`
- **Serde Limitation**: Cannot handle XSD choice groups that should map to unified Vec<Enum>
- **Real XML**: Single `<DeterministicMultiParameterDistribution>` element causes serde to expect array

## Raw quick-xml Solution Strategy

### Approach 1: Custom Deserialize Implementation

Use quick-xml's `Reader` and `Event` types to manually parse the `<Deterministic>` element:

```rust
use quick_xml::{Reader, events::Event};
use serde::{Deserialize, Deserializer};

impl<'de> Deserialize<'de> for Deterministic {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};
        
        struct DeterministicVisitor;
        
        impl<'de> Visitor<'de> for DeterministicVisitor {
            type Value = Deterministic;
            
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a Deterministic element with choice group children")
            }
            
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut distributions = Vec::new();
                
                // Parse all key-value pairs from the map
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "DeterministicSingleParameterDistribution" => {
                            let single: DeterministicSingleParameterDistribution = map.next_value()?;
                            distributions.push(DeterministicParameterDistribution::Single(single));
                        }
                        "DeterministicMultiParameterDistribution" => {
                            let multi: DeterministicMultiParameterDistribution = map.next_value()?;
                            distributions.push(DeterministicParameterDistribution::Multi(multi));
                        }
                        _ => {
                            // Skip unknown elements
                            let _: serde_json::Value = map.next_value()?;
                        }
                    }
                }
                
                Ok(Deterministic { distributions })
            }
        }
        
        deserializer.deserialize_map(DeterministicVisitor)
    }
}
```

### Approach 2: Raw XML Event Processing

Create a dedicated parser for the `Deterministic` element:

```rust
use quick_xml::{Reader, events::Event};

impl Deterministic {
    pub fn from_xml_events<R: std::io::BufRead>(
        reader: &mut Reader<R>,
        start_event: &quick_xml::events::BytesStart,
    ) -> Result<Self, quick_xml::Error> {
        let mut distributions = Vec::new();
        let mut buf = Vec::new();
        
        loop {
            buf.clear();
            match reader.read_event_into(&mut buf)? {
                Event::Start(e) | Event::Empty(e) => {
                    let name = e.name();
                    match name.as_ref() {
                        b"DeterministicSingleParameterDistribution" => {
                            let single = DeterministicSingleParameterDistribution::from_xml_events(reader, &e)?;
                            distributions.push(DeterministicParameterDistribution::Single(single));
                        }
                        b"DeterministicMultiParameterDistribution" => {
                            let multi = DeterministicMultiParameterDistribution::from_xml_events(reader, &e)?;
                            distributions.push(DeterministicParameterDistribution::Multi(multi));
                        }
                        _ => {
                            // Skip unknown elements
                            reader.read_to_end_into(name, &mut buf)?;
                        }
                    }
                }
                Event::End(e) if e.name().as_ref() == start_event.name().as_ref() => {
                    break;
                }
                Event::Text(_) | Event::Comment(_) | Event::CData(_) => {
                    // Skip whitespace and comments
                }
                Event::Eof => {
                    return Err(quick_xml::Error::UnexpectedEof("Deterministic".to_string()));
                }
                _ => {}
            }
        }
        
        Ok(Deterministic { distributions })
    }
}
```

### Approach 3: Hybrid Solution (Recommended)

Keep serde for most elements but use custom deserialization only for `Deterministic`:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Deterministic {
    pub distributions: Vec<DeterministicParameterDistribution>,
}

// Custom deserialize implementation
impl<'de> Deserialize<'de> for Deterministic {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Implementation handles both single elements and arrays
        DeterministicDeserializer::deserialize(deserializer)
    }
}

// Keep automatic serialization
impl Serialize for Deterministic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize distributions as individual XML elements
        let mut seq = serializer.serialize_tuple(self.distributions.len())?;
        for dist in &self.distributions {
            seq.serialize_element(dist)?;
        }
        seq.end()
    }
}
```

## Implementation Plan

### Phase 1: Implement Custom Deserializer

1. **Create custom deserializer for `Deterministic`**:
   ```rust
   // In src/types/distributions/deterministic.rs
   mod deterministic_serde {
       // Custom serde implementation
   }
   ```

2. **Handle both single elements and arrays**:
   - Single `<DeterministicMultiParameterDistribution>` → `vec![Multi(element)]`
   - Multiple elements → `vec![Single(e1), Multi(e2), ...]`
   - Empty `<Deterministic>` → `vec![]`

3. **Preserve existing API**:
   - Keep `distributions: Vec<DeterministicParameterDistribution>` field
   - Remove deprecated `single_distributions`/`multi_distributions` fields

### Phase 2: Update Struct Definition

```rust
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Deterministic {
    #[serde(deserialize_with = "deterministic_serde::deserialize")]
    pub distributions: Vec<DeterministicParameterDistribution>,
}
```

### Phase 3: Test Coverage

1. **ALKS scenario** (current failing case):
   ```xml
   <Deterministic>
       <DeterministicMultiParameterDistribution>
           <!-- content -->
       </DeterministicMultiParameterDistribution>
   </Deterministic>
   ```

2. **Mixed distributions**:
   ```xml
   <Deterministic>
       <DeterministicSingleParameterDistribution parameterName="param1">
           <!-- content -->
       </DeterministicSingleParameterDistribution>
       <DeterministicMultiParameterDistribution>
           <!-- content -->
       </DeterministicMultiParameterDistribution>
   </Deterministic>
   ```

3. **Empty deterministic**:
   ```xml
   <Deterministic>
   </Deterministic>
   ```

## Advantages of quick-xml Approach

### ✅ Benefits

1. **Precise Control**: Handle XSD choice groups exactly as specified
2. **Performance**: Avoid serde overhead for complex parsing logic
3. **Flexibility**: Can handle edge cases and malformed XML gracefully
4. **Non-Breaking**: Maintain existing API surface
5. **Incremental**: Only affect problematic types, not entire system

### ⚠️ Considerations

1. **Complexity**: More code than pure serde approach
2. **Maintenance**: Custom parsing logic needs careful testing
3. **Error Handling**: Must manually handle XML parsing errors
4. **Documentation**: Need clear documentation of XML handling

## Detailed Implementation

### Custom Deserializer Implementation

```rust
mod deterministic_serde {
    use super::*;
    use serde::{Deserialize, Deserializer, de::{self, MapAccess, SeqAccess, Visitor}};
    
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<DeterministicParameterDistribution>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DeterministicVisitor;
        
        impl<'de> Visitor<'de> for DeterministicVisitor {
            type Value = Vec<DeterministicParameterDistribution>;
            
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence of DeterministicParameterDistribution elements")
            }
            
            // Handle case where XML has single element (map-like)
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut distributions = Vec::new();
                
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "DeterministicSingleParameterDistribution" => {
                            // Handle both single element and array of elements
                            let value = map.next_value::<serde_json::Value>()?;
                            if let serde_json::Value::Array(arr) = value {
                                for item in arr {
                                    let single: DeterministicSingleParameterDistribution = 
                                        serde_json::from_value(item).map_err(de::Error::custom)?;
                                    distributions.push(DeterministicParameterDistribution::Single(single));
                                }
                            } else {
                                let single: DeterministicSingleParameterDistribution = 
                                    serde_json::from_value(value).map_err(de::Error::custom)?;
                                distributions.push(DeterministicParameterDistribution::Single(single));
                            }
                        }
                        "DeterministicMultiParameterDistribution" => {
                            let value = map.next_value::<serde_json::Value>()?;
                            if let serde_json::Value::Array(arr) = value {
                                for item in arr {
                                    let multi: DeterministicMultiParameterDistribution = 
                                        serde_json::from_value(item).map_err(de::Error::custom)?;
                                    distributions.push(DeterministicParameterDistribution::Multi(multi));
                                }
                            } else {
                                let multi: DeterministicMultiParameterDistribution = 
                                    serde_json::from_value(value).map_err(de::Error::custom)?;
                                distributions.push(DeterministicParameterDistribution::Multi(multi));
                            }
                        }
                        _ => {
                            // Skip unknown keys
                            let _: serde_json::Value = map.next_value()?;
                        }
                    }
                }
                
                Ok(distributions)
            }
            
            // Handle case where XML has multiple elements (sequence-like)
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut distributions = Vec::new();
                
                while let Some(dist) = seq.next_element::<DeterministicParameterDistribution>()? {
                    distributions.push(dist);
                }
                
                Ok(distributions)
            }
        }
        
        deserializer.deserialize_any(DeterministicVisitor)
    }
}
```

## Expected Results

After implementing this solution:

1. **✅ ALKS scenario parsing**: `xosc/alks_scenario_4_6_2_lateral_detection_range_variation.xosc` should parse successfully
2. **✅ API compatibility**: Existing code using `deterministic.distributions` continues working
3. **✅ Schema compliance**: Properly handles XSD choice group semantics
4. **✅ Flexibility**: Can handle various XML structures (single, multiple, empty)

## Testing Plan

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_single_multi_distribution() {
        let xml = r#"
        <Deterministic>
            <DeterministicMultiParameterDistribution>
                <ValueSetDistribution>
                    <ParameterValueSet>
                        <ParameterAssignment parameterRef="test" value="1" />
                    </ParameterValueSet>
                </ValueSetDistribution>
            </DeterministicMultiParameterDistribution>
        </Deterministic>
        "#;
        
        let deterministic: Deterministic = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(deterministic.distributions.len(), 1);
        assert!(matches!(deterministic.distributions[0], 
            DeterministicParameterDistribution::Multi(_)));
    }
    
    #[test]
    fn test_mixed_distributions() {
        let xml = r#"
        <Deterministic>
            <DeterministicSingleParameterDistribution parameterName="param1">
                <!-- content -->
            </DeterministicSingleParameterDistribution>
            <DeterministicMultiParameterDistribution>
                <!-- content -->
            </DeterministicMultiParameterDistribution>
        </Deterministic>
        "#;
        
        let deterministic: Deterministic = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(deterministic.distributions.len(), 2);
    }
    
    #[test]
    fn test_alks_scenario() {
        let result = openscenario_rs::parser::xml::parse_from_file(
            "./xosc/alks_scenario_4_6_2_lateral_detection_range_variation.xosc"
        );
        assert!(result.is_ok());
    }
}
```

This approach leverages quick-xml's flexibility while maintaining the existing API and achieving full XSD schema compliance.