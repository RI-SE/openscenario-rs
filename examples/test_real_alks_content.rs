use openscenario_rs::parser::xml::parse_from_str;

fn main() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<OpenSCENARIO>
  <FileHeader revMajor="1" revMinor="3" date="2023-07-26T09:32:28.775404" description="ALKS Scenario 4.2_2 PartiallyBlockingTarget template variation" author="BMW Group" />
  <ParameterValueDistribution>
    <ScenarioFile filepath="./concrete_scenarios/alks_scenario_4_2_2_partially_blocking_target_template.xosc" />
    <Deterministic>
      <DeterministicSingleParameterDistribution parameterName="Road">
        <DistributionSet>
          <Element value="./road_networks/alks_road_straight.xodr" />
          <Element value="./road_networks/alks_road_left_radius_250m.xodr" />
          <Element value="./road_networks/alks_road_right_radius_250m.xodr" />
          <Element value="./road_networks/alks_road_left_radius_1000m.xodr" />
          <Element value="./road_networks/alks_road_right_radius_1000m.xodr" />
        </DistributionSet>
      </DeterministicSingleParameterDistribution>
      <DeterministicSingleParameterDistribution parameterName="Ego_InitSpeed_Ve0_kph">
        <!--Variation between 5 kph and 60 kph with a step width of 5 kph-->
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
          <ParameterValueSet>
            <ParameterAssignment parameterRef="TargetBlocking_Catalog" value="vehicle_catalog" />
            <ParameterAssignment parameterRef="TargetBlocking_Model" value="car" />
          </ParameterValueSet>
        </ValueSetDistribution>
      </DeterministicMultiParameterDistribution>
      <DeterministicSingleParameterDistribution parameterName="TargetBlocking_InitPosition_LateralOffset_m">
        <!--Variation between left and right border of lane (3.5 m width)-->
        <DistributionRange stepWidth="0.25">
          <Range lowerLimit="-2" upperLimit="2" />
        </DistributionRange>
      </DeterministicSingleParameterDistribution>
    </Deterministic>
  </ParameterValueDistribution>
</OpenSCENARIO>"#;

    println!("🔍 Testing real ALKS content with parse_from_str...");
    
    match parse_from_str(xml) {
        Ok(scenario) => {
            println!("✅ Success!");
            if let Some(pvd) = &scenario.parameter_value_distribution {
                if let Some(det) = &pvd.deterministic {
                    println!("📊 Single distributions: {}", det.single_distributions.len());
                    println!("📊 Multi distributions: {}", det.multi_distributions.len());
                } else {
                    println!("❌ No deterministic found");
                }
            } else {
                println!("❌ No parameter value distribution found");
            }
        },
        Err(e) => {
            println!("❌ Error: {:?}", e);
        }
    }
}