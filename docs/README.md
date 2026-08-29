# Documentation

`openscenario-rs` parses, validates, constructs and serializes
[OpenSCENARIO](https://www.asam.net/standards/detail/openscenario/) files. It targets
**OpenSCENARIO 1.3** as defined by the bundled `Schema/OpenSCENARIO.xsd`, and the Rust type
model follows that schema rather than approximating it: fields the schema marks optional are
`Option<T>`, no default is invented beyond what the schema defines, and a type with no
counterpart in the XSD does not belong in the crate.

## The documents

| Document | Covers |
|---|---|
| [User guide](user_guide.md) | Installation, parsing, values and parameters, catalogs, error handling |
| [API reference](api_reference.md) | The public surface: signatures, modules, features, errors |
| [Builder guide](builder_guide.md) | Programmatic construction behind the `builder` feature |
| [Type system guide](type_system_guide.md) | `Value<T>`, serde conventions, choice groups, adding a type |
| [Validation guide](validation_guide.md) | The four validation layers and which one answers which question |
| [XSD gaps](xsd_gaps.md) | The conformance ledger: what the corpus proves, what remains open |
| [Development guide](development_guide.md) | Contributor patterns and troubleshooting |

Also in the repository: [`CONTRIBUTING.md`](../CONTRIBUTING.md) for the workflow and
conventions, [`CHANGELOG.md`](../CHANGELOG.md) for what changed between releases, and
[`examples/`](../examples/) for runnable programs covering each area of the crate.

## Where to start

**Reading and writing scenario files.** Start with the [user guide](user_guide.md), then keep
the [API reference](api_reference.md) open. The `basic_parsing` and `parse` examples are the
shortest path to working code.

**Constructing scenarios in Rust.** Read the [builder guide](builder_guide.md) and run
`builder_basic_demo`, then `builder_comprehensive_demo` when the detached style becomes
relevant.

**Contributing types or fixing conformance.** Read the
[type system guide](type_system_guide.md) for the conventions, then
[xsd_gaps.md](xsd_gaps.md) for the audit methods and the gates a change has to pass.
[`CONTRIBUTING.md`](../CONTRIBUTING.md) has the commands.

## One caveat worth reading first

The test corpus exercises **156 of the schema's 294 element declarations**, or about 53%. A
green round-trip run therefore means the crate is stable over what the corpus contains, not
that it is lossless over the standard. The distinction matters, and
[xsd_gaps.md](xsd_gaps.md) explains why: serde ignores unknown XML by default, so a field the
Rust types do not model is dropped identically on every pass and the comparison still succeeds.
The corpus reported a perfect score for a long time while silently discarding route positions,
vehicle light states and global actions.

Known gaps are tracked in one place, [xsd_gaps.md](xsd_gaps.md), rather than being scattered
through the guides.
