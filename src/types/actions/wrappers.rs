//! Action wrapper types matching OpenSCENARIO XSD schema structure
//!
//! This module contains the main wrapper types that organize individual actions
//! according to the OpenSCENARIO specification hierarchy.

use crate::types::basic::{Boolean, Double, OSString};
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

// XSD `UserDefinedAction` (:2416-2420): sequence containing exactly one
// required `CustomCommandAction` element.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserDefinedAction {
    #[serde(rename = "CustomCommandAction")]
    pub custom_command_action: CustomCommandAction,
}

impl Default for UserDefinedAction {
    fn default() -> Self {
        Self {
            custom_command_action: CustomCommandAction::default(),
        }
    }
}

/// XSD `CustomCommandAction` (:1009-1015): `simpleContent` extending
/// `xsd:string` with a required `@type` attribute — the text body is not a
/// child element.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomCommandAction {
    #[serde(rename = "@type")]
    pub command_type: OSString,
    #[serde(rename = "$text", default)]
    pub content: String,
}

impl Default for CustomCommandAction {
    fn default() -> Self {
        Self {
            command_type: OSString::literal("default".to_string()),
            content: String::new(),
        }
    }
}

/// XSD `EnvironmentAction` (:1195-1200): choice of `Environment` |
/// `CatalogReference` (to a `CatalogEnvironment` catalog entry).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct EnvironmentAction {
    #[serde(
        rename = "Environment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub environment: Option<crate::types::environment::Environment>,
    #[serde(
        rename = "CatalogReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub catalog_reference: Option<
        crate::types::catalogs::references::CatalogReference<
            crate::types::catalogs::environments::CatalogEnvironment,
        >,
    >,
}

// Monitor Action - Set monitor state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SetMonitorAction {
    #[serde(rename = "@monitorRef")]
    pub monitor_ref: OSString,
    #[serde(rename = "@value")]
    pub value: Boolean,
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
    /// XSD element `<SetAction>` of type `VariableSetAction` (XSD:2458)
    #[serde(rename = "SetAction")]
    VariableSetAction(VariableSetAction),
    /// XSD element `<ModifyAction>` of type `VariableModifyAction` (XSD:2459)
    #[serde(rename = "ModifyAction")]
    VariableModifyAction(VariableModifyAction),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VariableSetAction {
    #[serde(rename = "@value")]
    pub value: OSString,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VariableModifyAction {
    /// XSD:2482 `<xsd:all><xsd:element name="Rule" type="VariableModifyRule"/></xsd:all>`
    #[serde(rename = "Rule")]
    pub rule: VariableModifyRule,
}

/// XSD `VariableModifyRule` (XSD:2486-2491) — wrapper for the `<Rule>` element content
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VariableModifyRule {
    #[serde(flatten)]
    pub rule: VariableModifyRuleChoice,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VariableModifyRuleChoice {
    /// XSD element `<AddValue>` of type `VariableAddValueRule` (XSD:2488)
    #[serde(rename = "AddValue")]
    VariableAddValueRule(VariableAddValueRule),
    /// XSD element `<MultiplyByValue>` of type `VariableMultiplyByValueRule` (XSD:2489)
    #[serde(rename = "MultiplyByValue")]
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
    /// XSD element `<SetAction>` of type `ParameterSetAction` (XSD:1609)
    #[serde(rename = "SetAction")]
    ParameterSetAction(ParameterSetAction),
    /// XSD element `<ModifyAction>` of type `ParameterModifyAction` (XSD:1612)
    #[serde(rename = "ModifyAction")]
    ParameterModifyAction(ParameterModifyAction),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParameterSetAction {
    #[serde(rename = "@value")]
    pub value: OSString,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParameterModifyAction {
    /// XSD:1649 `<xsd:all><xsd:element name="Rule" type="ModifyRule"/></xsd:all>`
    #[serde(rename = "Rule")]
    pub rule: ModifyRule,
}

/// XSD `ModifyRule` (XSD:1490-1496) — wrapper for the `<Rule>` element content
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModifyRule {
    #[serde(flatten)]
    pub rule: ModifyRuleChoice,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModifyRuleChoice {
    /// XSD element `<AddValue>` of type `ParameterAddValueRule` (XSD:1493)
    #[serde(rename = "AddValue")]
    ParameterAddValueRule(ParameterAddValueRule),
    /// XSD element `<MultiplyByValue>` of type `ParameterMultiplyByValueRule` (XSD:1494)
    #[serde(rename = "MultiplyByValue")]
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

// XSD `RandomRouteAction` (:1813-1814) is an empty complexType.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RandomRouteAction {}

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
            monitor_ref: OSString::literal("defaultMonitor".to_string()),
            value: Boolean::literal(true),
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
            rule: VariableModifyRule {
                rule: VariableModifyRuleChoice::VariableAddValueRule(
                    VariableAddValueRule::default(),
                ),
            },
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
            rule: ModifyRule {
                rule: ModifyRuleChoice::ParameterAddValueRule(ParameterAddValueRule::default()),
            },
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
        assert!(matches!(
            action,
            Action::PrivateAction(PrivateAction::TeleportAction(_))
        ));
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
        assert!(matches!(
            ea.action,
            EntityActionChoice::DeleteEntityAction(_)
        ));
    }

    #[test]
    fn test_variable_action_default() {
        let va = VariableAction::default();
        assert_eq!(va.variable_ref.as_literal().unwrap(), "defaultVariable");
        assert!(matches!(
            va.action,
            VariableActionChoice::VariableSetAction(_)
        ));
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
        assert_eq!(sma.value.as_literal().unwrap(), &true);
        assert_eq!(
            sma.monitor_ref.as_literal().unwrap(),
            &"defaultMonitor".to_string()
        );
    }

    #[test]
    fn test_set_monitor_action_roundtrip() {
        // XSD: required @monitorRef (String) and @value (Boolean).
        let xml = r#"<SetMonitorAction monitorRef="speedMonitor" value="true"/>"#;
        let action: SetMonitorAction = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(
            action.monitor_ref.as_literal().unwrap(),
            &"speedMonitor".to_string()
        );
        assert_eq!(action.value.as_literal().unwrap(), &true);
    }

    #[test]
    fn test_random_route_action_default() {
        let _rra = RandomRouteAction::default();
    }

    #[test]
    fn test_custom_command_action_round_trip() {
        // XSD: simpleContent extension of xsd:string with required @type attribute.
        let xml = r#"<CustomCommandAction type="myCommand">do something</CustomCommandAction>"#;
        let action: CustomCommandAction = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(
            action.command_type.as_literal().unwrap(),
            &"myCommand".to_string()
        );
        assert_eq!(action.content, "do something");

        let serialized = quick_xml::se::to_string(&action).unwrap();
        assert!(serialized.contains(r#"type="myCommand""#));
        assert!(serialized.contains("do something"));
        let reparsed: CustomCommandAction = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(action, reparsed);
    }

    #[test]
    fn test_user_defined_action_round_trip() {
        let xml = r#"<UserDefinedAction><CustomCommandAction type="myCommand">payload</CustomCommandAction></UserDefinedAction>"#;
        let action: UserDefinedAction = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(
            action
                .custom_command_action
                .command_type
                .as_literal()
                .unwrap(),
            &"myCommand".to_string()
        );

        let serialized = quick_xml::se::to_string(&action).unwrap();
        assert!(serialized.contains("CustomCommandAction"));
        let reparsed: UserDefinedAction = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(action, reparsed);
    }

    #[test]
    fn test_environment_action_with_environment_round_trip() {
        let xml = r#"<EnvironmentAction><Environment name="Env1"/></EnvironmentAction>"#;
        let action: EnvironmentAction = quick_xml::de::from_str(xml).unwrap();
        assert!(action.environment.is_some());
        assert!(action.catalog_reference.is_none());

        let serialized = quick_xml::se::to_string(&action).unwrap();
        assert!(serialized.contains("<Environment"));
        let reparsed: EnvironmentAction = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(action, reparsed);
    }

    #[test]
    fn test_environment_action_with_catalog_reference_round_trip() {
        let xml = r#"<EnvironmentAction><CatalogReference catalogName="EnvCatalog" entryName="Sunny"/></EnvironmentAction>"#;
        let action: EnvironmentAction = quick_xml::de::from_str(xml).unwrap();
        assert!(action.environment.is_none());
        assert!(action.catalog_reference.is_some());

        let serialized = quick_xml::se::to_string(&action).unwrap();
        assert!(serialized.contains("<CatalogReference"));
        let reparsed: EnvironmentAction = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(action, reparsed);
    }
}
