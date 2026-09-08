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

# fetch_pinned <name> <url> <sha> <dest>
#
# Clones a single commit of a repo into $dest, pinned to $sha. Idempotent: if
# $dest already exists and its HEAD already matches $sha, this is a fast
# no-op. Relies on the "fetch an arbitrary SHA" idiom below, which requires
# the server to allow fetching by commit hash rather than just by ref
# (uploadpack.allowReachableSHA1InWant). This has been verified to work
# against GitHub for both repos used here.
fetch_pinned() {
  local name="$1" url="$2" sha="$3" dest="$4"

  if [[ -d "$dest/.git" ]]; then
    local current_sha
    current_sha="$(git -C "$dest" rev-parse HEAD 2>/dev/null || true)"
    if [[ "$current_sha" == "$sha" ]]; then
      echo "==> ${name}: already at ${sha}, skipping"
      return 0
    fi
    echo "==> ${name}: present but at ${current_sha:-<unknown>}, expected ${sha}; re-fetching"
    rm -rf "$dest"
  fi

  echo "==> ${name}: fetching ${sha} from ${url}"
  git init -q "$dest"
  git -C "$dest" remote add origin "$url" 2>/dev/null || true
  git -C "$dest" fetch -q --depth 1 origin "$sha"
  git -C "$dest" checkout -q FETCH_HEAD

  local got_sha
  got_sha="$(git -C "$dest" rev-parse HEAD)"
  if [[ "$got_sha" != "$sha" ]]; then
    echo "==> ${name}: ERROR: fetched HEAD ${got_sha} does not match pinned SHA ${sha}" >&2
    exit 1
  fi
}

fetch_pinned "OSC-ALKS-scenarios" "$ALKS_URL" "$ALKS_SHA" "${CORPUS_DIR}/OSC-ALKS-scenarios"
fetch_pinned "OSC-NCAP-scenarios" "$NCAP_URL" "$NCAP_SHA" "${CORPUS_DIR}/OSC-NCAP-scenarios"

# --- Summary -------------------------------------------------------------
xosc_count="$(find "$CORPUS_DIR" -name '*.xosc' | wc -l | tr -d ' ')"
echo "==> done: ${xosc_count} .xosc files under ${CORPUS_DIR} (expected 172)"
echo "==> both corpora are MPL-2.0 licensed; each clone carries its own upstream LICENSE file"
