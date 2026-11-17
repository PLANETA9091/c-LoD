#!/usr/bin/env bash
# v6.57 HORIZON SETUP: clone + build the Rust client and import the vanilla
# content the bench already downloaded, so horizon-client renders the same
# 26.3 assets the vanilla leg sees. Pinned by HORIZON_SHA (workflow env) for
# reproducibility; "main" tracks upstream for the first probe runs.
# The checkout NEVER downloads the game (their design): horizon-prepare
# imports OUR official client.jar (setup_client.py fetched it sha1-verified
# from the Mojang manifest) and OUR paper.jar, verifying against their
# research-lock.json pins.
set -euo pipefail
REPO="${HORIZON_REPO:-https://github.com/horizon-rust/horizon}"
SHA="${HORIZON_SHA:-main}"
SRC="${HORIZON_SRC:-$HOME/horizon-src}"
VJ="$HOME/mc-test/client"
PJAR="${MC_TEST_DIST:-$HOME/my-dist}/versions/paper-26.3.jar"

echo "- horizon: cloning $REPO ($SHA)" >&2
if [ ! -d "$SRC/.git" ]; then
  git clone --depth 1 "$REPO" "$SRC"
fi
if [ "$SHA" != "main" ]; then
  git -C "$SRC" fetch --depth 1 origin "$SHA" 2>/dev/null || true
  git -C "$SRC" checkout --quiet FETCH_HEAD
fi
echo "- horizon: at $(git -C "$SRC" rev-parse --short HEAD)"

# The fast dev profile is the vendor's own recommendation for CI builds
# (docs/native-client.md: cargo build --profile fast -p horizon-game).
echo "- horizon: cargo build --profile fast (bins: horizon-client, horizon-ctl)" >&2
cargo build --profile fast -p horizon-game --bins --manifest-path "$SRC/Cargo.toml"
ls -1 "$SRC/target/fast/" | grep -E "^horizon-(client|ctl)$" || {
  echo "FAIL: horizon bins missing after build"; ls "$SRC/target/fast/" 2>/dev/null; exit 1; }

VJAR="$VJ/versions/26.3/26.3.jar"
[ -f "$VJAR" ] || { echo "FAIL: vanilla client.jar missing at $VJAR (setup_client.py must run first)"; exit 1; }
[ -f "$PJAR" ] || { echo "FAIL: paper.jar missing at $PJAR (fetch step must run first)"; exit 1; }

echo "- horizon: prepare (import client.jar + assets + libraries, verify vs research-lock)" >&2
cargo run --profile fast --manifest-path "$SRC/Cargo.toml" -p horizon-prepare -- \
  --prepare \
  --client-jar "$VJAR" \
  --server-jar "$PJAR" \
  --assets-dir "$VJ/assets" \
  --libraries-dir "$VJ/libraries"

# Sounds (~380 MB) come from the same resource store the vanilla leg already
# downloaded; a failure here is non-fatal — the visual bench needs no audio.
echo "- horizon: sounds import (best-effort)" >&2
cargo run --profile fast --manifest-path "$SRC/Cargo.toml" -p horizon-prepare -- sounds \
  || echo "WARN: sounds import failed (non-fatal for the visual bench)"

echo "- horizon: setup done" >&2
