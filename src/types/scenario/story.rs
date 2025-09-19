//! Story and Act types for scenario execution flow
//!
//! This file contains:
//! - Story definition with parameter scope and act sequences
//! - Act organization with maneuver groups and execution triggers
//! - ManeuverGroup for coordinating entity behaviors
//! - Maneuver definitions with event sequences and timing
//! - Actor selection and entity assignment to maneuvers
//!
//! Contributes to project by:
//! - Organizing scenario execution into logical narrative sequences
//! - Supporting parallel and sequential execution patterns
//! - Enabling entity coordination through maneuver groups
//! - Providing flexible actor assignment and role management
//! - Facilitating scenario reuse through parameterized stories

use crate::types::basic::{OSString, UnsignedInt};
use crate::types::enums::Priority;
use serde::{Deserialize, Serialize};

// Import the real Trigger from triggers module
use super::triggers::Trigger;

// Import the real ParameterDeclarations from basic module
use crate::types::basic::ParameterDeclarations;

/// Story-level Action wrapper with name attribute and action content
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoryAction {
    /// Name of the action
    #[serde(rename = "@name")]
    pub name: OSString,

    /// Private action for individual entities
    #[serde(rename = "PrivateAction", skip_serializing_if = "Option::is_none")]
    pub private_action: Option<StoryPrivateAction>,
    // Other action types can be added later as optional fields
}

/// Types of actions available at the story level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum StoryActionType {
    /// Private action for individual entities
    PrivateAction(StoryPrivateAction),
    // Global action affecting the entire scenario (future)
    // GlobalAction(StoryGlobalAction),

    // User-defined action (future)
    // UserDefinedAction(UserDefinedAction),
}

/// Private action at story level (reuses init-level structure)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoryPrivateAction {
    #[serde(rename = "LongitudinalAction", skip_serializing_if = "Option::is_none")]
    pub longitudinal_action: Option<crate::types::scenario::init::LongitudinalAction>,
    #[serde(rename = "LateralAction", skip_serializing_if = "Option::is_none")]
    pub lateral_action: Option<crate::types::actions::movement::LateralAction>,
    #[serde(rename = "VisibilityAction", skip_serializing_if = "Option::is_none")]
    pub visibility_action: Option<crate::types::actions::VisibilityAction>,
    #[serde(rename = "SynchronizeAction", skip_serializing_if = "Option::is_none")]
    pub synchronize_action: Option<crate::types::actions::SynchronizeAction>,
    #[serde(rename = "ControllerAction", skip_serializing_if = "Option::is_none")]
    pub controller_action: Option<crate::types::actions::ControllerAction>,
    #[serde(rename = "TeleportAction", skip_serializing_if = "Option::is_none")]
    pub teleport_action: Option<crate::types::actions::movement::TeleportAction>,
    #[serde(rename = "RoutingAction", skip_serializing_if = "Option::is_none")]
    pub routing_action: Option<crate::types::actions::movement::RoutingAction>,
    #[serde(rename = "AppearanceAction", skip_serializing_if = "Option::is_none")]
    pub appearance_action: Option<crate::types::actions::AppearanceAction>,
    #[serde(rename = "TrailerAction", skip_serializing_if = "Option::is_none")]
    pub trailer_action: Option<crate::types::actions::TrailerAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CatalogReference;

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
    pub acts: Vec<Act>,
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
    pub maneuver_groups: Vec<ManeuverGroup>,

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
    #[serde(
        rename = "@maximumExecutionCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub maximum_execution_count: Option<UnsignedInt>,

    /// Actors (entities) assigned to this maneuver group
    #[serde(rename = "Actors")]
    pub actors: Actors,

    /// Optional catalog reference instead of direct maneuvers
    #[serde(rename = "CatalogReference", skip_serializing_if = "Option::is_none")]
    pub catalog_reference: Option<CatalogReference>,

    /// Direct maneuver definitions
    #[serde(rename = "Maneuver")]
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
    pub events: Vec<Event>,
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
    #[serde(rename = "@priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<Priority>,

    /// The action to execute when this event triggers
    #[serde(rename = "Action")]
    pub action: StoryAction,

    /// Trigger conditions to start this event
    #[serde(rename = "StartTrigger", skip_serializing_if = "Option::is_none")]
    pub start_trigger: Option<Trigger>,
}

/// Actor selection and entity assignment to maneuvers
///
/// Actors define which entities will participate in a ManeuverGroup
/// and can optionally select from triggering entities.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Actors {
    /// Whether to select entities that triggered the maneuver group
    #[serde(
        rename = "@selectTriggeringEntities",
        skip_serializing_if = "Option::is_none"
    )]
    pub select_triggering_entities: Option<bool>,

    /// Direct entity references for actors
    #[serde(rename = "EntityRef")]
    pub entity_refs: Vec<EntityRef>,
}

/// Reference to an entity for actor assignment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EntityRef {
    /// Name of the referenced entity
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,
}

// Default implementations for all structs
impl Default for StoryAction {
    fn default() -> Self {
        Self {
            name: OSString::literal("DefaultAction".to_string()),
            private_action: Some(StoryPrivateAction::default()),
        }
    }
}

impl Default for StoryActionType {
    fn default() -> Self {
        Self::PrivateAction(StoryPrivateAction::default())
    }
}

impl Default for StoryPrivateAction {
    fn default() -> Self {
        Self {
            longitudinal_action: Some(crate::types::scenario::init::LongitudinalAction::default()),
            lateral_action: None,
            visibility_action: None,
            synchronize_action: None,
            controller_action: None,
            teleport_action: None,
            routing_action: None,
            appearance_action: None,
            trailer_action: None,
        }
    }
}

impl Default for ScenarioStory {
    fn default() -> Self {
        Self {
            name: OSString::literal("DefaultStory".to_string()),
            parameter_declarations: None,
            acts: Vec::new(),
        }
    }
}

impl Default for Act {
    fn default() -> Self {
        Self {
            name: OSString::literal("DefaultAct".to_string()),
            maneuver_groups: Vec::new(),
            start_trigger: None,
            stop_trigger: None,
        }
    }
}

impl Default for ManeuverGroup {
    fn default() -> Self {
        Self {
            name: OSString::literal("DefaultManeuverGroup".to_string()),
            maximum_execution_count: None,
            actors: Actors::default(),
            catalog_reference: None,
            maneuvers: Vec::new(),
        }
    }
}

impl Default for Maneuver {
    fn default() -> Self {
        Self {
            name: OSString::literal("DefaultManeuver".to_string()),
            parameter_declarations: None,
            events: Vec::new(),
        }
    }
}

impl Default for Event {
    fn default() -> Self {
        Self {
            name: OSString::literal("DefaultEvent".to_string()),
            maximum_execution_count: None,
            priority: None,
            action: StoryAction::default(),
            start_trigger: None,
        }
    }
}

impl Default for EntityRef {
    fn default() -> Self {
        Self {
            entity_ref: OSString::literal("DefaultEntity".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::basic::Value;

    #[test]
    fn test_story_creation() {
        let story = ScenarioStory {
            name: Value::literal("TestStory".to_string()),
            parameter_declarations: None,
            acts: vec![Act::default()],
        };

        assert_eq!(story.name.as_literal().unwrap(), "TestStory");
        assert_eq!(story.acts.len(), 1);
        assert_eq!(story.acts[0].name.as_literal().unwrap(), "DefaultAct");
    }

    #[test]
    fn test_act_with_triggers() {
        let act = Act {
            name: Value::literal("TestAct".to_string()),
            maneuver_groups: vec![ManeuverGroup::default()],
            start_trigger: None, // Will add proper trigger tests when Trigger is implemented
            stop_trigger: None,
        };

        assert_eq!(act.name.as_literal().unwrap(), "TestAct");
        assert_eq!(act.maneuver_groups.len(), 1);
    }

    #[test]
    fn test_maneuver_group_with_actors() {
        let actors = Actors {
            select_triggering_entities: Some(true),
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
            maximum_execution_count: Some(Value::literal(3)),
            actors,
            catalog_reference: None,
            maneuvers: vec![Maneuver::default()],
        };

        assert_eq!(maneuver_group.name.as_literal().unwrap(), "TestGroup");
        assert_eq!(
            maneuver_group
                .maximum_execution_count
                .as_ref()
                .unwrap()
                .as_literal()
                .unwrap(),
            &3
        );
        assert_eq!(maneuver_group.actors.entity_refs.len(), 2);
        assert_eq!(maneuver_group.actors.select_triggering_entities, Some(true));
    }

    #[test]
    fn test_maneuver_with_events() {
        let maneuver = Maneuver {
            name: Value::literal("TestManeuver".to_string()),
            parameter_declarations: None,
            events: vec![
                Event {
                    name: Value::literal("Event1".to_string()),
                    maximum_execution_count: Some(Value::literal(1)),
                    priority: Some(Priority::Override),
                    action: StoryAction::default(),
                    start_trigger: None,
                },
                Event {
                    name: Value::literal("Event2".to_string()),
                    maximum_execution_count: None,
                    priority: None,
                    action: StoryAction::default(),
                    start_trigger: None,
                },
            ],
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
            priority: Some(Priority::Parallel),
            action: StoryAction::default(),
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
        assert_eq!(event.priority.as_ref().unwrap(), &Priority::Parallel);
    }

    #[test]
    fn test_actors_entity_selection() {
        let actors = Actors {
            select_triggering_entities: Some(false),
            entity_refs: vec![EntityRef {
                entity_ref: Value::literal("Vehicle1".to_string()),
            }],
        };

        assert_eq!(actors.select_triggering_entities, Some(false));
        assert_eq!(actors.entity_refs.len(), 1);
        assert_eq!(
            actors.entity_refs[0].entity_ref.as_literal().unwrap(),
            "Vehicle1"
        );
    }

    #[test]
    fn test_story_serialization() {
        let story = ScenarioStory::default();
        let serialized = quick_xml::se::to_string(&story).expect("Serialization should succeed");
        assert!(serialized.contains("DefaultStory"));
    }
}
