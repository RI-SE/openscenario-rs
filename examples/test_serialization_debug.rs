use openscenario_rs::types::{
    conditions::entity::{EntityCondition, RelativeDistanceCondition, ByEntityCondition},
    basic::{Double, Boolean, OSString},
    enums::{RelativeDistanceType, Rule, CoordinateSystem},
    scenario::triggers::TriggeringEntities,
};

fn main() {
    println!("🧪 Testing XML Serialization...");

    // Test EntityCondition serialization directly
    let rel_dist_condition = RelativeDistanceCondition {
        entity_ref: OSString::literal("vehicle1".to_string()),
        value: Double::literal(5.0),
        freespace: Boolean::literal(true),
        relative_distance_type: RelativeDistanceType::Longitudinal,
        rule: Rule::GreaterThan,
        coordinate_system: Some(CoordinateSystem::Entity),
        routing_algorithm: None,
    };

    let entity_condition = EntityCondition::RelativeDistance(rel_dist_condition);
    
    match quick_xml::se::to_string(&entity_condition) {
        Ok(xml) => {
            println!("✅ EntityCondition XML:");
            println!("{}", xml);
            
            // Check if the fix worked - should have attributes instead of child elements
            if xml.contains("entityRef=") && xml.contains("value=") && xml.contains("freespace=") {
                println!("🎉 SUCCESS: RelativeDistanceCondition is using XML attributes!");
            } else {
                println!("❌ FAILED: Still using child elements instead of attributes");
                println!("Expected: entityRef=\"vehicle1\" value=\"5\" freespace=\"true\"");
            }
        }
        Err(e) => {
            println!("❌ EntityCondition serialization failed: {}", e);
        }
    }

    // Test ByEntityCondition (the wrapper) serialization
    let triggering_entities = TriggeringEntities::default();
    let by_entity_condition = ByEntityCondition {
        triggering_entities,
        entity_condition: EntityCondition::RelativeDistance(RelativeDistanceCondition {
            entity_ref: OSString::literal("vehicle1".to_string()),
            value: Double::literal(5.0),
            freespace: Boolean::literal(true),
            relative_distance_type: RelativeDistanceType::Longitudinal,
            rule: Rule::GreaterThan,
            coordinate_system: Some(CoordinateSystem::Entity),
            routing_algorithm: None,
        }),
    };

    match quick_xml::se::to_string(&by_entity_condition) {
        Ok(xml) => {
            println!("\n✅ ByEntityCondition XML:");
            println!("{}", xml);
            println!("\nThis should show EntityCondition element with RelativeDistanceCondition inside");
        }
        Err(e) => {
            println!("❌ ByEntityCondition serialization failed: {}", e);
        }
    }
}