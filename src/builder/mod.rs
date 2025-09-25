//! Builder module root for programmatic scenario construction
//!
//! This file contains:
//! - Base builder traits and common building patterns
//! - Builder state management and validation
//! - Builder error handling and recovery
//! - Integration between different builder types
//! - Builder performance optimization and memory management
//!
//! Contributes to project by:
//! - Providing centralized access to all builder functionality
//! - Coordinating between different builder types and construction patterns
//! - Supporting type-safe scenario construction with compile-time validation
//! - Enabling fluent API patterns for ergonomic scenario building
//! - Facilitating builder debugging and construction monitoring

use crate::error::Error;

/// Builder-specific error type
pub type BuilderError = Error;

/// Builder result type
pub type BuilderResult<T> = Result<T, BuilderError>;

pub mod positions;
pub mod scenario;
pub mod fluent;
