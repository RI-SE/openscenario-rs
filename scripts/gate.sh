#!/usr/bin/env bash
# gate.sh — the project's quality gate: twelve stages, in a fixed order.
#
# This used to live only inside scripts/hooks/pre-push, which meant nothing
# else (CI, the release workflow) could run the same checks without copying
# the stage list into a second file that would drift from this one. This
# script is now the single source of truth; the pre-push hook delegates to it.
#
# Usage:
#   scripts/gate.sh [all|lint|build|conformance]
#
#   (no argument), all  -> all twelve stages (markdown links, fmt, clippy,
#                          build, test, fetch corpus, harness test, report,
#                          lossy, validate, validate-input, mutate)
#   lint                -> markdown links, fmt, clippy               (3 stages)
#   build               -> build, test                               (2 stages)
#   conformance         -> fetch corpus, harness test, report, lossy,
#                          validate, validate-input, mutate          (7 stages)
#
# Any other argument is a usage error: a gate that would otherwise run zero
# stages must never exit 0.
set -euo pipefail

filter="${1:-all}"
case "$filter" in
  all|lint|build|conformance) ;;
  *)
    echo "gate.sh: unknown stage filter '${filter}'" >&2
    echo "usage: $(basename "$0") [all|lint|build|conformance]" >&2
    exit 1
    ;;
esac

# --- Locate the repo ---------------------------------------------------------
# `--show-toplevel` is correct under `git worktree`, unlike deriving paths from
# $GIT_DIR (which points at the shared common dir's per-worktree subdirectory).
REPO_ROOT="$(git rev-parse --show-toplevel)"
cd "$REPO_ROOT"

bold=""; red=""; green=""; reset=""
if [[ -t 2 ]]; then
  bold=$'\033[1m'; red=$'\033[31m'; green=$'\033[32m'; reset=$'\033[0m'
fi

fail() {
  echo "${red}${bold}gate.sh: $1${reset}" >&2
  shift
  for line in "$@"; do echo "  $line" >&2; done
  exit 1
}

# --- Preflight: toolchain ----------------------------------------------------
# The MSRV lives in Cargo.toml's `rust-version`. Bump both together: that field
# and any CI/toolchain pin elsewhere in the repo.
msrv="$(sed -n 's/^rust-version *= *"\([0-9.]*\)".*/\1/p' Cargo.toml | head -1)"
rustc_ver="$(rustc --version 2>/dev/null | awk '{print $2}')" ||
  fail "rustc not found on PATH." "Install Rust: https://rustup.rs"

if [[ -n "$msrv" ]]; then
  # Sort the two versions and check the MSRV comes first.
  lowest="$(printf '%s\n%s\n' "$msrv" "$rustc_ver" | sort -V | head -1)"
  if [[ "$lowest" != "$msrv" && "$rustc_ver" != "$msrv" ]]; then
    fail "rustc $rustc_ver is below the crate MSRV $msrv (Cargo.toml: rust-version)." \
         "Run: rustup update stable"
  fi
fi

for component in fmt clippy; do
  cargo "$component" --version >/dev/null 2>&1 ||
    fail "cargo $component is not available." "Run: rustup component add rust$component"
done

# --- Preflight: system libxml2 ----------------------------------------------
# The `validation` feature links against system libxml2 via the `libxml` crate,
# whose build.rs locates it through pkg-config
# (see libxml's build.rs: `pkg_config::Config::new().probe("libxml-2.0")`).
# Checking here turns an opaque mid-build linker error into a clear message,
# in CI as well as locally.
if ! pkg-config --exists libxml-2.0 2>/dev/null; then
  fail "pkg-config cannot find libxml-2.0, required by the 'validation' feature." \
       "Arch:          sudo pacman -S libxml2 pkgconf" \
       "Debian/Ubuntu: sudo apt-get install -y libxml2-dev pkg-config" \
       "Fedora:        sudo dnf install -y libxml2-devel pkgconf-pkg-config"
fi

# --- Stages ------------------------------------------------------------------
# Each stage is tagged with the group it belongs to (lint / build /
# conformance). TOTAL_STAGES and the [N/M] counter only count stages that
# survive the filter, computed up front so the counter is correct for
# whichever subset is selected.
group_selected() {
  [[ "$filter" == "all" || "$filter" == "$1" ]]
}

declare -a STAGE_GROUPS=(lint lint lint build build conformance conformance conformance conformance conformance conformance conformance)
TOTAL_STAGES=0
for g in "${STAGE_GROUPS[@]}"; do
  group_selected "$g" && TOTAL_STAGES=$((TOTAL_STAGES + 1))
done
stage_num=0

stage() {
  local group="$1" name="$2"; shift 2
  [[ "${1:-}" == "--" ]] && shift
  if ! group_selected "$group"; then
    return 0
  fi
  stage_num=$((stage_num + 1))
  echo
  echo "${bold}[${stage_num}/${TOTAL_STAGES}] ${name}${reset}"
  echo "      \$ ${STAGE_CMD_DISPLAY:-$*}"
  if ! "$@"; then
    fail "stage ${stage_num}/${TOTAL_STAGES} failed: ${name}" "Command: ${STAGE_CMD_DISPLAY:-$*}"
  fi
}

# `.rustfmt.toml` sets options that are nightly-only (space_before_colon,
# trailing_semicolon, brace_style, …); stable rustfmt ignores them and warns
# once per file. The warnings do not affect the exit status, so drop them and
# keep the real diff output.
fmt_check() {
  local out status=0
  out="$(cargo fmt --all --check 2>&1)" || status=$?
  printf '%s\n' "$out" | grep -v 'unstable features are only available in nightly channel' || true
  return "$status"
}

# `docs/xsd_gaps.md` was lost once already: it was never committed, 19
# relative links across the repo pointed at it, and nothing caught the dead
# links before a release shipped with them (see docs/xsd_gaps.md's own
# history). This stage is bash + grep/sed only, no new dependency, and is
# what makes that failure mode loud instead of silent.
#
# Walk every tracked *.md file, pull out `[text](target)` link targets,
# skip absolute URLs/mailto/bare in-page anchors, strip a trailing `#anchor`,
# resolve what is left relative to the *linking file's* directory, and fail
# listing anything that does not exist on disk.
check_md_links() {
  local status=0
  local file link target
  while IFS= read -r -d '' file; do
    while IFS= read -r link; do
      [[ -z "$link" ]] && continue
      case "$link" in
        http://*|https://*|mailto:*|\#*) continue ;;
      esac
      target="${link%%#*}"
      [[ -z "$target" ]] && continue
      if [[ ! -e "$(dirname -- "$file")/$target" ]]; then
        echo "  ${file}: broken link -> ${link}" >&2
        status=1
      fi
    done < <(grep -oE '\]\([^)[:space:]]+([[:space:]]+"[^"]*")?\)' "$file" |
              sed -E 's/^\]\(([^) ]+).*\)$/\1/')
  done < <(git ls-files -z -- '*.md')
  return "$status"
}

echo "${bold}gate.sh: running the '${filter}' gate${reset}"

stage lint "Check markdown links"                  -- check_md_links
STAGE_CMD_DISPLAY="cargo fmt --all --check" \
  stage lint "Check formatting"                    -- fmt_check
unset STAGE_CMD_DISPLAY
stage lint "Clippy"                                -- cargo clippy --workspace --features builder,validation --all-targets
stage build "Build"                                -- cargo build --features builder,validation --all-targets
stage build "Test"                                 -- cargo test --features builder,validation

# The corpus is third-party content, fetched (never vendored) by
# scripts/fetch-corpus.sh, which pins two upstream repos by commit SHA. The
# script is idempotent: when each clone's HEAD already matches its pinned SHA
# it is a fast no-op, so this costs nothing on a warm checkout.
stage conformance "Fetch conformance corpus"              -- bash scripts/fetch-corpus.sh

stage conformance "Test conformance harness"              -- cargo test -p openscenario-roundtrip-harness
stage conformance "Conformance report (round-trip)"       -- cargo run -p openscenario-roundtrip-harness --bin report
stage conformance "Conformance lossy (dropped/invented)"  -- cargo run -p openscenario-roundtrip-harness --bin lossy
stage conformance "Conformance XSD validation (output)"   -- cargo run -p openscenario-roundtrip-harness --bin validate

# The only stage that asks a question about the corpus rather than about this
# crate: is each input file itself XSD-valid? Added by OSP-14, because a
# schema-invalid input whose invalid part the crate does not model is parsed,
# the content is dropped, and the output validates — so `validate` went green
# *because* something was lost.
stage conformance "Conformance XSD validation (input)"    -- cargo run -p openscenario-roundtrip-harness --bin validate-input

# The only stage that asks whether the crate *refuses* what the schema refuses. The other five
# feed it valid documents, so none of them can see a type that accepts anything: the choice groups
# modelled as parallel Options, the required repeated elements with no minOccurs, and the flattened
# children all parsed invalid input without complaint while this gate reported eleven green stages.
# This one breaks each valid corpus file in XSD-guided ways and requires the crate to reject every
# mutant libxml2 rejects.
stage conformance "Conformance refusal (mutation)"        -- cargo run -p openscenario-roundtrip-harness --bin mutate

echo
echo "${green}${bold}gate.sh: all ${TOTAL_STAGES} stages passed in ${SECONDS}s.${reset}"
