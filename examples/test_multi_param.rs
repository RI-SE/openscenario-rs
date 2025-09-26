use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestParameterValueSet {
    #[serde(rename = "ParameterAssignment")]
    pub parameter_assignments: Vec<TestParameterAssignment>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestParameterAssignment {
    #[serde(rename = "@parameterRef")]
    pub parameter_ref: String,
    #[serde(rename = "@value")]
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestValueSetDistribution {
    #[serde(rename = "ParameterValueSet")]
    pub parameter_value_sets: Vec<TestParameterValueSet>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestMultiParameterDistribution {
    #[serde(rename = "ValueSetDistribution")]
    pub distribution_type: TestValueSetDistribution,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestDeterministicWithMulti {
    #[serde(rename = "DeterministicMultiParameterDistribution")]
    pub multi_distributions: Vec<TestMultiParameterDistribution>,
}

fn main() {
    println!("🔍 Testing multi-parameter distributions...\n");

    // Test the multi-parameter distribution from the ALKS file
    let multi_xml = r#"<DeterministicMultiParameterDistribution>
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
      </DeterministicMultiParameterDistribution>"#;

    println!("Test 1: Single multi-parameter distribution");
    match quick_xml::de::from_str::<TestMultiParameterDistribution>(multi_xml) {
        Ok(result) => {
            println!("✅ Success: {} parameter value sets", result.distribution_type.parameter_value_sets.len());
        },
        Err(e) => {
            println!("❌ Error: {:?}", e);
        },
    }

    // Test multiple multi-parameter distributions in a container
    let multi_container_xml = r#"<Deterministic>
        <DeterministicMultiParameterDistribution>
            <ValueSetDistribution>
              <ParameterValueSet>
                <ParameterAssignment parameterRef="TargetBlocking_Catalog" value="pedestrian_catalog" />
                <ParameterAssignment parameterRef="TargetBlocking_Model" value="pedestrian" />
              </ParameterValueSet>
            </ValueSetDistribution>
        </DeterministicMultiParameterDistribution>
      </Deterministic>"#;

    println!("\nTest 2: Container with multi-parameter distribution");
    match quick_xml::de::from_str::<TestDeterministicWithMulti>(multi_container_xml) {
        Ok(result) => {
            println!("✅ Success: {} multi distributions", result.multi_distributions.len());
        },
        Err(e) => {
            println!("❌ Error: {:?}", e);
            println!("🔍 This might be where the issue is!");
        },
    }

    // Test mixed single and multi distributions
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct TestSingle {
        #[serde(rename = "@parameterName")]
        pub parameter_name: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct TestDeterministicMixed {
        #[serde(rename = "DeterministicSingleParameterDistribution")]
        pub single_distributions: Vec<TestSingle>,
        #[serde(rename = "DeterministicMultiParameterDistribution")]
        pub multi_distributions: Vec<TestMultiParameterDistribution>,
    }

    let mixed_xml = r#"<Deterministic>
        <DeterministicSingleParameterDistribution parameterName="Road">
        </DeterministicSingleParameterDistribution>
        <DeterministicMultiParameterDistribution>
            <ValueSetDistribution>
              <ParameterValueSet>
                <ParameterAssignment parameterRef="TargetBlocking_Catalog" value="pedestrian_catalog" />
                <ParameterAssignment parameterRef="TargetBlocking_Model" value="pedestrian" />
              </ParameterValueSet>
            </ValueSetDistribution>
        </DeterministicMultiParameterDistribution>
      </Deterministic>"#;

    println!("\nTest 3: Mixed single and multi distributions");
    match quick_xml::de::from_str::<TestDeterministicMixed>(mixed_xml) {
        Ok(result) => {
            println!("✅ Success: {} single, {} multi", result.single_distributions.len(), result.multi_distributions.len());
        },
        Err(e) => {
            println!("❌ Error: {:?}", e);
            println!("🔍 Mixed distributions might be the issue!");
        },
    }
}