//! The story tree: `Story` → `Act` → `ManeuverGroup` → `Maneuver` → `Event`.
//!
//! Each level carries its own parameter scope and its own triggers. A `ManeuverGroup`
//! binds maneuvers to actors; an `Act` starts and stops on triggers of its own.
use crate::types::basic::{MinVec, OSString, UnsignedInt, Value};
use crate::types::enums::Priority;
use serde::{Deserialize, Serialize};

// Import the real Trigger from triggers module
use super::triggers::Trigger;

// Import the real ParameterDeclarations from basic module
use crate::types::basic::ParameterDeclarations;

/// Story-level Action wrapper with name attribute and action content
///
/// XSD `Action` (:705-712): required `@name` plus a choice of `GlobalAction`
/// | `UserDefinedAction` | `PrivateAction`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoryAction {
    /// Name of the action
    #[serde(rename = "@name")]
    pub name: OSString,

    /// The concrete action carried by this element.
    ///
    /// The group is a bare `xsd:choice`, so `minOccurs` and `maxOccurs` both
    /// default to 1 and exactly one branch is required. An externally-tagged
    /// enum behind `$value` states that: the element name comes from the
    /// serialized variant, the sibling `@name` attribute coexists with it, and
    /// serde rejects a document naming no branch with `missing field $value`
    /// and one naming two with `duplicate field $value`. Parallel `Option`
    /// fields instead describe `xsd:all` with optional members, which accepts
    /// both of those documents.
    #[serde(rename = "$value")]
    pub action: StoryActionChoice,
}

/// The three branches of the XSD `Action` choice (:705-712).
///
/// Each variant carries an element wrapper rather than the branch enum itself.
/// quick-xml cannot serialize an externally-tagged enum whose variant payload
/// is another externally-tagged enum, and it reports
/// `Unsupported("cannot serialize enum newtype variant ...")` when asked to.
/// The wrapper struct holds its own `$value`, so each level writes one element
/// name.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum StoryActionChoice {
    GlobalAction(StoryGlobalAction),
    UserDefinedAction(crate::types::actions::wrappers::UserDefinedAction),
    PrivateAction(StoryPrivateAction),
}

/// Element wrapper hosting the `GlobalAction` choice.
///
/// XSD `GlobalAction` (:1282-1293) is a choice, already modeled as
/// `wrappers::GlobalAction`. As a named child element the choice has to sit
/// behind a wrapper struct, the same shape `RoutePosition.RouteRefElement`
/// uses.
// `#[derive(Default)]` removed: it required
// `wrappers::GlobalAction: Default`, which fabricated a choice branch
// (`GlobalAction::TrafficAction`) for what XSD:1282-1293 declares a
// `xsd:choice` with no default. `global_action` above is `Option<..>`, so
// no `Default` is needed here.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename = "GlobalAction")]
pub struct StoryGlobalAction {
    /// The concrete global action carried by this element.
    ///
    /// `$value` takes the element name from the serialized variant, so the
    /// branch is read from the live reader. `#[serde(flatten)]` cannot do
    /// that: it buffers the children into a map through `deserialize_any`,
    /// and a `Vec<T>` replayed out of that buffer fails with
    /// `invalid type: map, expected a sequence`. `TrafficAction` reaches
    /// `RoadRange`, which requires two `RoadCursor` children, hence that
    /// branch was unreadable at story level.
    #[serde(rename = "$value")]
    pub action: crate::types::actions::wrappers::GlobalAction,
}

/// Private action at story level (reuses init-level structure)
///
/// XSD `PrivateAction` (:1777-1791) is a bare `xsd:choice` of ten branches.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoryPrivateAction {
    /// The concrete private action carried by this element.
    ///
    /// The group is a bare `xsd:choice`, so `minOccurs` and `maxOccurs` both
    /// default to 1 and exactly one branch is required. An externally-tagged
    /// enum behind `$value` states that: the element name comes from the
    /// serialized variant, and serde rejects a document naming no branch with
    /// `missing field $value` and one naming two with
    /// `duplicate field $value`. Parallel `Option` fields instead describe
    /// `xsd:all` with optional members, which accepts both of those documents.
    #[serde(rename = "$value")]
    pub action: StoryPrivateActionChoice,
}

/// The ten branches of the XSD `PrivateAction` choice (:1777-1791).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum StoryPrivateActionChoice {
    LongitudinalAction(crate::types::scenario::init::LongitudinalAction),
    LateralAction(crate::types::actions::movement::LateralAction),
    VisibilityAction(crate::types::actions::VisibilityAction),
    SynchronizeAction(crate::types::actions::SynchronizeAction),
    /// Deprecated in XSD `PrivateAction` (:1777-1791) but widely emitted
    ActivateControllerAction(crate::types::actions::ActivateControllerAction),
    ControllerAction(crate::types::actions::ControllerAction),
    TeleportAction(crate::types::actions::movement::TeleportAction),
    RoutingAction(crate::types::actions::movement::RoutingAction),
    AppearanceAction(crate::types::actions::AppearanceAction),
    TrailerAction(crate::types::actions::TrailerAction),
}

/// XSD declares one `CatalogReference` complexType (`Schema/OpenSCENARIO.xsd:879-885`)
/// for every position that names a catalog entry. A `ManeuverGroup`'s `CatalogReference`
/// resolves to a `Maneuver`, so this is
/// `catalogs::references::CatalogReference<CatalogManeuver>` rather than a second,
/// maneuver-specific struct.
pub use crate::types::catalogs::references::ManeuverCatalogReference as CatalogReference;

/// Story definition with parameter scope and act sequences
///
/// A Story represents a complete narrative sequence within an OpenSCENARIO,
/// containing multiple Acts that define the scenario execution flow.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScenarioStory {
    /// Name of the story
    #[serde(rename = "@name")]
    pub name: OSString,

    /// Parameter declarations scoped to this story
    #[serde(
        rename = "ParameterDeclarations",
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_declarations: Option<ParameterDeclarations>,

    /// Sequence of acts within this story
    #[serde(rename = "Act")]
    pub acts: MinVec<Act, 1>,
}

/// Act definition with maneuver groups and execution triggers
///
/// An Act represents a major phase of scenario execution, containing
/// ManeuverGroups that coordinate entity behaviors.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Act {
    /// Name of the act
    #[serde(rename = "@name")]
    pub name: OSString,

    /// ManeuverGroups defining coordinated entity behaviors
    #[serde(rename = "ManeuverGroup")]
    pub maneuver_groups: MinVec<ManeuverGroup, 1>,

    /// Trigger conditions to start this act
    #[serde(rename = "StartTrigger", skip_serializing_if = "Option::is_none")]
    pub start_trigger: Option<Trigger>,

    /// Trigger conditions to stop this act
    #[serde(rename = "StopTrigger", skip_serializing_if = "Option::is_none")]
    pub stop_trigger: Option<Trigger>,
}

/// ManeuverGroup for coordinating entity behaviors
///
/// A ManeuverGroup defines which entities (actors) will execute
/// a set of coordinated maneuvers together.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ManeuverGroup {
    /// Name of the maneuver group
    #[serde(rename = "@name")]
    pub name: OSString,

    /// Maximum number of times this group can execute
    #[serde(rename = "@maximumExecutionCount")]
    pub maximum_execution_count: UnsignedInt,

    /// Actors (entities) assigned to this maneuver group
    #[serde(rename = "Actors")]
    pub actors: Actors,

    /// Optional catalog references instead of direct maneuvers
    #[serde(
        rename = "CatalogReference",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub catalog_reference: Vec<CatalogReference>,

    /// Direct maneuver definitions
    #[serde(rename = "Maneuver", default, skip_serializing_if = "Vec::is_empty")]
    pub maneuvers: Vec<Maneuver>,
}

/// Maneuver definition with event sequences and timing
///
/// A Maneuver contains a sequence of Events that define specific
/// actions to be taken by entities.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Maneuver {
    /// Name of the maneuver
    #[serde(rename = "@name")]
    pub name: OSString,

    /// Parameter declarations scoped to this maneuver
    #[serde(
        rename = "ParameterDeclarations",
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_declarations: Option<ParameterDeclarations>,

    /// Sequence of events within this maneuver
    #[serde(rename = "Event")]
    pub events: MinVec<Event, 1>,
}

/// Event definition with action and trigger
///
/// An Event represents a single action that can be triggered
/// under specific conditions during scenario execution.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Event {
    /// Name of the event
    #[serde(rename = "@name")]
    pub name: OSString,

    /// Maximum number of times this event can execute
    #[serde(
        rename = "@maximumExecutionCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub maximum_execution_count: Option<UnsignedInt>,

    /// Priority of this event
    #[serde(rename = "@priority")]
    pub priority: Value<Priority>,

    /// The actions to execute when this event triggers
    #[serde(rename = "Action")]
    pub actions: MinVec<StoryAction, 1>,

    /// Trigger conditions to start this event
    #[serde(rename = "StartTrigger", skip_serializing_if = "Option::is_none")]
    pub start_trigger: Option<Trigger>,
}

/// Actor selection and entity assignment to maneuvers
///
/// Actors define which entities will participate in a ManeuverGroup
/// and can optionally select from triggering entities.
///
/// No `Default`. XSD `Actors` (`Schema/OpenSCENARIO.xsd:723-728`) marks
/// `@selectTriggeringEntities` `use="required"` with no schema `default="…"`, so the
/// derived impl invented `false` — category 1, a fabricated required attribute. The
/// `EntityRef` child *is* `minOccurs="0"`, so only the bool half was wrong; the
/// constructors below keep the empty-`Vec` form available while forcing the caller to
/// state the flag.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Actors {
    /// Whether to select entities that triggered the maneuver group
    #[serde(rename = "@selectTriggeringEntities")]
    pub select_triggering_entities: bool,

    /// Direct entity references for actors
    #[serde(rename = "EntityRef", default)]
    pub entity_refs: Vec<EntityRef>,
}

impl Actors {
    /// Create an actor set, stating `@selectTriggeringEntities` explicitly.
    pub fn new(select_triggering_entities: bool, entity_refs: Vec<EntityRef>) -> Self {
        Self {
            select_triggering_entities,
            entity_refs,
        }
    }

    /// Actors drawn from the triggering entities (`selectTriggeringEntities="true"`).
    pub fn triggering() -> Self {
        Self::new(true, Vec::new())
    }

    /// Actors named explicitly (`selectTriggeringEntities="false"`).
    pub fn named(entity_refs: Vec<EntityRef>) -> Self {
        Self::new(false, entity_refs)
    }
}

/// Reference to an entity for actor assignment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EntityRef {
    /// Name of the referenced entity
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,
}

// `Default` impls removed for both types below. `StoryAction`'s `@name`
// is `use="required"` (XSD `Action`, :705-712) with no schema default, and its old impl
// additionally fabricated a whole `PrivateAction` child nobody wrote. `StoryPrivateAction`
// mirrors XSD `PrivateAction` (:1777-1791), a bare `xsd:choice` with no `minOccurs="0"`
// override, and its old impl fabricated the `SpeedAction` branch. Neither choice has a
// schema-declared default, hence neither type gets a replacement `Default`; callers build one
// branch explicitly via the constructors below.
impl StoryAction {
    /// Create a named `PrivateAction` (XSD `Action` choice member; `@name` is required and has
    /// no schema default, so it must be supplied).
    pub fn private(name: &str, private_action: StoryPrivateAction) -> Self {
        Self {
            name: OSString::literal(name.to_string()),
            action: StoryActionChoice::PrivateAction(private_action),
        }
    }
}

impl StoryPrivateAction {
    /// `PrivateAction` choosing the `LongitudinalAction` branch.
    pub fn longitudinal(action: crate::types::scenario::init::LongitudinalAction) -> Self {
        Self {
            action: StoryPrivateActionChoice::LongitudinalAction(action),
        }
    }

    /// `PrivateAction` choosing the `VisibilityAction` branch.
    pub fn visibility(action: crate::types::actions::VisibilityAction) -> Self {
        Self {
            action: StoryPrivateActionChoice::VisibilityAction(action),
        }
    }

    /// `PrivateAction` choosing the `TeleportAction` branch.
    pub fn teleport(action: crate::types::actions::movement::TeleportAction) -> Self {
        Self {
            action: StoryPrivateActionChoice::TeleportAction(action),
        }
    }
}

impl ScenarioStory {
    /// Create a story with the given name and acts.
    ///
    /// The acts are a parameter rather than something the caller appends afterwards
    /// because XSD `Story` (`Schema/OpenSCENARIO.xsd:2108`) declares `Act` with the
    /// default `minOccurs="1"`. An empty story is not a story the schema can describe,
    /// so there is no state in which this type is legitimately empty and no reason to
    /// offer a constructor that produces one.
    pub fn new(name: impl Into<String>, acts: Vec<Act>) -> crate::Result<Self> {
        Ok(Self {
            name: OSString::literal(name.into()),
            parameter_declarations: None,
            acts: MinVec::new(acts)?,
        })
    }
}

impl Act {
    /// Create an act with the given name and maneuver groups.
    ///
    /// XSD `Act` (`Schema/OpenSCENARIO.xsd:699`) declares `ManeuverGroup` with the
    /// default `minOccurs="1"`, so the groups are supplied here rather than appended
    /// later.
    pub fn new(
        name: impl Into<String>,
        maneuver_groups: Vec<ManeuverGroup>,
    ) -> crate::Result<Self> {
        Ok(Self {
            name: OSString::literal(name.into()),
            maneuver_groups: MinVec::new(maneuver_groups)?,
            start_trigger: None,
            stop_trigger: None,
        })
    }
}

impl ManeuverGroup {
    /// Create a new maneuver group with the given name, maximum execution count and
    /// actors, and no catalog references or maneuvers.
    ///
    /// `@maximumExecutionCount` is `use="required"` in the XSD
    /// (`Schema/OpenSCENARIO.xsd`: `<xsd:attribute name="maximumExecutionCount"
    /// type="UnsignedInt" use="required"/>`) with no `default="…"`, so there is
    /// no schema-backed value to assume here — the caller must supply one.
    ///
    /// `actors` likewise became a parameter: it used to be `Actors::default()`,
    /// which fabricated `selectTriggeringEntities="false"` — an attribute the XSD marks
    /// `use="required"` with no schema default. `Actors` is required here too
    /// (`Schema/OpenSCENARIO.xsd` `ManeuverGroup`), so there is nothing to elide.
    pub fn new(name: impl Into<String>, maximum_execution_count: u32, actors: Actors) -> Self {
        Self {
            name: OSString::literal(name.into()),
            maximum_execution_count: UnsignedInt::literal(maximum_execution_count),
            actors,
            catalog_reference: Vec::new(),
            maneuvers: Vec::new(),
        }
    }
}

impl Maneuver {
    /// Create a maneuver with the given name and events.
    ///
    /// XSD `Maneuver` (`Schema/OpenSCENARIO.xsd:1453`) declares `Event` with the default
    /// `minOccurs="1"`, so the events are supplied here rather than appended later.
    pub fn new(name: impl Into<String>, events: Vec<Event>) -> crate::Result<Self> {
        Ok(Self {
            name: OSString::literal(name.into()),
            parameter_declarations: None,
            events: MinVec::new(events)?,
        })
    }
}

impl Event {
    /// Create an event with the given name, priority and actions.
    ///
    /// `@priority` is `use="required"` in the XSD with no `default="…"`, so it
    /// must be supplied explicitly.
    /// `Action` carries the default `minOccurs="1"` in XSD `Event`
    /// (`Schema/OpenSCENARIO.xsd:1208`), so the actions are supplied here. An event that
    /// triggers nothing is not a document the schema admits.
    pub fn new(
        name: impl Into<String>,
        priority: Priority,
        actions: Vec<StoryAction>,
    ) -> crate::Result<Self> {
        Ok(Self {
            name: OSString::literal(name.into()),
            maximum_execution_count: None,
            priority: Value::Literal(priority),
            actions: MinVec::new(actions)?,
            start_trigger: None,
        })
    }
}

impl EntityRef {
    /// Create a new entity reference
    pub fn new(entity_ref: impl Into<String>) -> Self {
        Self {
            entity_ref: OSString::literal(entity_ref.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::basic::Value;

    /// One minimal action, for the tests that need an `Event` to be well formed.
    fn one_action() -> Vec<StoryAction> {
        vec![StoryAction::private(
            "TestAction",
            StoryPrivateAction::visibility(crate::types::actions::VisibilityAction::new(
                true, true, true,
            )),
        )]
    }

    fn one_event(name: &str) -> Event {
        Event::new(name, Priority::Override, one_action()).unwrap()
    }

    fn one_maneuver(name: &str) -> Maneuver {
        Maneuver::new(name, vec![one_event("Event1")]).unwrap()
    }

    fn one_group(name: &str) -> ManeuverGroup {
        let mut group = ManeuverGroup::new(name, 1, Actors::named(Vec::new()));
        group.maneuvers.push(one_maneuver("Maneuver1"));
        group
    }

    fn one_act(name: &str) -> Act {
        Act::new(name, vec![one_group("Group1")]).unwrap()
    }

    #[test]
    fn test_story_creation() {
        let story = ScenarioStory {
            name: Value::literal("TestStory".to_string()),
            parameter_declarations: None,
            acts: MinVec::new(vec![one_act("Act1")]).unwrap(),
        };

        assert_eq!(story.name.as_literal().unwrap(), "TestStory");
        assert_eq!(story.acts.len(), 1);
        assert_eq!(story.acts[0].name.as_literal().unwrap(), "Act1");
    }

    #[test]
    fn test_act_with_triggers() {
        let act = Act {
            name: Value::literal("TestAct".to_string()),
            maneuver_groups: MinVec::new(vec![one_group("Group1")]).unwrap(),
            start_trigger: None, // Will add proper trigger tests when Trigger is implemented
            stop_trigger: None,
        };

        assert_eq!(act.name.as_literal().unwrap(), "TestAct");
        assert_eq!(act.maneuver_groups.len(), 1);
    }

    #[test]
    fn test_maneuver_group_with_actors() {
        let actors = Actors {
            select_triggering_entities: true,
            entity_refs: vec![
                EntityRef {
                    entity_ref: Value::literal("Ego".to_string()),
                },
                EntityRef {
                    entity_ref: Value::literal("Target".to_string()),
                },
            ],
        };

        let maneuver_group = ManeuverGroup {
            name: Value::literal("TestGroup".to_string()),
            maximum_execution_count: Value::literal(3),
            actors,
            catalog_reference: Vec::new(),
            maneuvers: vec![one_maneuver("Maneuver1")],
        };

        assert_eq!(maneuver_group.name.as_literal().unwrap(), "TestGroup");
        assert_eq!(
            maneuver_group.maximum_execution_count.as_literal().unwrap(),
            &3
        );
        assert_eq!(maneuver_group.actors.entity_refs.len(), 2);
        assert_eq!(maneuver_group.actors.select_triggering_entities, true);
    }

    #[test]
    fn test_maneuver_with_events() {
        let maneuver = Maneuver {
            name: Value::literal("TestManeuver".to_string()),
            parameter_declarations: None,
            events: MinVec::new(vec![
                Event {
                    name: Value::literal("Event1".to_string()),
                    maximum_execution_count: Some(Value::literal(1)),
                    priority: Value::Literal(Priority::Override),
                    actions: MinVec::new(vec![StoryAction::private(
                        "TestAction",
                        StoryPrivateAction::visibility(
                            crate::types::actions::VisibilityAction::new(true, true, true),
                        ),
                    )])
                    .unwrap(),
                    start_trigger: None,
                },
                Event {
                    name: Value::literal("Event2".to_string()),
                    maximum_execution_count: None,
                    priority: Value::Literal(Priority::Overwrite),
                    actions: MinVec::new(vec![StoryAction::private(
                        "TestAction",
                        StoryPrivateAction::visibility(
                            crate::types::actions::VisibilityAction::new(true, true, true),
                        ),
                    )])
                    .unwrap(),
                    start_trigger: None,
                },
            ])
            .unwrap(),
        };

        assert_eq!(maneuver.name.as_literal().unwrap(), "TestManeuver");
        assert_eq!(maneuver.events.len(), 2);
        assert_eq!(maneuver.events[0].name.as_literal().unwrap(), "Event1");
        assert_eq!(maneuver.events[1].name.as_literal().unwrap(), "Event2");
    }

    #[test]
    fn test_event_with_action() {
        let event = Event {
            name: Value::literal("TestEvent".to_string()),
            maximum_execution_count: Some(Value::literal(5)),
            priority: Value::Literal(Priority::Parallel),
            actions: MinVec::new(vec![StoryAction::private(
                "TestAction",
                StoryPrivateAction::visibility(crate::types::actions::VisibilityAction::new(
                    true, true, true,
                )),
            )])
            .unwrap(),
            start_trigger: None,
        };

        assert_eq!(event.name.as_literal().unwrap(), "TestEvent");
        assert_eq!(
            event
                .maximum_execution_count
                .as_ref()
                .unwrap()
                .as_literal()
                .unwrap(),
            &5
        );
        assert_eq!(event.priority, Value::Literal(Priority::Parallel));
    }

    #[test]
    fn test_actors_entity_selection() {
        let actors = Actors {
            select_triggering_entities: false,
            entity_refs: vec![EntityRef {
                entity_ref: Value::literal("Vehicle1".to_string()),
            }],
        };

        assert_eq!(actors.select_triggering_entities, false);
        assert_eq!(actors.entity_refs.len(), 1);
        assert_eq!(
            actors.entity_refs[0].entity_ref.as_literal().unwrap(),
            "Vehicle1"
        );
    }

    #[test]
    fn test_story_serialization() {
        let story = ScenarioStory::new("TestStory", vec![one_act("Act1")]).unwrap();
        let serialized = quick_xml::se::to_string(&story).expect("Serialization should succeed");
        assert!(serialized.contains("TestStory"));
    }

    #[test]
    fn test_event_missing_priority_fails() {
        let xml = r#"<Event name="TestEvent">
            <Action name="Action1">
                <PrivateAction>
                    <TeleportAction>
                        <Position><WorldPosition x="1" y="2"/></Position>
                    </TeleportAction>
                </PrivateAction>
            </Action>
        </Event>"#;
        let result = quick_xml::de::from_str::<Event>(xml);
        assert!(
            result.is_err(),
            "Event without @priority must fail to parse since it is required by the XSD"
        );
    }

    #[test]
    fn test_event_with_priority_succeeds() {
        let xml = r#"<Event name="TestEvent" priority="override">
            <Action name="Action1">
                <PrivateAction>
                    <TeleportAction>
                        <Position><WorldPosition x="1" y="2"/></Position>
                    </TeleportAction>
                </PrivateAction>
            </Action>
        </Event>"#;
        let result = quick_xml::de::from_str::<Event>(xml);
        assert!(
            result.is_ok(),
            "Event with @priority should parse: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap().priority, Value::Literal(Priority::Override));
    }

    #[test]
    fn test_maneuver_group_catalog_reference_absent_parameter_assignments_roundtrip() {
        let xml = r#"<ManeuverGroup name="Group1" maximumExecutionCount="1"><Actors selectTriggeringEntities="false"/><CatalogReference catalogName="ManeuverCatalog" entryName="Entry1"/></ManeuverGroup>"#;
        let parsed: ManeuverGroup = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.catalog_reference.len(), 1);
        assert!(
            parsed.catalog_reference[0].parameter_assignments.is_none(),
            "absent <ParameterAssignments> must yield None"
        );

        let ser = quick_xml::se::to_string(&parsed).unwrap();
        assert_eq!(ser, xml, "byte-exact round trip, no ParameterAssignments");
    }

    #[test]
    fn test_maneuver_group_catalog_reference_with_parameter_assignments_roundtrip() {
        let xml = r#"<ManeuverGroup name="Group1" maximumExecutionCount="1"><Actors selectTriggeringEntities="false"/><CatalogReference catalogName="ManeuverCatalog" entryName="Entry1"><ParameterAssignments><ParameterAssignment parameterRef="Speed" value="30.0"/></ParameterAssignments></CatalogReference></ManeuverGroup>"#;
        let parsed: ManeuverGroup = quick_xml::de::from_str(xml).unwrap();
        let assignments = parsed.catalog_reference[0]
            .parameter_assignments
            .as_ref()
            .expect("present <ParameterAssignments> must yield Some");
        assert_eq!(assignments.assignments.len(), 1);
        assert_eq!(
            assignments.assignments[0]
                .parameter_ref
                .as_literal()
                .unwrap(),
            "Speed"
        );

        let ser = quick_xml::se::to_string(&parsed).unwrap();
        assert_eq!(ser, xml, "byte-exact round trip with ParameterAssignments");
    }
}
