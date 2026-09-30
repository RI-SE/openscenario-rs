//! Sequences below a `#[serde(rename = "$value")]` choice wrapper.
//!
//! `#[serde(flatten)]` makes serde buffer an element's children into a `Content`
//! map through `deserialize_any`. quick-xml cannot know at that point that a
//! child will later be read back as a sequence, so a `Vec<T>` replayed out of
//! the buffer fails with `invalid type: map, expected a sequence` — even when
//! the element occurs only once.
//!
//! `src/types/actions/wrappers.rs` therefore hosts each choice behind
//! `#[serde(rename = "$value")]` instead: the externally-tagged enum stays
//! public, and quick-xml reads the branch from the live reader by element
//! name rather than through a buffered map. Serde enforces "exactly one
//! branch" structurally this way, so no hand-written `validate()` is needed.
//!
//! The cases in sections 1 to 5 fail with `invalid type: map, expected a sequence`
//! when using `#[serde(flatten)]`, except where marked as a control. The wrappers
//! in sections 6 and 7 never hit that failure, because nothing in their schema
//! subtree is repeatable; however, they did accept a document carrying two branches
//! and keep whichever came first under `flatten`. Their rejection cases verify
//! that, and fail under `flatten` by parsing successfully.
//!
//! The byte-exact round-trip fixtures are written in the order the crate emits.
//! `TrafficAreaAction` and `FollowTrajectoryAction` are both `xsd:all`
//! (`Schema/OpenSCENARIO.xsd:2220-2227` and `:1244-1257`), so attribute and
//! child order carry no meaning there and any order parses; the crate
//! normalises to its struct field order, which is what these strings use.

use openscenario_rs::types::actions::movement::{
    FinalSpeed, LaneChangeTarget, LaneOffsetTarget, LateralAction,
};
use openscenario_rs::types::actions::traffic::{TrafficAreaChoice, TrafficSignalAction};
use openscenario_rs::types::actions::wrappers::{
    EntityAction, EntityActionChoice, GlobalAction, GlobalActionElement, ModifyRule,
    ParameterAction, PrivateAction, PrivateActionElement, TrafficAction, TrafficActionChoice,
    VariableAction, VariableModifyRule,
};

fn de<T: serde::de::DeserializeOwned>(xml: &str) -> T {
    quick_xml::de::from_str(xml).unwrap_or_else(|e| panic!("deserialize failed for {xml}: {e}"))
}

fn de_err<T: serde::de::DeserializeOwned>(xml: &str) -> String {
    match quick_xml::de::from_str::<T>(xml) {
        Ok(_) => panic!("expected deserialize to fail for {xml}"),
        Err(e) => e.to_string(),
    }
}

// ── A Vec-bearing payload reachable from a Position, used by several cases ──
const TRAJECTORY_POSITION: &str = concat!(
    r#"<Position><TrajectoryPosition s="0"><TrajectoryRef>"#,
    r#"<Trajectory name="T" closed="false"><Shape><Polyline>"#,
    r#"<Vertex time="0"><Position><WorldPosition x="0" y="0"/></Position></Vertex>"#,
    r#"<Vertex time="1"><Position><WorldPosition x="1" y="1"/></Position></Vertex>"#,
    r#"</Polyline></Shape></Trajectory></TrajectoryRef></TrajectoryPosition></Position>"#
);

const TRAFFIC_AREA_ACTION: &str = concat!(
    r#"<TrafficAction><TrafficAreaAction numberOfEntities="100" continuous="true">"#,
    r#"<TrafficDistribution><TrafficDistributionEntry weight="1"><EntityDistribution>"#,
    r#"<EntityDistributionEntry weight="1"><ScenarioObjectTemplate>"#,
    r#"<CatalogReference catalogName="c" entryName="e"/>"#,
    r#"</ScenarioObjectTemplate></EntityDistributionEntry></EntityDistribution>"#,
    r#"</TrafficDistributionEntry></TrafficDistribution>"#,
    r#"<TrafficArea><RoadRange><RoadCursor roadId="R" s="100"/>"#,
    r#"<RoadCursor roadId="R" s="500"/></RoadRange></TrafficArea>"#,
    r#"</TrafficAreaAction></TrafficAction>"#
);

// ═══ 1. TrafficAction — the reported defect ══════════════════════════════════════════════
//
// Variants of the bisect that found the defect, reproduced as assertions. Before
// conversion to `$value`, A / C / D / F failed and B / E passed. C (a one-element
// sequence, `RoadRange` here) is asserted by A; B (`TrafficStopAction`) and F
// (`@trafficName`) by `choice_wrappers_roundtrip_test.rs::traffic_action_stop_round_trip`
// and `traffic_action_with_name_round_trips_byte_for_byte` below.

#[test]
fn traffic_action_variant_a_original_payload_parses() {
    let action: TrafficAction = de(TRAFFIC_AREA_ACTION);
    assert!(action.traffic_name.is_none());
    let TrafficActionChoice::TrafficAreaAction(area) = &action.action else {
        panic!("expected TrafficAreaAction, got {:?}", action.action);
    };
    // `minOccurs="2"` RoadCursor children survived the parse.
    let TrafficAreaChoice::RoadRange(road_ranges) = &area.traffic_area.choice else {
        panic!("expected RoadRange, got {:?}", area.traffic_area.choice);
    };
    assert_eq!(road_ranges.len(), 1);
    assert_eq!(road_ranges[0].road_cursor.len(), 2);
    assert_eq!(
        area.traffic_distribution.traffic_distribution_entry.len(),
        1
    );
}

#[test]
fn traffic_action_variant_d_traffic_source_action_parses() {
    let xml = concat!(
        r#"<TrafficAction><TrafficSourceAction radius="10" rate="2">"#,
        r#"<Position><WorldPosition x="0" y="0" z="0"/></Position>"#,
        r#"<TrafficDefinition name="td">"#,
        r#"<VehicleCategoryDistribution><VehicleCategoryDistributionEntry category="car" weight="1"/></VehicleCategoryDistribution>"#,
        r#"<ControllerDistribution><ControllerDistributionEntry weight="1">"#,
        r#"<CatalogReference catalogName="c" entryName="e"/>"#,
        r#"</ControllerDistributionEntry></ControllerDistribution>"#,
        r#"</TrafficDefinition></TrafficSourceAction></TrafficAction>"#
    );
    let action: TrafficAction = de(xml);
    let TrafficActionChoice::TrafficSourceAction(source) = &action.action else {
        panic!("expected TrafficSourceAction");
    };
    let definition = source
        .traffic_definition
        .as_ref()
        .expect("TrafficDefinition dropped");
    assert_eq!(definition.vehicle_category_distribution.entries.len(), 1);
}

#[test]
fn traffic_action_variant_e_no_sequence_field_parses() {
    // Control: passed both before and after conversion to `$value`.
    let xml = concat!(
        r#"<TrafficAction><TrafficSinkAction radius="10">"#,
        r#"<Position><WorldPosition x="0" y="0" z="0"/></Position>"#,
        r#"</TrafficSinkAction></TrafficAction>"#
    );
    let action: TrafficAction = de(xml);
    assert!(matches!(
        action.action,
        TrafficActionChoice::TrafficSinkAction(_)
    ));
}

#[test]
fn traffic_action_round_trips_byte_for_byte() {
    let action: TrafficAction = de(TRAFFIC_AREA_ACTION);
    let out = quick_xml::se::to_string_with_root("TrafficAction", &action)
        .expect("TrafficAction failed to serialize");
    assert_eq!(out, TRAFFIC_AREA_ACTION);
}

#[test]
fn traffic_action_with_name_round_trips_byte_for_byte() {
    let xml = TRAFFIC_AREA_ACTION.replace("<TrafficAction>", r#"<TrafficAction trafficName="t">"#);
    let action: TrafficAction = de(&xml);
    assert_eq!(action.traffic_name.as_ref().unwrap().to_string(), "t");
    let out = quick_xml::se::to_string_with_root("TrafficAction", &action)
        .expect("TrafficAction failed to serialize");
    assert_eq!(out, xml);
}

// ═══ 2. EntityAction — same shape, Vec reachable through Position ══════════

#[test]
fn entity_action_with_sequence_below_position_parses() {
    let xml = format!(
        r#"<EntityAction entityRef="e"><AddEntityAction>{TRAJECTORY_POSITION}</AddEntityAction></EntityAction>"#
    );
    let action: EntityAction = de(&xml);
    assert_eq!(action.entity_ref.to_string(), "e");
    let EntityActionChoice::AddEntityAction(add) = &action.action else {
        panic!("expected AddEntityAction");
    };
    let vertices = &add
        .position
        .trajectory_position()
        .as_ref()
        .expect("TrajectoryPosition dropped")
        .trajectory_ref
        .trajectory_ref
        .as_trajectory()
        .expect("Trajectory dropped")
        .shape
        .as_polyline()
        .expect("Polyline dropped")
        .vertices;
    assert_eq!(vertices.len(), 2);
    let out = quick_xml::se::to_string_with_root("EntityAction", &action)
        .expect("EntityAction failed to serialize");
    assert_eq!(out, xml);
}

// ═══ 3. GlobalActionElement — same shape, wraps Vec-bearing branches ═══════

#[test]
fn global_action_element_with_sequence_below_parses() {
    let xml = format!(
        r#"<GlobalAction><EntityAction entityRef="e"><AddEntityAction>{TRAJECTORY_POSITION}</AddEntityAction></EntityAction></GlobalAction>"#
    );
    let element: GlobalActionElement = de(&xml);
    assert!(matches!(element.action, GlobalAction::EntityAction(_)));
    let out = quick_xml::se::to_string_with_root("GlobalAction", &element)
        .expect("GlobalActionElement failed to serialize");
    assert_eq!(out, xml);
}

#[test]
fn global_action_element_with_traffic_area_action_parses() {
    let xml = format!("<GlobalAction>{TRAFFIC_AREA_ACTION}</GlobalAction>");
    let element: GlobalActionElement = de(&xml);
    assert!(matches!(element.action, GlobalAction::TrafficAction(_)));
    let out = quick_xml::se::to_string_with_root("GlobalAction", &element)
        .expect("GlobalActionElement failed to serialize");
    assert_eq!(out, xml);
}

// ═══ 4. PrivateActionElement — same shape; nothing repeatable below it ══════
//
// No corpus file routes through it: the document model reaches a
// story-level `<PrivateAction>` via `scenario::story::StoryPrivateAction`,
// which is already a parallel-`Option` struct with no `flatten` at all. Nothing in this
// wrapper's XSD subtree is repeatable, which is why it was believed to work. It did not.

#[test]
fn private_action_element_with_sequence_below_parses() {
    let xml = concat!(
        r#"<PrivateAction><RoutingAction><FollowTrajectoryAction>"#,
        r#"<TimeReference><None/></TimeReference><TrajectoryRef>"#,
        r#"<Trajectory name="T" closed="false"><Shape><Polyline>"#,
        r#"<Vertex time="0"><Position><WorldPosition x="0" y="0"/></Position></Vertex>"#,
        r#"<Vertex time="1"><Position><WorldPosition x="1" y="1"/></Position></Vertex>"#,
        r#"</Polyline></Shape></Trajectory></TrajectoryRef>"#,
        r#"<TrajectoryFollowingMode followingMode="position"/>"#,
        r#"</FollowTrajectoryAction></RoutingAction></PrivateAction>"#
    );
    let element: PrivateActionElement = de(xml);
    assert!(matches!(element.action, PrivateAction::RoutingAction(_)));
    let out = quick_xml::se::to_string_with_root("PrivateAction", &element)
        .expect("PrivateActionElement failed to serialize");
    assert_eq!(out, xml);
}

// ═══ 5. The choice invariant, now enforced at parse time ══════════════════
//
// The public types are still externally-tagged enums, so "no branch" and "two
// branches" remain unrepresentable in memory. The private wire representation
// can express both, and the `TryFrom` rejects them — the same pair of errors
// `GlobalAction::validate` (src/types/scenario/init.rs) returns.

#[test]
fn traffic_action_rejects_zero_branches() {
    let err = de_err::<TrafficAction>(r#"<TrafficAction trafficName="t"/>"#);
    assert!(err.contains("missing field `$value`"), "got: {err}");
}

#[test]
fn traffic_action_rejects_two_branches() {
    let err = de_err::<TrafficAction>(
        r#"<TrafficAction><TrafficStopAction/><TrafficSinkAction radius="1"><Position><WorldPosition x="0" y="0"/></Position></TrafficSinkAction></TrafficAction>"#,
    );
    assert!(err.contains("duplicate field `$value`"), "got: {err}");
}

#[test]
fn entity_action_rejects_zero_and_two_branches() {
    let none = de_err::<EntityAction>(r#"<EntityAction entityRef="e"/>"#);
    assert!(none.contains("missing field `$value`"), "got: {none}");
    let two = de_err::<EntityAction>(
        r#"<EntityAction entityRef="e"><DeleteEntityAction/><AddEntityAction><Position><WorldPosition x="0" y="0"/></Position></AddEntityAction></EntityAction>"#,
    );
    assert!(two.contains("duplicate field `$value`"), "got: {two}");
}

#[test]
fn global_action_element_rejects_zero_and_two_branches() {
    let none = de_err::<GlobalActionElement>("<GlobalAction/>");
    assert!(none.contains("missing field `$value`"), "got: {none}");
    let two = de_err::<GlobalActionElement>(
        r#"<GlobalAction><TrafficAction><TrafficStopAction/></TrafficAction><VariableAction variableRef="v"><SetAction value="1"/></VariableAction></GlobalAction>"#,
    );
    assert!(two.contains("duplicate field `$value`"), "got: {two}");
}

#[test]
fn private_action_element_rejects_zero_and_two_branches() {
    let none = de_err::<PrivateActionElement>("<PrivateAction/>");
    assert!(none.contains("missing field `$value`"), "got: {none}");
    let two = de_err::<PrivateActionElement>(
        r#"<PrivateAction><VisibilityAction graphics="true" traffic="true" sensors="true"/><TeleportAction><Position><WorldPosition x="0" y="0"/></Position></TeleportAction></PrivateAction>"#,
    );
    assert!(two.contains("duplicate field `$value`"), "got: {two}");
}

// ═══ 6. Four wrappers that were immune, converted too ═════════════════════
//
// `VariableAction`, `VariableModifyRule`, `ParameterAction` and `ModifyRule`
// carried `#[serde(flatten)]` and parsed correctly because their entire XSD
// subtree has no element with `maxOccurs > 1`. That immunity was a property of
// the schema as it stands, not of the construct. All four now use `$value`,
// which rejects a missing or duplicated choice structurally; under `flatten` a
// two-branch document was accepted and the second branch discarded without a
// word. Their byte-exact round trips live in `choice_wrappers_roundtrip_test.rs`.

#[test]
fn variable_action_rejects_zero_and_two_branches() {
    let none = de_err::<VariableAction>(r#"<VariableAction variableRef="v"/>"#);
    assert!(none.contains("missing field `$value`"), "got: {none}");
    let two = de_err::<VariableAction>(
        r#"<VariableAction variableRef="v"><SetAction value="1"/><ModifyAction><Rule><AddValue value="1"/></Rule></ModifyAction></VariableAction>"#,
    );
    assert!(two.contains("duplicate field `$value`"), "got: {two}");
}

#[test]
fn variable_modify_rule_rejects_zero_and_two_branches() {
    let none = de_err::<VariableModifyRule>("<Rule/>");
    assert!(none.contains("missing field `$value`"), "got: {none}");
    let two = de_err::<VariableModifyRule>(
        r#"<Rule><AddValue value="1"/><MultiplyByValue value="2"/></Rule>"#,
    );
    assert!(two.contains("duplicate field `$value`"), "got: {two}");
}

#[test]
fn parameter_action_rejects_zero_and_two_branches() {
    let none = de_err::<ParameterAction>(r#"<ParameterAction parameterRef="p"/>"#);
    assert!(none.contains("missing field `$value`"), "got: {none}");
    let two = de_err::<ParameterAction>(
        r#"<ParameterAction parameterRef="p"><SetAction value="1"/><ModifyAction><Rule><AddValue value="1"/></Rule></ModifyAction></ParameterAction>"#,
    );
    assert!(two.contains("duplicate field `$value`"), "got: {two}");
}

#[test]
fn modify_rule_rejects_zero_and_two_branches() {
    let none = de_err::<ModifyRule>("<Rule/>");
    assert!(none.contains("missing field `$value`"), "got: {none}");
    let two =
        de_err::<ModifyRule>(r#"<Rule><AddValue value="1"/><MultiplyByValue value="2"/></Rule>"#);
    assert!(two.contains("duplicate field `$value`"), "got: {two}");
}

// ═══ 7. Five more sites, immune until now, converted from `flatten` to
// `$value` ═════
//
// `TrafficSignalAction`, `LaneChangeTarget`, `LaneOffsetTarget`,
// `LateralAction` and `FinalSpeed` parsed correctly under `flatten` because
// nothing in their schema subtree carries `maxOccurs > 1`. That immunity was
// a property of the schema as it stands, not of the construct, so they are
// converted along with the rest rather than left as a list of places the
// construct happens to be survivable. `TrafficSignalStateAction` round-trips
// here; the other four branches' round trips live in
// `choice_wrappers_roundtrip_test.rs`.

#[test]
fn traffic_signal_action_state_round_trips_byte_for_byte() {
    let xml = r#"<TrafficSignalAction><TrafficSignalStateAction name="s1" state="green"/></TrafficSignalAction>"#;
    let action: TrafficSignalAction = de(xml);
    let out = quick_xml::se::to_string_with_root("TrafficSignalAction", &action)
        .expect("TrafficSignalAction failed to serialize");
    assert_eq!(out, xml);
}

#[test]
fn traffic_signal_action_rejects_zero_and_two_branches() {
    let none = de_err::<TrafficSignalAction>("<TrafficSignalAction/>");
    assert!(none.contains("missing field `$value`"), "got: {none}");
    let two = de_err::<TrafficSignalAction>(
        r#"<TrafficSignalAction><TrafficSignalStateAction name="s1" state="green"/><TrafficSignalControllerAction trafficSignalControllerRef="c" phase="p"/></TrafficSignalAction>"#,
    );
    assert!(two.contains("duplicate field `$value`"), "got: {two}");
}

#[test]
fn lane_change_target_rejects_zero_and_two_branches() {
    let none = de_err::<LaneChangeTarget>("<LaneChangeTarget/>");
    assert!(none.contains("missing field `$value`"), "got: {none}");
    let two = de_err::<LaneChangeTarget>(
        r#"<LaneChangeTarget><AbsoluteTargetLane value="2"/><RelativeTargetLane entityRef="e" value="1"/></LaneChangeTarget>"#,
    );
    assert!(two.contains("duplicate field `$value`"), "got: {two}");
}

#[test]
fn lane_offset_target_rejects_zero_and_two_branches() {
    let none = de_err::<LaneOffsetTarget>("<LaneOffsetTarget/>");
    assert!(none.contains("missing field `$value`"), "got: {none}");
    let two = de_err::<LaneOffsetTarget>(
        r#"<LaneOffsetTarget><AbsoluteTargetLaneOffset value="1.5"/><RelativeTargetLaneOffset entityRef="e" value="1"/></LaneOffsetTarget>"#,
    );
    assert!(two.contains("duplicate field `$value`"), "got: {two}");
}

#[test]
fn lateral_action_rejects_zero_and_two_branches() {
    let none = de_err::<LateralAction>("<LateralAction/>");
    assert!(none.contains("missing field `$value`"), "got: {none}");
    let two = de_err::<LateralAction>(
        r#"<LateralAction><LateralDistanceAction entityRef="ego" freespace="true" continuous="false"/><LateralDistanceAction entityRef="ego" freespace="true" continuous="false"/></LateralAction>"#,
    );
    assert!(two.contains("duplicate field `$value`"), "got: {two}");
}

#[test]
fn final_speed_rejects_zero_and_two_branches() {
    let none = de_err::<FinalSpeed>("<FinalSpeed/>");
    assert!(none.contains("missing field `$value`"), "got: {none}");
    let two = de_err::<FinalSpeed>(
        r#"<FinalSpeed><AbsoluteSpeed value="27.5"/><RelativeSpeedToMaster speedTargetValueType="delta" value="1.0"/></FinalSpeed>"#,
    );
    assert!(two.contains("duplicate field `$value`"), "got: {two}");
}
