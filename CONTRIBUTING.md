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

### The pre-push gate

All eleven gate stages – markdown links, `cargo fmt --check`, clippy, build, test, and the six
conformance stages – live in one place, [`scripts/gate.sh`](scripts/gate.sh). Both the local
`pre-push` hook and the hosted `gate` job in
[`.github/workflows/ci.yml`](.github/workflows/ci.yml) run that same script, so there is exactly
one copy of the stage list and it cannot drift between "what the hook checks" and "what CI
checks."

Enable the hook once per clone:

```bash
git config core.hooksPath scripts/hooks
```

`core.hooksPath` replaces `.git/hooks` wholesale, which costs nothing here – that directory holds
only Git's stock `*.sample` files. The relative path is resolved against the top of the working
tree (`git-config(1)`: "A relative path is taken as relative to the directory where the hooks are
run"), so this one setting works from every `git worktree` of the repo.

`scripts/hooks/pre-push` itself only parses what Git hands it on stdin and decides whether there
is anything new to verify; if there is, it `exec`s `scripts/gate.sh` with no arguments, so it
always runs all eleven stages. It checks the working tree as it currently sits, not the commits
being pushed – a dirty tree, or a commit amended after the last green run, is not what was
verified. That caveat is no reason to distrust the hook; it is the reason the hosted `gate` job
exists too, checking the actual commits on every pull request and every push to `main`.

You can also run the gate directly, without pushing:

```bash
bash scripts/gate.sh
```

It takes an optional stage filter as its one argument – `lint` (markdown links, fmt, clippy),
`build` (build, test), or `conformance` (fetch corpus, the harness tests, and the four corpus
gate binaries) – for a faster loop while iterating; an unrecognized filter is a usage error, not a silent no-op. It also
checks system prerequisites first, for every filter – the MSRV from `Cargo.toml`'s
`rust-version`, and `pkg-config`'s view of libxml2, which the `validation` feature links against
– so a missing dependency reports itself instead of surfacing as a linker error halfway through a
build.

## How changes land

Only `main` exists as a long-lived branch, and nothing is committed to it directly. Every change
is a branch and a pull request:

- Branch names follow the same conventional-commit prefixes this file already mandates for
  commits: `feat/`, `fix/`, `refactor/`, `docs/`, `chore/`, plus `release/x.y.z` for a release
  branch (see [RELEASING.md](RELEASING.md)).
- Pull requests are **squash merged only**, and the branch is deleted on merge. `main` keeps a
  linear history – no merge commits.
- The `gate` check (the same `scripts/gate.sh` described above, run by
  [`.github/workflows/ci.yml`](.github/workflows/ci.yml)) must be green on the PR. It is a
  required check; there is no override short of an admin bypass.
- Anything user-visible goes into `CHANGELOG.md`'s `[Unreleased]` section as part of the same PR
  – not just breaking changes, any addition, fix or behavior change a downstream user could
  notice. The crate has downstream users, and a change that only shows up in `git log` is
  effectively undocumented.
- Releases – tagging, the `release.yml` workflow, and the one manual step it deliberately does
  not automate (publishing to crates.io stays a manual, local `cargo publish`, never run in CI) –
  are the whole subject of [RELEASING.md](RELEASING.md).

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

Four gates run against the corpus in the `conformance` workspace member
(`openscenario-roundtrip-harness`), which depends on this crate by path with the `validation`
feature. All four exit non-zero on failure.

The corpus itself is third-party content from three upstream repositories (two MPL-2.0, one
EPL-2.0), not vendored into this GPL-3.0-only repo. Fetch it once with:

```bash
bash scripts/fetch-corpus.sh
```

| Command | Compares | Catches |
|---|---|---|
| `cargo run -p openscenario-roundtrip-harness --bin report` | `xml1` vs `xml2` | parse failures, instability across serialization |
| `cargo run -p openscenario-roundtrip-harness --bin lossy` | the original file vs `xml1` | data dropped or invented on the first parse |
| `cargo run -p openscenario-roundtrip-harness --bin validate` | `xml1` vs the XSD | schema-invalid output |
| `cargo run -p openscenario-roundtrip-harness --bin validate-input` | the **original file** vs the XSD | a schema-invalid input, which the crate may otherwise "improve" into a valid document by dropping the invalid part |

Run them from the repo root. `lossy` is the one to check after adding a type, because it is the
only gate that sees first-parse data loss. `validate-input` is the odd one out: it never parses
the file with this crate, so its verdict is a fact about the corpus rather than about the code,
and it is what stops a green `validate` run from meaning "the output is valid" when the reason is
that content went missing.

[`conformance/expectations.toml`](conformance/expectations.toml) records the handful of corpus
files whose expected outcome is not "passes everything", with the reason and the gates each is
exempt from. It is not a skip list: every entry asserts something the harness checks, so an
exemption that stops being needed fails loudly. The categories and the per-gate table are in
[conformance/README.md](conformance/README.md). All four, plus `cargo test -p
openscenario-roundtrip-harness`, also run from [the pre-push hook](#the-pre-push-gate), so a push
that reaches the remote has already cleared them.

Be careful how you read a green `report` run. serde drops unknown XML on every pass
identically, so the round-trip comparison still succeeds over data the types never modeled.
The corpus reported 172/172 for a long time while discarding route positions, vehicle light
states and global actions. A green `report` means *stable*, not *lossless*.

The corpus also covers only part of the schema: 175 of 294 element declarations are present in
it, and only 165 are reached by a file that a gate actually passes. If you add a type and `lossy`
does not move, that is expected, but it also means the corpus cannot validate your work and unit
tests are the only check that ran.

For the audit methods that find gaps in the first place, and the shell caveats that bite while
running them, see [docs/xsd_gaps.md](docs/xsd_gaps.md).

## Local schema validation

Without the corpus, the bundled binary validates individual files:

```bash
cargo run --bin xosc-validate --features validation -- tests/data/alks_scenario.xosc
```

It takes `--recursive` for a directory and `--format json|junit` for machine-readable output.

## Commits and pull requests

Commit messages follow the conventional-commit prefixes already in the history (`feat`,
`fix`, `refactor`, `docs`), with a scope naming the area: `fix(schema):`, `feat(actions):`,
`refactor(types):`. Write the subject as what the change does, not what it touches.

The `pre-push` hook runs the full gate on every push. Note that it checks the working tree as it
currently stands, not the commits being pushed – so a dirty tree, or a commit amended after the
last green run, is not what was verified. `git push --no-verify` skips the hook; if you use it,
say so in the pull request.

A pull request should say which schema declaration it follows, name the gates it ran, and note
any conformance gap it knowingly leaves open. Breaking changes belong in
[CHANGELOG.md](CHANGELOG.md) under `Unreleased`; the crate has downstream users and removed
types are otherwise invisible until something fails to compile.

## Documentation

Documentation drifts faster than code, and this repository has been through one large
correction already. When you change public API, update the guide that covers it in the same
change. Every type name, function signature and cargo command in `docs/` should be checkable
against the source – if you cannot grep for it, do not write it.

`docs/` is tracked wholesale, including every `.md` file in it. That is easy to miss:
`.gitignore`'s `/*.md` is root-anchored (its own comment says "Temporary Rust files in root
directory") and does not reach into `docs/`. `docs/xsd_gaps.md` was assumed to be covered by
that rule, was never committed, and survived only as a `cargo package` build artifact until
it was recovered – with a `scripts/hooks/pre-push` stage now checking that every relative
markdown link in the repo still resolves, specifically so this cannot happen silently again.
Do not add a `docs/*.md` ignore rule to "clean up" an untracked file in that directory –
check first whether it belongs in git.
