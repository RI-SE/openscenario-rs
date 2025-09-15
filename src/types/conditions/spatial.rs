//! Spatial condition types for position and distance-based triggering
//!
//! This file contains:
//! - Position-based conditions (reach position with tolerance)
//! - Distance conditions (absolute distance to position)
//! - Relative distance conditions (distance between entities)
//! - Coordinate system and routing algorithm support
//! - Distance measurement type configurations
//!
//! Contributes to project by:
//! - Enabling realistic spatial triggering in autonomous driving scenarios
//! - Supporting safety-critical distance monitoring and collision avoidance
//! - Providing flexible coordinate system and measurement options
//! - Facilitating complex multi-entity spatial relationship detection
//! - Supporting both freespace and reference point distance calculations

use crate::types::basic::{Boolean, Double, OSString};
use crate::types::enums::{CoordinateSystem, RelativeDistanceType, RoutingAlgorithm, Rule};
use crate::types::positions::Position;
use serde::{Deserialize, Serialize};

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

// Builder implementations for ergonomic construction
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
        entity_ref: impl Into<OSString>,
        value: f64,
        freespace: bool,
        distance_type: RelativeDistanceType,
        rule: Rule,
    ) -> Self {
        Self {
            entity_ref: entity_ref.into(),
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
        entity_ref: impl Into<OSString>,
        distance: f64,
        freespace: bool,
        rule: Rule,
    ) -> Self {
        Self::new(entity_ref, distance, freespace, RelativeDistanceType::Longitudinal, rule)
    }
    
    /// Create lateral distance condition
    pub fn lateral(
        entity_ref: impl Into<OSString>,
        distance: f64,
        freespace: bool,
        rule: Rule,
    ) -> Self {
        Self::new(entity_ref, distance, freespace, RelativeDistanceType::Lateral, rule)
    }
    
    /// Create cartesian distance condition
    pub fn cartesian(
        entity_ref: impl Into<OSString>,
        distance: f64,
        freespace: bool,
        rule: Rule,
    ) -> Self {
        Self::new(entity_ref, distance, freespace, RelativeDistanceType::Cartesian, rule)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::positions::WorldPosition;

    #[test]
    fn test_reach_position_condition_creation() {
        let condition = ReachPositionCondition::at_world_position(100.0, 200.0, 0.0, 1.57, 2.0);
        
        assert_eq!(condition.tolerance, Double::literal(2.0));
        assert!(condition.position.world_position.is_some());
        
        let world_pos = condition.position.world_position.unwrap();
        assert_eq!(world_pos.x, Double::literal(100.0));
        assert_eq!(world_pos.y, Double::literal(200.0));
    }

    #[test]
    fn test_distance_condition_builder() {
        let position = Position::default();
        let condition = DistanceCondition::less_than(position, 50.0, true)
            .with_coordinate_system(CoordinateSystem::Entity)
            .with_distance_type(RelativeDistanceType::Cartesian);
        
        assert_eq!(condition.value, Double::literal(50.0));
        assert_eq!(condition.freespace, Boolean::literal(true));
        assert_eq!(condition.rule, Rule::LessThan);
        assert_eq!(condition.coordinate_system, Some(CoordinateSystem::Entity));
        assert_eq!(condition.relative_distance_type, Some(RelativeDistanceType::Cartesian));
    }

    #[test]
    fn test_relative_distance_condition_types() {
        let longitudinal = RelativeDistanceCondition::longitudinal("vehicle1", 20.0, false, Rule::GreaterThan);
        assert_eq!(longitudinal.relative_distance_type, RelativeDistanceType::Longitudinal);
        
        let lateral = RelativeDistanceCondition::lateral("vehicle2", 5.0, true, Rule::LessThan);
        assert_eq!(lateral.relative_distance_type, RelativeDistanceType::Lateral);
        
        let cartesian = RelativeDistanceCondition::cartesian("vehicle3", 15.0, true, Rule::EqualTo);
        assert_eq!(cartesian.relative_distance_type, RelativeDistanceType::Cartesian);
    }

    #[test]
    fn test_spatial_condition_defaults() {
        let reach_pos = ReachPositionCondition::default();
        assert_eq!(reach_pos.tolerance, Double::literal(1.0));
        
        let distance = DistanceCondition::default();
        assert_eq!(distance.value, Double::literal(10.0));
        assert_eq!(distance.rule, Rule::LessThan);
        
        let relative_distance = RelativeDistanceCondition::default();
        assert_eq!(relative_distance.relative_distance_type, RelativeDistanceType::Cartesian);
        assert_eq!(relative_distance.rule, Rule::LessThan);
    }

    #[test]
    fn test_xml_serialization() {
        let condition = RelativeDistanceCondition::cartesian("ego_vehicle", 25.0, true, Rule::GreaterThan)
            .with_coordinate_system(CoordinateSystem::Road);
        
        let xml = quick_xml::se::to_string(&condition).expect("Failed to serialize");
        assert!(xml.contains("entityRef=\"ego_vehicle\""));
        assert!(xml.contains("value=\"25\""));
        assert!(xml.contains("freespace=\"true\""));
        assert!(xml.contains("relativeDistanceType=\"cartesianDistance\""));
        assert!(xml.contains("rule=\"greaterThan\""));
        assert!(xml.contains("coordinateSystem=\"road\""));
    }
}