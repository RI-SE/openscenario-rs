# Phase 4: Full Reference Resolution Plan

## Overview
Phase 4 implements complete catalog reference resolution, enabling scenarios to load and integrate external catalog content dynamically. This builds on the foundation established in Phases 1-3 to create a production-ready catalog system.

## Current Foundation (Phases 1-3)
✅ **Completed Infrastructure:**
- Directory type with path validation and parameter support
- Catalog infrastructure (loader, resolver, caching)
- 8 catalog location types with comprehensive testing
- Error handling and circular dependency detection framework
- Async-ready architecture for file operations

## Phase 4 Scope

### 4.1 Core Entity Types & Catalog Content 🎯
**Implement the actual catalog entity types that references resolve to**

#### 4.1.1 Vehicle Catalog Entities
- `CatalogVehicle` - Complete vehicle definition from catalogs
- Integrate with existing `Vehicle` type in `src/types/entities/vehicle.rs`
- Support parameter substitution for vehicle properties

#### 4.1.2 Controller Catalog Entities
- `CatalogController` - Controller definitions from catalogs
- Integrate with existing controller types in `src/types/controllers/mod.rs`
- Support dynamic controller property resolution

#### 4.1.3 Additional Catalog Entity Types
- `CatalogPedestrian` - Pedestrian definitions
- `CatalogMiscObject` - Miscellaneous object definitions
- `CatalogEnvironment` - Environment configurations
- `CatalogManeuver` - Reusable maneuver definitions
- `CatalogTrajectory` - Trajectory patterns
- `CatalogRoute` - Route definitions

### 4.2 Reference Resolution Engine 🔧
**Transform catalog references into actual resolved entities**

#### 4.2.1 Enhanced CatalogReference Types
```rust
// Current: Basic reference in controllers/mod.rs
pub struct CatalogReference {
    pub catalog_name: Value<String>,
    pub entry_name: Value<String>,
}

// Enhanced: Generic reference with parameter support
pub struct CatalogReference<T> {
    pub catalog_name: Value<String>,
    pub entry_name: Value<String>,
    pub parameters: HashMap<String, Value<String>>,
    phantom: std::marker::PhantomData<T>,
}
```

#### 4.2.2 Resolution Trait Implementation
```rust
#[async_trait]
impl<T> CatalogReference<T> 
where 
    T: CatalogEntity + DeserializeOwned 
{
    async fn resolve(&self, manager: &CatalogManager) -> Result<ResolvedCatalog<T>>;
}
```

#### 4.2.3 Parameter Substitution Engine
- Parse parameter expressions in catalog content
- Substitute parameter values during resolution
- Validate parameter types and constraints
- Support nested parameter references

### 4.3 Catalog File Parsing & Loading 📄
**Parse actual XOSC catalog files and extract entity definitions**

#### 4.3.1 XML Catalog Parser
- Extend existing parser in `src/parser/xml.rs`
- Support catalog-specific XML structures
- Handle catalog parameter declarations
- Parse entity definitions within catalogs

#### 4.3.2 Catalog Content Models
```rust
// Catalog file structure
pub struct CatalogFile {
    pub file_info: FileHeader,
    pub parameter_declarations: Vec<ParameterDeclaration>,
    pub vehicles: Option<VehicleCatalog>,
    pub controllers: Option<ControllerCatalog>,
    pub pedestrians: Option<PedestrianCatalog>,
    // ... other catalog types
}

pub struct VehicleCatalog {
    pub vehicles: Vec<CatalogVehicle>,
}
```

#### 4.3.3 Lazy Loading Strategy
- Load catalog files on-demand when references are resolved
- Cache parsed catalog content efficiently
- Support partial catalog loading for large files
- Handle catalog file dependencies

### 4.4 Integration with Existing Types 🔗
**Seamlessly integrate resolved catalog content with scenario entities**

#### 4.4.1 Entity Trait Unification
```rust
pub trait CatalogEntity {
    type ResolvedType;
    
    fn into_scenario_entity(self, parameters: HashMap<String, String>) -> Result<Self::ResolvedType>;
    fn parameter_schema() -> Vec<ParameterDefinition>;
}
```

#### 4.4.2 Scenario-Level Resolution
- Implement `ScenarioResolver` trait for main scenario types
- Recursive resolution of nested catalog references
- Maintain resolution context and parameter scope
- Support resolution rollback on errors

#### 4.4.3 Type-Safe Resolution APIs
```rust
// Type-safe resolution methods
impl ScenarioObject {
    pub async fn resolve_vehicle_catalog(&mut self, manager: &CatalogManager) -> Result<()>;
    pub async fn resolve_controller_catalog(&mut self, manager: &CatalogManager) -> Result<()>;
}
```

### 4.5 Advanced Resolution Features ⚡
**Production-grade catalog system capabilities**

#### 4.5.1 Multi-Catalog Dependencies
- Support catalogs that reference other catalogs
- Dependency graph construction and validation
- Circular dependency prevention across catalog files
- Version compatibility checking

#### 4.5.2 Parameter System Enhancement
- Parameter inheritance from parent scopes
- Default parameter values from catalog definitions
- Parameter validation against catalog schemas
- Dynamic parameter evaluation

#### 4.5.3 Conditional Resolution
- Support conditional catalog loading based on parameters
- Environment-specific catalog selection
- Platform-dependent entity resolution

### 4.6 Performance & Reliability 🚀
**Production-ready performance and error handling**

#### 4.6.1 Advanced Caching
- Multi-level caching (file, parsed content, resolved entities)
- Cache invalidation on file changes
- Memory-efficient caching strategies
- Cache statistics and monitoring

#### 4.6.2 Error Recovery
- Graceful handling of missing catalog files
- Partial resolution with fallback entities
- Detailed error reporting with resolution context
- Recovery strategies for corrupted catalog files

#### 4.6.3 Async Performance
- Concurrent catalog loading and parsing
- Parallel resolution of independent references
- Streaming for large catalog files
- Resource-aware loading strategies

## Implementation Tasks

### 4.1 Core Entity Types (4 weeks)
- [ ] **4.1.1** Define CatalogVehicle and integration (1 week)
- [ ] **4.1.2** Define CatalogController and integration (1 week)  
- [ ] **4.1.3** Implement remaining catalog entity types (2 weeks)

### 4.2 Reference Resolution Engine (3 weeks)
- [ ] **4.2.1** Enhanced CatalogReference with generics (1 week)
- [ ] **4.2.2** Core resolution trait implementation (1 week)
- [ ] **4.2.3** Parameter substitution engine (1 week)

### 4.3 Catalog File Parsing (3 weeks)
- [ ] **4.3.1** XML catalog parser extension (1 week)
- [ ] **4.3.2** Catalog content models (1 week)
- [ ] **4.3.3** Lazy loading implementation (1 week)

### 4.4 Type Integration (2 weeks)
- [ ] **4.4.1** CatalogEntity trait system (1 week)
- [ ] **4.4.2** Scenario-level resolution (1 week)

### 4.5 Advanced Features (4 weeks)
- [ ] **4.5.1** Multi-catalog dependencies (2 weeks)
- [ ] **4.5.2** Enhanced parameter system (1 week)
- [ ] **4.5.3** Conditional resolution (1 week)

### 4.6 Production Readiness (2 weeks)
- [ ] **4.6.1** Advanced caching system (1 week)
- [ ] **4.6.2** Error recovery & async performance (1 week)

## Success Criteria

### Functional Requirements ✅
- [ ] All 8 catalog types can be loaded and parsed from XOSC files
- [ ] Catalog references resolve to actual typed entities
- [ ] Parameter substitution works correctly in all contexts
- [ ] Circular dependencies are detected and prevented
- [ ] Multi-catalog scenarios work seamlessly

### Performance Requirements 📊
- [ ] Catalog loading: <100ms for typical catalog files (<1MB)
- [ ] Reference resolution: <10ms per reference with caching
- [ ] Memory usage: <50MB for 1000 catalog entities
- [ ] Concurrent loading: Support 10+ catalogs simultaneously

### Quality Requirements 🔍
- [ ] 95%+ test coverage for all resolution code paths
- [ ] Integration tests with real XOSC catalog files
- [ ] Error handling covers all failure scenarios
- [ ] Documentation with complete API examples

## Integration Points

### With Existing Systems 🔌
- **Parser Integration**: Extend `src/parser/` for catalog-specific XML
- **Validation System**: Add catalog reference validation to `src/parser/validation.rs`
- **Expression Engine**: Integrate parameter substitution with `src/expression.rs`
- **Error System**: Extend error types in `src/error.rs` for catalog-specific errors

### API Consistency 📋
- Follow existing naming conventions (snake_case functions, PascalCase types)
- Use `Result<T>` error handling pattern consistently
- Implement `Debug`, `Clone`, `Serialize`, `Deserialize` for all types
- Use `#[cfg(feature)]` for optional catalog features

## Risk Mitigation

### Technical Risks ⚠️
- **Complex Parameter Resolution**: Implement incremental parameter system
- **Circular Dependencies**: Use proven graph algorithms for detection
- **Memory Usage**: Implement streaming and efficient caching
- **File I/O Errors**: Comprehensive error handling with recovery

### Implementation Risks 📅
- **Scope Creep**: Stick to core resolution functionality first
- **Performance**: Profile early and optimize critical paths
- **Testing Complexity**: Create comprehensive test data sets
- **Integration Challenges**: Maintain backward compatibility

## Deliverables

### Phase 4.1 - Core Foundation (Week 4)
- All catalog entity types implemented and tested
- Basic CatalogReference resolution working
- Integration with existing type system complete

### Phase 4.2 - Full Resolution (Week 8)  
- Complete parameter substitution system
- Multi-catalog dependency resolution
- Production-ready caching and error handling

### Phase 4.3 - Production Ready (Week 12)
- Advanced features (conditional resolution, performance optimization)
- Comprehensive test suite and documentation
- Example applications demonstrating full functionality

## Success Metrics
- **Type Coverage**: Support all 8 catalog types with full resolution
- **Performance**: Handle 10,000+ catalog entities efficiently  
- **Reliability**: Zero catalog-related crashes in test scenarios
- **Usability**: Simple API for common catalog operations
- **Maintainability**: Clear separation of concerns and modular design

---

*This plan transforms the catalog system from a foundation into a production-ready, type-safe reference resolution engine that enables complex, modular OpenSCENARIO development.*