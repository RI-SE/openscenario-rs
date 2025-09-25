//! Action wrapper types matching OpenSCENARIO XSD schema structure
//!
//! This module contains the main wrapper types that organize individual actions
//! according to the OpenSCENARIO specification hierarchy.

use serde::{Deserialize, Serialize};
use crate::types::basic::{OSString, Double, Boolean, Value, UnsignedInt};
use crate::types::positions::Position;

// Import individual action types
use super::{
    ActivateControllerAction, AppearanceAction, ControllerAction, LateralAction,
    LongitudinalAction, RoutingAction, SynchronizeAction, TeleportAction, TrailerAction,
    TrafficAreaAction, TrafficSignalAction, TrafficSinkAction, TrafficSourceAction,
    TrafficStopAction, TrafficSwarmAction, VisibilityAction,
};

// Main Action wrapper type matching XSD schema
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum CoreAction {
    GlobalAction(CoreGlobalAction),
    UserDefinedAction(CoreUserDefinedAction),
    PrivateAction(CorePrivateAction),
}

// GlobalAction wrapper type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum CoreGlobalAction {
    EnvironmentAction(CoreEnvironmentAction),
    EntityAction(CoreEntityAction),
    InfrastructureAction(CoreInfrastructureAction),
    SetMonitorAction(CoreSetMonitorAction),
    #[serde(rename = "ParameterAction")]
    ParameterAction(CoreParameterAction), // deprecated
    TrafficAction(CoreTrafficAction),
    VariableAction(CoreVariableAction),
}

// PrivateAction wrapper type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum CorePrivateAction {
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
pub struct CoreEntityAction {
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,
    #[serde(flatten)]
    pub action: CoreEntityActionChoice,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum CoreEntityActionChoice {
    AddEntityAction(CoreAddEntityAction),
    DeleteEntityAction(CoreDeleteEntityAction),
}

// TrafficAction wrapper type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CoreTrafficAction {
    #[serde(rename = "@trafficName", skip_serializing_if = "Option::is_none")]
    pub traffic_name: Option<OSString>,
    #[serde(flatten)]
    pub action: CoreTrafficActionChoice,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum CoreTrafficActionChoice {
    TrafficSourceAction(TrafficSourceAction),
    TrafficSinkAction(TrafficSinkAction),
    TrafficSwarmAction(TrafficSwarmAction),
    TrafficAreaAction(TrafficAreaAction),
    TrafficStopAction(TrafficStopAction),
}

// InfrastructureAction wrapper type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CoreInfrastructureAction {
    pub traffic_signal_action: TrafficSignalAction,
}

// AddEntityAction type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CoreAddEntityAction {
    pub position: Position,
}

// DeleteEntityAction type (empty per XSD)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CoreDeleteEntityAction {}

// UserDefinedAction placeholder
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CoreUserDefinedAction {
    #[serde(rename = "CustomCommandAction")]
    pub custom_command_action: CoreCustomCommandAction,
}

// CustomCommandAction placeholder
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CoreCustomCommandAction {
    // Implementation depends on specific use case
}

// Environment Action (placeholder for now)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CoreEnvironmentAction {
    // Will be implemented when environment types are available
}

// Monitor Action - Set monitor state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CoreSetMonitorAction {
    #[serde(rename = "@enable")]
    pub enable: Boolean,
    #[serde(rename = "@monitorName", skip_serializing_if = "Option::is_none")]
    pub monitor_name: Option<OSString>,
}

// Variable Action System
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CoreVariableAction {
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
pub struct CoreParameterAction {
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

// Main Action wrapper type matching XSD schema
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Action {
    #[serde(rename = "@name")]
    pub name: OSString,
    #[serde(flatten)]
    pub action: CoreAction,
}

// PrivateAction wrapper type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrivateAction {
    #[serde(flatten)]
    pub action: CorePrivateAction,
}

// EntityAction wrapper type (already exists as CoreEntityAction)
pub type EntityAction = CoreEntityAction;

// InfrastructureAction wrapper type (already exists as CoreInfrastructureAction)  
pub type InfrastructureAction = CoreInfrastructureAction;

// UserDefinedAction wrapper type (already exists as CoreUserDefinedAction)
pub type UserDefinedAction = CoreUserDefinedAction;

// VariableAction wrapper type (already exists as CoreVariableAction)
pub type VariableAction = CoreVariableAction;

// ParameterAction wrapper type (already exists as CoreParameterAction)
pub type ParameterAction = CoreParameterAction;

// SetMonitorAction wrapper type (already exists as CoreSetMonitorAction)
pub type SetMonitorAction = CoreSetMonitorAction;

// TrafficAction wrapper type (already exists as CoreTrafficAction)
pub type TrafficAction = CoreTrafficAction;

// Additional action types for completeness
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RandomRouteAction {
    #[serde(rename = "@numberOfRoutes", skip_serializing_if = "Option::is_none")]
    pub number_of_routes: Option<UnsignedInt>,
    #[serde(rename = "@randomSeed", skip_serializing_if = "Option::is_none")]
    pub random_seed: Option<UnsignedInt>,
}

// Define Action wrapper with common attributes (legacy name for compatibility)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreActionWrapper {
    #[serde(rename = "@name")]
    pub name: OSString,
    #[serde(flatten)]
    pub action: CoreAction,
}

// Default implementations
impl Default for CoreAction {
    fn default() -> Self {
        CoreAction::PrivateAction(CorePrivateAction::TeleportAction(TeleportAction::default()))
    }
}

impl Default for CoreGlobalAction {
    fn default() -> Self {
        CoreGlobalAction::TrafficAction(CoreTrafficAction::default())
    }
}

impl Default for CorePrivateAction {
    fn default() -> Self {
        CorePrivateAction::TeleportAction(TeleportAction::default())
    }
}

impl Default for CoreEntityAction {
    fn default() -> Self {
        CoreEntityAction {
            entity_ref: OSString::literal("defaultEntity".to_string()),
            action: CoreEntityActionChoice::DeleteEntityAction(CoreDeleteEntityAction::default()),
        }
    }
}

impl Default for CoreTrafficAction {
    fn default() -> Self {
        CoreTrafficAction {
            traffic_name: None,
            action: CoreTrafficActionChoice::TrafficStopAction(TrafficStopAction::default()),
        }
    }
}

impl Default for CoreInfrastructureAction {
    fn default() -> Self {
        CoreInfrastructureAction {
            traffic_signal_action: TrafficSignalAction::default(),
        }
    }
}

impl Default for CoreAddEntityAction {
    fn default() -> Self {
        CoreAddEntityAction {
            position: Position::default(),
        }
    }
}

impl Default for CoreUserDefinedAction {
    fn default() -> Self {
        CoreUserDefinedAction {
            custom_command_action: CoreCustomCommandAction::default(),
        }
    }
}

impl Default for CoreActionWrapper {
    fn default() -> Self {
        CoreActionWrapper {
            name: OSString::literal("defaultTraffic".to_string()),
            action: CoreAction::default(),
        }
    }
}

impl Default for Action {
    fn default() -> Self {
        Action {
            name: OSString::literal("defaultAction".to_string()),
            action: CoreAction::default(),
        }
    }
}

impl Default for PrivateAction {
    fn default() -> Self {
        PrivateAction {
            action: CorePrivateAction::default(),
        }
    }
}

impl Default for CoreSetMonitorAction {
    fn default() -> Self {
        CoreSetMonitorAction {
            enable: Boolean::literal(true),
            monitor_name: None,
        }
    }
}

impl Default for CoreVariableAction {
    fn default() -> Self {
        CoreVariableAction {
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

impl Default for CoreParameterAction {
    fn default() -> Self {
        CoreParameterAction {
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

impl Default for RandomRouteAction {
    fn default() -> Self {
        RandomRouteAction {
            number_of_routes: None,
            random_seed: None,
        }
    }
}