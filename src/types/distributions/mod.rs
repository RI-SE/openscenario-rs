//! Distribution system module for scenario parameterization and variation
//!
//! This file contains:
//! - Base distribution traits and common distribution behaviors
//! - Distribution evaluation and sampling utilities
//! - Parameter value generation and constraint checking
//! - Cross-cutting distribution concerns (seeding, reproducibility)
//! - Integration with parameter resolution and scenario instantiation
//!
//! Contributes to project by:
//! - Organizing 18+ distribution types into logical categories
//! - Providing consistent framework for scenario parameterization
//! - Supporting both deterministic and stochastic scenario variations
//! - Enabling reproducible scenario generation through proper seeding
//! - Facilitating integration with parameter systems and scenario builders