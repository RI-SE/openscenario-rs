use serde::{Deserialize, Serialize};
use openscenario_rs::types::basic::Value;

// Let's test progressively more complex structures to isolate the issue

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestDistributionSet {
    #[serde(rename = "Element")]
    pub elements: Vec<TestElement>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestElement {
    #[serde(rename = "@value")]
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestDistributionRange {
    #[serde(rename = "@stepWidth")]
    pub step_width: String,
    #[serde(rename = "Range")]
    pub range: TestRange,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestRange {
    #[serde(rename = "@lowerLimit")]
    pub lower_limit: String,
    #[serde(rename = "@upperLimit")]
    pub upper_limit: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestSingleWithChildren {
    #[serde(rename = "@parameterName")]
    pub parameter_name: String,
    #[serde(rename = "DistributionSet", skip_serializing_if = "Option::is_none")]
    pub distribution_set: Option<TestDistributionSet>,
    #[serde(rename = "DistributionRange", skip_serializing_if = "Option::is_none")]
    pub distribution_range: Option<TestDistributionRange>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestDeterministicWithChildren {
    #[serde(rename = "DeterministicSingleParameterDistribution")]
    pub single_distributions: Vec<TestSingleWithChildren>,
}

fn main() {
    println!("🔍 Investigating XML structure parsing step by step...\n");

    // Test 1: Simple element array
    println!("Test 1: Simple Element array");
    let simple_elements_xml = r#"<DistributionSet>
          <Element value="./road_networks/alks_road_straight.xodr" />
          <Element value="./road_networks/alks_road_left_radius_250m.xodr" />
        </DistributionSet>"#;
    
    match quick_xml::de::from_str::<TestDistributionSet>(simple_elements_xml) {
        Ok(result) => println!("✅ Success: {} elements", result.elements.len()),
        Err(e) => println!("❌ Error: {:?}", e),
    }

    // Test 2: Distribution range
    println!("\nTest 2: Distribution range");
    let range_xml = r#"<DistributionRange stepWidth="5.0">
          <Range lowerLimit="5.0" upperLimit="60.0" />
        </DistributionRange>"#;
    
    match quick_xml::de::from_str::<TestDistributionRange>(range_xml) {
        Ok(result) => println!("✅ Success: stepWidth={}, range={}-{}", result.step_width, result.range.lower_limit, result.range.upper_limit),
        Err(e) => println!("❌ Error: {:?}", e),
    }

    // Test 3: Single distribution with DistributionSet
    println!("\nTest 3: Single distribution with DistributionSet");
    let single_with_set_xml = r#"<DeterministicSingleParameterDistribution parameterName="Road">
        <DistributionSet>
          <Element value="./road_networks/alks_road_straight.xodr" />
          <Element value="./road_networks/alks_road_left_radius_250m.xodr" />
        </DistributionSet>
      </DeterministicSingleParameterDistribution>"#;
    
    match quick_xml::de::from_str::<TestSingleWithChildren>(single_with_set_xml) {
        Ok(result) => {
            println!("✅ Success: parameter={}", result.parameter_name);
            if let Some(set) = &result.distribution_set {
                println!("   DistributionSet with {} elements", set.elements.len());
            }
        },
        Err(e) => println!("❌ Error: {:?}", e),
    }

    // Test 4: Single distribution with DistributionRange
    println!("\nTest 4: Single distribution with DistributionRange");
    let single_with_range_xml = r#"<DeterministicSingleParameterDistribution parameterName="Ego_InitSpeed_Ve0_kph">
        <DistributionRange stepWidth="5.0">
          <Range lowerLimit="5.0" upperLimit="60.0" />
        </DistributionRange>
      </DeterministicSingleParameterDistribution>"#;
    
    match quick_xml::de::from_str::<TestSingleWithChildren>(single_with_range_xml) {
        Ok(result) => {
            println!("✅ Success: parameter={}", result.parameter_name);
            if let Some(range) = &result.distribution_range {
                println!("   DistributionRange: step={}, range={}-{}", range.step_width, range.range.lower_limit, range.range.upper_limit);
            }
        },
        Err(e) => println!("❌ Error: {:?}", e),
    }

    // Test 5: Full Deterministic with multiple single distributions (this should reveal the issue)
    println!("\nTest 5: Full Deterministic with multiple single distributions");
    let full_deterministic_xml = r#"<Deterministic>
      <DeterministicSingleParameterDistribution parameterName="Road">
        <DistributionSet>
          <Element value="./road_networks/alks_road_straight.xodr" />
          <Element value="./road_networks/alks_road_left_radius_250m.xodr" />
        </DistributionSet>
      </DeterministicSingleParameterDistribution>
      <DeterministicSingleParameterDistribution parameterName="Ego_InitSpeed_Ve0_kph">
        <DistributionRange stepWidth="5.0">
          <Range lowerLimit="5.0" upperLimit="60.0" />
        </DistributionRange>
      </DeterministicSingleParameterDistribution>
    </Deterministic>"#;
    
    match quick_xml::de::from_str::<TestDeterministicWithChildren>(full_deterministic_xml) {
        Ok(result) => {
            println!("✅ Success: {} single distributions", result.single_distributions.len());
            for (i, dist) in result.single_distributions.iter().enumerate() {
                println!("   {}. parameter={}", i+1, dist.parameter_name);
            }
        },
        Err(e) => {
            println!("❌ Error: {:?}", e);
            println!("🔍 This is likely where the issue occurs!");
        },
    }

    // Test 6: Now test with Value<String> instead of String
    println!("\nTest 6: Testing with Value<String> (like our actual implementation)");
    
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct TestSingleWithValue {
        #[serde(rename = "@parameterName")]
        pub parameter_name: Value<String>,
        #[serde(rename = "DistributionSet", skip_serializing_if = "Option::is_none")]
        pub distribution_set: Option<TestDistributionSet>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct TestDeterministicWithValue {
        #[serde(rename = "DeterministicSingleParameterDistribution")]
        pub single_distributions: Vec<TestSingleWithValue>,
    }

    match quick_xml::de::from_str::<TestDeterministicWithValue>(full_deterministic_xml) {
        Ok(result) => {
            println!("✅ Success: {} single distributions", result.single_distributions.len());
        },
        Err(e) => {
            println!("❌ Error: {:?}", e);
            println!("🔍 Value<String> might be causing issues!");
        },
    }
}