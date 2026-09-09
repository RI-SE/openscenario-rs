#!/usr/bin/env bash
# fetch-corpus.sh — fetch the third-party .xosc conformance corpus.
#
# The corpus itself is MPL-2.0 licensed content owned by two upstream
# repositories. Because this repo is GPL-3.0-only, the corpus is never
# vendored into git — it is cloned on demand into conformance/corpus/,
# which is gitignored, and each clone carries its own upstream LICENSE
# file alongside the .xosc files it supplies.
#
# Usage:
#   scripts/fetch-corpus.sh            # fetch (idempotent no-op if already present)
#   scripts/fetch-corpus.sh --force    # wipe conformance/corpus/ and re-fetch
set -euo pipefail

# --- Pinned commit SHAs -----------------------------------------------------
# Each repo is pinned to a specific commit rather than a moving branch, so the
# corpus is reproducible. To update: run
#   git ls-remote <url> HEAD
# and replace the SHA below with the new HEAD, then re-run this script with
# --force and confirm the resulting .xosc count still matches expectations.

# https://github.com/asam-oss/OSC-ALKS-scenarios, default branch: master
ALKS_URL="https://github.com/asam-oss/OSC-ALKS-scenarios"
ALKS_SHA="b49dc1dd1750692502c3fc98436f19c6a16ee9f1" # resolved via `git ls-remote "$ALKS_URL" HEAD` on 2026-08-29

# https://github.com/vectorgrp/OSC-NCAP-scenarios, default branch: main
NCAP_URL="https://github.com/vectorgrp/OSC-NCAP-scenarios"
NCAP_SHA="15365d18bd7d1d6aff46c75938eddaf4325ac8f3" # resolved via `git ls-remote "$NCAP_URL" HEAD` on 2026-08-29

# --- Paths -------------------------------------------------------------------
# Resolve everything relative to this script's own location, not $PWD, so it
# works regardless of the caller's current directory.
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/.." >/dev/null 2>&1 && pwd -P)"
CORPUS_DIR="${REPO_ROOT}/conformance/corpus"

FORCE=0
for arg in "$@"; do
  case "$arg" in
    --force)
      FORCE=1
      ;;
    *)
      echo "fetch-corpus.sh: unknown argument: $arg" >&2
      echo "usage: $0 [--force]" >&2
      exit 1
      ;;
  esac
done

if [[ "$FORCE" -eq 1 ]]; then
  echo "==> --force: removing ${CORPUS_DIR}"
  rm -rf "$CORPUS_DIR"
fi

mkdir -p "$CORPUS_DIR"

# git_at <dir> <args...>
#
# Runs git against $dir with an explicit --git-dir/--work-tree rather than
# `git -C $dir`. This matters because the corpus lives *inside* this
# repository's working tree: when $dir is not a valid repo, `git -C $dir`
# silently walks up and operates on openscenario-rs itself — reporting the
# outer repo's HEAD, finding the outer repo's `origin`, and fetching the
# pinned corpus SHA from git@github.com:RI-SE/openscenario-rs.git, which
# fails with the baffling "upload-pack: not our ref <sha>". An explicit
# --git-dir refuses to escape and errors instead.
git_at() {
  local dir="$1"
  shift
  git --git-dir="${dir}/.git" --work-tree="$dir" "$@"
}

# clone_sha_at <dir>
#
# Prints the HEAD SHA if $dir is a usable clone, otherwise fails. A directory
# that exists but holds a half-built or broken .git (an interrupted earlier
# run) fails here rather than reporting a nonsense SHA.
clone_sha_at() {
  local dir="$1"
  [[ -d "${dir}/.git" ]] || return 1
  git_at "$dir" rev-parse HEAD 2>/dev/null
}

# fetch_pinned <name> <url> <sha> <dest>
#
# Clones a single commit of a repo into $dest, pinned to $sha. Idempotent: if
# $dest is already a clone whose HEAD matches $sha, this is a fast no-op.
# Relies on the "fetch an arbitrary SHA" idiom below, which requires the
# server to allow fetching by commit hash rather than just by ref
# (uploadpack.allowReachableSHA1InWant). This has been verified to work
# against GitHub for both repos used here.
#
# The clone is built in a temporary directory and moved into place only after
# its SHA is verified, so an interrupted or failed run can never leave a
# broken clone at $dest for the next run to trip over.
fetch_pinned() {
  local name="$1" url="$2" sha="$3" dest="$4"

  local current_sha
  if current_sha="$(clone_sha_at "$dest")" && [[ "$current_sha" == "$sha" ]]; then
    echo "==> ${name}: already at ${sha}, skipping"
    return 0
  fi

  if [[ -e "$dest" ]]; then
    if [[ -n "${current_sha:-}" ]]; then
      echo "==> ${name}: present but at ${current_sha}, expected ${sha}; re-fetching"
    else
      echo "==> ${name}: present but not a usable clone; re-fetching"
    fi
  fi

  local tmp="${CORPUS_DIR}/.tmp-${name}.$$"
  rm -rf "$tmp"
  # shellcheck disable=SC2064  # $tmp is intentionally expanded now, not at trap time
  trap "rm -rf '${tmp}'" RETURN

  echo "==> ${name}: fetching ${sha} from ${url}"
  git init -q "$tmp"
  git_at "$tmp" remote add origin "$url"

  # Retry the fetch: GitHub intermittently answers a fetch-by-SHA with
  # "not our ref" even for a current branch tip, and a corpus fetch is not
  # worth failing a whole push over a blip.
  local attempt
  for attempt in 1 2 3; do
    if git_at "$tmp" fetch -q --depth 1 origin "$sha"; then
      break
    fi
    if [[ "$attempt" -eq 3 ]]; then
      echo "==> ${name}: ERROR: could not fetch ${sha} from ${url} after 3 attempts" >&2
      return 1
    fi
    echo "==> ${name}: fetch attempt ${attempt} failed; retrying in $((attempt * 3))s" >&2
    sleep "$((attempt * 3))"
  done

  git_at "$tmp" checkout -q FETCH_HEAD

  local got_sha
  got_sha="$(git_at "$tmp" rev-parse HEAD)"
  if [[ "$got_sha" != "$sha" ]]; then
    echo "==> ${name}: ERROR: fetched HEAD ${got_sha} does not match pinned SHA ${sha}" >&2
    return 1
  fi

  # Only now is the old clone replaced.
  rm -rf "$dest"
  mv "$tmp" "$dest"
}

fetch_pinned "OSC-ALKS-scenarios" "$ALKS_URL" "$ALKS_SHA" "${CORPUS_DIR}/OSC-ALKS-scenarios"
fetch_pinned "OSC-NCAP-scenarios" "$NCAP_URL" "$NCAP_SHA" "${CORPUS_DIR}/OSC-NCAP-scenarios"

# --- Summary -------------------------------------------------------------
xosc_count="$(find "$CORPUS_DIR" -name '*.xosc' | wc -l | tr -d ' ')"
echo "==> done: ${xosc_count} .xosc files under ${CORPUS_DIR} (expected 172)"
echo "==> both corpora are MPL-2.0 licensed; each clone carries its own upstream LICENSE file"
