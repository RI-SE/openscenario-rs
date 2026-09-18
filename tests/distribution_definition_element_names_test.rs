//! `DistributionDefinition` and `DistributionDefinitionGroup` both model XSD group
//! `DistributionDefinition` (`Schema/OpenSCENARIO.xsd:1086-1091`), a choice between the
//! `Deterministic` and `Stochastic` elements. Before the fix, both enums carried a
//! `Deterministic` branch typed as the per-item choice used inside `<Deterministic>`'s
//! sequence (`DeterministicParameterDistribution`, itself an externally-tagged enum),
//! rather than the `Deterministic` container itself. Nesting one externally-tagged enum
//! inside another does not serialize under quick-xml at all: it failed with
//! `Unsupported("cannot serialize enum newtype variant ...")` rather than emitting a
//! wrong tag, and deserializing the schema's actual element name
//! (`DeterministicSingleParameterDistribution`) failed with `unknown variant`. This
//! fixture pins the corrected shape: the branch holds the sequence container directly, so
//! serializing it produces the element name the schema declares.

use openscenario_rs::types::basic::Value;
use openscenario_rs::types::distributions::deterministic::{
    Deterministic, DeterministicSingleParameterDistribution, DistributionSet,
    DistributionSetElement,
};
use openscenario_rs::types::distributions::{DistributionDefinition, DistributionDefinitionGroup};

fn sample_deterministic() -> Deterministic {
    Deterministic {
        single_distributions: vec![DeterministicSingleParameterDistribution::new(
            Value::literal("speed".to_string()),
            Some(DistributionSet::new(
                DistributionSetElement::new(Value::literal("30.0".to_string())),
                vec![],
            )),
            None,
            None,
        )],
        multi_distributions: vec![],
    }
}

#[test]
fn distribution_definition_emits_the_schema_element_name() {
    let def = DistributionDefinition::Deterministic(sample_deterministic());

    let xml = quick_xml::se::to_string(&def).expect("serialize DistributionDefinition");

    // The schema-declared element is `<Deterministic>`. Before the fix the payload type
    // was the per-item choice (`DeterministicParameterDistribution`), whose own variant
    // name (`Single`) would have been emitted as a nested tag instead.
    assert!(
        xml.starts_with("<Deterministic>"),
        "expected the element the schema declares, got: {xml}"
    );
    assert!(!xml.contains("<Single>"), "got: {xml}");

    let round_tripped: DistributionDefinition =
        quick_xml::de::from_str(&xml).expect("deserialize DistributionDefinition");
    assert_eq!(def, round_tripped);
}

#[test]
fn distribution_definition_group_emits_the_schema_element_name() {
    let group = DistributionDefinitionGroup::deterministic(sample_deterministic());

    let xml = quick_xml::se::to_string(&group).expect("serialize DistributionDefinitionGroup");

    assert!(
        xml.starts_with("<Deterministic>"),
        "expected the element the schema declares, got: {xml}"
    );
    assert!(!xml.contains("<Single>"), "got: {xml}");

    let round_tripped: DistributionDefinitionGroup =
        quick_xml::de::from_str(&xml).expect("deserialize DistributionDefinitionGroup");
    assert_eq!(group, round_tripped);
}

#[test]
fn distribution_definition_has_no_user_defined_branch() {
    // XSD group `DistributionDefinition` is a choice of exactly two elements, `Deterministic`
    // and `Stochastic`. A third `UserDefined` branch was never in the schema; this test
    // exists so that reintroducing it (a compile-time change, not a runtime one) draws a
    // reviewer's attention here. Matching exhaustively on both real branches, with no
    // wildcard arm, fails to compile if a third variant reappears.
    let def = DistributionDefinition::Deterministic(sample_deterministic());
    match def {
        DistributionDefinition::Deterministic(_) => {}
        DistributionDefinition::Stochastic(_) => {}
    }
}
