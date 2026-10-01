//! Conditions on an entity's own state: speed, acceleration and standstill; distance
//! and collision; end-of-road, off-road and lane clearance; time headway and time to
//! collision. Several have both an absolute and an entity-relative form.
use crate::types::basic::{Boolean, Double, Int, OSString, Value};
use crate::types::enums::{
    AngleType, CoordinateSystem, DirectionalDimension, ObjectType, RelativeDistanceType,
    RoutingAlgorithm, Rule,
};
use crate::types::positions::Position;
use crate::types::scenario::triggers::{EntityRef, TriggeringEntities};
use serde::{Deserialize, Serialize};

#[deprecated(
    since = "0.2.0",
    note = "Use openscenario_rs::types::conditions::spatial::ReachPositionCondition instead"
)]
pub use crate::types::conditions::spatial::ReachPositionCondition;

#[deprecated(
    since = "0.2.0",
    note = "Use openscenario_rs::types::conditions::spatial::DistanceCondition instead"
)]
pub use crate::types::conditions::spatial::DistanceCondition;

#[deprecated(
    since = "0.2.0",
    note = "Use openscenario_rs::types::conditions::spatial::RelativeDistanceCondition instead"
)]
pub use crate::types::conditions::spatial::RelativeDistanceCondition;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpeedCondition {
    /// Speed value to compare against
    #[serde(rename = "@value")]
    pub value: Double,

    /// Comparison rule (greater than, less than, etc.)
    #[serde(rename = "@rule")]
    pub rule: Value<Rule>,

    /// Direction of speed measurement (optional)
    #[serde(rename = "@direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<Value<DirectionalDimension>>,
}

impl SpeedCondition {
    /// Create a new speed condition. XSD:2055-2059 `SpeedCondition` — `value` and `rule` are
    /// both `use="required"`; `direction` is optional and left unset.
    pub fn new(value: f64, rule: Rule) -> Self {
        Self {
            value: Double::literal(value),
            rule: Value::Literal(rule),
            direction: None,
        }
    }

    /// Set direction of speed measurement
    pub fn with_direction(mut self, direction: DirectionalDimension) -> Self {
        self.direction = Some(Value::Literal(direction));
        self
    }
}

/// Condition based on entity acceleration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccelerationCondition {
    /// Acceleration value to compare against
    #[serde(rename = "@value")]
    pub value: Double,

    /// Comparison rule (greater than, less than, etc.)
    #[serde(rename = "@rule")]
    pub rule: Value<Rule>,

    /// Direction of acceleration measurement (optional)
    #[serde(rename = "@direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<Value<DirectionalDimension>>,
}

/// Condition for detecting standstill state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StandStillCondition {
    /// Duration entity must be stationary
    #[serde(rename = "@duration")]
    pub duration: Double,
}

/// Condition for detecting collisions. XSD `CollisionCondition` (`:923-928`) is a bare
/// `xsd:choice` between a specific target entity and a target entity type, so exactly one
/// branch is present; there is no "any collision" state.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CollisionCondition {
    #[serde(rename = "$value")]
    pub choice: CollisionConditionChoice,
}

/// The two `CollisionCondition` branches. XSD:924-926.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CollisionConditionChoice {
    /// Specific target entity — XSD child `<EntityRef entityRef="..."/>`
    EntityRef(EntityRef),
    /// Collision detection by entity type — XSD:926 child `<ByType type="..."/>`
    /// (element name `ByType`, of XSD type `ByObjectType`)
    ByType(CollisionTarget),
}

/// Target specification for collision detection — wraps XSD `<ByType type="..."/>`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CollisionTarget {
    /// XSD:832 required attribute `type` on complexType `ByObjectType`
    #[serde(rename = "@type")]
    pub target_type: Value<ObjectType>,
}

/// Condition for detecting end-of-road state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EndOfRoadCondition {
    /// Duration entity must be at end of road — XSD required attr `duration`
    #[serde(rename = "@duration")]
    pub duration: Double,
}

/// Time headway condition for following distance measurement
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimeHeadwayCondition {
    /// Target entity reference for headway measurement
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,

    /// Headway time value
    #[serde(rename = "@value")]
    pub value: Double,

    /// Comparison rule
    #[serde(rename = "@rule")]
    pub rule: Value<Rule>,

    /// Whether to measure in freespace or bounding box
    #[serde(rename = "@freespace")]
    pub freespace: Boolean,

    /// Whether to measure headway along route (deprecated)
    #[serde(rename = "@alongRoute", skip_serializing_if = "Option::is_none")]
    pub along_route: Option<Boolean>,

    /// Optional coordinate system for measurement
    #[serde(rename = "@coordinateSystem", skip_serializing_if = "Option::is_none")]
    pub coordinate_system: Option<Value<CoordinateSystem>>,

    /// Optional relative distance type
    #[serde(
        rename = "@relativeDistanceType",
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_distance_type: Option<Value<RelativeDistanceType>>,

    /// Optional routing algorithm for route-based measurement
    #[serde(rename = "@routingAlgorithm", skip_serializing_if = "Option::is_none")]
    pub routing_algorithm: Option<Value<RoutingAlgorithm>>,
}

/// Time to collision condition for collision prediction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimeToCollisionCondition {
    /// Time to collision value
    #[serde(rename = "@value")]
    pub value: Double,

    /// Comparison rule
    #[serde(rename = "@rule")]
    pub rule: Value<Rule>,

    /// Whether to measure in freespace or bounding box
    #[serde(rename = "@freespace")]
    pub freespace: Boolean,

    /// Whether to measure along route (deprecated)
    #[serde(rename = "@alongRoute", skip_serializing_if = "Option::is_none")]
    pub along_route: Option<Boolean>,

    /// Optional coordinate system for measurement
    #[serde(rename = "@coordinateSystem", skip_serializing_if = "Option::is_none")]
    pub coordinate_system: Option<Value<CoordinateSystem>>,

    /// Optional relative distance type
    #[serde(
        rename = "@relativeDistanceType",
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_distance_type: Option<Value<RelativeDistanceType>>,

    /// Optional routing algorithm for route-based measurement
    #[serde(rename = "@routingAlgorithm", skip_serializing_if = "Option::is_none")]
    pub routing_algorithm: Option<Value<RoutingAlgorithm>>,

    /// Target specification for collision detection — XSD child `<TimeToCollisionConditionTarget>`
    #[serde(rename = "TimeToCollisionConditionTarget")]
    pub target: TimeToCollisionTarget,
}

/// Target for time to collision condition - matches XSD TimeToCollisionConditionTarget.
/// XSD `TimeToCollisionConditionTarget` (`:2193-2198`) is a bare `xsd:choice`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimeToCollisionTarget {
    #[serde(rename = "$value")]
    pub choice: TimeToCollisionTargetChoice,
}

/// The two `TimeToCollisionConditionTarget` branches. XSD:2194-2196.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TimeToCollisionTargetChoice {
    /// Target position — XSD child `<Position>`
    Position(Position),
    /// Target entity reference — XSD child `<EntityRef>`
    EntityRef(EntityRef),
}

/// Angle condition for entity orientation/direction triggering
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AngleCondition {
    /// Type of angle measurement (relative or absolute) — XSD required attr `angleType`
    #[serde(rename = "@angleType")]
    pub angle_type: Value<AngleType>,

    /// Target angle value in radians — XSD required attr `angle`
    #[serde(rename = "@angle")]
    pub angle: Double,

    /// Tolerance for angle matching in radians — XSD required attr `angleTolerance`
    #[serde(rename = "@angleTolerance")]
    pub angle_tolerance: Double,

    /// Coordinate system for angle measurement — XSD optional attr `coordinateSystem`
    #[serde(
        rename = "@coordinateSystem",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub coordinate_system: Option<Value<CoordinateSystem>>,
}

/// Off-road detection condition - matches XSD OffroadCondition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OffroadCondition {
    /// Duration entity must be off-road — XSD required attr `duration`
    #[serde(rename = "@duration")]
    pub duration: Double,
}

/// Relative speed monitoring between entities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelativeSpeedCondition {
    /// Reference entity for speed comparison — XSD required attr `entityRef`
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,

    /// Comparison rule (greater than, less than, etc.) — XSD required attr `rule`
    #[serde(rename = "@rule")]
    pub rule: Value<Rule>,

    /// Speed difference value — XSD required attr `value`
    #[serde(rename = "@value")]
    pub value: Double,

    /// Direction of speed measurement (optional) — XSD optional attr `direction`
    #[serde(
        rename = "@direction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub direction: Option<Value<DirectionalDimension>>,
}

/// Relative lane range specification for clearance conditions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelativeLaneRange {
    /// Starting lane offset — XSD optional attr `from`
    #[serde(rename = "@from", default, skip_serializing_if = "Option::is_none")]
    pub from: Option<Int>,

    /// Ending lane offset — XSD optional attr `to`
    #[serde(rename = "@to", default, skip_serializing_if = "Option::is_none")]
    pub to: Option<Int>,
}

/// Clearance monitoring between entities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelativeClearanceCondition {
    /// Lane ranges to check for clearance — XSD repeated child `<RelativeLaneRange>`
    #[serde(rename = "RelativeLaneRange", default)]
    pub relative_lane_ranges: Vec<RelativeLaneRange>,

    /// Entity references to check clearance against — XSD repeated child `<EntityRef>`
    #[serde(rename = "EntityRef", default)]
    pub entity_refs: Vec<EntityRef>,

    /// Whether to check opposite lanes — XSD required attr `oppositeLanes`
    #[serde(rename = "@oppositeLanes")]
    pub opposite_lanes: Boolean,

    /// Distance to check forward (optional) — XSD optional attr `distanceForward`
    #[serde(
        rename = "@distanceForward",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_forward: Option<Double>,

    /// Distance to check backward (optional) — XSD optional attr `distanceBackward`
    #[serde(
        rename = "@distanceBackward",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_backward: Option<Double>,

    /// Whether to use free space measurement — XSD required attr `freeSpace`
    #[serde(rename = "@freeSpace")]
    pub free_space: Boolean,
}

/// Relative angle conditions between entities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelativeAngleCondition {
    /// Reference entity for angle comparison — XSD required attr `entityRef`
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,

    /// Type of angle measurement (relative or absolute) — XSD required attr `angleType`
    #[serde(rename = "@angleType")]
    pub angle_type: Value<AngleType>,

    /// Target angle value in radians — XSD required attr `angle`
    #[serde(rename = "@angle")]
    pub angle: Double,

    /// Tolerance for angle matching in radians — XSD required attr `angleTolerance`
    #[serde(rename = "@angleTolerance")]
    pub angle_tolerance: Double,

    /// Coordinate system for angle measurement — XSD optional attr `coordinateSystem`
    #[serde(
        rename = "@coordinateSystem",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub coordinate_system: Option<Value<CoordinateSystem>>,
}

/// Distance-based condition triggering
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TraveledDistanceCondition {
    /// Distance value to trigger on
    #[serde(rename = "@value")]
    pub value: Double,
}

/// Schema-compliant ByEntityCondition structure matching OpenSCENARIO XSD exactly
/// This is the main ByEntityCondition type that should be used
///
/// No `Default` impl: both fields are XSD-required (`TriggeringEntities`, `EntityCondition`),
/// and `EntityCondition` is itself an `xsd:choice` with no "nothing" state — any default would
/// silently pick a branch. Use `ByEntityCondition::new` or one of the per-condition
/// constructors below instead.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ByEntityCondition {
    /// Entities that can trigger this condition
    #[serde(rename = "TriggeringEntities")]
    pub triggering_entities: TriggeringEntities,

    /// The actual entity condition
    #[serde(rename = "EntityCondition")]
    pub entity_condition: EntityCondition,
}

/// EntityCondition enum for the actual condition types inside ByEntityConditionSchema
/// Matches the XSD EntityCondition choice group exactly
#[derive(Debug, Clone, PartialEq)]
pub enum EntityCondition {
    /// End-of-road detection condition
    EndOfRoad(EndOfRoadCondition),
    /// Collision detection condition
    Collision(CollisionCondition),
    /// Off-road detection condition (matches XSD OffroadCondition)
    Offroad(OffroadCondition),
    /// Time headway condition
    TimeHeadway(TimeHeadwayCondition),
    /// Time to collision condition
    TimeToCollision(TimeToCollisionCondition),
    /// Acceleration-based condition
    Acceleration(AccelerationCondition),
    /// Standstill detection condition
    StandStill(StandStillCondition),
    /// Speed-based condition
    Speed(SpeedCondition),
    /// Relative speed monitoring between entities
    RelativeSpeed(RelativeSpeedCondition),
    /// Distance-based condition triggering
    TraveledDistance(TraveledDistanceCondition),
    /// Position reach condition (deprecated but supported)
    ReachPosition(ReachPositionCondition),
    /// Distance to position condition
    Distance(DistanceCondition),
    /// Relative distance between entities condition
    RelativeDistance(RelativeDistanceCondition),
    /// Clearance monitoring between entities
    RelativeClearance(RelativeClearanceCondition),
    /// Angle condition for entity orientation/direction triggering
    Angle(AngleCondition),
    /// Relative angle conditions between entities
    RelativeAngle(RelativeAngleCondition),
}

impl serde::Serialize for EntityCondition {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;

        let mut map = serializer.serialize_map(None)?;

        match self {
            EntityCondition::EndOfRoad(condition) => {
                map.serialize_entry("EndOfRoadCondition", condition)?;
            }
            EntityCondition::Collision(condition) => {
                map.serialize_entry("CollisionCondition", condition)?;
            }
            EntityCondition::Offroad(condition) => {
                map.serialize_entry("OffroadCondition", condition)?;
            }
            EntityCondition::TimeHeadway(condition) => {
                map.serialize_entry("TimeHeadwayCondition", condition)?;
            }
            EntityCondition::TimeToCollision(condition) => {
                map.serialize_entry("TimeToCollisionCondition", condition)?;
            }
            EntityCondition::Acceleration(condition) => {
                map.serialize_entry("AccelerationCondition", condition)?;
            }
            EntityCondition::StandStill(condition) => {
                map.serialize_entry("StandStillCondition", condition)?;
            }
            EntityCondition::Speed(condition) => {
                map.serialize_entry("SpeedCondition", condition)?;
            }
            EntityCondition::RelativeSpeed(condition) => {
                map.serialize_entry("RelativeSpeedCondition", condition)?;
            }
            EntityCondition::TraveledDistance(condition) => {
                map.serialize_entry("TraveledDistanceCondition", condition)?;
            }
            EntityCondition::ReachPosition(condition) => {
                map.serialize_entry("ReachPositionCondition", condition)?;
            }
            EntityCondition::Distance(condition) => {
                map.serialize_entry("DistanceCondition", condition)?;
            }
            EntityCondition::RelativeDistance(condition) => {
                map.serialize_entry("RelativeDistanceCondition", condition)?;
            }
            EntityCondition::RelativeClearance(condition) => {
                map.serialize_entry("RelativeClearanceCondition", condition)?;
            }
            EntityCondition::Angle(condition) => {
                map.serialize_entry("AngleCondition", condition)?;
            }
            EntityCondition::RelativeAngle(condition) => {
                map.serialize_entry("RelativeAngleCondition", condition)?;
            }
        }

        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for EntityCondition {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;

        struct EntityConditionVisitor;

        impl<'de> Visitor<'de> for EntityConditionVisitor {
            type Value = EntityCondition;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an EntityCondition with exactly one condition type")
            }

            fn visit_map<M>(self, mut map: M) -> std::result::Result<EntityCondition, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut condition_found = false;
                let mut result = None;

                // Process elements in the order they appear
                while let Some(key) = map.next_key::<String>()? {
                    if condition_found {
                        return Err(serde::de::Error::custom(
                            "EntityCondition can only contain one condition type",
                        ));
                    }

                    match key.as_str() {
                        "EndOfRoadCondition" => {
                            let condition: EndOfRoadCondition = map.next_value()?;
                            result = Some(EntityCondition::EndOfRoad(condition));
                            condition_found = true;
                        }
                        "CollisionCondition" => {
                            let condition: CollisionCondition = map.next_value()?;
                            result = Some(EntityCondition::Collision(condition));
                            condition_found = true;
                        }
                        "OffroadCondition" => {
                            let condition: OffroadCondition = map.next_value()?;
                            result = Some(EntityCondition::Offroad(condition));
                            condition_found = true;
                        }
                        "TimeHeadwayCondition" => {
                            let condition: TimeHeadwayCondition = map.next_value()?;
                            result = Some(EntityCondition::TimeHeadway(condition));
                            condition_found = true;
                        }
                        "TimeToCollisionCondition" => {
                            let condition: TimeToCollisionCondition = map.next_value()?;
                            result = Some(EntityCondition::TimeToCollision(condition));
                            condition_found = true;
                        }
                        "AccelerationCondition" => {
                            let condition: AccelerationCondition = map.next_value()?;
                            result = Some(EntityCondition::Acceleration(condition));
                            condition_found = true;
                        }
                        "StandStillCondition" => {
                            let condition: StandStillCondition = map.next_value()?;
                            result = Some(EntityCondition::StandStill(condition));
                            condition_found = true;
                        }
                        "SpeedCondition" => {
                            let condition: SpeedCondition = map.next_value()?;
                            result = Some(EntityCondition::Speed(condition));
                            condition_found = true;
                        }
                        "RelativeSpeedCondition" => {
                            let condition: RelativeSpeedCondition = map.next_value()?;
                            result = Some(EntityCondition::RelativeSpeed(condition));
                            condition_found = true;
                        }
                        "TraveledDistanceCondition" => {
                            let condition: TraveledDistanceCondition = map.next_value()?;
                            result = Some(EntityCondition::TraveledDistance(condition));
                            condition_found = true;
                        }
                        "ReachPositionCondition" => {
                            let condition: ReachPositionCondition = map.next_value()?;
                            result = Some(EntityCondition::ReachPosition(condition));
                            condition_found = true;
                        }
                        "DistanceCondition" => {
                            let condition: DistanceCondition = map.next_value()?;
                            result = Some(EntityCondition::Distance(condition));
                            condition_found = true;
                        }
                        "RelativeDistanceCondition" => {
                            let condition: RelativeDistanceCondition =
                                map.next_value().map_err(|e| {
                                    serde::de::Error::custom(format!(
                                        "Failed to deserialize RelativeDistanceCondition: {}",
                                        e
                                    ))
                                })?;
                            result = Some(EntityCondition::RelativeDistance(condition));
                            condition_found = true;
                        }
                        "RelativeClearanceCondition" => {
                            let condition: RelativeClearanceCondition = map.next_value()?;
                            result = Some(EntityCondition::RelativeClearance(condition));
                            condition_found = true;
                        }
                        "AngleCondition" => {
                            let condition: AngleCondition = map.next_value()?;
                            result = Some(EntityCondition::Angle(condition));
                            condition_found = true;
                        }
                        "RelativeAngleCondition" => {
                            let condition: RelativeAngleCondition = map.next_value()?;
                            result = Some(EntityCondition::RelativeAngle(condition));
                            condition_found = true;
                        }
                        _ => {
                            return Err(serde::de::Error::custom(format!(
                                "Unknown EntityCondition type: {}",
                                key
                            )));
                        }
                    }
                }

                result.ok_or_else(|| {
                    serde::de::Error::custom(
                        "EntityCondition must contain exactly one condition type",
                    )
                })
            }
        }

        deserializer.deserialize_map(EntityConditionVisitor)
    }
}

impl AccelerationCondition {
    /// Create a new acceleration condition
    pub fn new(value: f64, rule: Rule) -> Self {
        Self {
            value: Double::literal(value),
            rule: Value::Literal(rule),
            direction: None,
        }
    }

    /// Set direction of acceleration measurement
    pub fn with_direction(mut self, direction: DirectionalDimension) -> Self {
        self.direction = Some(Value::Literal(direction));
        self
    }

    /// Create condition for acceleration greater than threshold
    pub fn greater_than(acceleration: f64) -> Self {
        Self::new(acceleration, Rule::GreaterThan)
    }

    /// Create condition for acceleration less than threshold
    pub fn less_than(acceleration: f64) -> Self {
        Self::new(acceleration, Rule::LessThan)
    }

    /// Create condition for longitudinal acceleration
    pub fn longitudinal(acceleration: f64, rule: Rule) -> Self {
        Self::new(acceleration, rule).with_direction(DirectionalDimension::Longitudinal)
    }

    /// Create condition for lateral acceleration
    pub fn lateral(acceleration: f64, rule: Rule) -> Self {
        Self::new(acceleration, rule).with_direction(DirectionalDimension::Lateral)
    }

    /// Create condition for vertical acceleration
    pub fn vertical(acceleration: f64, rule: Rule) -> Self {
        Self::new(acceleration, rule).with_direction(DirectionalDimension::Vertical)
    }
}

impl StandStillCondition {
    /// Create a new standstill condition
    pub fn new(duration: f64) -> Self {
        Self {
            duration: Double::literal(duration),
        }
    }

    /// Create condition with specific duration
    pub fn with_duration(duration: f64) -> Self {
        Self::new(duration)
    }
}

impl CollisionTarget {
    /// Create a new collision target. XSD:831-833 `ByObjectType` — `type` is `use="required"`.
    pub fn new(target_type: ObjectType) -> Self {
        Self {
            target_type: Value::Literal(target_type),
        }
    }
}

impl CollisionCondition {
    /// Create a new collision condition with specific target
    pub fn with_target(target: &str) -> Self {
        Self {
            choice: CollisionConditionChoice::EntityRef(EntityRef {
                entity_ref: OSString::literal(target.to_string()),
            }),
        }
    }

    /// Create collision condition for entity type
    pub fn with_type(entity_type: ObjectType) -> Self {
        Self {
            choice: CollisionConditionChoice::ByType(CollisionTarget {
                target_type: Value::Literal(entity_type),
            }),
        }
    }
}

impl OffroadCondition {
    /// Create a new off-road condition
    pub fn new(duration: f64) -> Self {
        Self {
            duration: Double::literal(duration),
        }
    }

    /// Create condition with specific duration
    pub fn with_duration(duration: f64) -> Self {
        Self::new(duration)
    }
}

impl EndOfRoadCondition {
    /// Create a new end-of-road condition
    pub fn new(duration: f64) -> Self {
        Self {
            duration: Double::literal(duration),
        }
    }

    /// Create condition with specific duration
    pub fn with_duration(duration: f64) -> Self {
        Self::new(duration)
    }
}

impl TimeHeadwayCondition {
    /// Create a new time headway condition
    pub fn new(entity_ref: &str, value: f64, rule: Rule, freespace: bool) -> Self {
        Self {
            entity_ref: OSString::literal(entity_ref.to_string()),
            value: Double::literal(value),
            rule: Value::Literal(rule),
            freespace: Boolean::literal(freespace),
            along_route: None,
            coordinate_system: None,
            relative_distance_type: None,
            routing_algorithm: None,
        }
    }

    /// Create condition for headway less than threshold
    pub fn less_than(entity_ref: &str, headway: f64, freespace: bool) -> Self {
        Self::new(entity_ref, headway, Rule::LessThan, freespace)
    }

    /// Create condition for headway greater than threshold
    pub fn greater_than(entity_ref: &str, headway: f64, freespace: bool) -> Self {
        Self::new(entity_ref, headway, Rule::GreaterThan, freespace)
    }

    /// Set coordinate system for measurement
    pub fn with_coordinate_system(mut self, system: CoordinateSystem) -> Self {
        self.coordinate_system = Some(Value::Literal(system));
        self
    }

    /// Set relative distance type for measurement
    pub fn with_distance_type(mut self, distance_type: RelativeDistanceType) -> Self {
        self.relative_distance_type = Some(Value::Literal(distance_type));
        self
    }

    /// Set routing algorithm for route-based measurement
    pub fn with_routing_algorithm(mut self, algorithm: RoutingAlgorithm) -> Self {
        self.routing_algorithm = Some(Value::Literal(algorithm));
        self
    }
}

impl TimeToCollisionCondition {
    /// Create a new time to collision condition with entity target
    pub fn with_entity_target(entity_ref: &str, value: f64, rule: Rule, freespace: bool) -> Self {
        let target = TimeToCollisionTarget {
            choice: TimeToCollisionTargetChoice::EntityRef(EntityRef {
                entity_ref: OSString::literal(entity_ref.to_string()),
            }),
        };

        Self {
            value: Double::literal(value),
            rule: Value::Literal(rule),
            freespace: Boolean::literal(freespace),
            along_route: None,
            coordinate_system: None,
            relative_distance_type: None,
            routing_algorithm: None,
            target,
        }
    }

    /// Create a new time to collision condition with position target
    pub fn with_position_target(
        position: Position,
        value: f64,
        rule: Rule,
        freespace: bool,
    ) -> Self {
        let target = TimeToCollisionTarget {
            choice: TimeToCollisionTargetChoice::Position(position),
        };

        Self {
            value: Double::literal(value),
            rule: Value::Literal(rule),
            freespace: Boolean::literal(freespace),
            along_route: None,
            coordinate_system: None,
            relative_distance_type: None,
            routing_algorithm: None,
            target,
        }
    }

    /// Create condition for collision time less than threshold (entity target)
    pub fn entity_less_than(entity_ref: &str, time: f64, freespace: bool) -> Self {
        Self::with_entity_target(entity_ref, time, Rule::LessThan, freespace)
    }

    /// Create condition for collision time greater than threshold (entity target)
    pub fn entity_greater_than(entity_ref: &str, time: f64, freespace: bool) -> Self {
        Self::with_entity_target(entity_ref, time, Rule::GreaterThan, freespace)
    }

    /// Create condition for collision time less than threshold (position target)
    pub fn position_less_than(position: Position, time: f64, freespace: bool) -> Self {
        Self::with_position_target(position, time, Rule::LessThan, freespace)
    }

    /// Create condition for collision time greater than threshold (position target)
    pub fn position_greater_than(position: Position, time: f64, freespace: bool) -> Self {
        Self::with_position_target(position, time, Rule::GreaterThan, freespace)
    }

    /// Set coordinate system for measurement
    pub fn with_coordinate_system(mut self, system: CoordinateSystem) -> Self {
        self.coordinate_system = Some(Value::Literal(system));
        self
    }

    /// Set relative distance type for measurement
    pub fn with_distance_type(mut self, distance_type: RelativeDistanceType) -> Self {
        self.relative_distance_type = Some(Value::Literal(distance_type));
        self
    }

    /// Set routing algorithm for route-based measurement
    pub fn with_routing_algorithm(mut self, algorithm: RoutingAlgorithm) -> Self {
        self.routing_algorithm = Some(Value::Literal(algorithm));
        self
    }
}

impl AngleCondition {
    /// Create a new angle condition. XSD:734-739 `AngleCondition` — `angleType`, `angle`, and
    /// `angleTolerance` are all `use="required"`; `coordinateSystem` is optional and left unset.
    pub fn new(angle_type: AngleType, angle: f64, angle_tolerance: f64) -> Self {
        Self {
            angle_type: Value::Literal(angle_type),
            angle: Double::literal(angle),
            angle_tolerance: Double::literal(angle_tolerance),
            coordinate_system: None,
        }
    }

    /// Set coordinate system for angle measurement
    pub fn with_coordinate_system(mut self, system: CoordinateSystem) -> Self {
        self.coordinate_system = Some(Value::Literal(system));
        self
    }
}

impl RelativeSpeedCondition {
    /// Create a new relative speed condition. XSD:1883-1888 `RelativeSpeedCondition` —
    /// `entityRef`, `rule`, and `value` are all `use="required"`; `direction` is optional and
    /// left unset.
    pub fn new(entity_ref: &str, rule: Rule, value: f64) -> Self {
        Self {
            entity_ref: OSString::literal(entity_ref.to_string()),
            rule: Value::Literal(rule),
            value: Double::literal(value),
            direction: None,
        }
    }

    /// Set direction of speed measurement
    pub fn with_direction(mut self, direction: DirectionalDimension) -> Self {
        self.direction = Some(Value::Literal(direction));
        self
    }
}

impl RelativeAngleCondition {
    /// Create a new relative angle condition. XSD:1826-1831 `RelativeAngleCondition` —
    /// `entityRef`, `angleType`, `angle`, and `angleTolerance` are all `use="required"`;
    /// `coordinateSystem` is optional and left unset.
    pub fn new(entity_ref: &str, angle_type: AngleType, angle: f64, angle_tolerance: f64) -> Self {
        Self {
            entity_ref: OSString::literal(entity_ref.to_string()),
            angle_type: Value::Literal(angle_type),
            angle: Double::literal(angle),
            angle_tolerance: Double::literal(angle_tolerance),
            coordinate_system: None,
        }
    }

    /// Set coordinate system for angle measurement
    pub fn with_coordinate_system(mut self, system: CoordinateSystem) -> Self {
        self.coordinate_system = Some(Value::Literal(system));
        self
    }
}

impl TraveledDistanceCondition {
    /// Create a new traveled distance condition. XSD:2392-2394 `TraveledDistanceCondition` —
    /// `value` is `use="required"`.
    pub fn new(value: f64) -> Self {
        Self {
            value: Double::literal(value),
        }
    }
}

impl RelativeLaneRange {
    /// Create a new relative lane range. XSD:1862-1865 `RelativeLaneRange` — both `from` and
    /// `to` are optional attributes with no schema-declared default.
    pub fn new(from: Option<i32>, to: Option<i32>) -> Self {
        Self {
            from: from.map(Int::literal),
            to: to.map(Int::literal),
        }
    }
}

impl RelativeClearanceCondition {
    /// Create a new relative clearance condition with no lane ranges or entity refs and no
    /// forward/backward distance limits. XSD:1833-1842 `RelativeClearanceCondition` —
    /// `oppositeLanes` and `freeSpace` are `use="required"`; `distanceForward` and
    /// `distanceBackward` are optional with no schema-declared default.
    pub fn new(opposite_lanes: bool, free_space: bool) -> Self {
        Self {
            relative_lane_ranges: Vec::new(),
            entity_refs: Vec::new(),
            opposite_lanes: Boolean::literal(opposite_lanes),
            distance_forward: None,
            distance_backward: None,
            free_space: Boolean::literal(free_space),
        }
    }
}

impl TimeToCollisionTarget {
    /// Create target with entity reference
    pub fn entity(entity_ref: &str) -> Self {
        Self {
            choice: TimeToCollisionTargetChoice::EntityRef(EntityRef {
                entity_ref: OSString::literal(entity_ref.to_string()),
            }),
        }
    }

    /// Create target with position
    pub fn position(position: Position) -> Self {
        Self {
            choice: TimeToCollisionTargetChoice::Position(position),
        }
    }
}

// Convenience constructors for ByEntityCondition
impl ByEntityCondition {
    /// Create a new ByEntityCondition with the given triggering entities and entity condition
    pub fn new(triggering_entities: TriggeringEntities, entity_condition: EntityCondition) -> Self {
        Self {
            triggering_entities,
            entity_condition,
        }
    }

    /// Create a speed condition
    ///
    /// The entity is identified via `triggering_entities`; `entity_ref` is accepted
    /// for backward-compatible call sites but no longer duplicated onto the condition.
    pub fn speed(
        triggering_entities: TriggeringEntities,
        value: f64,
        rule: Rule,
        _entity_ref: &str,
    ) -> Self {
        Self::new(
            triggering_entities,
            EntityCondition::Speed(SpeedCondition {
                value: Double::literal(value),
                rule: Value::Literal(rule),
                direction: None,
            }),
        )
    }

    /// Create a reach position condition
    pub fn reach_position(
        triggering_entities: TriggeringEntities,
        position: Position,
        tolerance: f64,
    ) -> Self {
        Self::new(
            triggering_entities,
            EntityCondition::ReachPosition(ReachPositionCondition::new(position, tolerance)),
        )
    }

    /// Create a distance condition
    pub fn distance(
        triggering_entities: TriggeringEntities,
        position: Position,
        value: f64,
        freespace: bool,
        rule: Rule,
    ) -> Self {
        Self::new(
            triggering_entities,
            EntityCondition::Distance(DistanceCondition::new(position, value, freespace, rule)),
        )
    }

    /// Create a relative distance condition
    pub fn relative_distance(
        triggering_entities: TriggeringEntities,
        entity_ref: &str,
        value: f64,
        freespace: bool,
        distance_type: RelativeDistanceType,
        rule: Rule,
    ) -> Self {
        Self::new(
            triggering_entities,
            EntityCondition::RelativeDistance(RelativeDistanceCondition::new(
                OSString::literal(entity_ref.to_string()),
                value,
                freespace,
                distance_type,
                rule,
            )),
        )
    }

    /// Create an acceleration condition
    pub fn acceleration(triggering_entities: TriggeringEntities, value: f64, rule: Rule) -> Self {
        Self::new(
            triggering_entities,
            EntityCondition::Acceleration(AccelerationCondition::new(value, rule)),
        )
    }

    /// Create an acceleration condition with direction
    pub fn acceleration_with_direction(
        triggering_entities: TriggeringEntities,
        value: f64,
        rule: Rule,
        direction: DirectionalDimension,
    ) -> Self {
        Self::new(
            triggering_entities,
            EntityCondition::Acceleration(
                AccelerationCondition::new(value, rule).with_direction(direction),
            ),
        )
    }

    /// Create a standstill condition
    pub fn standstill(triggering_entities: TriggeringEntities, duration: f64) -> Self {
        Self::new(
            triggering_entities,
            EntityCondition::StandStill(StandStillCondition::new(duration)),
        )
    }

    /// Create a collision condition with specific target
    pub fn collision_with_target(triggering_entities: TriggeringEntities, target: &str) -> Self {
        Self::new(
            triggering_entities,
            EntityCondition::Collision(CollisionCondition::with_target(target)),
        )
    }

    /// Create a collision condition for entity type
    pub fn collision_with_type(
        triggering_entities: TriggeringEntities,
        entity_type: ObjectType,
    ) -> Self {
        Self::new(
            triggering_entities,
            EntityCondition::Collision(CollisionCondition::with_type(entity_type)),
        )
    }

    /// Create an offroad condition (XSD-compliant OffroadCondition)
    pub fn offroad(triggering_entities: TriggeringEntities, duration: f64) -> Self {
        Self::new(
            triggering_entities,
            EntityCondition::Offroad(OffroadCondition {
                duration: Double::literal(duration),
            }),
        )
    }

    /// Create an end-of-road condition
    pub fn end_of_road(triggering_entities: TriggeringEntities, duration: f64) -> Self {
        Self::new(
            triggering_entities,
            EntityCondition::EndOfRoad(EndOfRoadCondition::new(duration)),
        )
    }

    /// Create a time headway condition
    pub fn time_headway(
        triggering_entities: TriggeringEntities,
        entity_ref: &str,
        value: f64,
        rule: Rule,
        freespace: bool,
    ) -> Self {
        Self::new(
            triggering_entities,
            EntityCondition::TimeHeadway(TimeHeadwayCondition::new(
                entity_ref, value, rule, freespace,
            )),
        )
    }

    /// Create a time to collision condition with entity target
    pub fn time_to_collision_entity(
        triggering_entities: TriggeringEntities,
        entity_ref: &str,
        value: f64,
        rule: Rule,
        freespace: bool,
    ) -> Self {
        Self::new(
            triggering_entities,
            EntityCondition::TimeToCollision(TimeToCollisionCondition::with_entity_target(
                entity_ref, value, rule, freespace,
            )),
        )
    }

    /// Create a time to collision condition with position target
    pub fn time_to_collision_position(
        triggering_entities: TriggeringEntities,
        position: Position,
        value: f64,
        rule: Rule,
        freespace: bool,
    ) -> Self {
        Self::new(
            triggering_entities,
            EntityCondition::TimeToCollision(TimeToCollisionCondition::with_position_target(
                position, value, rule, freespace,
            )),
        )
    }

    /// Create an angle condition
    pub fn angle(
        triggering_entities: TriggeringEntities,
        angle_type: AngleType,
        angle: f64,
        angle_tolerance: f64,
    ) -> Self {
        Self::new(
            triggering_entities,
            EntityCondition::Angle(AngleCondition {
                angle_type: Value::Literal(angle_type),
                angle: Double::literal(angle),
                angle_tolerance: Double::literal(angle_tolerance),
                coordinate_system: None,
            }),
        )
    }

    /// Create a relative speed condition
    pub fn relative_speed(
        triggering_entities: TriggeringEntities,
        entity_ref: &str,
        rule: Rule,
        value: f64,
    ) -> Self {
        Self::new(
            triggering_entities,
            EntityCondition::RelativeSpeed(RelativeSpeedCondition {
                entity_ref: OSString::literal(entity_ref.to_string()),
                rule: Value::Literal(rule),
                value: Double::literal(value),
                direction: None,
            }),
        )
    }

    /// Create a traveled distance condition
    pub fn traveled_distance(triggering_entities: TriggeringEntities, value: f64) -> Self {
        Self::new(
            triggering_entities,
            EntityCondition::TraveledDistance(TraveledDistanceCondition {
                value: Double::literal(value),
            }),
        )
    }

    /// Create a relative clearance condition
    pub fn relative_clearance(
        triggering_entities: TriggeringEntities,
        opposite_lanes: bool,
        free_space: bool,
    ) -> Self {
        Self::new(
            triggering_entities,
            EntityCondition::RelativeClearance(RelativeClearanceCondition {
                relative_lane_ranges: vec![],
                entity_refs: vec![],
                opposite_lanes: Boolean::literal(opposite_lanes),
                distance_forward: None,
                distance_backward: None,
                free_space: Boolean::literal(free_space),
            }),
        )
    }

    /// Create a relative angle condition
    pub fn relative_angle(
        triggering_entities: TriggeringEntities,
        entity_ref: &str,
        angle_type: AngleType,
        angle: f64,
        angle_tolerance: f64,
    ) -> Self {
        Self::new(
            triggering_entities,
            EntityCondition::RelativeAngle(RelativeAngleCondition {
                entity_ref: OSString::literal(entity_ref.to_string()),
                angle_type: Value::Literal(angle_type),
                angle: Double::literal(angle),
                angle_tolerance: Double::literal(angle_tolerance),
                coordinate_system: None,
            }),
        )
    }
}
