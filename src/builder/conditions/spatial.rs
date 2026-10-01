//! Condition builders for spatial relationships: `DistanceCondition` to a position,
//! `RelativeDistanceCondition` between entities, `ReachPositionCondition`, and
//! `CollisionCondition`.

use crate::builder::{BuilderError, BuilderResult};
use crate::types::basic::Value;
use crate::types::{
    basic::{Double, OSString},
    conditions::entity::{ByEntityCondition, DistanceCondition, EntityCondition},
    enums::{ConditionEdge, RelativeDistanceType, Rule},
    positions::Position,
    scenario::triggers::{Condition, ConditionChoice, EntityRef, TriggeringEntities},
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
    rule: Value<Rule>,
    freespace: bool,
}

impl DistanceConditionBuilder {
    /// Create a new distance condition builder
    pub fn new() -> Self {
        Self {
            entity_ref: None,
            target_position: None,
            distance: None,
            rule: Value::Literal(Rule::LessThan),
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
        self.rule = Value::Literal(Rule::LessThan);
        self
    }

    /// Set distance threshold (entity farther than this distance triggers)
    pub fn farther_than(mut self, distance: f64) -> Self {
        self.distance = Some(distance);
        self.rule = Value::Literal(Rule::GreaterThan);
        self
    }

    /// Set distance with custom rule
    pub fn distance_rule(mut self, distance: f64, rule: Rule) -> Self {
        self.distance = Some(distance);
        self.rule = Value::Literal(rule);
        self
    }

    /// Set `rule` to a parameter reference (`rule="$name"`) -- the attribute's `xsd:union` admits a `parameter` member alongside the enumeration, so `$name` is schema-valid here; `name` omits the `$`.
    pub fn rule_param(mut self, name: &str) -> Self {
        self.rule = Value::Parameter(name.to_string());
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
            return Err(BuilderError::validation_error(
                "Entity reference is required",
            ));
        }
        if self.target_position.is_none() {
            return Err(BuilderError::validation_error(
                "Target position is required",
            ));
        }
        if self.distance.is_none() {
            return Err(BuilderError::validation_error(
                "Distance threshold is required",
            ));
        }

        Ok(Condition {
            name: OSString::literal("DistanceCondition".to_string()),
            condition_edge: Value::Literal(ConditionEdge::Rising),
            delay: Double::literal(0.0),
            choice: ConditionChoice::ByEntityCondition(ByEntityCondition {
                triggering_entities: TriggeringEntities::any(vec![EntityRef {
                    entity_ref: OSString::literal(self.entity_ref.unwrap()),
                }])?,
                entity_condition: EntityCondition::Distance(DistanceCondition {
                    position: self.target_position.unwrap(),
                    value: Double::literal(self.distance.unwrap()),
                    freespace: Value::Literal(self.freespace),
                    rule: self.rule,
                    along_route: None,
                    coordinate_system: None,
                    relative_distance_type: Some(Value::Literal(RelativeDistanceType::Cartesian)),
                    routing_algorithm: None,
                }),
            }),
        })
    }
}

/// Builder for relative distance conditions
///
/// `RelativeDistanceCondition` (`Schema/OpenSCENARIO.xsd:1843-1851`) declares `freespace`,
/// `relativeDistanceType` and `rule` all `use="required"` with no `default="…"` -- none of the
/// three has a schema-sanctioned default, so every one must be set explicitly before `build()`.
#[derive(Debug, Default)]
pub struct RelativeDistanceConditionBuilder {
    entity_ref: Option<String>,
    target_entity: Option<String>,
    distance: Option<f64>,
    rule: Option<Value<Rule>>,
    freespace: Option<bool>,
    relative_distance_type: Option<Value<RelativeDistanceType>>,
}

impl RelativeDistanceConditionBuilder {
    /// Create new relative distance condition builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set entity to monitor
    pub fn for_entity(mut self, entity_ref: &str) -> Self {
        self.entity_ref = Some(entity_ref.to_string());
        self
    }

    /// Set target entity
    pub fn to_entity(mut self, target_entity: &str) -> Self {
        self.target_entity = Some(target_entity.to_string());
        self
    }

    /// Set distance threshold (closer than)
    pub fn closer_than(mut self, distance: f64) -> Self {
        self.distance = Some(distance);
        self.rule = Some(Value::Literal(Rule::LessThan));
        self
    }

    /// Set distance threshold (farther than)
    pub fn farther_than(mut self, distance: f64) -> Self {
        self.distance = Some(distance);
        self.rule = Some(Value::Literal(Rule::GreaterThan));
        self
    }

    /// Use freespace distance calculation
    pub fn use_freespace(mut self, freespace: bool) -> Self {
        self.freespace = Some(freespace);
        self
    }

    /// Set distance type to longitudinal
    pub fn longitudinal(mut self) -> Self {
        self.relative_distance_type = Some(Value::Literal(RelativeDistanceType::Longitudinal));
        self
    }

    /// Set distance type to lateral
    pub fn lateral(mut self) -> Self {
        self.relative_distance_type = Some(Value::Literal(RelativeDistanceType::Lateral));
        self
    }

    /// Set distance type to cartesian
    pub fn cartesian(mut self) -> Self {
        self.relative_distance_type = Some(Value::Literal(RelativeDistanceType::Cartesian));
        self
    }

    /// Build the condition
    pub fn build(self) -> BuilderResult<Condition> {
        if self.entity_ref.is_none() {
            return Err(BuilderError::validation_error(
                "Entity reference is required",
            ));
        }
        if self.target_entity.is_none() {
            return Err(BuilderError::validation_error("Target entity is required"));
        }
        if self.distance.is_none() {
            return Err(BuilderError::validation_error(
                "Distance threshold is required",
            ));
        }
        let rule = self
            .rule
            .ok_or_else(|| BuilderError::validation_error("Rule is required"))?;
        let freespace = self.freespace.ok_or_else(|| {
            BuilderError::validation_error("Freespace flag is required (call use_freespace)")
        })?;
        let relative_distance_type = self.relative_distance_type.ok_or_else(|| {
            BuilderError::validation_error(
                "Relative distance type is required (call longitudinal/lateral/cartesian)",
            )
        })?;

        // Create a relative distance condition using entity condition structure
        Ok(Condition {
            name: OSString::literal("RelativeDistanceCondition".to_string()),
            condition_edge: Value::Literal(ConditionEdge::Rising),
            delay: Double::literal(0.0),
            choice: ConditionChoice::ByEntityCondition(ByEntityCondition {
                triggering_entities: TriggeringEntities::any(vec![EntityRef {
                    entity_ref: OSString::literal(self.entity_ref.unwrap()),
                }])?,
                entity_condition: EntityCondition::RelativeDistance(
                    crate::types::conditions::entity::RelativeDistanceCondition {
                        entity_ref: OSString::literal(self.target_entity.unwrap()),
                        value: Double::literal(self.distance.unwrap()),
                        freespace: Value::Literal(freespace),
                        rule,
                        relative_distance_type,
                        coordinate_system: None,
                        routing_algorithm: None,
                    },
                ),
            }),
        })
    }
}

/// Builder for collision conditions
#[derive(Debug, Default)]
pub struct CollisionConditionBuilder {
    entity_ref: Option<String>,
    target_entity: Option<String>,
    collision_type: Option<crate::types::enums::ObjectType>,
}

impl CollisionConditionBuilder {
    /// Create new collision condition builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set entity to monitor
    pub fn for_entity(mut self, entity_ref: &str) -> Self {
        self.entity_ref = Some(entity_ref.to_string());
        self
    }

    /// Set target entity for collision detection
    pub fn with_entity(mut self, target_entity: &str) -> Self {
        self.target_entity = Some(target_entity.to_string());
        self
    }

    /// Set collision type
    pub fn collision_type(mut self, collision_type: crate::types::enums::ObjectType) -> Self {
        self.collision_type = Some(collision_type);
        self
    }

    /// Build the condition. `CollisionCondition` (XSD:923-928) is a bare `xsd:choice`
    /// between a target entity and a target entity type, so exactly one of `with_entity`
    /// or `collision_type` must have been set; the entity branch takes precedence when
    /// both are given.
    pub fn build(self) -> BuilderResult<Condition> {
        if self.entity_ref.is_none() {
            return Err(BuilderError::validation_error(
                "Entity reference is required",
            ));
        }

        let choice = match (self.target_entity, self.collision_type) {
            (Some(target), _) => {
                crate::types::conditions::entity::CollisionConditionChoice::EntityRef(EntityRef {
                    entity_ref: OSString::literal(target),
                })
            }
            (None, Some(collision_type)) => {
                crate::types::conditions::entity::CollisionConditionChoice::ByType(
                    crate::types::conditions::entity::CollisionTarget {
                        target_type: Value::Literal(collision_type),
                    },
                )
            }
            (None, None) => {
                return Err(BuilderError::validation_error(
                    "Either a target entity or a collision type is required",
                ));
            }
        };

        Ok(Condition {
            name: OSString::literal("CollisionCondition".to_string()),
            condition_edge: Value::Literal(ConditionEdge::Rising),
            delay: Double::literal(0.0),
            choice: ConditionChoice::ByEntityCondition(ByEntityCondition {
                triggering_entities: TriggeringEntities::any(vec![EntityRef {
                    entity_ref: OSString::literal(self.entity_ref.unwrap()),
                }])?,
                entity_condition: EntityCondition::Collision(
                    crate::types::conditions::entity::CollisionCondition { choice },
                ),
            }),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::positions::PositionBuilder;
    use crate::types::{
        basic::Value,
        positions::{Position, WorldPosition},
    };
    fn create_test_position() -> Position {
        Position::world(WorldPosition {
            x: Double::literal(100.0),
            y: Double::literal(200.0),
            z: Some(Double::literal(0.0)),
            h: Some(Double::literal(0.0)),
            p: Some(Double::literal(0.0)),
            r: Some(Double::literal(0.0)),
        })
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

        let ConditionChoice::ByEntityCondition(by_entity) = condition.choice else {
            panic!("Expected ByEntityCondition");
        };

        match by_entity.entity_condition {
            EntityCondition::Distance(distance_condition) => {
                assert_eq!(distance_condition.value.as_literal().unwrap(), &10.0);
                assert_eq!(distance_condition.rule, Value::Literal(Rule::LessThan));
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

        let ConditionChoice::ByEntityCondition(by_entity) = condition.choice else {
            panic!("Expected ByEntityCondition");
        };
        match by_entity.entity_condition {
            EntityCondition::Distance(distance_condition) => {
                assert_eq!(distance_condition.value.as_literal().unwrap(), &50.0);
                assert_eq!(distance_condition.rule, Value::Literal(Rule::GreaterThan));
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

        let ConditionChoice::ByEntityCondition(by_entity) = condition.choice else {
            panic!("Expected ByEntityCondition");
        };
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
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Entity reference is required"));

        // Missing position
        let result = DistanceConditionBuilder::new()
            .for_entity("ego")
            .closer_than(10.0)
            .build();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Target position is required"));

        // Missing distance
        let position = create_test_position();
        let result = DistanceConditionBuilder::new()
            .for_entity("ego")
            .to_position(position)
            .build();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Distance threshold is required"));
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

        let ConditionChoice::ByEntityCondition(by_entity) = condition.choice else {
            panic!("Expected ByEntityCondition");
        };
        match by_entity.entity_condition {
            EntityCondition::Distance(distance_condition) => {
                assert_eq!(distance_condition.value.as_literal().unwrap(), &25.0);
                assert_eq!(distance_condition.rule, Value::Literal(Rule::EqualTo));
            }
            _ => panic!("Expected Distance condition"),
        }
    }

    fn condition_xml(condition: &Condition) -> String {
        quick_xml::se::to_string_with_root("Condition", condition).expect("serialize Condition")
    }

    /// Wraps an `EntityCondition` child in the `Condition`/`ByEntityCondition` envelope the
    /// builders emit for `ego` (XSD `Condition` :953, `ByEntityCondition` :825,
    /// `TriggeringEntities` :2400).
    fn by_entity_xml(name: &str, entity_condition: &str) -> String {
        format!(
            concat!(
                r#"<Condition name="{}" conditionEdge="rising" delay="0"><ByEntityCondition>"#,
                r#"<TriggeringEntities triggeringEntitiesRule="any"><EntityRef entityRef="ego"/></TriggeringEntities>"#,
                r#"<EntityCondition>{}</EntityCondition></ByEntityCondition></Condition>"#,
            ),
            name, entity_condition
        )
    }

    /// `RelativeDistanceCondition` (XSD:1843-1851): each threshold and distance-type setter
    /// lands on its own required attribute. `cartesian()` emits `cartesianDistance`, the
    /// `RelativeDistanceType` literal at XSD:448 (deprecated in 1.3, still valid).
    #[test]
    fn relative_distance_condition_builder_emits_xsd_condition() {
        let rows = [
            (
                "closer_than + longitudinal + freespace",
                RelativeDistanceConditionBuilder::new()
                    .for_entity("ego")
                    .to_entity("target")
                    .closer_than(5.0)
                    .use_freespace(true)
                    .longitudinal(),
                r#"<RelativeDistanceCondition entityRef="target" value="5" freespace="true" relativeDistanceType="longitudinal" rule="lessThan"/>"#,
            ),
            (
                "farther_than + lateral",
                RelativeDistanceConditionBuilder::new()
                    .for_entity("ego")
                    .to_entity("lead")
                    .farther_than(30.5)
                    .use_freespace(false)
                    .lateral(),
                r#"<RelativeDistanceCondition entityRef="lead" value="30.5" freespace="false" relativeDistanceType="lateral" rule="greaterThan"/>"#,
            ),
            (
                "cartesian",
                RelativeDistanceConditionBuilder::new()
                    .for_entity("ego")
                    .to_entity("target")
                    .closer_than(2.0)
                    .use_freespace(false)
                    .cartesian(),
                r#"<RelativeDistanceCondition entityRef="target" value="2" freespace="false" relativeDistanceType="cartesianDistance" rule="lessThan"/>"#,
            ),
        ];
        for (row, builder, expected) in rows {
            let condition = builder.build().unwrap_or_else(|e| panic!("{row}: {e}"));
            assert_eq!(
                condition_xml(&condition),
                by_entity_xml("RelativeDistanceCondition", expected),
                "{row}"
            );
        }
    }

    /// The three XSD-required attributes without a schema default (`freespace`,
    /// `relativeDistanceType`, `rule`) and the entity, target and value are refused when unset.
    #[test]
    fn relative_distance_condition_builder_refuses_missing_required_fields() {
        let complete = || {
            RelativeDistanceConditionBuilder::new()
                .for_entity("ego")
                .to_entity("target")
                .use_freespace(true)
                .longitudinal()
        };
        let rows = [
            (
                RelativeDistanceConditionBuilder::new()
                    .to_entity("target")
                    .closer_than(5.0)
                    .use_freespace(true)
                    .longitudinal(),
                "Entity reference is required",
            ),
            (
                RelativeDistanceConditionBuilder::new()
                    .for_entity("ego")
                    .closer_than(5.0)
                    .use_freespace(true)
                    .longitudinal(),
                "Target entity is required",
            ),
            (complete(), "Distance threshold is required"),
            (
                RelativeDistanceConditionBuilder::new()
                    .for_entity("ego")
                    .to_entity("target")
                    .closer_than(5.0)
                    .longitudinal(),
                "Freespace flag is required",
            ),
            (
                RelativeDistanceConditionBuilder::new()
                    .for_entity("ego")
                    .to_entity("target")
                    .closer_than(5.0)
                    .use_freespace(true),
                "Relative distance type is required",
            ),
        ];
        for (builder, message) in rows {
            let error = builder.build().expect_err(message).to_string();
            assert!(
                error.contains(message),
                "expected `{message}`, got `{error}`"
            );
        }
    }

    /// `CollisionCondition` (XSD:923-928) is a choice between `EntityRef` and `ByType`; the
    /// entity wins when both are set, and neither set is refused.
    #[test]
    fn collision_condition_builder_emits_one_xsd_choice_branch() {
        use crate::types::enums::ObjectType;

        let rows = [
            (
                "target entity",
                CollisionConditionBuilder::new()
                    .for_entity("ego")
                    .with_entity("target"),
                r#"<CollisionCondition><EntityRef entityRef="target"/></CollisionCondition>"#,
            ),
            (
                "object type",
                CollisionConditionBuilder::new()
                    .for_entity("ego")
                    .collision_type(ObjectType::Pedestrian),
                r#"<CollisionCondition><ByType type="pedestrian"/></CollisionCondition>"#,
            ),
            (
                "both: entity takes precedence",
                CollisionConditionBuilder::new()
                    .for_entity("ego")
                    .with_entity("target")
                    .collision_type(ObjectType::Vehicle),
                r#"<CollisionCondition><EntityRef entityRef="target"/></CollisionCondition>"#,
            ),
        ];
        for (row, builder, expected) in rows {
            let condition = builder.build().unwrap_or_else(|e| panic!("{row}: {e}"));
            assert_eq!(
                condition_xml(&condition),
                by_entity_xml("CollisionCondition", expected),
                "{row}"
            );
        }

        let neither = CollisionConditionBuilder::new().for_entity("ego").build();
        assert!(neither
            .expect_err("no target")
            .to_string()
            .contains("Either a target entity or a collision type is required"));
        let no_entity = CollisionConditionBuilder::new()
            .with_entity("target")
            .build();
        assert!(no_entity
            .expect_err("no triggering entity")
            .to_string()
            .contains("Entity reference is required"));
    }
}
