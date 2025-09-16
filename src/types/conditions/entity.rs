//! Entity-based condition types for scenario triggering
//!
//! This file contains:
//! - Spatial conditions (distance, collision, position-based triggers)
//! - Motion conditions (speed, acceleration, standstill detection)
//! - State conditions (end-of-road, off-road, clearance checks)
//! - Temporal conditions (time headway, time-to-collision)
//! - Relative conditions comparing entities to each other
//!
//! Contributes to project by:
//! - Enabling realistic scenario triggering based on entity behavior
//! - Providing comprehensive spatial relationship monitoring
//! - Supporting safety-critical condition detection (collision, clearance)
//! - Facilitating dynamic scenario adaptation based on entity states
//! - Enabling complex multi-entity coordination and interaction patterns

use crate::types::basic::{Boolean, Double, OSString};
use crate::types::enums::{CoordinateSystem, DirectionalDimension, RelativeDistanceType, RoutingAlgorithm, Rule};
use crate::types::positions::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpeedCondition {
    #[serde(rename = "@value")]
    pub value: Double,
    #[serde(rename = "@rule")]
    pub rule: Rule,
    #[serde(rename = "@entityRef")]
    pub entity_ref: String,
}

/// Condition for reaching a specific position within tolerance
/// Note: Marked as deprecated in XSD but still supported for compatibility
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename = "ReachPositionCondition")]
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
#[serde(rename = "DistanceCondition")]
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
    #[serde(rename = "@relativeDistanceType", skip_serializing_if = "Option::is_none")]
    pub relative_distance_type: Option<RelativeDistanceType>,
    
    /// Algorithm for route-based distance calculation
    #[serde(rename = "@routingAlgorithm", skip_serializing_if = "Option::is_none")]
    pub routing_algorithm: Option<RoutingAlgorithm>,
}

/// Condition based on relative distance between entities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename = "RelativeDistanceCondition")]
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
#[serde(rename = "AccelerationCondition")]
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
#[serde(rename = "StandStillCondition")]
pub struct StandStillCondition {
    /// Duration entity must be stationary
    #[serde(rename = "@duration")]
    pub duration: Double,
}

/// Entity-based condition types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ByEntityCondition {
    /// Speed-based condition
    #[serde(rename = "SpeedCondition")]
    Speed(SpeedCondition),
    /// Position reach condition (deprecated but supported)
    #[serde(rename = "ReachPositionCondition")]
    ReachPosition(ReachPositionCondition),
    /// Distance to position condition
    #[serde(rename = "DistanceCondition")]
    Distance(DistanceCondition),
    /// Relative distance between entities condition
    #[serde(rename = "RelativeDistanceCondition")]
    RelativeDistance(RelativeDistanceCondition),
    /// Acceleration-based condition
    #[serde(rename = "AccelerationCondition")]
    Acceleration(AccelerationCondition),
    /// Standstill detection condition
    #[serde(rename = "StandStillCondition")]
    StandStill(StandStillCondition),
    // More entity conditions will be added later
}

impl Default for SpeedCondition {
    fn default() -> Self {
        Self {
            value: Double::literal(10.0),
            rule: Rule::GreaterThan,
            entity_ref: "DefaultEntity".to_string(),
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
    pub fn new(
        position: Position,
        value: f64,
        freespace: bool,
        rule: Rule,
    ) -> Self {
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
    pub fn longitudinal(
        entity_ref: &str,
        distance: f64,
        freespace: bool,
        rule: Rule,
    ) -> Self {
        Self::new(entity_ref, distance, freespace, RelativeDistanceType::Longitudinal, rule)
    }
    
    /// Create lateral distance condition
    pub fn lateral(
        entity_ref: &str,
        distance: f64,
        freespace: bool,
        rule: Rule,
    ) -> Self {
        Self::new(entity_ref, distance, freespace, RelativeDistanceType::Lateral, rule)
    }
    
    /// Create cartesian distance condition
    pub fn cartesian(
        entity_ref: &str,
        distance: f64,
        freespace: bool,
        rule: Rule,
    ) -> Self {
        Self::new(entity_ref, distance, freespace, RelativeDistanceType::Cartesian, rule)
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

impl Default for ByEntityCondition {
    fn default() -> Self {
        ByEntityCondition::Speed(SpeedCondition::default())
    }
}

// Convenience constructors for ByEntityCondition
impl ByEntityCondition {
    /// Create a speed condition
    pub fn speed(value: f64, rule: Rule, entity_ref: impl Into<String>) -> Self {
        ByEntityCondition::Speed(SpeedCondition {
            value: Double::literal(value),
            rule,
            entity_ref: entity_ref.into(),
        })
    }
    
    /// Create a reach position condition
    pub fn reach_position(position: Position, tolerance: f64) -> Self {
        ByEntityCondition::ReachPosition(ReachPositionCondition::new(position, tolerance))
    }
    
    /// Create a distance condition
    pub fn distance(
        position: Position,
        value: f64,
        freespace: bool,
        rule: Rule,
    ) -> Self {
        ByEntityCondition::Distance(DistanceCondition::new(position, value, freespace, rule))
    }
    
    /// Create a relative distance condition
    pub fn relative_distance(
        entity_ref: &str,
        value: f64,
        freespace: bool,
        distance_type: RelativeDistanceType,
        rule: Rule,
    ) -> Self {
        ByEntityCondition::RelativeDistance(RelativeDistanceCondition::new(
            entity_ref,
            value,
            freespace,
            distance_type,
            rule,
        ))
    }
    
    /// Create an acceleration condition
    pub fn acceleration(value: f64, rule: Rule) -> Self {
        ByEntityCondition::Acceleration(AccelerationCondition::new(value, rule))
    }
    
    /// Create an acceleration condition with direction
    pub fn acceleration_with_direction(
        value: f64,
        rule: Rule,
        direction: DirectionalDimension,
    ) -> Self {
        ByEntityCondition::Acceleration(
            AccelerationCondition::new(value, rule).with_direction(direction)
        )
    }
    
    /// Create a standstill condition
    pub fn standstill(duration: f64) -> Self {
        ByEntityCondition::StandStill(StandStillCondition::new(duration))
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
        assert_eq!(condition.direction, Some(DirectionalDimension::Longitudinal));
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
        assert_eq!(condition.direction, Some(DirectionalDimension::Longitudinal));
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
        let condition = ByEntityCondition::acceleration(3.0, Rule::GreaterThan);
        match condition {
            ByEntityCondition::Acceleration(acc_condition) => {
                assert_eq!(acc_condition.value, Double::literal(3.0));
                assert_eq!(acc_condition.rule, Rule::GreaterThan);
                assert_eq!(acc_condition.direction, None);
            }
            _ => panic!("Expected Acceleration variant"),
        }
    }

    #[test]
    fn test_by_entity_condition_acceleration_with_direction() {
        let condition = ByEntityCondition::acceleration_with_direction(
            2.5,
            Rule::LessThan,
            DirectionalDimension::Lateral,
        );
        match condition {
            ByEntityCondition::Acceleration(acc_condition) => {
                assert_eq!(acc_condition.value, Double::literal(2.5));
                assert_eq!(acc_condition.rule, Rule::LessThan);
                assert_eq!(acc_condition.direction, Some(DirectionalDimension::Lateral));
            }
            _ => panic!("Expected Acceleration variant"),
        }
    }

    #[test]
    fn test_by_entity_condition_standstill() {
        let condition = ByEntityCondition::standstill(4.0);
        match condition {
            ByEntityCondition::StandStill(standstill_condition) => {
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
        let acceleration = ByEntityCondition::acceleration(1.0, Rule::GreaterThan);
        let standstill = ByEntityCondition::standstill(2.0);
        
        match acceleration {
            ByEntityCondition::Acceleration(_) => (),
            _ => panic!("Expected Acceleration variant"),
        }
        
        match standstill {
            ByEntityCondition::StandStill(_) => (),
            _ => panic!("Expected StandStill variant"),
        }
    }
}
