//! Regression: `StochasticDistribution` wrapped its choice of distribution
//! types behind `#[serde(flatten)]`. Two of the seven branches,
//! `ProbabilityDistributionSet` and `Histogram`, hold a `Vec` directly below
//! the choice. `flatten` buffers an element's children into a map through
//! `deserialize_any` before the enum picks a variant, and a `Vec<T>` replayed
//! out of that buffer fails with `invalid type: map, expected a sequence`,
//! even when the repeated element occurs only once. No stochastic parameter
//! distribution in either of those two shapes could be read at all. The fix
//! nests the choice behind the special field name `$value`, which quick-xml
//! reads directly from the live parser instead of buffering it, so a
//! sequence below the chosen branch parses normally.
use openscenario_rs::types::distributions::stochastic::StochasticDistribution;

const ONE_ELEMENT_PROBABILITY_SET: &str = concat!(
    r#"<StochasticDistribution parameterName="p">"#,
    r#"<ProbabilityDistributionSet>"#,
    r#"<Element value="a" weight="0.5"/>"#,
    r#"</ProbabilityDistributionSet>"#,
    r#"</StochasticDistribution>"#,
);

const TWO_ELEMENT_PROBABILITY_SET: &str = concat!(
    r#"<StochasticDistribution parameterName="p">"#,
    r#"<ProbabilityDistributionSet>"#,
    r#"<Element value="a" weight="0.5"/>"#,
    r#"<Element value="b" weight="0.5"/>"#,
    r#"</ProbabilityDistributionSet>"#,
    r#"</StochasticDistribution>"#,
);

const ONE_BIN_HISTOGRAM: &str = concat!(
    r#"<StochasticDistribution parameterName="p">"#,
    r#"<Histogram>"#,
    r#"<Bin weight="0.5"><Range lowerLimit="0" upperLimit="1"/></Bin>"#,
    r#"</Histogram>"#,
    r#"</StochasticDistribution>"#,
);

const TWO_BIN_HISTOGRAM: &str = concat!(
    r#"<StochasticDistribution parameterName="p">"#,
    r#"<Histogram>"#,
    r#"<Bin weight="0.5"><Range lowerLimit="0" upperLimit="1"/></Bin>"#,
    r#"<Bin weight="0.5"><Range lowerLimit="1" upperLimit="2"/></Bin>"#,
    r#"</Histogram>"#,
    r#"</StochasticDistribution>"#,
);

const ZERO_BRANCH: &str = r#"<StochasticDistribution parameterName="p"></StochasticDistribution>"#;

const TWO_BRANCH: &str = concat!(
    r#"<StochasticDistribution parameterName="p">"#,
    r#"<NormalDistribution expectedValue="0" variance="1"/>"#,
    r#"<PoissonDistribution expectedValue="2"/>"#,
    r#"</StochasticDistribution>"#,
);

#[test]
fn single_element_probability_distribution_set_parses() {
    let dist: StochasticDistribution = quick_xml::de::from_str(ONE_ELEMENT_PROBABILITY_SET)
        .expect("a single <Element> child must parse");
    let _ = dist;
}

#[test]
fn two_element_probability_distribution_set_parses() {
    let dist: StochasticDistribution = quick_xml::de::from_str(TWO_ELEMENT_PROBABILITY_SET)
        .expect("two <Element> children must parse");
    let _ = dist;
}

#[test]
fn single_bin_histogram_parses() {
    let dist: StochasticDistribution =
        quick_xml::de::from_str(ONE_BIN_HISTOGRAM).expect("a single <Bin> child must parse");
    let _ = dist;
}

#[test]
fn two_bin_histogram_parses() {
    let dist: StochasticDistribution =
        quick_xml::de::from_str(TWO_BIN_HISTOGRAM).expect("two <Bin> children must parse");
    let _ = dist;
}

#[test]
fn two_element_probability_distribution_set_round_trips_byte_exact() {
    let dist: StochasticDistribution =
        quick_xml::de::from_str(TWO_ELEMENT_PROBABILITY_SET).expect("parse must succeed");
    let serialized = quick_xml::se::to_string(&dist).expect("serialize must succeed");
    assert_eq!(
        serialized.as_bytes(),
        TWO_ELEMENT_PROBABILITY_SET.as_bytes(),
        "serialized output must equal the source document byte-for-byte, including the sibling @parameterName attribute"
    );
}

#[test]
fn two_bin_histogram_round_trips_byte_exact() {
    let dist: StochasticDistribution =
        quick_xml::de::from_str(TWO_BIN_HISTOGRAM).expect("parse must succeed");
    let serialized = quick_xml::se::to_string(&dist).expect("serialize must succeed");
    assert_eq!(
        serialized.as_bytes(),
        TWO_BIN_HISTOGRAM.as_bytes(),
        "serialized output must equal the source document byte-for-byte, including the sibling @parameterName attribute"
    );
}

#[test]
fn zero_branch_document_is_rejected() {
    let result: Result<StochasticDistribution, _> = quick_xml::de::from_str(ZERO_BRANCH);
    let err = result.expect_err("a document with no distribution branch must be rejected");
    assert!(
        err.to_string().contains("missing field `$value`"),
        "unexpected error text: {err}"
    );
}

#[test]
fn two_branch_document_is_rejected() {
    let result: Result<StochasticDistribution, _> = quick_xml::de::from_str(TWO_BRANCH);
    let err = result.expect_err("a document with two distribution branches must be rejected");
    assert!(
        err.to_string().contains("duplicate field `$value`"),
        "unexpected error text: {err}"
    );
}
