//! Deterministic distribution types for systematic parameter variation

use crate::error::Result;
use crate::types::basic::Value;
use crate::types::distributions::{DistributionSampler, ValidateDistribution};
use serde::{Deserialize, Serialize};

use crate::types::basic::OSString;
/// Wrapper for deterministic parameter distributions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum DeterministicParameterDistribution {
    Single(DeterministicSingleParameterDistribution),
    Multi(DeterministicMultiParameterDistribution),
}

/// Single parameter deterministic distribution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeterministicSingleParameterDistribution {
    #[serde(flatten)]
    pub distribution_type: DeterministicSingleParameterDistributionType,
    #[serde(rename = "@parameterName")]
    pub parameter_name: OSString,
}

/// Types of single parameter distributions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum DeterministicSingleParameterDistributionType {
    DistributionSet(DistributionSet),
    DistributionRange(DistributionRange),
    UserDefinedDistribution(crate::types::distributions::UserDefinedDistribution),
}

/// Multi-parameter deterministic distribution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeterministicMultiParameterDistribution {
    #[serde(flatten)]
    pub distribution_type: DeterministicMultiParameterDistributionType,
}

/// Types of multi-parameter distributions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum DeterministicMultiParameterDistributionType {
    ValueSetDistribution(ValueSetDistribution),
}

/// Discrete value set distribution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DistributionSet {
    #[serde(rename = "Element")]
    pub elements: Vec<DistributionSetElement>,
}

/// Element in a distribution set
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DistributionSetElement {
    #[serde(rename = "@value")]
    pub value: OSString,
}

/// Continuous range distribution with step size
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DistributionRange {
    #[serde(rename = "@stepWidth")]
    pub step_width: OSString,
    #[serde(rename = "@lowerLimit")]
    pub lower_limit: OSString,
    #[serde(rename = "@upperLimit")]
    pub upper_limit: OSString,
}

/// Multi-parameter value set distribution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValueSetDistribution {
    #[serde(rename = "ParameterValueSet")]
    pub parameter_value_sets: Vec<ParameterValueSet>,
    #[serde(rename = "@assignmentAuthor", skip_serializing_if = "Option::is_none")]
    pub assignment_author: Option<String>,
}

/// Set of parameter assignments
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterValueSet {
    #[serde(rename = "ParameterAssignment")]
    pub parameter_assignments: Vec<ParameterAssignment>,
}

/// Individual parameter assignment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterAssignment {
    #[serde(rename = "@parameterRef")]
    pub parameter_ref: String,
    #[serde(rename = "@value")]
    pub value: OSString,
}

impl ValidateDistribution for DeterministicParameterDistribution {
    fn validate(&self) -> Result<()> {
        match self {
            DeterministicParameterDistribution::Single(dist) => dist.validate(),
            DeterministicParameterDistribution::Multi(dist) => dist.validate(),
        }
    }
}

impl ValidateDistribution for DeterministicSingleParameterDistribution {
    fn validate(&self) -> Result<()> {
        match &self.distribution_type {
            DeterministicSingleParameterDistributionType::DistributionSet(dist) => dist.validate(),
            DeterministicSingleParameterDistributionType::DistributionRange(dist) => {
                dist.validate()
            }
            DeterministicSingleParameterDistributionType::UserDefinedDistribution(dist) => {
                dist.validate()
            }
        }
    }
}

impl ValidateDistribution for DeterministicMultiParameterDistribution {
    fn validate(&self) -> Result<()> {
        match &self.distribution_type {
            DeterministicMultiParameterDistributionType::ValueSetDistribution(dist) => {
                dist.validate()
            }
        }
    }
}

impl ValidateDistribution for DistributionSet {
    fn validate(&self) -> Result<()> {
        if self.elements.is_empty() {
            return Err(crate::error::Error::validation_error(
                "elements",
                "DistributionSet must have at least one element",
            ));
        }
        Ok(())
    }
}

impl ValidateDistribution for DistributionRange {
    fn validate(&self) -> Result<()> {
        // Basic validation - for more detailed validation, parameter resolution would be needed
        Ok(())
    }
}

impl ValidateDistribution for ValueSetDistribution {
    fn validate(&self) -> Result<()> {
        if self.parameter_value_sets.is_empty() {
            return Err(crate::error::Error::validation_error(
                "parameter_value_sets",
                "ValueSetDistribution must have at least one parameter value set",
            ));
        }

        for value_set in &self.parameter_value_sets {
            value_set.validate()?;
        }
        Ok(())
    }
}

impl ValidateDistribution for ParameterValueSet {
    fn validate(&self) -> Result<()> {
        if self.parameter_assignments.is_empty() {
            return Err(crate::error::Error::validation_error(
                "parameter_assignments",
                "ParameterValueSet must have at least one parameter assignment",
            ));
        }

        // Check for duplicate parameter references
        let mut param_refs = std::collections::HashSet::new();
        for assignment in &self.parameter_assignments {
            if !param_refs.insert(&assignment.parameter_ref) {
                return Err(crate::error::Error::validation_error(
                    "parameter_assignments",
                    "Duplicate parameter reference found",
                ));
            }
        }
        Ok(())
    }
}

impl DistributionSampler for DistributionSet {
    type Output = String;

    fn sample(&self) -> Result<Self::Output> {
        if let Some(first_element) = self.elements.first() {
            match &first_element.value {
                Value::Literal(val) => Ok(val.clone()),
                Value::Parameter(_) => Err(crate::error::Error::validation_error("sampling",
                    "Cannot sample from parameterized distribution without parameter resolution"
                )),
                Value::Expression(_) => Err(crate::error::Error::validation_error("sampling",
                    "Cannot sample from expression-based distribution without expression evaluation"
                )),
            }
        } else {
            Err(crate::error::Error::validation_error(
                "sampling",
                "Cannot sample from empty distribution set",
            ))
        }
    }

    fn enumerate(&self) -> Result<Vec<Self::Output>> {
        self.elements
            .iter()
            .map(|elem| match &elem.value {
                Value::Literal(val) => Ok(val.clone()),
                Value::Parameter(_) => Err(crate::error::Error::validation_error(
                    "enumeration",
                    "Cannot enumerate parameterized distribution without parameter resolution",
                )),
                Value::Expression(_) => Err(crate::error::Error::validation_error(
                    "enumeration",
                    "Cannot enumerate expression-based distribution without expression evaluation",
                )),
            })
            .collect()
    }

    fn is_deterministic(&self) -> bool {
        true
    }
}

impl DistributionSampler for DistributionRange {
    type Output = String;

    fn sample(&self) -> Result<Self::Output> {
        match &self.lower_limit {
            Value::Literal(val) => Ok(val.clone()),
            Value::Parameter(_) => Err(crate::error::Error::validation_error(
                "sampling",
                "Cannot sample from parameterized distribution without parameter resolution",
            )),
            Value::Expression(_) => Err(crate::error::Error::validation_error(
                "sampling",
                "Cannot sample from expression-based distribution without expression evaluation",
            )),
        }
    }

    fn is_deterministic(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distribution_set_validation() {
        let valid_set = DistributionSet {
            elements: vec![
                DistributionSetElement {
                    value: Value::Literal("10.0".to_string()),
                },
                DistributionSetElement {
                    value: Value::Literal("20.0".to_string()),
                },
            ],
        };
        assert!(valid_set.validate().is_ok());

        let empty_set = DistributionSet { elements: vec![] };
        assert!(empty_set.validate().is_err());
    }

    #[test]
    fn test_distribution_set_sampling() {
        let dist_set = DistributionSet {
            elements: vec![
                DistributionSetElement {
                    value: Value::Literal("10.0".to_string()),
                },
                DistributionSetElement {
                    value: Value::Literal("20.0".to_string()),
                },
            ],
        };

        assert!(dist_set.sample().is_ok());
        let values = dist_set.enumerate().unwrap();
        assert_eq!(values.len(), 2);
        assert_eq!(values[0], "10.0");
        assert_eq!(values[1], "20.0");
        assert!(dist_set.is_deterministic());
    }

    #[test]
    fn test_parameter_value_set_validation() {
        let valid_set = ParameterValueSet {
            parameter_assignments: vec![
                ParameterAssignment {
                    parameter_ref: "speed".to_string(),
                    value: Value::Literal("30.0".to_string()),
                },
                ParameterAssignment {
                    parameter_ref: "position".to_string(),
                    value: Value::Literal("100.0".to_string()),
                },
            ],
        };
        assert!(valid_set.validate().is_ok());

        let duplicate_set = ParameterValueSet {
            parameter_assignments: vec![
                ParameterAssignment {
                    parameter_ref: "speed".to_string(),
                    value: Value::Literal("30.0".to_string()),
                },
                ParameterAssignment {
                    parameter_ref: "speed".to_string(),
                    value: Value::Literal("40.0".to_string()),
                },
            ],
        };
        assert!(duplicate_set.validate().is_err());
    }
}

