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

/// Position enum that can hold different position types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Position {
    /// World position (absolute X,Y,Z coordinates)
    WorldPosition(WorldPosition),
    /// Relative world position (relative to an entity)
    RelativeWorldPosition(RelativeWorldPosition),
    // Other position types will be added later
}

/// World position with X, Y, Z coordinates
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WorldPosition {
    #[serde(rename = "@x")]
    pub x: f64,
    #[serde(rename = "@y")]
    pub y: f64,
    #[serde(rename = "@z")]
    pub z: f64,
}

/// Relative world position relative to an entity
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RelativeWorldPosition {
    #[serde(rename = "@entityRef")]
    pub entity_ref: String,
    #[serde(rename = "@dx")]
    pub dx: f64,
    #[serde(rename = "@dy")]
    pub dy: f64,
    #[serde(rename = "@dz")]
    pub dz: f64,
}