//! Action wrapper types matching OpenSCENARIO XSD schema structure
//!
//! This module contains the main wrapper types that organize individual actions
//! according to the OpenSCENARIO specification hierarchy.

use serde::{Deserialize, Serialize};
use crate::types::basic::OSString;
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

// Placeholder types for missing actions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CoreEnvironmentAction {
    // Will be implemented when environment types are available
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CoreSetMonitorAction {
    // Will be implemented when monitor types are available
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CoreParameterAction {
    // Deprecated action type
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CoreVariableAction {
    // Will be implemented when variable types are available
}

// Define Action wrapper with common attributes
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