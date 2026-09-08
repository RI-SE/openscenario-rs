# openscenario-roundtrip-harness

This crate is the `conformance` workspace member: a corpus of real `.xosc` files plus four
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

The corpus is third-party content from two upstream repositories, each pinned to a commit SHA
in `scripts/fetch-corpus.sh` so the fetch is reproducible:

- [asam-oss/OSC-ALKS-scenarios](https://github.com/asam-oss/OSC-ALKS-scenarios)
- [vectorgrp/OSC-NCAP-scenarios](https://github.com/vectorgrp/OSC-NCAP-scenarios)

Both are **MPL-2.0** licensed. `openscenario-rs` is **GPL-3.0-only**, and MPL-2.0 is a
file-level copyleft license that does not propagate onto a work merely built alongside it —
but committing the corpus into this repository would still mean redistributing someone else's
licensed files under terms this repo does not control. Fetching on demand keeps the corpus's
license and provenance with the files themselves: each clone carries its own upstream `LICENSE`
next to the `.xosc` files it supplies, and nothing MPL-2.0 ever lands in this repo's git history.

Without the corpus, `report`, `lossy` and `validate` print a hint and exit 1; `cargo test -p
openscenario-roundtrip-harness` generates zero corpus tests and still runs the builder-fixture
gate; `builder` (without `--coverage`) needs no corpus at all.

## The four binaries

```bash
cargo run -p openscenario-roundtrip-harness --bin report
cargo run -p openscenario-roundtrip-harness --bin lossy
cargo run -p openscenario-roundtrip-harness --bin validate
cargo run -p openscenario-roundtrip-harness --bin builder
cargo run -p openscenario-roundtrip-harness --bin builder -- --coverage
```

| Binary | What it checks |
|---|---|
| `report` | Every corpus file parses, serializes, reparses and reserializes to the same XML twice in a row (a round-trip fixed point). |
| `lossy` | The *first* serialization against the *original* file, so it can see data dropped or invented on the initial parse — the one thing `report` cannot see. |
| `validate` | The serialized output against `Schema/OpenSCENARIO.xsd`. |
| `builder` | Every builder fixture through the same three questions (round trip, fidelity, schema), using code as the corpus instead of files. With `--coverage`, reports how much of one real scenario the builder can reconstruct — a figure, not a gate. |

## What a green run proves, and what it does not

`report`, `lossy` and `validate` all exit non-zero on failure, so they gate cleanly on `$?`.
But a green `report` run means the round trip is **stable**, not **lossless**: serde drops
unknown XML identically on every pass, so a field the Rust types never modeled produces a
passing comparison anyway. `lossy` is what makes dropped or invented data visible, and
`validate` is the only one of the three that consults the schema at all.

The corpus itself bounds every result above. It is two narrow scenario families and covers
about **53%** of the schema's element declarations. A green run says nothing about the other
47% — no unit test either, for several of them. If you add a type and `lossy` does not move,
that is expected when the corpus does not exercise it; it does not mean the type is correct.

See [../docs/xsd_gaps.md](../docs/xsd_gaps.md) for the audit methods that find these gaps and
the currently open one (parameterized enumeration attributes).
