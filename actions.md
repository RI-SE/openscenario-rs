# OpenSCENARIO Actions Implementation Plan

## Overview

This document outlines the comprehensive implementation plan for adding **44 missing action types** to achieve complete OpenSCENARIO action system coverage. Currently, only 4/48 action types (8%) are implemented. This plan will bring the library to 100% action coverage.

## Current Implementation Status

### ✅ **Implemented Actions (4/48 - 8%)**
- `SpeedAction` - Speed control with dynamics - `src/types/actions/movement.rs:28`
- `TeleportAction` - Instant position changes - `src/types/actions/movement.rs:37`
- `FollowTrajectoryAction` - Trajectory following - `src/types/actions/movement.rs:244`
- `RoutingAction` - Route-based movement - `src/types/actions/movement.rs:186`

### 🔄 **Missing Actions (44/48 - 92%)**
- Movement Actions: 8 missing
- Controller Actions: 9 missing  
- Traffic Management: 8 missing
- Appearance Actions: 4 missing
- Infrastructure & Entity: 8 missing
- Specialized Actions: 7 missing

## Implementation Phases

---

## **Phase 4A: Core Movement Actions** 🚗
**Priority: HIGH** | **Complexity: Medium** | **Estimated Effort: 3-4 days**

### **Objective**
Implement the remaining 8 critical movement actions that are essential for realistic vehicle behavior and positioning in scenarios.

### **Actions to Implement**

#### **1. LaneChangeAction**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LaneChangeAction {
    #[serde(rename = "@targetLaneOffset")]
    pub target_lane_offset: Option<f64>,
    #[serde(rename = "LaneChangeActionDynamics")]
    pub lane_change_action_dynamics: TransitionDynamics,
    #[serde(rename = "LaneChangeTarget")]
    pub lane_change_target: LaneChangeTarget,
}
```

#### **2. LaneOffsetAction**  
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LaneOffsetAction {
    #[serde(rename = "@continuous")]
    pub continuous: Boolean,
    #[serde(rename = "LaneOffsetActionDynamics")]
    pub lane_offset_action_dynamics: TransitionDynamics,
    #[serde(rename = "LaneOffsetTarget")]
    pub lane_offset_target: LaneOffsetTarget,
}
```

#### **3. LateralAction**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LateralAction {
    #[serde(flatten)]
    pub lateral_action_choice: LateralActionChoice,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum LateralActionChoice {
    LaneChangeAction(LaneChangeAction),
    LaneOffsetAction(LaneOffsetAction), 
    LateralDistanceAction(LateralDistanceAction),
}
```

#### **4. LateralDistanceAction**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LateralDistanceAction {
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,
    #[serde(rename = "@distance")]
    pub distance: f64,
    #[serde(rename = "@freespace")]
    pub freespace: Option<Boolean>,
    #[serde(rename = "@continuous")]
    pub continuous: Boolean,
    #[serde(rename = "DynamicConstraints")]
    pub dynamic_constraints: Option<DynamicConstraints>,
}
```

#### **5. LongitudinalAction**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LongitudinalAction {
    #[serde(flatten)]
    pub longitudinal_action_choice: LongitudinalActionChoice,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum LongitudinalActionChoice {
    SpeedAction(SpeedAction),
    LongitudinalDistanceAction(LongitudinalDistanceAction),
    SpeedProfileAction(SpeedProfileAction),
}
```

#### **6. LongitudinalDistanceAction**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LongitudinalDistanceAction {
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,
    #[serde(rename = "@distance")]
    pub distance: f64,
    #[serde(rename = "@freespace")]
    pub freespace: Option<Boolean>,
    #[serde(rename = "@continuous")]
    pub continuous: Boolean,
    #[serde(rename = "DynamicConstraints")]
    pub dynamic_constraints: Option<DynamicConstraints>,
}
```

#### **7. SynchronizeAction**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SynchronizeAction {
    #[serde(rename = "@targetEntityRef")]
    pub target_entity_ref: OSString,
    #[serde(rename = "TargetPositionMaster")]
    pub target_position_master: Position,
    #[serde(rename = "TargetPosition")]
    pub target_position: Position,
    #[serde(rename = "FinalSpeed")]
    pub final_speed: Option<FinalSpeed>,
}
```

#### **8. AcquirePositionAction**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AcquirePositionAction {
    #[serde(rename = "Position")]
    pub position: Position,
}
```

### **Supporting Types to Add**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LaneChangeTarget {
    #[serde(flatten)]
    pub target_choice: LaneChangeTargetChoice,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum LaneChangeTargetChoice {
    RelativeTargetLane(RelativeTargetLane),
    AbsoluteTargetLane(AbsoluteTargetLane),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DynamicConstraints {
    #[serde(rename = "@maxLateralAcc")]
    pub max_lateral_acc: Option<f64>,
    #[serde(rename = "@maxSpeed")]
    pub max_speed: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpeedProfileAction {
    #[serde(rename = "@entityRef")]
    pub entity_ref: Option<OSString>,
    #[serde(rename = "DynamicConstraints")]
    pub dynamic_constraints: Option<DynamicConstraints>,
    #[serde(rename = "Entry", default)]
    pub entries: Vec<SpeedProfileEntry>,
}
```

### **Files to Modify**
1. **`src/types/actions/movement.rs`**
   - Add 8 new action structs and supporting types
   - Update imports and dependencies
   - Add validation implementations
   - Add Default implementations where appropriate

2. **`src/types/actions/mod.rs`**
   - Export new action types
   - Update Action enum with new variants:
     ```rust
     #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
     #[serde(tag = "type")]
     pub enum Action {
         // Existing
         Speed(SpeedAction),
         Teleport(TeleportAction),
         FollowTrajectory(FollowTrajectoryAction),
         Routing(RoutingAction),
         // New Phase 4A
         LaneChange(LaneChangeAction),
         LaneOffset(LaneOffsetAction),
         Lateral(LateralAction),
         LateralDistance(LateralDistanceAction),
         Longitudinal(LongitudinalAction),
         LongitudinalDistance(LongitudinalDistanceAction),
         Synchronize(SynchronizeAction),
         AcquirePosition(AcquirePositionAction),
     }
     ```

3. **`src/types/enums.rs`**
   - Add movement-related enumerations if missing

### **Tasks Breakdown**
- [ ] **Task 4A.1**: Implement LaneChangeAction and supporting types (0.5 days)
- [ ] **Task 4A.2**: Implement LaneOffsetAction and targets (0.5 days)
- [ ] **Task 4A.3**: Implement LateralAction wrapper and LateralDistanceAction (0.5 days)
- [ ] **Task 4A.4**: Implement LongitudinalAction wrapper and LongitudinalDistanceAction (0.5 days)
- [ ] **Task 4A.5**: Implement SynchronizeAction with position synchronization (0.5 days)
- [ ] **Task 4A.6**: Implement AcquirePositionAction (0.25 days)
- [ ] **Task 4A.7**: Update mod.rs exports and Action enum (0.25 days)
- [ ] **Task 4A.8**: Add comprehensive unit tests for all new actions (0.5 days)

---

## **Phase 4B: Controller Actions** 🎮
**Priority: MEDIUM** | **Complexity: Medium** | **Estimated Effort: 2-3 days**

### **Objective**  
Implement controller assignment, activation, and vehicle control override actions for realistic vehicle control simulation.

### **Actions to Implement**

#### **1. AssignControllerAction**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AssignControllerAction {
    #[serde(rename = "Controller")]
    pub controller: Option<Controller>,
    #[serde(rename = "CatalogReference")]
    pub catalog_reference: Option<CatalogReference<CatalogController>>,
}
```

#### **2. ActivateControllerAction**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActivateControllerAction {
    #[serde(rename = "@longitudinal")]
    pub longitudinal: Option<Boolean>,
    #[serde(rename = "@lateral")]
    pub lateral: Option<Boolean>,
    #[serde(rename = "@lighting")]
    pub lighting: Option<Boolean>,
    #[serde(rename = "@animation")]
    pub animation: Option<Boolean>,
}
```

#### **3. OverrideControllerValueAction**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OverrideControllerValueAction {
    #[serde(rename = "Throttle")]
    pub throttle: Option<OverrideControllerValueActionThrottle>,
    #[serde(rename = "Brake")]
    pub brake: Option<OverrideControllerValueActionBrake>,
    #[serde(rename = "Clutch")]
    pub clutch: Option<OverrideControllerValueActionClutch>,
    #[serde(rename = "ParkingBrake")]
    pub parking_brake: Option<OverrideControllerValueActionParkingBrake>,
    #[serde(rename = "SteeringWheel")]
    pub steering_wheel: Option<OverrideControllerValueActionSteeringWheel>,
    #[serde(rename = "Gear")]
    pub gear: Option<OverrideControllerValueActionGear>,
}
```

#### **4-9. Override Actions (Individual)**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OverrideBrakeAction {
    #[serde(rename = "@active")]
    pub active: Boolean,
    #[serde(rename = "@value")]
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OverrideThrottleAction {
    #[serde(rename = "@active")]
    pub active: Boolean,
    #[serde(rename = "@value")]
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OverrideSteeringWheelAction {
    #[serde(rename = "@active")]
    pub active: Boolean,
    #[serde(rename = "@value")]
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OverrideGearAction {
    #[serde(rename = "@active")]
    pub active: Boolean,
    #[serde(rename = "ManualGear")]
    pub manual_gear: Option<ManualGear>,
    #[serde(rename = "AutomaticGear")]
    pub automatic_gear: Option<AutomaticGear>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OverrideParkingBrakeAction {
    #[serde(rename = "@active")]
    pub active: Boolean,
    #[serde(rename = "@force")]
    pub force: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OverrideClutchAction {
    #[serde(rename = "@active")]
    pub active: Boolean,
    #[serde(rename = "@value")]
    pub value: f64,
}
```

### **Supporting Types**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ManualGear {
    #[serde(rename = "@gear")]
    pub gear: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AutomaticGear {
    #[serde(rename = "@gear")]
    pub gear: AutomaticGearType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AutomaticGearType {
    #[serde(rename = "park")]
    Park,
    #[serde(rename = "reverse")]
    Reverse,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "drive")]
    Drive,
}
```

### **Files to Modify**
1. **`src/types/actions/control.rs`**
   - Implement all 9 controller action types
   - Add supporting types and enumerations
   - Integrate with existing controller types

2. **`src/types/actions/mod.rs`**
   - Export controller actions
   - Add to Action enum

### **Tasks Breakdown**
- [ ] **Task 4B.1**: Implement AssignControllerAction with catalog integration (0.5 days)
- [ ] **Task 4B.2**: Implement ActivateControllerAction (0.25 days)
- [ ] **Task 4B.3**: Implement OverrideControllerValueAction composite (0.5 days)
- [ ] **Task 4B.4**: Implement individual override actions (Brake, Throttle, Steering) (0.5 days)
- [ ] **Task 4B.5**: Implement gear and clutch override actions (0.5 days)
- [ ] **Task 4B.6**: Add supporting types and enumerations (0.25 days)
- [ ] **Task 4B.7**: Update exports and integrate with Action enum (0.25 days)
- [ ] **Task 4B.8**: Add controller action unit tests (0.25 days)

---

## **Phase 4C: Traffic Management Actions** 🚦
**Priority: MEDIUM** | **Complexity: High** | **Estimated Effort: 3-4 days**

### **Objective**
Implement comprehensive traffic generation, control, and signal management actions for realistic traffic environments.

### **Actions to Implement**

#### **1. TrafficSourceAction**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficSourceAction {
    #[serde(rename = "@rate")]
    pub rate: f64,
    #[serde(rename = "@velocity")]
    pub velocity: Option<f64>,
    #[serde(rename = "Position")]
    pub position: Position,
    #[serde(rename = "TrafficDefinition")]
    pub traffic_definition: TrafficDefinition,
}
```

#### **2. TrafficSinkAction**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficSinkAction {
    #[serde(rename = "@rate")]
    pub rate: f64,
    #[serde(rename = "@radius")]
    pub radius: f64,
    #[serde(rename = "Position")]
    pub position: Position,
    #[serde(rename = "TrafficDefinition")]
    pub traffic_definition: Option<TrafficDefinition>,
}
```

#### **3. TrafficSwarmAction**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficSwarmAction {
    #[serde(rename = "@centralObject")]
    pub central_object: OSString,
    #[serde(rename = "@innerRadius")]
    pub inner_radius: f64,
    #[serde(rename = "@outerRadius")]
    pub outer_radius: f64,
    #[serde(rename = "@numberOfVehicles")]
    pub number_of_vehicles: u32,
    #[serde(rename = "CentralSwarmObject")]
    pub central_swarm_object: CentralSwarmObject,
    #[serde(rename = "TrafficDefinition")]
    pub traffic_definition: TrafficDefinition,
}
```

#### **4. TrafficAreaAction**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficAreaAction {
    #[serde(rename = "TrafficArea")]
    pub traffic_area: TrafficArea,
    #[serde(rename = "TrafficDefinition")]
    pub traffic_definition: TrafficDefinition,
}
```

#### **5. TrafficSignalAction**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficSignalAction {
    #[serde(flatten)]
    pub signal_action_choice: TrafficSignalActionChoice,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum TrafficSignalActionChoice {
    TrafficSignalControllerAction(TrafficSignalControllerAction),
    TrafficSignalStateAction(TrafficSignalStateAction),
}
```

#### **6. TrafficSignalStateAction**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficSignalStateAction {
    #[serde(rename = "@name")]
    pub name: OSString,
    #[serde(rename = "@state")]
    pub state: OSString,
}
```

#### **7. TrafficSignalControllerAction**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficSignalControllerAction {
    #[serde(rename = "@trafficSignalControllerRef")]
    pub traffic_signal_controller_ref: OSString,
    #[serde(rename = "@phaseRef")]
    pub phase_ref: OSString,
}
```

#### **8. TrafficStopAction**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficStopAction {
    #[serde(rename = "@enable")]
    pub enable: Boolean,
}
```

### **Supporting Types**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficDefinition {
    #[serde(rename = "VehicleCategoryDistribution")]
    pub vehicle_category_distribution: Option<VehicleCategoryDistribution>,
    #[serde(rename = "ControllerDistribution")]
    pub controller_distribution: Option<ControllerDistribution>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VehicleCategoryDistribution {
    #[serde(rename = "VehicleCategoryDistributionEntry", default)]
    pub entries: Vec<VehicleCategoryDistributionEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CentralSwarmObject {
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficArea {
    #[serde(rename = "Vertex", default)]
    pub vertices: Vec<Vertex>,
}
```

### **Files to Modify**
1. **`src/types/actions/traffic.rs`**
   - Implement all 8 traffic action types
   - Add comprehensive traffic management system
   - Integrate with existing traffic types

2. **`src/types/actions/mod.rs`**
   - Export traffic actions
   - Update Action enum

3. **`src/types/enums.rs`**
   - Add traffic-related enumerations

### **Tasks Breakdown**
- [ ] **Task 4C.1**: Implement TrafficSourceAction and TrafficSinkAction (0.5 days)
- [ ] **Task 4C.2**: Implement TrafficSwarmAction with swarm behavior (0.5 days)
- [ ] **Task 4C.3**: Implement TrafficAreaAction and area definitions (0.5 days)
- [ ] **Task 4C.4**: Implement TrafficSignalAction wrapper (0.25 days)
- [ ] **Task 4C.5**: Implement TrafficSignalStateAction and TrafficSignalControllerAction (0.5 days)
- [ ] **Task 4C.6**: Implement TrafficStopAction (0.25 days)
- [ ] **Task 4C.7**: Add TrafficDefinition and supporting types (0.75 days)
- [ ] **Task 4C.8**: Integration testing with traffic scenarios (0.5 days)

---

## **Phase 4D: Appearance & Animation Actions** 🎨
**Priority: LOW** | **Complexity: Low** | **Estimated Effort: 1-2 days**

### **Objective**
Implement visual appearance and animation actions for realistic entity representation and lighting control.

### **Actions to Implement**

#### **1. LightStateAction**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LightStateAction {
    #[serde(rename = "@lightType")]
    pub light_type: LightType,
    #[serde(rename = "@state")]
    pub state: LightState,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LightType {
    #[serde(rename = "headLight")]
    HeadLight,
    #[serde(rename = "tailLight")]
    TailLight,
    #[serde(rename = "brakeLight")]
    BrakeLight,
    #[serde(rename = "indicatorLeft")]
    IndicatorLeft,
    #[serde(rename = "indicatorRight")]
    IndicatorRight,
    #[serde(rename = "hazardWarning")]
    HazardWarning,
    #[serde(rename = "fogLight")]
    FogLight,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LightState {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "flashing")]
    Flashing,
}
```

#### **2. AnimationAction**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnimationAction {
    #[serde(rename = "@animationType")]
    pub animation_type: AnimationType,
    #[serde(rename = "UserDefinedAnimation")]
    pub user_defined_animation: Option<UserDefinedAnimation>,
    #[serde(rename = "PedestrianAnimation")]
    pub pedestrian_animation: Option<PedestrianAnimation>,
    #[serde(rename = "VehicleComponentAnimation")]
    pub vehicle_component_animation: Option<VehicleComponentAnimation>,
}
```

#### **3. VisibilityAction**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VisibilityAction {
    #[serde(rename = "@graphics")]
    pub graphics: Boolean,
    #[serde(rename = "@traffic")]
    pub traffic: Boolean,
    #[serde(rename = "@sensors")]
    pub sensors: Boolean,
}
```

#### **4. AppearanceAction**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppearanceAction {
    #[serde(flatten)]
    pub appearance_action_choice: AppearanceActionChoice,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum AppearanceActionChoice {
    LightStateAction(LightStateAction),
    AnimationAction(AnimationAction),
}
```

### **Files to Modify**
1. **`src/types/actions/appearance.rs`**
   - Implement all 4 appearance action types
   - Add animation and lighting enumerations

2. **`src/types/actions/mod.rs`**
   - Export appearance actions

### **Tasks Breakdown**
- [ ] **Task 4D.1**: Implement LightStateAction with light types and states (0.5 days)
- [ ] **Task 4D.2**: Implement AnimationAction with animation types (0.5 days)
- [ ] **Task 4D.3**: Implement VisibilityAction (0.25 days)
- [ ] **Task 4D.4**: Implement AppearanceAction wrapper (0.25 days)
- [ ] **Task 4D.5**: Add supporting enumerations and types (0.25 days)
- [ ] **Task 4D.6**: Integration and testing (0.25 days)

---

## **Phase 4E: Infrastructure & Entity Actions** 🏗️
**Priority: MEDIUM** | **Complexity: Medium** | **Estimated Effort: 2-3 days**

### **Objective**
Implement entity lifecycle management, parameter control, and infrastructure modification actions.

### **New Files to Create**

#### **1. `src/types/actions/infrastructure.rs`**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InfrastructureAction {
    #[serde(flatten)]
    pub infrastructure_action_choice: InfrastructureActionChoice,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum InfrastructureActionChoice {
    TrafficSignalAction(TrafficSignalAction),
    ParameterAction(ParameterAction),
}
```

#### **2. `src/types/actions/entity.rs`**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AddEntityAction {
    #[serde(rename = "Position")]
    pub position: Position,
    #[serde(rename = "EntityRef")]
    pub entity_ref: EntityRef,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteEntityAction {
    // Empty - deletion is triggered by entity reference in parent
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EntityAction {
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,
    #[serde(flatten)]
    pub entity_action_choice: EntityActionChoice,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum EntityActionChoice {
    AddEntityAction(AddEntityAction),
    DeleteEntityAction(DeleteEntityAction),
}
```

#### **3. `src/types/actions/parameter.rs`**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParameterAction {
    #[serde(rename = "@parameterRef")]
    pub parameter_ref: OSString,
    #[serde(flatten)]
    pub parameter_action_choice: ParameterActionChoice,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ParameterActionChoice {
    ParameterSetAction(ParameterSetAction),
    ParameterModifyAction(ParameterModifyAction),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParameterSetAction {
    #[serde(rename = "@value")]
    pub value: OSString,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParameterModifyAction {
    #[serde(rename = "Rule")]
    pub rule: ParameterModifyRule,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VariableAction {
    #[serde(rename = "@variableRef")]
    pub variable_ref: OSString,
    #[serde(flatten)]
    pub variable_action_choice: VariableActionChoice,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum VariableActionChoice {
    VariableSetAction(VariableSetAction),
    VariableModifyAction(VariableModifyAction),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SetMonitorAction {
    #[serde(rename = "@enable")]
    pub enable: Boolean,
}
```

### **Tasks Breakdown**
- [ ] **Task 4E.1**: Create infrastructure.rs with InfrastructureAction (0.5 days)
- [ ] **Task 4E.2**: Create entity.rs with AddEntityAction and DeleteEntityAction (0.5 days)
- [ ] **Task 4E.3**: Create parameter.rs with parameter and variable actions (1 day)
- [ ] **Task 4E.4**: Implement SetMonitorAction (0.25 days)
- [ ] **Task 4E.5**: Add supporting types and enumerations (0.5 days)
- [ ] **Task 4E.6**: Update mod.rs exports and Action enum (0.25 days)

---

## **Phase 4F: Specialized Actions** ⚙️
**Priority: LOW** | **Complexity: Low** | **Estimated Effort: 1-2 days**

### **New Files to Create**

#### **1. `src/types/actions/trailer.rs`**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrailerAction {
    #[serde(flatten)]
    pub trailer_action_choice: TrailerActionChoice,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum TrailerActionChoice {
    ConnectTrailerAction(ConnectTrailerAction),
    DisconnectTrailerAction(DisconnectTrailerAction),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectTrailerAction {
    #[serde(rename = "Trailer")]
    pub trailer: TrailerRef,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DisconnectTrailerAction {
    // Empty - disconnection is implicit
}
```

#### **2. `src/types/actions/environment.rs`**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnvironmentAction {
    #[serde(rename = "Environment")]
    pub environment: Option<Environment>,
    #[serde(rename = "CatalogReference")]
    pub catalog_reference: Option<CatalogReference<CatalogEnvironment>>,
}
```

### **Tasks Breakdown**
- [ ] **Task 4F.1**: Create trailer.rs with trailer connection actions (0.5 days)
- [ ] **Task 4F.2**: Create environment.rs with EnvironmentAction (0.25 days)
- [ ] **Task 4F.3**: Implement CustomCommandAction and UserDefinedAction (0.5 days)
- [ ] **Task 4F.4**: Final integration and testing (0.25 days)

---

## **Integration & Testing Plan**

### **Global Action System Integration**

#### **Update `src/types/actions/mod.rs`**
```rust
// Complete Action enum with all 48+ action types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum Action {
    // Phase 4A: Movement Actions (12 total)
    Speed(SpeedAction),
    Teleport(TeleportAction),
    FollowTrajectory(FollowTrajectoryAction),
    Routing(RoutingAction),
    LaneChange(LaneChangeAction),
    LaneOffset(LaneOffsetAction),
    Lateral(LateralAction),
    LateralDistance(LateralDistanceAction),
    Longitudinal(LongitudinalAction),
    LongitudinalDistance(LongitudinalDistanceAction),
    Synchronize(SynchronizeAction),
    AcquirePosition(AcquirePositionAction),
    
    // Phase 4B: Controller Actions (9 total)
    AssignController(AssignControllerAction),
    ActivateController(ActivateControllerAction),
    OverrideControllerValue(OverrideControllerValueAction),
    OverrideBrake(OverrideBrakeAction),
    OverrideThrottle(OverrideThrottleAction),
    OverrideSteeringWheel(OverrideSteeringWheelAction),
    OverrideGear(OverrideGearAction),
    OverrideParkingBrake(OverrideParkingBrakeAction),
    OverrideClutch(OverrideClutchAction),
    
    // Phase 4C: Traffic Actions (8 total)
    TrafficSource(TrafficSourceAction),
    TrafficSink(TrafficSinkAction),
    TrafficSwarm(TrafficSwarmAction),
    TrafficArea(TrafficAreaAction),
    TrafficSignal(TrafficSignalAction),
    TrafficSignalState(TrafficSignalStateAction),
    TrafficSignalController(TrafficSignalControllerAction),
    TrafficStop(TrafficStopAction),
    
    // Phase 4D: Appearance Actions (4 total)
    LightState(LightStateAction),
    Animation(AnimationAction),
    Visibility(VisibilityAction),
    Appearance(AppearanceAction),
    
    // Phase 4E: Infrastructure & Entity Actions (8 total)
    Infrastructure(InfrastructureAction),
    AddEntity(AddEntityAction),
    DeleteEntity(DeleteEntityAction),
    Entity(EntityAction),
    Parameter(ParameterAction),
    ParameterSet(ParameterSetAction),
    ParameterModify(ParameterModifyAction),
    Variable(VariableAction),
    VariableSet(VariableSetAction),
    VariableModify(VariableModifyAction),
    SetMonitor(SetMonitorAction),
    
    // Phase 4F: Specialized Actions (7 total)
    Trailer(TrailerAction),
    ConnectTrailer(ConnectTrailerAction),
    DisconnectTrailer(DisconnectTrailerAction),
    Environment(EnvironmentAction),
    CustomCommand(CustomCommandAction),
    UserDefined(UserDefinedAction),
}
```

### **Comprehensive Testing Strategy**

#### **Unit Tests** (`tests/integration_tests.rs`)
```rust
#[cfg(test)]
mod action_tests {
    use super::*;
    
    // Phase 4A: Movement Action Tests
    #[test]
    fn test_lane_change_action_parsing() { /* ... */ }
    
    #[test]
    fn test_lateral_distance_action_validation() { /* ... */ }
    
    #[test]
    fn test_synchronize_action_serialization() { /* ... */ }
    
    // Phase 4B: Controller Action Tests
    #[test]
    fn test_override_brake_action_parsing() { /* ... */ }
    
    #[test] 
    fn test_controller_assignment_with_catalog() { /* ... */ }
    
    // Phase 4C: Traffic Action Tests
    #[test]
    fn test_traffic_source_action_parsing() { /* ... */ }
    
    #[test]
    fn test_traffic_signal_state_changes() { /* ... */ }
    
    // Additional test categories...
}
```

#### **Integration Tests**
- [ ] Parse real XOSC files containing all action types
- [ ] Validate action serialization round-trips
- [ ] Test action validation logic
- [ ] Verify catalog integration for all applicable actions

### **Performance Considerations**
- [ ] Minimize memory allocation in action parsing
- [ ] Optimize enum dispatching for large action sets
- [ ] Implement lazy loading for complex traffic definitions
- [ ] Add benchmarks for action-heavy scenarios

## **Success Criteria**

### **Completion Metrics**
- ✅ **48/48 Action Types Implemented (100% coverage)**
- ✅ **Zero Compilation Errors**
- ✅ **All Integration Tests Passing**
- ✅ **Comprehensive Unit Test Coverage (>90%)**
- ✅ **Real-World XOSC File Compatibility**
- ✅ **Performance Benchmarks Within Acceptable Ranges**

### **Quality Assurance**
- [ ] **Code Review**: All action implementations reviewed for consistency
- [ ] **Documentation**: Each action type documented with examples
- [ ] **Validation**: Comprehensive validation rules for all action types
- [ ] **Error Handling**: Proper error messages for invalid action configurations
- [ ] **Backward Compatibility**: No breaking changes to existing action API

## **Timeline Summary**

| Phase | Duration | Actions | Complexity |
|-------|----------|---------|------------|
| **4A: Movement** | 3-4 days | 8 actions | Medium |
| **4B: Controller** | 2-3 days | 9 actions | Medium |
| **4C: Traffic** | 3-4 days | 8 actions | High |
| **4D: Appearance** | 1-2 days | 4 actions | Low |
| **4E: Infrastructure** | 2-3 days | 8 actions | Medium |
| **4F: Specialized** | 1-2 days | 7 actions | Low |
| **Integration** | 1-2 days | Testing | - |
| **TOTAL** | **13-20 days** | **44 actions** | - |

**Expected Completion**: 3-4 weeks for complete action system implementation.

This plan will bring the OpenSCENARIO-rs library from 8% to 100% action coverage, enabling support for the most complex real-world scenarios and establishing the library as a comprehensive OpenSCENARIO implementation.