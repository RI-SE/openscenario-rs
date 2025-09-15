//! Monitor declaration types for OpenSCENARIO
//!
//! This module contains monitor declaration types for runtime monitoring
//! and validation of scenario conditions.

use crate::types::basic::OSString;
use serde::{Deserialize, Serialize};

/// Monitor declarations container
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MonitorDeclarations {
    #[serde(rename = "MonitorDeclaration", default)]
    pub monitor_declarations: Vec<MonitorDeclaration>,
}

/// Individual monitor declaration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MonitorDeclaration {
    #[serde(rename = "@name")]
    pub name: OSString,
    #[serde(rename = "@condition")]
    pub condition: OSString,
    #[serde(rename = "@frequency", skip_serializing_if = "Option::is_none")]
    pub frequency: Option<f64>,
    #[serde(rename = "@enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl Default for MonitorDeclaration {
    fn default() -> Self {
        Self {
            name: OSString::literal("DefaultMonitor".to_string()),
            condition: OSString::literal("true".to_string()),
            frequency: None,
            enabled: Some(true),
        }
    }
}

impl MonitorDeclarations {
    /// Create empty monitor declarations
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with single monitor
    pub fn with_monitor(name: String, condition: String) -> Self {
        Self {
            monitor_declarations: vec![MonitorDeclaration {
                name: OSString::literal(name),
                condition: OSString::literal(condition),
                frequency: None,
                enabled: Some(true),
            }],
        }
    }

    /// Add a monitor declaration
    pub fn add_monitor(
        &mut self,
        name: String,
        condition: String,
        frequency: Option<f64>,
        enabled: Option<bool>,
    ) {
        self.monitor_declarations.push(MonitorDeclaration {
            name: OSString::literal(name),
            condition: OSString::literal(condition),
            frequency,
            enabled,
        });
    }

    /// Check if declarations is empty
    pub fn is_empty(&self) -> bool {
        self.monitor_declarations.is_empty()
    }

    /// Get number of monitor declarations
    pub fn len(&self) -> usize {
        self.monitor_declarations.len()
    }
}

impl MonitorDeclaration {
    /// Create new monitor declaration
    pub fn new(name: String, condition: String) -> Self {
        Self {
            name: OSString::literal(name),
            condition: OSString::literal(condition),
            frequency: None,
            enabled: Some(true),
        }
    }

    /// Create monitor with frequency
    pub fn with_frequency(name: String, condition: String, frequency: f64) -> Self {
        Self {
            name: OSString::literal(name),
            condition: OSString::literal(condition),
            frequency: Some(frequency),
            enabled: Some(true),
        }
    }

    /// Create disabled monitor
    pub fn disabled(name: String, condition: String) -> Self {
        Self {
            name: OSString::literal(name),
            condition: OSString::literal(condition),
            frequency: None,
            enabled: Some(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_declarations_creation() {
        let decls = MonitorDeclarations::new();
        assert!(decls.is_empty());
        assert_eq!(decls.len(), 0);

        let single_monitor =
            MonitorDeclarations::with_monitor("test_monitor".to_string(), "speed > 50".to_string());
        assert!(!single_monitor.is_empty());
        assert_eq!(single_monitor.len(), 1);
    }

    #[test]
    fn test_monitor_declaration_creation() {
        let basic_monitor =
            MonitorDeclaration::new("speed_monitor".to_string(), "speed > 50".to_string());
        assert_eq!(basic_monitor.enabled, Some(true));
        assert_eq!(basic_monitor.frequency, None);

        let freq_monitor = MonitorDeclaration::with_frequency(
            "speed_monitor".to_string(),
            "speed > 50".to_string(),
            10.0,
        );
        assert_eq!(freq_monitor.frequency, Some(10.0));

        let disabled_monitor =
            MonitorDeclaration::disabled("debug_monitor".to_string(), "false".to_string());
        assert_eq!(disabled_monitor.enabled, Some(false));
    }

    #[test]
    fn test_add_monitor() {
        let mut decls = MonitorDeclarations::new();
        decls.add_monitor(
            "monitor1".to_string(),
            "condition1".to_string(),
            None,
            Some(true),
        );
        decls.add_monitor(
            "monitor2".to_string(),
            "condition2".to_string(),
            Some(5.0),
            Some(false),
        );

        assert_eq!(decls.len(), 2);
        assert_eq!(decls.monitor_declarations[0].frequency, None);
        assert_eq!(decls.monitor_declarations[1].frequency, Some(5.0));
        assert_eq!(decls.monitor_declarations[0].enabled, Some(true));
        assert_eq!(decls.monitor_declarations[1].enabled, Some(false));
    }
}
