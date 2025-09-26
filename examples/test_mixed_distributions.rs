use openscenario_rs::types::distributions::deterministic::Deterministic;

fn main() {
    println!("🔍 Testing mixed single and multi distributions...");
    
    // Test 1: Just 3 single distributions
    let xml_single_only = r#"<Deterministic>
  <DeterministicSingleParameterDistribution parameterName="Road">
    <DistributionSet>
      <Element value="./road_networks/alks_road_straight.xodr" />
    </DistributionSet>
  </DeterministicSingleParameterDistribution>
  <DeterministicSingleParameterDistribution parameterName="Speed">
    <DistributionRange stepWidth="5.0">
      <Range lowerLimit="5.0" upperLimit="60.0" />
    </DistributionRange>
  </DeterministicSingleParameterDistribution>
  <DeterministicSingleParameterDistribution parameterName="Position">
    <DistributionRange stepWidth="0.25">
      <Range lowerLimit="-2" upperLimit="2" />
    </DistributionRange>
  </DeterministicSingleParameterDistribution>
</Deterministic>"#;

    println!("\nTest 1: 3 single distributions");
    match quick_xml::de::from_str::<Deterministic>(xml_single_only) {
        Ok(d) => println!("✅ Success: {} single, {} multi", d.single_distributions.len(), d.multi_distributions.len()),
        Err(e) => println!("❌ Error: {:?}", e),
    }
    
    // Test 2: Mixed order exactly like ALKS
    let xml_mixed = r#"<Deterministic>
  <DeterministicSingleParameterDistribution parameterName="Road">
    <DistributionSet>
      <Element value="./road_networks/alks_road_straight.xodr" />
    </DistributionSet>
  </DeterministicSingleParameterDistribution>
  <DeterministicSingleParameterDistribution parameterName="Speed">
    <DistributionRange stepWidth="5.0">
      <Range lowerLimit="5.0" upperLimit="60.0" />
    </DistributionRange>
  </DeterministicSingleParameterDistribution>
  <DeterministicMultiParameterDistribution>
    <ValueSetDistribution>
      <ParameterValueSet>
        <ParameterAssignment parameterRef="TargetBlocking_Catalog" value="pedestrian_catalog" />
        <ParameterAssignment parameterRef="TargetBlocking_Model" value="pedestrian" />
      </ParameterValueSet>
    </ValueSetDistribution>
  </DeterministicMultiParameterDistribution>
  <DeterministicSingleParameterDistribution parameterName="Position">
    <DistributionRange stepWidth="0.25">
      <Range lowerLimit="-2" upperLimit="2" />
    </DistributionRange>
  </DeterministicSingleParameterDistribution>
</Deterministic>"#;

    println!("\nTest 2: Mixed order like ALKS");
    match quick_xml::de::from_str::<Deterministic>(xml_mixed) {
        Ok(d) => println!("✅ Success: {} single, {} multi", d.single_distributions.len(), d.multi_distributions.len()),
        Err(e) => println!("❌ Error: {:?}", e),
    }
}