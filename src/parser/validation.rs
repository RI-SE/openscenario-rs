//! Validation system for parsed OpenSCENARIO content
//!
//! This file contains:
//! - Validation framework with context-aware constraint checking
//! - Cross-reference validation (entity references, catalog references)
//! - Semantic validation beyond XML schema (logical consistency)
//! - Validation error collection and reporting
//! - Performance-optimized validation for large scenarios
//!
//! Contributes to project by:
//! - Ensuring parsed scenarios are logically consistent and valid
//! - Providing detailed error reporting for invalid scenarios
//! - Supporting incremental validation for interactive editing
//! - Enabling custom validation rules and domain-specific constraints
//! - Facilitating debugging through comprehensive validation feedback