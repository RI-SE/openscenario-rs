//! `ConnectTrailerAction` and `DisconnectTrailerAction`, and the trailer reference
//! the first of them takes.

use crate::types::basic::OSString;
use serde::{Deserialize, Serialize};

/// Main trailer action wrapper containing all trailer action types
///
/// XSD `TrailerAction` (`:2342-2347`): a bare `xsd:choice` of `ConnectTrailerAction` |
/// `DisconnectTrailerAction`, no occurrence attributes, so exactly one branch is required.
/// `$value` reads the branch from the live reader by element name; parallel `Option`
/// fields would let both branches populate at once and both re-serialize, which no
/// schema-valid document can express.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrailerAction {
    #[serde(rename = "$value")]
    pub choice: TrailerActionChoice,
}

/// The branch selected by a `TrailerAction`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum TrailerActionChoice {
    ConnectTrailerAction(ConnectTrailerAction),
    DisconnectTrailerAction(DisconnectTrailerAction),
}

/// Connect trailer action for attaching trailers to vehicles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectTrailerAction {
    /// Reference to the trailer entity to connect
    #[serde(rename = "@trailerRef")]
    pub trailer_ref: OSString,
}

/// Disconnect trailer action for detaching trailers from vehicles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DisconnectTrailerAction {
    // Empty according to schema
}

impl ConnectTrailerAction {
    /// XSD `ConnectTrailerAction` (:967-969): required `@trailerRef`.
    pub fn new(trailer_ref: impl Into<String>) -> Self {
        Self {
            trailer_ref: OSString::literal(trailer_ref.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connect_trailer_action() {
        let action = ConnectTrailerAction {
            trailer_ref: OSString::literal("TestTrailer".to_string()),
        };

        assert_eq!(
            action.trailer_ref.as_literal(),
            Some(&"TestTrailer".to_string())
        );
    }

    #[test]
    fn test_trailer_action_serialization() {
        let action = TrailerAction {
            choice: TrailerActionChoice::ConnectTrailerAction(ConnectTrailerAction::new(
                "DefaultTrailer",
            )),
        };

        let serialized = quick_xml::se::to_string(&action).expect("Serialization should succeed");
        assert!(serialized.contains("ConnectTrailerAction"));
        assert!(serialized.contains("DefaultTrailer"));
    }

    #[test]
    fn test_trailer_action_zero_branches_rejected() {
        let xml = "<TrailerAction></TrailerAction>";
        let err = quick_xml::de::from_str::<TrailerAction>(xml).unwrap_err();
        assert!(
            err.to_string().contains("missing field `$value`"),
            "got: {err}"
        );
    }

    #[test]
    fn test_trailer_action_two_branches_rejected() {
        let xml = r#"<TrailerAction><ConnectTrailerAction trailerRef="t1"/><DisconnectTrailerAction/></TrailerAction>"#;
        let err = quick_xml::de::from_str::<TrailerAction>(xml).unwrap_err();
        assert!(
            err.to_string().contains("duplicate field `$value`"),
            "got: {err}"
        );
    }

    #[test]
    fn test_trailer_action_connect_round_trip() {
        let xml = r#"<TrailerAction><ConnectTrailerAction trailerRef="trailer1"/></TrailerAction>"#;
        let action: TrailerAction = quick_xml::de::from_str(xml).unwrap();
        match &action.choice {
            TrailerActionChoice::ConnectTrailerAction(c) => {
                assert_eq!(c.trailer_ref.as_literal(), Some(&"trailer1".to_string()))
            }
            other => panic!("expected ConnectTrailerAction, got {other:?}"),
        }
        let serialized = quick_xml::se::to_string(&action).unwrap();
        assert_eq!(serialized, xml);
        let reparsed: TrailerAction = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(action, reparsed);
    }

    #[test]
    fn test_trailer_action_disconnect_round_trip() {
        let xml = "<TrailerAction><DisconnectTrailerAction/></TrailerAction>";
        let action: TrailerAction = quick_xml::de::from_str(xml).unwrap();
        assert!(matches!(
            action.choice,
            TrailerActionChoice::DisconnectTrailerAction(_)
        ));
        let serialized = quick_xml::se::to_string(&action).unwrap();
        assert_eq!(serialized, xml);
        let reparsed: TrailerAction = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(action, reparsed);
    }
}
