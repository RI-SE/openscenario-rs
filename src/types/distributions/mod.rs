//! Distribution system module for scenario parameterization and variation
//!
//! This module provides the core distribution framework for OpenSCENARIO parameter variation.
//! It supports both deterministic (systematic) and stochastic (probabilistic) parameter distributions.

use crate::error::Result;
use serde::{Deserialize, Serialize};

pub mod deterministic;
pub mod stochastic;

pub use deterministic::*;
pub use stochastic::*;

/// Core parameter value distribution wrapper
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterValueDistribution {
    pub distribution_definition: DistributionDefinition,
    #[serde(rename = "@definitionTypeHint", skip_serializing_if = "Option::is_none")]
    pub definition_type_hint: Option<String>,
    #[serde(rename = "@distributionSet", skip_serializing_if = "Option::is_none")]
    pub distribution_set: Option<String>,
}

/// Union of all distribution types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum DistributionDefinition {
    Deterministic(DeterministicParameterDistribution),
    Stochastic(StochasticDistribution),
    UserDefined(UserDefinedDistribution),
}

/// User-defined distribution for custom parameter distributions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserDefinedDistribution {
    pub content: String,
    #[serde(rename = "@type")]
    pub distribution_type: String,
}

/// Base trait for parameter distribution evaluation
pub trait DistributionSampler {
    type Output;
    
    /// Sample a value from the distribution
    fn sample(&self) -> Result<Self::Output>;
    
    /// Get all possible values (for deterministic distributions)
    fn enumerate(&self) -> Result<Vec<Self::Output>> {
        Err(crate::error::Error::validation_error(
            "enumeration", "Enumeration not supported for this distribution type"
        ))
    }
    
    /// Check if the distribution is deterministic
    fn is_deterministic(&self) -> bool;
}

/// Trait for validating distribution parameters
pub trait ValidateDistribution {
    fn validate(&self) -> Result<()>;
}

impl ParameterValueDistribution {
    pub fn new_deterministic(dist: DeterministicParameterDistribution) -> Self {
        Self {
            distribution_definition: DistributionDefinition::Deterministic(dist),
            definition_type_hint: None,
            distribution_set: None,
        }
    }
    
    pub fn new_stochastic(dist: StochasticDistribution) -> Self {
        Self {
            distribution_definition: DistributionDefinition::Stochastic(dist),
            definition_type_hint: None,
            distribution_set: None,
        }
    }
    
    pub fn new_user_defined(dist: UserDefinedDistribution) -> Self {
        Self {
            distribution_definition: DistributionDefinition::UserDefined(dist),
            definition_type_hint: None,
            distribution_set: None,
        }
    }
}

impl ValidateDistribution for ParameterValueDistribution {
    fn validate(&self) -> Result<()> {
        match &self.distribution_definition {
            DistributionDefinition::Deterministic(dist) => dist.validate(),
            DistributionDefinition::Stochastic(dist) => dist.validate(),
            DistributionDefinition::UserDefined(dist) => dist.validate(),
        }
    }
}

impl ValidateDistribution for UserDefinedDistribution {
    fn validate(&self) -> Result<()> {
        if self.distribution_type.trim().is_empty() {
            return Err(crate::error::Error::validation_error(
                "type", "UserDefinedDistribution type cannot be empty"
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::basic::Value;
    use crate::types::distributions::deterministic::*;

    #[test]
    fn test_parameter_value_distribution_creation() {
        let dist_set = DistributionSet {
            elements: vec![DistributionSetElement {
                value: Value::Literal("10.0".to_string()),
            }],
        };
        
        let det_dist = DeterministicParameterDistribution::Single(
            DeterministicSingleParameterDistribution {
                distribution_type: DeterministicSingleParameterDistributionType::DistributionSet(dist_set),
                parameter_name: Value::Literal("speed".to_string()),
            }
        );
        
        let param_dist = ParameterValueDistribution::new_deterministic(det_dist);
        
        assert!(matches!(
            param_dist.distribution_definition,
            DistributionDefinition::Deterministic(_)
        ));
    }

    #[test]
    fn test_user_defined_distribution_validation() {
        let valid_dist = UserDefinedDistribution {
            content: "custom content".to_string(),
            distribution_type: "custom".to_string(),
        };
        assert!(valid_dist.validate().is_ok());
        
        let invalid_dist = UserDefinedDistribution {
            content: "content".to_string(),
            distribution_type: "".to_string(),
        };
        assert!(invalid_dist.validate().is_err());
    }
}