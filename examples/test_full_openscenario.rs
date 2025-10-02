use openscenario_rs::types::scenario::storyboard::OpenScenario;

fn main() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<OpenSCENARIO>
  <FileHeader revMajor="1" revMinor="0" date="2020-01-01T00:00:00" description="Test parameter value distribution" author="test"/>
  <ParameterValueDistribution>
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
  </ParameterValueDistribution>
</OpenSCENARIO>"#;

    println!("🔍 Testing full OpenSCENARIO deserialization...");

    match quick_xml::de::from_str::<OpenScenario>(xml) {
        Ok(scenario) => {
            println!("✅ Success!");
            if let Some(pvd) = &scenario.parameter_value_distribution {
                if let Some(det) = &pvd.deterministic {
                    println!(
                        "📊 Single distributions: {}",
                        det.single_distributions.len()
                    );
                    println!("📊 Multi distributions: {}", det.multi_distributions.len());
                } else {
                    println!("❌ No deterministic found");
                }
            } else {
                println!("❌ No parameter value distribution found");
            }
        }
        Err(e) => {
            println!("❌ Error: {:?}", e);
        }
    }
}
