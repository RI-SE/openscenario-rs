//! Custom deserialization logic for complex OpenSCENARIO patterns
//!
//! This file contains:
//! - Custom serde deserializers for parameter expressions and references
//! - Polymorphic deserialization for choice-based XML elements
//! - Attribute vs. element handling for mixed content patterns
//! - Custom date/time parsing for OpenSCENARIO format requirements
//! - Deserialization hooks for catalog reference resolution
//!
//! Contributes to project by:
//! - Handling complex XML patterns that don't map directly to Rust types
//! - Supporting OpenSCENARIO's parameter and expression system
//! - Enabling proper deserialization of polymorphic XML content
//! - Providing custom serialization formats for specific data types
//! - Supporting lazy deserialization for performance optimization