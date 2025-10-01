//! Entity condition builders for entity-specific triggers

use crate::builder::{BuilderError, BuilderResult};
use crate::types::{
    conditions::{
        entity::{
            ByEntityCondition, AccelerationCondition, SpeedCondition, TraveledDistanceCondition,
            EndOfRoadCondition, ReachPositionCondition, CollisionCondition, OffroadCondition,
            TimeHeadwayCondition, TimeToCollisionCondition, StandStillCondition, EntityCondition
        },
    },
    basic::{Double, Boolean, OSString},
    enums::{Rule, ConditionEdge, TriggeringEntitiesRule, DirectionalDimension},
    positions::Position,
    scenario::triggers::{TriggeringEntities, Condition, EntityRef},
};

/// Builder for acceleration conditions
#[derive(Debug)]
pub struct AccelerationConditionBuilder {
    entity_ref: Option<String>,
    value: Option<f64>,
    rule: Rule,
    direction: Option<DirectionalDimension>,
}

impl Default for AccelerationConditionBuilder {
    fn default() -> Self {
        Self {
            entity_ref: None,
            value: None,
            rule: Rule::GreaterThan,
            direction: None,
        }
    }
}

impl AccelerationConditionBuilder {
    /// Create new acceleration condition builder
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set target entity
    pub fn for_entity(mut self, entity_ref: &str) -> Self {
        self.entity_ref = Some(entity_ref.to_string());
        self
    }
    
    /// Set acceleration threshold
    pub fn acceleration_above(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Rule::GreaterThan;
        self
    }
    
    /// Set acceleration threshold (below)
    pub fn acceleration_below(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Rule::LessThan;
        self
    }
    
    /// Set exact acceleration value
    pub fn acceleration_equals(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Rule::EqualTo;
        self
    }
    
    /// Set direction for acceleration measurement
    pub fn with_direction(mut self, direction: DirectionalDimension) -> Self {
        self.direction = Some(direction);
        self
    }
    
    /// Build the condition
    pub fn build(self) -> BuilderResult<Condition> {
        if self.entity_ref.is_none() {
            return Err(BuilderError::validation_error("Entity reference is required"));
        }
        if self.value.is_none() {
            return Err(BuilderError::validation_error("Acceleration value is required"));
        }
        
        let acceleration_condition = AccelerationCondition {
            value: Double::literal(self.value.unwrap()),
            rule: self.rule,
            direction: self.direction,
        };
        
        let by_entity_condition = ByEntityCondition {
            triggering_entities: TriggeringEntities {
                triggering_entities_rule: TriggeringEntitiesRule::Any,
                entity_refs: vec![EntityRef {
                    entity_ref: OSString::literal(self.entity_ref.unwrap()),
                }],
            },
            entity_condition: EntityCondition::Acceleration(acceleration_condition),
        };
        
        Ok(Condition {
            name: OSString::literal("AccelerationCondition".to_string()),
            condition_edge: ConditionEdge::Rising,
            delay: Some(Double::literal(0.0)),
            by_value_condition: None,
            by_entity_condition: Some(by_entity_condition),
        })
    }
}

/// Builder for enhanced speed conditions
#[derive(Debug)]
pub struct EnhancedSpeedConditionBuilder {
    entity_ref: Option<String>,
    value: Option<f64>,
    rule: Rule,
}

impl Default for EnhancedSpeedConditionBuilder {
    fn default() -> Self {
        Self {
            entity_ref: None,
            value: None,
            rule: Rule::GreaterThan,
        }
    }
}

impl EnhancedSpeedConditionBuilder {
    /// Create new speed condition builder
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set target entity
    pub fn for_entity(mut self, entity_ref: &str) -> Self {
        self.entity_ref = Some(entity_ref.to_string());
        self
    }
    
    /// Set speed threshold (above)
    pub fn speed_above(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Rule::GreaterThan;
        self
    }
    
    /// Set speed threshold (below)
    pub fn speed_below(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Rule::LessThan;
        self
    }
    
    /// Set exact speed value
    pub fn speed_equals(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Rule::EqualTo;
        self
    }
    
    /// Build the condition
    pub fn build(self) -> BuilderResult<Condition> {
        if self.entity_ref.is_none() {
            return Err(BuilderError::validation_error("Entity reference is required"));
        }
        if self.value.is_none() {
            return Err(BuilderError::validation_error("Speed value is required"));
        }
        
        let speed_condition = SpeedCondition {
            value: Double::literal(self.value.unwrap()),
            rule: self.rule,
            entity_ref: OSString::literal(self.entity_ref.clone().unwrap()),
            direction: None,
        };
        
        let by_entity_condition = ByEntityCondition {
            triggering_entities: TriggeringEntities {
                triggering_entities_rule: TriggeringEntitiesRule::Any,
                entity_refs: vec![EntityRef {
                    entity_ref: OSString::literal(self.entity_ref.unwrap()),
                }],
            },
            entity_condition: EntityCondition::Speed(speed_condition),
        };
        
        Ok(Condition {
            name: OSString::literal("SpeedCondition".to_string()),
            condition_edge: ConditionEdge::Rising,
            delay: Some(Double::literal(0.0)),
            by_value_condition: None,
            by_entity_condition: Some(by_entity_condition),
        })
    }
}

/// Builder for traveled distance conditions
#[derive(Debug)]
pub struct TraveledDistanceConditionBuilder {
    entity_ref: Option<String>,
    value: Option<f64>,
    rule: Rule,
}

impl Default for TraveledDistanceConditionBuilder {
    fn default() -> Self {
        Self {
            entity_ref: None,
            value: None,
            rule: Rule::GreaterThan,
        }
    }
}

impl TraveledDistanceConditionBuilder {
    /// Create new traveled distance condition builder
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set target entity
    pub fn for_entity(mut self, entity_ref: &str) -> Self {
        self.entity_ref = Some(entity_ref.to_string());
        self
    }
    
    /// Set distance threshold (above)
    pub fn distance_above(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Rule::GreaterThan;
        self
    }
    
    /// Set distance threshold (below)
    pub fn distance_below(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Rule::LessThan;
        self
    }
    
    /// Set exact distance value
    pub fn distance_equals(mut self, value: f64) -> Self {
        self.value = Some(value);
        self.rule = Rule::EqualTo;
        self
    }
    
    /// Build the condition
    pub fn build(self) -> BuilderResult<Condition> {
        if self.entity_ref.is_none() {
            return Err(BuilderError::validation_error("Entity reference is required"));
        }
        if self.value.is_none() {
            return Err(BuilderError::validation_error("Distance value is required"));
        }
        
        let traveled_distance_condition = TraveledDistanceCondition {
            value: Double::literal(self.value.unwrap()),
        };
        
        let by_entity_condition = ByEntityCondition {
            triggering_entities: TriggeringEntities {
                triggering_entities_rule: TriggeringEntitiesRule::Any,
                entity_refs: vec![EntityRef {
                    entity_ref: OSString::literal(self.entity_ref.unwrap()),
                }],
            },
            entity_condition: EntityCondition::TraveledDistance(traveled_distance_condition),
        };
        
        Ok(Condition {
            name: OSString::literal("TraveledDistanceCondition".to_string()),
            condition_edge: ConditionEdge::Rising,
            delay: Some(Double::literal(0.0)),
            by_value_condition: None,
            by_entity_condition: Some(by_entity_condition),
        })
    }
}

/// Builder for reach position conditions
#[derive(Debug, Default)]
pub struct ReachPositionConditionBuilder {
    entity_ref: Option<String>,
    position: Option<Position>,
    tolerance: Option<f64>,
}

impl ReachPositionConditionBuilder {
    /// Create new reach position condition builder
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set target entity
    pub fn for_entity(mut self, entity_ref: &str) -> Self {
        self.entity_ref = Some(entity_ref.to_string());
        self
    }
    
    /// Set target position
    pub fn at_position(mut self, position: Position) -> Self {
        self.position = Some(position);
        self
    }
    
    /// Set tolerance for position matching
    pub fn with_tolerance(mut self, tolerance: f64) -> Self {
        self.tolerance = Some(tolerance);
        self
    }
    
    /// Build the condition
    pub fn build(self) -> BuilderResult<Condition> {
        if self.entity_ref.is_none() {
            return Err(BuilderError::validation_error("Entity reference is required"));
        }
        if self.position.is_none() {
            return Err(BuilderError::validation_error("Position is required"));
        }
        
        let reach_position_condition = ReachPositionCondition {
            position: self.position.unwrap(),
            tolerance: self.tolerance.map(Double::literal).unwrap_or(Double::literal(1.0)),
        };
        
        let by_entity_condition = ByEntityCondition {
            triggering_entities: TriggeringEntities {
                triggering_entities_rule: TriggeringEntitiesRule::Any,
                entity_refs: vec![EntityRef {
                    entity_ref: OSString::literal(self.entity_ref.unwrap()),
                }],
            },
            entity_condition: EntityCondition::ReachPosition(reach_position_condition),
        };
        
        Ok(Condition {
            name: OSString::literal("ReachPositionCondition".to_string()),
            condition_edge: ConditionEdge::Rising,
            delay: Some(Double::literal(0.0)),
            by_value_condition: None,
            by_entity_condition: Some(by_entity_condition),
        })
    }
}

/// Builder for end of road conditions
#[derive(Debug, Default)]
pub struct EndOfRoadConditionBuilder {
    entity_ref: Option<String>,
    duration: Option<f64>,
}

impl EndOfRoadConditionBuilder {
    /// Create new end of road condition builder
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set target entity
    pub fn for_entity(mut self, entity_ref: &str) -> Self {
        self.entity_ref = Some(entity_ref.to_string());
        self
    }
    
    /// Set duration threshold
    pub fn with_duration(mut self, duration: f64) -> Self {
        self.duration = Some(duration);
        self
    }
    
    /// Build the condition
    pub fn build(self) -> BuilderResult<Condition> {
        if self.entity_ref.is_none() {
            return Err(BuilderError::validation_error("Entity reference is required"));
        }
        
        let end_of_road_condition = EndOfRoadCondition {
            duration: self.duration.map(Double::literal).unwrap_or(Double::literal(0.0)),
        };
        
        let by_entity_condition = ByEntityCondition {
            triggering_entities: TriggeringEntities {
                triggering_entities_rule: TriggeringEntitiesRule::Any,
                entity_refs: vec![EntityRef {
                    entity_ref: OSString::literal(self.entity_ref.unwrap()),
                }],
            },
            entity_condition: EntityCondition::EndOfRoad(end_of_road_condition),
        };
        
        Ok(Condition {
            name: OSString::literal("EndOfRoadCondition".to_string()),
            condition_edge: ConditionEdge::Rising,
            delay: Some(Double::literal(0.0)),
            by_value_condition: None,
            by_entity_condition: Some(by_entity_condition),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acceleration_condition_builder() {
        let condition = AccelerationConditionBuilder::new()
            .for_entity("ego")
            .acceleration_above(2.0)
            .build()
            .unwrap();
            
        if let Some(by_entity) = condition.by_entity_condition {
            if let EntityCondition::Acceleration(acc_condition) = by_entity.entity_condition {
                assert_eq!(acc_condition.value.value(), 2.0);
                assert_eq!(acc_condition.rule, Rule::GreaterThan);
            } else {
                panic!("Expected Acceleration condition");
            }
        } else {
            panic!("Expected ByEntityCondition");
        }
    }

    #[test]
    fn test_speed_condition_builder() {
        let condition = EnhancedSpeedConditionBuilder::new()
            .for_entity("ego")
            .speed_below(30.0)
            .build()
            .unwrap();
            
        if let Some(by_entity) = condition.by_entity_condition {
            if let EntityCondition::Speed(speed_condition) = by_entity.entity_condition {
                assert_eq!(speed_condition.value.value(), 30.0);
                assert_eq!(speed_condition.rule, Rule::LessThan);
            } else {
                panic!("Expected Speed condition");
            }
        } else {
            panic!("Expected ByEntityCondition");
        }
    }

    #[test]
    fn test_traveled_distance_condition_builder() {
        let condition = TraveledDistanceConditionBuilder::new()
            .for_entity("ego")
            .distance_above(100.0)
            .build()
            .unwrap();
            
        if let Some(by_entity) = condition.by_entity_condition {
            if let EntityCondition::TraveledDistance(distance_condition) = by_entity.entity_condition {
                assert_eq!(distance_condition.value.value(), 100.0);
            } else {
                panic!("Expected TraveledDistance condition");
            }
        } else {
            panic!("Expected ByEntityCondition");
        }
    }

    #[test]
    fn test_reach_position_condition_builder() {
        use crate::types::positions::{Position, WorldPosition};
        use crate::types::basic::Double;
        
        let position = Position {
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
        };
        
        let condition = ReachPositionConditionBuilder::new()
            .for_entity("ego")
            .at_position(position)
            .with_tolerance(2.0)
            .build()
            .unwrap();
            
        if let Some(by_entity) = condition.by_entity_condition {
            if let EntityCondition::ReachPosition(reach_condition) = by_entity.entity_condition {
                assert_eq!(reach_condition.tolerance.value(), 2.0);
            } else {
                panic!("Expected ReachPosition condition");
            }
        } else {
            panic!("Expected ByEntityCondition");
        }
    }

    #[test]
    fn test_end_of_road_condition_builder() {
        let condition = EndOfRoadConditionBuilder::new()
            .for_entity("ego")
            .with_duration(1.0)
            .build()
            .unwrap();
            
        if let Some(by_entity) = condition.by_entity_condition {
            if let EntityCondition::EndOfRoad(end_condition) = by_entity.entity_condition {
                assert_eq!(end_condition.duration.value(), 1.0);
            } else {
                panic!("Expected EndOfRoad condition");
            }
        } else {
            panic!("Expected ByEntityCondition");
        }
    }
}