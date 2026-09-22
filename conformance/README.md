# openscenario-roundtrip-harness

This crate is the `conformance` workspace member: a corpus of real `.xosc` files plus six
binaries that run this repo's types against them. It depends on `openscenario-rs` by path with
the `builder` and `validation` features, so it always exercises the working tree, never a
published version. See [../CONTRIBUTING.md](../CONTRIBUTING.md) for the day-to-day commands and
[../docs/xsd_gaps.md](../docs/xsd_gaps.md) for the audit methods and the current known gap.

## The corpus is not vendored

`corpus/` is gitignored. It is fetched on demand:

```bash
bash scripts/fetch-corpus.sh          # from the repo root, one time
bash scripts/fetch-corpus.sh --force  # wipe and re-fetch
```

The corpus is third-party content from three upstream repositories, each pinned to a commit SHA
in `scripts/fetch-corpus.sh` so the fetch is reproducible:

| Source | Licence | Pinned SHA |
|---|---|---|
| [asam-oss/OSC-ALKS-scenarios](https://github.com/asam-oss/OSC-ALKS-scenarios) | MPL-2.0 | `b49dc1dd1750692502c3fc98436f19c6a16ee9f1` |
| [vectorgrp/OSC-NCAP-scenarios](https://github.com/vectorgrp/OSC-NCAP-scenarios) | MPL-2.0 | `15365d18bd7d1d6aff46c75938eddaf4325ac8f3` |
| [eclipse/openpass/openscenario1_engine](https://gitlab.eclipse.org/eclipse/openpass/openscenario1_engine) | **EPL-2.0** | `f51968308e464fd8ebdbe5aea6323209d186c70c` |

Two are MPL-2.0 and the third is EPL-2.0. `openscenario-rs` is **GPL-3.0-only**. Both of those
are file-level copyleft licenses that do not propagate onto a work merely built alongside them —
but committing the corpus into this repository would still mean redistributing someone else's
licensed files under terms this repo does not control, and that reasoning does not depend on
which licence it is. Fetching on demand keeps each corpus's licence and provenance with the files
themselves: every clone carries its own upstream `LICENSE` next to the `.xosc` files it supplies,
and none of those third-party files — MPL-2.0 or EPL-2.0 — ever lands in this repo's git history.
The openpass clone is fetched with a cone-mode sparse checkout of `engine/tests/data`; cone mode
also materialises every ancestor-level file, which is how its `LICENSE` and `NOTICE.md` land in
the clone rather than being sparsed away.

Without the corpus, `report`, `lossy`, `validate`, `validate-input` and `mutate` print a hint and
exit 1;
`cargo test -p openscenario-roundtrip-harness` generates zero corpus tests and still runs the
builder-fixture gate; `builder` (without `--coverage`) needs no corpus at all.

## The six binaries

```bash
cargo run -p openscenario-roundtrip-harness --bin report
cargo run -p openscenario-roundtrip-harness --bin lossy
cargo run -p openscenario-roundtrip-harness --bin validate
cargo run -p openscenario-roundtrip-harness --bin validate-input
cargo run -p openscenario-roundtrip-harness --bin mutate
cargo run -p openscenario-roundtrip-harness --bin builder
cargo run -p openscenario-roundtrip-harness --bin builder -- --coverage
```

| Binary | What it checks |
|---|---|
| `report` | Every corpus file parses, serializes, reparses and reserializes to the same XML twice in a row (a round-trip fixed point). |
| `lossy` | The *first* serialization against the *original* file, so it can see data dropped or invented on the initial parse — the one thing `report` cannot see. It checks two things: names (did an element or attribute disappear or appear) and values (did an attribute or text value change at the same name). A number reserialized in a different lexical form, such as `1.0` becoming `1`, is reported separately and does not fail the gate, since the XSD types are numeric and the value space is unchanged. |
| `validate` | The serialized output against `Schema/OpenSCENARIO.xsd`. |
| `validate-input` | The corpus file *as it sits on disk* against `Schema/OpenSCENARIO.xsd`. The only gate that never parses the file with this crate, so its verdict is a fact about the corpus. It exists because a schema-invalid input whose invalid part the crate does not model is parsed, the content is dropped, and the output validates — `validate` then goes green *because* something was lost. Added by OSP-14. |
| `mutate` | The other half of the question: does the crate **refuse** what the schema refuses? It breaks each schema-valid corpus file in XSD-guided ways — dropping a required child or a required attribute, duplicating a child whose `maxOccurs` is 1, adding a second branch to a choice, writing a value outside an enumeration — keeps the mutants libxml2 rejects, and requires the crate to reject them too. A mutant the crate accepts is a **hole**, reported by mutation kind and XSD type. It also runs an unmutated control per file, since refusing a valid document is a defect in the other direction. |
| `builder` | Every builder fixture through the same three questions (round trip, fidelity, schema), using code as the corpus instead of files. With `--coverage`, reports how much of one real scenario the builder can reconstruct — a figure, not a gate. |

## Expected failures: `expectations.toml`

Absent any entry in `conformance/expectations.toml`, a corpus file is expected to parse,
round-trip losslessly, and serialize to schema-valid XML. The manifest records the handful of
files whose expected outcome is something else — and it records *why*, because an exclusion that
asserts nothing is a silent skip by another name, and `lossy` and `validate` deliberately count a
skip as a failure.

### The two categories

They are opposites, and the distinction is the point:

| `assert` | Premise about the **input**, checked against `Schema/OpenSCENARIO.xsd` with the crate's own validator |
|---|---|
| `input-schema-invalid` | The input file **must fail** XSD validation. The data is bad on purpose. If upstream fixes it when the pinned SHA moves, this trips. |
| `crate-defect` | The input file **must be schema-valid**. The file is fine; this crate is not. `reason` must name the issue tracking the fix. |

There is no "upstream-broken" category and no way to exclude a file without stating which of
these two things is true about it.

### Exemptions are per gate

`gates` names the binaries a file is exempt from — `"report"` (which also suppresses the
round-trip test `build.rs` generates for it), `"lossy"`, `"validate"` (the crate's *output*
against the XSD), `"validate-input"` (the file *as it sits on disk* against the XSD). A file
exempt from one is still live on the others and must still pass them.

`"validate"` and `"validate-input"` ask different questions and an entry needs whichever one it
actually fails. Being exempt from `"validate-input"` while staying live on `"validate"` is the
normal shape of a deliberately invalid fixture the crate reproduces faithfully; for an
`assert = "crate-defect"` entry, whose premise is that the input is valid, a `"validate-input"`
exemption can never be right.

| File (under `corpus/openscenario1-engine/engine/tests/data/Scenarios/`) | `gates` | `assert` | Why |
|---|---|---|---|
| `AutomatedLaneKeepingSystemScenarios/Invalid.xosc` | report, lossy, validate, validate-input | `input-schema-invalid` | Deliberately invalid upstream fixture with a bare `<ParameterDeclaration />` missing required attributes. The crate correctly refuses to parse it, so the other gates could only skip it. |
| `OSC_1_3_test_invalid.xosc` | report, lossy, validate, validate-input | `input-schema-invalid` | Deliberately invalid upstream fixture with `<ScenarioObject>` missing its required `EntityObject` child. Used to round-trip losslessly as parallel `Option` fields; now fails to parse as a `$value` enum with missing field. |
| `trajectory_shape.xosc` | lossy, validate-input | `input-schema-invalid` | Upstream typo leaves loose text inside `<ParameterDeclarations>`, which is element-only. The crate has nowhere to put it, drops it, and emits a **valid** document — an invalid input quietly improved. Stays live on `report` and on `validate`, both of which it genuinely passes. |

### The anti-rot mechanism is two assertions, not one

Both run for every entry, in every binary:

1. The `assert` premise about the input must hold.
2. **Every gate named in `gates` must still genuinely fail.** An exemption that has stopped being
   needed — the crate was fixed, or upstream changed the file — trips this and the entry has to
   be deleted rather than quietly protecting nothing.

That second assertion is how the `crate-defect` entry is designed to expire: when the crate is
fixed and the gates stop failing, the assertion trips and the entry must be deleted — this is how
the former TrafficAction exemption (`traffic_area_action_test_scenario.xosc`) expired when OSR-12
landed.

`path` is relative to `conformance/corpus/`, so the manifest is machine-independent, and every
`path` must exist in the fetched corpus or the harness errors out.

### Known holes: `[[hole]]` entries

`mutate` asks about a constraint rather than about a file, so it is the one gate an `[[entry]]`
cannot name; writing `"mutate"` in an entry's `gates` is a load error that says as much. A gap in
what the crate refuses is recorded in a `[[hole]]` entry instead, keyed by the mutation kind and
the XSD complex type of the mutated element, because every document containing that element
exhibits the same gap:

```toml
[[hole]]
gate = "mutate"
kind = "double-choice"
type = "OpenScenario"
assert = "crate-accepts-invalid"
reason = "…why the gap is open, and what closing it would cost…"
```

The anti-rot mechanism is the same one, in its one-assertion form: `assert = "crate-accepts-invalid"`
claims the schema rejects this mutation and the crate accepts it, and the gate proves that claim on
every run. When the crate starts refusing it, the entry describes nothing, the gate fails with
`the mutate gate found no such hole`, and the entry has to be deleted. Hence a hole recorded here
is a decision on record rather than a filter.

The two entries present both describe the document root. `OpenScenario` (XSD:1532) is a sequence
of `<FileHeader>` followed by an `xsd:group` reference whose choice is over three further group
references, one of which inlines seven sibling elements into the root. There is no
`<ScenarioDefinition>` element in the XML, so the `$value` idiom cannot express that choice —
`$value` takes one element name per instance — and the nine root children are parallel `Option`
fields. Thus a document that names no branch, or two at once, parses. The alternative is an
untagged enum, which collapses every parse diagnostic in the document to "data did not match any
variant"; that trade is a design decision and is recorded as an open one.

### Output and exit codes

Each excluded file prints an `XFAIL` line naming the gate result and the `assert` value, and the
summary line carries an `N excluded` term alongside the totals:

```
XFAIL …/OSC_1_3_test_invalid.xosc  (missing field `$value`)  (expectations.toml: input-schema-invalid)

--- summary ---
209 schema-valid, 0 schema-invalid, 0 skipped, 3 excluded, 212 total
```

The denominator of a green run is therefore not 212. `report` reports 2 excluded; `lossy` and
`validate-input` report 3 each; `validate` reports 2. The three files are `Invalid.xosc`,
`OSC_1_3_test_invalid.xosc` and `trajectory_shape.xosc`, but each gate excludes a different subset.
`report` and `validate` exclude `Invalid.xosc` and `OSC_1_3_test_invalid.xosc`; `lossy` and
`validate-input` exclude all three: `Invalid.xosc`, `OSC_1_3_test_invalid.xosc` and
`trajectory_shape.xosc`.

| Exit code | Meaning |
|---|---|
| `0` | Clean: every non-excluded file passed, and every expectation still holds. |
| `1` | A gate failure, or a broken expectation (an `assert` premise that no longer holds, or a gate that no longer fails for an exempt file). |
| `2` | The **manifest itself** is wrong: unparseable, a `path` that does not exist, a duplicate entry, an empty `gates` list, a `[[hole]]` naming a gate other than `mutate` or a mutation kind that does not exist, or an `[[entry]]` naming the `mutate` gate. |

## What a green run proves, and what it does not

`report`, `lossy`, `validate` and `validate-input` all exit non-zero on failure, so they gate
cleanly on `$?`. But a green `report` run means the round trip is **stable**, not **lossless**:
serde drops unknown XML identically on every pass, so a field the Rust types never modeled
produces a passing comparison anyway. `lossy` is what makes dropped or invented data visible —
including dropped *character content*, which it could not see before OSP-14 — and the two
`validate` binaries are the ones that consult the schema, one about the output and one about the
input.

A green run of those four proves the crate **keeps what it is given**. It cannot prove the crate
refuses what the schema refuses, because every file they read is valid: a type that accepts
anything passes all four. That is not a hypothetical. Thirty-four `xsd:choice` groups modelled as
parallel `Option` fields, thirty-four required repeated elements with no `minOccurs`, and the
`flatten` and untagged sites all parsed invalid documents without complaint while this gate
reported eleven green stages. `mutate` is the stage that asks the other half, and its green run
proves something narrower than it looks: 1829 mutants over 103 of the schema's 287 complex types,
which is every type the corpus reaches at a site the cap selected. A type no mutant touched is
untested, not sound — the same caveat the element-coverage figure above carries, measured on a
different axis.

`lossy` used to compare element and attribute *names* only. A value rewritten in place —
`${pi}` reserialized as `$pi`, same-named siblings swapping position and so swapping priority or
execution order — touched no name `lossy` counted, so it passed as lossless. `lossy` now also
compares, position by position within each element path, the attribute values and text of every
matched pair of elements, which is sound against the reorders `xsd:all` permits (differently-named
siblings share no path) and catches the ones it forbids (same-named siblings do).

The corpus itself bounds every result above. It is **212 `.xosc` files** from three scenario
families, and it covers **175 of the schema's 294 element declarations (59.5%)** — but only
**165 (56.1%)** are reached by a file that a gate actually passes. A green run says nothing
about the remaining **129**, and for several of them there is no unit test either. If you add a
type and `lossy` does not move, that is expected when the corpus does not exercise it; it does
not mean the type is correct. The method behind both figures, and which one to use when, is in
[../docs/xsd_gaps.md](../docs/xsd_gaps.md).

Two further ways a green run can mislead, one of them still open:

- `trajectory_shape.xosc` used to pass all three gates while being `xmllint`-invalid anyway,
  because `lossy` could not see character content and so could not see the crate dropping the
  stray text that made the input invalid. `profile()` now counts `path/to/Element#text` keys and
  the `validate-input` gate checks each input against the XSD, so the file is flagged by both and
  carries an `expectations.toml` entry stating why. Fixed in OSP-14.
- `conformance/build.rs` generates the round-trip tests from whatever is in `corpus/` at build
  time. Cargo tracks `corpus/` by the directory's own mtime and `mv` preserves it, so moving the
  corpus aside and back used to leave the empty generated file behind and let
  `bash scripts/gate.sh` exit 0 having run **zero** generated tests. The build script now records
  how many corpus files it saw and `tests/generated.rs` checks that against the corpus at run
  time, so a stale suite fails and names its recovery, `touch conformance/build.rs`. Fixed in
  OSP-15.

See [../docs/xsd_gaps.md](../docs/xsd_gaps.md) for the audit methods that find these gaps and
the currently known gaps in the `mutate` gate (recorded as `[[hole]]` entries: the root choice's
group-of-seven branches cannot be modeled with `$value` and are parallel `Option` fields instead,
allowing documents that take zero or two branches to parse).
