use openscenario_rs::types::scenario::init::Init;

fn main() {
    let xml = r#"
    <Init>
        <Actions/>
    </Init>
    "#;

    match quick_xml::de::from_str::<Init>(xml) {
        Ok(init) => println!("Success: {:?}", init),
        Err(e) => println!("Error: {:?}", e),
    }
}
