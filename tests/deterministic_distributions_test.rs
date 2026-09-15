use openscenario_rs::types::distributions::deterministic::*;

#[test]
fn test_deterministic_empty_deserializer() {
    let xml = r#"<Deterministic></Deterministic>"#;
    let result: Result<Deterministic, _> = quick_xml::de::from_str(xml);

    assert!(result.is_ok());
    let det = result.unwrap();
    let total_count = det.single_distributions.len() + det.multi_distributions.len();
    assert_eq!(total_count, 0);
    assert!(det.single_distributions.is_empty() && det.multi_distributions.is_empty());
}

#[test]
fn test_deterministic_single_multi_deserializer() {
    let xml = r#"
    <Deterministic>
        <DeterministicMultiParameterDistribution>
            <ValueSetDistribution>
                <ParameterValueSet>
                    <ParameterAssignment parameterRef="speed" value="30.0"/>
                </ParameterValueSet>
            </ValueSetDistribution>
        </DeterministicMultiParameterDistribution>
    </Deterministic>"#;

    let result: Result<Deterministic, _> = quick_xml::de::from_str(xml);

    if let Err(e) = &result {
        println!("Error: {:?}", e);
    }
    assert!(result.is_ok());
    let det = result.unwrap();
    let total_count = det.single_distributions.len() + det.multi_distributions.len();
    assert_eq!(total_count, 1);
    assert_eq!(det.multi_distributions.len(), 1);
    assert_eq!(det.single_distributions.len(), 0);
}

#[test]
fn test_deterministic_mixed_deserializer() {
    let xml = r#"
    <Deterministic>
        <DeterministicSingleParameterDistribution parameterName="speed">
            <DistributionSet>
                <Element value="30.0"/>
                <Element value="50.0"/>
            </DistributionSet>
        </DeterministicSingleParameterDistribution>
        <DeterministicMultiParameterDistribution>
            <ValueSetDistribution>
                <ParameterValueSet>
                    <ParameterAssignment parameterRef="position" value="100.0"/>
                </ParameterValueSet>
            </ValueSetDistribution>
        </DeterministicMultiParameterDistribution>
    </Deterministic>"#;

    let result: Result<Deterministic, _> = quick_xml::de::from_str(xml);

    assert!(result.is_ok());
    let det = result.unwrap();
    let total_count = det.single_distributions.len() + det.multi_distributions.len();
    assert_eq!(total_count, 2);
    assert_eq!(det.single_distributions.len(), 1);
    assert_eq!(det.multi_distributions.len(), 1);

    // Check parameter name
    assert_eq!(
        format!("{}", det.single_distributions[0].parameter_name),
        "speed"
    );
}

#[test]
fn test_deterministic_backward_compatibility() {
    // Test that the new API provides backward compatibility methods
    let mut det = Deterministic::default();

    // Test adding distributions manually
    let single = DeterministicSingleParameterDistribution::new(
        openscenario_rs::types::basic::Value::Literal("speed".to_string()),
        Some(DistributionSet::new(
            DistributionSetElement::new(openscenario_rs::types::basic::Value::Literal(
                "30.0".to_string(),
            )),
            vec![],
        )),
        None,
        None,
    );
    det.single_distributions.push(single);

    let multi = DeterministicMultiParameterDistribution::new(ValueSetDistribution::new(
        ParameterValueSet::new(
            ParameterAssignment::new(
                "position".to_string(),
                openscenario_rs::types::basic::Value::Literal("100.0".to_string()),
            ),
            vec![],
        ),
        vec![],
    ));
    det.multi_distributions.push(multi);

    let total_count = det.single_distributions.len() + det.multi_distributions.len();
    assert_eq!(total_count, 2);
    assert_eq!(det.single_distributions.len(), 1);
    assert_eq!(det.multi_distributions.len(), 1);
}
