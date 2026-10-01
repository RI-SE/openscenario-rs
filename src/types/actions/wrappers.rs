//! The wrapper types that hold the XSD action hierarchy: `Action` and its
//! global / user-defined / private branches, down to the individual action choices.

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
    EnvironmentAction(crate::types::scenario::init::EnvironmentAction),
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
//
// XSD `EntityAction` (:1128-1134): required `@entityRef` plus a choice of
// `AddEntityAction` | `DeleteEntityAction`. `$value` takes the element name
// from the serialized variant, so the branch is read from the live reader
// and coexists with the sibling attribute. `#[serde(flatten)]` cannot do
// that: it buffers the children into a map through `deserialize_any`, and a
// sequence replayed out of that buffer fails with `invalid type: map,
// expected a sequence` even when the element occurs once. This used to be
// worked around with a private parallel-`Option` wire representation joined
// by `#[serde(try_from/into)]`; `$value` gets the same exactly-one-branch
// guarantee structurally, so that representation and its hand-written
// `TryFrom` are no longer needed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EntityAction {
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,
    #[serde(rename = "$value")]
    pub action: EntityActionChoice,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum EntityActionChoice {
    AddEntityAction(AddEntityAction),
    DeleteEntityAction(DeleteEntityAction),
}

// TrafficAction wrapper type
//
// XSD `TrafficAction` (:2204-2212): optional `@trafficName` plus a choice of
// the five traffic actions. `$value` takes the element name from the
// serialized variant, so the branch is read from the live reader and
// coexists with the sibling attribute; `flatten` would buffer the children
// into a map first. `TrafficAreaAction` holds `RoadRange`, which requires
// `minOccurs="2"` `RoadCursor` children, so that branch was unreadable under
// `flatten` regardless of how many `RoadCursor` elements were present.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficAction {
    #[serde(
        rename = "@trafficName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub traffic_name: Option<OSString>,
    #[serde(rename = "$value")]
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
//
// `#[derive(Default)]` removed: it required
// `TrafficSignalAction: Default`, which fabricated a choice — the removed
// impl silently picked the `TrafficSignalStateAction` branch. Construct the
// field explicitly instead.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct InfrastructureAction {
    pub traffic_signal_action: TrafficSignalAction,
}

impl InfrastructureAction {
    pub fn new(traffic_signal_action: TrafficSignalAction) -> Self {
        Self {
            traffic_signal_action,
        }
    }
}

// AddEntityAction type
//
// `#[derive(Default)]` removed. A derive on a struct with a required, non-`Option` field
// fabricates that field silently, which a grep for `impl Default` cannot see; this was one
// such case. XSD `AddEntityAction`
// (`Schema/OpenSCENARIO.xsd:729-732`) requires the `Position` child, and XSD `Position`
// (`:1738-1751`) is a bare `xsd:choice` — category 3, schema-invalid empty.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct AddEntityAction {
    pub position: Position,
}

impl AddEntityAction {
    /// Add an entity at the given position (XSD `AddEntityAction`, `:729-732`).
    pub fn new(position: Position) -> Self {
        Self { position }
    }
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

impl UserDefinedAction {
    /// XSD `UserDefinedAction` (:2416-2420): the required `CustomCommandAction` child.
    pub fn new(custom_command_action: CustomCommandAction) -> Self {
        Self {
            custom_command_action,
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

impl CustomCommandAction {
    /// XSD `CustomCommandAction` (:1009-1015): required `@type` attribute plus
    /// the string content.
    pub fn new(command_type: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            command_type: OSString::literal(command_type.into()),
            content: content.into(),
        }
    }
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
//
// XSD `VariableAction` (:2456-2461) is a required `@variableRef` attribute
// plus a choice of `SetAction` | `ModifyAction`. `$value` reads the branch
// from the live reader by element name and coexists with the sibling
// attribute; `flatten` would buffer the children into a map first, which
// breaks the moment a sequence appears below the choice.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VariableAction {
    #[serde(rename = "@variableRef")]
    pub variable_ref: OSString,
    #[serde(rename = "$value")]
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

/// XSD `VariableModifyRule` (XSD:2486-2491) — wrapper for the `<Rule>` element content.
///
/// The choice below is read by `$value`; `flatten` would buffer the children
/// into a map first, which breaks the moment a sequence appears below it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VariableModifyRule {
    #[serde(rename = "$value")]
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
//
// XSD `ParameterAction` (:1604-1614) is a required `@parameterRef` attribute
// plus a choice of `SetAction` | `ModifyAction`. `$value` reads the branch
// from the live reader by element name and coexists with the sibling
// attribute; `flatten` would buffer the children into a map first, which
// breaks the moment a sequence appears below the choice.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParameterAction {
    #[serde(rename = "@parameterRef")]
    pub parameter_ref: OSString,
    #[serde(rename = "$value")]
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

/// XSD `ModifyRule` (XSD:1490-1496) — wrapper for the `<Rule>` element content.
///
/// The choice below is read by `$value`; `flatten` would buffer the children
/// into a map first, which breaks the moment a sequence appears below it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModifyRule {
    #[serde(rename = "$value")]
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

/// Element wrapper hosting the `GlobalAction` choice as a named child element.
///
/// XSD `GlobalAction` (:1282-1293) is a choice, modeled as the externally-tagged
/// enum `GlobalAction` above. As a *named child element* the choice has to sit
/// behind a wrapper struct — the same shape `StoryGlobalAction`
/// (`scenario/story.rs`) and `RoutePosition.RouteRefElement` already use.
/// `$value` takes the element name from the serialized variant, so the
/// branch is read from the live reader; the private parallel-`Option` wire
/// representation this used to hold reconstructed the same exactly-one-branch
/// guarantee by hand and is no longer needed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GlobalActionElement {
    #[serde(rename = "$value")]
    pub action: GlobalAction,
}

/// Element wrapper hosting the `PrivateAction` choice as a named child element.
///
/// XSD `PrivateAction` (:1777-1791). Same shape as `GlobalActionElement`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrivateActionElement {
    #[serde(rename = "$value")]
    pub action: PrivateAction,
}

/// XSD `Action` (:705-712): `@name` (required) plus
/// `xsd:choice(GlobalAction | UserDefinedAction | PrivateAction)`.
///
/// This used to be `#[serde(flatten)] action: Action`. Flattening an
/// externally-tagged enum whose *variant payloads are themselves* externally-tagged
/// enums is not serializable by quick-xml — `GlobalAction` and `PrivateAction`
/// deserialized fine and then failed to write with
/// `Unsupported("cannot serialize enum newtype variant ...")`. The group is a bare
/// `xsd:choice`, so `minOccurs`/`maxOccurs` both default to 1 and exactly one branch
/// is required: a bare `$value` states that directly, and serde rejects a document
/// naming no branch with `missing field $value` and one naming two with
/// `duplicate field $value`, structurally, with no hand-written validation. The
/// nesting problem above is why the payload is an enum over element wrapper structs
/// (`GlobalActionElement`, `PrivateActionElement`) rather than over `GlobalAction`
/// and `PrivateAction` directly — each wrapper holds its own `$value`, so each level
/// writes exactly one element name. `StoryAction` (`scenario/story.rs`) models the
/// same XSD type the same way.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NamedAction {
    #[serde(rename = "@name")]
    pub name: OSString,

    #[serde(rename = "$value")]
    pub action: NamedActionChoice,
}

/// The three branches of the XSD `Action` choice (:705-712).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum NamedActionChoice {
    GlobalAction(GlobalActionElement),
    UserDefinedAction(UserDefinedAction),
    PrivateAction(PrivateActionElement),
}

impl NamedAction {
    /// `<GlobalAction>` branch of the XSD `Action` choice (:705-712).
    pub fn global(name: impl Into<String>, action: GlobalAction) -> Self {
        NamedAction {
            name: OSString::literal(name.into()),
            action: NamedActionChoice::GlobalAction(GlobalActionElement { action }),
        }
    }

    /// `<UserDefinedAction>` branch of the XSD `Action` choice (:705-712).
    pub fn user_defined(name: impl Into<String>, action: UserDefinedAction) -> Self {
        NamedAction {
            name: OSString::literal(name.into()),
            action: NamedActionChoice::UserDefinedAction(action),
        }
    }

    /// `<PrivateAction>` branch of the XSD `Action` choice (:705-712).
    pub fn private(name: impl Into<String>, action: PrivateAction) -> Self {
        NamedAction {
            name: OSString::literal(name.into()),
            action: NamedActionChoice::PrivateAction(PrivateActionElement { action }),
        }
    }
}

// XSD `RandomRouteAction` (:1813-1814) is an empty complexType.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RandomRouteAction {}

// `Action`, `GlobalAction` and `PrivateAction` no longer
// implement `Default`: each is an externally-tagged `xsd:choice` (XSD:705-712,
// 1282-1293, 1777-1786) and a `Default` silently picked one branch
// (`PrivateAction::TeleportAction`, `TrafficAction`). None of the three has a
// schema-declared default. Construct the chosen variant directly, e.g.
// `Action::PrivateAction(PrivateAction::TeleportAction(TeleportAction::new(..)))`
// — the enum variants are the constructors.

impl EntityAction {
    /// `<AddEntityAction>` branch of the `EntityAction` choice (XSD:1128-1132).
    pub fn add(entity_ref: impl Into<String>, position: Position) -> Self {
        EntityAction {
            entity_ref: OSString::literal(entity_ref.into()),
            action: EntityActionChoice::AddEntityAction(AddEntityAction::new(position)),
        }
    }

    /// `<DeleteEntityAction>` branch of the `EntityAction` choice (XSD:1128-1132).
    pub fn delete(entity_ref: impl Into<String>) -> Self {
        EntityAction {
            entity_ref: OSString::literal(entity_ref.into()),
            action: EntityActionChoice::DeleteEntityAction(DeleteEntityAction::default()),
        }
    }
}

impl TrafficAction {
    /// XSD `TrafficAction` (:2204-2213): optional `@trafficName` plus the
    /// required action choice.
    pub fn new(action: TrafficActionChoice) -> Self {
        TrafficAction {
            traffic_name: None,
            action,
        }
    }

    /// Attach the optional `@trafficName`.
    pub fn with_name(mut self, traffic_name: impl Into<String>) -> Self {
        self.traffic_name = Some(OSString::literal(traffic_name.into()));
        self
    }
}

impl SetMonitorAction {
    /// XSD `SetMonitorAction` (:2027-2030): both attributes are required.
    pub fn new(monitor_ref: impl Into<String>, value: bool) -> Self {
        SetMonitorAction {
            monitor_ref: OSString::literal(monitor_ref.into()),
            value: Boolean::literal(value),
        }
    }
}

impl VariableAction {
    /// XSD `VariableAction` (deprecated group): required `@variableRef` plus
    /// the required action choice.
    pub fn new(variable_ref: impl Into<String>, action: VariableActionChoice) -> Self {
        VariableAction {
            variable_ref: OSString::literal(variable_ref.into()),
            action,
        }
    }
}

impl VariableSetAction {
    /// XSD `VariableSetAction` (:2495-2497): required `@value`.
    pub fn new(value: impl Into<String>) -> Self {
        VariableSetAction {
            value: OSString::literal(value.into()),
        }
    }
}

impl VariableModifyRule {
    /// `<AddValue>` branch of `VariableModifyRule` (XSD:2486-2491).
    pub fn add_value(value: f64) -> Self {
        VariableModifyRule {
            rule: VariableModifyRuleChoice::VariableAddValueRule(VariableAddValueRule::new(value)),
        }
    }

    /// `<MultiplyByValue>` branch of `VariableModifyRule` (XSD:2486-2491).
    pub fn multiply_by_value(value: f64) -> Self {
        VariableModifyRule {
            rule: VariableModifyRuleChoice::VariableMultiplyByValueRule(
                VariableMultiplyByValueRule::new(value),
            ),
        }
    }
}

impl VariableModifyAction {
    /// XSD `VariableModifyAction` (:2481-2485): required `<Rule>` child.
    pub fn new(rule: VariableModifyRule) -> Self {
        VariableModifyAction { rule }
    }
}

impl VariableAddValueRule {
    /// XSD `VariableAddValueRule` (:2463-2465): required `@value`.
    pub fn new(value: f64) -> Self {
        VariableAddValueRule {
            value: Double::literal(value),
        }
    }
}

impl VariableMultiplyByValueRule {
    /// XSD `VariableMultiplyByValueRule` (:2492-2494): required `@value`.
    pub fn new(value: f64) -> Self {
        VariableMultiplyByValueRule {
            value: Double::literal(value),
        }
    }
}

impl ParameterAction {
    /// XSD `ParameterAction` (deprecated group): required `@parameterRef`
    /// plus the required action choice.
    pub fn new(parameter_ref: impl Into<String>, action: ParameterActionChoice) -> Self {
        ParameterAction {
            parameter_ref: OSString::literal(parameter_ref.into()),
            action,
        }
    }
}

impl ParameterSetAction {
    /// XSD `ParameterSetAction` (:1657-1660): required `@value`.
    pub fn new(value: impl Into<String>) -> Self {
        ParameterSetAction {
            value: OSString::literal(value.into()),
        }
    }
}

impl ModifyRule {
    /// `<AddValue>` branch of `ModifyRule` (XSD:1490-1496).
    pub fn add_value(value: f64) -> Self {
        ModifyRule {
            rule: ModifyRuleChoice::ParameterAddValueRule(ParameterAddValueRule::new(value)),
        }
    }

    /// `<MultiplyByValue>` branch of `ModifyRule` (XSD:1490-1496).
    pub fn multiply_by_value(value: f64) -> Self {
        ModifyRule {
            rule: ModifyRuleChoice::ParameterMultiplyByValueRule(
                ParameterMultiplyByValueRule::new(value),
            ),
        }
    }
}

impl ParameterModifyAction {
    /// XSD `ParameterModifyAction` (:1647-1652): required `<Rule>` child.
    pub fn new(rule: ModifyRule) -> Self {
        ParameterModifyAction { rule }
    }
}

impl ParameterAddValueRule {
    /// XSD `ParameterAddValueRule` (:1616-1619): required `@value`.
    pub fn new(value: f64) -> Self {
        ParameterAddValueRule {
            value: Double::literal(value),
        }
    }
}

impl ParameterMultiplyByValueRule {
    /// XSD `ParameterMultiplyByValueRule` (:1653-1656): required `@value`.
    pub fn new(value: f64) -> Self {
        ParameterMultiplyByValueRule {
            value: Double::literal(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_action_delete_constructor() {
        let ea = EntityAction::delete("defaultEntity");
        assert_eq!(ea.entity_ref.as_literal().unwrap(), "defaultEntity");
        assert!(matches!(
            ea.action,
            EntityActionChoice::DeleteEntityAction(_)
        ));
    }

    #[test]
    fn test_entity_action_add_constructor() {
        let ea = EntityAction::add("newEntity", Position::world_origin());
        assert_eq!(ea.entity_ref.as_literal().unwrap(), "newEntity");
        assert!(matches!(ea.action, EntityActionChoice::AddEntityAction(_)));
    }

    #[test]
    fn test_variable_action_new_constructor() {
        let va = VariableAction::new(
            "defaultVariable",
            VariableActionChoice::VariableSetAction(VariableSetAction::new("0")),
        );
        assert_eq!(va.variable_ref.as_literal().unwrap(), "defaultVariable");
        assert!(matches!(
            va.action,
            VariableActionChoice::VariableSetAction(_)
        ));
    }

    #[test]
    fn test_variable_multiply_new_value() {
        let rule = VariableMultiplyByValueRule::new(1.0);
        assert_eq!(rule.value.as_literal().unwrap(), &1.0);
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

    // `EnvironmentAction` used to be duplicated here; it is now only
    // `crate::types::scenario::init::EnvironmentAction`, which carries its own
    // round-trip and cardinality tests in `scenario/init.rs`.
}
