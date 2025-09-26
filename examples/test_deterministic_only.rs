use openscenario_rs::types::distributions::deterministic::Deterministic;

fn main() {
    let xml = r#"<Deterministic>
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
</Deterministic>"#;

    println!("🔍 Testing direct Deterministic deserialization...");
    
    match quick_xml::de::from_str::<Deterministic>(xml) {
        Ok(deterministic) => {
            println!("✅ Success!");
            println!("📊 Single distributions: {}", deterministic.single_distributions.len());
            println!("📊 Multi distributions: {}", deterministic.multi_distributions.len());
        },
        Err(e) => {
            println!("❌ Error: {:?}", e);
        }
    }
}