//! Deterministic distribution types for systematic parameter variation

use crate::error::Result;
use crate::types::basic::{Double, MinVec, OSString, Value};
use crate::types::distributions::{
    DeterministicParameterDistributionGroup, DistributionSampler, ValidateDistribution,
};
use serde::{Deserialize, Serialize};

/// Container for deterministic parameter distributions.
///
/// XSD `Deterministic` (`Schema/OpenSCENARIO.xsd:1024-1028`) is a sequence of one particle,
/// `<xsd:group ref="DeterministicParameterDistribution" minOccurs="0" maxOccurs="unbounded"/>`.
/// Each occurrence of the group independently picks one of its two branches
/// (`DeterministicMultiParameterDistribution` | `DeterministicSingleParameterDistribution`,
/// XSD:1038-1042), so a document may freely interleave the two kinds. The single ordered
/// `Vec` below preserves that interleaving; the crate's earlier hand-written `Serialize`
/// collected singles and multis into two separate `Vec`s and re-emitted all of one kind
/// before the other, which reorders any document that interleaves them (invisible to the
/// conformance corpus, since no corpus file does).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Deterministic {
    /// The distribution entries, in document order.
    #[serde(rename = "$value", default, skip_serializing_if = "Vec::is_empty")]
    pub distributions: Vec<DeterministicParameterDistributionGroup>,
}

impl Deterministic {
    /// Check if the container has no distributions
    pub fn is_empty(&self) -> bool {
        self.distributions.is_empty()
    }

    /// Add a single parameter distribution
    pub fn add_single(&mut self, distribution: DeterministicSingleParameterDistribution) {
        self.distributions
            .push(DeterministicParameterDistributionGroup::single(
                distribution,
            ));
    }

    /// Add a multi parameter distribution
    pub fn add_multi(&mut self, distribution: DeterministicMultiParameterDistribution) {
        self.distributions
            .push(DeterministicParameterDistributionGroup::multi(distribution));
    }

    /// Iterate the single-parameter entries, in document order (other entries skipped).
    pub fn single_distributions(
        &self,
    ) -> impl Iterator<Item = &DeterministicSingleParameterDistribution> {
        self.distributions.iter().filter_map(|d| match d {
            DeterministicParameterDistributionGroup::DeterministicSingleParameterDistribution(
                s,
            ) => Some(s),
            _ => None,
        })
    }

    /// Iterate the multi-parameter entries, in document order (other entries skipped).
    pub fn multi_distributions(
        &self,
    ) -> impl Iterator<Item = &DeterministicMultiParameterDistribution> {
        self.distributions.iter().filter_map(|d| match d {
            DeterministicParameterDistributionGroup::DeterministicMultiParameterDistribution(m) => {
                Some(m)
            }
            _ => None,
        })
    }

    /// Get total count of all distributions
    pub fn total_count(&self) -> usize {
        self.distributions.len()
    }
}

/// Single parameter deterministic distribution
///
/// XSD `DeterministicSingleParameterDistribution` (`:1045-1049`): sequence of the bare
/// choice group `DeterministicSingleParameterDistributionType` (`:1051-1057`, no
/// `minOccurs` on the group or any of its three branches, so the choice is required), plus
/// required attribute `parameterName`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeterministicSingleParameterDistribution {
    #[serde(rename = "@parameterName")]
    pub parameter_name: OSString,
    #[serde(rename = "$value")]
    pub distribution: DeterministicSingleParameterDistributionType,
}

/// The three branches of the `DeterministicSingleParameterDistributionType` choice (XSD:1051-1057).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum DeterministicSingleParameterDistributionType {
    DistributionSet(DistributionSet),
    DistributionRange(DistributionRange),
    UserDefinedDistribution(crate::types::distributions::UserDefinedDistribution),
}

impl DeterministicSingleParameterDistribution {
    /// Construct a single-parameter distribution from its required name and chosen branch.
    pub fn new(
        parameter_name: OSString,
        distribution: DeterministicSingleParameterDistributionType,
    ) -> Self {
        Self {
            parameter_name,
            distribution,
        }
    }
}

/// Multi-parameter deterministic distribution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeterministicMultiParameterDistribution {
    #[serde(rename = "ValueSetDistribution")]
    pub distribution_type: ValueSetDistribution,
}

impl DeterministicMultiParameterDistribution {
    pub fn new(distribution_type: ValueSetDistribution) -> Self {
        Self { distribution_type }
    }
}

/// Discrete value set distribution
///
/// XSD `DistributionSet` (`:1098-1102`): sequence of `Element`
/// (`DistributionSetElement`), `maxOccurs="unbounded"`, no `minOccurs`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DistributionSet {
    #[serde(rename = "Element")]
    pub elements: MinVec<DistributionSetElement, 1>,
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
    pub step_width: Double,
    #[serde(rename = "Range")]
    pub range: crate::types::basic::Range,
}

/// Multi-parameter value set distribution
///
/// XSD `ValueSetDistribution` (`:2451-2455`): sequence of `ParameterValueSet`,
/// `maxOccurs="unbounded"`, no `minOccurs`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValueSetDistribution {
    #[serde(rename = "ParameterValueSet")]
    pub parameter_value_sets: MinVec<ParameterValueSet, 1>,
}

/// Set of parameter assignments
///
/// XSD `ParameterValueSet` (`:1672-1676`): sequence of `ParameterAssignment`,
/// `maxOccurs="unbounded"`, no `minOccurs`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterValueSet {
    #[serde(rename = "ParameterAssignment")]
    pub parameter_assignments: MinVec<ParameterAssignment, 1>,
}

/// Individual parameter assignment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterAssignment {
    #[serde(rename = "@parameterRef")]
    pub parameter_ref: String,
    #[serde(rename = "@value")]
    pub value: OSString,
}

impl ValidateDistribution for DeterministicSingleParameterDistribution {
    fn validate(&self) -> Result<()> {
        self.distribution.validate()
    }
}

impl ValidateDistribution for DeterministicSingleParameterDistributionType {
    fn validate(&self) -> Result<()> {
        match self {
            Self::DistributionSet(dist) => dist.validate(),
            Self::DistributionRange(dist) => dist.validate(),
            Self::UserDefinedDistribution(dist) => dist.validate(),
        }
    }
}

impl ValidateDistribution for Deterministic {
    fn validate(&self) -> Result<()> {
        for dist in &self.distributions {
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
    /// Nothing left to check at run time.
    ///
    /// The only rule this ever enforced was "at least one element", which `elements`
    /// now states in its own type (`MinVec<_, 1>`, XSD `DistributionSet` :1073 at the
    /// default `minOccurs="1"`). A set that breaks it cannot be constructed, so there is
    /// no invalid value left for this to report.
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}

impl ValidateDistribution for DistributionRange {
    /// The model reference states no range for `stepWidth`, so only the `Range` is checked.
    fn validate(&self) -> Result<()> {
        self.range.validate()
    }
}

impl ValidateDistribution for ValueSetDistribution {
    fn validate(&self) -> Result<()> {
        for value_set in &self.parameter_value_sets {
            value_set.validate()?;
        }
        Ok(())
    }
}

impl ValidateDistribution for ParameterValueSet {
    fn validate(&self) -> Result<()> {
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

// No Default impls below `Deterministic` itself: every other type in this file is either an
// XSD choice group (`DeterministicSingleParameterDistributionType`) whose variants each carry
// required scenario content, or a container whose child element has minOccurs="1" in the
// schema (`DistributionSet.Element`, `ValueSetDistribution.ParameterValueSet`,
// `ParameterValueSet.ParameterAssignment` all lack `minOccurs="0"` — verified against
// `Schema/OpenSCENARIO.xsd`), so an empty `Vec` would not be schema-valid either. There is no
// default that states nothing; callers must supply the required content via `::new()`.
// `Deterministic`'s own derived `Default` is the one exception: its sole field is the sequence
// itself (`minOccurs="0" maxOccurs="unbounded"`), so an empty `Vec` — zero occurrences — is
// schema-valid content, not a fabrication.

impl DistributionSet {
    /// Construct a distribution set. The schema requires at least one `Element`
    /// (`maxOccurs="unbounded"`, no `minOccurs="0"`), so this takes the first element plus any
    /// further ones rather than allowing an empty set.
    pub fn new(first: DistributionSetElement, rest: Vec<DistributionSetElement>) -> Self {
        Self {
            elements: MinVec::from_min([first], rest),
        }
    }
}

impl DistributionSetElement {
    pub fn new(value: OSString) -> Self {
        Self { value }
    }
}

impl DistributionRange {
    pub fn new(step_width: Double, range: crate::types::basic::Range) -> Self {
        Self { step_width, range }
    }
}

impl ValueSetDistribution {
    /// The schema requires at least one `ParameterValueSet` (`maxOccurs="unbounded"`, no
    /// `minOccurs="0"`). `first` proves it, so building this cannot fail.
    pub fn new(first: ParameterValueSet, rest: Vec<ParameterValueSet>) -> Self {
        Self {
            parameter_value_sets: MinVec::from_min([first], rest),
        }
    }
}

impl ParameterValueSet {
    /// The schema requires at least one `ParameterAssignment` (`maxOccurs="unbounded"`, no
    /// `minOccurs="0"`). `first` proves it, so building this cannot fail.
    pub fn new(first: ParameterAssignment, rest: Vec<ParameterAssignment>) -> Self {
        Self {
            parameter_assignments: MinVec::from_min([first], rest),
        }
    }
}

impl ParameterAssignment {
    pub fn new(parameter_ref: String, value: OSString) -> Self {
        Self {
            parameter_ref,
            value,
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
            Value::Literal(val) => Ok(val.to_string()),
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
    fn test_distribution_set_sampling() {
        let dist_set = DistributionSet {
            elements: MinVec::new(vec![
                DistributionSetElement {
                    value: Value::Literal("10.0".to_string()),
                },
                DistributionSetElement {
                    value: Value::Literal("20.0".to_string()),
                },
            ])
            .unwrap(),
        };

        assert_eq!(dist_set.sample().unwrap(), "10.0");
        let values = dist_set.enumerate().unwrap();
        assert_eq!(values.len(), 2);
        assert_eq!(values[0], "10.0");
        assert_eq!(values[1], "20.0");
        assert!(dist_set.is_deterministic());
    }

    #[test]
    fn test_parameter_value_set_validation() {
        let valid_set = ParameterValueSet {
            parameter_assignments: MinVec::new(vec![
                ParameterAssignment {
                    parameter_ref: "speed".to_string(),
                    value: Value::Literal("30.0".to_string()),
                },
                ParameterAssignment {
                    parameter_ref: "position".to_string(),
                    value: Value::Literal("100.0".to_string()),
                },
            ])
            .unwrap(),
        };
        assert!(valid_set.validate().is_ok());

        let duplicate_set = ParameterValueSet {
            parameter_assignments: MinVec::new(vec![
                ParameterAssignment {
                    parameter_ref: "speed".to_string(),
                    value: Value::Literal("30.0".to_string()),
                },
                ParameterAssignment {
                    parameter_ref: "speed".to_string(),
                    value: Value::Literal("40.0".to_string()),
                },
            ])
            .unwrap(),
        };
        assert!(duplicate_set.validate().is_err());
    }

    fn sample_single(name: &str) -> DeterministicSingleParameterDistribution {
        DeterministicSingleParameterDistribution::new(
            Value::literal(name.to_string()),
            DeterministicSingleParameterDistributionType::DistributionSet(DistributionSet::new(
                DistributionSetElement::new(Value::literal("1.0".to_string())),
                vec![],
            )),
        )
    }

    fn sample_multi() -> DeterministicMultiParameterDistribution {
        DeterministicMultiParameterDistribution::new(ValueSetDistribution::new(
            ParameterValueSet::new(
                ParameterAssignment::new("p".to_string(), Value::literal("1.0".to_string())),
                vec![],
            ),
            vec![],
        ))
    }

    #[test]
    fn test_deterministic_single_parameter_distribution_round_trip() {
        let dist = sample_single("speed");
        let xml = quick_xml::se::to_string(&dist).expect("serialize");
        assert!(
            xml.starts_with(r#"<DeterministicSingleParameterDistribution parameterName="speed">"#),
            "{xml}"
        );
        let reparsed: DeterministicSingleParameterDistribution =
            quick_xml::de::from_str(&xml).expect("deserialize");
        assert_eq!(dist, reparsed);
    }

    #[test]
    fn test_deterministic_single_parameter_distribution_zero_branches_rejected() {
        let xml = r#"<DeterministicSingleParameterDistribution parameterName="speed"></DeterministicSingleParameterDistribution>"#;
        let err = quick_xml::de::from_str::<DeterministicSingleParameterDistribution>(xml)
            .expect_err("empty choice must be rejected");
        assert!(err.to_string().contains("missing field `$value`"), "{err}");
    }

    #[test]
    fn test_deterministic_single_parameter_distribution_two_branches_rejected() {
        let xml = r#"<DeterministicSingleParameterDistribution parameterName="speed"><DistributionSet><Element value="1.0"/></DistributionSet><DistributionRange stepWidth="1.0"><Range lowerLimit="0.0" upperLimit="1.0"/></DistributionRange></DeterministicSingleParameterDistribution>"#;
        let err = quick_xml::de::from_str::<DeterministicSingleParameterDistribution>(xml)
            .expect_err("two branches on the choice must be rejected");
        assert!(
            err.to_string().contains("duplicate field `$value`"),
            "{err}"
        );
    }

    #[test]
    fn test_deterministic_empty_parses_and_round_trips() {
        // The `Deterministic` sequence particle is `minOccurs="0" maxOccurs="unbounded"`, so
        // zero entries is legal content, not a rejected case.
        let xml = r#"<Deterministic></Deterministic>"#;
        let det: Deterministic = quick_xml::de::from_str(xml).expect("deserialize");
        assert!(det.is_empty());
        assert_eq!(det.total_count(), 0);
        let serialized = quick_xml::se::to_string(&det).expect("serialize");
        assert_eq!(serialized, "<Deterministic/>");
    }

    #[test]
    fn test_deterministic_repeated_single_entries_accepted() {
        // Two entries of the same branch (both `DeterministicSingleParameterDistribution`)
        // must be accepted: the repetition lives in `Deterministic`'s own sequence, not in a
        // one-of-two choice.
        let mut det = Deterministic::default();
        det.add_single(sample_single("a"));
        det.add_single(sample_single("b"));
        assert_eq!(det.total_count(), 2);
        assert_eq!(det.single_distributions().count(), 2);

        let xml = quick_xml::se::to_string(&det).expect("serialize");
        let reparsed: Deterministic = quick_xml::de::from_str(&xml).expect("deserialize");
        assert_eq!(det, reparsed);
    }

    #[test]
    fn test_deterministic_interleaved_entries_preserve_order() {
        // Reproduces the interleaving defect: the crate's old hand-written `Serialize`
        // collected singles and multis into two separate `Vec`s and re-emitted all of one
        // kind before the other, so this document came back reordered.
        let xml = format!(
            "<Deterministic>{}{}{}</Deterministic>",
            quick_xml::se::to_string(&DeterministicParameterDistributionGroup::single(
                sample_single("a")
            ))
            .unwrap(),
            quick_xml::se::to_string(&DeterministicParameterDistributionGroup::multi(
                sample_multi()
            ))
            .unwrap(),
            quick_xml::se::to_string(&DeterministicParameterDistributionGroup::single(
                sample_single("b")
            ))
            .unwrap(),
        );

        let det: Deterministic = quick_xml::de::from_str(&xml).expect("deserialize");
        assert_eq!(det.distributions.len(), 3);
        assert!(det.distributions[0].is_single());
        assert!(det.distributions[1].is_multi());
        assert!(det.distributions[2].is_single());

        let serialized = quick_xml::se::to_string(&det).expect("serialize");
        assert_eq!(
            serialized, xml,
            "document order (single, multi, single) must be preserved on round trip"
        );
    }

    fn range_with_lower(lower: Value<f64>) -> DistributionRange {
        DistributionRange::new(
            Value::literal(1.0),
            crate::types::basic::Range {
                lower_limit: lower,
                upper_limit: Value::literal(10.0),
            },
        )
    }

    /// `DistributionRange::new` and `DistributionSampler::{sample, is_deterministic}`.
    #[test]
    fn distribution_range_sample_returns_lower_limit() {
        let range = range_with_lower(Value::literal(0.0));
        assert_eq!(range.sample().unwrap(), "0");
        assert!(range.is_deterministic());
    }

    /// `DistributionRange::validate` checks its `Range` on literal limits (upper limit 10 in
    /// every row). The model reference states no range for `stepWidth`. `None` expects `Ok`;
    /// `Some(field)` expects a `ValidationError` on that field.
    #[test]
    fn distribution_range_bounds_are_checked_on_literals() {
        let rows: [(&str, Value<f64>, Option<&str>); 4] = [
            ("lower below upper", Value::literal(0.0), None),
            ("lower equal to upper", Value::literal(10.0), None),
            (
                "lower above upper",
                Value::literal(11.0),
                Some("Range.lowerLimit"),
            ),
            (
                "parameter lower limit",
                Value::Parameter("lo".to_string()),
                None,
            ),
        ];
        for (name, lower, expected) in rows {
            match (range_with_lower(lower).validate(), expected) {
                (Ok(()), None) => {}
                (Err(crate::error::Error::ValidationError { field, .. }), Some(expected)) => {
                    assert_eq!(field, expected, "{name}")
                }
                (result, expected) => panic!("{name}: expected {expected:?}, got {result:?}"),
            }
        }
    }

    #[test]
    fn distribution_range_sample_rejects_parameterized_and_expression_lower_limit() {
        let range = range_with_lower(Value::Parameter("lo".to_string()));
        let err = range
            .sample()
            .expect_err("a parameterized lower limit cannot be sampled");
        assert!(
            err.to_string().contains("without parameter resolution"),
            "{err}"
        );

        let range = range_with_lower(Value::Expression("${lo}".to_string()));
        let err = range
            .sample()
            .expect_err("an expression lower limit cannot be sampled");
        assert!(
            err.to_string().contains("without expression evaluation"),
            "{err}"
        );
    }

    /// `DeterministicSingleParameterDistributionType::validate` dispatches on its three
    /// branches; every existing test only ever built the `DistributionSet` branch
    /// (`sample_single`). This reaches the other two.
    #[test]
    fn deterministic_single_parameter_distribution_validates_range_and_user_defined_branches() {
        let range_dist = DeterministicSingleParameterDistribution::new(
            Value::literal("speed".to_string()),
            DeterministicSingleParameterDistributionType::DistributionRange(range_with_lower(
                Value::literal(0.0),
            )),
        );
        assert!(range_dist.validate().is_ok());

        let user_dist = DeterministicSingleParameterDistribution::new(
            Value::literal("mode".to_string()),
            DeterministicSingleParameterDistributionType::UserDefinedDistribution(
                crate::types::distributions::UserDefinedDistribution::new(
                    "x".to_string(),
                    "custom".to_string(),
                ),
            ),
        );
        assert!(user_dist.validate().is_ok());
    }

    /// `DeterministicMultiParameterDistribution::validate` (delegates to
    /// `ValueSetDistribution::validate`) was never called by any test.
    #[test]
    fn deterministic_multi_parameter_distribution_validate_delegates_to_value_set() {
        let multi = sample_multi();
        assert!(multi.validate().is_ok());
    }

    /// `DistributionSet::{sample, enumerate}` only had literal-value coverage; the
    /// parameterized/expression error arms of both were untested.
    #[test]
    fn distribution_set_sample_and_enumerate_reject_parameter_and_expression_elements() {
        let param_set = DistributionSet {
            elements: MinVec::new(vec![DistributionSetElement {
                value: Value::Parameter("p".to_string()),
            }])
            .unwrap(),
        };
        let err = param_set
            .sample()
            .expect_err("a parameterized element cannot be sampled");
        assert!(
            err.to_string().contains("without parameter resolution"),
            "{err}"
        );
        let err = param_set
            .enumerate()
            .expect_err("a parameterized element cannot be enumerated");
        assert!(
            err.to_string().contains("without parameter resolution"),
            "{err}"
        );

        let expr_set = DistributionSet {
            elements: MinVec::new(vec![DistributionSetElement {
                value: Value::Expression("${p}".to_string()),
            }])
            .unwrap(),
        };
        let err = expr_set
            .sample()
            .expect_err("an expression element cannot be sampled");
        assert!(
            err.to_string().contains("without expression evaluation"),
            "{err}"
        );
        let err = expr_set
            .enumerate()
            .expect_err("an expression element cannot be enumerated");
        assert!(
            err.to_string().contains("without expression evaluation"),
            "{err}"
        );
    }

    #[test]
    fn distribution_set_new_takes_no_result() {
        // `first` proves the schema minimum, so `DistributionSet::new` returns `Self`
        // directly; there is nothing here to `.unwrap()` or propagate with `?`.
        let set = DistributionSet::new(
            DistributionSetElement::new(Value::literal("1.0".to_string())),
            vec![DistributionSetElement::new(Value::literal(
                "2.0".to_string(),
            ))],
        );
        assert_eq!(set.elements.len(), 2);
    }

    #[test]
    fn value_set_distribution_and_parameter_value_set_new_take_no_result() {
        let value_set = ParameterValueSet::new(
            ParameterAssignment::new("p".to_string(), Value::literal("1.0".to_string())),
            vec![],
        );
        assert_eq!(value_set.parameter_assignments.len(), 1);

        let distribution = ValueSetDistribution::new(value_set, vec![]);
        assert_eq!(distribution.parameter_value_sets.len(), 1);
    }
}
