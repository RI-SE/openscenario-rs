# OpenSCENARIO XSD Datatypes Reference

This document provides a comprehensive reference of all datatypes, parameters, and complex types defined in the OpenSCENARIO XSD schema.

## Summary

| Category | Count | Implemented | Description |
|----------|--------|------------|-------------|
| Basic Data Types | 9 | 9 | Fundamental data types with validation patterns |
| Simple Enumeration Types | 37 | 41 | Predefined value lists |
| Complex Types | 287 | 66 | Structured data with elements and attributes |
| Groups | 14 | 1 | Reusable element collections |
| **Total** | **347** | **117** | **Complete type system** |

---

## 1. Basic Data Types

| Type | Base Type | Validation Pattern/Union | Description | Status |
|------|-----------|-------------------------|-------------|--------|
| `parameter` | xsd:string | Pattern: `[$][A-Za-z_][A-Za-z0-9_]*` | Parameter reference starting with $ | ✅ Implemented |
| `expression` | xsd:string | Pattern: `[$][{][ A-Za-z0-9_\+\-\*/%$\(\)\.,]*[\}]` | Mathematical expression in ${} format | ✅ Implemented |
| `Boolean` | Union | expression \| parameter \| xsd:boolean | Boolean value or reference | ✅ Implemented |
| `DateTime` | Union | parameter \| xsd:dateTime | Date/time value or reference | ✅ Implemented |
| `Double` | Union | expression \| parameter \| xsd:double | Double precision number or reference | ✅ Implemented |
| `Int` | Union | expression \| parameter \| xsd:int | Integer value or reference | ✅ Implemented |
| `String` | Union | parameter \| xsd:string | String value or reference | ✅ Implemented |
| `UnsignedInt` | Union | expression \| parameter \| xsd:unsignedInt | Unsigned integer or reference | ✅ Implemented |
| `UnsignedShort` | Union | expression \| parameter \| xsd:unsignedShort | Unsigned short integer or reference | ✅ Implemented |

---

## 2. Simple Enumeration Types

### 2.1 Geometry & Positioning

| Type | Values | Description | Status |
|------|--------|-------------|--------|
| `AngleType` | heading, pitch, roll | Types of angular measurements | 🚧 Planned |
| `CoordinateSystem` | entity, lane, road, trajectory, world | Reference coordinate systems | ✅ Implemented |
| `DirectionalDimension` | longitudinal, lateral, vertical | Directional axes | 🚧 Planned |
| `ReferenceContext` | absolute, relative | Position reference type | 🚧 Planned |
| `RelativeDistanceType` | lateral, longitudinal, cartesianDistance¹, euclidianDistance | Distance measurement types | ✅ Implemented |

### 2.2 Vehicle & Object Categories

| Type | Values | Description | Status |
|------|--------|-------------|--------|
| `VehicleCategory` | bicycle, bus, car, motorbike, semitrailer, trailer, train, tram, truck, van | Vehicle classification | ✅ Implemented |
| `PedestrianCategory` | animal, pedestrian, wheelchair | Pedestrian types | ✅ Implemented |
| `ObjectType` | miscellaneous, pedestrian, vehicle, external | General object classification | ✅ Implemented |
| `MiscObjectCategory` | barrier, building, crosswalk, gantry, none, obstacle, parkingSpace, patch, pole, railing, roadMark, soundBarrier, streetLamp, trafficIsland, tree, vegetation, wind¹ | Miscellaneous object types | 🚧 Planned |
| `Role` | none, ambulance, civil, fire, military, police, publicTransport, roadAssistance | Entity roles/purposes | 🚧 Planned |

### 2.3 Vehicle Components & Lights

| Type | Values | Description | Status |
|------|--------|-------------|--------|
| `VehicleComponentType` | hood, trunk, doorFrontRight, doorFrontLeft, doorRearRight, doorRearLeft, windowFrontRight, windowFrontLeft, windowRearRight, windowRearLeft, sideMirrors, sideMirrorRight, sideMirrorLeft | Vehicle body components | 🚧 Planned |
| `VehicleLightType` | daytimeRunningLights, lowBeam, highBeam, fogLights, fogLightsFront, fogLightsRear, brakeLights, warningLights, indicatorLeft, indicatorRight, reversingLights, licensePlateIllumination, specialPurposeLights | Vehicle lighting systems | 🚧 Planned |
| `AutomaticGearType` | n, p, r, d | Automatic transmission gears | 🚧 Planned |

### 2.4 Pedestrian Behavior

| Type | Values | Description | Status |
|------|--------|-------------|--------|
| `PedestrianMotionType` | standing, sitting, lying, squatting, walking, running, reeling, crawling, cycling, jumping, ducking, bendingDown | Pedestrian movement types | 🚧 Planned |
| `PedestrianGestureType` | phoneCallRightHand, phoneCallLeftHand, phoneTextRightHand, phoneTextLeftHand, wavingRightArm, wavingLeftArm, umbrellaRightHand, umbrellaLeftHand, crossArms, coffeeRightHand, coffeeLeftHand, sandwichRightHand, sandwichLeftHand | Pedestrian gestures | 🚧 Planned |

### 2.5 Environment & Weather

| Type | Values | Description | Status |
|------|--------|-------------|--------|
| `ColorType` | other, red, yellow, green, blue, violet, orange, brown, black, grey, white | Color definitions | 🚧 Planned |
| `PrecipitationType` | dry, rain, snow | Weather precipitation | 🚧 Planned |
| `Wetness` | dry, moist, wetWithPuddles, lowFlooded, highFlooded | Road surface conditions | 🚧 Planned |
| `FractionalCloudCover` | zeroOktas, oneOktas, twoOktas, threeOktas, fourOktas, fiveOktas, sixOktas, sevenOktas, eightOktas, nineOktas | Cloud coverage levels | 🚧 Planned |
| `CloudState`¹ | cloudy, free, overcast, rainy, skyOff | Sky conditions (deprecated) | 🚧 Planned |

### 2.6 Control & Dynamics

| Type | Values | Description | Status |
|------|--------|-------------|--------|
| `ControllerType` | lateral, longitudinal, lighting, animation, movement, appearance, all | Controller system types | 🚧 Planned |
| `DynamicsDimension` | distance, rate, time | Dynamics measurement types | ✅ Implemented |
| `DynamicsShape` | cubic, linear, sinusoidal, step | Transition curve shapes | ✅ Implemented |
| `FollowingMode` | follow, position | Trajectory following modes | ✅ Implemented |
| `LightMode` | on, off, flashing | Light operation modes | 🚧 Planned |
| `SpeedTargetValueType` | delta, factor | Speed target calculation methods | ✅ Implemented |

### 2.7 Conditions & Logic

| Type | Values | Description | Status |
|------|--------|-------------|--------|
| `ConditionEdge` | falling, none, rising, risingOrFalling | Condition trigger edges | ✅ Implemented |
| `Rule` | equalTo, greaterThan, lessThan, greaterOrEqual, lessOrEqual, notEqualTo | Comparison operators | ✅ Implemented |
| `TriggeringEntitiesRule` | all, any | Entity trigger logic | ✅ Implemented |

### 2.8 Scenario Structure

| Type | Values | Description | Status |
|------|--------|-------------|--------|
| `Priority` | overwrite¹, override, parallel, skip | Action priority levels | ✅ Implemented |
| `StoryboardElementState` | completeState, endTransition, runningState, skipTransition, standbyState, startTransition, stopTransition | Storyboard element states | ✅ Implemented |
| `StoryboardElementType` | act, action, event, maneuver, maneuverGroup, story | Storyboard element types | ✅ Implemented |

### 2.9 Routing & Navigation

| Type | Values | Description | Status |
|------|--------|-------------|--------|
| `RouteStrategy` | fastest, leastIntersections, random, shortest | Route calculation strategies | 🚧 Planned |
| `RoutingAlgorithm` | assignedRoute, fastest, leastIntersections, shortest, undefined | Routing algorithms | 🚧 Planned |

### 2.10 Spatial Relationships

| Type | Values | Description | Status |
|------|--------|-------------|--------|
| `LateralDisplacement` | any, leftToReferencedEntity, rightToReferencedEntity | Lateral position relationships | 🚧 Planned |
| `LongitudinalDisplacement` | any, trailingReferencedEntity, leadingReferencedEntity | Longitudinal position relationships | 🚧 Planned |

### 2.11 Data Types

| Type | Values | Description | Status |
|------|--------|-------------|--------|
| `ParameterType` | boolean, dateTime, double, integer¹, string, unsignedInt, unsignedShort, int | Parameter data types | ✅ Implemented |

¹ *Deprecated element*

---

## 3. Complex Types by Category

### 3.1 Actions (48 types)

#### 3.1.1 Movement Actions

| Type | Description | Key Attributes/Elements | Status |
|------|-------------|------------------------|--------|
| `SpeedAction` | Change entity speed | SpeedActionDynamics, SpeedActionTarget | ✅ Implemented |
| `LaneChangeAction` | Change lanes | LaneChangeActionDynamics, LaneChangeTarget, targetLaneOffset | 🚧 Planned |
| `LaneOffsetAction` | Adjust lane offset | LaneOffsetActionDynamics, LaneOffsetTarget, continuous | 🚧 Planned |
| `TeleportAction` | Instantly move entity | Position | ✅ Implemented |
| `AcquirePositionAction` | Move to specific position | Position | 🚧 Planned |
| `FollowTrajectoryAction` | Follow predefined path | Trajectory/CatalogReference, TimeReference, TrajectoryFollowingMode | ✅ Implemented |
| `LateralAction` | Generic lateral movement | LaneChangeAction \| LaneOffsetAction \| LateralDistanceAction | 🚧 Planned |
| `LongitudinalAction` | Generic longitudinal movement | SpeedAction \| LongitudinalDistanceAction \| SpeedProfileAction | ✅ Implemented |
| `LateralDistanceAction` | Maintain lateral distance | entityRef, continuous, distance, freespace, displacement | 🚧 Planned |
| `LongitudinalDistanceAction` | Maintain longitudinal distance | entityRef, continuous, distance, freespace, timeGap, displacement | 🚧 Planned |
| `SynchronizeAction` | Synchronize with another entity | masterEntityRef, TargetPositionMaster, TargetPosition, FinalSpeed | 🚧 Planned |

#### 3.1.2 Control Actions

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `ActivateControllerAction` | Enable/disable controllers | controllerRef¹, objectControllerRef, lateral, longitudinal, animation, lighting |
| `AssignControllerAction` | Assign controller to entity | Controller/CatalogReference/ObjectController, activate* flags |
| `ControllerAction` | Generic controller action | AssignControllerAction \| OverrideControllerValueAction \| ActivateControllerAction |
| `OverrideControllerValueAction` | Override control inputs | Throttle, Brake, Clutch, ParkingBrake, SteeringWheel, Gear |
| `OverrideThrottleAction` | Override throttle | active, value, maxRate |
| `OverrideBrakeAction` | Override brake | active, BrakeInput, value¹ |
| `OverrideSteeringWheelAction` | Override steering | active, value, maxRate, maxTorque |
| `OverrideClutchAction` | Override clutch | active, value, maxRate |
| `OverrideGearAction` | Override gear | active, Gear, number¹ |
| `OverrideParkingBrakeAction` | Override parking brake | active, BrakeInput, value¹ |

#### 3.1.3 Appearance Actions

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `AppearanceAction` | Change entity appearance | LightStateAction \| AnimationAction |
| `LightStateAction` | Control lighting | LightType, LightState, transitionTime |
| `AnimationAction` | Animate entity | AnimationType, AnimationState, loop, animationDuration |

#### 3.1.4 Global Actions

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `GlobalAction` | Scene-level actions | EnvironmentAction \| EntityAction \| InfrastructureAction \| SetMonitorAction \| ParameterAction¹ \| TrafficAction \| VariableAction | ✅ Implemented |
| `EnvironmentAction` | Change environment | Environment \| CatalogReference | ✅ Implemented |
| `EntityAction` | Entity lifecycle | AddEntityAction \| DeleteEntityAction, entityRef |
| `AddEntityAction` | Add entity to scene | Position |
| `DeleteEntityAction` | Remove entity from scene | (no attributes) |
| `InfrastructureAction` | Infrastructure control | TrafficSignalAction |
| `SetMonitorAction` | Set monitor state | monitorRef, value |

#### 3.1.5 Traffic Actions

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `TrafficAction` | Traffic control | TrafficSourceAction \| TrafficSinkAction \| TrafficSwarmAction \| TrafficAreaAction \| TrafficStopAction, trafficName |
| `TrafficSourceAction` | Generate traffic | Position, TrafficDefinition¹, TrafficDistribution, radius, rate, velocity¹, speed |
| `TrafficSinkAction` | Remove traffic | Position, TrafficDefinition¹, radius, rate |
| `TrafficSwarmAction` | Swarm traffic around object | CentralObject, TrafficDefinition¹, TrafficDistribution, InitialSpeedRange, DirectionOfTravelDistribution, innerRadius, numberOfVehicles, offset, semiMajorAxis, semiMinorAxis, velocity¹ |
| `TrafficAreaAction` | Traffic in area | TrafficDistribution, TrafficArea, numberOfEntities, continuous |
| `TrafficStopAction` | Stop traffic | (no attributes) |

#### 3.1.6 Signal Actions

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `TrafficSignalAction` | Control traffic signals | TrafficSignalControllerAction \| TrafficSignalStateAction |
| `TrafficSignalControllerAction` | Control signal controller | trafficSignalControllerRef, phase |
| `TrafficSignalStateAction` | Set signal state | name, statee |

#### 3.1.7 Routing Actions

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `RoutingAction` | Route planning | AssignRouteAction \| FollowTrajectoryAction \| AcquirePositionAction \| RandomRouteAction | ✅ Implemented |
| `AssignRouteAction` | Assign route to entity | Route \| CatalogReference |
| `RandomRouteAction` | Generate random route | (no attributes) |

#### 3.1.8 Trailer Actions

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `TrailerAction` | Trailer operations | ConnectTrailerAction \| DisconnectTrailerAction |
| `ConnectTrailerAction` | Connect trailer | trailerRef |
| `DisconnectTrailerAction` | Disconnect trailer | (no attributes) |

#### 3.1.9 Variable Actions

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `VariableAction` | Variable manipulation | SetAction \| ModifyAction, variableRef |
| `VariableSetAction` | Set variable value | value |
| `VariableModifyAction` | Modify variable | Rule |

#### 3.1.10 Deprecated Actions

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `ParameterAction`¹ | Parameter manipulation | SetAction \| ModifyAction, parameterRef |
| `ParameterSetAction`¹ | Set parameter value | value |
| `ParameterModifyAction`¹ | Modify parameter | Rule |

#### 3.1.11 Private Actions

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `PrivateAction` | Entity-specific actions | LongitudinalAction \| LateralAction \| VisibilityAction \| SynchronizeAction \| ActivateControllerAction¹ \| ControllerAction \| TeleportAction \| RoutingAction \| AppearanceAction \| TrailerAction |
| `VisibilityAction` | Control entity visibility | SensorReferenceSet, graphics, sensors, traffic |

#### 3.1.12 User-Defined Actions

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `UserDefinedAction` | Custom actions | CustomCommandAction |
| `CustomCommandAction` | Custom command | type, content |

### 3.2 Conditions (21 types)

#### 3.2.1 Entity-Based Conditions

| Type | Description | Key Attributes/Elements | Status |
|------|-------------|------------------------|--------|
| `ByEntityCondition` | Entity-based triggers | TriggeringEntities, EntityCondition | ✅ Implemented |
| `CollisionCondition` | Collision detection | EntityRef \| ByType | 🚧 Planned |
| `DistanceCondition` | Distance-based trigger | Position, alongRoute¹, freespace, rule, value, coordinateSystem, relativeDistanceType, routingAlgorithm | 🚧 Planned |
| `RelativeDistanceCondition` | Relative distance trigger | entityRef, freespace, relativeDistanceType, rule, value, coordinateSystem, routingAlgorithm | 🚧 Planned |
| `RelativeClearanceCondition` | Clearance check | RelativeLaneRange, EntityRef, oppositeLanes, distanceForward, distanceBackward, freeSpace | 🚧 Planned |
| `SpeedCondition` | Speed-based trigger | rule, value, direction | ✅ Implemented |
| `RelativeSpeedCondition` | Relative speed trigger | entityRef, rule, value, direction | 🚧 Planned |
| `AccelerationCondition` | Acceleration trigger | rule, value, direction | 🚧 Planned |
| `TimeHeadwayCondition` | Following distance trigger | entityRef, alongRoute¹, freespace, rule, value, coordinateSystem, relativeDistanceType, routingAlgorithm | 🚧 Planned |
| `TimeToCollisionCondition` | Time to collision trigger | TimeToCollisionConditionTarget, alongRoute¹, freespace, rule, value, relativeDistanceType, coordinateSystem, routingAlgorithm | 🚧 Planned |
| `TraveledDistanceCondition` | Distance traveled trigger | value | 🚧 Planned |
| `StandStillCondition` | Stationary state trigger | duration | 🚧 Planned |
| `EndOfRoadCondition` | End of road trigger | duration | 🚧 Planned |
| `OffroadCondition` | Off-road state trigger | duration | 🚧 Planned |
| `AngleCondition` | Angle-based trigger | angleType, angle, angleTolerance, coordinateSystem | 🚧 Planned |
| `RelativeAngleCondition` | Relative angle trigger | entityRef, angleType, angle, angleTolerance, coordinateSystem | 🚧 Planned |

#### 3.2.2 Value-Based Conditions

| Type | Description | Key Attributes/Elements | Status |
|------|-------------|------------------------|--------|
| `ByValueCondition` | Value-based triggers | ParameterCondition \| TimeOfDayCondition \| SimulationTimeCondition \| StoryboardElementStateCondition \| UserDefinedValueCondition \| TrafficSignalCondition \| TrafficSignalControllerCondition \| VariableCondition | ✅ Implemented |
| `ParameterCondition` | Parameter value trigger | parameterRef, rule, value | 🚧 Planned |
| `VariableCondition` | Variable value trigger | variableRef, rule, value | 🚧 Planned |
| `TimeOfDayCondition` | Time-based trigger | dateTime, rule | 🚧 Planned |
| `SimulationTimeCondition` | Simulation time trigger | rule, value | ✅ Implemented |
| `StoryboardElementStateCondition` | Element state trigger | storyboardElementRef, statee, storyboardElementType | 🚧 Planned |
| `TrafficSignalCondition` | Traffic signal trigger | name, statee | 🚧 Planned |
| `TrafficSignalControllerCondition` | Signal controller trigger | trafficSignalControllerRef, phase | 🚧 Planned |
| `UserDefinedValueCondition` | Custom value trigger | name, rule, value | 🚧 Planned |

#### 3.2.3 Deprecated Conditions

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `ReachPositionCondition`¹ | Position reached trigger | Position, tolerance |

### 3.3 Positions (15 types)

#### 3.3.1 World Positions

| Type | Description | Key Attributes/Elements | Status |
|------|-------------|------------------------|--------|
| `Position` | Generic position | WorldPosition \| RelativeWorldPosition \| RelativeObjectPosition \| RoadPosition \| RelativeRoadPosition \| LanePosition \| RelativeLanePosition \| RoutePosition \| GeoPosition \| TrajectoryPosition | ✅ Implemented |
| `WorldPosition` | Absolute world coordinates | x, y, z, h, p, r | ✅ Implemented |
| `RelativeWorldPosition` | Relative to entity in world | entityRef, dx, dy, dz, Orientation | ✅ Implemented |

#### 3.3.2 Road-Based Positions

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `RoadPosition` | Road coordinate position | roadId, s, t, Orientation |
| `RelativeRoadPosition` | Relative road position | entityRef, ds, dt, Orientation |
| `LanePosition` | Lane-based position | laneId, roadId, s, offset, Orientation |
| `RelativeLanePosition` | Relative lane position | entityRef, dLane, ds, offset, dsLane, Orientation |

#### 3.3.3 Geographic & Object Positions

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `GeoPosition` | GPS coordinates | latitude¹, longitude¹, height¹, latitudeDeg, longitudeDeg, altitude, verticalRoadSelection, Orientation |
| `RelativeObjectPosition` | Relative to object | entityRef, dx, dy, dz, Orientation |

#### 3.3.4 Route & Trajectory Positions

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `RoutePosition` | Position along route | RouteRef, Orientation, InRoutePosition |
| `TrajectoryPosition` | Position along trajectory | TrajectoryRef, s, t, Orientation |
| `InRoutePosition` | Route internal position | FromCurrentEntity \| FromRoadCoordinates \| FromLaneCoordinates |
| `PositionInLaneCoordinates` | Lane coordinates in route | laneId, laneOffset, pathS |
| `PositionInRoadCoordinates` | Road coordinates in route | pathS, t |
| `PositionOfCurrentEntity` | Current entity position | entityRef |

### 3.4 Entities (12 types)

#### 3.4.1 Entity Definitions

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `Vehicle` | Vehicle definition | name, vehicleCategory, role, mass, model3d, ParameterDeclarations, BoundingBox, Performance, Axles, Properties, TrailerHitch, TrailerCoupler, Trailer | ✅ Implemented |
| `Pedestrian` | Pedestrian definition | name, mass, pedestrianCategory, model¹, model3d, role, ParameterDeclarations, BoundingBox, Properties |
| `MiscObject` | Miscellaneous object | name, mass, miscObjectCategory, model3d, ParameterDeclarations, BoundingBox, Properties |
| `ExternalObjectReference` | External object reference | name |

#### 3.4.2 Entity Management

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `ScenarioObject` | Scenario entity | name, EntityObject, ObjectController |
| `ScenarioObjectTemplate` | Entity template | EntityObject, ObjectController |
| `Entities` | Entity collection | ScenarioObject, EntitySelection |
| `EntitySelection` | Entity selection | name, Members |
| `SelectedEntities` | Selected entity list | EntityRef \| ByType |
| `EntityRef` | Entity reference | entityRef |
| `ByType` | Selection by type | objectType |
| `ByObjectType` | Object type selection | type |

### 3.5 Geometry & Shapes (12 types)

#### 3.5.1 Shape Definitions

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `Shape` | Generic shape | Polyline \| Clothoid \| ClothoidSpline \| Nurbs | ✅ Implemented |
| `Polyline` | Connected line segments | Vertex (2+) | ✅ Implemented |
| `Clothoid` | Clothoid curve | Position, curvature, curvatureDot¹, length, startTime, stopTime, curvaturePrime |
| `ClothoidSpline` | Clothoid spline | ClothoidSplineSegment, timeEnd |
| `ClothoidSplineSegment` | Spline segment | PositionStart, curvatureStart, curvatureEnd, length, hOffset, timeStart |
| `Nurbs` | NURBS curve | ControlPoint (2+), Knot (2+), order |

#### 3.5.2 Geometric Elements

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `Vertex` | Polyline vertex | Position, time | ✅ Implemented |
| `ControlPoint` | NURBS control point | Position, time, weight |
| `Knot` | NURBS knot | value |
| `Polygon` | Closed polygon | Position (3+) |
| `BoundingBox` | Object bounds | Center, Dimensions |
| `Dimensions` | Size definition | height, length, width |
| `Center` | Center point | x, y, z |

### 3.6 Distributions & Randomization (18 types)

#### 3.6.1 Distribution Framework

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `ParameterValueDistribution` | Parameter distribution | ScenarioFile, DistributionDefinition |
| `Deterministic` | Deterministic distribution | DeterministicParameterDistribution |
| `Stochastic` | Stochastic distribution | StochasticDistribution, numberOfTestRuns, randomSeed |

#### 3.6.2 Deterministic Distributions

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `DeterministicSingleParameterDistribution` | Single parameter | parameterName, DeterministicSingleParameterDistributionType |
| `DeterministicMultiParameterDistribution` | Multi-parameter | DeterministicMultiParameterDistributionType |
| `DistributionSet` | Value set | Element |
| `DistributionSetElement` | Set element | value |
| `DistributionRange` | Value range | Range, stepWidth |
| `ValueSetDistribution` | Parameter value sets | ParameterValueSet |
| `ParameterValueSet` | Parameter assignments | ParameterAssignment |

#### 3.6.3 Stochastic Distributions

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `StochasticDistribution` | Stochastic parameter | parameterName, StochasticDistributionType |
| `NormalDistribution` | Normal distribution | expectedValue, variance, Range |
| `LogNormalDistribution` | Log-normal distribution | expectedValue, variance, Range |
| `UniformDistribution` | Uniform distribution | Range |
| `PoissonDistribution` | Poisson distribution | expectedValue, Range |
| `Histogram` | Histogram distribution | Bin |
| `HistogramBin` | Histogram bin | Range, weight |
| `ProbabilityDistributionSet` | Probability set | Element |
| `ProbabilityDistributionSetElement` | Probability element | value, weight |
| `UserDefinedDistribution` | Custom distribution | type, content |

### 3.7 Controllers (8 types)

#### 3.7.1 Controller Definitions

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `Controller` | Controller definition | name, controllerType, ParameterDeclarations, Properties |
| `ObjectController` | Object controller | name, CatalogReference \| Controller |

#### 3.7.2 Controller Distribution

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `ControllerDistribution` | Controller distribution | ControllerDistributionEntry |
| `ControllerDistributionEntry` | Distribution entry | weight, Controller \| CatalogReference |

#### 3.7.3 Entity Distributions

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `EntityDistribution` | Entity distribution | EntityDistributionEntry |
| `EntityDistributionEntry` | Entity entry | weight, ScenarioObjectTemplate |
| `VehicleCategoryDistribution` | Vehicle category dist. | VehicleCategoryDistributionEntry |
| `VehicleCategoryDistributionEntry` | Vehicle category entry | category, weight |
| `VehicleRoleDistribution` | Vehicle role distribution | VehicleRoleDistributionEntry |
| `VehicleRoleDistributionEntry` | Vehicle role entry | role, weight |

### 3.8 Traffic Management (8 types)

#### 3.8.1 Traffic Definition

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `TrafficDefinition` | Traffic characteristics | name, VehicleCategoryDistribution, VehicleRoleDistribution, ControllerDistribution |
| `TrafficDistribution` | Traffic distribution | TrafficDistributionEntry |
| `TrafficDistributionEntry` | Traffic entry | weight, EntityDistribution, Properties |

#### 3.8.2 Traffic Areas

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `TrafficArea` | Traffic area definition | Polygon \| RoadRange |
| `CentralSwarmObject` | Swarm center object | entityRef |
| `DirectionOfTravelDistribution` | Travel direction | same, opposite |

#### 3.8.3 Traffic Support

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `RoadRange` | Road segment range | RoadCursor (2+), length |
| `RoadCursor` | Road position cursor | roadId, s, Lane |

### 3.9 Weather & Environment (12 types)

#### 3.9.1 Environment Definition

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `Environment` | Environment settings | name, ParameterDeclarations, TimeOfDay, Weather, RoadCondition | ✅ Implemented |
| `Weather` | Weather conditions | cloudState¹, atmosphericPressure, temperature, fractionalCloudCover, Sun, Fog, Precipitation, Wind, DomeImage | ✅ Implemented |

#### 3.9.2 Weather Components

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `Sun` | Solar conditions | azimuth, elevation, intensity¹, illuminance | ✅ Implemented |
| `Fog` | Fog conditions | visualRange, BoundingBox | ✅ Implemented |
| `Precipitation` | Precipitation | intensity¹, precipitationType, precipitationIntensity | ✅ Implemented |
| `Wind` | Wind conditions | direction, speed |
| `DomeImage` | Sky dome image | DomeFile, azimuthOffset |

#### 3.9.3 Other Environment

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `TimeOfDay` | Time settings | animation, dateTime | ✅ Implemented |
| `RoadCondition` | Road surface | frictionScaleFactor, wetness, Properties | ✅ Implemented |
| `Color` | Color definition | colorType, ColorRgb \| ColorCmyk |
| `ColorRgb` | RGB color | red, green, blue |
| `ColorCmyk` | CMYK color | cyan, magenta, yellow, key |

### 3.10 Catalogs & References (9 types)

#### 3.10.1 Catalog System

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `Catalog` | Catalog definition | name, Vehicle, Controller, Pedestrian, MiscObject, Environment, Maneuver, Trajectory, Route |
| `CatalogReference` | Catalog item reference | catalogName, entryName, ParameterAssignments |
| `CatalogLocations` | Catalog locations | VehicleCatalog, ControllerCatalog, PedestrianCatalog, MiscObjectCatalog, EnvironmentCatalog, ManeuverCatalog, TrajectoryCatalog, RouteCatalog |

#### 3.10.2 Catalog Locations

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `VehicleCatalogLocation` | Vehicle catalog location | Directory |
| `ControllerCatalogLocation` | Controller catalog location | Directory |
| `PedestrianCatalogLocation` | Pedestrian catalog location | Directory |
| `MiscObjectCatalogLocation` | MiscObject catalog location | Directory |
| `EnvironmentCatalogLocation` | Environment catalog location | Directory |
| `ManeuverCatalogLocation` | Maneuver catalog location | Directory |
| `TrajectoryCatalogLocation` | Trajectory catalog location | Directory |
| `RouteCatalogLocation` | Route catalog location | Directory |

### 3.11 Scenario Structure (17 types)

#### 3.11.1 Core Structure

| Type | Description | Key Attributes/Elements | Status |
|------|-------------|------------------------|--------|
| `OpenScenario` | Root element | FileHeader, OpenScenarioCategory | ✅ Implemented |
| `Storyboard` | Main scenario structure | Init, Story, StopTrigger | ✅ Implemented |
| `Story` | Story definition | name, ParameterDeclarations, Act | ✅ Implemented |
| `Act` | Act definition | name, ManeuverGroup, StartTrigger, StopTrigger | ✅ Implemented |
| `ManeuverGroup` | Maneuver group | name, maximumExecutionCount, Actors, CatalogReference, Maneuver | ✅ Implemented |
| `Maneuver` | Maneuver definition | name, ParameterDeclarations, Event | ✅ Implemented |
| `Event` | Event definition | name, maximumExecutionCount, priority, Action, StartTrigger | ✅ Implemented |
| `Action` | Generic action | name, GlobalAction \| UserDefinedAction \| PrivateAction | ✅ Implemented |
| `StoryAction` | Story-level action | name, StoryActionType | ✅ Implemented |
| `StoryActionType` | Story action types | PrivateAction \| GlobalAction \| UserDefinedAction | ✅ Implemented |
| `StoryPrivateAction` | Story private action | PrivateActionType | ✅ Implemented |

#### 3.11.2 Initialization

| Type | Description | Key Attributes/Elements | Status |
|------|-------------|------------------------|--------|
| `Init` | Initialization | Actions | ✅ Implemented |
| `InitActions` | Initial actions | GlobalAction, UserDefinedAction, Private | ✅ Implemented |
| `Private` | Private actions | entityRef, PrivateAction | ✅ Implemented |

#### 3.11.3 Triggers

| Type | Description | Key Attributes/Elements | Status |
|------|-------------|------------------------|--------|
| `Trigger` | Trigger definition | ConditionGroup | ✅ Implemented |
| `ConditionGroup` | Condition group | Condition | ✅ Implemented |
| `Condition` | Condition definition | name, conditionEdge, delay, ByEntityCondition \| ByValueCondition | ✅ Implemented |
| `TriggeringEntities` | Triggering entities | triggeringEntitiesRule, EntityRef | ✅ Implemented |

#### 3.11.4 Actors

| Type | Description | Key Attributes/Elements | Status |
|------|-------------|------------------------|--------|
| `Actors` | Actor selection | selectTriggeringEntities, EntityRef | ✅ Implemented |

### 3.12 Data Containers (25 types)

#### 3.12.1 File & Metadata

| Type | Description | Key Attributes/Elements | Status |
|------|-------------|------------------------|--------|
| `FileHeader` | File metadata | author, date, description, revMajor, revMinor, License, Properties | ✅ Implemented |
| `File` | File reference | filepath | ✅ Implemented |
| `Directory` | Directory path | path | 🚧 Planned |
| `License` | License information | name, resource, spdxId, content | ✅ Implemented |

#### 3.12.2 Properties & Parameters

| Type | Description | Key Attributes/Elements | Status |
|------|-------------|------------------------|--------|
| `Properties` | Property collection | Property, File, CustomContent | ✅ Implemented |
| `Property` | Property definition | name, value | ✅ Implemented |
| `CustomContent` | Custom content | content | ✅ Implemented |
| `ParameterDeclarations` | Parameter declarations | ParameterDeclaration | ✅ Implemented |
| `ParameterDeclaration` | Parameter definition | name, parameterType, value, ConstraintGroup | ✅ Implemented |
| `ParameterAssignments` | Parameter assignments | ParameterAssignment | ✅ Implemented |
| `ParameterAssignment` | Parameter assignment | parameterRef, value | ✅ Implemented |

#### 3.12.3 Variables & Monitors

| Type | Description | Key Attributes/Elements | Status |
|------|-------------|------------------------|--------|
| `VariableDeclarations` | Variable declarations | VariableDeclaration | ✅ Implemented |
| `VariableDeclaration` | Variable definition | name, variableType, value | ✅ Implemented |
| `MonitorDeclarations` | Monitor declarations | MonitorDeclaration | ✅ Implemented |
| `MonitorDeclaration` | Monitor definition | name, value | ✅ Implemented |

#### 3.12.4 Constraints & Ranges

| Type | Description | Key Attributes/Elements | Status |
|------|-------------|------------------------|--------|
| `ValueConstraintGroup` | Constraint group | ValueConstraint | ✅ Implemented |
| `ValueConstraint` | Value constraint | rule, value | ✅ Implemented |
| `Range` | Value range | lowerLimit, upperLimit | ✅ Implemented |

#### 3.12.5 Rules (Deprecated)

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `ModifyRule`¹ | Modify rule | AddValue \| MultiplyByValue |
| `ParameterAddValueRule`¹ | Add value rule | value |
| `ParameterMultiplyByValueRule`¹ | Multiply rule | value |

#### 3.12.6 Variable Rules

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `VariableModifyRule` | Variable modify rule | AddValue \| MultiplyByValue |
| `VariableAddValueRule` | Variable add rule | value |
| `VariableMultiplyByValueRule` | Variable multiply rule | value |

#### 3.12.7 Support Types

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `UsedArea` | Used area definition | Position (2+) |
| `None` | Empty element | (no attributes) |

### 3.13 Dynamics & Physics (13 types)

#### 3.13.1 Vehicle Physics

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `Performance` | Vehicle performance | maxSpeed, maxAcceleration, maxDeceleration, maxAccelerationRate, maxDecelerationRate | ✅ Implemented |
| `DynamicConstraints` | Dynamic limits | maxSpeed, maxAcceleration, maxDeceleration, maxAccelerationRate, maxDecelerationRate | 🚧 Planned |
| `Axles` | Vehicle axles | FrontAxle, RearAxle, AdditionalAxle | ✅ Implemented |
| `Axle` | Axle definition | maxSteering, positionX, positionZ, trackWidth, wheelDiameter | ✅ Implemented |

#### 3.13.2 Control Systems

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `Brake` | Brake control | value, maxRate |
| `ManualGear` | Manual transmission | number |
| `AutomaticGear` | Automatic transmission | gear |

#### 3.13.3 Dynamics

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `TransitionDynamics` | Transition dynamics | dynamicsDimension, dynamicsShape, followingMode, value |
| `LaneOffsetActionDynamics` | Lane offset dynamics | dynamicsShape, maxLateralAcc |
| `FinalSpeed` | Final speed | AbsoluteSpeed \| RelativeSpeedToMaster |
| `AbsoluteSpeed` | Absolute speed target | value, SteadyState |
| `RelativeSpeedToMaster` | Relative speed target | speedTargetValueType, value, SteadyState |

#### 3.13.4 Speed Profiles

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `SpeedProfileAction` | Speed profile | entityRef, followingMode, DynamicConstraints, SpeedProfileEntry |
| `SpeedProfileEntry` | Profile entry | speed, time |

### 3.14 Animation & Lighting (14 types)

#### 3.14.1 Animation Framework

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `AnimationType` | Animation type | ComponentAnimation \| PedestrianAnimation \| AnimationFile \| UserDefinedAnimation |
| `AnimationState` | Animation state | statee |
| `AnimationFile` | Animation file | File, timeOffset |

#### 3.14.2 Component Animation

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `ComponentAnimation` | Component animation | VehicleComponent \| UserDefinedComponent |
| `VehicleComponent` | Vehicle component | vehicleComponentType |
| `UserDefinedComponent` | Custom component | userDefinedComponentType |

#### 3.14.3 Pedestrian Animation

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `PedestrianAnimation` | Pedestrian animation | motion, userDefinedPedestrianAnimation, PedestrianGesture |
| `PedestrianGesture` | Pedestrian gesture | gesture |

#### 3.14.4 Lighting System

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `LightType` | Light type | VehicleLight \| UserDefinedLight |
| `VehicleLight` | Vehicle light | vehicleLightType |
| `UserDefinedLight` | Custom light | userDefinedLightType |
| `LightState` | Light state | mode, luminousIntensity, flashingOnDuration, flashingOffDuration, Color |

#### 3.14.5 User-Defined Elements

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `UserDefinedAnimation` | Custom animation | userDefinedAnimationType |

### 3.15 Routes & Trajectories (12 types)

#### 3.15.1 Route System

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `Route` | Route definition | name, closed, ParameterDeclarations, Waypoint (2+) |
| `Waypoint` | Route waypoint | routeStrategy, Position |
| `RouteRef` | Route reference | Route \| CatalogReference |

#### 3.15.2 Trajectory System

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `Trajectory` | Trajectory definition | name, closed, ParameterDeclarations, Shape | ✅ Implemented |
| `TrajectoryRef` | Trajectory reference | Trajectory \| CatalogReference | 🚧 Planned |
| `TrajectoryFollowingMode` | Following mode | followingMode | ✅ Implemented |

#### 3.15.3 Time References

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `TimeReference` | Time reference | None \| Timing |
| `Timing` | Timing definition | domainAbsoluteRelative, offset, scale |

#### 3.15.4 Target Systems

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `SpeedActionTarget` | Speed target | RelativeTargetSpeed \| AbsoluteTargetSpeed |
| `AbsoluteTargetSpeed` | Absolute speed target | value |
| `RelativeTargetSpeed` | Relative speed target | entityRef, continuous, speedTargetValueType, value |
| `LaneChangeTarget` | Lane change target | RelativeTargetLane \| AbsoluteTargetLane |
| `AbsoluteTargetLane` | Absolute lane target | value |
| `RelativeTargetLane` | Relative lane target | entityRef, value |
| `LaneOffsetTarget` | Lane offset target | RelativeTargetLaneOffset \| AbsoluteTargetLaneOffset |
| `AbsoluteTargetLaneOffset` | Absolute offset target | value |
| `RelativeTargetLaneOffset` | Relative offset target | entityRef, value |

#### 3.15.5 Steady State

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `TargetDistanceSteadyState` | Distance steady state | distance |
| `TargetTimeSteadyState` | Time steady state | time |

### 3.16 Traffic Signals (9 types)

#### 3.16.1 Signal Infrastructure

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `RoadNetwork` | Road network | LogicFile, SceneGraphFile, TrafficSignals, UsedArea |
| `TrafficSignals` | Traffic signals | TrafficSignalController |
| `TrafficSignalController` | Signal controller | name, reference, delay, Phase |

#### 3.16.2 Signal Phases

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `Phase` | Signal phase | name, duration, TrafficSignalState, TrafficSignalGroupState |
| `TrafficSignalState` | Signal state | trafficSignalId, statee |
| `TrafficSignalGroupState` | Signal group state | statee |

#### 3.16.3 Collision Targets

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `TimeToCollisionConditionTarget` | Collision target | Position \| EntityRef |

#### 3.16.4 Sensor References

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `SensorReferenceSet` | Sensor reference set | SensorReference |
| `SensorReference` | Sensor reference | name |

### 3.17 Trailer System (3 types)

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `Trailer` | Trailer definition | Trailer \| TrailerRef |
| `TrailerHitch` | Trailer hitch | dx, dz |
| `TrailerCoupler` | Trailer coupler | dx, dz |

### 3.18 Lane System (3 types)

| Type | Description | Key Attributes/Elements |
|------|-------------|------------------------|
| `Lane` | Lane definition | id |
| `RelativeLaneRange` | Lane range | from, to |
| `Orientation` | Orientation | h, p, r, type |

¹ *Deprecated element*

---

## 4. Groups (14 types)

| Group | Elements | Description | Status |
|-------|----------|-------------|--------|
| `BrakeInput` | BrakePercent \| BrakeForce | Brake input methods | 🚧 Planned |
| `CatalogDefinition` | Catalog | Catalog definition group | 🚧 Planned |
| `DeterministicMultiParameterDistributionType` | ValueSetDistribution | Multi-parameter deterministic distribution | 🚧 Planned |
| `DeterministicParameterDistribution` | DeterministicMultiParameterDistribution \| DeterministicSingleParameterDistribution | Deterministic distribution choice | 🚧 Planned |
| `DeterministicSingleParameterDistributionType` | DistributionSet \| DistributionRange \| UserDefinedDistribution | Single parameter deterministic distribution | 🚧 Planned |
| `DistributionDefinition` | Deterministic \| Stochastic | Distribution type choice | 🚧 Planned |
| `EntityObject` | CatalogReference \| Vehicle \| Pedestrian \| MiscObject \| ExternalObjectReference | Entity object types | ✅ Implemented |
| `Gear` | ManualGear \| AutomaticGear | Gear type choice | 🚧 Planned |
| `OpenScenarioCategory` | ScenarioDefinition \| CatalogDefinition \| ParameterValueDistributionDefinition | Top-level scenario categories | 🚧 Planned |
| `ParameterValueDistributionDefinition` | ParameterValueDistribution | Parameter distribution definition | 🚧 Planned |
| `ScenarioDefinition` | ParameterDeclarations, VariableDeclarations, MonitorDeclarations, CatalogLocations, RoadNetwork, Entities, Storyboard | Complete scenario definition | 🚧 Planned |
| `SteadyState` | TargetDistanceSteadyState \| TargetTimeSteadyState | Steady state options | 🚧 Planned |
| `StochasticDistributionType` | ProbabilityDistributionSet \| NormalDistribution \| LogNormalDistribution \| UniformDistribution \| PoissonDistribution \| Histogram \| UserDefinedDistribution | Stochastic distribution types | 🚧 Planned |

---

## Implementation Status Legend

| Status | Description |
|--------|-------------|
| ✅ Implemented | Fully implemented and tested |
| ✅ Partial | Partially implemented with basic functionality |
| 🚧 Planned | Planned for future implementation |
| 🚧 Partial | Partially implemented, more work needed |

---

## Root Element

| Element | Type | Description |
|---------|------|-------------|
| `OpenSCENARIO` | OpenScenario | Root element of OpenSCENARIO documents |

---

This comprehensive reference provides all 347 datatypes, parameters, and structural elements defined in the OpenSCENARIO XSD schema, organized by functional categories for easy navigation and understanding.

## Recent Implementation Updates

### Position System Architecture Fix (2025-09-08)

**Major Breakthrough**: Fixed fundamental Position parsing issue that was blocking real-world OpenSCENARIO file compatibility.

**Problem**: XML structure `<Position><WorldPosition .../></Position>` was incompatible with flattened enum approach.

**Solution**: Implemented Position wrapper pattern:
```rust
// ❌ Previous (Problematic):
pub enum Position {
    WorldPosition(WorldPosition),
    RelativeWorldPosition(RelativeWorldPosition),
}

// ✅ Current (Working):
pub struct Position {
    #[serde(rename = "WorldPosition", skip_serializing_if = "Option::is_none")]
    pub world_position: Option<WorldPosition>,
    #[serde(rename = "RelativeWorldPosition", skip_serializing_if = "Option::is_none")]
    pub relative_world_position: Option<RelativeWorldPosition>,
}
```

**Impact**: 
- ✅ Position elements now parse correctly from real XOSC files
- ✅ TeleportAction with WorldPosition works end-to-end
- ✅ Foundation established for fixing similar XML wrapper structures
- ✅ Critical parsing bottleneck resolved

**Similar Fixes Applied**:
- SpeedActionTarget structure (absolute/relative speed targets)
- LongitudinalAction structure (removed problematic flatten patterns)

This architectural fix represents a major step toward complete OpenSCENARIO file parsing compatibility.

