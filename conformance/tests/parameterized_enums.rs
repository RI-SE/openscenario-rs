//! Gate for the OSR-06 parameterized-enumeration fixtures.
//!
//! The corpus cannot cover this. All 172 corpus files spell every enum-typed attribute as a
//! literal, so `report`, `lossy` and `validate` were green for four audit passes on a crate
//! that rejected `vehicleCategory="$cat"` outright -- schema-valid input, hard parse error.
//! The corpus is also fetched and gitignored, so a file added to it would not survive.
//!
//! These two fixtures live in `tests/data/` and are driven through exactly the same three
//! questions the corpus binaries ask: does it round-trip to a fixed point, and does the
//! crate's own serialization of it validate against `Schema/OpenSCENARIO.xsd`?
//!
//! The schema check is the load-bearing half. `Value<T>` serializes through `Display`, so a
//! wrong parameter spelling would round-trip perfectly -- both directions agreeing on the
//! same invalid string -- and only libxml2 would notice. That is precisely how the braced
//! `${cat}` spelling would have slipped through: it is the schema's `expression` production,
//! which no enumeration union admits.

use openscenario_roundtrip_harness::{check_catalog_roundtrip, check_roundtrip, load_schema};
use openscenario_rs::{parse_catalog_from_file, serialize_catalog_to_string, serialize_to_string};
use std::path::PathBuf;

fn fixture(name: &str) -> String {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../tests/data")
        .join(name)
        .to_string_lossy()
        .into_owned()
}

#[test]
fn parameterized_enum_scenario_roundtrips_to_a_fixed_point() {
    let path = fixture("parameterized_enums.xosc");
    if let Err(e) = check_roundtrip(&path) {
        panic!("{path}\n{e}");
    }
}

#[test]
fn parameterized_enum_catalog_roundtrips_to_a_fixed_point() {
    let path = fixture("parameterized_enums_catalog.xosc");
    if let Err(e) = check_catalog_roundtrip(&path) {
        panic!("{path}\n{e}");
    }
}

#[test]
fn parameterized_enum_scenario_serializes_to_schema_valid_xml() {
    let mut validator = load_schema().expect("load Schema/OpenSCENARIO.xsd");
    let path = fixture("parameterized_enums.xosc");

    let doc = openscenario_rs::parse_from_file(&path).expect("fixture parses");
    let xml = serialize_to_string(&doc).expect("fixture serializes");

    let errors = validator.validate_str(&xml).expect("validator runs");
    assert!(
        errors.is_empty(),
        "{path}: {} schema violation(s):\n{}\n--- produced XML ---\n{xml}",
        errors.len(),
        errors
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn parameterized_enum_catalog_serializes_to_schema_valid_xml() {
    let mut validator = load_schema().expect("load Schema/OpenSCENARIO.xsd");
    let path = fixture("parameterized_enums_catalog.xosc");

    let doc = parse_catalog_from_file(&path).expect("catalog fixture parses");
    let xml = serialize_catalog_to_string(&doc).expect("catalog fixture serializes");

    let errors = validator.validate_str(&xml).expect("validator runs");
    assert!(
        errors.is_empty(),
        "{path}: {} schema violation(s):\n{}\n--- produced XML ---\n{xml}",
        errors.len(),
        errors
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}
