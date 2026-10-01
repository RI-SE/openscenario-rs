//! The named constructors of the action wrapper types emit the XSD wire form.
//!
//! Each row builds a value only through the public constructors and compares the
//! serialized bytes with the element the schema describes, so a constructor that fills
//! the wrong field, picks the wrong choice branch or drops a required attribute fails
//! here rather than in a downstream document.

use openscenario_rs::types::actions::traffic::TrafficSignalAction;
use openscenario_rs::types::actions::trailer::ConnectTrailerAction;
use openscenario_rs::types::actions::wrappers::{
    AddEntityAction, CustomCommandAction, InfrastructureAction, ModifyRule, NamedAction,
    ParameterAction, ParameterActionChoice, ParameterModifyAction, ParameterSetAction,
    PrivateAction, TrafficAction, TrafficActionChoice, UserDefinedAction, VariableAction,
    VariableActionChoice, VariableModifyAction, VariableModifyRule, VariableSetAction,
};
use openscenario_rs::types::actions::{TrafficStopAction, VisibilityAction};
use openscenario_rs::types::basic::OSString;
use openscenario_rs::types::positions::{Position, WorldPosition};

fn xml<T: serde::Serialize>(value: &T) -> String {
    quick_xml::se::to_string(value).expect("serialize failed")
}

/// `NamedAction` is written as the XSD `Action` element, whose name the type does not carry.
fn action_xml(value: &NamedAction) -> String {
    quick_xml::se::to_string_with_root("Action", value).expect("serialize failed")
}

#[test]
fn named_constructors_emit_the_xsd_wire_form() {
    let rows: Vec<(String, &str)> = vec![
        // XSD `VariableAction` (:2456-2462) / `VariableSetAction` (:2495-2497).
        (
            xml(&VariableAction::new(
                "v1",
                VariableActionChoice::VariableSetAction(VariableSetAction::new("42")),
            )),
            r#"<VariableAction variableRef="v1"><SetAction value="42"/></VariableAction>"#,
        ),
        // XSD `VariableModifyAction` (:2481-2485) / `VariableModifyRule` (:2486-2491).
        (
            xml(&VariableAction::new(
                "v2",
                VariableActionChoice::VariableModifyAction(VariableModifyAction::new(
                    VariableModifyRule::add_value(10.5),
                )),
            )),
            r#"<VariableAction variableRef="v2"><ModifyAction><Rule><AddValue value="10.5"/></Rule></ModifyAction></VariableAction>"#,
        ),
        (
            xml(&VariableAction::new(
                "v3",
                VariableActionChoice::VariableModifyAction(VariableModifyAction::new(
                    VariableModifyRule::multiply_by_value(2.0),
                )),
            )),
            r#"<VariableAction variableRef="v3"><ModifyAction><Rule><MultiplyByValue value="2"/></Rule></ModifyAction></VariableAction>"#,
        ),
        // XSD `ParameterAction` (:1604-1615) / `ParameterSetAction` (:1657-1660).
        (
            xml(&ParameterAction {
                parameter_ref: OSString::literal("p1".to_string()),
                action: ParameterActionChoice::ParameterSetAction(ParameterSetAction::new("100")),
            }),
            r#"<ParameterAction parameterRef="p1"><SetAction value="100"/></ParameterAction>"#,
        ),
        // XSD `ParameterModifyAction` (:1647-1652) / `ModifyRule` (:1490-1496).
        (
            xml(&ParameterAction {
                parameter_ref: OSString::literal("p2".to_string()),
                action: ParameterActionChoice::ParameterModifyAction(ParameterModifyAction::new(
                    ModifyRule::add_value(5.0),
                )),
            }),
            r#"<ParameterAction parameterRef="p2"><ModifyAction><Rule><AddValue value="5"/></Rule></ModifyAction></ParameterAction>"#,
        ),
        (
            xml(&ParameterAction {
                parameter_ref: OSString::literal("p3".to_string()),
                action: ParameterActionChoice::ParameterModifyAction(ParameterModifyAction::new(
                    ModifyRule::multiply_by_value(3.0),
                )),
            }),
            r#"<ParameterAction parameterRef="p3"><ModifyAction><Rule><MultiplyByValue value="3"/></Rule></ModifyAction></ParameterAction>"#,
        ),
        // XSD `UserDefinedAction` (:2416-2420) / `CustomCommandAction` (:1009-1015).
        (
            xml(&UserDefinedAction::new(CustomCommandAction::new(
                "myCommand",
                "payload",
            ))),
            r#"<UserDefinedAction><CustomCommandAction type="myCommand">payload</CustomCommandAction></UserDefinedAction>"#,
        ),
        // XSD `AddEntityAction` (:729-732): the required `Position` child.
        (
            xml(&AddEntityAction::new(Position::world(WorldPosition::new(
                1.0, 2.0,
            )))),
            r#"<AddEntityAction><Position><WorldPosition x="1" y="2"/></Position></AddEntityAction>"#,
        ),
        // XSD `TrafficAction` (:2204-2213): `@trafficName` is optional.
        (
            xml(&TrafficAction::new(TrafficActionChoice::TrafficStopAction(
                TrafficStopAction::default(),
            ))),
            r#"<TrafficAction><TrafficStopAction/></TrafficAction>"#,
        ),
        (
            xml(&TrafficAction::new(TrafficActionChoice::TrafficStopAction(
                TrafficStopAction::default(),
            ))
            .with_name("t1")),
            r#"<TrafficAction trafficName="t1"><TrafficStopAction/></TrafficAction>"#,
        ),
        // XSD `Action` (:705-712): required `@name` plus the action choice.
        (
            action_xml(&NamedAction::private(
                "a1",
                PrivateAction::VisibilityAction(VisibilityAction::new(true, false, true)),
            )),
            r#"<Action name="a1"><PrivateAction><VisibilityAction graphics="true" sensors="false" traffic="true"/></PrivateAction></Action>"#,
        ),
        (
            action_xml(&NamedAction::user_defined(
                "a2",
                UserDefinedAction::new(CustomCommandAction::new("cmd", "go")),
            )),
            r#"<Action name="a2"><UserDefinedAction><CustomCommandAction type="cmd">go</CustomCommandAction></UserDefinedAction></Action>"#,
        ),
        // XSD `InfrastructureAction` (:1306-1310) / `TrafficSignalAction` (:2248-2254) /
        // `TrafficSignalStateAction` (:2286-2289): `@name` is the signal id, `@state` its state.
        (
            xml(&InfrastructureAction::new(
                TrafficSignalAction::state_action("signal7".to_string(), "green".to_string()),
            )),
            r#"<InfrastructureAction><TrafficSignalAction><TrafficSignalStateAction name="signal7" state="green"/></TrafficSignalAction></InfrastructureAction>"#,
        ),
        // XSD `ConnectTrailerAction`: required `@trailerRef`.
        (
            xml(&ConnectTrailerAction::new("trailer1")),
            r#"<ConnectTrailerAction trailerRef="trailer1"/>"#,
        ),
    ];

    for (emitted, expected) in rows {
        assert_eq!(emitted, expected);
    }
}
