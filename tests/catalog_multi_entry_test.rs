//! Regression: a `<Catalog>` element with two `<Vehicle>` children used to fail
//! to parse through the crate's `Catalog` type, which wrapped `CatalogContent`
//! behind `#[serde(flatten)]` for no reason the schema requires. `Catalog` in
//! `Schema/OpenSCENARIO.xsd` is an `xsd:sequence` of optional, repeatable
//! elements plus a required `@name` attribute — exactly what `CatalogContent`
//! already models directly, so the wrapper added indirection that broke
//! parsing without adding anything the schema asks for. The wrapper and its
//! matching `CatalogDefinition` have been removed; `CatalogContent` is now
//! parsed directly wherever a `Catalog` element appears.
use openscenario_rs::types::catalogs::CatalogFile;

/// A catalog file holding two `Vehicle` entries, in the compact form
/// `quick_xml::se::to_string` emits, so parsing it and serializing the
/// result again is a byte-exact round trip rather than merely a fixed
/// point.
const TWO_VEHICLE_CATALOG_FILE: &str = concat!(
    r#"<OpenSCENARIO>"#,
    r#"<FileHeader author="a" date="2024-01-01T00:00:00" description="d" revMajor="1" revMinor="3"/>"#,
    r#"<Catalog name="c">"#,
    r#"<Vehicle name="v1" vehicleCategory="car">"#,
    r#"<BoundingBox><Center x="0" y="0" z="0"/><Dimensions width="2" length="4" height="1.5"/></BoundingBox>"#,
    r#"<Performance maxSpeed="50" maxAcceleration="5" maxDeceleration="10"/>"#,
    r#"<Axles>"#,
    r#"<FrontAxle maxSteering="0.5" wheelDiameter="0.6" trackWidth="1.8" positionX="1.3" positionZ="0.3"/>"#,
    r#"<RearAxle maxSteering="0" wheelDiameter="0.6" trackWidth="1.8" positionX="-1.3" positionZ="0.3"/>"#,
    r#"</Axles>"#,
    r#"</Vehicle>"#,
    r#"<Vehicle name="v2" vehicleCategory="car">"#,
    r#"<BoundingBox><Center x="0" y="0" z="0"/><Dimensions width="2" length="4" height="1.5"/></BoundingBox>"#,
    r#"<Performance maxSpeed="50" maxAcceleration="5" maxDeceleration="10"/>"#,
    r#"<Axles>"#,
    r#"<FrontAxle maxSteering="0.5" wheelDiameter="0.6" trackWidth="1.8" positionX="1.3" positionZ="0.3"/>"#,
    r#"<RearAxle maxSteering="0" wheelDiameter="0.6" trackWidth="1.8" positionX="-1.3" positionZ="0.3"/>"#,
    r#"</Axles>"#,
    r#"</Vehicle>"#,
    r#"</Catalog>"#,
    r#"</OpenSCENARIO>"#,
);

#[test]
fn multi_vehicle_catalog_parses_without_the_flatten_wrapper() {
    let file: CatalogFile = quick_xml::de::from_str(TWO_VEHICLE_CATALOG_FILE)
        .expect("a Catalog with two Vehicle children must parse without a redundant wrapper");
    assert_eq!(file.catalog.vehicles.len(), 2);
    assert_eq!(file.catalog.vehicles[0].name.as_literal().unwrap(), "v1");
    assert_eq!(file.catalog.vehicles[1].name.as_literal().unwrap(), "v2");
}

#[test]
fn multi_vehicle_catalog_round_trips_byte_exact() {
    let file: CatalogFile =
        quick_xml::de::from_str(TWO_VEHICLE_CATALOG_FILE).expect("parse must succeed");
    let serialized = quick_xml::se::to_string(&file).expect("serialize must succeed");
    assert_eq!(
        serialized.as_bytes(),
        TWO_VEHICLE_CATALOG_FILE.as_bytes(),
        "serialized output must equal the source document byte-for-byte"
    );
}
