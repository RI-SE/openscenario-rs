# Detailed Agent Implementation Plans for OpenSCENARIO-rs

Based on the comprehensive XSD analysis showing **240/346 types (69.4% complete)**, this document provides detailed implementation plans for completing the remaining **106 missing types** (30.6%) through systematic agent execution.

## Overview: Current Status and Remaining Work

### Current Status: ~290/346 types (83.8% complete) - UPDATED

- ✅ **Basic Data Types**: 9/9 (100%)
- ✅ **Simple Enumeration Types**: 37/37 (100%)
- ✅ **Distributions**: 18/18 (100%)
- ✅ **Controllers**: 8/8 (100%)
- ✅ **Catalogs**: 25/25 (100%)
- ✅ **Groups**: 13/13 (100%) - COMPLETED
- ✅ **Actions**: ~45/48 (94%) - MAJOR PROGRESS
- 📊 **Complex Types**: ~185/287 (64.5%) - SIGNIFICANT PROGRESS

### Recently Completed (Last Week):
- ✅ **Week 1: Lane Actions** - LaneChangeAction, LaneOffsetAction, LateralAction (COMPLETED)
- ✅ **Week 2: Traffic Actions** - TrafficSignalController, TrafficSwarmAction (COMPLETED)
- ✅ **XSD Groups Implementation** - All 13 groups completed (COMPLETED)

### Remaining Work: ~56 types organized by priority

- **High Priority**: ~15 types (critical conditions, enhanced vehicle components)
- **Medium Priority**: ~25 types (advanced simulation features)
- **Low Priority**: ~16 types (specialized use cases)

---

## Phase 1: Critical Missing Types (HIGH PRIORITY - 6 weeks, 23 types)

### Week 1: Lane Actions Implementation (3 types) - ✅ COMPLETED

#### ✅ Agent Task 1.1: LaneChangeAction Implementation - COMPLETED

**Status**: ✅ **COMPLETED** (commit: bf6bd42)
**Agent Type**: `subagents/coder-agent`  
**Duration**: 2 days

**Implementation Status:**

```markdown
## Task: Implement LaneChangeAction System

### XSD Analysis Required:
1. Read `Schema/OpenSCENARIO.xsd` lines 1336-1348 for LaneChangeAction definition
2. Extract all required/optional attributes and child elements
3. Identify dependencies: LaneChangeTarget, TransitionDynamics

### Implementation Steps:

#### Step 1: Create Core Types (src/types/actions/lateral.rs)
```rust
use crate::types::basic::*;
use crate::types::enums::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "LaneChangeAction")]
pub struct LaneChangeAction {
    #[serde(rename = "LaneChangeActionDynamics")]
    pub dynamics: TransitionDynamics,
    
    #[serde(rename = "LaneChangeTarget")]
    pub target: LaneChangeTarget,
    
    #[serde(rename = "@targetLaneOffset")]
    pub target_lane_offset: Option<Double>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LaneChangeTarget {
    #[serde(rename = "RelativeTargetLane")]
    Relative(RelativeTargetLane),
    #[serde(rename = "AbsoluteTargetLane")]
    Absolute(AbsoluteTargetLane),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelativeTargetLane {
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,
    #[serde(rename = "@value")]
    pub value: Int,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbsoluteTargetLane {
    #[serde(rename = "@value")]
    pub value: OSString,
}
```

#### Step 2: Add Helper Methods

```rust
impl LaneChangeAction {
    pub fn new(dynamics: TransitionDynamics, target: LaneChangeTarget) -> Self {
        Self {
            dynamics,
            target,
            target_lane_offset: None,
        }
    }
    
    pub fn with_offset(mut self, offset: Double) -> Self {
        self.target_lane_offset = Some(offset);
        self
    }
}

impl LaneChangeTarget {
    pub fn relative(entity_ref: impl Into<OSString>, value: i32) -> Self {
        Self::Relative(RelativeTargetLane {
            entity_ref: entity_ref.into(),
            value: Value::literal(value),
        })
    }
    
    pub fn absolute(lane_id: impl Into<OSString>) -> Self {
        Self::Absolute(AbsoluteTargetLane {
            value: lane_id.into(),
        })
    }
}
```

#### Step 3: Integration with Action System

```rust
// In src/types/actions/mod.rs - add to PrivateAction enum:
#[serde(rename = "LateralAction")]
LateralAction(LateralAction),

// In src/types/actions/lateral.rs:
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LateralAction {
    #[serde(rename = "LaneChangeAction")]
    LaneChange(LaneChangeAction),
    // Future: LaneOffsetAction, LateralDistanceAction
}
```

#### Step 4: Comprehensive Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lane_change_action_creation() {
        let dynamics = TransitionDynamics::default();
        let target = LaneChangeTarget::relative("Ego", -1);
        let action = LaneChangeAction::new(dynamics, target);
        
        assert!(action.target_lane_offset.is_none());
    }

    #[test]
    fn test_lane_change_with_offset() {
        let dynamics = TransitionDynamics::default();
        let target = LaneChangeTarget::absolute("1");
        let action = LaneChangeAction::new(dynamics, target)
            .with_offset(Value::literal(0.5));
        
        assert_eq!(action.target_lane_offset.unwrap().as_literal().unwrap(), &0.5);
    }

    #[test]
    fn test_xml_serialization() {
        let action = LaneChangeAction::new(
            TransitionDynamics::default(),
            LaneChangeTarget::relative("Ego", -1)
        );
        
        let xml = quick_xml::se::to_string(&action).unwrap();
        assert!(xml.contains("LaneChangeAction"));
        assert!(xml.contains("RelativeTargetLane"));
    }

    #[test]
    fn test_xml_deserialization() {
        let xml = r#"
        <LaneChangeAction targetLaneOffset="0.5">
            <LaneChangeActionDynamics dynamicsShape="linear" value="2.0" />
            <LaneChangeTarget>
                <RelativeTargetLane entityRef="Ego" value="-1" />
            </LaneChangeTarget>
        </LaneChangeAction>"#;
        
        let action: LaneChangeAction = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(action.target_lane_offset.unwrap().as_literal().unwrap(), &0.5);
    }
}
```

#### Step 5: Update Module Exports

```rust
// In src/types/actions/mod.rs:
pub mod lateral;
pub use lateral::*;

// In src/types/mod.rs:
pub use actions::LaneChangeAction;
```

### Validation Criteria - ✅ COMPLETED

- [x] All XSD attributes and elements implemented correctly
- [x] Serde serialization/deserialization works
- [x] Helper methods provide ergonomic API
- [x] 100% test coverage with XML round-trip tests
- [x] Integration with existing action system
- [x] Zero compilation warnings

**Implementation Location**: `src/types/actions/movement.rs`

#### ✅ Agent Task 1.2: LaneOffsetAction Implementation - COMPLETED

**Status**: ✅ **COMPLETED** (commit: bf6bd42)
**Agent Type**: `subagents/coder-agent`
**Duration**: 1 day

**Detailed Instructions:**

```markdown
## Task: Implement LaneOffsetAction System

### XSD Analysis Required:
1. Read `Schema/OpenSCENARIO.xsd` lines 1349-1365 for LaneOffsetAction definition
2. Extract LaneOffsetActionDynamics and LaneOffsetTarget structures

### Implementation Steps:

#### Step 1: Core Implementation (src/types/actions/lateral.rs)
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "LaneOffsetAction")]
pub struct LaneOffsetAction {
    #[serde(rename = "LaneOffsetActionDynamics")]
    pub dynamics: LaneOffsetActionDynamics,
    
    #[serde(rename = "LaneOffsetTarget")]
    pub target: LaneOffsetTarget,
    
    #[serde(rename = "@continuous")]
    pub continuous: Boolean,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaneOffsetActionDynamics {
    #[serde(rename = "@dynamicsShape")]
    pub dynamics_shape: DynamicsShape,
    
    #[serde(rename = "@maxLateralAcc")]
    pub max_lateral_acc: Option<Double>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LaneOffsetTarget {
    #[serde(rename = "RelativeTargetLaneOffset")]
    Relative(RelativeTargetLaneOffset),
    #[serde(rename = "AbsoluteTargetLaneOffset")]
    Absolute(AbsoluteTargetLaneOffset),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelativeTargetLaneOffset {
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,
    #[serde(rename = "@value")]
    pub value: Double,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbsoluteTargetLaneOffset {
    #[serde(rename = "@value")]
    pub value: Double,
}
```

#### Step 2: Add to LateralAction enum

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LateralAction {
    #[serde(rename = "LaneChangeAction")]
    LaneChange(LaneChangeAction),
    #[serde(rename = "LaneOffsetAction")]
    LaneOffset(LaneOffsetAction),
}
```

#### Step 3: Helper Methods and Tests

```rust
impl LaneOffsetAction {
    pub fn new(
        dynamics: LaneOffsetActionDynamics,
        target: LaneOffsetTarget,
        continuous: bool,
    ) -> Self {
        Self {
            dynamics,
            target,
            continuous: Value::literal(continuous),
        }
    }
}

// Add comprehensive tests similar to LaneChangeAction
```

### Validation Criteria - ✅ COMPLETED

- [x] XSD compliance verified
- [x] Integration with LateralAction enum
- [x] Complete test coverage
- [x] XML serialization working

**Implementation Location**: `src/types/actions/movement.rs`

#### ✅ Agent Task 1.3: LateralAction Container Implementation - COMPLETED

**Status**: ✅ **COMPLETED** (commit: bf6bd42)
**Agent Type**: `subagents/coder-agent`
**Duration**: 1 day

**Detailed Instructions:**

```markdown
## Task: Complete LateralAction Container and Integration

### Implementation Steps:

#### Step 1: Complete LateralAction enum (src/types/actions/lateral.rs)
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LateralAction {
    #[serde(rename = "LaneChangeAction")]
    LaneChange(LaneChangeAction),
    #[serde(rename = "LaneOffsetAction")]
    LaneOffset(LaneOffsetAction),
    // Future: LateralDistanceAction when implemented
}

impl LateralAction {
    pub fn lane_change(action: LaneChangeAction) -> Self {
        Self::LaneChange(action)
    }
    
    pub fn lane_offset(action: LaneOffsetAction) -> Self {
        Self::LaneOffset(action)
    }
}
```

#### Step 2: Integration with PrivateAction

```rust
// In src/types/actions/mod.rs - update PrivateAction enum:
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrivateAction {
    #[serde(rename = "LongitudinalAction")]
    Longitudinal(LongitudinalAction),
    #[serde(rename = "LateralAction")]
    Lateral(LateralAction),
    // ... existing variants
}
```

#### Step 3: Integration Tests

```rust
#[test]
fn test_lateral_action_in_private_action() {
    let lane_change = LaneChangeAction::new(
        TransitionDynamics::default(),
        LaneChangeTarget::relative("Ego", -1)
    );
    
    let lateral = LateralAction::lane_change(lane_change);
    let private = PrivateAction::Lateral(lateral);
    
    // Test XML serialization
    let xml = quick_xml::se::to_string(&private).unwrap();
    assert!(xml.contains("LateralAction"));
    assert!(xml.contains("LaneChangeAction"));
}
```

### Validation Criteria - ✅ COMPLETED

- [x] Complete integration with action hierarchy
- [x] All lateral actions accessible through PrivateAction
- [x] End-to-end XML serialization working
- [x] Integration tests passing

**Implementation Location**: `src/types/actions/movement.rs`

### Week 2: Traffic Actions Implementation (4 types) - ✅ COMPLETED

#### ✅ Agent Task 2.1: Traffic Signal System Implementation - COMPLETED

**Status**: ✅ **COMPLETED** (commits: fdc7ce2, fb35a65)
**Agent Type**: `subagents/coder-agent`
**Duration**: 3 days

**Detailed Instructions:**

```markdown
## Task: Implement Complete Traffic Signal System

### XSD Analysis Required:
1. Read `Schema/OpenSCENARIO.xsd` for TrafficSignalController, TrafficSignalState, TrafficSignalStateAction
2. Analyze Phase, TrafficSignalGroupState structures
3. Understand traffic signal timing and state management

### Implementation Steps:

#### Step 1: Create Traffic Signal Types (src/types/actions/traffic.rs)
```rust
use crate::types::basic::*;
use crate::types::enums::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "TrafficSignalController")]
pub struct TrafficSignalController {
    #[serde(rename = "@name")]
    pub name: OSString,
    
    #[serde(rename = "@delay")]
    pub delay: Option<Double>,
    
    #[serde(rename = "Phase")]
    pub phases: Vec<Phase>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "Phase")]
pub struct Phase {
    #[serde(rename = "@name")]
    pub name: OSString,
    
    #[serde(rename = "@duration")]
    pub duration: Double,
    
    #[serde(rename = "TrafficSignalState")]
    pub traffic_signal_states: Vec<TrafficSignalState>,
    
    #[serde(rename = "TrafficSignalGroupState")]
    pub traffic_signal_group_state: Option<TrafficSignalGroupState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "TrafficSignalState")]
pub struct TrafficSignalState {
    #[serde(rename = "@trafficSignalId")]
    pub traffic_signal_id: OSString,
    
    #[serde(rename = "@state")]
    pub state: OSString,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "TrafficSignalGroupState")]
pub struct TrafficSignalGroupState {
    #[serde(rename = "@state")]
    pub state: OSString,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "TrafficSignalStateAction")]
pub struct TrafficSignalStateAction {
    #[serde(rename = "@name")]
    pub name: OSString,
    
    #[serde(rename = "@state")]
    pub state: OSString,
}
```

#### Step 2: Helper Methods and Builders

```rust
impl TrafficSignalController {
    pub fn new(name: impl Into<OSString>) -> Self {
        Self {
            name: name.into(),
            delay: None,
            phases: Vec::new(),
        }
    }
    
    pub fn with_delay(mut self, delay: f64) -> Self {
        self.delay = Some(Value::literal(delay));
        self
    }
    
    pub fn add_phase(mut self, phase: Phase) -> Self {
        self.phases.push(phase);
        self
    }
}

impl Phase {
    pub fn new(name: impl Into<OSString>, duration: f64) -> Self {
        Self {
            name: name.into(),
            duration: Value::literal(duration),
            traffic_signal_states: Vec::new(),
            traffic_signal_group_state: None,
        }
    }
    
    pub fn add_signal_state(mut self, signal_id: impl Into<OSString>, state: impl Into<OSString>) -> Self {
        self.traffic_signal_states.push(TrafficSignalState {
            traffic_signal_id: signal_id.into(),
            state: state.into(),
        });
        self
    }
}

impl TrafficSignalStateAction {
    pub fn new(name: impl Into<OSString>, state: impl Into<OSString>) -> Self {
        Self {
            name: name.into(),
            state: state.into(),
        }
    }
}
```

#### Step 3: Integration with InfrastructureAction

```rust
// In src/types/actions/infrastructure.rs (create new file):
use super::traffic::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InfrastructureAction {
    #[serde(rename = "TrafficSignalAction")]
    TrafficSignal(TrafficSignalAction),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "TrafficSignalAction")]
pub struct TrafficSignalAction {
    #[serde(rename = "TrafficSignalController")]
    pub controller: Option<TrafficSignalController>,
    
    #[serde(rename = "TrafficSignalStateAction")]
    pub state_action: Option<TrafficSignalStateAction>,
}
```

#### Step 4: Comprehensive Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traffic_signal_controller_creation() {
        let controller = TrafficSignalController::new("intersection_1")
            .with_delay(1.0)
            .add_phase(
                Phase::new("green_ns", 30.0)
                    .add_signal_state("signal_1", "green")
                    .add_signal_state("signal_2", "red")
            );
        
        assert_eq!(controller.phases.len(), 1);
        assert_eq!(controller.delay.unwrap().as_literal().unwrap(), &1.0);
    }

    #[test]
    fn test_traffic_signal_xml_serialization() {
        let controller = TrafficSignalController::new("test")
            .add_phase(Phase::new("phase1", 10.0));
        
        let xml = quick_xml::se::to_string(&controller).unwrap();
        assert!(xml.contains("TrafficSignalController"));
        assert!(xml.contains("Phase"));
    }

    #[test]
    fn test_traffic_signal_state_action() {
        let action = TrafficSignalStateAction::new("signal_1", "green");
        
        assert_eq!(action.name.as_literal().unwrap(), "signal_1");
        assert_eq!(action.state.as_literal().unwrap(), "green");
    }
}
```

### Validation Criteria - ✅ COMPLETED

- [x] All traffic signal types implemented per XSD
- [x] Builder pattern for easy construction
- [x] Integration with infrastructure actions
- [x] Complete test coverage with XML validation
- [x] Real-world traffic signal scenarios supported

**Implementation Location**: `src/types/actions/traffic.rs`

#### ✅ Agent Task 2.2: TrafficSwarmAction Implementation - COMPLETED

**Status**: ✅ **COMPLETED** (commit: fb35a65)
**Agent Type**: `subagents/coder-agent`
**Duration**: 2 days

**Detailed Instructions:**

```markdown
## Task: Implement TrafficSwarmAction System

### XSD Analysis Required:
1. Read XSD definition for TrafficSwarmAction
2. Understand swarm traffic simulation concepts
3. Identify integration points with existing traffic system

### Implementation Steps:

#### Step 1: Core TrafficSwarmAction (src/types/actions/traffic.rs)
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "TrafficSwarmAction")]
pub struct TrafficSwarmAction {
    #[serde(rename = "@centralObject")]
    pub central_object: OSString,
    
    #[serde(rename = "@semiMajorAxis")]
    pub semi_major_axis: Double,
    
    #[serde(rename = "@semiMinorAxis")]
    pub semi_minor_axis: Double,
    
    #[serde(rename = "@innerRadius")]
    pub inner_radius: Option<Double>,
    
    #[serde(rename = "@offset")]
    pub offset: Option<Double>,
    
    #[serde(rename = "@numberOfVehicles")]
    pub number_of_vehicles: UnsignedInt,
    
    #[serde(rename = "CentralSwarmObject")]
    pub central_swarm_object: Option<CentralSwarmObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "CentralSwarmObject")]
pub struct CentralSwarmObject {
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,
}
```

#### Step 2: Helper Methods

```rust
impl TrafficSwarmAction {
    pub fn new(
        central_object: impl Into<OSString>,
        semi_major_axis: f64,
        semi_minor_axis: f64,
        number_of_vehicles: u32,
    ) -> Self {
        Self {
            central_object: central_object.into(),
            semi_major_axis: Value::literal(semi_major_axis),
            semi_minor_axis: Value::literal(semi_minor_axis),
            inner_radius: None,
            offset: None,
            number_of_vehicles: Value::literal(number_of_vehicles),
            central_swarm_object: None,
        }
    }
    
    pub fn with_inner_radius(mut self, radius: f64) -> Self {
        self.inner_radius = Some(Value::literal(radius));
        self
    }
    
    pub fn with_central_swarm_object(mut self, entity_ref: impl Into<OSString>) -> Self {
        self.central_swarm_object = Some(CentralSwarmObject {
            entity_ref: entity_ref.into(),
        });
        self
    }
}
```

#### Step 3: Integration and Testing

```rust
// Add to TrafficAction enum if it exists, or create new container

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traffic_swarm_action_creation() {
        let swarm = TrafficSwarmAction::new("Ego", 100.0, 50.0, 10)
            .with_inner_radius(20.0)
            .with_central_swarm_object("CentralVehicle");
        
        assert_eq!(swarm.number_of_vehicles.as_literal().unwrap(), &10);
        assert!(swarm.inner_radius.is_some());
        assert!(swarm.central_swarm_object.is_some());
    }
}
```

### Validation Criteria - ✅ COMPLETED

- [x] XSD-compliant implementation
- [x] Builder pattern for configuration
- [x] Integration with traffic system
- [x] Complete test coverage

**Implementation Location**: `src/types/actions/traffic.rs`

## Phase 1: UPDATED Remaining Critical Types (HIGH PRIORITY - 3 weeks, ~15 types)

### Week 3: Advanced Conditions Implementation (5 types) - 🔄 NEXT PRIORITY

#### Agent Task 3.1: Position and Distance Conditions

**Agent Type**: `subagents/coder-agent`
**Duration**: 3 days

**Detailed Instructions:**

```markdown
## Task: Implement Position and Distance-Based Conditions

### XSD Analysis Required:
1. Read XSD for ReachPositionCondition, DistanceCondition, RelativeDistanceCondition
2. Understand position tolerance and distance measurement types
3. Analyze coordinate system and routing algorithm options

### Implementation Steps:

#### Step 1: Core Condition Types (src/types/conditions/spatial.rs)
```rust
use crate::types::basic::*;
use crate::types::enums::*;
use crate::types::positions::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "ReachPositionCondition")]
pub struct ReachPositionCondition {
    #[serde(rename = "Position")]
    pub position: Position,
    
    #[serde(rename = "@tolerance")]
    pub tolerance: Double,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "DistanceCondition")]
pub struct DistanceCondition {
    #[serde(rename = "Position")]
    pub position: Position,
    
    #[serde(rename = "@value")]
    pub value: Double,
    
    #[serde(rename = "@freespace")]
    pub freespace: Boolean,
    
    #[serde(rename = "@rule")]
    pub rule: Rule,
    
    #[serde(rename = "@alongRoute")]
    pub along_route: Option<Boolean>,
    
    #[serde(rename = "@coordinateSystem")]
    pub coordinate_system: Option<CoordinateSystem>,
    
    #[serde(rename = "@relativeDistanceType")]
    pub relative_distance_type: Option<RelativeDistanceType>,
    
    #[serde(rename = "@routingAlgorithm")]
    pub routing_algorithm: Option<RoutingAlgorithm>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "RelativeDistanceCondition")]
pub struct RelativeDistanceCondition {
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,
    
    #[serde(rename = "@value")]
    pub value: Double,
    
    #[serde(rename = "@freespace")]
    pub freespace: Boolean,
    
    #[serde(rename = "@relativeDistanceType")]
    pub relative_distance_type: RelativeDistanceType,
    
    #[serde(rename = "@rule")]
    pub rule: Rule,
    
    #[serde(rename = "@coordinateSystem")]
    pub coordinate_system: Option<CoordinateSystem>,
    
    #[serde(rename = "@routingAlgorithm")]
    pub routing_algorithm: Option<RoutingAlgorithm>,
}
```

#### Step 2: Helper Methods and Builders

```rust
impl ReachPositionCondition {
    pub fn new(position: Position, tolerance: f64) -> Self {
        Self {
            position,
            tolerance: Value::literal(tolerance),
        }
    }
}

impl DistanceCondition {
    pub fn new(
        position: Position,
        value: f64,
        freespace: bool,
        rule: Rule,
    ) -> Self {
        Self {
            position,
            value: Value::literal(value),
            freespace: Value::literal(freespace),
            rule,
            along_route: None,
            coordinate_system: None,
            relative_distance_type: None,
            routing_algorithm: None,
        }
    }
    
    pub fn with_coordinate_system(mut self, system: CoordinateSystem) -> Self {
        self.coordinate_system = Some(system);
        self
    }
    
    pub fn with_distance_type(mut self, distance_type: RelativeDistanceType) -> Self {
        self.relative_distance_type = Some(distance_type);
        self
    }
}

impl RelativeDistanceCondition {
    pub fn new(
        entity_ref: impl Into<OSString>,
        value: f64,
        freespace: bool,
        distance_type: RelativeDistanceType,
        rule: Rule,
    ) -> Self {
        Self {
            entity_ref: entity_ref.into(),
            value: Value::literal(value),
            freespace: Value::literal(freespace),
            relative_distance_type: distance_type,
            rule,
            coordinate_system: None,
            routing_algorithm: None,
        }
    }
}
```

#### Step 3: Integration with EntityCondition

```rust
// In src/types/conditions/entity.rs - add to EntityCondition enum:
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityCondition {
    // ... existing variants
    #[serde(rename = "ReachPositionCondition")]
    ReachPosition(ReachPositionCondition),
    #[serde(rename = "DistanceCondition")]
    Distance(DistanceCondition),
    #[serde(rename = "RelativeDistanceCondition")]
    RelativeDistance(RelativeDistanceCondition),
}
```

#### Step 4: Comprehensive Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::positions::*;

    #[test]
    fn test_reach_position_condition() {
        let position = Position::World(WorldPosition::new(100.0, 200.0, 0.0));
        let condition = ReachPositionCondition::new(position, 5.0);
        
        assert_eq!(condition.tolerance.as_literal().unwrap(), &5.0);
    }

    #[test]
    fn test_distance_condition_with_options() {
        let position = Position::World(WorldPosition::new(0.0, 0.0, 0.0));
        let condition = DistanceCondition::new(position, 50.0, true, Rule::LessThan)
            .with_coordinate_system(CoordinateSystem::World)
            .with_distance_type(RelativeDistanceType::EuclidianDistance);
        
        assert_eq!(condition.value.as_literal().unwrap(), &50.0);
        assert_eq!(condition.coordinate_system, Some(CoordinateSystem::World));
    }

    #[test]
    fn test_relative_distance_condition() {
        let condition = RelativeDistanceCondition::new(
            "Ego",
            30.0,
            false,
            RelativeDistanceType::Longitudinal,
            Rule::GreaterThan,
        );
        
        assert_eq!(condition.entity_ref.as_literal().unwrap(), "Ego");
        assert_eq!(condition.relative_distance_type, RelativeDistanceType::Longitudinal);
    }

    #[test]
    fn test_xml_serialization() {
        let position = Position::World(WorldPosition::new(100.0, 200.0, 0.0));
        let condition = DistanceCondition::new(position, 25.0, true, Rule::EqualTo);
        
        let xml = quick_xml::se::to_string(&condition).unwrap();
        assert!(xml.contains("DistanceCondition"));
        assert!(xml.contains("value=\"25\""));
        assert!(xml.contains("freespace=\"true\""));
    }
}
```

### Validation Criteria

- [ ] All spatial conditions implemented per XSD
- [ ] Integration with existing condition system
- [ ] Support for all coordinate systems and distance types
- [ ] Complete test coverage with XML validation
- [ ] Builder patterns for easy construction

#### Agent Task 3.2: Speed and Acceleration Conditions

**Agent Type**: `subagents/coder-agent`
**Duration**: 2 days

**Detailed Instructions:**

```markdown
## Task: Implement Speed and Acceleration Conditions

### XSD Analysis Required:
1. Read XSD for SpeedCondition, AccelerationCondition, RelativeSpeedCondition
2. Understand directional dimensions and measurement rules
3. Analyze integration with existing condition system

### Implementation Steps:

#### Step 1: Enhanced Speed Conditions (src/types/conditions/entity.rs)
```rust
// Enhance existing SpeedCondition if needed, add new conditions:

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "AccelerationCondition")]
pub struct AccelerationCondition {
    #[serde(rename = "@value")]
    pub value: Double,
    
    #[serde(rename = "@rule")]
    pub rule: Rule,
    
    #[serde(rename = "@direction")]
    pub direction: Option<DirectionalDimension>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "RelativeSpeedCondition")]
pub struct RelativeSpeedCondition {
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,
    
    #[serde(rename = "@value")]
    pub value: Double,
    
    #[serde(rename = "@rule")]
    pub rule: Rule,
    
    #[serde(rename = "@direction")]
    pub direction: Option<DirectionalDimension>,
}
```

#### Step 2: Helper Methods

```rust
impl AccelerationCondition {
    pub fn new(value: f64, rule: Rule) -> Self {
        Self {
            value: Value::literal(value),
            rule,
            direction: None,
        }
    }
    
    pub fn with_direction(mut self, direction: DirectionalDimension) -> Self {
        self.direction = Some(direction);
        self
    }
}

impl RelativeSpeedCondition {
    pub fn new(entity_ref: impl Into<OSString>, value: f64, rule: Rule) -> Self {
        Self {
            entity_ref: entity_ref.into(),
            value: Value::literal(value),
            rule,
            direction: None,
        }
    }
    
    pub fn with_direction(mut self, direction: DirectionalDimension) -> Self {
        self.direction = Some(direction);
        self
    }
}
```

#### Step 3: Integration and Testing

```rust
// Add to EntityCondition enum:
#[serde(rename = "AccelerationCondition")]
Acceleration(AccelerationCondition),
#[serde(rename = "RelativeSpeedCondition")]
RelativeSpeed(RelativeSpeedCondition),

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acceleration_condition() {
        let condition = AccelerationCondition::new(2.5, Rule::GreaterThan)
            .with_direction(DirectionalDimension::Longitudinal);
        
        assert_eq!(condition.value.as_literal().unwrap(), &2.5);
        assert_eq!(condition.direction, Some(DirectionalDimension::Longitudinal));
    }

    #[test]
    fn test_relative_speed_condition() {
        let condition = RelativeSpeedCondition::new("LeadVehicle", 10.0, Rule::LessThan);
        
        assert_eq!(condition.entity_ref.as_literal().unwrap(), "LeadVehicle");
        assert_eq!(condition.value.as_literal().unwrap(), &10.0);
    }
}
```

### Validation Criteria

- [ ] All speed/acceleration conditions implemented
- [ ] Directional dimension support
- [ ] Integration with EntityCondition enum
- [ ] Complete test coverage

#### Agent Task 3.3: CollisionCondition Implementation

**Agent Type**: `subagents/coder-agent`
**Duration**: 1 day

**Detailed Instructions:**

```markdown
## Task: Implement CollisionCondition System

### XSD Analysis Required:
1. Read XSD for CollisionCondition structure
2. Understand collision detection with entity references and object types
3. Analyze ByType vs EntityRef collision targets

### Implementation Steps:

#### Step 1: Core Implementation (src/types/conditions/entity.rs)
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "CollisionCondition")]
pub struct CollisionCondition {
    #[serde(flatten)]
    pub target: CollisionTarget,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CollisionTarget {
    #[serde(rename = "EntityRef")]
    Entity(EntityRef),
    #[serde(rename = "ByType")]
    ByType(ByObjectType),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "ByType")]
pub struct ByObjectType {
    #[serde(rename = "@type")]
    pub object_type: ObjectType,
}
```

#### Step 2: Helper Methods

```rust
impl CollisionCondition {
    pub fn with_entity(entity_ref: impl Into<OSString>) -> Self {
        Self {
            target: CollisionTarget::Entity(EntityRef {
                entity_ref: entity_ref.into(),
            }),
        }
    }
    
    pub fn with_object_type(object_type: ObjectType) -> Self {
        Self {
            target: CollisionTarget::ByType(ByObjectType { object_type }),
        }
    }
}
```

#### Step 3: Integration and Testing

```rust
// Add to EntityCondition enum:
#[serde(rename = "CollisionCondition")]
Collision(CollisionCondition),

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collision_condition_with_entity() {
        let condition = CollisionCondition::with_entity("Ego");
        
        match condition.target {
            CollisionTarget::Entity(entity_ref) => {
                assert_eq!(entity_ref.entity_ref.as_literal().unwrap(), "Ego");
            }
            _ => panic!("Expected Entity target"),
        }
    }

    #[test]
    fn test_collision_condition_with_type() {
        let condition = CollisionCondition::with_object_type(ObjectType::Vehicle);
        
        match condition.target {
            CollisionTarget::ByType(by_type) => {
                assert_eq!(by_type.object_type, ObjectType::Vehicle);
            }
            _ => panic!("Expected ByType target"),
        }
    }
}
```

### Validation Criteria

- [ ] XSD-compliant collision condition
- [ ] Support for both entity and type-based collision detection
- [ ] Integration with condition system
- [ ] Complete test coverage

### Week 4: Vehicle Components Implementation (4 types)

#### Agent Task 4.1: Enhanced BoundingBox and Dimensions

**Agent Type**: `subagents/coder-agent`
**Duration**: 2 days

**Detailed Instructions:**

```markdown
## Task: Enhance BoundingBox and Dimensions Systems

### XSD Analysis Required:
1. Read XSD for complete BoundingBox, Dimensions, Center definitions
2. Compare with existing implementation in src/types/geometry/shapes.rs
3. Identify missing attributes or elements

### Implementation Steps:

#### Step 1: Enhance Existing Types (src/types/geometry/shapes.rs)
```rust
// Review and enhance existing BoundingBox implementation:
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "BoundingBox")]
pub struct BoundingBox {
    #[serde(rename = "Center")]
    pub center: Center,
    
    #[serde(rename = "Dimensions")]
    pub dimensions: Dimensions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "Center")]
pub struct Center {
    #[serde(rename = "@x")]
    pub x: Double,
    
    #[serde(rename = "@y")]
    pub y: Double,
    
    #[serde(rename = "@z")]
    pub z: Double,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "Dimensions")]
pub struct Dimensions {
    #[serde(rename = "@width")]
    pub width: Double,
    
    #[serde(rename = "@length")]
    pub length: Double,
    
    #[serde(rename = "@height")]
    pub height: Double,
}
```

#### Step 2: Add Advanced Helper Methods

```rust
impl BoundingBox {
    pub fn new(center: Center, dimensions: Dimensions) -> Self {
        Self { center, dimensions }
    }
    
    pub fn from_dimensions(width: f64, length: f64, height: f64) -> Self {
        Self {
            center: Center::origin(),
            dimensions: Dimensions::new(width, length, height),
        }
    }
    
    pub fn volume(&self) -> Result<f64, crate::Error> {
        let w = self.dimensions.width.resolve(&Default::default())?;
        let l = self.dimensions.length.resolve(&Default::default())?;
        let h = self.dimensions.height.resolve(&Default::default())?;
        Ok(w * l * h)
    }
    
    pub fn contains_point(&self, x: f64, y: f64, z: f64) -> Result<bool, crate::Error> {
        let cx = self.center.x.resolve(&Default::default())?;
        let cy = self.center.y.resolve(&Default::default())?;
        let cz = self.center.z.resolve(&Default::default())?;
        
        let w = self.dimensions.width.resolve(&Default::default())? / 2.0;
        let l = self.dimensions.length.resolve(&Default::default())? / 2.0;
        let h = self.dimensions.height.resolve(&Default::default())? / 2.0;
        
        Ok((x >= cx - w && x <= cx + w) &&
           (y >= cy - l && y <= cy + l) &&
           (z >= cz - h && z <= cz + h))
    }
}

impl Center {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x: Value::literal(x),
            y: Value::literal(y),
            z: Value::literal(z),
        }
    }
    
    pub fn origin() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
    
    pub fn distance_to(&self, other: &Center) -> Result<f64, crate::Error> {
        let x1 = self.x.resolve(&Default::default())?;
        let y1 = self.y.resolve(&Default::default())?;
        let z1 = self.z.resolve(&Default::default())?;
        
        let x2 = other.x.resolve(&Default::default())?;
        let y2 = other.y.resolve(&Default::default())?;
        let z2 = other.z.resolve(&Default::default())?;
        
        Ok(((x2 - x1).powi(2) + (y2 - y1).powi(2) + (z2 - z1).powi(2)).sqrt())
    }
}

impl Dimensions {
    pub fn new(width: f64, length: f64, height: f64) -> Self {
        Self {
            width: Value::literal(width),
            length: Value::literal(length),
            height: Value::literal(height),
        }
    }
    
    pub fn vehicle_default() -> Self {
        Self::new(2.0, 4.5, 1.8) // Typical car dimensions
    }
    
    pub fn pedestrian_default() -> Self {
        Self::new(0.6, 0.6, 1.8) // Typical pedestrian dimensions
    }
    
    pub fn truck_default() -> Self {
        Self::new(2.5, 12.0, 3.5) // Typical truck dimensions
    }
}
```

#### Step 3: Enhanced Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounding_box_volume() {
        let bbox = BoundingBox::from_dimensions(2.0, 4.0, 1.5);
        let volume = bbox.volume().unwrap();
        assert_eq!(volume, 12.0);
    }

    #[test]
    fn test_bounding_box_contains_point() {
        let bbox = BoundingBox::new(
            Center::new(0.0, 0.0, 0.0),
            Dimensions::new(4.0, 6.0, 2.0)
        );
        
        assert!(bbox.contains_point(1.0, 2.0, 0.5).unwrap());
        assert!(!bbox.contains_point(3.0, 4.0, 1.5).unwrap());
    }

    #[test]
    fn test_center_distance() {
        let center1 = Center::new(0.0, 0.0, 0.0);
        let center2 = Center::new(3.0, 4.0, 0.0);
        
        let distance = center1.distance_to(&center2).unwrap();
        assert_eq!(distance, 5.0);
    }

    #[test]
    fn test_dimensions_presets() {
        let vehicle_dims = Dimensions::vehicle_default();
        assert_eq!(vehicle_dims.width.as_literal().unwrap(), &2.0);
        
        let pedestrian_dims = Dimensions::pedestrian_default();
        assert_eq!(pedestrian_dims.height.as_literal().unwrap(), &1.8);
    }
}
```

### Validation Criteria

- [ ] Enhanced BoundingBox with geometric operations
- [ ] Utility methods for common use cases
- [ ] Parameter resolution support
- [ ] Complete test coverage with edge cases
- [ ] Backward compatibility maintained

#### Agent Task 4.2: Axle System Implementation

**Agent Type**: `subagents/coder-agent`
**Duration**: 2 days

**Detailed Instructions:**

```markdown
## Task: Implement Complete Axle System

### XSD Analysis Required:
1. Read XSD for Axle and Axles definitions (lines 795-808)
2. Understand axle positioning and wheel properties
3. Analyze integration with existing Vehicle type

### Implementation Steps:

#### Step 1: Core Axle Types (src/types/entities/axles.rs - create new file)
```rust
use crate::types::basic::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "Axles")]
pub struct Axles {
    #[serde(rename = "FrontAxle")]
    pub front_axle: Option<Axle>,
    
    #[serde(rename = "RearAxle")]
    pub rear_axle: Axle,
    
    #[serde(rename = "AdditionalAxle")]
    pub additional_axles: Vec<Axle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "Axle")]
pub struct Axle {
    #[serde(rename = "@maxSteering")]
    pub max_steering: Double,
    
    #[serde(rename = "@wheelDiameter")]
    pub wheel_diameter: Double,
    
    #[serde(rename = "@trackWidth")]
    pub track_width: Double,
    
    #[serde(rename = "@positionX")]
    pub position_x: Double,
    
    #[serde(rename = "@positionZ")]
    pub position_z: Double,
}
```

#### Step 2: Helper Methods and Builders

```rust
impl Axles {
    pub fn new(rear_axle: Axle) -> Self {
        Self {
            front_axle: None,
            rear_axle,
            additional_axles: Vec::new(),
        }
    }
    
    pub fn with_front_axle(mut self, front_axle: Axle) -> Self {
        self.front_axle = Some(front_axle);
        self
    }
    
    pub fn add_additional_axle(mut self, axle: Axle) -> Self {
        self.additional_axles.push(axle);
        self
    }
    
    pub fn total_axles(&self) -> usize {
        let front_count = if self.front_axle.is_some() { 1 } else { 0 };
        front_count + 1 + self.additional_axles.len() // +1 for rear axle
    }
    
    pub fn is_steerable(&self) -> bool {
        if let Some(ref front) = self.front_axle {
            if let Ok(max_steering) = front.max_steering.resolve(&Default::default()) {
                return max_steering > 0.0;
            }
        }
        false
    }
}

impl Axle {
    pub fn new(
        max_steering: f64,
        wheel_diameter: f64,
        track_width: f64,
        position_x: f64,
        position_z: f64,
    ) -> Self {
        Self {
            max_steering: Value::literal(max_steering),
            wheel_diameter: Value::literal(wheel_diameter),
            track_width: Value::literal(track_width),
            position_x: Value::literal(position_x),
            position_z: Value::literal(position_z),
        }
    }
    
    pub fn front_axle_default() -> Self {
        Self::new(
            0.5236, // ~30 degrees in radians
            0.6,    // 60cm wheel diameter
            1.8,    // 1.8m track width
            2.5,    // 2.5m from rear
            0.3,    // 30cm height
        )
    }
    
    pub fn rear_axle_default() -> Self {
        Self::new(
            0.0,    // No steering
            0.6,    // 60cm wheel diameter
            1.8,    // 1.8m track width
            0.0,    // Reference position
            0.3,    // 30cm height
        )
    }
    
    pub fn wheel_circumference(&self) -> Result<f64, crate::Error> {
        let diameter = self.wheel_diameter.resolve(&Default::default())?;
        Ok(std::f64::consts::PI * diameter)
    }
    
    pub fn max_steering_degrees(&self) -> Result<f64, crate::Error> {
        let radians = self.max_steering.resolve(&Default::default())?;
        Ok(radians.to_degrees())
    }
}
```

#### Step 3: Integration with Vehicle

```rust
// In src/types/entities/vehicle.rs - update Vehicle struct:
use super::axles::Axles;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "Vehicle")]
pub struct Vehicle {
    // ... existing fields
    
    #[serde(rename = "Axles")]
    pub axles: Axles,
    
    // ... rest of fields
}

impl Default for Vehicle {
    fn default() -> Self {
        Self {
            // ... existing defaults
            axles: Axles::new(Axle::rear_axle_default())
                .with_front_axle(Axle::front_axle_default()),
            // ... rest of defaults
        }
    }
}
```

#### Step 4: Comprehensive Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_axles_creation() {
        let rear_axle = Axle::rear_axle_default();
        let front_axle = Axle::front_axle_default();
        
        let axles = Axles::new(rear_axle)
            .with_front_axle(front_axle);
        
        assert_eq!(axles.total_axles(), 2);
        assert!(axles.is_steerable());
    }

    #[test]
    fn test_axle_calculations() {
        let axle = Axle::new(0.5236, 0.6, 1.8, 2.5, 0.3);
        
        let circumference = axle.wheel_circumference().unwrap();
        assert!((circumference - (std::f64::consts::PI * 0.6)).abs() < 0.001);
        
        let steering_degrees = axle.max_steering_degrees().unwrap();
        assert!((steering_degrees - 30.0).abs() < 0.1);
    }

    #[test]
    fn test_additional_axles() {
        let rear = Axle::rear_axle_default();
        let additional = Axle::new(0.0, 0.7, 2.0, -1.5, 0.3);
        
        let axles = Axles::new(rear)
            .add_additional_axle(additional);
        
        assert_eq!(axles.total_axles(), 2); // rear + additional
        assert_eq!(axles.additional_axles.len(), 1);
    }

    #[test]
    fn test_xml_serialization() {
        let axles = Axles::new(Axle::rear_axle_default())
            .with_front_axle(Axle::front_axle_default());
        
        let xml = quick_xml::se::to_string(&axles).unwrap();
        assert!(xml.contains("Axles"));
        assert!(xml.contains("FrontAxle"));
        assert!(xml.contains("RearAxle"));
    }

    #[test]
    fn test_vehicle_integration() {
        let vehicle = Vehicle::default();
        assert!(vehicle.axles.front_axle.is_some());
        assert!(vehicle.axles.is_steerable());
    }
}
```

#### Step 5: Update Module Exports

```rust
// In src/types/entities/mod.rs:
pub mod axles;
pub use axles::*;

// In src/types/mod.rs:
pub use entities::{Axles, Axle};
```

### Validation Criteria

- [ ] Complete axle system per XSD specification
- [ ] Integration with existing Vehicle type
- [ ] Utility methods for common calculations
- [ ] Support for multi-axle vehicles (trucks, trailers)
- [ ] Complete test coverage with XML validation
- [ ] Backward compatibility maintained

### Week 5: Routing & Navigation Implementation (3 types)

#### Agent Task 5.1: Waypoint and Route System

**Agent Type**: `subagents/coder-agent`
**Duration**: 3 days

**Detailed Instructions:**

```markdown
## Task: Implement Complete Waypoint and Route System

### XSD Analysis Required:
1. Read XSD for Waypoint, Route, RouteRef definitions (lines 1955-1980)
2. Understand route closure and waypoint positioning
3. Analyze integration with existing routing actions

### Implementation Steps:

#### Step 1: Core Route Types (src/types/routing/mod.rs - create new file)
```rust
use crate::types::basic::*;
use crate::types::positions::Position;
use crate::types::catalogs::CatalogReference;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "Route")]
pub struct Route {
    #[serde(rename = "ParameterDeclarations")]
    pub parameter_declarations: Option<ParameterDeclarations>,
    
    #[serde(rename = "Waypoint")]
    pub waypoints: Vec<Waypoint>,
    
    #[serde(rename = "@closed")]
    pub closed: Boolean,
    
    #[serde(rename = "@name")]
    pub name: OSString,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "Waypoint")]
pub struct Waypoint {
    #[serde(rename = "Position")]
    pub position: Position,
    
    #[serde(rename = "@routeStrategy")]
    pub route_strategy: RouteStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RouteRef {
    #[serde(rename = "Route")]
    Direct(Route),
    #[serde(rename = "CatalogReference")]
    Catalog(CatalogReference),
}
```

#### Step 2: Helper Methods and Builders

```rust
impl Route {
    pub fn new(name: impl Into<OSString>, closed: bool) -> Self {
        Self {
            parameter_declarations: None,
            waypoints: Vec::new(),
            closed: Value::literal(closed),
            name: name.into(),
        }
    }
    
    pub fn add_waypoint(mut self, waypoint: Waypoint) -> Self {
        self.waypoints.push(waypoint);
        self
    }
    
    pub fn add_position(mut self, position: Position, strategy: RouteStrategy) -> Self {
        self.waypoints.push(Waypoint::new(position, strategy));
        self
    }
    
    pub fn with_parameter_declarations(mut self, declarations: ParameterDeclarations) -> Self {
        self.parameter_declarations = Some(declarations);
        self
    }
    
    pub fn total_distance(&self) -> Result<f64, crate::Error> {
        if self.waypoints.len() < 2 {
            return Ok(0.0);
        }
        
        let mut total = 0.0;
        for i in 1..self.waypoints.len() {
            // Calculate distance between consecutive waypoints
            // This is a simplified calculation - real implementation would
            // consider route strategy and road network
            total += self.calculate_waypoint_distance(&self.waypoints[i-1], &self.waypoints[i])?;
        }
        
        // If closed route, add distance from last to first waypoint
        if self.closed.resolve(&Default::default())? && self.waypoints.len() > 2 {
            total += self.calculate_waypoint_distance(
                self.waypoints.last().unwrap(),
                self.waypoints.first().unwrap()
            )?;
        }
        
        Ok(total)
    }
    
    fn calculate_waypoint_distance(&self, wp1: &Waypoint, wp2: &Waypoint) -> Result<f64, crate::Error> {
        // Simplified Euclidean distance calculation
        // Real implementation would use route strategy and road network
        match (&wp1.position, &wp2.position) {
            (Position::World(pos1), Position::World(pos2)) => {
                let x1 = pos1.x.resolve(&Default::default())?;
                let y1 = pos1.y.resolve(&Default::default())?;
                let x2 = pos2.x.resolve(&Default::default())?;
                let y2 = pos2.y.resolve(&Default::default())?;
                
                Ok(((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt())
            }
            _ => Ok(0.0), // Placeholder for other position types
        }
    }
}

impl Waypoint {
    pub fn new(position: Position, route_strategy: RouteStrategy) -> Self {
        Self {
            position,
            route_strategy,
        }
    }
    
    pub fn world_position(x: f64, y: f64, z: f64, strategy: RouteStrategy) -> Self {
        Self::new(
            Position::World(WorldPosition::new(x, y, z)),
            strategy
        )
    }
    
    pub fn lane_position(
        road_id: impl Into<OSString>,
        lane_id: impl Into<OSString>,
        s: f64,
        strategy: RouteStrategy,
    ) -> Self {
        Self::new(
            Position::Lane(LanePosition::new(road_id, lane_id, s)),
            strategy
        )
    }
}

impl RouteRef {
    pub fn direct(route: Route) -> Self {
        Self::Direct(route)
    }
    
    pub fn catalog(catalog_name: impl Into<OSString>, entry_name: impl Into<OSString>) -> Self {
        Self::Catalog(CatalogReference::new(catalog_name, entry_name))
    }
}
```

#### Step 3: Integration with Actions

```rust
// In src/types/actions/routing.rs - update AssignRouteAction:
use crate::types::routing::RouteRef;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "AssignRouteAction")]
pub struct AssignRouteAction {
    #[serde(flatten)]
    pub route: RouteRef,
}

impl AssignRouteAction {
    pub fn new(route: RouteRef) -> Self {
        Self { route }
    }
    
    pub fn direct_route(route: Route) -> Self {
        Self::new(RouteRef::direct(route))
    }
    
    pub fn catalog_route(catalog_name: impl Into<OSString>, entry_name: impl Into<OSString>) -> Self {
        Self::new(RouteRef::catalog(catalog_name, entry_name))
    }
}
```

#### Step 4: Comprehensive Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::positions::*;
    use crate::types::enums::RouteStrategy;

    #[test]
    fn test_route_creation() {
        let route = Route::new("test_route", false)
            .add_position(
                Position::World(WorldPosition::new(0.0, 0.0, 0.0)),
                RouteStrategy::Shortest
            )
            .add_position(
                Position::World(WorldPosition::new(100.0, 100.0, 0.0)),
                RouteStrategy::Fastest
            );
        
        assert_eq!(route.waypoints.len(), 2);
        assert_eq!(route.name.as_literal().unwrap(), "test_route");
        assert!(!route.closed.as_literal().unwrap());
    }

    #[test]
    fn test_waypoint_creation() {
        let waypoint = Waypoint::world_position(50.0, 25.0, 0.0, RouteStrategy::Shortest);
        
        match waypoint.position {
            Position::World(ref pos) => {
                assert_eq!(pos.x.as_literal().unwrap(), &50.0);
                assert_eq!(pos.y.as_literal().unwrap(), &25.0);
            }
            _ => panic!("Expected WorldPosition"),
        }
        
        assert_eq!(waypoint.route_strategy, RouteStrategy::Shortest);
    }

    #[test]
    fn test_route_distance_calculation() {
        let route = Route::new("distance_test", false)
            .add_position(
                Position::World(WorldPosition::new(0.0, 0.0, 0.0)),
                RouteStrategy::Shortest
            )
            .add_position(
                Position::World(WorldPosition::new(3.0, 4.0, 0.0)),
                RouteStrategy::Shortest
            );
        
        let distance = route.total_distance().unwrap();
        assert_eq!(distance, 5.0); // 3-4-5 triangle
    }

    #[test]
    fn test_closed_route_distance() {
        let route = Route::new("closed_test", true)
            .add_position(
                Position::World(WorldPosition::new(0.0, 0.0, 0.0)),
                RouteStrategy::Shortest
            )
            .add_position(
                Position::World(WorldPosition::new(10.0, 0.0, 0.0)),
                RouteStrategy::Shortest
            )
            .add_position(
                Position::World(WorldPosition::new(10.0, 10.0, 0.0)),
                RouteStrategy::Shortest
            );
        
        let distance = route.total_distance().unwrap();
        // 10 + 10 + sqrt(200) ≈ 34.14
        assert!((distance - 34.142135623730951).abs() < 0.001);
    }

    #[test]
    fn test_route_ref_variants() {
        let direct_route = Route::new("direct", false);
        let route_ref = RouteRef::direct(direct_route);
        
        match route_ref {
            RouteRef::Direct(ref route) => {
                assert_eq!(route.name.as_literal().unwrap(), "direct");
            }
            _ => panic!("Expected Direct route"),
        }
        
        let catalog_ref = RouteRef::catalog("RouteCatalog", "highway_route");
        match catalog_ref {
            RouteRef::Catalog(ref cat_ref) => {
                assert_eq!(cat_ref.catalog_name.as_literal().unwrap(), "RouteCatalog");
            }
            _ => panic!("Expected Catalog route"),
        }
    }

    #[test]
    fn test_assign_route_action() {
        let route = Route::new("action_test", false);
        let action = AssignRouteAction::direct_route(route);
        
        match action.route {
            RouteRef::Direct(ref route) => {
                assert_eq!(route.name.as_literal().unwrap(), "action_test");
            }
            _ => panic!("Expected Direct route"),
        }
    }

    #[test]
    fn test_xml_serialization() {
        let route = Route::new("xml_test", false)
            .add_waypoint(Waypoint::world_position(0.0, 0.0, 0.0, RouteStrategy::Shortest));
        
        let xml = quick_xml::se::to_string(&route).unwrap();
        assert!(xml.contains("Route"));
        assert!(xml.contains("Waypoint"));
        assert!(xml.contains("closed=\"false\""));
    }
}
```

#### Step 5: Update Module Exports

```rust
// In src/types/mod.rs:
pub mod routing;
pub use routing::*;

// In src/types/actions/mod.rs:
// Update RoutingAction to include AssignRouteAction
```

### Validation Criteria

- [ ] Complete route and waypoint system per XSD
- [ ] Integration with existing routing actions
- [ ] Support for both direct and catalog routes
- [ ] Route distance calculations and utilities
- [ ] Complete test coverage with XML validation
- [ ] Parameter declaration support

### Week 6: Animation & Visual Implementation (4 types)

#### Agent Task 6.1: Animation System Implementation

**Agent Type**: `subagents/coder-agent`
**Duration**: 4 days

**Detailed Instructions:**

```markdown
## Task: Implement Complete Animation and Visual System

### XSD Analysis Required:
1. Read XSD for AnimationAction, AnimationFile, AnimationType definitions (lines 740-764)
2. Understand ComponentAnimation, PedestrianAnimation structures
3. Analyze AppearanceAction and VisibilityAction integration

### Implementation Steps:

#### Step 1: Core Animation Types (src/types/actions/animation.rs - create new file)
```rust
use crate::types::basic::*;
use crate::types::enums::*;
use crate::types::entities::File;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "AnimationAction")]
pub struct AnimationAction {
    #[serde(rename = "AnimationType")]
    pub animation_type: AnimationType,
    
    #[serde(rename = "AnimationState")]
    pub animation_state: Option<AnimationState>,
    
    #[serde(rename = "@loop")]
    pub loop_animation: Option<Boolean>,
    
    #[serde(rename = "@animationDuration")]
    pub animation_duration: Option<Double>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnimationType {
    #[serde(rename = "ComponentAnimation")]
    Component(ComponentAnimation),
    #[serde(rename = "PedestrianAnimation")]
    Pedestrian(PedestrianAnimation),
    #[serde(rename = "AnimationFile")]
    File(AnimationFile),
    #[serde(rename = "UserDefinedAnimation")]
    UserDefined(UserDefinedAnimation),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "AnimationState")]
pub struct AnimationState {
    #[serde(rename = "@state")]
    pub state: Double,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "ComponentAnimation")]
pub struct ComponentAnimation {
    #[serde(flatten)]
    pub component: ComponentType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComponentType {
    #[serde(rename = "VehicleComponent")]
    Vehicle(VehicleComponent),
    #[serde(rename = "UserDefinedComponent")]
    UserDefined(UserDefinedComponent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "VehicleComponent")]
pub struct VehicleComponent {
    #[serde(rename = "@vehicleComponentType")]
    pub vehicle_component_type: VehicleComponentType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "UserDefinedComponent")]
pub struct UserDefinedComponent {
    #[serde(rename = "@userDefinedComponentType")]
    pub user_defined_component_type: OSString,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "PedestrianAnimation")]
pub struct PedestrianAnimation {
    #[serde(rename = "PedestrianGesture")]
    pub pedestrian_gestures: Vec<PedestrianGesture>,
    
    #[serde(rename = "@motion")]
    pub motion: Option<PedestrianMotionType>,
    
    #[serde(rename = "@userDefinedPedestrianAnimation")]
    pub user_defined_pedestrian_animation: Option<OSString>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "PedestrianGesture")]
pub struct PedestrianGesture {
    #[serde(rename = "@gesture")]
    pub gesture: PedestrianGestureType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "AnimationFile")]
pub struct AnimationFile {
    #[serde(rename = "File")]
    pub file: File,
    
    #[serde(rename = "@timeOffset")]
    pub time_offset: Option<Double>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "UserDefinedAnimation")]
pub struct UserDefinedAnimation {
    #[serde(rename = "@userDefinedAnimationType")]
    pub user_defined_animation_type: OSString,
}
```

#### Step 2: Helper Methods and Builders

```rust
impl AnimationAction {
    pub fn new(animation_type: AnimationType) -> Self {
        Self {
            animation_type,
            animation_state: None,
            loop_animation: None,
            animation_duration: None,
        }
    }
    
    pub fn with_loop(mut self, loop_animation: bool) -> Self {
        self.loop_animation = Some(Value::literal(loop_animation));
        self
    }
    
    pub fn with_duration(mut self, duration: f64) -> Self {
        self.animation_duration = Some(Value::literal(duration));
        self
    }
    
    pub fn with_state(mut self, state: f64) -> Self {
        self.animation_state = Some(AnimationState {
            state: Value::literal(state),
        });
        self
    }
    
    pub fn vehicle_component(component_type: VehicleComponentType) -> Self {
        Self::new(AnimationType::Component(ComponentAnimation {
            component: ComponentType::Vehicle(VehicleComponent {
                vehicle_component_type: component_type,
            }),
        }))
    }
    
    pub fn pedestrian_motion(motion: PedestrianMotionType) -> Self {
        Self::new(AnimationType::Pedestrian(PedestrianAnimation {
            pedestrian_gestures: Vec::new(),
            motion: Some(motion),
            user_defined_pedestrian_animation: None,
        }))
    }
    
    pub fn from_file(file_path: impl Into<OSString>) -> Self {
        Self::new(AnimationType::File(AnimationFile {
            file: File {
                filepath: file_path.into(),
            },
            time_offset: None,
        }))
    }
}

impl PedestrianAnimation {
    pub fn new() -> Self {
        Self {
            pedestrian_gestures: Vec::new(),
            motion: None,
            user_defined_pedestrian_animation: None,
        }
    }
    
    pub fn with_motion(mut self, motion: PedestrianMotionType) -> Self {
        self.motion = Some(motion);
        self
    }
    
    pub fn add_gesture(mut self, gesture: PedestrianGestureType) -> Self {
        self.pedestrian_gestures.push(PedestrianGesture { gesture });
        self
    }
    
    pub fn with_user_defined(mut self, animation: impl Into<OSString>) -> Self {
        self.user_defined_pedestrian_animation = Some(animation.into());
        self
    }
}

impl AnimationFile {
    pub fn new(file_path: impl Into<OSString>) -> Self {
        Self {
            file: File {
                filepath: file_path.into(),
            },
            time_offset: None,
        }
    }
    
    pub fn with_time_offset(mut self, offset: f64) -> Self {
        self.time_offset = Some(Value::literal(offset));
        self
    }
}
```

#### Step 3: AppearanceAction and VisibilityAction

```rust
// In src/types/actions/appearance.rs - create new file:
use super::animation::*;
use super::lighting::*; // Assuming lighting actions exist
use crate::types::basic::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppearanceAction {
    #[serde(rename = "LightStateAction")]
    LightState(LightStateAction),
    #[serde(rename = "AnimationAction")]
    Animation(AnimationAction),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "VisibilityAction")]
pub struct VisibilityAction {
    #[serde(rename = "@graphics")]
    pub graphics: Boolean,
    
    #[serde(rename = "@traffic")]
    pub traffic: Boolean,
    
    #[serde(rename = "@sensors")]
    pub sensors: Boolean,
}

impl AppearanceAction {
    pub fn animation(action: AnimationAction) -> Self {
        Self::Animation(action)
    }
    
    pub fn light_state(action: LightStateAction) -> Self {
        Self::LightState(action)
    }
}

impl VisibilityAction {
    pub fn new(graphics: bool, traffic: bool, sensors: bool) -> Self {
        Self {
            graphics: Value::literal(graphics),
            traffic: Value::literal(traffic),
            sensors: Value::literal(sensors),
        }
    }
    
    pub fn visible() -> Self {
        Self::new(true, true, true)
    }
    
    pub fn invisible() -> Self {
        Self::new(false, false, false)
    }
    
    pub fn graphics_only() -> Self {
        Self::new(true, false, false)
    }
    
    pub fn traffic_only() -> Self {
        Self::new(false, true, false)
    }
}
```

#### Step 4: Integration with PrivateAction

```rust
// In src/types/actions/mod.rs - update PrivateAction enum:
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrivateAction {
    // ... existing variants
    #[serde(rename = "AppearanceAction")]
    Appearance(AppearanceAction),
    #[serde(rename = "VisibilityAction")]
    Visibility(VisibilityAction),
}
```

#### Step 5: Comprehensive Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_action_creation() {
        let action = AnimationAction::vehicle_component(VehicleComponentType::Hood)
            .with_loop(true)
            .with_duration(5.0)
            .with_state(0.5);
        
        assert!(action.loop_animation.unwrap().as_literal().unwrap());
        assert_eq!(action.animation_duration.unwrap().as_literal().unwrap(), &5.0);
        assert_eq!(action.animation_state.unwrap().state.as_literal().unwrap(), &0.5);
    }

    #[test]
    fn test_pedestrian_animation() {
        let animation = PedestrianAnimation::new()
            .with_motion(PedestrianMotionType::Walking)
            .add_gesture(PedestrianGestureType::WavingRightArm)
            .add_gesture(PedestrianGestureType::PhoneCallLeftHand);
        
        assert_eq!(animation.motion, Some(PedestrianMotionType::Walking));
        assert_eq!(animation.pedestrian_gestures.len(), 2);
    }

    #[test]
    fn test_animation_file() {
        let file_animation = AnimationFile::new("animations/door_open.fbx")
            .with_time_offset(1.5);
        
        assert_eq!(file_animation.file.filepath.as_literal().unwrap(), "animations/door_open.fbx");
        assert_eq!(file_animation.time_offset.unwrap().as_literal().unwrap(), &1.5);
    }

    #[test]
    fn test_visibility_action() {
        let visible = VisibilityAction::visible();
        assert!(visible.graphics.as_literal().unwrap());
        assert!(visible.traffic.as_literal().unwrap());
        assert!(visible.sensors.as_literal().unwrap());
        
        let graphics_only = VisibilityAction::graphics_only();
        assert!(graphics_only.graphics.as_literal().unwrap());
        assert!(!graphics_only.traffic.as_literal().unwrap());
        assert!(!graphics_only.sensors.as_literal().unwrap());
    }

    #[test]
    fn test_appearance_action() {
        let animation = AnimationAction::from_file("test.fbx");
        let appearance = AppearanceAction::animation(animation);
        
        match appearance {
            AppearanceAction::Animation(ref anim) => {
                match anim.animation_type {
                    AnimationType::File(ref file) => {
                        assert_eq!(file.file.filepath.as_literal().unwrap(), "test.fbx");
                    }
                    _ => panic!("Expected File animation type"),
                }
            }
            _ => panic!("Expected Animation appearance action"),
        }
    }

    #[test]
    fn test_xml_serialization() {
        let action = AnimationAction::pedestrian_motion(PedestrianMotionType::Running)
            .with_loop(false)
            .with_duration(10.0);
        
        let xml = quick_xml::se::to_string(&action).unwrap();
        assert!(xml.contains("AnimationAction"));
        assert!(xml.contains("PedestrianAnimation"));
        assert!(xml.contains("loop=\"false\""));
    }
}
```

#### Step 6: Update Module Exports

```rust
// In src/types/actions/mod.rs:
pub mod animation;
pub mod appearance;
pub use animation::*;
pub use appearance::*;

// In src/types/mod.rs:
pub use actions::{AnimationAction, AppearanceAction, VisibilityAction};
```

### Validation Criteria

- [ ] Complete animation system per XSD specification
- [ ] Support for vehicle, pedestrian, and file-based animations
- [ ] Integration with appearance and visibility actions
- [ ] Builder patterns for easy construction
- [ ] Complete test coverage with XML validation
- [ ] Integration with PrivateAction system

---

## Summary: Phase 1 Implementation Plan

**Phase 1 delivers 23 critical missing types in 6 weeks, bringing coverage from 69.4% to 76%:**

### Week-by-Week Progress

- **Week 1**: Lane Actions (3 types) - Essential automotive maneuvers
- **Week 2**: Traffic Actions (4 types) - Traffic signal and swarm control
- **Week 3**: Advanced Conditions (5 types) - Complex scenario triggers
- **Week 4**: Vehicle Components (4 types) - Enhanced vehicle modeling  
- **Week 5**: Routing & Navigation (3 types) - Path planning capabilities
- **Week 6**: Animation & Visual (4 types) - Visual simulation features

### Implementation Quality Standards

- **XSD Compliance**: Every type matches XSD specification exactly
- **Type Safety**: Full `Value<T>` integration with parameter support
- **XML Compatibility**: Complete serde serialization/deserialization
- **Test Coverage**: 100% test coverage with XML round-trip validation
- **Builder Patterns**: Ergonomic APIs for easy construction
- **Integration**: Seamless integration with existing action/condition systems

### Success Metrics

- **276/346 types implemented** (76% coverage)
- **Real-world automotive scenarios** fully supported
- **Production-ready quality** with comprehensive testing
- **Zero regressions** in existing functionality

This detailed plan provides agents with complete specifications for implementing all critical missing types systematically, ensuring high-quality, production-ready OpenSCENARIO support for real-world automotive simulation scenarios.

---

*This document serves as the definitive implementation guide for Phase 1 critical missing types. Continue with Phase 2-4 plans for complete 100% XSD coverage.*

