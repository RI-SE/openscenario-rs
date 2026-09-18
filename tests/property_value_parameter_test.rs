//! `Property`'s `@name` and `@value` (XSD `Property`, `Schema/OpenSCENARIO.xsd:1809-1812`)
//! both use the schema's `String`, a union that includes the parameter member. A `Property`
//! whose attributes were plain Rust `String` parsed a document carrying `$speedLimit` and kept
//! the literal text, so nothing ever substituted the parameter: the document parsed, round-tripped
//! byte-identically, and meant the wrong thing. These tests guard against that regressing.

use openscenario_rs::types::basic::{OSString, Value};
use openscenario_rs::types::entities::vehicle::Property;
use std::collections::HashMap;

#[test]
fn property_value_with_literal_round_trips_byte_exact() {
    let xml = r#"<Property name="maxSpeed" value="30"/>"#;
    let property: Property = quick_xml::de::from_str(xml).expect("literal property parses");
    assert_eq!(property.name, OSString::literal("maxSpeed".to_string()));
    assert_eq!(property.value, OSString::literal("30".to_string()));

    let serialized = quick_xml::se::to_string(&property).expect("serializes");
    assert_eq!(serialized, xml);
}

#[test]
fn property_value_with_parameter_reference_round_trips_byte_exact() {
    let xml = r#"<Property name="maxSpeed" value="$speedLimit"/>"#;
    let property: Property = quick_xml::de::from_str(xml).expect("parameterized property parses");
    assert_eq!(
        property.value,
        Value::Parameter("speedLimit".to_string()),
        "the parameter reference must parse into the Parameter variant, not a literal string"
    );

    let serialized = quick_xml::se::to_string(&property).expect("serializes");
    assert_eq!(serialized, xml);
}

#[test]
fn property_value_parameter_reference_resolves_through_the_existing_path() {
    let xml = r#"<Property name="maxSpeed" value="$speedLimit"/>"#;
    let property: Property = quick_xml::de::from_str(xml).unwrap();

    let mut params = HashMap::new();
    params.insert("speedLimit".to_string(), "120".to_string());

    // `Value<T>::resolve` (src/types/basic.rs) is the crate's parameter-resolution path,
    // used throughout src/types for every other parameterizable attribute. Property's
    // value now goes through the same code, not a second one.
    let resolved: String = property.value.resolve(&params).expect("parameter resolves");
    assert_eq!(resolved, "120");
}

#[test]
fn property_name_can_also_be_a_parameter_reference() {
    // XSD types @name with the same `String` union as @value (`:1810` vs `:1811`), so a
    // parameterized name is equally schema-valid.
    let xml = r#"<Property name="$propName" value="30"/>"#;
    let property: Property = quick_xml::de::from_str(xml).unwrap();
    assert_eq!(property.name, Value::Parameter("propName".to_string()));

    let mut params = HashMap::new();
    params.insert("propName".to_string(), "maxSpeed".to_string());
    let resolved: String = property.name.resolve(&params).expect("parameter resolves");
    assert_eq!(resolved, "maxSpeed");
}
