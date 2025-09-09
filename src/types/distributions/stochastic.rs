//! Stochastic distribution types for probabilistic parameter variation

use crate::error::Result;
use crate::types::basic::Value;
use crate::types::distributions::{DistributionSampler, ValidateDistribution};
use serde::{Deserialize, Serialize};

/// Wrapper for stochastic distributions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StochasticDistribution {
    #[serde(flatten)]
    pub distribution_type: StochasticDistributionType,
    #[serde(rename = "@parameterName")]
    pub parameter_name: Value<String>,
    #[serde(rename = "@randomSeed", skip_serializing_if = "Option::is_none")]
    pub random_seed: Option<Value<String>>,
}

/// Types of stochastic distributions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum StochasticDistributionType {
    ProbabilityDistributionSet(ProbabilityDistributionSet),
    NormalDistribution(NormalDistribution),
    LogNormalDistribution(LogNormalDistribution),
    UniformDistribution(UniformDistribution),
    PoissonDistribution(PoissonDistribution),
    Histogram(Histogram),
    UserDefinedDistribution(crate::types::distributions::UserDefinedDistribution),
}

/// Weighted discrete probability distribution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProbabilityDistributionSet {
    #[serde(rename = "Element")]
    pub elements: Vec<ProbabilityDistributionSetElement>,
}

/// Element in a probability distribution set
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProbabilityDistributionSetElement {
    #[serde(rename = "@value")]
    pub value: Value<String>,
    #[serde(rename = "@weight")]
    pub weight: Value<String>,
}

/// Normal (Gaussian) distribution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NormalDistribution {
    #[serde(rename = "@expectedValue")]
    pub expected_value: Value<String>,
    #[serde(rename = "@variance")]
    pub variance: Value<String>,
    #[serde(rename = "@range", skip_serializing_if = "Option::is_none")]
    pub range: Option<Range>,
}

/// Log-normal distribution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogNormalDistribution {
    #[serde(rename = "@expectedValue")]
    pub expected_value: Value<String>,
    #[serde(rename = "@variance")]
    pub variance: Value<String>,
    #[serde(rename = "@range", skip_serializing_if = "Option::is_none")]
    pub range: Option<Range>,
}

/// Uniform distribution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UniformDistribution {
    #[serde(rename = "@range")]
    pub range: Range,
}

/// Poisson distribution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PoissonDistribution {
    #[serde(rename = "@expectedValue")]
    pub expected_value: Value<String>,
    #[serde(rename = "@range", skip_serializing_if = "Option::is_none")]
    pub range: Option<Range>,
}

/// Histogram-based distribution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Histogram {
    #[serde(rename = "HistogramBin")]
    pub bins: Vec<HistogramBin>,
}

/// Bin in a histogram distribution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistogramBin {
    #[serde(rename = "@range")]
    pub range: Range,
    #[serde(rename = "@weight")]
    pub weight: Value<String>,
}

/// Range specification for distributions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Range {
    #[serde(rename = "@lowerLimit")]
    pub lower_limit: Value<String>,
    #[serde(rename = "@upperLimit")]
    pub upper_limit: Value<String>,
}

impl ValidateDistribution for StochasticDistribution {
    fn validate(&self) -> Result<()> {
        match &self.distribution_type {
            StochasticDistributionType::ProbabilityDistributionSet(dist) => dist.validate(),
            StochasticDistributionType::NormalDistribution(dist) => dist.validate(),
            StochasticDistributionType::LogNormalDistribution(dist) => dist.validate(),
            StochasticDistributionType::UniformDistribution(dist) => dist.validate(),
            StochasticDistributionType::PoissonDistribution(dist) => dist.validate(),
            StochasticDistributionType::Histogram(dist) => dist.validate(),
            StochasticDistributionType::UserDefinedDistribution(dist) => dist.validate(),
        }
    }
}

impl ValidateDistribution for ProbabilityDistributionSet {
    fn validate(&self) -> Result<()> {
        if self.elements.is_empty() {
            return Err(crate::error::Error::validation_error("elements",
                "ProbabilityDistributionSet must have at least one element"
            ));
        }
        Ok(())
    }
}

impl ValidateDistribution for NormalDistribution {
    fn validate(&self) -> Result<()> {
        // Basic validation - detailed validation would require parameter resolution
        Ok(())
    }
}

impl ValidateDistribution for LogNormalDistribution {
    fn validate(&self) -> Result<()> {
        // Basic validation - detailed validation would require parameter resolution
        Ok(())
    }
}

impl ValidateDistribution for UniformDistribution {
    fn validate(&self) -> Result<()> {
        self.range.validate()
    }
}

impl ValidateDistribution for PoissonDistribution {
    fn validate(&self) -> Result<()> {
        // Basic validation - detailed validation would require parameter resolution
        Ok(())
    }
}

impl ValidateDistribution for Histogram {
    fn validate(&self) -> Result<()> {
        if self.bins.is_empty() {
            return Err(crate::error::Error::validation_error("bins",
                "Histogram must have at least one bin"
            ));
        }
        
        for bin in &self.bins {
            bin.validate()?;
        }
        
        Ok(())
    }
}

impl ValidateDistribution for HistogramBin {
    fn validate(&self) -> Result<()> {
        self.range.validate()
    }
}

impl ValidateDistribution for Range {
    fn validate(&self) -> Result<()> {
        // Basic validation - detailed validation would require parameter resolution
        Ok(())
    }
}

impl DistributionSampler for ProbabilityDistributionSet {
    type Output = String;
    
    fn sample(&self) -> Result<Self::Output> {
        // For basic implementation, return the first element
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
            Err(crate::error::Error::validation_error("sampling",
                "Cannot sample from empty probability distribution set"
            ))
        }
    }
    
    fn is_deterministic(&self) -> bool {
        false
    }
}

impl DistributionSampler for UniformDistribution {
    type Output = String;
    
    fn sample(&self) -> Result<Self::Output> {
        // For basic implementation, return a placeholder
        match (&self.range.lower_limit, &self.range.upper_limit) {
            (Value::Literal(lower), Value::Literal(upper)) => {
                Ok(format!("uniform({}, {})", lower, upper))
            },
            _ => Err(crate::error::Error::validation_error("sampling",
                "Cannot sample from parameterized distribution without parameter resolution"
            )),
        }
    }
    
    fn is_deterministic(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_probability_distribution_set_validation() {
        let valid_set = ProbabilityDistributionSet {
            elements: vec![
                ProbabilityDistributionSetElement {
                    value: Value::Literal("A".to_string()),
                    weight: Value::Literal("0.6".to_string()),
                },
                ProbabilityDistributionSetElement {
                    value: Value::Literal("B".to_string()),
                    weight: Value::Literal("0.4".to_string()),
                },
            ],
        };
        assert!(valid_set.validate().is_ok());
        
        let empty_set = ProbabilityDistributionSet { elements: vec![] };
        assert!(empty_set.validate().is_err());
    }

    #[test]
    fn test_range_validation() {
        let valid_range = Range {
            lower_limit: Value::Literal("0.0".to_string()),
            upper_limit: Value::Literal("10.0".to_string()),
        };
        assert!(valid_range.validate().is_ok());
    }

    #[test]
    fn test_uniform_distribution_sampling() {
        let uniform = UniformDistribution {
            range: Range {
                lower_limit: Value::Literal("0.0".to_string()),
                upper_limit: Value::Literal("10.0".to_string()),
            },
        };
        
        let sample = uniform.sample().unwrap();
        assert!(sample.contains("uniform"));
        assert!(!uniform.is_deterministic());
    }

    #[test]
    fn test_histogram_validation() {
        let valid_histogram = Histogram {
            bins: vec![
                HistogramBin {
                    range: Range {
                        lower_limit: Value::Literal("0.0".to_string()),
                        upper_limit: Value::Literal("5.0".to_string()),
                    },
                    weight: Value::Literal("0.3".to_string()),
                },
            ],
        };
        assert!(valid_histogram.validate().is_ok());
        
        let empty_histogram = Histogram { bins: vec![] };
        assert!(empty_histogram.validate().is_err());
    }
}