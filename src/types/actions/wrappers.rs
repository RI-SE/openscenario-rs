//! Action wrapper types matching OpenSCENARIO XSD schema structure
//!
//! This module contains the main wrapper types that organize individual actions
//! according to the OpenSCENARIO specification hierarchy.

use crate::types::basic::{Boolean, Double, OSString, UnsignedInt};
use crate::types::positions::Position;
use serde::{Deserialize, Serialize};

// Import individual action types
use super::{
    ActivateControllerAction, AppearanceAction, ControllerAction, LateralAction,
    LongitudinalAction, RoutingAction, SynchronizeAction, TeleportAction, TrafficAreaAction,
    TrafficSignalAction, TrafficSinkAction, TrafficSourceAction, TrafficStopAction,
    TrafficSwarmAction, TrailerAction, VisibilityAction,
};

// Main Action wrapper type matching XSD schema
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum Action {
    GlobalAction(GlobalAction),
    UserDefinedAction(UserDefinedAction),
    PrivateAction(PrivateAction),
}

// GlobalAction wrapper type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum GlobalAction {
    EnvironmentAction(EnvironmentAction),
    EntityAction(EntityAction),
    InfrastructureAction(InfrastructureAction),
    SetMonitorAction(SetMonitorAction),
    #[serde(rename = "ParameterAction")]
    ParameterAction(ParameterAction), // deprecated
    TrafficAction(TrafficAction),
    VariableAction(VariableAction),
}

// PrivateAction wrapper type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum PrivateAction {
    LongitudinalAction(LongitudinalAction),
    LateralAction(LateralAction),
    VisibilityAction(VisibilityAction),
    SynchronizeAction(SynchronizeAction),
    ActivateControllerAction(ActivateControllerAction),
    ControllerAction(ControllerAction),
    TeleportAction(TeleportAction),
    RoutingAction(RoutingAction),
    AppearanceAction(AppearanceAction),
    TrailerAction(TrailerAction),
}

// EntityAction wrapper type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EntityAction {
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,
    #[serde(flatten)]
    pub action: EntityActionChoice,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum EntityActionChoice {
    AddEntityAction(AddEntityAction),
    DeleteEntityAction(DeleteEntityAction),
}

// TrafficAction wrapper type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficAction {
    #[serde(rename = "@trafficName", skip_serializing_if = "Option::is_none")]
    pub traffic_name: Option<OSString>,
    #[serde(flatten)]
    pub action: TrafficActionChoice,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum TrafficActionChoice {
    TrafficSourceAction(TrafficSourceAction),
    TrafficSinkAction(TrafficSinkAction),
    TrafficSwarmAction(TrafficSwarmAction),
    TrafficAreaAction(TrafficAreaAction),
    TrafficStopAction(TrafficStopAction),
}

// InfrastructureAction wrapper type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub struct InfrastructureAction {
    pub traffic_signal_action: TrafficSignalAction,
}

// AddEntityAction type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub struct AddEntityAction {
    pub position: Position,
}

// DeleteEntityAction type (empty per XSD)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DeleteEntityAction {}

// UserDefinedAction placeholder
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[derive(Default)]
pub struct UserDefinedAction {
    #[serde(rename = "CustomCommandAction")]
    pub custom_command_action: CustomCommandAction,
}

// CustomCommandAction placeholder
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CustomCommandAction {
    // Implementation depends on specific use case
}

// Environment Action (placeholder for now)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct EnvironmentAction {
    // Will be implemented when environment types are available
}

// Monitor Action - Set monitor state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SetMonitorAction {
    #[serde(rename = "@enable")]
    pub enable: Boolean,
    #[serde(rename = "@monitorName", skip_serializing_if = "Option::is_none")]
    pub monitor_name: Option<OSString>,
}

// Variable Action System
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VariableAction {
    #[serde(rename = "@variableRef")]
    pub variable_ref: OSString,
    #[serde(flatten)]
    pub action: VariableActionChoice,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum VariableActionChoice {
    VariableSetAction(VariableSetAction),
    VariableModifyAction(VariableModifyAction),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VariableSetAction {
    #[serde(rename = "@value")]
    pub value: OSString,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VariableModifyAction {
    #[serde(flatten)]
    pub rule: VariableModifyRule,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum VariableModifyRule {
    VariableAddValueRule(VariableAddValueRule),
    VariableMultiplyByValueRule(VariableMultiplyByValueRule),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VariableAddValueRule {
    #[serde(rename = "@value")]
    pub value: Double,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VariableMultiplyByValueRule {
    #[serde(rename = "@value")]
    pub value: Double,
}

// Parameter Action System (deprecated but needed for compatibility)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParameterAction {
    #[serde(rename = "@parameterRef")]
    pub parameter_ref: OSString,
    #[serde(flatten)]
    pub action: ParameterActionChoice,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ParameterActionChoice {
    ParameterSetAction(ParameterSetAction),
    ParameterModifyAction(ParameterModifyAction),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParameterSetAction {
    #[serde(rename = "@value")]
    pub value: OSString,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParameterModifyAction {
    #[serde(flatten)]
    pub rule: ModifyRule,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ModifyRule {
    ParameterAddValueRule(ParameterAddValueRule),
    ParameterMultiplyByValueRule(ParameterMultiplyByValueRule),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParameterAddValueRule {
    #[serde(rename = "@value")]
    pub value: Double,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParameterMultiplyByValueRule {
    #[serde(rename = "@value")]
    pub value: Double,
}

// Named Action wrapper type matching XSD schema
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NamedAction {
    #[serde(rename = "@name")]
    pub name: OSString,
    #[serde(flatten)]
    pub action: Action,
}

// Additional action types for completeness
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[derive(Default)]
pub struct RandomRouteAction {
    #[serde(rename = "@numberOfRoutes", skip_serializing_if = "Option::is_none")]
    pub number_of_routes: Option<UnsignedInt>,
    #[serde(rename = "@randomSeed", skip_serializing_if = "Option::is_none")]
    pub random_seed: Option<UnsignedInt>,
}

// Default implementations
impl Default for Action {
    fn default() -> Self {
        Action::PrivateAction(PrivateAction::TeleportAction(TeleportAction::default()))
    }
}

impl Default for GlobalAction {
    fn default() -> Self {
        GlobalAction::TrafficAction(TrafficAction::default())
    }
}

impl Default for PrivateAction {
    fn default() -> Self {
        PrivateAction::TeleportAction(TeleportAction::default())
    }
}

impl Default for EntityAction {
    fn default() -> Self {
        EntityAction {
            entity_ref: OSString::literal("defaultEntity".to_string()),
            action: EntityActionChoice::DeleteEntityAction(DeleteEntityAction::default()),
        }
    }
}

impl Default for TrafficAction {
    fn default() -> Self {
        TrafficAction {
            traffic_name: None,
            action: TrafficActionChoice::TrafficStopAction(TrafficStopAction::default()),
        }
    }
}




impl Default for NamedAction {
    fn default() -> Self {
        NamedAction {
            name: OSString::literal("defaultTraffic".to_string()),
            action: Action::default(),
        }
    }
}

impl Default for SetMonitorAction {
    fn default() -> Self {
        SetMonitorAction {
            enable: Boolean::literal(true),
            monitor_name: None,
        }
    }
}

impl Default for VariableAction {
    fn default() -> Self {
        VariableAction {
            variable_ref: OSString::literal("defaultVariable".to_string()),
            action: VariableActionChoice::VariableSetAction(VariableSetAction::default()),
        }
    }
}

impl Default for VariableSetAction {
    fn default() -> Self {
        VariableSetAction {
            value: OSString::literal("0".to_string()),
        }
    }
}

impl Default for VariableModifyAction {
    fn default() -> Self {
        VariableModifyAction {
            rule: VariableModifyRule::VariableAddValueRule(VariableAddValueRule::default()),
        }
    }
}

impl Default for VariableAddValueRule {
    fn default() -> Self {
        VariableAddValueRule {
            value: Double::literal(0.0),
        }
    }
}

impl Default for VariableMultiplyByValueRule {
    fn default() -> Self {
        VariableMultiplyByValueRule {
            value: Double::literal(1.0),
        }
    }
}

impl Default for ParameterAction {
    fn default() -> Self {
        ParameterAction {
            parameter_ref: OSString::literal("defaultParameter".to_string()),
            action: ParameterActionChoice::ParameterSetAction(ParameterSetAction::default()),
        }
    }
}

impl Default for ParameterSetAction {
    fn default() -> Self {
        ParameterSetAction {
            value: OSString::literal("0".to_string()),
        }
    }
}

impl Default for ParameterModifyAction {
    fn default() -> Self {
        ParameterModifyAction {
            rule: ModifyRule::ParameterAddValueRule(ParameterAddValueRule::default()),
        }
    }
}

impl Default for ParameterAddValueRule {
    fn default() -> Self {
        ParameterAddValueRule {
            value: Double::literal(0.0),
        }
    }
}

impl Default for ParameterMultiplyByValueRule {
    fn default() -> Self {
        ParameterMultiplyByValueRule {
            value: Double::literal(1.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_default_is_private_teleport() {
        let action = Action::default();
        assert!(matches!(action, Action::PrivateAction(PrivateAction::TeleportAction(_))));
    }

    #[test]
    fn test_global_action_default_is_traffic() {
        let action = GlobalAction::default();
        assert!(matches!(action, GlobalAction::TrafficAction(_)));
    }

    #[test]
    fn test_entity_action_default() {
        let ea = EntityAction::default();
        assert_eq!(ea.entity_ref.as_literal().unwrap(), "defaultEntity");
        assert!(matches!(ea.action, EntityActionChoice::DeleteEntityAction(_)));
    }

    #[test]
    fn test_variable_action_default() {
        let va = VariableAction::default();
        assert_eq!(va.variable_ref.as_literal().unwrap(), "defaultVariable");
        assert!(matches!(va.action, VariableActionChoice::VariableSetAction(_)));
    }

    #[test]
    fn test_variable_multiply_default_value_is_one() {
        let rule = VariableMultiplyByValueRule::default();
        assert_eq!(rule.value.as_literal().unwrap(), &1.0);
    }

    #[test]
    fn test_named_action_default() {
        let na = NamedAction::default();
        assert_eq!(na.name.as_literal().unwrap(), "defaultTraffic");
    }

    #[test]
    fn test_delete_entity_action_xml_roundtrip() {
        let action = DeleteEntityAction::default();
        let xml = quick_xml::se::to_string(&action).unwrap();
        let deserialized: DeleteEntityAction = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(action, deserialized);
    }

    #[test]
    fn test_infrastructure_action_xml_roundtrip() {
        let action = InfrastructureAction::default();
        let xml = quick_xml::se::to_string(&action).unwrap();
        let deserialized: InfrastructureAction = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(action, deserialized);
    }

    #[test]
    fn test_set_monitor_action_default() {
        let sma = SetMonitorAction::default();
        assert_eq!(sma.enable.as_literal().unwrap(), &true);
        assert!(sma.monitor_name.is_none());
    }

    #[test]
    fn test_random_route_action_default_has_no_options() {
        let rra = RandomRouteAction::default();
        assert!(rra.number_of_routes.is_none());
        assert!(rra.random_seed.is_none());
    }
}

