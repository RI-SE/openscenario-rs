# openscenario-rs

A Rust library for parsing, validating, and manipulating [OpenSCENARIO](https://www.asam.net/standards/detail/openscenario/) files.

[![Crates.io](https://img.shields.io/crates/v/openscenario-rs)](https://crates.io/crates/openscenario-rs)
[![Documentation](https://docs.rs/openscenario-rs/badge.svg)](https://docs.rs/openscenario-rs)
[![License](https://img.shields.io/badge/license-GPLv3-blue.svg)](LICENSE)
[![Build Status](https://github.com/ashfaqfarooqui/openscenario-rs/workflows/CI/badge.svg)](https://github.com/ashfaqfarooqui/openscenario-rs/actions)

## Features

- Parse and serialize `.xosc` files (scenarios, catalogs, parameter variations)
- Type-safe data model covering actions, conditions, entities, and distributions
- Parameter resolution with mathematical expression support (`${param + 1}`)
- Catalog loading and reference resolution
- Optional builder API for programmatic scenario construction (`--features builder`)
- CLI tools: `xosc-validate`, `scenario_analyzer`

## Status

Core parsing and serialization is functional. Actions and conditions have broad but not complete coverage — see the implementation table in [docs/user_guide.md](docs/user_guide.md) for details.

## Quick Start

```toml
[dependencies]
openscenario-rs = "0.2.0"
```

### Parsing

```rust
use openscenario_rs::{parse_file, OpenScenarioDocumentType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let scenario = parse_file("scenario.xosc")?;

    println!("Author: {:?}", scenario.file_header.author);

    match scenario.document_type() {
        OpenScenarioDocumentType::Scenario => {
            if let Some(entities) = &scenario.entities {
                for entity in &entities.scenario_objects {
                    println!("Entity: {:?}", entity.name);
                }
            }
        }
        OpenScenarioDocumentType::Catalog => println!("Catalog file"),
        OpenScenarioDocumentType::ParameterVariation => println!("Parameter variation file"),
        _ => {}
    }

    Ok(())
}
```

### Analysis Tools

```bash
cargo run --bin scenario_analyzer -- scenario.xosc
cargo run --bin xosc-validate -- scenario.xosc
```

## Modules

- `types/` — OpenSCENARIO data types
- `parser/` — XML parsing and serialization
- `catalog/` — catalog loading and reference resolution
- `expression/` — expression evaluation
- `builder/` — programmatic scenario construction (feature-gated)

## Testing

```bash
cargo test
cargo test --features builder
```

## Documentation

- [User Guide](docs/user_guide.md)
- [Builder Guide](docs/builder_guide.md)
- [API Reference](docs/api_reference.md)
- [Design](docs/design.md)

## Contributing

Check existing patterns before adding new types. Add tests. Verify XML round-trip behavior.

## License & Attribution

The Rust source code is licensed under the [GNU General Public License v3.0](LICENSE).

### ASAM OpenSCENARIO Schema

`Schema/OpenSCENARIO.xsd` is published by ASAM e.V. and redistributed unchanged for validation purposes under the [ASAM license terms](https://www.asam.net/license/). See [`Schema/NOTICE`](Schema/NOTICE) for details.

### ALKS Test Scenarios

`tests/data/alks_scenario.xosc` originates from [openMSL/sl-3-1-osc-alks-scenarios](https://github.com/openMSL/sl-3-1-osc-alks-scenarios) (© BMW Group), licensed under [MPL 2.0](https://www.mozilla.org/en-US/MPL/2.0/). See [`tests/data/NOTICE`](tests/data/NOTICE) for details.
