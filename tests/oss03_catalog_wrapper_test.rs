//! Regression: a `<Catalog>` element with two `<Vehicle>` children used to fail
//! to parse through the crate's `Catalog` type, which wrapped `CatalogContent`
//! behind `#[serde(flatten)]` for no reason the schema requires. `Catalog` in
//! `Schema/OpenSCENARIO.xsd` is an `xsd:sequence` of optional, repeatable
//! elements plus a required `@name` attribute — exactly what `CatalogContent`
//! already models directly, so the wrapper added indirection that broke
//! parsing without adding anything the schema asks for. The wrapper and its
//! matching `CatalogDefinition` have been removed; `CatalogContent` is now
//! parsed directly wherever a `Catalog` element appears.
use openscenario_rs::types::catalogs::CatalogContent;

const MULTI_VEHICLE_CATALOG_XML: &str = r#"<Catalog name="c">
    <Vehicle name="v1" vehicleCategory="car">
        <BoundingBox>
            <Center x="0" y="0" z="0"/>
            <Dimensions width="2" length="4" height="1.5"/>
        </BoundingBox>
        <Performance maxSpeed="50" maxDeceleration="10" maxAcceleration="5"/>
        <Axles>
            <FrontAxle maxSteering="0.5" wheelDiameter="0.6" trackWidth="1.8" positionX="1.3" positionZ="0.3"/>
            <RearAxle maxSteering="0" wheelDiameter="0.6" trackWidth="1.8" positionX="-1.3" positionZ="0.3"/>
        </Axles>
    </Vehicle>
    <Vehicle name="v2" vehicleCategory="car">
        <BoundingBox>
            <Center x="0" y="0" z="0"/>
            <Dimensions width="2" length="4" height="1.5"/>
        </BoundingBox>
        <Performance maxSpeed="50" maxDeceleration="10" maxAcceleration="5"/>
        <Axles>
            <FrontAxle maxSteering="0.5" wheelDiameter="0.6" trackWidth="1.8" positionX="1.3" positionZ="0.3"/>
            <RearAxle maxSteering="0" wheelDiameter="0.6" trackWidth="1.8" positionX="-1.3" positionZ="0.3"/>
        </Axles>
    </Vehicle>
</Catalog>"#;

#[test]
fn multi_vehicle_catalog_parses_directly_into_catalog_content() {
    let catalog: CatalogContent = quick_xml::de::from_str(MULTI_VEHICLE_CATALOG_XML)
        .expect("a Catalog with two Vehicle children must parse without a redundant wrapper");
    assert_eq!(catalog.vehicles.len(), 2);
    assert_eq!(catalog.vehicles[0].name, "v1");
    assert_eq!(catalog.vehicles[1].name, "v2");
}

#[test]
fn multi_vehicle_catalog_round_trips_byte_exact() {
    let catalog: CatalogContent =
        quick_xml::de::from_str(MULTI_VEHICLE_CATALOG_XML).expect("first parse must succeed");
    let serialized = quick_xml::se::to_string(&catalog).expect("serialize must succeed");
    let reparsed: CatalogContent =
        quick_xml::de::from_str(&serialized).expect("reparse must succeed");
    let reserialized = quick_xml::se::to_string(&reparsed).expect("reserialize must succeed");
    assert_eq!(
        serialized, reserialized,
        "serialize -> parse -> serialize must be a fixed point"
    );
}
