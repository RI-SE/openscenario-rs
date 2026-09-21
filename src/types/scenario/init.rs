//! The `Init` block: the state of the world before the story starts.
//!
//! Its `Actions` container holds `GlobalAction`s for environment and infrastructure
//! and `Private` actions for each entity's starting position, speed and controller.
use crate::types::actions::appearance::{AppearanceAction, VisibilityAction};
use crate::types::actions::control::{ActivateControllerAction, ControllerAction};
use crate::types::actions::movement::{
    LongitudinalDistanceAction, RoutingAction, SpeedAction, SpeedProfileAction, SynchronizeAction,
    TeleportAction,
};
use crate::types::actions::trailer::TrailerAction;
use crate::types::basic::OSString;
use crate::types::environment::Environment;
use serde::{Deserialize, Serialize};

/// Initialization structure containing actions to run at scenario start
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Init {
    #[serde(rename = "Actions")]
    pub actions: Actions,
}

/// Actions container holding all initialization actions
///
/// XSD `InitActions` (:1316-1321): a sequence of `GlobalAction*`,
/// `UserDefinedAction*`, `Private*` (all `minOccurs=0 maxOccurs=unbounded`).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Actions {
    #[serde(rename = "GlobalAction", default)]
    pub global_actions: Vec<GlobalAction>,
    #[serde(
        rename = "UserDefinedAction",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub user_defined_actions: Vec<crate::types::actions::wrappers::UserDefinedAction>,
    #[serde(rename = "Private", default)]
    pub private_actions: Vec<Private>,
}

/// Global actions that affect the entire scenario.
///
/// XSD `GlobalAction` (:1282-1295) is a bare `xsd:choice` of `EnvironmentAction |
/// EntityAction | InfrastructureAction | SetMonitorAction | ParameterAction (deprecated) |
/// TrafficAction | VariableAction`. Neither the choice nor any of its branches carries
/// `minOccurs`, so every branch defaults to 1 and exactly one must be selected. A bare
/// `$value` states that directly: serde rejects a document naming no branch with
/// `missing field $value` and one naming two with `duplicate field $value`, hence the
/// cardinality is a property of the type rather than a convention its callers must
/// remember to check.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GlobalAction {
    #[serde(rename = "$value")]
    pub action: GlobalActionChoice,
}

/// The seven branches of the XSD `GlobalAction` choice (:1282-1295), in schema order.
///
/// Each payload is a struct, not another externally-tagged enum. An enum nested directly
/// inside an enum deserializes and then fails to serialize, since each level would have to
/// write its own element name from one position.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum GlobalActionChoice {
    EnvironmentAction(EnvironmentAction),
    EntityAction(crate::types::actions::wrappers::EntityAction),
    InfrastructureAction(crate::types::actions::wrappers::InfrastructureAction),
    SetMonitorAction(crate::types::actions::wrappers::SetMonitorAction),
    /// Deprecated in the schema (XSD:1288) but still valid OpenSCENARIO 1.3.
    ParameterAction(crate::types::actions::wrappers::ParameterAction),
    TrafficAction(crate::types::actions::wrappers::TrafficAction),
    VariableAction(crate::types::actions::wrappers::VariableAction),
}

impl GlobalAction {
    /// `<EnvironmentAction>` branch of the XSD `GlobalAction` choice (:1283).
    pub fn environment(action: EnvironmentAction) -> Self {
        Self {
            action: GlobalActionChoice::EnvironmentAction(action),
        }
    }

    /// `<EntityAction>` branch of the XSD `GlobalAction` choice (:1284).
    pub fn entity(action: crate::types::actions::wrappers::EntityAction) -> Self {
        Self {
            action: GlobalActionChoice::EntityAction(action),
        }
    }

    /// `<InfrastructureAction>` branch of the XSD `GlobalAction` choice (:1285).
    pub fn infrastructure(action: crate::types::actions::wrappers::InfrastructureAction) -> Self {
        Self {
            action: GlobalActionChoice::InfrastructureAction(action),
        }
    }

    /// `<SetMonitorAction>` branch of the XSD `GlobalAction` choice (:1286).
    pub fn set_monitor(action: crate::types::actions::wrappers::SetMonitorAction) -> Self {
        Self {
            action: GlobalActionChoice::SetMonitorAction(action),
        }
    }

    /// `<ParameterAction>` branch of the XSD `GlobalAction` choice (:1287-1289).
    /// The schema marks this branch deprecated and still declares it.
    pub fn parameter(action: crate::types::actions::wrappers::ParameterAction) -> Self {
        Self {
            action: GlobalActionChoice::ParameterAction(action),
        }
    }

    /// `<TrafficAction>` branch of the XSD `GlobalAction` choice (:1290).
    pub fn traffic(action: crate::types::actions::wrappers::TrafficAction) -> Self {
        Self {
            action: GlobalActionChoice::TrafficAction(action),
        }
    }

    /// `<VariableAction>` branch of the XSD `GlobalAction` choice (:1291).
    pub fn variable(action: crate::types::actions::wrappers::VariableAction) -> Self {
        Self {
            action: GlobalActionChoice::VariableAction(action),
        }
    }

    /// The XSD element name of the selected branch.
    pub fn action_type(&self) -> &'static str {
        match &self.action {
            GlobalActionChoice::EnvironmentAction(_) => "EnvironmentAction",
            GlobalActionChoice::EntityAction(_) => "EntityAction",
            GlobalActionChoice::InfrastructureAction(_) => "InfrastructureAction",
            GlobalActionChoice::SetMonitorAction(_) => "SetMonitorAction",
            GlobalActionChoice::ParameterAction(_) => "ParameterAction",
            GlobalActionChoice::TrafficAction(_) => "TrafficAction",
            GlobalActionChoice::VariableAction(_) => "VariableAction",
        }
    }
}

/// XSD `EnvironmentAction` (:1195-1200): a bare `xsd:choice` of `Environment` |
/// `CatalogReference` (to a `CatalogEnvironment` catalog entry). Neither branch
/// carries `minOccurs`, so both default to 1 and exactly one is required: a bare
/// `$value` states that directly, and serde rejects a document naming no branch
/// with `missing field $value` and one naming two with `duplicate field $value`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnvironmentAction {
    #[serde(rename = "$value")]
    pub action: EnvironmentActionChoice,
}

/// The two branches of the XSD `EnvironmentAction` choice (:1195-1200).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum EnvironmentActionChoice {
    Environment(Environment),
    CatalogReference(
        crate::types::catalogs::references::CatalogReference<
            crate::types::catalogs::environments::CatalogEnvironment,
        >,
    ),
}

impl EnvironmentAction {
    /// `<Environment>` branch of the XSD `EnvironmentAction` choice (:1195-1200).
    pub fn environment(environment: Environment) -> Self {
        Self {
            action: EnvironmentActionChoice::Environment(environment),
        }
    }

    /// `<CatalogReference>` branch of the XSD `EnvironmentAction` choice (:1195-1200).
    pub fn catalog_reference(
        catalog_reference: crate::types::catalogs::references::CatalogReference<
            crate::types::catalogs::environments::CatalogEnvironment,
        >,
    ) -> Self {
        Self {
            action: EnvironmentActionChoice::CatalogReference(catalog_reference),
        }
    }
}

/// Private actions specific to individual entities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Private {
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,
    #[serde(rename = "PrivateAction")]
    pub private_actions: Vec<PrivateAction>,
}

/// A private action applied to one entity.
///
/// XSD `PrivateAction` (:1777-1791) is a bare `xsd:choice` of ten branches. Nothing on the
/// choice or on any branch carries `minOccurs`, so exactly one branch must be selected and a
/// bare `$value` is the shape that says so. The previous parallel-`Option` shape said
/// something weaker: ten independent optional slots, narrowed back to the schema's meaning by
/// a `validate()` every caller had to remember to invoke. A document selecting no branch and a
/// document selecting two were both accepted, and the second kept both branches and
/// re-serialized them.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrivateAction {
    #[serde(rename = "$value")]
    pub action: PrivateActionChoice,
}

/// The ten branches of the XSD `PrivateAction` choice (:1778-1790), in schema order.
///
/// Every payload is a struct. `LongitudinalAction` is itself a choice, and it appears here as
/// its wrapper struct rather than as the bare `LongitudinalActionChoice`, because an
/// externally-tagged enum nested directly inside another fails to serialize. The wrapper holds
/// its own `$value`, so each level writes exactly one element name.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum PrivateActionChoice {
    LongitudinalAction(LongitudinalAction),
    LateralAction(crate::types::actions::movement::LateralAction),
    VisibilityAction(VisibilityAction),
    SynchronizeAction(SynchronizeAction),
    /// Deprecated in the schema (XSD:1782-1784) but still valid OpenSCENARIO 1.3.
    ActivateControllerAction(ActivateControllerAction),
    ControllerAction(ControllerAction),
    TeleportAction(TeleportAction),
    RoutingAction(RoutingAction),
    AppearanceAction(AppearanceAction),
    TrailerAction(TrailerAction),
}

impl PrivateAction {
    /// `<LongitudinalAction>` branch of the XSD `PrivateAction` choice (:1778).
    pub fn longitudinal(action: LongitudinalAction) -> Self {
        Self {
            action: PrivateActionChoice::LongitudinalAction(action),
        }
    }

    /// `<LateralAction>` branch of the XSD `PrivateAction` choice (:1779).
    pub fn lateral(action: crate::types::actions::movement::LateralAction) -> Self {
        Self {
            action: PrivateActionChoice::LateralAction(action),
        }
    }

    /// `<VisibilityAction>` branch of the XSD `PrivateAction` choice (:1780).
    pub fn visibility(action: VisibilityAction) -> Self {
        Self {
            action: PrivateActionChoice::VisibilityAction(action),
        }
    }

    /// `<SynchronizeAction>` branch of the XSD `PrivateAction` choice (:1781).
    pub fn synchronize(action: SynchronizeAction) -> Self {
        Self {
            action: PrivateActionChoice::SynchronizeAction(action),
        }
    }

    /// `<ActivateControllerAction>` branch of the XSD `PrivateAction` choice (:1782-1784).
    /// The schema marks this branch deprecated and still declares it.
    pub fn activate_controller(action: ActivateControllerAction) -> Self {
        Self {
            action: PrivateActionChoice::ActivateControllerAction(action),
        }
    }

    /// `<ControllerAction>` branch of the XSD `PrivateAction` choice (:1785).
    pub fn controller(action: ControllerAction) -> Self {
        Self {
            action: PrivateActionChoice::ControllerAction(action),
        }
    }

    /// `<TeleportAction>` branch of the XSD `PrivateAction` choice (:1786).
    pub fn teleport(action: TeleportAction) -> Self {
        Self {
            action: PrivateActionChoice::TeleportAction(action),
        }
    }

    /// `<RoutingAction>` branch of the XSD `PrivateAction` choice (:1787).
    pub fn routing(action: RoutingAction) -> Self {
        Self {
            action: PrivateActionChoice::RoutingAction(action),
        }
    }

    /// `<AppearanceAction>` branch of the XSD `PrivateAction` choice (:1788).
    pub fn appearance(action: AppearanceAction) -> Self {
        Self {
            action: PrivateActionChoice::AppearanceAction(action),
        }
    }

    /// `<TrailerAction>` branch of the XSD `PrivateAction` choice (:1789).
    pub fn trailer(action: TrailerAction) -> Self {
        Self {
            action: PrivateActionChoice::TrailerAction(action),
        }
    }

    /// The XSD element name of the selected branch.
    pub fn action_type(&self) -> &'static str {
        match &self.action {
            PrivateActionChoice::LongitudinalAction(_) => "LongitudinalAction",
            PrivateActionChoice::LateralAction(_) => "LateralAction",
            PrivateActionChoice::VisibilityAction(_) => "VisibilityAction",
            PrivateActionChoice::SynchronizeAction(_) => "SynchronizeAction",
            PrivateActionChoice::ActivateControllerAction(_) => "ActivateControllerAction",
            PrivateActionChoice::ControllerAction(_) => "ControllerAction",
            PrivateActionChoice::TeleportAction(_) => "TeleportAction",
            PrivateActionChoice::RoutingAction(_) => "RoutingAction",
            PrivateActionChoice::AppearanceAction(_) => "AppearanceAction",
            PrivateActionChoice::TrailerAction(_) => "TrailerAction",
        }
    }
}

/// Longitudinal movement actions (speed control and longitudinal distance keeping).
///
/// XSD `LongitudinalAction` (:1431-1437) is a bare `xsd:choice` of `SpeedAction |
/// LongitudinalDistanceAction | SpeedProfileAction`, with no `minOccurs` on the choice and
/// none on any branch, hence exactly one branch is required.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LongitudinalAction {
    #[serde(rename = "$value")]
    pub action: LongitudinalActionChoice,
}

/// The three branches of the XSD `LongitudinalAction` choice (:1432-1435), in schema order.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum LongitudinalActionChoice {
    SpeedAction(SpeedAction),
    LongitudinalDistanceAction(LongitudinalDistanceAction),
    SpeedProfileAction(SpeedProfileAction),
}

impl LongitudinalAction {
    /// `<SpeedAction>` branch of the XSD `LongitudinalAction` choice (:1432).
    pub fn speed(action: SpeedAction) -> Self {
        Self {
            action: LongitudinalActionChoice::SpeedAction(action),
        }
    }

    /// `<LongitudinalDistanceAction>` branch of the XSD `LongitudinalAction` choice (:1433).
    pub fn longitudinal_distance(action: LongitudinalDistanceAction) -> Self {
        Self {
            action: LongitudinalActionChoice::LongitudinalDistanceAction(action),
        }
    }

    /// `<SpeedProfileAction>` branch of the XSD `LongitudinalAction` choice (:1434).
    pub fn speed_profile(action: SpeedProfileAction) -> Self {
        Self {
            action: LongitudinalActionChoice::SpeedProfileAction(action),
        }
    }

    /// The XSD element name of the selected branch.
    pub fn action_type(&self) -> &'static str {
        match &self.action {
            LongitudinalActionChoice::SpeedAction(_) => "SpeedAction",
            LongitudinalActionChoice::LongitudinalDistanceAction(_) => "LongitudinalDistanceAction",
            LongitudinalActionChoice::SpeedProfileAction(_) => "SpeedProfileAction",
        }
    }
}

/// The `movement` module models the same XSD `LongitudinalAction` choice (:1431-1437) under
/// its own type. Converting between them is a branch-for-branch rename, and having it in one
/// place keeps the six builder call sites that need it from each re-deriving the mapping; the
/// hand-written versions they replace each ran three `match`es over the same value and
/// recovered the branch through `Option`.
impl From<crate::types::actions::movement::LongitudinalAction> for LongitudinalAction {
    fn from(action: crate::types::actions::movement::LongitudinalAction) -> Self {
        use crate::types::actions::movement::LongitudinalActionChoice as Movement;
        match action.longitudinal_action_choice {
            Movement::SpeedAction(a) => Self::speed(a),
            Movement::LongitudinalDistanceAction(a) => Self::longitudinal_distance(a),
            Movement::SpeedProfileAction(a) => Self::speed_profile(a),
        }
    }
}

impl Private {
    /// Create a new Private action container for the specified entity
    pub fn new(entity_ref: &str) -> Self {
        Self {
            entity_ref: crate::types::basic::Value::literal(entity_ref.to_string()),
            private_actions: Vec::new(),
        }
    }

    /// Add a private action to this entity's initialization
    pub fn add_action(mut self, action: PrivateAction) -> Self {
        self.private_actions.push(action);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::actions::movement::{
        SpeedActionTarget, SpeedProfileEntry, TransitionDynamics,
    };
    use crate::types::basic::Value;
    use crate::types::enums::{DynamicsDimension, DynamicsShape, FollowingMode};
    use crate::types::environment::{RoadCondition, TimeOfDay, Weather};

    #[test]
    fn test_init_creation() {
        let init = Init {
            actions: Actions {
                global_actions: vec![GlobalAction::environment(EnvironmentAction::environment(
                    Environment {
                        name: Value::literal("TestEnvironment".to_string()),
                        parameter_declarations: None,
                        time_of_day: None,
                        weather: None,
                        road_condition: None,
                    },
                ))],
                user_defined_actions: Vec::new(),
                private_actions: vec![Private::new("Ego")],
            },
        };

        assert_eq!(init.actions.global_actions.len(), 1);
        assert_eq!(init.actions.private_actions.len(), 1);
        assert_eq!(
            init.actions.private_actions[0]
                .entity_ref
                .as_literal()
                .unwrap(),
            "Ego"
        );
    }

    #[test]
    fn test_private_action_builder() {
        let private = Private::new("TestEntity")
            .add_action(PrivateAction::longitudinal(LongitudinalAction::speed(
                SpeedAction::new(
                    TransitionDynamics::new(DynamicsDimension::Time, DynamicsShape::Linear, 1.0),
                    SpeedActionTarget::absolute(10.0),
                ),
            )))
            .add_action(PrivateAction::teleport(TeleportAction::new(
                crate::types::positions::Position::world(
                    crate::types::positions::WorldPosition::new(1.0, 2.0),
                ),
            )));

        assert_eq!(private.entity_ref.as_literal().unwrap(), "TestEntity");
        assert_eq!(private.private_actions.len(), 2);
        assert_eq!(
            private.private_actions[0].action_type(),
            "LongitudinalAction"
        );
        assert_eq!(private.private_actions[1].action_type(), "TeleportAction");
    }

    #[test]
    fn test_init_default() {
        let init = Init::default();

        assert!(init.actions.global_actions.is_empty());
        assert!(init.actions.user_defined_actions.is_empty());
        assert!(init.actions.private_actions.is_empty());
    }

    #[test]
    fn test_actions_user_defined_action_round_trip() {
        // XSD `InitActions` (:1316-1321): sequence of GlobalAction*, UserDefinedAction*, Private*.
        let xml = r#"<Actions><UserDefinedAction><CustomCommandAction type="myCommand">payload</CustomCommandAction></UserDefinedAction></Actions>"#;
        let actions: Actions = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(actions.user_defined_actions.len(), 1);
        assert_eq!(
            actions.user_defined_actions[0]
                .custom_command_action
                .command_type
                .as_literal()
                .unwrap(),
            &"myCommand".to_string()
        );

        let serialized = quick_xml::se::to_string(&actions).unwrap();
        assert!(serialized.contains("UserDefinedAction"));
        assert!(serialized.contains("CustomCommandAction"));
        let reparsed: Actions = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(actions, reparsed);
    }

    #[test]
    fn test_environment_action_creation() {
        let env_action = EnvironmentAction::environment(Environment {
            name: Value::literal("TestEnvironment".to_string()),
            parameter_declarations: None,
            time_of_day: Some(TimeOfDay {
                animation: Value::literal(false),
                date_time: "2021-12-10T11:00:00".to_string(),
            }),
            weather: Some(Weather::default()),
            road_condition: Some(RoadCondition {
                friction_scale_factor: crate::types::basic::Double::literal(1.0),
                wetness: None,
                properties: None,
            }),
        });

        let EnvironmentActionChoice::Environment(environment) = &env_action.action else {
            panic!("expected Environment branch, got {:?}", env_action.action);
        };
        assert_eq!(environment.name.as_literal().unwrap(), "TestEnvironment");
        assert_eq!(
            environment.time_of_day.as_ref().unwrap().date_time,
            "2021-12-10T11:00:00"
        );
    }

    #[test]
    fn test_init_serialization() {
        let init = Init {
            actions: Actions {
                global_actions: vec![GlobalAction::environment(EnvironmentAction::environment(
                    Environment {
                        name: Value::literal("TestEnvironment".to_string()),
                        parameter_declarations: None,
                        time_of_day: None,
                        weather: None,
                        road_condition: None,
                    },
                ))],
                user_defined_actions: Vec::new(),
                private_actions: vec![Private::new("Ego")],
            },
        };

        let serialized = quick_xml::se::to_string(&init).unwrap();
        assert!(serialized.contains("<Actions"));
        assert!(serialized.contains("<GlobalAction"));
        assert!(serialized.contains("<Private"));
        assert!(serialized.contains("entityRef=\"Ego\""));
    }

    #[test]
    fn test_longitudinal_action_branch_selection() {
        // XSD `LongitudinalAction` (:1431-1437) is a bare `xsd:choice`, so the type names one
        // branch and cannot name none or two. The hand-written `validate()` this replaces
        // reported the same three cases at run time, after a value had already been built.
        let speed = LongitudinalAction::speed(SpeedAction::new(
            TransitionDynamics::new(DynamicsDimension::Time, DynamicsShape::Linear, 1.0),
            SpeedActionTarget::absolute(10.0),
        ));
        assert_eq!(speed.action_type(), "SpeedAction");

        let distance = LongitudinalAction::longitudinal_distance(
            LongitudinalDistanceAction::new("DefaultEntity", true, false).with_distance(10.0),
        );
        assert_eq!(distance.action_type(), "LongitudinalDistanceAction");

        let profile = LongitudinalAction::speed_profile(SpeedProfileAction::new(
            FollowingMode::Follow,
            vec![SpeedProfileEntry::new(10.0)],
        ));
        assert_eq!(profile.action_type(), "SpeedProfileAction");
    }

    #[test]
    fn test_private_action_branch_selection() {
        let longitudinal =
            PrivateAction::longitudinal(LongitudinalAction::speed(SpeedAction::new(
                TransitionDynamics::new(DynamicsDimension::Time, DynamicsShape::Linear, 1.0),
                SpeedActionTarget::absolute(10.0),
            )));
        assert_eq!(longitudinal.action_type(), "LongitudinalAction");

        let lateral =
            PrivateAction::lateral(crate::types::actions::movement::LateralAction::lane_change(
                crate::types::actions::movement::LaneChangeAction::new(
                    TransitionDynamics::new(DynamicsDimension::Time, DynamicsShape::Linear, 1.0),
                    crate::types::actions::movement::LaneChangeTarget::relative("Ego", -1),
                ),
            ));
        assert_eq!(lateral.action_type(), "LateralAction");
    }
}
