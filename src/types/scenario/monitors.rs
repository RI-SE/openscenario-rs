//! Monitor declaration types for OpenSCENARIO
//!
//! This module contains monitor declaration types for runtime monitoring
//! and validation of scenario conditions.

use crate::types::basic::{Boolean, OSString};
use serde::{Deserialize, Serialize};

/// Monitor declarations container
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MonitorDeclarations {
    #[serde(rename = "MonitorDeclaration", default)]
    pub monitor_declarations: Vec<MonitorDeclaration>,
}

/// Individual monitor declaration
///
/// Corresponds to XSD complexType `MonitorDeclaration`: exactly `@name`
/// (String, required) and `@value` (Boolean, required).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MonitorDeclaration {
    #[serde(rename = "@name")]
    pub name: OSString,
    #[serde(rename = "@value")]
    pub value: Boolean,
}

impl Default for MonitorDeclaration {
    fn default() -> Self {
        Self {
            name: OSString::literal("DefaultMonitor".to_string()),
            value: Boolean::literal(false),
        }
    }
}

impl MonitorDeclarations {
    /// Create empty monitor declarations
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with single monitor
    pub fn with_monitor(name: String, value: bool) -> Self {
        Self {
            monitor_declarations: vec![MonitorDeclaration {
                name: OSString::literal(name),
                value: Boolean::literal(value),
            }],
        }
    }

    /// Add a monitor declaration
    pub fn add_monitor(&mut self, name: String, value: bool) {
        self.monitor_declarations.push(MonitorDeclaration {
            name: OSString::literal(name),
            value: Boolean::literal(value),
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
    pub fn new(name: String, value: bool) -> Self {
        Self {
            name: OSString::literal(name),
            value: Boolean::literal(value),
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

        let single_monitor = MonitorDeclarations::with_monitor("test_monitor".to_string(), true);
        assert!(!single_monitor.is_empty());
        assert_eq!(single_monitor.len(), 1);
    }

    #[test]
    fn test_monitor_declaration_creation() {
        let basic_monitor = MonitorDeclaration::new("speed_monitor".to_string(), true);
        assert_eq!(basic_monitor.value.as_literal(), Some(&true));

        let disabled_monitor = MonitorDeclaration::new("debug_monitor".to_string(), false);
        assert_eq!(disabled_monitor.value.as_literal(), Some(&false));
    }

    #[test]
    fn test_add_monitor() {
        let mut decls = MonitorDeclarations::new();
        decls.add_monitor("monitor1".to_string(), true);
        decls.add_monitor("monitor2".to_string(), false);

        assert_eq!(decls.len(), 2);
        assert_eq!(decls.monitor_declarations[0].value.as_literal(), Some(&true));
        assert_eq!(decls.monitor_declarations[1].value.as_literal(), Some(&false));
    }

    #[test]
    fn test_monitor_declaration_roundtrip() {
        // XSD: <MonitorDeclaration name="..." value="..."/>
        let xml = r#"<MonitorDeclaration name="speedMonitor" value="true"/>"#;
        let decl: MonitorDeclaration = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(decl.name.as_literal(), Some(&"speedMonitor".to_string()));
        assert_eq!(decl.value.as_literal(), Some(&true));
    }
}
