//! Spatial condition builders (distance, position, etc.)
//!
//! This module provides builders for creating spatial conditions that trigger
//! based on entity positions, distances, and spatial relationships.
//!
//! # Supported Conditions
//!
//! - **DistanceCondition**: Triggers based on distance to a position
//! - **ReachPositionCondition**: Triggers when entity reaches a position
//! - **RelativeDistanceCondition**: Triggers based on distance between entities
//! - **CollisionCondition**: Triggers on collision detection
//!
//! # Usage
//!
//! ```rust
//! use openscenario_rs::builder::conditions::DistanceConditionBuilder;
//! use openscenario_rs::builder::positions::WorldPositionBuilder;
//!
//! let position = WorldPositionBuilder::new()
//!     .x(100.0)
//!     .y(200.0)
//!     .z(0.0)
//!     .finish()?;
//!
//! let condition = DistanceConditionBuilder::new()
//!     .for_entity("ego")
//!     .to_position(position)
//!     .closer_than(10.0)
//!     .build()?;
//! ```

use crate::builder::{BuilderError, BuilderResult};
use crate::types::{
    conditions::entity::{DistanceCondition, ByEntityCondition, EntityCondition},
    scenario::triggers::{Condition, TriggeringEntities, EntityRef},
    positions::Position,
    basic::{Double, OSString},
    enums::{Rule, TriggeringEntitiesRule, ConditionEdge, RelativeDistanceType},
};

/// Builder for distance conditions
///
/// Creates conditions that trigger when an entity is within a certain distance
/// of a target position. Supports both closer-than and farther-than triggers.
#[derive(Debug)]
pub struct DistanceConditionBuilder {
    entity_ref: Option<String>,
    target_position: Option<Position>,
    distance: Option<f64>,
    rule: Rule,
    freespace: bool,
}

impl DistanceConditionBuilder {
    /// Create a new distance condition builder
    pub fn new() -> Self {
        Self {
            entity_ref: None,
            target_position: None,
            distance: None,
            rule: Rule::LessThan,
            freespace: false,
        }
    }
    
    /// Set entity to monitor
    pub fn for_entity(mut self, entity_ref: &str) -> Self {
        self.entity_ref = Some(entity_ref.to_string());
        self
    }
    
    /// Set target position
    pub fn to_position(mut self, position: Position) -> Self {
        self.target_position = Some(position);
        self
    }
    
    /// Set distance threshold (entity closer than this distance triggers)
    pub fn closer_than(mut self, distance: f64) -> Self {
        self.distance = Some(distance);
        self.rule = Rule::LessThan;
        self
    }
    
    /// Set distance threshold (entity farther than this distance triggers)
    pub fn farther_than(mut self, distance: f64) -> Self {
        self.distance = Some(distance);
        self.rule = Rule::GreaterThan;
        self
    }
    
    /// Set distance with custom rule
    pub fn distance_rule(mut self, distance: f64, rule: Rule) -> Self {
        self.distance = Some(distance);
        self.rule = rule;
        self
    }
    
    /// Use freespace distance (bounding box edges) instead of reference point
    pub fn use_freespace(mut self, freespace: bool) -> Self {
        self.freespace = freespace;
        self
    }
    
    /// Build the condition
    pub fn build(self) -> BuilderResult<Condition> {
        if self.entity_ref.is_none() {
            return Err(BuilderError::validation_error("Entity reference is required"));
        }
        if self.target_position.is_none() {
            return Err(BuilderError::validation_error("Target position is required"));
        }
        if self.distance.is_none() {
            return Err(BuilderError::validation_error("Distance threshold is required"));
        }
        
        Ok(Condition {
            name: OSString::literal("DistanceCondition".to_string()),
            condition_edge: ConditionEdge::Rising,
            delay: None,
            by_value_condition: None,
            by_entity_condition: Some(ByEntityCondition {
                triggering_entities: TriggeringEntities {
                    triggering_entities_rule: TriggeringEntitiesRule::Any,
                    entity_refs: vec![EntityRef {
                        entity_ref: OSString::literal(self.entity_ref.unwrap()),
                    }],
                },
                entity_condition: EntityCondition::Distance(DistanceCondition {
                    position: self.target_position.unwrap(),
                    value: Double::literal(self.distance.unwrap()),
                    freespace: crate::types::basic::Value::Literal(self.freespace),
                    rule: self.rule,
                    along_route: None,
                    coordinate_system: None,
                    relative_distance_type: Some(RelativeDistanceType::Cartesian),
                    routing_algorithm: None,
                }),
            }),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        positions::{Position, WorldPosition},
        basic::Value,
    };

    fn create_test_position() -> Position {
        Position {
            world_position: Some(WorldPosition {
                x: Double::literal(100.0),
                y: Double::literal(200.0),
                z: Some(Double::literal(0.0)),
                h: Some(Double::literal(0.0)),
                p: Some(Double::literal(0.0)),
                r: Some(Double::literal(0.0)),
            }),
            relative_world_position: None,
            road_position: None,
            relative_road_position: None,
            lane_position: None,
            relative_lane_position: None,
            trajectory_position: None,
            geographic_position: None,
            relative_object_position: None,
        }
    }

    #[test]
    fn test_distance_condition_builder() {
        let position = create_test_position();
        
        let condition = DistanceConditionBuilder::new()
            .for_entity("ego")
            .to_position(position)
            .closer_than(10.0)
            .build()
            .unwrap();
            
        assert!(condition.by_entity_condition.is_some());
        let by_entity = condition.by_entity_condition.unwrap();
        
        match by_entity.entity_condition {
            EntityCondition::Distance(distance_condition) => {
                assert_eq!(distance_condition.value.as_literal().unwrap(), &10.0);
                assert_eq!(distance_condition.rule, Rule::LessThan);
                assert_eq!(distance_condition.freespace.as_literal().unwrap(), &false);
            }
            _ => panic!("Expected Distance condition"),
        }
    }

    #[test]
    fn test_distance_condition_farther_than() {
        let position = create_test_position();
        
        let condition = DistanceConditionBuilder::new()
            .for_entity("target")
            .to_position(position)
            .farther_than(50.0)
            .build()
            .unwrap();
            
        let by_entity = condition.by_entity_condition.unwrap();
        match by_entity.entity_condition {
            EntityCondition::Distance(distance_condition) => {
                assert_eq!(distance_condition.value.as_literal().unwrap(), &50.0);
                assert_eq!(distance_condition.rule, Rule::GreaterThan);
            }
            _ => panic!("Expected Distance condition"),
        }
    }

    #[test]
    fn test_distance_condition_with_freespace() {
        let position = create_test_position();
        
        let condition = DistanceConditionBuilder::new()
            .for_entity("ego")
            .to_position(position)
            .closer_than(5.0)
            .use_freespace(true)
            .build()
            .unwrap();
            
        let by_entity = condition.by_entity_condition.unwrap();
        match by_entity.entity_condition {
            EntityCondition::Distance(distance_condition) => {
                assert_eq!(distance_condition.freespace.as_literal().unwrap(), &true);
            }
            _ => panic!("Expected Distance condition"),
        }
    }

    #[test]
    fn test_distance_condition_validation() {
        // Missing entity reference
        let position = create_test_position();
        let result = DistanceConditionBuilder::new()
            .to_position(position)
            .closer_than(10.0)
            .build();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Entity reference is required"));

        // Missing position
        let result = DistanceConditionBuilder::new()
            .for_entity("ego")
            .closer_than(10.0)
            .build();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Target position is required"));

        // Missing distance
        let position = create_test_position();
        let result = DistanceConditionBuilder::new()
            .for_entity("ego")
            .to_position(position)
            .build();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Distance threshold is required"));
    }

    #[test]
    fn test_distance_condition_custom_rule() {
        let position = create_test_position();
        
        let condition = DistanceConditionBuilder::new()
            .for_entity("ego")
            .to_position(position)
            .distance_rule(25.0, Rule::EqualTo)
            .build()
            .unwrap();
            
        let by_entity = condition.by_entity_condition.unwrap();
        match by_entity.entity_condition {
            EntityCondition::Distance(distance_condition) => {
                assert_eq!(distance_condition.value.as_literal().unwrap(), &25.0);
                assert_eq!(distance_condition.rule, Rule::EqualTo);
            }
            _ => panic!("Expected Distance condition"),
        }
    }
}