//! Lateral action builders (LaneChangeAction, LateralDistanceAction, LaneOffsetAction)

use crate::builder::actions::base::{ActionBuilder, ManeuverAction};
use crate::builder::{BuilderError, BuilderResult};
use crate::types::{
    actions::movement::{
        AbsoluteTargetLane, AbsoluteTargetLaneOffset, DynamicConstraints, LaneChangeAction,
        LaneChangeTarget, LaneChangeTargetChoice, LaneOffsetAction, LaneOffsetActionDynamics,
        LaneOffsetTarget, LaneOffsetTargetChoice, LateralAction, LateralDistanceAction,
        RelativeTargetLane, RelativeTargetLaneOffset, TransitionDynamics,
    },
    actions::wrappers::{CorePrivateAction, PrivateAction},
    basic::{Boolean, Double, Int, OSString},
    enums::{DynamicsDimension, DynamicsShape},
};

/// Builder for lane change actions
#[derive(Debug, Default)]
pub struct LaneChangeActionBuilder {
    entity_ref: Option<String>,
    target_lane_offset: Option<f64>,
    dynamics: Option<TransitionDynamics>,
    target: Option<LaneChangeTargetChoice>,
}

impl LaneChangeActionBuilder {
    /// Create new lane change action builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set target entity for this action
    pub fn for_entity(mut self, entity_ref: &str) -> Self {
        self.entity_ref = Some(entity_ref.to_string());
        self
    }

    /// Set target lane offset
    pub fn with_lane_offset(mut self, offset: f64) -> Self {
        self.target_lane_offset = Some(offset);
        self
    }

    /// Set transition dynamics
    pub fn with_dynamics(mut self, dynamics: TransitionDynamics) -> Self {
        self.dynamics = Some(dynamics);
        self
    }

    /// Set dynamics with simple parameters
    pub fn with_simple_dynamics(mut self, duration: f64) -> Self {
        self.dynamics = Some(TransitionDynamics {
            dynamics_dimension: DynamicsDimension::Time,
            dynamics_shape: DynamicsShape::Linear,
            value: Double::literal(duration),
        });
        self
    }

    /// Target relative lane change (relative to another entity)
    pub fn to_relative_lane(mut self, entity_ref: &str, lane_offset: i32) -> Self {
        self.target = Some(LaneChangeTargetChoice::RelativeTargetLane(
            RelativeTargetLane {
                entity_ref: OSString::literal(entity_ref.to_string()),
                value: Int::literal(lane_offset),
            },
        ));
        self
    }

    /// Target absolute lane change
    pub fn to_absolute_lane(mut self, lane_id: &str) -> Self {
        self.target = Some(LaneChangeTargetChoice::AbsoluteTargetLane(
            AbsoluteTargetLane {
                value: OSString::literal(lane_id.to_string()),
            },
        ));
        self
    }
}

impl ActionBuilder for LaneChangeActionBuilder {
    fn build_action(self) -> BuilderResult<PrivateAction> {
        self.validate()?;

        let lane_change_action = LaneChangeAction {
            target_lane_offset: self.target_lane_offset.map(Double::literal),
            lane_change_action_dynamics: self.dynamics.unwrap_or_else(|| TransitionDynamics {
                dynamics_dimension: DynamicsDimension::Time,
                dynamics_shape: DynamicsShape::Linear,
                value: Double::literal(2.0),
            }),
            lane_change_target: LaneChangeTarget {
                target_choice: self.target.unwrap(),
            },
        };

        Ok(PrivateAction {
            action: CorePrivateAction::LateralAction(LateralAction::lane_change(
                lane_change_action,
            )),
        })
    }

    fn validate(&self) -> BuilderResult<()> {
        if self.target.is_none() {
            return Err(BuilderError::validation_error(
                "Lane change target is required",
            ));
        }
        Ok(())
    }
}

impl ManeuverAction for LaneChangeActionBuilder {
    fn entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

/// Builder for lateral distance actions
#[derive(Debug, Default)]
pub struct LateralDistanceActionBuilder {
    entity_ref: Option<String>,
    target_entity: Option<String>,
    distance: Option<f64>,
    freespace: bool,
    continuous: bool,
    dynamic_constraints: Option<DynamicConstraints>,
}

impl LateralDistanceActionBuilder {
    /// Create new lateral distance action builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set target entity for this action
    pub fn for_entity(mut self, entity_ref: &str) -> Self {
        self.entity_ref = Some(entity_ref.to_string());
        self
    }

    /// Set target entity to maintain distance from
    pub fn from_entity(mut self, target_entity: &str) -> Self {
        self.target_entity = Some(target_entity.to_string());
        self
    }

    /// Set target distance
    pub fn at_distance(mut self, distance: f64) -> Self {
        self.distance = Some(distance);
        self
    }

    /// Use freespace calculation
    pub fn with_freespace(mut self, freespace: bool) -> Self {
        self.freespace = freespace;
        self
    }

    /// Set continuous mode
    pub fn continuous(mut self, continuous: bool) -> Self {
        self.continuous = continuous;
        self
    }

    /// Set dynamic constraints
    pub fn with_constraints(mut self, constraints: DynamicConstraints) -> Self {
        self.dynamic_constraints = Some(constraints);
        self
    }
}

impl ActionBuilder for LateralDistanceActionBuilder {
    fn build_action(self) -> BuilderResult<PrivateAction> {
        self.validate()?;

        let lateral_distance_action = LateralDistanceAction {
            entity_ref: OSString::literal(self.target_entity.unwrap()),
            distance: self.distance.map(Double::literal),
            freespace: Boolean::literal(self.freespace),
            continuous: Boolean::literal(self.continuous),
            dynamic_constraints: self.dynamic_constraints,
        };

        Ok(PrivateAction {
            action: CorePrivateAction::LateralAction(LateralAction::lateral_distance(
                lateral_distance_action,
            )),
        })
    }

    fn validate(&self) -> BuilderResult<()> {
        if self.target_entity.is_none() {
            return Err(BuilderError::validation_error(
                "Target entity is required for lateral distance action",
            ));
        }
        Ok(())
    }
}

impl ManeuverAction for LateralDistanceActionBuilder {
    fn entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

/// Builder for lane offset actions
#[derive(Debug, Default)]
pub struct LaneOffsetActionBuilder {
    entity_ref: Option<String>,
    continuous: bool,
    dynamics: Option<LaneOffsetActionDynamics>,
    target: Option<LaneOffsetTargetChoice>,
}

impl LaneOffsetActionBuilder {
    /// Create new lane offset action builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set target entity for this action
    pub fn for_entity(mut self, entity_ref: &str) -> Self {
        self.entity_ref = Some(entity_ref.to_string());
        self
    }

    /// Set continuous mode
    pub fn continuous(mut self, continuous: bool) -> Self {
        self.continuous = continuous;
        self
    }

    /// Set dynamics
    pub fn with_dynamics(mut self, dynamics: LaneOffsetActionDynamics) -> Self {
        self.dynamics = Some(dynamics);
        self
    }

    /// Set simple dynamics
    pub fn with_simple_dynamics(
        mut self,
        shape: DynamicsShape,
        max_lateral_acc: Option<f64>,
    ) -> Self {
        self.dynamics = Some(LaneOffsetActionDynamics {
            dynamics_shape: shape,
            max_lateral_acc: max_lateral_acc.map(Double::literal),
        });
        self
    }

    /// Target relative lane offset (relative to another entity)
    pub fn to_relative_offset(mut self, entity_ref: &str, offset: f64) -> Self {
        self.target = Some(LaneOffsetTargetChoice::RelativeTargetLaneOffset(
            RelativeTargetLaneOffset {
                entity_ref: OSString::literal(entity_ref.to_string()),
                value: Double::literal(offset),
            },
        ));
        self
    }

    /// Target absolute lane offset
    pub fn to_absolute_offset(mut self, offset: f64) -> Self {
        self.target = Some(LaneOffsetTargetChoice::AbsoluteTargetLaneOffset(
            AbsoluteTargetLaneOffset {
                value: Double::literal(offset),
            },
        ));
        self
    }
}

impl ActionBuilder for LaneOffsetActionBuilder {
    fn build_action(self) -> BuilderResult<PrivateAction> {
        self.validate()?;

        let lane_offset_action = LaneOffsetAction {
            continuous: Boolean::literal(self.continuous),
            dynamics: self.dynamics.unwrap_or_else(|| LaneOffsetActionDynamics {
                dynamics_shape: DynamicsShape::Linear,
                max_lateral_acc: None,
            }),
            target: LaneOffsetTarget {
                target_choice: self.target.unwrap(),
            },
        };

        Ok(PrivateAction {
            action: CorePrivateAction::LateralAction(LateralAction::lane_offset(
                lane_offset_action,
            )),
        })
    }

    fn validate(&self) -> BuilderResult<()> {
        if self.target.is_none() {
            return Err(BuilderError::validation_error(
                "Lane offset target is required",
            ));
        }
        Ok(())
    }
}

impl ManeuverAction for LaneOffsetActionBuilder {
    fn entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::enums::DynamicsShape;

    #[test]
    fn test_lane_change_action_builder() {
        let action = LaneChangeActionBuilder::new()
            .for_entity("ego")
            .to_absolute_lane("1")
            .with_lane_offset(0.5)
            .with_simple_dynamics(2.0)
            .build_action()
            .unwrap();

        // Verify the action was built correctly
        if let CorePrivateAction::LateralAction(lateral_action) = action.action {
            if let crate::types::actions::movement::LateralActionChoice::LaneChangeAction(
                lane_change,
            ) = lateral_action.lateral_choice
            {
                assert_eq!(
                    *lane_change
                        .target_lane_offset
                        .unwrap()
                        .as_literal()
                        .unwrap(),
                    0.5
                );
                assert_eq!(
                    *lane_change
                        .lane_change_action_dynamics
                        .value
                        .as_literal()
                        .unwrap(),
                    2.0
                );
            } else {
                panic!("Expected LaneChangeAction");
            }
        } else {
            panic!("Expected LateralAction");
        }
    }

    #[test]
    fn test_lateral_distance_action_builder() {
        let action = LateralDistanceActionBuilder::new()
            .for_entity("ego")
            .from_entity("target")
            .at_distance(5.0)
            .with_freespace(true)
            .continuous(true)
            .build_action()
            .unwrap();

        // Verify the action was built correctly
        if let CorePrivateAction::LateralAction(lateral_action) = action.action {
            if let crate::types::actions::movement::LateralActionChoice::LateralDistanceAction(
                distance_action,
            ) = lateral_action.lateral_choice
            {
                assert_eq!(distance_action.entity_ref.as_literal().unwrap(), "target");
                assert_eq!(
                    *distance_action.distance.unwrap().as_literal().unwrap(),
                    5.0
                );
                assert!(*distance_action.freespace.as_literal().unwrap());
                assert!(*distance_action.continuous.as_literal().unwrap());
            } else {
                panic!("Expected LateralDistanceAction");
            }
        } else {
            panic!("Expected LateralAction");
        }
    }

    #[test]
    fn test_lane_offset_action_builder() {
        let action = LaneOffsetActionBuilder::new()
            .for_entity("ego")
            .to_absolute_offset(1.5)
            .continuous(false)
            .with_simple_dynamics(DynamicsShape::Linear, Some(2.0))
            .build_action()
            .unwrap();

        // Verify the action was built correctly
        if let CorePrivateAction::LateralAction(lateral_action) = action.action {
            if let crate::types::actions::movement::LateralActionChoice::LaneOffsetAction(
                offset_action,
            ) = lateral_action.lateral_choice
            {
                assert!(!*offset_action.continuous.as_literal().unwrap());
                assert_eq!(
                    *offset_action
                        .dynamics
                        .max_lateral_acc
                        .unwrap()
                        .as_literal()
                        .unwrap(),
                    2.0
                );
            } else {
                panic!("Expected LaneOffsetAction");
            }
        } else {
            panic!("Expected LateralAction");
        }
    }
}
