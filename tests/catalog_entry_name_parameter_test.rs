//! Regression: `@name` on the nine catalog entry types (`CatalogVehicle`,
//! `CatalogController` (both `catalogs::entities` and `catalogs::controllers`),
//! `CatalogPedestrian`, `CatalogMiscObject`, `CatalogManeuver`, `CatalogRoute`,
//! `CatalogTrajectory`, `CatalogEnvironment`) used to be a plain Rust `String`,
//! though every one mirrors an XSD complex type whose `@name` is the schema's
//! `String`, a union including the parameter production. A document naming a
//! catalog entry with `$paramName` therefore parsed and round-tripped
//! byte-identically, but the literal text `$paramName` was carried as data and
//! never substituted. The field is now `OSString`, so a catalog entry can name
//! itself with a parameter reference that resolves through the same path every
//! other parameterizable attribute already uses.
use openscenario_rs::catalog::ParameterSubstitutionEngine;
use openscenario_rs::types::catalogs::entities::CatalogVehicle;
use openscenario_rs::types::catalogs::CatalogFile;
use std::collections::HashMap;

const VEHICLE_WITH_PARAMETERIZED_NAME: &str = concat!(
    r#"<Vehicle name="$vehicleName" vehicleCategory="car">"#,
    r#"<BoundingBox><Center x="0" y="0" z="0"/><Dimensions width="2" length="4" height="1.5"/></BoundingBox>"#,
    r#"<Performance maxSpeed="50" maxAcceleration="5" maxDeceleration="10"/>"#,
    r#"<Axles>"#,
    r#"<RearAxle maxSteering="0" wheelDiameter="0.6" trackWidth="1.8" positionX="-1.3" positionZ="0.3"/>"#,
    r#"</Axles>"#,
    r#"</Vehicle>"#,
);

const LITERAL_NAME_CATALOG_FILE: &str = concat!(
    r#"<OpenSCENARIO>"#,
    r#"<FileHeader author="a" date="2024-01-01T00:00:00" description="d" revMajor="1" revMinor="3"/>"#,
    r#"<Catalog name="c">"#,
    r#"<Vehicle name="SportsCar" vehicleCategory="car">"#,
    r#"<BoundingBox><Center x="0" y="0" z="0"/><Dimensions width="2" length="4" height="1.5"/></BoundingBox>"#,
    r#"<Performance maxSpeed="50" maxAcceleration="5" maxDeceleration="10"/>"#,
    r#"<Axles>"#,
    r#"<RearAxle maxSteering="0" wheelDiameter="0.6" trackWidth="1.8" positionX="-1.3" positionZ="0.3"/>"#,
    r#"</Axles>"#,
    r#"</Vehicle>"#,
    r#"</Catalog>"#,
    r#"</OpenSCENARIO>"#,
);

/// A `$vehicleName` catalog entry name parses as a parameter reference, not as
/// the literal text `$vehicleName`, and resolves through
/// `ParameterSubstitutionEngine::resolve_value` exactly as any other
/// parameterizable attribute does.
#[test]
fn catalog_entry_name_parameter_reference_resolves() {
    let vehicle: CatalogVehicle = quick_xml::de::from_str(VEHICLE_WITH_PARAMETERIZED_NAME)
        .expect("a Vehicle whose @name is a parameter reference must still parse");

    assert!(
        vehicle.name.as_parameter() == Some("vehicleName"),
        "name must parse as a parameter reference, not a literal"
    );

    let mut context = HashMap::new();
    context.insert("vehicleName".to_string(), "SportsCar".to_string());
    let engine = ParameterSubstitutionEngine::with_context(context);

    let resolved = engine
        .resolve_value(&vehicle.name)
        .expect("the parameter must resolve against the substitution context");
    assert_eq!(resolved, "SportsCar");
}

/// A catalog entry carrying a literal `@name` round-trips byte-for-byte,
/// parsed and reserialized through the same `CatalogFile` wrapper the
/// production path uses (a bare `CatalogVehicle` serializes under its Rust
/// type name rather than `Vehicle`, so the wrapper is load-bearing here).
#[test]
fn catalog_entry_literal_name_round_trips_byte_exact() {
    let file: CatalogFile =
        quick_xml::de::from_str(LITERAL_NAME_CATALOG_FILE).expect("parse must succeed");
    assert_eq!(
        file.catalog.vehicles[0].name.as_literal().unwrap(),
        "SportsCar"
    );

    let serialized = quick_xml::se::to_string(&file).expect("serialize must succeed");
    assert_eq!(
        serialized.as_bytes(),
        LITERAL_NAME_CATALOG_FILE.as_bytes(),
        "serialized output must equal the source document byte-for-byte"
    );
}
