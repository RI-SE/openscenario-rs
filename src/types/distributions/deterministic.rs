//! Deterministic distribution types for systematic parameter variation

use crate::error::Result;
use crate::types::basic::{OSString, Value};
use crate::types::distributions::{DistributionSampler, ValidateDistribution};
use serde::{Deserialize, Serialize};

/// Container for deterministic parameter distributions (matches XSD Deterministic type)
/// This version handles interspersed elements by collecting them all in one place
#[derive(Debug, Clone, PartialEq)]
pub struct Deterministic {
    pub single_distributions: Vec<DeterministicSingleParameterDistribution>,
    pub multi_distributions: Vec<DeterministicMultiParameterDistribution>,
}



impl serde::Serialize for Deterministic {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        
        let mut map = serializer.serialize_map(None)?;
        
        // Serialize single distributions
        for dist in &self.single_distributions {
            map.serialize_entry("DeterministicSingleParameterDistribution", dist)?;
        }
        
        // Serialize multi distributions  
        for dist in &self.multi_distributions {
            map.serialize_entry("DeterministicMultiParameterDistribution", dist)?;
        }
        
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for Deterministic {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;

        struct DeterministicVisitor;

        impl<'de> Visitor<'de> for DeterministicVisitor {
            type Value = Deterministic;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a map with distribution elements")
            }

            fn visit_map<M>(self, mut map: M) -> std::result::Result<Deterministic, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut single_distributions = Vec::new();
                let mut multi_distributions = Vec::new();

                // Process elements in the order they appear
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "DeterministicSingleParameterDistribution" => {
                            let dist: DeterministicSingleParameterDistribution = map.next_value()?;
                            single_distributions.push(dist);
                        }
                        "DeterministicMultiParameterDistribution" => {
                            let dist: DeterministicMultiParameterDistribution = map.next_value()?;
                            multi_distributions.push(dist);
                        }
                        _ => {
                            // Skip unknown fields
                            let _: serde::de::IgnoredAny = map.next_value()?;
                        }
                    }
                }

                Ok(Deterministic {
                    single_distributions,
                    multi_distributions,
                })
            }
        }

        deserializer.deserialize_map(DeterministicVisitor)
    }
}

impl Deterministic {
    /// Check if the container has no distributions
    pub fn is_empty(&self) -> bool {
        self.single_distributions.is_empty() && self.multi_distributions.is_empty()
    }
    
    /// Add a single parameter distribution
    pub fn add_single(&mut self, distribution: DeterministicSingleParameterDistribution) {
        self.single_distributions.push(distribution);
    }
    
    /// Add a multi parameter distribution
    pub fn add_multi(&mut self, distribution: DeterministicMultiParameterDistribution) {
        self.multi_distributions.push(distribution);
    }
    
    /// Get single distributions
    pub fn single_distributions(&self) -> impl Iterator<Item = &DeterministicSingleParameterDistribution> {
        self.single_distributions.iter()
    }
    
    /// Get multi distributions
    pub fn multi_distributions(&self) -> impl Iterator<Item = &DeterministicMultiParameterDistribution> {
        self.multi_distributions.iter()
    }
}



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
    #[serde(rename = "@parameterName")]
    pub parameter_name: OSString,
    #[serde(rename = "DistributionSet", skip_serializing_if = "Option::is_none")]
    pub distribution_set: Option<DistributionSet>,
    #[serde(rename = "DistributionRange", skip_serializing_if = "Option::is_none")]
    pub distribution_range: Option<DistributionRange>,
    #[serde(
        rename = "UserDefinedDistribution",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_defined_distribution: Option<crate::types::distributions::UserDefinedDistribution>,
}

/// Types of single parameter distributions (legacy enum - kept for backward compatibility)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum DeterministicSingleParameterDistributionType {
    DistributionSet(DistributionSet),
    DistributionRange(DistributionRange),
    UserDefinedDistribution(crate::types::distributions::UserDefinedDistribution),
}

impl DeterministicSingleParameterDistribution {
    /// Get the distribution type as an enum (for backward compatibility)
    pub fn distribution_type(&self) -> Option<DeterministicSingleParameterDistributionType> {
        if let Some(set) = &self.distribution_set {
            Some(DeterministicSingleParameterDistributionType::DistributionSet(set.clone()))
        } else if let Some(range) = &self.distribution_range {
            Some(DeterministicSingleParameterDistributionType::DistributionRange(range.clone()))
        } else if let Some(user_defined) = &self.user_defined_distribution {
            Some(
                DeterministicSingleParameterDistributionType::UserDefinedDistribution(
                    user_defined.clone(),
                ),
            )
        } else {
            None
        }
    }

    /// Check if this has a distribution set
    pub fn has_distribution_set(&self) -> bool {
        self.distribution_set.is_some()
    }

    /// Check if this has a distribution range
    pub fn has_distribution_range(&self) -> bool {
        self.distribution_range.is_some()
    }

    /// Check if this has a user defined distribution
    pub fn has_user_defined_distribution(&self) -> bool {
        self.user_defined_distribution.is_some()
    }
}

/// Multi-parameter deterministic distribution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeterministicMultiParameterDistribution {
    #[serde(rename = "ValueSetDistribution")]
    pub distribution_type: ValueSetDistribution,
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
    #[serde(rename = "Range")]
    pub range: crate::types::basic::Range,
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
        // Ensure exactly one distribution type is present
        let count = [
            self.distribution_set.is_some(),
            self.distribution_range.is_some(),
            self.user_defined_distribution.is_some(),
        ]
        .iter()
        .filter(|&&x| x)
        .count();

        if count != 1 {
            return Err(crate::error::Error::validation_error(
                "DeterministicSingleParameterDistribution",
                "Must have exactly one distribution type (set, range, or user-defined)",
            ));
        }

        // Validate the present distribution
        if let Some(dist) = &self.distribution_set {
            dist.validate()?;
        } else if let Some(dist) = &self.distribution_range {
            dist.validate()?;
        } else if let Some(dist) = &self.user_defined_distribution {
            dist.validate()?;
        }

        Ok(())
    }
}

impl Deterministic {
    /// Get total count of all distributions
    pub fn total_count(&self) -> usize {
        self.single_distributions.len() + self.multi_distributions.len()
    }
    
    /// Get unified iterator over all distributions
    pub fn all_distributions(&self) -> impl Iterator<Item = DeterministicParameterDistribution> + '_ {
        self.single_distributions
            .iter()
            .map(|d| DeterministicParameterDistribution::Single(d.clone()))
            .chain(
                self.multi_distributions
                    .iter()
                    .map(|d| DeterministicParameterDistribution::Multi(d.clone()))
            )
    }
}

impl Default for Deterministic {
    fn default() -> Self {
        Self {
            single_distributions: Vec::new(),
            multi_distributions: Vec::new(),
        }
    }
}

impl ValidateDistribution for Deterministic {
    fn validate(&self) -> Result<()> {
        for dist in &self.single_distributions {
            dist.validate()?;
        }
        for dist in &self.multi_distributions {
            dist.validate()?;
        }
        Ok(())
    }
}

impl ValidateDistribution for DeterministicMultiParameterDistribution {
    fn validate(&self) -> Result<()> {
        self.distribution_type.validate()
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

// Default implementations for distribution group support

impl Default for DeterministicParameterDistribution {
    fn default() -> Self {
        Self::Single(DeterministicSingleParameterDistribution::default())
    }
}

impl Default for DeterministicSingleParameterDistribution {
    fn default() -> Self {
        Self {
            parameter_name: Value::Literal("parameter".to_string()),
            distribution_set: Some(DistributionSet::default()),
            distribution_range: None,
            user_defined_distribution: None,
        }
    }
}

impl Default for DeterministicSingleParameterDistributionType {
    fn default() -> Self {
        Self::DistributionSet(DistributionSet::default())
    }
}

impl Default for DeterministicMultiParameterDistribution {
    fn default() -> Self {
        Self {
            distribution_type: ValueSetDistribution::default(),
        }
    }
}

impl Default for DistributionSet {
    fn default() -> Self {
        Self {
            elements: vec![DistributionSetElement::default()],
        }
    }
}

impl Default for DistributionSetElement {
    fn default() -> Self {
        Self {
            value: Value::Literal("0.0".to_string()),
        }
    }
}

impl Default for DistributionRange {
    fn default() -> Self {
        Self {
            step_width: Value::Literal("1.0".to_string()),
            range: crate::types::basic::Range::default(),
        }
    }
}

impl Default for ValueSetDistribution {
    fn default() -> Self {
        Self {
            parameter_value_sets: vec![ParameterValueSet::default()],
            assignment_author: None,
        }
    }
}

impl Default for ParameterValueSet {
    fn default() -> Self {
        Self {
            parameter_assignments: vec![ParameterAssignment::default()],
        }
    }
}

impl Default for ParameterAssignment {
    fn default() -> Self {
        Self {
            parameter_ref: "parameter".to_string(),
            value: Value::Literal("0.0".to_string()),
        }
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
        match &self.range.lower_limit {
            crate::types::basic::Value::Literal(val) => Ok(val.to_string()),
            crate::types::basic::Value::Parameter(_) => Err(crate::error::Error::validation_error(
                "sampling",
                "Cannot sample from parameterized distribution without parameter resolution",
            )),
            crate::types::basic::Value::Expression(_) => Err(crate::error::Error::validation_error(
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
