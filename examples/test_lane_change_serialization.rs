use openscenario_rs::types::actions::movement::{LaneChangeAction, LaneChangeTarget, TransitionDynamics};
use openscenario_rs::types::basic::Double;
use openscenario_rs::types::enums::{DynamicsDimension, DynamicsShape};

fn main() {
    println!("🧪 Testing LaneChangeAction XML Serialization...");

    // Test 1: LaneChangeAction with None for target_lane_offset (should omit attribute)
    let lane_change = LaneChangeAction {
        target_lane_offset: None,
        lane_change_action_dynamics: TransitionDynamics {
            dynamics_dimension: DynamicsDimension::Time,
            dynamics_shape: DynamicsShape::Linear,
            value: Double::literal(2.0),
        },
        lane_change_target: LaneChangeTarget::relative("Ego", -1),
    };

    match quick_xml::se::to_string(&lane_change) {
        Ok(xml) => {
            println!("✅ LaneChangeAction XML (None offset):");
            println!("{}", xml);
            if xml.contains("targetLaneOffset=\"\"") {
                println!("❌ ERROR: Contains empty targetLaneOffset attribute!");
            } else if xml.contains("targetLaneOffset") {
                println!("❌ ERROR: Contains targetLaneOffset attribute when it should be omitted!");
            } else {
                println!("✅ GOOD: No targetLaneOffset attribute present");
            }
            println!();
        }
        Err(e) => {
            println!("❌ LaneChangeAction serialization failed: {}", e);
        }
    }

    // Test 2: LaneChangeAction with Some value for target_lane_offset
    let lane_change_with_offset = LaneChangeAction {
        target_lane_offset: Some(Double::literal(0.5)),
        lane_change_action_dynamics: TransitionDynamics {
            dynamics_dimension: DynamicsDimension::Time,
            dynamics_shape: DynamicsShape::Linear,
            value: Double::literal(2.0),
        },
        lane_change_target: LaneChangeTarget::relative("Ego", -1),
    };

    match quick_xml::se::to_string(&lane_change_with_offset) {
        Ok(xml) => {
            println!("✅ LaneChangeAction XML (with offset):");
            println!("{}", xml);
            if xml.contains("targetLaneOffset=\"0.5\"") {
                println!("✅ GOOD: Contains correct targetLaneOffset=\"0.5\"");
            } else {
                println!("❌ ERROR: Missing or incorrect targetLaneOffset attribute!");
            }
            println!();
        }
        Err(e) => {
            println!("❌ LaneChangeAction with offset serialization failed: {}", e);
        }
    }

    // Test 3: Test deserialization of XML with empty targetLaneOffset=""
    let test_xml = r#"<LaneChangeAction targetLaneOffset="">
        <LaneChangeActionDynamics dynamicsDimension="time" dynamicsShape="linear" value="2.0"/>
        <LaneChangeTarget>
            <RelativeTargetLane entityRef="Ego" value="-1"/>
        </LaneChangeTarget>
    </LaneChangeAction>"#;

    println!("🧪 Testing deserialization of XML with empty targetLaneOffset=\"\"...");
    match quick_xml::de::from_str::<LaneChangeAction>(test_xml) {
        Ok(deserialized) => {
            println!("✅ Deserialization successful!");
            println!("target_lane_offset: {:?}", deserialized.target_lane_offset);
            if deserialized.target_lane_offset.is_none() {
                println!("✅ GOOD: Empty string was deserialized as None");
            } else {
                println!("❌ ERROR: Empty string was not deserialized as None");
            }
        }
        Err(e) => {
            println!("❌ Deserialization failed: {}", e);
        }
    }
}