use openscenario_rs::types::distributions::ParameterValueDistribution;

fn main() {
    let xml = r#"<ParameterValueDistribution>
    <ScenarioFile filepath="./concrete_scenarios/test_template.xosc" />
    <Deterministic>
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
    </Deterministic>
  </ParameterValueDistribution>"#;

    println!("🔍 Testing ParameterValueDistribution deserialization...");

    match quick_xml::de::from_str::<ParameterValueDistribution>(xml) {
        Ok(dist) => {
            println!("✅ Success!");
            if let Some(det) = &dist.deterministic {
                println!(
                    "📊 Single distributions: {}",
                    det.single_distributions.len()
                );
                println!("📊 Multi distributions: {}", det.multi_distributions.len());
            } else {
                println!("❌ No deterministic found");
            }
        }
        Err(e) => {
            println!("❌ Error: {:?}", e);
        }
    }
}
