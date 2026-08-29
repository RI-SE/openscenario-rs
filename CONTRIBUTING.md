# Contributing

Thanks for looking at this. `openscenario-rs` is a schema-driven crate, which shapes almost
every convention below: the OpenSCENARIO 1.3 XSD is the specification, and the Rust types
follow it rather than approximating it.

## Getting set up

The crate requires Rust **1.90** or later (`rust-version` in `Cargo.toml`).

```bash
cargo build --features builder,validation --all-targets
cargo test
cargo test --features builder,validation
cargo fmt --check
cargo clippy --features builder,validation --all-targets
```

The build should be clean. If `cargo build` fails on a fresh checkout, that is a bug worth
reporting rather than something to work around.

Neither feature is enabled by default, so a change touching `src/builder/` or
`src/validation.rs` needs the feature flags on the command line or it will not be compiled at
all – which is an easy way to land code that does not build.

## The one rule that matters

**The schema decides.** Before adding or changing a type, read its declaration in
`Schema/OpenSCENARIO.xsd`. Three consequences follow:

- A field the schema marks optional is `Option<T>`. A field it marks required is not.
- No default is invented beyond what the schema defines. A `Default` that fabricates a
  position or a speed produces a document that parses cleanly and describes something nobody
  wrote, which is worse than a compile error.
- A type with no counterpart in the XSD does not belong in the crate, however convenient it
  would be. Several such types were removed in the last conformance pass.

## Conventions for a new type

The [type system guide](docs/type_system_guide.md) covers these in full; the summary:

| Schema shape | Rust |
|---|---|
| Attribute | `#[serde(rename = "@name")]`, where the `@` is not optional |
| Child element | `#[serde(rename = "ElementName")]` |
| `minOccurs="0"` | `Option<T>` with `skip_serializing_if = "Option::is_none"` |
| `maxOccurs` above one | `Vec<T>` with `default`, usually plus `skip_serializing_if = "Vec::is_empty"` |
| Required | The bare type, no `default` |
| Enumeration | A unit enum with each variant renamed to its schema value |

Two further points that cause real bugs:

**A missing `@` compiles.** It produces a struct that parses without complaint and silently
drops every attribute it was meant to read. Nothing in the crate uses
`deny_unknown_fields`, so serde will not tell you.

**Choice-group variant names must equal the XML element name.** Not the Rust type name, and
not the schema's `complexType` name. Those are different namespaces and they diverge –
`TransitionDynamics` appears on the wire as `<SpeedActionDynamics>`. Naming a variant after
the type emits an element the schema has never heard of.

Two idioms exist for choice groups: parallel `Option` fields with a hand-written
`validate()` / `get_action_type()` pair, and `#[serde(flatten)]` over an externally tagged
enum. Match whichever the surrounding code uses for that part of the tree.

## Tests

Every new or corrected type needs a round-trip test: build the value, serialize it, parse it
back, and compare. `tests/` is organized by area (conditions, positions, actions, catalogs,
entity selection, builders), so put it next to its neighbors rather than in a new file.

A flattened choice group additionally belongs in `tests/choice_flatten_roundtrip_test.rs`,
which asserts the emitted element name for every such site.

## The conformance gates

Three gates run against the corpus in the sibling crate `../test`
(`openscenario-roundtrip-harness`), which depends on this crate by path with the `validation`
feature. All three exit non-zero on failure.

| Command | Compares | Catches |
|---|---|---|
| `cargo run --bin report` | `xml1` vs `xml2` | parse failures, instability across serialization |
| `cargo run --bin lossy` | the original file vs `xml1` | data dropped or invented on the first parse |
| `cargo run --bin validate` | `xml1` vs the XSD | schema-invalid output |

Run them from `../test`. `lossy` is the one to check after adding a type, because it is the
only gate that sees first-parse data loss.

Be careful how you read a green `report` run. serde drops unknown XML on every pass
identically, so the round-trip comparison still succeeds over data the types never modeled.
The corpus reported 172/172 for a long time while discarding route positions, vehicle light
states and global actions. A green `report` means *stable*, not *lossless*.

The corpus also covers only about half the schema – 156 of 294 element declarations. If you
add a type and `lossy` does not move, that is expected, but it also means the corpus cannot
validate your work and unit tests are the only check that ran.

For the audit methods that find gaps in the first place, and the shell caveats that bite while
running them, see [docs/xsd_gaps.md](docs/xsd_gaps.md).

## Local schema validation

Without the sibling harness, the bundled binary validates individual files:

```bash
cargo run --bin xosc-validate --features validation -- tests/data/alks_scenario.xosc
```

It takes `--recursive` for a directory and `--format json|junit` for machine-readable output.

## Commits and pull requests

Commit messages follow the conventional-commit prefixes already in the history (`feat`,
`fix`, `refactor`, `docs`), with a scope naming the area: `fix(schema):`, `feat(actions):`,
`refactor(types):`. Write the subject as what the change does, not what it touches.

A pull request should say which schema declaration it follows, name the gates it ran, and note
any conformance gap it knowingly leaves open. Breaking changes belong in
[CHANGELOG.md](CHANGELOG.md) under `Unreleased`; the crate has downstream users and removed
types are otherwise invisible until something fails to compile.

## Documentation

Documentation drifts faster than code, and this repository has been through one large
correction already. When you change public API, update the guide that covers it in the same
change. Every type name, function signature and cargo command in `docs/` should be checkable
against the source – if you cannot grep for it, do not write it.
