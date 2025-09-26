//! Distribution system module for scenario parameterization and variation
//!
//! This module provides the core distribution framework for OpenSCENARIO parameter variation.
//! It supports both deterministic (systematic) and stochastic (probabilistic) parameter distributions.

use crate::error::Result;
use crate::types::entities::vehicle::File;
use serde::{Deserialize, Serialize};

// Import distribution types
pub use deterministic::{Deterministic, DeterministicParameterDistribution};
pub use stochastic::{Stochastic, StochasticDistribution};

pub mod deterministic;
pub mod stochastic;

pub use deterministic::*;
pub use stochastic::*;

/// Core parameter value distribution wrapper
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterValueDistribution {
    #[serde(rename = "ScenarioFile")]
    pub scenario_file: File,
    #[serde(rename = "Deterministic", skip_serializing_if = "Option::is_none")]
    pub deterministic: Option<Deterministic>,
    #[serde(rename = "Stochastic", skip_serializing_if = "Option::is_none")]
    pub stochastic: Option<Stochastic>,
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
            "enumeration",
            "Enumeration not supported for this distribution type",
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
    pub fn new_deterministic(scenario_file: File, deterministic: Deterministic) -> Self {
        Self {
            scenario_file,
            deterministic: Some(deterministic),
            stochastic: None,
        }
    }

    pub fn new_stochastic(scenario_file: File, stochastic: Stochastic) -> Self {
        Self {
            scenario_file,
            deterministic: None,
            stochastic: Some(stochastic),
        }
    }
}

impl Default for ParameterValueDistribution {
    fn default() -> Self {
        // Create a minimal valid deterministic distribution for default
        let single_dist = DeterministicSingleParameterDistribution::default();
        let deterministic = Deterministic {
            single_distributions: vec![single_dist],
            multi_distributions: vec![],
        };
        
        Self {
            scenario_file: File {
                filepath: "default.xosc".to_string(),
            },
            deterministic: Some(deterministic),
            stochastic: None,
        }
    }
}

impl Default for DistributionDefinition {
    fn default() -> Self {
        Self::Deterministic(DeterministicParameterDistribution::default())
    }
}

impl Default for UserDefinedDistribution {
    fn default() -> Self {
        Self {
            content: "default".to_string(),
            distribution_type: "default".to_string(),
        }
    }
}

impl ValidateDistribution for ParameterValueDistribution {
    fn validate(&self) -> Result<()> {
        if let Some(det) = &self.deterministic {
            det.validate()
        } else if let Some(stoc) = &self.stochastic {
            stoc.validate()
        } else {
            Err(crate::error::Error::validation_error(
                "ParameterValueDistribution",
                "Must have either deterministic or stochastic distribution",
            ))
        }
    }
}

impl ValidateDistribution for UserDefinedDistribution {
    fn validate(&self) -> Result<()> {
        if self.distribution_type.trim().is_empty() {
            return Err(crate::error::Error::validation_error(
                "type",
                "UserDefinedDistribution type cannot be empty",
            ));
        }
        Ok(())
    }
}

// Phase 3 XSD Group Implementations - Distribution Groups
// All 5 groups are trivial wrappers around existing complete infrastructure

/// DistributionDefinition group - XSD group wrapper for deterministic/stochastic choice
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DistributionDefinitionGroup {
    #[serde(rename = "Deterministic")]
    Deterministic(DeterministicParameterDistribution),
    #[serde(rename = "Stochastic")]
    Stochastic(StochasticDistribution),
}

/// DeterministicParameterDistribution group - XSD group wrapper for single/multi parameter choice
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeterministicParameterDistributionGroup {
    #[serde(rename = "DeterministicMultiParameterDistribution")]
    DeterministicMultiParameterDistribution(DeterministicMultiParameterDistribution),
    #[serde(rename = "DeterministicSingleParameterDistribution")]
    DeterministicSingleParameterDistribution(DeterministicSingleParameterDistribution),
}

/// DeterministicSingleParameterDistributionType group - XSD group wrapper for set/range/user-defined choice
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeterministicSingleParameterDistributionTypeGroup {
    #[serde(rename = "DistributionSet")]
    DistributionSet(DistributionSet),
    #[serde(rename = "DistributionRange")]
    DistributionRange(DistributionRange),
    #[serde(rename = "UserDefinedDistribution")]
    UserDefinedDistribution(UserDefinedDistribution),
}

/// DeterministicMultiParameterDistributionType group - XSD group wrapper for value set sequence
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeterministicMultiParameterDistributionTypeGroup {
    #[serde(rename = "ValueSetDistribution")]
    pub value_set_distribution: ValueSetDistribution,
}

/// ParameterValueDistributionDefinition group - XSD group wrapper for parameter value distribution sequence
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterValueDistributionDefinitionGroup {
    #[serde(rename = "ParameterValueDistribution")]
    pub parameter_value_distribution: ParameterValueDistribution,
}

// Default implementations for all groups

impl Default for DistributionDefinitionGroup {
    fn default() -> Self {
        Self::Deterministic(DeterministicParameterDistribution::default())
    }
}

impl Default for DeterministicParameterDistributionGroup {
    fn default() -> Self {
        Self::DeterministicSingleParameterDistribution(
            DeterministicSingleParameterDistribution::default(),
        )
    }
}

impl Default for DeterministicSingleParameterDistributionTypeGroup {
    fn default() -> Self {
        Self::DistributionSet(DistributionSet::default())
    }
}

impl Default for DeterministicMultiParameterDistributionTypeGroup {
    fn default() -> Self {
        Self {
            value_set_distribution: ValueSetDistribution::default(),
        }
    }
}

impl Default for ParameterValueDistributionDefinitionGroup {
    fn default() -> Self {
        Self {
            parameter_value_distribution: ParameterValueDistribution::default(),
        }
    }
}

// Helper implementations for ergonomic group usage

impl DistributionDefinitionGroup {
    /// Create deterministic distribution group
    pub fn deterministic(dist: DeterministicParameterDistribution) -> Self {
        Self::Deterministic(dist)
    }

    /// Create stochastic distribution group
    pub fn stochastic(dist: StochasticDistribution) -> Self {
        Self::Stochastic(dist)
    }

    /// Check if this is a deterministic distribution
    pub fn is_deterministic(&self) -> bool {
        matches!(self, Self::Deterministic(_))
    }

    /// Check if this is a stochastic distribution
    pub fn is_stochastic(&self) -> bool {
        matches!(self, Self::Stochastic(_))
    }

    /// Get deterministic distribution if available
    pub fn as_deterministic(&self) -> Option<&DeterministicParameterDistribution> {
        match self {
            Self::Deterministic(dist) => Some(dist),
            _ => None,
        }
    }

    /// Get stochastic distribution if available
    pub fn as_stochastic(&self) -> Option<&StochasticDistribution> {
        match self {
            Self::Stochastic(dist) => Some(dist),
            _ => None,
        }
    }
}

impl DeterministicParameterDistributionGroup {
    /// Create single parameter distribution group
    pub fn single(dist: DeterministicSingleParameterDistribution) -> Self {
        Self::DeterministicSingleParameterDistribution(dist)
    }

    /// Create multi parameter distribution group
    pub fn multi(dist: DeterministicMultiParameterDistribution) -> Self {
        Self::DeterministicMultiParameterDistribution(dist)
    }

    /// Check if this is a single parameter distribution
    pub fn is_single(&self) -> bool {
        matches!(self, Self::DeterministicSingleParameterDistribution(_))
    }

    /// Check if this is a multi parameter distribution
    pub fn is_multi(&self) -> bool {
        matches!(self, Self::DeterministicMultiParameterDistribution(_))
    }
}

impl DeterministicSingleParameterDistributionTypeGroup {
    /// Create distribution set group
    pub fn distribution_set(set: DistributionSet) -> Self {
        Self::DistributionSet(set)
    }

    /// Create distribution range group
    pub fn distribution_range(range: DistributionRange) -> Self {
        Self::DistributionRange(range)
    }

    /// Create user defined distribution group
    pub fn user_defined(dist: UserDefinedDistribution) -> Self {
        Self::UserDefinedDistribution(dist)
    }

    /// Check if this is a distribution set
    pub fn is_set(&self) -> bool {
        matches!(self, Self::DistributionSet(_))
    }

    /// Check if this is a distribution range
    pub fn is_range(&self) -> bool {
        matches!(self, Self::DistributionRange(_))
    }

    /// Check if this is a user defined distribution
    pub fn is_user_defined(&self) -> bool {
        matches!(self, Self::UserDefinedDistribution(_))
    }
}

impl DeterministicMultiParameterDistributionTypeGroup {
    /// Create new multi parameter distribution type group
    pub fn new(value_set_distribution: ValueSetDistribution) -> Self {
        Self {
            value_set_distribution,
        }
    }

    /// Get the value set distribution
    pub fn value_set_distribution(&self) -> &ValueSetDistribution {
        &self.value_set_distribution
    }
}

impl ParameterValueDistributionDefinitionGroup {
    /// Create new parameter value distribution definition group
    pub fn new(parameter_value_distribution: ParameterValueDistribution) -> Self {
        Self {
            parameter_value_distribution,
        }
    }

    /// Get the parameter value distribution
    pub fn parameter_value_distribution(&self) -> &ParameterValueDistribution {
        &self.parameter_value_distribution
    }
}

// Validation trait implementations - delegate to underlying types

impl ValidateDistribution for DistributionDefinitionGroup {
    fn validate(&self) -> Result<()> {
        match self {
            Self::Deterministic(dist) => dist.validate(),
            Self::Stochastic(dist) => dist.validate(),
        }
    }
}

impl ValidateDistribution for DeterministicParameterDistributionGroup {
    fn validate(&self) -> Result<()> {
        match self {
            Self::DeterministicSingleParameterDistribution(dist) => dist.validate(),
            Self::DeterministicMultiParameterDistribution(dist) => dist.validate(),
        }
    }
}

impl ValidateDistribution for DeterministicSingleParameterDistributionTypeGroup {
    fn validate(&self) -> Result<()> {
        match self {
            Self::DistributionSet(dist) => dist.validate(),
            Self::DistributionRange(dist) => dist.validate(),
            Self::UserDefinedDistribution(dist) => dist.validate(),
        }
    }
}

impl ValidateDistribution for DeterministicMultiParameterDistributionTypeGroup {
    fn validate(&self) -> Result<()> {
        self.value_set_distribution.validate()
    }
}

impl ValidateDistribution for ParameterValueDistributionDefinitionGroup {
    fn validate(&self) -> Result<()> {
        self.parameter_value_distribution.validate()
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

        let single_param_dist = DeterministicSingleParameterDistribution {
            parameter_name: Value::Literal("speed".to_string()),
            distribution_set: Some(dist_set),
            distribution_range: None,
            user_defined_distribution: None,
        };

        let det_dist = Deterministic {
            single_distributions: vec![single_param_dist],
            multi_distributions: vec![],
        };

        let scenario_file = File {
            filepath: "test.xosc".to_string(),
        };
        let param_dist = ParameterValueDistribution::new_deterministic(scenario_file, det_dist);

        assert!(param_dist.deterministic.is_some());
        assert!(param_dist.stochastic.is_none());
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

    // Tests for Phase 3 Distribution Groups

    #[test]
    fn test_distribution_definition_group_creation() {
        let det_dist = DeterministicParameterDistribution::default();
        let group = DistributionDefinitionGroup::deterministic(det_dist);

        assert!(group.is_deterministic());
        assert!(!group.is_stochastic());
        assert!(group.as_deterministic().is_some());
        assert!(group.as_stochastic().is_none());

        let default_group = DistributionDefinitionGroup::default();
        assert!(default_group.is_deterministic());
    }

    #[test]
    fn test_distribution_definition_group_validation() {
        let group = DistributionDefinitionGroup::default();
        assert!(group.validate().is_ok());
    }

    #[test]
    fn test_deterministic_parameter_distribution_group_creation() {
        let single_dist = DeterministicSingleParameterDistribution::default();
        let group = DeterministicParameterDistributionGroup::single(single_dist);

        assert!(group.is_single());
        assert!(!group.is_multi());

        let multi_dist = DeterministicMultiParameterDistribution::default();
        let multi_group = DeterministicParameterDistributionGroup::multi(multi_dist);

        assert!(!multi_group.is_single());
        assert!(multi_group.is_multi());
    }

    #[test]
    fn test_deterministic_parameter_distribution_group_validation() {
        let group = DeterministicParameterDistributionGroup::default();
        assert!(group.validate().is_ok());
    }

    #[test]
    fn test_deterministic_single_parameter_distribution_type_group() {
        let dist_set = DistributionSet::default();
        let group = DeterministicSingleParameterDistributionTypeGroup::distribution_set(dist_set);

        assert!(group.is_set());
        assert!(!group.is_range());
        assert!(!group.is_user_defined());

        let dist_range = DistributionRange::default();
        let range_group =
            DeterministicSingleParameterDistributionTypeGroup::distribution_range(dist_range);

        assert!(!range_group.is_set());
        assert!(range_group.is_range());
        assert!(!range_group.is_user_defined());

        let user_dist = UserDefinedDistribution::default();
        let user_group = DeterministicSingleParameterDistributionTypeGroup::user_defined(user_dist);

        assert!(!user_group.is_set());
        assert!(!user_group.is_range());
        assert!(user_group.is_user_defined());

        let default_group = DeterministicSingleParameterDistributionTypeGroup::default();
        assert!(default_group.is_set());
        assert!(default_group.validate().is_ok());
    }

    #[test]
    fn test_deterministic_multi_parameter_distribution_type_group() {
        let value_set_dist = ValueSetDistribution::default();
        let group = DeterministicMultiParameterDistributionTypeGroup::new(value_set_dist);

        assert!(!group
            .value_set_distribution()
            .parameter_value_sets
            .is_empty());
        assert!(group.validate().is_ok());

        let default_group = DeterministicMultiParameterDistributionTypeGroup::default();
        assert!(default_group.validate().is_ok());
    }

#[test]
    fn test_parameter_value_distribution_definition_group() {
        let param_value_dist = ParameterValueDistribution::default();
        let group = ParameterValueDistributionDefinitionGroup::new(param_value_dist);
        
        assert!(group.parameter_value_distribution().deterministic.is_some());
        assert!(group.validate().is_ok());
        
        let default_group = ParameterValueDistributionDefinitionGroup::default();
        assert!(default_group.validate().is_ok());
    }

    #[test]
    fn test_all_distribution_groups_serialization_round_trip() {
        // Test DistributionDefinitionGroup
        let dist_def_group = DistributionDefinitionGroup::default();
        let serialized = serde_json::to_string(&dist_def_group)
            .expect("Failed to serialize DistributionDefinitionGroup");
        let deserialized: DistributionDefinitionGroup = serde_json::from_str(&serialized)
            .expect("Failed to deserialize DistributionDefinitionGroup");
        assert_eq!(dist_def_group, deserialized);

        // Test DeterministicParameterDistributionGroup
        let det_param_group = DeterministicParameterDistributionGroup::default();
        let serialized = serde_json::to_string(&det_param_group)
            .expect("Failed to serialize DeterministicParameterDistributionGroup");
        let deserialized: DeterministicParameterDistributionGroup =
            serde_json::from_str(&serialized)
                .expect("Failed to deserialize DeterministicParameterDistributionGroup");
        assert_eq!(det_param_group, deserialized);

        // Test DeterministicSingleParameterDistributionTypeGroup
        let det_single_group = DeterministicSingleParameterDistributionTypeGroup::default();
        let serialized = serde_json::to_string(&det_single_group)
            .expect("Failed to serialize DeterministicSingleParameterDistributionTypeGroup");
        let deserialized: DeterministicSingleParameterDistributionTypeGroup =
            serde_json::from_str(&serialized)
                .expect("Failed to deserialize DeterministicSingleParameterDistributionTypeGroup");
        assert_eq!(det_single_group, deserialized);

        // Test DeterministicMultiParameterDistributionTypeGroup
        let det_multi_group = DeterministicMultiParameterDistributionTypeGroup::default();
        let serialized = serde_json::to_string(&det_multi_group)
            .expect("Failed to serialize DeterministicMultiParameterDistributionTypeGroup");
        let deserialized: DeterministicMultiParameterDistributionTypeGroup =
            serde_json::from_str(&serialized)
                .expect("Failed to deserialize DeterministicMultiParameterDistributionTypeGroup");
        assert_eq!(det_multi_group, deserialized);

        // Test ParameterValueDistributionDefinitionGroup
        let param_value_group = ParameterValueDistributionDefinitionGroup::default();
        let serialized = serde_json::to_string(&param_value_group)
            .expect("Failed to serialize ParameterValueDistributionDefinitionGroup");
        println!("Serialized JSON: {}", serialized);
        
        // Try to deserialize step by step
        let deserialized_result: std::result::Result<ParameterValueDistributionDefinitionGroup, serde_json::Error> = 
            serde_json::from_str(&serialized);
            
        match deserialized_result {
            Ok(deserialized) => {
                assert_eq!(param_value_group, deserialized);
            }
            Err(e) => {
                println!("Deserialization error: {:?}", e);
                panic!("Failed to deserialize ParameterValueDistributionDefinitionGroup: {}", e);
            }
        }
    }

    #[test]
    fn test_distribution_groups_helper_methods_comprehensive() {
        // Test all helper methods for DistributionDefinitionGroup
        let det_dist = DeterministicParameterDistribution::default();
        let det_group = DistributionDefinitionGroup::deterministic(det_dist.clone());

        assert!(det_group.is_deterministic());
        assert!(!det_group.is_stochastic());
        assert_eq!(det_group.as_deterministic().unwrap(), &det_dist);
        assert!(det_group.as_stochastic().is_none());

        // Test all helper methods for DeterministicParameterDistributionGroup
        let single_dist = DeterministicSingleParameterDistribution::default();
        let single_group = DeterministicParameterDistributionGroup::single(single_dist.clone());

        assert!(single_group.is_single());
        assert!(!single_group.is_multi());

        let multi_dist = DeterministicMultiParameterDistribution::default();
        let multi_group = DeterministicParameterDistributionGroup::multi(multi_dist.clone());

        assert!(!multi_group.is_single());
        assert!(multi_group.is_multi());

        // Test all helper methods for DeterministicSingleParameterDistributionTypeGroup
        let set = DistributionSet::default();
        let set_group = DeterministicSingleParameterDistributionTypeGroup::distribution_set(set);
        assert!(set_group.is_set());

        let range = DistributionRange::default();
        let range_group =
            DeterministicSingleParameterDistributionTypeGroup::distribution_range(range);
        assert!(range_group.is_range());

        let user_defined = UserDefinedDistribution::default();
        let user_group =
            DeterministicSingleParameterDistributionTypeGroup::user_defined(user_defined);
        assert!(user_group.is_user_defined());
    }
}
