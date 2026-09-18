//! Guards the `Properties` container shared by vehicles, controllers, and catalog
//! controllers. The crate used to carry three separate models of XSD `Properties`,
//! two of which declared only `Property` and dropped `File`/`CustomContent`, and one
//! of which lacked `#[serde(default)]` on its `Vec` so an absent `<Property>` list was
//! a parse error instead of an empty one. These tests exercise the surviving,
//! consolidated type through the two places that carry it: a scenario `Controller`
//! and a catalog `CatalogController`.

use openscenario_rs::types::catalogs::controllers::CatalogController;
use openscenario_rs::types::controllers::Controller;
use openscenario_rs::types::entities::vehicle::Properties;

#[test]
fn empty_properties_parses_everywhere_it_appears() {
    let parsed: Properties = quick_xml::de::from_str("<Properties/>")
        .expect("an empty <Properties/> is schema-valid and must parse");
    assert!(parsed.properties.is_empty());
    assert!(parsed.files.is_empty());
    assert!(parsed.custom_content.is_empty());

    let controller: Controller =
        quick_xml::de::from_str(r#"<Controller name="C"><Properties/></Controller>"#)
            .expect("a scenario controller with no properties is schema-valid");
    assert!(controller.properties.unwrap().properties.is_empty());

    let catalog_controller: CatalogController =
        quick_xml::de::from_str(r#"<Controller name="C"><Properties/></Controller>"#)
            .expect("a catalog controller with no properties is schema-valid");
    assert!(catalog_controller.properties.unwrap().properties.is_empty());
}

#[test]
fn catalog_controller_keeps_file_and_custom_content_children() {
    let xml = concat!(
        "<Controller name=\"C\">",
        "<Properties>",
        "<Property name=\"p\" value=\"v\"/>",
        "<File filepath=\"f.xml\"/>",
        "<CustomContent>payload</CustomContent>",
        "</Properties>",
        "</Controller>"
    );

    let controller: CatalogController =
        quick_xml::de::from_str(xml).expect("this document is schema-valid");
    let properties = controller.properties.as_ref().expect("Properties present");
    assert_eq!(properties.properties.len(), 1, "Property child dropped");
    assert_eq!(properties.files.len(), 1, "File child dropped");
    assert_eq!(
        properties.custom_content.len(),
        1,
        "CustomContent child dropped"
    );

    let serialized = quick_xml::se::to_string(&controller).expect("serializes");
    assert_eq!(serialized, xml, "round trip must be byte-identical");
}
