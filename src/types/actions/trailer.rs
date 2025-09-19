//! Trailer action types for trailer connection and disconnection
//!
//! This file contains:
//! - Trailer connection and disconnection actions
//! - Trailer reference management
//!
//! Contributes to project by:
//! - Enabling dynamic trailer coupling/uncoupling during scenario execution
//! - Supporting complex multi-vehicle scenarios with trailers
//! - Following OpenSCENARIO specification for trailer operations

use crate::types::basic::OSString;
use serde::{Deserialize, Serialize};

/// Main trailer action wrapper containing all trailer action types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrailerAction {
    /// Connect trailer action
    #[serde(rename = "ConnectTrailerAction", skip_serializing_if = "Option::is_none")]
    pub connect_trailer_action: Option<ConnectTrailerAction>,
    
    /// Disconnect trailer action
    #[serde(rename = "DisconnectTrailerAction", skip_serializing_if = "Option::is_none")]
    pub disconnect_trailer_action: Option<DisconnectTrailerAction>,
}

/// Connect trailer action for attaching trailers to vehicles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectTrailerAction {
    /// Reference to the trailer entity to connect
    #[serde(rename = "@trailerRef")]
    pub trailer_ref: OSString,
}

/// Disconnect trailer action for detaching trailers from vehicles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DisconnectTrailerAction {
    // Empty according to schema
}

impl Default for TrailerAction {
    fn default() -> Self {
        Self {
            connect_trailer_action: None,
            disconnect_trailer_action: None,
        }
    }
}

impl Default for ConnectTrailerAction {
    fn default() -> Self {
        Self {
            trailer_ref: OSString::literal("DefaultTrailer".to_string()),
        }
    }
}

impl Default for DisconnectTrailerAction {
    fn default() -> Self {
        Self {}
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
            connect_trailer_action: Some(ConnectTrailerAction::default()),
            disconnect_trailer_action: None,
        };

        let serialized = quick_xml::se::to_string(&action).expect("Serialization should succeed");
        assert!(serialized.contains("ConnectTrailerAction"));
        assert!(serialized.contains("DefaultTrailer"));
    }
}