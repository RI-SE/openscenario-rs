# OpenSCENARIO-rs MVP Development Roadmap

## MVP Philosophy

**Goal**: Create the minimal viable product that can parse and represent basic OpenSCENARIO files, proving the core architecture while delivering immediate value to users.

**Success Criteria for MVP**:
- Parse simple, real-world OpenSCENARIO files (ASAM examples)
- Provide type-safe access to parsed data
- Validate basic scenarios
- Support round-trip serialization (parse → modify → save)
- Demonstrate architectural soundness for future expansion

---

## Phase 1: Foundation MVP (Weeks 1-4)

### 1.1 Core Infrastructure (Week 1)
**Priority: CRITICAL** - Nothing works without this

#### Implement First:
```rust
// src/lib.rs - Minimal public API
pub use error::{Error, Result};
pub use types::{OpenScenario, FileHeader};

pub fn parse_file(path: impl AsRef<Path>) -> Result<OpenScenario>;
pub fn parse_str(xml: &str) -> Result<OpenScenario>;
```

#### Files to Implement:
- `src/error.rs` - Core error types only
- `src/types/basic.rs` - String, Double, Int (no expressions yet)
- `src/types/enums.rs` - 5-10 most critical enums only
- `src/parser/xml.rs` - Serde-based XML parsing with quick-xml
- `Cargo.toml` - Minimal dependencies (quick-xml with serialize feature, serde, thiserror)

#### Why Start Here:
- **Proves the architecture works** - Can we actually parse XML into Rust types with serde + quick-xml?
- **Enables testing** - Can write tests immediately
- **Shows progress** - Users can load files and see data structures
- **Validates dependencies** - Ensures chosen libraries work together

### 1.2 Basic Scenario Structure (Week 2)
**Priority: CRITICAL** - Core scenario representation

#### Implement:
```rust
// Minimal scenario structure with serde
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "OpenSCENARIO")]
pub struct OpenScenario {
    #[serde(rename = "FileHeader")]
    pub file_header: FileHeader,
    
    #[serde(rename = "Entities")]
    pub entities: Entities,        // Simplified - just scenario objects
    
    #[serde(rename = "Storyboard")]
    pub storyboard: Storyboard,    // Simplified - init + one story
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileHeader {
    #[serde(rename = "@description")]
    pub description: String,
    #[serde(rename = "@author")]
    pub author: String,
    #[serde(rename = "@revMajor")]
    pub rev_major: u16,
    #[serde(rename = "@revMinor")]
    pub rev_minor: u16,
    // Skip optional fields for MVP
}
```

#### Files to Complete:
- `src/types/scenario/storyboard.rs` - Basic structure only
- `src/types/entities/mod.rs` - Entity reference system
- `tests/integration_tests.rs` - First parsing test

#### Success Metric:
```rust
#[test]
fn can_parse_minimal_scenario() {
    let scenario = openscenario::parse_str(MINIMAL_XML)?;
    assert_eq!(scenario.file_header.author, "Test Author");
    assert!(!scenario.entities.scenario_objects.is_empty());
}
```

### 1.3 Basic Entities (Week 3)
**Priority: HIGH** - Core entity types

#### Implement:
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Vehicle {
    #[serde(rename = "@name")]
    pub name: String,
    
    #[serde(rename = "@vehicleCategory")]
    pub vehicle_category: VehicleCategory,  // Enum with 3-4 variants
    
    #[serde(rename = "BoundingBox")]
    pub bounding_box: BoundingBox,
    // Skip complex fields like Performance, Axles for MVP
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pedestrian {
    #[serde(rename = "@name")]
    pub name: String,
    
    #[serde(rename = "@pedestrianCategory")]
    pub pedestrian_category: PedestrianCategory,
    
    #[serde(rename = "BoundingBox")]
    pub bounding_box: BoundingBox,
    // Minimal implementation for MVP
}
```

#### Files to Complete:
- `src/types/entities/vehicle.rs` - Basic vehicle only
- `src/types/entities/pedestrian.rs` - Basic pedestrian only
- `src/types/geometry/shapes.rs` - BoundingBox only

#### Success Metric:
```rust
#[test]
fn can_access_entity_data() {
    let scenario = openscenario::parse_file("tests/fixtures/simple.xosc")?;
    let vehicle = &scenario.entities.scenario_objects[0];
    assert_eq!(vehicle.name, "Ego");
    // Can access type-safe entity data
}
```

### 1.4 Basic Actions & Conditions (Week 4)
**Priority: MEDIUM** - Minimal scenario behavior

#### Implement Only:
- `SpeedAction` - Most common action
- `TeleportAction` - Simplest positioning
- `SimulationTimeCondition` - Time-based trigger
- `SpeedCondition` - Entity state condition

#### Files to Complete:
- `src/types/actions/movement.rs` - 2 actions only
- `src/types/conditions/value.rs` - 1 condition
- `src/types/conditions/entity.rs` - 1 condition

#### Success Metric:
```rust
#[test]
fn can_parse_basic_scenario_logic() {
    let scenario = openscenario::parse_file("tests/fixtures/simple.xosc")?;
    // Can access actions and conditions in type-safe way
    assert!(scenario.storyboard.stories[0].acts[0]
        .maneuver_groups[0].maneuvers[0]
        .events[0].actions.len() > 0);
}
```

**MVP Milestone**: Can parse real ASAM example scenarios and access data!

---

## Phase 2: Usability MVP (Weeks 5-8)

### 2.1 Expression System (Week 5)
**Priority: HIGH** - Enables parameter-based scenarios

#### Implement:
```rust
#[derive(Debug, Clone)]
pub enum Value<T> {
    Literal(T),
    Parameter(String),    // ${parameterName} 
    // Skip expressions for now
}

// Custom serde deserializer for parameter expressions
impl<'de, T> Deserialize<'de> for Value<T>
where
    T: Deserialize<'de> + FromStr,
    T::Err: std::fmt::Display,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Handle ${param} vs literal values during deserialization
    }
}

impl<T> Value<T> where T: Clone {
    pub fn resolve(&self, params: &HashMap<String, String>) -> Result<T>;
}
```

#### Why Now:
- Many real scenarios use parameters
- Unlocks more ASAM examples
- Shows advanced type system working

### 2.2 Validation Framework (Week 6)
**Priority: HIGH** - Catch invalid scenarios early

#### Implement:
```rust
pub trait Validate {
    fn validate(&self) -> Result<(), ValidationError>;
}

// Basic validations:
// - Required fields present
// - Entity references exist  
// - Basic constraint checking
```

#### Why Now:
- Users need to know if scenarios are valid
- Prevents runtime errors
- Shows quality focus

### 2.3 Builder Pattern Basics (Week 7)
**Priority: MEDIUM** - Programmatic scenario creation

#### Implement:
```rust
pub struct ScenarioBuilder {
    scenario: OpenScenario,
}

impl ScenarioBuilder {
    pub fn new() -> Self;
    pub fn file_header(self, header: FileHeader) -> Self;
    pub fn add_vehicle(self, vehicle: Vehicle) -> Self;
    pub fn build(self) -> Result<OpenScenario>;
}
```

#### Why Now:
- Enables programmatic scenario generation
- Great for testing
- Shows API design working

### 2.4 Serialization (Week 8)
**Priority: HIGH** - Save modified scenarios

#### Implement:
```rust
impl OpenScenario {
    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let xml = quick_xml::se::to_string(self)?;
        std::fs::write(path, xml)?;
        Ok(())
    }
    
    pub fn to_string(&self) -> Result<String> {
        Ok(quick_xml::se::to_string(self)?)
    }
}

// Round-trip validation
#[cfg(test)]
mod tests {
    #[test]
    fn test_round_trip_serialization() {
        let original = openscenario::parse_str(XML_EXAMPLE)?;
        let serialized = original.to_string()?;
        let deserialized = openscenario::parse_str(&serialized)?;
        assert_eq!(original, deserialized);
    }
}
```

#### Why Now:
- Completes the round-trip capability with serde serialization
- Users can modify and save scenarios  
- Enables tooling scenarios
- Validates serde approach for both reading and writing
- Round-trip testing ensures data integrity

**Usability Milestone**: Can parse, validate, modify, and save scenarios!

---

## Phase 3: Ecosystem MVP (Weeks 9-12)

### 3.1 Position System (Week 9)
**Priority: HIGH** - Spatial positioning

#### Implement:
- `WorldPosition` - Absolute coordinates
- `RelativeWorldPosition` - Relative to entity
- `LanePosition` - Road-based positioning

#### Why Now:
- Critical for automotive scenarios
- Unlocks more complex scenarios
- Shows geometric system working

### 3.2 More Actions (Week 10)
**Priority: HIGH** - Expand scenario capabilities  

#### Implement:
- `LaneChangeAction` - Common automotive maneuver
- `FollowTrajectoryAction` - Path following
- `SynchronizeAction` - Entity coordination

#### Why Now:
- Unlocks realistic automotive scenarios
- Shows action system scalability
- High user value

### 3.3 More Conditions (Week 11)
**Priority: HIGH** - Rich trigger system

#### Implement:
- `DistanceCondition` - Spatial triggers
- `CollisionCondition` - Safety conditions  
- `TimeHeadwayCondition` - Following distance

#### Why Now:
- Enables complex scenario logic
- Safety-critical functionality
- High user value

### 3.4 Geometry Basics (Week 12)
**Priority: MEDIUM** - Spatial representations

#### Implement:
- `Polyline` - Simple paths
- Basic trajectory support
- Coordinate transformations

#### Why Now:
- Enables path-based scenarios
- Shows geometric capabilities
- Foundation for advanced features

**Ecosystem Milestone**: Can handle complex automotive scenarios with realistic geometry!

---

## Phase 4: Production Ready + Streaming (Weeks 13-16)

### 4.1 Streaming Parser Addition (Week 13)
**Priority: MEDIUM** - Performance for large files

#### Implement:
```rust
// Add streaming feature flag
[features]
streaming = ["manual-parser"]

// Streaming API alongside standard API
pub fn parse_file(path: impl AsRef<Path>) -> Result<OpenScenario>;          // Serde
pub fn parse_file_streaming(path: impl AsRef<Path>) -> Result<OpenScenario>; // Manual

// Or unified API with config
pub struct ParserConfig {
    pub use_streaming: bool,
    pub max_memory_usage: Option<usize>,
}
```

#### When to Add:
- Users request large file support (>50MB scenarios)
- Performance benchmarks show memory issues
- Real-world use cases require streaming

### 4.2 Error Handling Polish (Week 14)
- Enhanced serde error messages with XML context
- Error recovery strategies for both parsers
- User-friendly diagnostics and suggestions

### 4.3 Documentation & Examples (Week 15)
- Comprehensive API documentation
- Tutorial examples showing both parsing approaches
- Performance guidance (when to use streaming)
- Real-world integration guides

### 4.4 Testing & Quality (Week 16)
- Property-based testing for both parsers
- Real ASAM scenario validation
- Performance comparison benchmarks
- Cross-platform testing

---

## Implementation Strategy

### Development Principles:

1. **Hybrid Serde + Manual Parsing Approach**:
   ```rust
   // Phase 1-3: Serde-based parsing for rapid development
   pub fn parse_file(path: impl AsRef<Path>) -> Result<OpenScenario> {
       let xml = std::fs::read_to_string(path)?;
       Ok(quick_xml::de::from_str(&xml)?)
   }
   
   // Phase 4+: Add streaming option for large files
   #[cfg(feature = "streaming")]
   pub fn parse_file_streaming(path: impl AsRef<Path>) -> Result<OpenScenario> {
       let reader = BufReader::new(File::open(path)?);
       StreamingParser::new(reader).parse()
   }
   ```
   
   **Why Hybrid Approach**:
   - **Speed to Market**: Serde gets us to MVP 3x faster
   - **Type Safety**: Automatic validation through Rust type system
   - **Performance When Needed**: Streaming parser for large files (50MB+)
   - **User Choice**: Standard API for most users, streaming for power users
   - **Lower Risk**: Proven serde approach first, optimization later

2. **Test-First Development**:
   ```rust
   #[test]
   fn test_what_we_want_to_work() {
       // Write the test for desired behavior
       let scenario = openscenario::parse_str(EXAMPLE_XML)?;
       assert_eq!(scenario.entities.len(), 2);
   }
   ```
   Then implement until test passes.

3. **Real Data Validation**:
   - Start with ASAM example scenarios from day 1
   - If we can't parse real scenarios, we're building wrong thing

4. **Incremental Type System**:
   ```rust
   // Week 1: Basic enum
   pub enum VehicleCategory { Car, Truck }
   
   // Week 5: Complete enum
   pub enum VehicleCategory { 
       Car, Truck, Bus, Bicycle, Motorbike, 
       Semitrailer, Trailer, Train, Tram, Van 
   }
   ```

5. **Feature Flags for Stability**:
   ```toml
   [features]
   default = ["serde-parsing"]
   serde-parsing = []  # Standard serde-based parsing
   validation = []
   builder = []
   expressions = []
   streaming = []      # Manual parsing for large files (Phase 4+)
   ```
   Users can choose parsing strategy based on needs.

### Risk Mitigation:

1. **Architecture Validation Early**: Phase 1 proves serde + quick-xml → Rust types works
2. **Real Data Testing**: Use ASAM scenarios from week 1 to validate approach
3. **Incremental Complexity**: Add one concept at a time, serde first then streaming
4. **User Feedback**: Release early with serde, get feedback, add streaming if needed
5. **Performance Monitoring**: Benchmark serde from day 1, add streaming benchmarks later
6. **Proven Technology**: Serde is battle-tested, reduces implementation risk
7. **Migration Path**: Clear path from serde to streaming for performance-critical users

### Success Metrics by Phase:

**Phase 1**: Parse 1 ASAM example scenario
**Phase 2**: Parse 5 ASAM example scenarios + validate them  
**Phase 3**: Parse 20+ ASAM scenarios + build new ones
**Phase 4**: Production-ready with docs and examples

### Hybrid Implementation Strategy:

#### Phase 1-3: Serde-First Approach (Weeks 1-12)
```rust
// src/parser/xml.rs - Simple serde-based parsing
use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};

pub fn parse_scenario(xml: &str) -> Result<OpenScenario> {
    Ok(from_str(xml)?)
}

pub fn serialize_scenario(scenario: &OpenScenario) -> Result<String> {
    Ok(quick_xml::se::to_string(scenario)?)
}

// Custom deserializers for OpenSCENARIO-specific patterns
mod custom_deserializers {
    use serde::{Deserializer, Deserialize};
    
    pub fn deserialize_parameter_or_value<'de, D, T>(
        deserializer: D,
    ) -> Result<Value<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de> + FromStr,
    {
        // Handle ${param} vs literal during deserialization
    }
}
```

#### Phase 4+: Add Streaming Parser (Week 13+)
```rust
// src/parser/streaming.rs - Manual parsing for large files
#[cfg(feature = "streaming")]
pub struct StreamingParser<R: BufRead> {
    reader: Reader<R>,
    buffer: Vec<u8>,
}

#[cfg(feature = "streaming")]
impl<R: BufRead> StreamingParser<R> {
    pub fn parse(&mut self) -> Result<OpenScenario> {
        // Event-driven parsing for memory efficiency
        // Only implemented when streaming feature is enabled
    }
}
```

#### Benefits of Hybrid Approach:
- **Fast MVP**: Serde gets us working parser in days, not weeks
- **Type Safety**: Automatic XML validation through Rust type system
- **Ecosystem**: Works with serde tooling, debugging, and testing
- **Performance Option**: Streaming parser available when needed
- **User Choice**: Standard parsing for most users, streaming for large files

### What NOT to Implement in MVP:

- **Catalogs** - Complex, not needed for basic scenarios
- **Distributions** - Advanced feature for parameter variation  
- **Complex Geometry** - Clothoids, NURBS can wait
- **All Entity Types** - Focus on Vehicle and Pedestrian
- **All Actions** - Start with 3-4 most common
- **Weather/Environment** - Nice-to-have, not critical
- **Complex Validation** - Basic validation first
- **Advanced Builder APIs** - Simple builder first
- **Manual Parsing for All Types** - Start with serde, add streaming selectively

This roadmap balances delivering value quickly with building the right architecture for long-term success. Each phase delivers working software that users can immediately benefit from while building toward the complete vision.