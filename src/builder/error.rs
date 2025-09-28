//! Builder-specific error extensions and utilities
//!
//! This module provides specialized error types for builder operations,
//! extending the existing error system with builder-specific context.

use thiserror::Error;

/// Builder-specific error types with detailed context
#[derive(Debug, Error)]
pub enum BuilderError {
    /// Validation failed during builder operation
    #[error("Validation error: {message}. Suggestion: {suggestion}")]
    ValidationError { 
        message: String, 
        suggestion: String 
    },
    
    /// Required field was not set before building
    #[error("Missing required field: {field}. Call {suggestion} first")]
    MissingField { 
        field: String, 
        suggestion: String 
    },
    
    /// Invalid entity reference in action or condition
    #[error("Invalid entity reference: '{entity}'. Available entities: {available}")]
    InvalidEntityRef { 
        entity: String, 
        available: String 
    },
    
    /// Constraint violation (range, compatibility, etc.)
    #[error("Constraint violation: {constraint}. Details: {details}")]
    ConstraintViolation { 
        constraint: String, 
        details: String 
    },
    
    /// Integration error with existing OpenSCENARIO systems
    #[error("OpenSCENARIO error: {0}")]
    OpenScenarioError(#[from] crate::error::Error),
}

impl BuilderError {
    /// Create a validation error with suggestion
    pub fn validation_error(message: &str) -> Self {
        Self::ValidationError { 
            message: message.to_string(),
            suggestion: "Check the documentation for valid values".to_string()
        }
    }
    
    /// Create a validation error with custom suggestion  
    pub fn validation_error_with_suggestion(message: &str, suggestion: &str) -> Self {
        Self::ValidationError {
            message: message.to_string(),
            suggestion: suggestion.to_string()
        }
    }
    
    /// Create a missing field error
    pub fn missing_field(field: &str, suggestion: &str) -> Self {
        Self::MissingField { 
            field: field.to_string(),
            suggestion: suggestion.to_string()
        }
    }
    
    /// Create an invalid entity reference error
    pub fn invalid_entity_ref(entity: &str, available: &[String]) -> Self {
        Self::InvalidEntityRef {
            entity: entity.to_string(),
            available: available.join(", ")
        }
    }
    
    /// Create a constraint violation error
    pub fn constraint_violation(constraint: &str, details: &str) -> Self {
        Self::ConstraintViolation {
            constraint: constraint.to_string(),
            details: details.to_string()
        }
    }
}

/// Result type for builder operations
pub type BuilderResult<T> = Result<T, BuilderError>;