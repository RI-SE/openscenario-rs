# openscenario-rs

<p align="center">
<img src="docs/assets/openscenario-rs-badge-vertical.svg" alt="openscenario-rs logo" width="160"/>
</p>

A Rust library for parsing, validating, and manipulating [OpenSCENARIO](https://www.asam.net/standards/detail/openscenario/) files.

[![Crates.io](https://img.shields.io/crates/v/openscenario-rs)](https://crates.io/crates/openscenario-rs)
[![Documentation](https://docs.rs/openscenario-rs/badge.svg)](https://docs.rs/openscenario-rs)
[![License](https://img.shields.io/badge/license-GPLv3-blue.svg)](LICENSE)

## Features

- Parse and serialize `.xosc` files (scenarios, catalogs, parameter variations)
- Type-safe data model covering actions, conditions, entities, and distributions
- Parameter resolution with mathematical expression support (`${param + 1}`)
- Catalog loading and reference resolution
- Optional builder API for programmatic scenario construction (`--features builder`)
- CLI tools: `xosc-validate`, `scenario_analyzer`

## Supported OpenSCENARIO version

This crate targets **OpenSCENARIO 1.3**, as defined by the bundled schema in
[`Schema/OpenSCENARIO.xsd`](Schema/OpenSCENARIO.xsd) (1.4 support is planned). Field
optionality follows the XSD: attributes and elements the schema marks optional are `Option<T>`
in the Rust model, and no default values are invented beyond what the schema defines. Known
remaining gaps against the XSD are tracked in [docs/xsd_gaps.md](docs/xsd_gaps.md).

## Status

Core parsing and serialization is functional. The type model has been audited element by
element against the schema over four conformance passes; what the test corpus proves, what it
does not, and the one gap that remains open are recorded in
[docs/xsd_gaps.md](docs/xsd_gaps.md). Recent changes, including breaking ones, are in
[CHANGELOG.md](CHANGELOG.md).

## Quick Start

```toml
[dependencies]
openscenario-rs = "0.3.2"
```

Requires Rust 1.90 or later. Neither optional feature is enabled by default.

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
cargo run --bin xosc-validate --features validation -- scenario.xosc
```

`xosc-validate` requires the `validation` feature and exits non-zero on failure, so it can be
gated in CI without parsing its output.

## Modules

- `types/`: OpenSCENARIO data types
- `parser/`: XML parsing, serialization, and semantic validation
- `catalog/`: catalog loading and reference resolution
- `expression/`: expression evaluation
- `builder/`: programmatic scenario construction (feature `builder`)
- `validation/`: XSD schema validation (feature `validation`)

## Testing

```bash
cargo test
cargo test --features builder
```

## Documentation

Start at the [documentation index](docs/README.md).

- [User Guide](docs/user_guide.md): parsing, values and parameters, catalogs
- [API Reference](docs/api_reference.md): the public surface
- [Builder Guide](docs/builder_guide.md): programmatic construction
- [Type System Guide](docs/type_system_guide.md): how the types map to the XSD
- [Validation Guide](docs/validation_guide.md): the four validation layers
- [XSD Gaps](docs/xsd_gaps.md): the conformance ledger

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). In brief: follow the existing schema-mapping
conventions, add a round-trip test, and keep the conformance gates green.

## License & Attribution

The Rust source code is licensed under the [GNU General Public License v3.0](LICENSE).

### ASAM OpenSCENARIO Schema

`Schema/OpenSCENARIO.xsd` is published by ASAM e.V. and redistributed unchanged for validation purposes under the [ASAM license terms](https://www.asam.net/license/). See [`Schema/NOTICE`](Schema/NOTICE) for details.

### ALKS Test Scenarios

`tests/data/alks_scenario.xosc` originates from [openMSL/sl-3-1-osc-alks-scenarios](https://github.com/openMSL/sl-3-1-osc-alks-scenarios) (© BMW Group), licensed under [MPL 2.0](https://www.mozilla.org/en-US/MPL/2.0/). See [`tests/data/NOTICE`](tests/data/NOTICE) for details.

## Acknowledgement

<p align="center">
<img src="docs/assets/synergies.svg" alt="Synergies logo" width="200"/>
</p>

This package is developed as part of the [SYNERGIES](https://synergies-ccam.eu/) project.

<p align="center">
<img src="docs/assets/funded_by_eu.svg" alt="Funded by EU" width="200"/>
</p>

Funded by the European Union. Views and opinions expressed are however those of the author(s) only and do not necessarily reflect those of the European Union or European Climate, Infrastructure and Environment Executive Agency (CINEA). Neither the European Union nor the granting authority can be held responsible for them.
