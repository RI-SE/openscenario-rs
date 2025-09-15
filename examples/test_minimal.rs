fn main() {
    let xml = std::fs::read_to_string("minimal_dist.xosc").unwrap();
    match openscenario_rs::parse_str(&xml) {
        Ok(_) => println!("✅ Minimal works"),
        Err(e) => println!("❌ Minimal fails: {}", e),
    }
}
