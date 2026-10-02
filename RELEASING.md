# Releasing

This is the whole release procedure. Steps 1–5 are automated by
[`.github/workflows/release.yml`](.github/workflows/release.yml) once the tag is pushed; step 6
is not and cannot be — it needs a crates.io credential, which the maintainer holds locally and
which this repository's CI never sees.

1. `git switch -c release/0.5.0`
2. Move the `CHANGELOG.md` `[Unreleased]` body under a new `## [0.5.0] - YYYY-MM-DD` heading;
   add the link-reference definition at the bottom of the file.
3. Bump `version` in `Cargo.toml`; run `cargo update -p openscenario-rs` so `Cargo.lock` follows.
4. Open a PR; squash merge to `main`.
5. `git switch main && git pull && git tag v0.5.0 && git push origin v0.5.0`
   — this triggers `.github/workflows/release.yml`, which checks the tag against
   `Cargo.toml`'s version, checks `CHANGELOG.md` has a `## [0.5.0]` section, runs
   `scripts/gate.sh` (the same gate the `pre-push` hook runs — a release must not ship something
   the gate would reject), slices that section's body into release notes, and creates a
   GitHub Release from them.
6. `cargo publish` — **MANUAL**, from a clean checkout of the tag. This is the only step no gate
   covers: the workflow deliberately has no `CARGO_REGISTRY_TOKEN` and no `cargo publish` step,
   by design (see [CONTRIBUTING.md](CONTRIBUTING.md)) — publishing to crates.io stays a manual
   local action, never automated CI.

The tag in step 5 must be pushed only after the version-bump commit from step 3/4 is already on
`main` — the workflow validates the tagged commit's own `Cargo.toml`, and a tag pushed against a
commit whose version doesn't match the tag fails the version-match check loudly rather than
publishing a mismatched release.

A tag containing a `-` (e.g. `v0.5.0-rc1`) is released as a GitHub prerelease automatically; a
plain `vX.Y.Z` tag is released as a normal release.

## Retroactive tags

`v0.4.0`, `v0.4.1` and `v0.4.2` were created after the fact, to give the six crates.io releases
that predate this workflow (0.1.0 through 0.4.2, all published by hand from commits whose entire
message was `bump`) a point in git history to refer back to. Pushing those tags deliberately does
**not** create a GitHub Release for them — there is no `CHANGELOG.md` section written at the time
those versions actually shipped, so the version-match and changelog-section checks above would
either fail or produce a release with fabricated notes. Only tags from 0.5.0 onward, pushed
through the procedure above, get a GitHub Release.
