//! Parameter variation: [`deterministic`] distributions enumerate their values
//! systematically, [`stochastic`] ones sample from a probability distribution.

use crate::error::Result;
use crate::types::entities::vehicle::File;
use serde::{Deserialize, Serialize};

// Import distribution types
pub use deterministic::Deterministic;
pub use stochastic::{Stochastic, StochasticDistribution};

pub mod deterministic;
pub mod stochastic;

pub use deterministic::*;
pub use stochastic::*;

/// Core parameter value distribution wrapper
///
/// XSD `ParameterValueDistribution` (`Schema/OpenSCENARIO.xsd:1661-1665`): sequence of a
/// required `ScenarioFile` followed by the bare choice group `DistributionDefinition`
/// (`:1086-1091`, no `minOccurs` on the group or either branch), so the choice is required.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterValueDistribution {
    #[serde(rename = "ScenarioFile")]
    pub scenario_file: File,
    #[serde(rename = "$value")]
    pub distribution: DistributionDefinition,
}

/// Union of all distribution types
///
/// XSD group `DistributionDefinition` (`Schema/OpenSCENARIO.xsd:1086-1091`) is a choice of
/// exactly two elements, `Deterministic` (type `Deterministic`) and `Stochastic` (type
/// `Stochastic`). There is no third `UserDefined` branch in the group, and neither payload is a
/// per-item choice: `Deterministic` is an unbounded sequence of single/multi entries, held by
/// the `Deterministic` container type, not by the single-entry choice
/// (`DeterministicParameterDistributionGroup`, below).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum DistributionDefinition {
    Deterministic(Deterministic),
    Stochastic(Stochastic),
}

/// User-defined distribution for custom parameter distributions
///
/// XSD `UserDefinedDistribution` is a `simpleContent` extension of `xsd:string`
/// with a required `@type` attribute, so the text body is not a child element.
///
/// The base type permits the empty string. Hence `<UserDefinedDistribution
/// type="t"/>` is schema-valid, and `content` carries `#[serde(default)]` so
/// that an absent text body reads as an empty string rather than failing with
/// `missing field $text`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserDefinedDistribution {
    #[serde(rename = "$text", default)]
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
            distribution: DistributionDefinition::Deterministic(deterministic),
        }
    }

    pub fn new_stochastic(scenario_file: File, stochastic: Stochastic) -> Self {
        Self {
            scenario_file,
            distribution: DistributionDefinition::Stochastic(stochastic),
        }
    }

    /// The deterministic branch, if that is the one selected.
    pub fn as_deterministic(&self) -> Option<&Deterministic> {
        match &self.distribution {
            DistributionDefinition::Deterministic(d) => Some(d),
            DistributionDefinition::Stochastic(_) => None,
        }
    }

    /// The stochastic branch, if that is the one selected.
    pub fn as_stochastic(&self) -> Option<&Stochastic> {
        match &self.distribution {
            DistributionDefinition::Stochastic(s) => Some(s),
            DistributionDefinition::Deterministic(_) => None,
        }
    }
}

// No Default for ParameterValueDistribution: the XSD's `ScenarioFile` and the
// Deterministic/Stochastic choice are both required, and the previous impl fabricated a whole
// nested distribution tree (a fake "default.xosc" file plus an invented parameter distribution)
// — the "fabricating a whole child" case, which never has a defensible replacement.
// Use `new_deterministic`/`new_stochastic`.

// No Default for DistributionDefinition: an xsd:choice group whose only variants each require
// real scenario content (a Deterministic or Stochastic distribution) — there is no "states
// nothing" member to default to.

impl UserDefinedDistribution {
    /// `type` is `use="required"` in the XSD; there is no schema default for it or for the
    /// text content, so both must be supplied.
    pub fn new(content: String, distribution_type: String) -> Self {
        Self {
            content,
            distribution_type,
        }
    }
}

impl ValidateDistribution for ParameterValueDistribution {
    fn validate(&self) -> Result<()> {
        self.distribution.validate()
    }
}

impl ValidateDistribution for DistributionDefinition {
    fn validate(&self) -> Result<()> {
        match self {
            Self::Deterministic(det) => det.validate(),
            Self::Stochastic(stoc) => stoc.validate(),
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

/// The model reference for `Range` states `lowerLimit <= value <= upperLimit`, so an inverted
/// range admits no value. Only literal limits are checked.
impl ValidateDistribution for crate::types::basic::Range {
    fn validate(&self) -> Result<()> {
        // A `$param` or `${expr}` limit is unknown until parameter resolution.
        if let (Some(lower), Some(upper)) =
            (self.lower_limit.as_literal(), self.upper_limit.as_literal())
        {
            if lower > upper {
                return Err(crate::error::Error::validation_error(
                    "Range.lowerLimit",
                    "Range lowerLimit must be <= upperLimit",
                ));
            }
        }
        Ok(())
    }
}

// XSD Group Implementations - Distribution Groups
//
// `DistributionDefinitionGroup` (a Deterministic|Stochastic choice) and
// `DeterministicSingleParameterDistributionTypeGroup` (a DistributionSet|DistributionRange|
// UserDefinedDistribution choice) used to duplicate `DistributionDefinition` above and
// `deterministic::DeterministicSingleParameterDistributionType` exactly, field for field, as
// unused scaffolding — neither type was reachable from a parsed document. Now that
// `ParameterValueDistribution` and `DeterministicSingleParameterDistribution` hold those two
// enums directly behind `$value`, the duplicates model nothing the wired-in types do not
// already model, so they are removed rather than converted. A second model of the same group
// is worse than none.
//
// `DeterministicParameterDistributionGroup` (below) is the one group wrapper this file keeps
// as a first-class type: `Deterministic.distributions` uses it directly as the per-entry
// choice of its repeated sequence (XSD:1024-1028), so it is no longer unused scaffolding.

/// DeterministicParameterDistribution group - the per-entry choice of `Deterministic`'s
/// repeated sequence (`Schema/OpenSCENARIO.xsd:1024-1028`, group `:1038-1042`): each entry is
/// independently either a `DeterministicMultiParameterDistribution` or a
/// `DeterministicSingleParameterDistribution` element, so a document may interleave the two.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeterministicParameterDistributionGroup {
    #[serde(rename = "DeterministicMultiParameterDistribution")]
    DeterministicMultiParameterDistribution(DeterministicMultiParameterDistribution),
    #[serde(rename = "DeterministicSingleParameterDistribution")]
    DeterministicSingleParameterDistribution(DeterministicSingleParameterDistribution),
}

// No Default for DeterministicParameterDistributionGroup: it wraps an xsd:choice whose every
// variant carries required content (see the notes on `DistributionDefinition` and
// `DeterministicSingleParameterDistributionType` above) — there is no schema-valid "empty"
// member to pick.

// Helper implementations for ergonomic group usage

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

// Validation trait implementations - delegate to underlying types

impl ValidateDistribution for DeterministicParameterDistributionGroup {
    fn validate(&self) -> Result<()> {
        match self {
            Self::DeterministicSingleParameterDistribution(dist) => dist.validate(),
            Self::DeterministicMultiParameterDistribution(dist) => dist.validate(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::basic::Value;
    use crate::types::distributions::deterministic::*;

    // Sample-content builders for tests. These stand in for the removed fabricating `Default`
    // impls: they build the same shapes those impls used to hand back, but as named, explicit
    // test fixtures rather than something `::default()` could invent silently in library code.
    fn sample_distribution_set() -> DistributionSet {
        DistributionSet::new(
            DistributionSetElement::new(Value::Literal("0.0".to_string())),
            vec![],
        )
    }

    fn sample_single_parameter_distribution() -> DeterministicSingleParameterDistribution {
        DeterministicSingleParameterDistribution::new(
            Value::Literal("parameter".to_string()),
            DeterministicSingleParameterDistributionType::DistributionSet(sample_distribution_set()),
        )
    }

    fn sample_multi_parameter_distribution() -> DeterministicMultiParameterDistribution {
        DeterministicMultiParameterDistribution::new(ValueSetDistribution::new(
            ParameterValueSet::new(
                ParameterAssignment::new(
                    "parameter".to_string(),
                    Value::Literal("0.0".to_string()),
                ),
                vec![],
            ),
            vec![],
        ))
    }

    fn sample_deterministic_parameter_distribution() -> Deterministic {
        let mut det = Deterministic::default();
        det.add_single(sample_single_parameter_distribution());
        det
    }

    fn sample_parameter_value_distribution() -> ParameterValueDistribution {
        ParameterValueDistribution::new_deterministic(
            File {
                filepath: "test.xosc".to_string(),
            },
            sample_deterministic_parameter_distribution(),
        )
    }

    #[test]
    fn test_parameter_value_distribution_creation() {
        let scenario_file = File {
            filepath: "test.xosc".to_string(),
        };
        let param_dist = ParameterValueDistribution::new_deterministic(
            scenario_file,
            sample_deterministic_parameter_distribution(),
        );

        assert!(param_dist.as_deterministic().is_some());
        assert!(param_dist.as_stochastic().is_none());
    }

    #[test]
    fn test_parameter_value_distribution_round_trip() {
        let param_dist = sample_parameter_value_distribution();
        let xml = quick_xml::se::to_string(&param_dist).expect("serialize");
        assert!(xml.starts_with("<ParameterValueDistribution>"), "{xml}");
        assert!(xml.contains("<Deterministic>"), "{xml}");
        let reparsed: ParameterValueDistribution =
            quick_xml::de::from_str(&xml).expect("deserialize");
        assert_eq!(param_dist, reparsed);
    }

    #[test]
    fn test_parameter_value_distribution_validate_reaches_multi_parameter_entries() {
        // `validate` descends through the `Deterministic` branch into each entry, so a
        // multi-parameter value set that assigns `parameter` twice fails the whole distribution.
        let mut det = sample_deterministic_parameter_distribution();
        det.add_multi(sample_multi_parameter_distribution());
        let file = File {
            filepath: "test.xosc".to_string(),
        };
        let valid = ParameterValueDistribution::new_deterministic(file.clone(), det.clone());
        assert!(valid.validate().is_ok());

        det.add_multi(DeterministicMultiParameterDistribution::new(
            ValueSetDistribution::new(
                ParameterValueSet::new(
                    ParameterAssignment::new("parameter".to_string(), Value::Literal("0".into())),
                    vec![ParameterAssignment::new(
                        "parameter".to_string(),
                        Value::Literal("1".into()),
                    )],
                ),
                vec![],
            ),
        ));
        let err = ParameterValueDistribution::new_deterministic(file, det)
            .validate()
            .expect_err("duplicate parameterRef in a multi-parameter entry must be rejected");
        assert!(
            err.to_string().contains("Duplicate parameter reference"),
            "{err}"
        );
    }

    #[test]
    fn test_parameter_value_distribution_zero_branches_rejected() {
        let xml = r#"<ParameterValueDistribution><ScenarioFile filepath="test.xosc"/></ParameterValueDistribution>"#;
        let err = quick_xml::de::from_str::<ParameterValueDistribution>(xml)
            .expect_err("empty choice must be rejected");
        assert!(err.to_string().contains("missing field `$value`"), "{err}");
    }

    #[test]
    fn test_parameter_value_distribution_two_branches_rejected() {
        let xml = r#"<ParameterValueDistribution><ScenarioFile filepath="test.xosc"/><Deterministic></Deterministic><Stochastic numberOfTestRuns="1"><StochasticDistribution parameterName="speed"><NormalDistribution expectedValue="0" variance="1"/></StochasticDistribution></Stochastic></ParameterValueDistribution>"#;
        let err = quick_xml::de::from_str::<ParameterValueDistribution>(xml)
            .expect_err("two branches on the choice must be rejected");
        assert!(
            err.to_string().contains("duplicate field `$value`"),
            "{err}"
        );
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

    // Tests for the `DeterministicParameterDistributionGroup` choice, the per-entry choice
    // used by `Deterministic.distributions`.

    #[test]
    fn test_deterministic_parameter_distribution_group_creation() {
        let single_dist = sample_single_parameter_distribution();
        let group = DeterministicParameterDistributionGroup::single(single_dist);

        assert!(group.is_single());
        assert!(!group.is_multi());

        let multi_dist = sample_multi_parameter_distribution();
        let multi_group = DeterministicParameterDistributionGroup::multi(multi_dist);

        assert!(!multi_group.is_single());
        assert!(multi_group.is_multi());
    }

    #[test]
    fn test_deterministic_parameter_distribution_group_validation() {
        let group =
            DeterministicParameterDistributionGroup::single(sample_single_parameter_distribution());
        assert!(group.validate().is_ok());
    }

    #[test]
    fn test_deterministic_parameter_distribution_group_round_trip() {
        let group =
            DeterministicParameterDistributionGroup::single(sample_single_parameter_distribution());
        let xml = quick_xml::se::to_string(&group).expect("serialize");
        assert!(
            xml.starts_with("<DeterministicSingleParameterDistribution"),
            "{xml}"
        );
        let reparsed: DeterministicParameterDistributionGroup =
            quick_xml::de::from_str(&xml).expect("deserialize");
        assert_eq!(group, reparsed);
    }
}
