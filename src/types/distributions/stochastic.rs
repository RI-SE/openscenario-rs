//! Stochastic distribution types for probabilistic parameter variation

use crate::error::Error;
use crate::error::Result;
use crate::types::basic::{Double, MinVec, OSString, Range, UnsignedInt, Value};
use crate::types::distributions::ValidateDistribution;
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
    pub weight: Double,
}

/// Normal (Gaussian) distribution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NormalDistribution {
    #[serde(rename = "@expectedValue")]
    pub expected_value: Double,
    #[serde(rename = "@variance")]
    pub variance: Double,
    /// Optional bounding range — serializes as a child `<Range>` element, not an attribute.
    #[serde(rename = "Range", skip_serializing_if = "Option::is_none")]
    pub range: Option<Range>,
}

/// Log-normal distribution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogNormalDistribution {
    #[serde(rename = "@expectedValue")]
    pub expected_value: Double,
    #[serde(rename = "@variance")]
    pub variance: Double,
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
    pub expected_value: Double,
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
    pub weight: Double,
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

/// Validates `Option<Range>` the same way for every distribution that bounds itself with one.
fn validate_optional_range(range: &Option<Range>) -> Result<()> {
    range.as_ref().map_or(Ok(()), Range::validate)
}

impl ValidateDistribution for NormalDistribution {
    /// The model reference states no range for `expectedValue` or `variance`, so only the
    /// optional `Range` is checked.
    fn validate(&self) -> Result<()> {
        validate_optional_range(&self.range)
    }
}

impl ValidateDistribution for LogNormalDistribution {
    /// The model reference gives `variance` the range `]0; inf[` and requires the lower limit
    /// of the optional `Range` to be `> 0`.
    fn validate(&self) -> Result<()> {
        // A `$param` or `${expr}` operand is unknown until parameter resolution.
        if let Some(&variance) = self.variance.as_literal() {
            if variance <= 0.0 {
                return Err(Error::validation_error(
                    "LogNormalDistribution.variance",
                    "LogNormalDistribution variance must be > 0",
                ));
            }
        }
        if let Some(range) = &self.range {
            // A `$param` or `${expr}` limit is unknown until parameter resolution.
            if let Some(&lower) = range.lower_limit.as_literal() {
                if lower <= 0.0 {
                    return Err(Error::validation_error(
                        "LogNormalDistribution.Range.lowerLimit",
                        "LogNormalDistribution Range lowerLimit must be > 0",
                    ));
                }
            }
        }
        validate_optional_range(&self.range)
    }
}

impl ValidateDistribution for UniformDistribution {
    fn validate(&self) -> Result<()> {
        self.range.validate()
    }
}

impl ValidateDistribution for PoissonDistribution {
    /// The model reference states no range for `expectedValue`, so only the optional `Range`
    /// is checked.
    fn validate(&self) -> Result<()> {
        validate_optional_range(&self.range)
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
    /// The model reference states no range for `weight`, so only the bin's `Range` is checked.
    fn validate(&self) -> Result<()> {
        self.range.validate()
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

    /// Bounds the ASAM model reference states for the stochastic distributions, checked on
    /// literal operands only. Each row wraps one distribution in a `Stochastic`, so the
    /// `Ok` rows also reach `Stochastic::validate`'s all-valid return and every
    /// `StochasticDistributionType` arm that bounds itself with a `Range`. `None` expects
    /// `Ok`; `Some(field)` expects a `ValidationError` on that field.
    #[test]
    fn stochastic_distribution_bounds_are_checked_on_literals() {
        let rows: &[(&str, &str, Option<&str>)] = &[
            (
                "normal, valid range",
                r#"<NormalDistribution expectedValue="0" variance="1"><Range lowerLimit="-1" upperLimit="1"/></NormalDistribution>"#,
                None,
            ),
            (
                "normal, inverted range",
                r#"<NormalDistribution expectedValue="0" variance="1"><Range lowerLimit="1" upperLimit="-1"/></NormalDistribution>"#,
                Some("Range.lowerLimit"),
            ),
            (
                "log-normal, valid",
                r#"<LogNormalDistribution expectedValue="1" variance="2"><Range lowerLimit="0.5" upperLimit="3"/></LogNormalDistribution>"#,
                None,
            ),
            (
                "log-normal, zero variance",
                r#"<LogNormalDistribution expectedValue="1" variance="0"/>"#,
                Some("LogNormalDistribution.variance"),
            ),
            (
                "log-normal, parameter variance",
                r#"<LogNormalDistribution expectedValue="1" variance="$v"/>"#,
                None,
            ),
            (
                "log-normal, zero range lower limit",
                r#"<LogNormalDistribution expectedValue="1" variance="2"><Range lowerLimit="0" upperLimit="3"/></LogNormalDistribution>"#,
                Some("LogNormalDistribution.Range.lowerLimit"),
            ),
            (
                "log-normal, inverted range",
                r#"<LogNormalDistribution expectedValue="1" variance="2"><Range lowerLimit="3" upperLimit="1"/></LogNormalDistribution>"#,
                Some("Range.lowerLimit"),
            ),
            (
                "log-normal, parameter range lower limit",
                r#"<LogNormalDistribution expectedValue="1" variance="2"><Range lowerLimit="$lo" upperLimit="3"/></LogNormalDistribution>"#,
                None,
            ),
            (
                "poisson, valid range",
                r#"<PoissonDistribution expectedValue="3"><Range lowerLimit="0" upperLimit="10"/></PoissonDistribution>"#,
                None,
            ),
            (
                "poisson, inverted range",
                r#"<PoissonDistribution expectedValue="3"><Range lowerLimit="10" upperLimit="0"/></PoissonDistribution>"#,
                Some("Range.lowerLimit"),
            ),
            (
                "uniform, equal limits",
                r#"<UniformDistribution><Range lowerLimit="2" upperLimit="2"/></UniformDistribution>"#,
                None,
            ),
            (
                "uniform, inverted range",
                r#"<UniformDistribution><Range lowerLimit="2" upperLimit="1"/></UniformDistribution>"#,
                Some("Range.lowerLimit"),
            ),
            (
                "uniform, parameter lower limit",
                r#"<UniformDistribution><Range lowerLimit="$lo" upperLimit="-1"/></UniformDistribution>"#,
                None,
            ),
            (
                "histogram, inverted bin range",
                r#"<Histogram><Bin weight="1"><Range lowerLimit="0" upperLimit="1"/></Bin><Bin weight="1"><Range lowerLimit="5" upperLimit="2"/></Bin></Histogram>"#,
                Some("Range.lowerLimit"),
            ),
        ];
        for (name, body, expected) in rows {
            let xml = format!(
                r#"<Stochastic numberOfTestRuns="1"><StochasticDistribution parameterName="p">{body}</StochasticDistribution></Stochastic>"#
            );
            let stochastic: Stochastic = quick_xml::de::from_str(&xml)
                .unwrap_or_else(|e| panic!("{name}: fixture must parse: {e}"));
            match (stochastic.validate(), expected) {
                (Ok(()), None) => {}
                (Err(Error::ValidationError { field, .. }), Some(expected)) => {
                    assert_eq!(&field, expected, "{name}")
                }
                (result, expected) => panic!("{name}: expected {expected:?}, got {result:?}"),
            }
        }
    }

    #[test]
    fn stochastic_new_takes_no_result() {
        // `first` proves the schema minimum, so `Stochastic::new` returns `Self` directly.
        let distribution = StochasticDistribution {
            distribution_type: StochasticDistributionType::UniformDistribution(
                UniformDistribution {
                    range: Range {
                        lower_limit: Value::Literal(0.0),
                        upper_limit: Value::Literal(1.0),
                    },
                },
            ),
            parameter_name: OSString::Literal("speed".to_string()),
        };
        let stochastic = Stochastic::new(Value::Literal(10), distribution, vec![]);
        assert_eq!(stochastic.distributions.len(), 1);
    }
}
