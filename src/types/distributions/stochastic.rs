//! Stochastic distribution types for probabilistic parameter variation

use crate::error::Result;
use crate::types::basic::{MinVec, OSString, UnsignedInt, Value};
use crate::types::distributions::{DistributionSampler, ValidateDistribution};
use serde::{Deserialize, Serialize};

/// Container for stochastic distributions
///
/// XSD `Stochastic` (`:2081-2087`): sequence of `StochasticDistribution`,
/// `maxOccurs="unbounded"`, no `minOccurs`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stochastic {
    #[serde(rename = "StochasticDistribution")]
    pub distributions: MinVec<StochasticDistribution, 1>,
    #[serde(rename = "@numberOfTestRuns")]
    pub number_of_test_runs: UnsignedInt,
    #[serde(rename = "@randomSeed", skip_serializing_if = "Option::is_none")]
    pub random_seed: Option<Value<f64>>,
}

/// Wrapper for stochastic distributions
///
/// XSD `StochasticDistribution` (`Schema/OpenSCENARIO.xsd:2088-2093`) is a
/// `StochasticDistributionType` choice group plus a sibling
/// `@parameterName` attribute. `$value` takes the element name from the
/// serialized variant and reads the chosen branch straight from the live
/// parser. `#[serde(flatten)]` cannot do that: it buffers the element's
/// children into a map through `deserialize_any` before the enum picks a
/// variant, and a `Vec<T>` replayed out of that buffer fails with `invalid
/// type: map, expected a sequence`. Two branches here hold a `Vec` directly,
/// `ProbabilityDistributionSet.elements` and `Histogram.bins`, so no
/// document using either could be parsed at all under `flatten`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StochasticDistribution {
    #[serde(rename = "$value")]
    pub distribution_type: StochasticDistributionType,
    #[serde(rename = "@parameterName")]
    pub parameter_name: OSString,
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
///
/// XSD `ProbabilityDistributionSet` (`:1793-1797`): sequence of `Element`
/// (`ProbabilityDistributionSetElement`), `maxOccurs="unbounded"`, no `minOccurs`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProbabilityDistributionSet {
    #[serde(rename = "Element")]
    pub elements: MinVec<ProbabilityDistributionSetElement, 1>,
}

/// Element in a probability distribution set
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProbabilityDistributionSetElement {
    #[serde(rename = "@value")]
    pub value: OSString,
    #[serde(rename = "@weight")]
    pub weight: OSString,
}

/// Normal (Gaussian) distribution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NormalDistribution {
    #[serde(rename = "@expectedValue")]
    pub expected_value: OSString,
    #[serde(rename = "@variance")]
    pub variance: OSString,
    /// Optional bounding range — serializes as a child `<Range>` element, not an attribute.
    #[serde(rename = "Range", skip_serializing_if = "Option::is_none")]
    pub range: Option<Range>,
}

/// Log-normal distribution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogNormalDistribution {
    #[serde(rename = "@expectedValue")]
    pub expected_value: OSString,
    #[serde(rename = "@variance")]
    pub variance: OSString,
    /// Optional bounding range — child `<Range>` element, not an attribute.
    #[serde(rename = "Range", skip_serializing_if = "Option::is_none")]
    pub range: Option<Range>,
}

/// Uniform distribution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UniformDistribution {
    /// Required bounding range — child `<Range>` element, not an attribute.
    #[serde(rename = "Range")]
    pub range: Range,
}

/// Poisson distribution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PoissonDistribution {
    #[serde(rename = "@expectedValue")]
    pub expected_value: OSString,
    /// Optional bounding range — child `<Range>` element, not an attribute.
    #[serde(rename = "Range", skip_serializing_if = "Option::is_none")]
    pub range: Option<Range>,
}

/// Histogram-based distribution
///
/// XSD `Histogram` (`:1295-1299`): sequence of `Bin` (`HistogramBin`),
/// `maxOccurs="unbounded"`, no `minOccurs`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Histogram {
    #[serde(rename = "Bin")]
    pub bins: MinVec<HistogramBin, 1>,
}

/// Bin in a histogram distribution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistogramBin {
    /// Required bounding range — child `<Range>` element, not an attribute.
    #[serde(rename = "Range")]
    pub range: Range,
    #[serde(rename = "@weight")]
    pub weight: OSString,
}

/// Range specification for distributions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Range {
    #[serde(rename = "@lowerLimit")]
    pub lower_limit: OSString,
    #[serde(rename = "@upperLimit")]
    pub upper_limit: OSString,
}

// No Default for Stochastic: `numberOfTestRuns` is `use="required"` in the XSD with no
// `default="…"` (confirmed against `Schema/OpenSCENARIO.xsd` — this project has found no genuine
// attribute default across ~131 checked so far), and `StochasticDistribution` has
// `maxOccurs="unbounded"` with no `minOccurs="0"`, so an empty `distributions` Vec is not
// schema-valid either. The previous impl fabricated `numberOfTestRuns: 1`. Use `new()`.
impl Stochastic {
    /// `StochasticDistribution` is `maxOccurs="unbounded"` with no `minOccurs="0"`, so `first`
    /// proves the schema minimum and building this cannot fail.
    pub fn new(
        number_of_test_runs: UnsignedInt,
        first: StochasticDistribution,
        rest: Vec<StochasticDistribution>,
    ) -> Self {
        Self {
            distributions: MinVec::from_min([first], rest),
            number_of_test_runs,
            random_seed: None,
        }
    }
}

impl ValidateDistribution for Stochastic {
    fn validate(&self) -> Result<()> {
        for dist in &self.distributions {
            dist.validate()?;
        }
        Ok(())
    }
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
    /// Nothing left to check at run time: `elements` states its own minimum
    /// (`MinVec<_, 1>`), so the empty set cannot be constructed.
    fn validate(&self) -> Result<()> {
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
        // "At least one bin" is now stated by the type (`MinVec<_, 1>`), so only the
        // per-bin checks remain here.
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
            Err(crate::error::Error::validation_error(
                "sampling",
                "Cannot sample from empty probability distribution set",
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
            (OSString::Literal(lower), OSString::Literal(upper)) => {
                Ok(format!("uniform({}, {})", lower, upper))
            }
            _ => Err(crate::error::Error::validation_error(
                "sampling",
                "Cannot sample from parameterized distribution without parameter resolution",
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

    /// `Stochastic::validate` visits every distribution it holds, so an invalid one after
    /// valid ones of other kinds is still reported.
    #[test]
    fn stochastic_validation_reports_an_invalid_distribution_it_holds() {
        let xml = r#"<Stochastic numberOfTestRuns="10">
    <StochasticDistribution parameterName="speed">
        <UniformDistribution><Range lowerLimit="0" upperLimit="1"/></UniformDistribution>
    </StochasticDistribution>
    <StochasticDistribution parameterName="lane">
        <ProbabilityDistributionSet><Element value="1" weight="1"/></ProbabilityDistributionSet>
    </StochasticDistribution>
    <StochasticDistribution parameterName="gap">
        <Histogram><Bin weight="1"><Range lowerLimit="0" upperLimit="5"/></Bin></Histogram>
    </StochasticDistribution>
    <StochasticDistribution parameterName="custom">
        <UserDefinedDistribution type=" ">content</UserDefinedDistribution>
    </StochasticDistribution>
</Stochastic>"#;
        let stochastic: Stochastic = quick_xml::de::from_str(xml).expect("fixture parses");
        assert_eq!(stochastic.distributions.len(), 4);
        let err = stochastic
            .validate()
            .expect_err("the user-defined distribution has a blank type");
        assert!(
            err.to_string()
                .contains("UserDefinedDistribution type cannot be empty"),
            "{err}"
        );
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
    fn stochastic_new_takes_no_result() {
        // `first` proves the schema minimum, so `Stochastic::new` returns `Self` directly.
        let distribution = StochasticDistribution {
            distribution_type: StochasticDistributionType::UniformDistribution(
                UniformDistribution {
                    range: Range {
                        lower_limit: Value::Literal("0.0".to_string()),
                        upper_limit: Value::Literal("1.0".to_string()),
                    },
                },
            ),
            parameter_name: OSString::Literal("speed".to_string()),
        };
        let stochastic = Stochastic::new(Value::Literal(10), distribution, vec![]);
        assert_eq!(stochastic.distributions.len(), 1);
    }
}
