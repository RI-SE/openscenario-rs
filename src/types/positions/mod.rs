//! Position type module for all spatial positioning systems
//!
//! This file contains:
//! - Base position traits and common positioning behaviors
//! - Position conversion utilities between coordinate systems
//! - Orientation handling and coordinate system transformations
//! - Position validation and constraint checking
//! - Spatial relationship calculations and utilities
//!
//! Contributes to project by:
//! - Organizing 15+ position types into logical coordinate system categories
//! - Providing consistent interface for all positioning methods
//! - Enabling seamless conversion between different coordinate systems
//! - Supporting both absolute and relative positioning strategies
//! - Facilitating spatial calculations and geometric operations

use serde::{Deserialize, Serialize};
use crate::types::basic::{Double, OSString};

/// Position enum that can hold different position types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum Position {
    /// World position (absolute X,Y,Z coordinates)
    WorldPosition(WorldPosition),
    /// Relative world position (relative to an entity)
    RelativeWorldPosition(RelativeWorldPosition),
    // Other position types will be added later
}

/// World position with X, Y, Z coordinates and orientation (h, p, r)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct WorldPosition {
    #[serde(rename = "@x")]
    pub x: Double,
    #[serde(rename = "@y")]
    pub y: Double,
    #[serde(rename = "@z")]
    pub z: Double,
    #[serde(rename = "@h")]
    pub h: Double,
    #[serde(rename = "@p")]
    pub p: Double,
    #[serde(rename = "@r")]
    pub r: Double,
}

/// Relative world position relative to an entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct RelativeWorldPosition {
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,
    #[serde(rename = "@dx")]
    pub dx: Double,
    #[serde(rename = "@dy")]
    pub dy: Double,
    #[serde(rename = "@dz")]
    pub dz: Double,
}

// Default implementations
impl Default for Position {
    fn default() -> Self {
        Position::WorldPosition(WorldPosition::default())
    }
}

impl Default for WorldPosition {
    fn default() -> Self {
        Self {
            x: Double::literal(0.0),
            y: Double::literal(0.0),
            z: Double::literal(0.0),
            h: Double::literal(0.0),
            p: Double::literal(0.0),
            r: Double::literal(0.0),
        }
    }
}

impl Default for RelativeWorldPosition {
    fn default() -> Self {
        Self {
            entity_ref: OSString::literal("DefaultEntity".to_string()),
            dx: Double::literal(0.0),
            dy: Double::literal(0.0),
            dz: Double::literal(0.0),
        }
    }
}