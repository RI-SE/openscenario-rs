use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestDeterministic {
    #[serde(rename = "DeterministicSingleParameterDistribution")]
    pub single_distributions: Vec<TestSingle>,
    #[serde(rename = "DeterministicMultiParameterDistribution")]
    pub multi_distributions: Vec<TestMulti>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestSingle {
    #[serde(rename = "@parameterName")]
    pub parameter_name: openscenario_rs::types::basic::Value<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestMulti {
    // Empty for now
}

fn main() {
    let xml = r#"<Deterministic>
      <DeterministicSingleParameterDistribution parameterName="Road">
      </DeterministicSingleParameterDistribution>
      <DeterministicSingleParameterDistribution parameterName="Speed">
      </DeterministicSingleParameterDistribution>
      <DeterministicMultiParameterDistribution>
      </DeterministicMultiParameterDistribution>
    </Deterministic>"#;

    println!("Testing XML: {}", xml);
    
    match quick_xml::de::from_str::<TestDeterministic>(xml) {
        Ok(result) => {
            println!("✅ Success: {:?}", result);
            println!("Single distributions: {}", result.single_distributions.len());
            println!("Multi distributions: {}", result.multi_distributions.len());
        }
        Err(e) => {
            println!("❌ Error: {:?}", e);
        }
    }
}