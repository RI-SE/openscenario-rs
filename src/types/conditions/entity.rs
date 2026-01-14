//! Entity-based condition types for scenario triggering
//!
//! This file contains:
//! - Spatial conditions (distance, collision, position-based triggers)
//! - Motion conditions (speed, acceleration, standstill detection)
//! - State conditions (end-of-road, off-road, clearance checks)
//! - Temporal conditions (time headway, time-to-collision)
//! - Relative conditions comparing entities to each other
//!
use crate::types::basic::{Boolean, Double, Int, OSString};
use crate::types::enums::{
    AngleType, CoordinateSystem, DirectionalDimension, RelativeDistanceType, RoutingAlgorithm, Rule,
};
use crate::types::positions::Position;
use crate::types::scenario::triggers::{EntityRef, TriggeringEntities};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpeedCondition {
    /// Speed value to compare against
    #[serde(rename = "@value")]
    pub value: Double,

    /// Comparison rule (greater than, less than, etc.)
    #[serde(rename = "@rule")]
    pub rule: Rule,

    /// Entity reference (library extension for convenience)
    //#[serde(rename = "@entityRef")]
    //
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,

    /// Direction of speed measurement (optional)
    #[serde(rename = "@direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<DirectionalDimension>,
}

/// Condition for reaching a specific position within tolerance
/// Note: Marked as deprecated in XSD but still supported for compatibility
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReachPositionCondition {
    /// Target position to reach
    #[serde(rename = "Position")]
    pub position: Position,

    /// Distance tolerance for considering position reached
    #[serde(rename = "@tolerance")]
    pub tolerance: Double,
}

/// Condition based on distance to a specific position
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DistanceCondition {
    /// Reference position for distance measurement
    #[serde(rename = "Position")]
    pub position: Position,

    /// Distance value to compare against
    #[serde(rename = "@value")]
    pub value: Double,

    /// Whether to use freespace (true) or reference point (false) distance
    #[serde(rename = "@freespace")]
    pub freespace: Boolean,

    /// Comparison rule (greater than, less than, etc.)
    #[serde(rename = "@rule")]
    pub rule: Rule,

    /// Whether to measure distance along route (deprecated)
    #[serde(rename = "@alongRoute", skip_serializing_if = "Option::is_none")]
    pub along_route: Option<Boolean>,

    /// Coordinate system for distance measurement
    #[serde(rename = "@coordinateSystem", skip_serializing_if = "Option::is_none")]
    pub coordinate_system: Option<CoordinateSystem>,

    /// Type of relative distance measurement
    #[serde(
        rename = "@relativeDistanceType",
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_distance_type: Option<RelativeDistanceType>,

    /// Algorithm for route-based distance calculation
    #[serde(rename = "@routingAlgorithm", skip_serializing_if = "Option::is_none")]
    pub routing_algorithm: Option<RoutingAlgorithm>,
}

/// Condition based on relative distance between entities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelativeDistanceCondition {
    /// Reference entity for distance measurement
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,

    /// Distance value to compare against
    #[serde(rename = "@value")]
    pub value: Double,

    /// Whether to use freespace (true) or reference point (false) distance
    #[serde(rename = "@freespace")]
    pub freespace: Boolean,

    /// Type of relative distance measurement
    #[serde(rename = "@relativeDistanceType")]
    pub relative_distance_type: RelativeDistanceType,

    /// Comparison rule (greater than, less than, etc.)
    #[serde(rename = "@rule")]
    pub rule: Rule,

    /// Coordinate system for distance measurement
    #[serde(rename = "@coordinateSystem", skip_serializing_if = "Option::is_none")]
    pub coordinate_system: Option<CoordinateSystem>,

    /// Algorithm for route-based distance calculation
    #[serde(rename = "@routingAlgorithm", skip_serializing_if = "Option::is_none")]
    pub routing_algorithm: Option<RoutingAlgorithm>,
}

/// Condition based on entity acceleration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccelerationCondition {
    /// Acceleration value to compare against
    #[serde(rename = "@value")]
    pub value: Double,

    /// Comparison rule (greater than, less than, etc.)
    #[serde(rename = "@rule")]
    pub rule: Rule,

    /// Direction of acceleration measurement (optional)
    #[serde(rename = "@direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<DirectionalDimension>,
}

/// Condition for detecting standstill state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StandStillCondition {
    /// Duration entity must be stationary
    #[serde(rename = "@duration")]
    pub duration: Double,
}

/// Condition for detecting collisions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CollisionCondition {
    /// Specific target entity (optional)
    pub target: Option<OSString>,

    /// Collision detection by entity type
    pub by_type: Option<CollisionTarget>,

    /// Position-based collision detection
    pub position: Option<Position>,
}

/// Target specification for collision detection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CollisionTarget {
    pub target_type: OSString,
}

/// Condition for detecting off-road state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OffRoadCondition {
    /// Duration entity must be off-road
    pub duration: Double,
}

/// Condition for detecting end-of-road state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EndOfRoadCondition {
    /// Duration entity must be at end of road
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
    pub rule: Rule,

    /// Whether to measure in freespace or bounding box
    #[serde(rename = "@freespace")]
    pub freespace: Boolean,

    /// Optional coordinate system for measurement
    #[serde(rename = "@coordinateSystem", skip_serializing_if = "Option::is_none")]
    pub coordinate_system: Option<CoordinateSystem>,

    /// Optional relative distance type
    #[serde(
        rename = "@relativeDistanceType",
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_distance_type: Option<RelativeDistanceType>,

    /// Optional routing algorithm for route-based measurement
    #[serde(rename = "@routingAlgorithm", skip_serializing_if = "Option::is_none")]
    pub routing_algorithm: Option<RoutingAlgorithm>,
}

/// Time to collision condition for collision prediction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimeToCollisionCondition {
    /// Time to collision value
    #[serde(rename = "@value")]
    pub value: Double,

    /// Comparison rule
    #[serde(rename = "@rule")]
    pub rule: Rule,

    /// Whether to measure in freespace or bounding box
    #[serde(rename = "@freespace")]
    pub freespace: Boolean,

    /// Optional coordinate system for measurement
    #[serde(rename = "@coordinateSystem", skip_serializing_if = "Option::is_none")]
    pub coordinate_system: Option<CoordinateSystem>,

    /// Optional relative distance type
    #[serde(
        rename = "@relativeDistanceType",
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_distance_type: Option<RelativeDistanceType>,

    /// Optional routing algorithm for route-based measurement
    #[serde(rename = "@routingAlgorithm", skip_serializing_if = "Option::is_none")]
    pub routing_algorithm: Option<RoutingAlgorithm>,

    /// Target specification for collision detection
    pub target: TimeToCollisionTarget,
}

/// Target for time to collision condition - matches XSD TimeToCollisionConditionTarget
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimeToCollisionTarget {
    /// Target entity reference
    pub entity_ref: Option<EntityRef>,

    /// Target position
    pub position: Option<Position>,
}

/// Angle condition for entity orientation/direction triggering
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AngleCondition {
    /// Type of angle measurement (relative or absolute)
    pub angle_type: AngleType,

    /// Target angle value in radians
    pub angle: Double,

    /// Tolerance for angle matching in radians
    pub angle_tolerance: Double,

    /// Coordinate system for angle measurement
    pub coordinate_system: Option<CoordinateSystem>,
}

/// Off-road detection condition - matches XSD OffroadCondition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OffroadCondition {
    /// Duration entity must be off-road
    pub duration: Double,
}

/// Relative speed monitoring between entities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelativeSpeedCondition {
    /// Reference entity for speed comparison
    pub entity_ref: OSString,

    /// Comparison rule (greater than, less than, etc.)
    pub rule: Rule,

    /// Speed difference value
    pub value: Double,

    /// Direction of speed measurement (optional)
    pub direction: Option<DirectionalDimension>,
}

/// Relative lane range specification for clearance conditions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelativeLaneRange {
    /// Starting lane offset
    pub from: Option<Int>,

    /// Ending lane offset
    pub to: Option<Int>,
}

/// Clearance monitoring between entities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelativeClearanceCondition {
    /// Lane ranges to check for clearance
    pub relative_lane_ranges: Vec<RelativeLaneRange>,

    /// Entity references to check clearance against
    pub entity_refs: Vec<EntityRef>,

    /// Whether to check opposite lanes
    pub opposite_lanes: Boolean,

    /// Distance to check forward (optional)
    pub distance_forward: Option<Double>,

    /// Distance to check backward (optional)
    pub distance_backward: Option<Double>,

    /// Whether to use free space measurement
    pub free_space: Boolean,
}

/// Relative angle conditions between entities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelativeAngleCondition {
    /// Reference entity for angle comparison
    pub entity_ref: OSString,

    /// Type of angle measurement (relative or absolute)
    pub angle_type: AngleType,

    /// Target angle value in radians
    pub angle: Double,

    /// Tolerance for angle matching in radians
    pub angle_tolerance: Double,

    /// Coordinate system for angle measurement
    pub coordinate_system: Option<CoordinateSystem>,
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

impl Default for SpeedCondition {
    fn default() -> Self {
        Self {
            value: Double::literal(10.0),
            rule: Rule::GreaterThan,
            entity_ref: OSString::Literal("DefaultEntity".to_string()),
            direction: None,
        }
    }
}

// Builder implementations for spatial conditions
impl ReachPositionCondition {
    /// Create a new reach position condition
    pub fn new(position: Position, tolerance: f64) -> Self {
        Self {
            position,
            tolerance: Double::literal(tolerance),
        }
    }

    /// Create with world position
    pub fn at_world_position(x: f64, y: f64, z: f64, h: f64, tolerance: f64) -> Self {
        use crate::types::positions::WorldPosition;

        let world_pos = WorldPosition {
            x: Double::literal(x),
            y: Double::literal(y),
            z: Some(Double::literal(z)),
            h: Some(Double::literal(h)),
            p: Some(Double::literal(0.0)),
            r: Some(Double::literal(0.0)),
        };
        let mut position = Position::default();
        position.world_position = Some(world_pos);
        position.relative_world_position = None;
        position.road_position = None;
        position.lane_position = None;

        Self::new(position, tolerance)
    }
}

impl DistanceCondition {
    /// Create a new distance condition
    pub fn new(position: Position, value: f64, freespace: bool, rule: Rule) -> Self {
        Self {
            position,
            value: Double::literal(value),
            freespace: Boolean::literal(freespace),
            rule,
            along_route: None,
            coordinate_system: None,
            relative_distance_type: None,
            routing_algorithm: None,
        }
    }

    /// Set coordinate system for distance measurement
    pub fn with_coordinate_system(mut self, system: CoordinateSystem) -> Self {
        self.coordinate_system = Some(system);
        self
    }

    /// Set distance measurement type
    pub fn with_distance_type(mut self, distance_type: RelativeDistanceType) -> Self {
        self.relative_distance_type = Some(distance_type);
        self
    }

    /// Set routing algorithm for route-based distance
    pub fn with_routing_algorithm(mut self, algorithm: RoutingAlgorithm) -> Self {
        self.routing_algorithm = Some(algorithm);
        self
    }

    /// Create condition for distance less than threshold
    pub fn less_than(position: Position, distance: f64, freespace: bool) -> Self {
        Self::new(position, distance, freespace, Rule::LessThan)
    }

    /// Create condition for distance greater than threshold
    pub fn greater_than(position: Position, distance: f64, freespace: bool) -> Self {
        Self::new(position, distance, freespace, Rule::GreaterThan)
    }
}

impl RelativeDistanceCondition {
    /// Create a new relative distance condition
    pub fn new(
        entity_ref: &str,
        value: f64,
        freespace: bool,
        distance_type: RelativeDistanceType,
        rule: Rule,
    ) -> Self {
        Self {
            entity_ref: OSString::literal(entity_ref.to_string()),
            value: Double::literal(value),
            freespace: Boolean::literal(freespace),
            relative_distance_type: distance_type,
            rule,
            coordinate_system: None,
            routing_algorithm: None,
        }
    }

    /// Set coordinate system for distance measurement
    pub fn with_coordinate_system(mut self, system: CoordinateSystem) -> Self {
        self.coordinate_system = Some(system);
        self
    }

    /// Set routing algorithm for route-based distance
    pub fn with_routing_algorithm(mut self, algorithm: RoutingAlgorithm) -> Self {
        self.routing_algorithm = Some(algorithm);
        self
    }

    /// Create longitudinal distance condition
    pub fn longitudinal(entity_ref: &str, distance: f64, freespace: bool, rule: Rule) -> Self {
        Self::new(
            entity_ref,
            distance,
            freespace,
            RelativeDistanceType::Longitudinal,
            rule,
        )
    }

    /// Create lateral distance condition
    pub fn lateral(entity_ref: &str, distance: f64, freespace: bool, rule: Rule) -> Self {
        Self::new(
            entity_ref,
            distance,
            freespace,
            RelativeDistanceType::Lateral,
            rule,
        )
    }

    /// Create cartesian distance condition
    pub fn cartesian(entity_ref: &str, distance: f64, freespace: bool, rule: Rule) -> Self {
        Self::new(
            entity_ref,
            distance,
            freespace,
            RelativeDistanceType::Cartesian,
            rule,
        )
    }
}

impl AccelerationCondition {
    /// Create a new acceleration condition
    pub fn new(value: f64, rule: Rule) -> Self {
        Self {
            value: Double::literal(value),
            rule,
            direction: None,
        }
    }

    /// Set direction of acceleration measurement
    pub fn with_direction(mut self, direction: DirectionalDimension) -> Self {
        self.direction = Some(direction);
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

impl CollisionCondition {
    /// Create a new collision condition with specific target
    pub fn with_target(target: &str) -> Self {
        Self {
            target: Some(OSString::literal(target.to_string())),
            by_type: None,
            position: None,
        }
    }

    /// Create collision condition for entity type
    pub fn with_type(entity_type: &str) -> Self {
        Self {
            target: None,
            by_type: Some(CollisionTarget {
                target_type: OSString::literal(entity_type.to_string()),
            }),
            position: None,
        }
    }

    /// Create collision condition at specific position
    pub fn at_position(position: Position) -> Self {
        Self {
            target: None,
            by_type: None,
            position: Some(position),
        }
    }

    /// Create general collision condition (any collision)
    pub fn any_collision() -> Self {
        Self {
            target: None,
            by_type: None,
            position: None,
        }
    }
}

impl OffRoadCondition {
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
            rule,
            freespace: Boolean::literal(freespace),
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
        self.coordinate_system = Some(system);
        self
    }

    /// Set relative distance type for measurement
    pub fn with_distance_type(mut self, distance_type: RelativeDistanceType) -> Self {
        self.relative_distance_type = Some(distance_type);
        self
    }

    /// Set routing algorithm for route-based measurement
    pub fn with_routing_algorithm(mut self, algorithm: RoutingAlgorithm) -> Self {
        self.routing_algorithm = Some(algorithm);
        self
    }
}

impl TimeToCollisionCondition {
    /// Create a new time to collision condition with entity target
    pub fn with_entity_target(entity_ref: &str, value: f64, rule: Rule, freespace: bool) -> Self {
        let target = TimeToCollisionTarget {
            entity_ref: Some(EntityRef {
                entity_ref: OSString::literal(entity_ref.to_string()),
            }),
            position: None,
        };

        Self {
            value: Double::literal(value),
            rule,
            freespace: Boolean::literal(freespace),
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
            entity_ref: None,
            position: Some(position),
        };

        Self {
            value: Double::literal(value),
            rule,
            freespace: Boolean::literal(freespace),
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
        self.coordinate_system = Some(system);
        self
    }

    /// Set relative distance type for measurement
    pub fn with_distance_type(mut self, distance_type: RelativeDistanceType) -> Self {
        self.relative_distance_type = Some(distance_type);
        self
    }

    /// Set routing algorithm for route-based measurement
    pub fn with_routing_algorithm(mut self, algorithm: RoutingAlgorithm) -> Self {
        self.routing_algorithm = Some(algorithm);
        self
    }
}

impl TimeToCollisionTarget {
    /// Create target with entity reference
    pub fn entity(entity_ref: &str) -> Self {
        Self {
            entity_ref: Some(EntityRef {
                entity_ref: OSString::literal(entity_ref.to_string()),
            }),
            position: None,
        }
    }

    /// Create target with position
    pub fn position(position: Position) -> Self {
        Self {
            entity_ref: None,
            position: Some(position),
        }
    }
}

// Default implementations for testing and fallback scenarios
impl Default for ReachPositionCondition {
    fn default() -> Self {
        Self {
            position: Position::default(),
            tolerance: Double::literal(1.0),
        }
    }
}

impl Default for DistanceCondition {
    fn default() -> Self {
        Self {
            position: Position::default(),
            value: Double::literal(10.0),
            freespace: Boolean::literal(true),
            rule: Rule::LessThan,
            along_route: None,
            coordinate_system: None,
            relative_distance_type: None,
            routing_algorithm: None,
        }
    }
}

impl Default for RelativeDistanceCondition {
    fn default() -> Self {
        Self {
            entity_ref: OSString::literal("DefaultEntity".to_string()),
            value: Double::literal(10.0),
            freespace: Boolean::literal(true),
            relative_distance_type: RelativeDistanceType::Cartesian,
            rule: Rule::LessThan,
            coordinate_system: None,
            routing_algorithm: None,
        }
    }
}

impl Default for AccelerationCondition {
    fn default() -> Self {
        Self {
            value: Double::literal(2.0),
            rule: Rule::GreaterThan,
            direction: None,
        }
    }
}

impl Default for StandStillCondition {
    fn default() -> Self {
        Self {
            duration: Double::literal(1.0),
        }
    }
}

impl Default for CollisionCondition {
    fn default() -> Self {
        Self {
            target: None,
            by_type: None,
            position: None,
        }
    }
}

impl Default for CollisionTarget {
    fn default() -> Self {
        Self {
            target_type: OSString::literal("vehicle".to_string()),
        }
    }
}

impl Default for OffRoadCondition {
    fn default() -> Self {
        Self {
            duration: Double::literal(1.0),
        }
    }
}

impl Default for EndOfRoadCondition {
    fn default() -> Self {
        Self {
            duration: Double::literal(1.0),
        }
    }
}

impl Default for TimeHeadwayCondition {
    fn default() -> Self {
        Self {
            entity_ref: OSString::literal("DefaultEntity".to_string()),
            value: Double::literal(2.0),
            rule: Rule::LessThan,
            freespace: Boolean::literal(true),
            coordinate_system: None,
            relative_distance_type: None,
            routing_algorithm: None,
        }
    }
}

impl Default for TimeToCollisionCondition {
    fn default() -> Self {
        Self {
            value: Double::literal(5.0),
            rule: Rule::LessThan,
            freespace: Boolean::literal(true),
            coordinate_system: None,
            relative_distance_type: None,
            routing_algorithm: None,
            target: TimeToCollisionTarget::default(),
        }
    }
}

impl Default for TimeToCollisionTarget {
    fn default() -> Self {
        Self {
            entity_ref: Some(EntityRef {
                entity_ref: OSString::literal("DefaultEntity".to_string()),
            }),
            position: None,
        }
    }
}

impl Default for AngleCondition {
    fn default() -> Self {
        Self {
            angle_type: AngleType::Relative,
            angle: Double::literal(0.0),
            angle_tolerance: Double::literal(0.1),
            coordinate_system: None,
        }
    }
}

impl Default for OffroadCondition {
    fn default() -> Self {
        Self {
            duration: Double::literal(1.0),
        }
    }
}

impl Default for RelativeSpeedCondition {
    fn default() -> Self {
        Self {
            entity_ref: OSString::literal("DefaultEntity".to_string()),
            rule: Rule::GreaterThan,
            value: Double::literal(5.0),
            direction: None,
        }
    }
}

impl Default for RelativeLaneRange {
    fn default() -> Self {
        Self {
            from: Some(Int::literal(-1)),
            to: Some(Int::literal(1)),
        }
    }
}

impl Default for RelativeClearanceCondition {
    fn default() -> Self {
        Self {
            relative_lane_ranges: vec![RelativeLaneRange::default()],
            entity_refs: vec![EntityRef::default()],
            opposite_lanes: Boolean::literal(false),
            distance_forward: Some(Double::literal(50.0)),
            distance_backward: Some(Double::literal(10.0)),
            free_space: Boolean::literal(true),
        }
    }
}

impl Default for RelativeAngleCondition {
    fn default() -> Self {
        Self {
            entity_ref: OSString::literal("DefaultEntity".to_string()),
            angle_type: AngleType::Relative,
            angle: Double::literal(0.0),
            angle_tolerance: Double::literal(0.1),
            coordinate_system: None,
        }
    }
}

impl Default for TraveledDistanceCondition {
    fn default() -> Self {
        Self {
            value: Double::literal(100.0),
        }
    }
}

impl Default for EntityCondition {
    fn default() -> Self {
        EntityCondition::Speed(SpeedCondition::default())
    }
}

impl Default for ByEntityCondition {
    fn default() -> Self {
        Self {
            triggering_entities: TriggeringEntities::default(),
            entity_condition: EntityCondition::default(),
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
    pub fn speed(
        triggering_entities: TriggeringEntities,
        value: f64,
        rule: Rule,
        entity_ref: &str,
    ) -> Self {
        Self::new(
            triggering_entities,
            EntityCondition::Speed(SpeedCondition {
                value: Double::literal(value),
                rule,
                entity_ref: OSString::Literal(entity_ref.to_string()),
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
                entity_ref,
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
    pub fn collision_with_type(triggering_entities: TriggeringEntities, entity_type: &str) -> Self {
        Self::new(
            triggering_entities,
            EntityCondition::Collision(CollisionCondition::with_type(entity_type)),
        )
    }

    /// Create a collision condition at position
    pub fn collision_at_position(
        triggering_entities: TriggeringEntities,
        position: Position,
    ) -> Self {
        Self::new(
            triggering_entities,
            EntityCondition::Collision(CollisionCondition::at_position(position)),
        )
    }

    /// Create a general collision condition
    pub fn collision(triggering_entities: TriggeringEntities) -> Self {
        Self::new(
            triggering_entities,
            EntityCondition::Collision(CollisionCondition::any_collision()),
        )
    }

    /// Create an off-road condition (legacy OffRoadCondition)
    pub fn off_road(triggering_entities: TriggeringEntities, duration: f64) -> Self {
        Self::new(
            triggering_entities,
            EntityCondition::EndOfRoad(EndOfRoadCondition::with_duration(duration)),
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
                angle_type,
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
                rule,
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
                angle_type,
                angle: Double::literal(angle),
                angle_tolerance: Double::literal(angle_tolerance),
                coordinate_system: None,
            }),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::enums::DirectionalDimension;

    #[test]
    fn test_acceleration_condition_new() {
        let condition = AccelerationCondition::new(5.0, Rule::GreaterThan);
        assert_eq!(condition.value, Double::literal(5.0));
        assert_eq!(condition.rule, Rule::GreaterThan);
        assert_eq!(condition.direction, None);
    }

    #[test]
    fn test_acceleration_condition_with_direction() {
        let condition = AccelerationCondition::new(3.0, Rule::LessThan)
            .with_direction(DirectionalDimension::Longitudinal);
        assert_eq!(condition.value, Double::literal(3.0));
        assert_eq!(condition.rule, Rule::LessThan);
        assert_eq!(
            condition.direction,
            Some(DirectionalDimension::Longitudinal)
        );
    }

    #[test]
    fn test_acceleration_condition_greater_than() {
        let condition = AccelerationCondition::greater_than(2.5);
        assert_eq!(condition.value, Double::literal(2.5));
        assert_eq!(condition.rule, Rule::GreaterThan);
        assert_eq!(condition.direction, None);
    }

    #[test]
    fn test_acceleration_condition_less_than() {
        let condition = AccelerationCondition::less_than(1.0);
        assert_eq!(condition.value, Double::literal(1.0));
        assert_eq!(condition.rule, Rule::LessThan);
        assert_eq!(condition.direction, None);
    }

    #[test]
    fn test_acceleration_condition_longitudinal() {
        let condition = AccelerationCondition::longitudinal(4.0, Rule::EqualTo);
        assert_eq!(condition.value, Double::literal(4.0));
        assert_eq!(condition.rule, Rule::EqualTo);
        assert_eq!(
            condition.direction,
            Some(DirectionalDimension::Longitudinal)
        );
    }

    #[test]
    fn test_acceleration_condition_lateral() {
        let condition = AccelerationCondition::lateral(2.0, Rule::GreaterOrEqual);
        assert_eq!(condition.value, Double::literal(2.0));
        assert_eq!(condition.rule, Rule::GreaterOrEqual);
        assert_eq!(condition.direction, Some(DirectionalDimension::Lateral));
    }

    #[test]
    fn test_acceleration_condition_vertical() {
        let condition = AccelerationCondition::vertical(1.5, Rule::LessOrEqual);
        assert_eq!(condition.value, Double::literal(1.5));
        assert_eq!(condition.rule, Rule::LessOrEqual);
        assert_eq!(condition.direction, Some(DirectionalDimension::Vertical));
    }

    #[test]
    fn test_acceleration_condition_default() {
        let condition = AccelerationCondition::default();
        assert_eq!(condition.value, Double::literal(2.0));
        assert_eq!(condition.rule, Rule::GreaterThan);
        assert_eq!(condition.direction, None);
    }

    #[test]
    fn test_standstill_condition_new() {
        let condition = StandStillCondition::new(3.0);
        assert_eq!(condition.duration, Double::literal(3.0));
    }

    #[test]
    fn test_standstill_condition_with_duration() {
        let condition = StandStillCondition::with_duration(5.5);
        assert_eq!(condition.duration, Double::literal(5.5));
    }

    #[test]
    fn test_standstill_condition_default() {
        let condition = StandStillCondition::default();
        assert_eq!(condition.duration, Double::literal(1.0));
    }

    #[test]
    fn test_by_entity_condition_acceleration() {
        let triggering_entities = TriggeringEntities::default();
        let condition =
            ByEntityCondition::acceleration(triggering_entities, 3.0, Rule::GreaterThan);
        match condition.entity_condition {
            EntityCondition::Acceleration(acc_condition) => {
                assert_eq!(acc_condition.value, Double::literal(3.0));
                assert_eq!(acc_condition.rule, Rule::GreaterThan);
                assert_eq!(acc_condition.direction, None);
            }
            _ => panic!("Expected Acceleration variant"),
        }
    }

    #[test]
    fn test_by_entity_condition_acceleration_with_direction() {
        let triggering_entities = TriggeringEntities::default();
        let condition = ByEntityCondition::acceleration_with_direction(
            triggering_entities,
            2.5,
            Rule::LessThan,
            DirectionalDimension::Lateral,
        );
        match condition.entity_condition {
            EntityCondition::Acceleration(acc_condition) => {
                assert_eq!(acc_condition.value, Double::literal(2.5));
                assert_eq!(acc_condition.rule, Rule::LessThan);
                assert_eq!(acc_condition.direction, Some(DirectionalDimension::Lateral));
            }
            _ => panic!("Expected Acceleration variant"),
        }
    }

    #[test]
    fn test_by_entity_condition_standstill() {
        let triggering_entities = TriggeringEntities::default();
        let condition = ByEntityCondition::standstill(triggering_entities, 4.0);
        match condition.entity_condition {
            EntityCondition::StandStill(standstill_condition) => {
                assert_eq!(standstill_condition.duration, Double::literal(4.0));
            }
            _ => panic!("Expected StandStill variant"),
        }
    }

    #[test]
    fn test_acceleration_condition_serialization() {
        let condition = AccelerationCondition::new(2.5, Rule::GreaterThan)
            .with_direction(DirectionalDimension::Longitudinal);

        // Test that it can be serialized and deserialized
        let serialized = serde_json::to_string(&condition).unwrap();
        let deserialized: AccelerationCondition = serde_json::from_str(&serialized).unwrap();

        assert_eq!(condition, deserialized);
    }

    #[test]
    fn test_standstill_condition_serialization() {
        let condition = StandStillCondition::new(3.5);

        // Test that it can be serialized and deserialized
        let serialized = serde_json::to_string(&condition).unwrap();
        let deserialized: StandStillCondition = serde_json::from_str(&serialized).unwrap();

        assert_eq!(condition, deserialized);
    }

    #[test]
    fn test_by_entity_condition_enum_variants() {
        // Test that all variants can be created and matched
        let triggering_entities = TriggeringEntities::default();
        let acceleration =
            ByEntityCondition::acceleration(triggering_entities.clone(), 1.0, Rule::GreaterThan);
        let standstill = ByEntityCondition::standstill(triggering_entities, 2.0);

        match acceleration.entity_condition {
            EntityCondition::Acceleration(_) => (),
            _ => panic!("Expected Acceleration variant"),
        }

        match standstill.entity_condition {
            EntityCondition::StandStill(_) => (),
            _ => panic!("Expected StandStill variant"),
        }
    }

    #[test]
    fn test_collision_condition_new_with_target() {
        let condition = CollisionCondition::with_target("vehicle1");
        assert_eq!(
            condition.target,
            Some(OSString::literal("vehicle1".to_string()))
        );
        assert_eq!(condition.by_type, None);
        assert_eq!(condition.position, None);
    }

    #[test]
    fn test_collision_condition_with_type() {
        let condition = CollisionCondition::with_type("pedestrian");
        assert_eq!(condition.target, None);
        assert!(condition.by_type.is_some());
        if let Some(by_type) = condition.by_type {
            assert_eq!(
                by_type.target_type,
                OSString::literal("pedestrian".to_string())
            );
        }
        assert_eq!(condition.position, None);
    }

    #[test]
    fn test_collision_condition_any_collision() {
        let condition = CollisionCondition::any_collision();
        assert_eq!(condition.target, None);
        assert_eq!(condition.by_type, None);
        assert_eq!(condition.position, None);
    }

    #[test]
    fn test_collision_condition_default() {
        let condition = CollisionCondition::default();
        assert_eq!(condition.target, None);
        assert_eq!(condition.by_type, None);
        assert_eq!(condition.position, None);
    }

    #[test]
    fn test_off_road_condition_new() {
        let condition = OffRoadCondition::new(2.5);
        assert_eq!(condition.duration, Double::literal(2.5));
    }

    #[test]
    fn test_off_road_condition_with_duration() {
        let condition = OffRoadCondition::with_duration(4.0);
        assert_eq!(condition.duration, Double::literal(4.0));
    }

    #[test]
    fn test_off_road_condition_default() {
        let condition = OffRoadCondition::default();
        assert_eq!(condition.duration, Double::literal(1.0));
    }

    #[test]
    fn test_end_of_road_condition_new() {
        let condition = EndOfRoadCondition::new(1.5);
        assert_eq!(condition.duration, Double::literal(1.5));
    }

    #[test]
    fn test_end_of_road_condition_with_duration() {
        let condition = EndOfRoadCondition::with_duration(3.0);
        assert_eq!(condition.duration, Double::literal(3.0));
    }

    #[test]
    fn test_end_of_road_condition_default() {
        let condition = EndOfRoadCondition::default();
        assert_eq!(condition.duration, Double::literal(1.0));
    }

    #[test]
    fn test_by_entity_condition_collision_variants() {
        let triggering_entities = TriggeringEntities::default();
        let collision_target =
            ByEntityCondition::collision_with_target(triggering_entities.clone(), "vehicle1");
        let collision_type =
            ByEntityCondition::collision_with_type(triggering_entities.clone(), "pedestrian");
        let collision_any = ByEntityCondition::collision(triggering_entities);

        match collision_target.entity_condition {
            EntityCondition::Collision(condition) => {
                assert_eq!(
                    condition.target,
                    Some(OSString::literal("vehicle1".to_string()))
                );
            }
            _ => panic!("Expected Collision variant"),
        }

        match collision_type.entity_condition {
            EntityCondition::Collision(condition) => {
                assert!(condition.by_type.is_some());
            }
            _ => panic!("Expected Collision variant"),
        }

        match collision_any.entity_condition {
            EntityCondition::Collision(_) => (),
            _ => panic!("Expected Collision variant"),
        }
    }

    #[test]
    fn test_by_entity_condition_safety_variants() {
        let triggering_entities = TriggeringEntities::default();
        let off_road = ByEntityCondition::off_road(triggering_entities.clone(), 2.0);
        let end_of_road = ByEntityCondition::end_of_road(triggering_entities, 3.0);

        match off_road.entity_condition {
            EntityCondition::EndOfRoad(condition) => {
                assert_eq!(condition.duration, Double::literal(2.0));
            }
            _ => panic!("Expected EndOfRoad variant"),
        }

        match end_of_road.entity_condition {
            EntityCondition::EndOfRoad(condition) => {
                assert_eq!(condition.duration, Double::literal(3.0));
            }
            _ => panic!("Expected EndOfRoad variant"),
        }
    }

    #[test]
    fn test_safety_conditions_serialization() {
        let collision = CollisionCondition::with_target("vehicle1");
        let off_road = OffRoadCondition::new(2.0);
        let end_of_road = EndOfRoadCondition::new(3.0);

        // Test that they can be serialized and deserialized
        let collision_serialized = serde_json::to_string(&collision).unwrap();
        let collision_deserialized: CollisionCondition =
            serde_json::from_str(&collision_serialized).unwrap();
        assert_eq!(collision, collision_deserialized);

        let off_road_serialized = serde_json::to_string(&off_road).unwrap();
        let off_road_deserialized: OffRoadCondition =
            serde_json::from_str(&off_road_serialized).unwrap();
        assert_eq!(off_road, off_road_deserialized);

        let end_of_road_serialized = serde_json::to_string(&end_of_road).unwrap();
        let end_of_road_deserialized: EndOfRoadCondition =
            serde_json::from_str(&end_of_road_serialized).unwrap();
        assert_eq!(end_of_road, end_of_road_deserialized);
    }
}
