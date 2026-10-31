#!/usr/bin/env bash
# v5.9 SEEDGEN preflight: the REAL java/ClodSeedgen helper run against a
# mock net.minecraft hierarchy — no Paper needed. Verifies chunk() vs
# chunkBatch() bit-parity, batch structure/status semantics, tabVer
# monotonicity, close discipline, and that the affine stride fast path and
# the non-affine fallback produce IDENTICAL cells (two JVMs, hashes compared).
#
# Usage: bash scripts/java/preflight/run.sh [path-to-javac]
set -euo pipefail
cd "$(dirname "$0")"
ROOT="$(cd ../../.. && pwd)"
JAVAC="${1:-${JAVAC:-javac}}"
OUT=/tmp/clod-preflight
rm -rf "$OUT"
mkdir -p "$OUT/classes" "$OUT/mock"

# 1) the real helper (same flags as scripts/java/build.sh)
"$JAVAC" --release 17 -Xlint:-rawtypes -d "$OUT/classes" "$ROOT/java/ClodSeedgen.java"
# 2) mocks + harness (plain current-release compile is fine for tests;
#    -cp carries the freshly compiled helper)
"$JAVAC" -cp "$OUT/classes" -d "$OUT/classes" $(find mock-src -name '*.java') \
    PreflightTerrain.java Preflight.java

run() { # $1 = layout property
    java -Dclod.mock.layout="$1" -cp "$OUT/classes" Preflight > "$OUT/$1.txt" 2>&1 || {
        echo "PREFLIGHT FAILED ($1):"; cat "$OUT/$1.txt"; exit 1;
    }
    grep -E "LAYOUT|CELLS_SHA|MAT |BIO |PASS" "$OUT/$1.txt"
}

run affine
run weird

SHA_A=$(grep CELLS_SHA "$OUT/affine.txt" | cut -d' ' -f2)
SHA_W=$(grep CELLS_SHA "$OUT/weird.txt" | cut -d' ' -f2)
if [ "$SHA_A" != "$SHA_W" ]; then
    echo "FAIL: affine vs weird layout hashes differ ($SHA_A != $SHA_W)"
    diff "$OUT/affine.txt" "$OUT/weird.txt" | head -20 || true
    exit 1
fi
MAT_A=$(grep '^MAT ' "$OUT/affine.txt"); MAT_W=$(grep '^MAT ' "$OUT/weird.txt")
BIO_A=$(grep '^BIO ' "$OUT/affine.txt"); BIO_W=$(grep '^BIO ' "$OUT/weird.txt")
[ "$MAT_A" = "$MAT_W" ] && [ "$BIO_A" = "$BIO_W" ] || {
    echo "FAIL: intern tables diverged between layouts"; exit 1;
}
# Both paths must have actually run: affine columns take the stride fast
# path, the Fisher-Yates layout falls back per column.
FAST_A=$(grep COLUMNS "$OUT/affine.txt"); FAST_W=$(grep COLUMNS "$OUT/weird.txt")
echo "$FAST_A" | grep -qv "fast=0 " || { echo "FAIL: fast path never taken (affine)"; exit 1; }
echo "$FAST_W" | grep -q "fast=0 " || { echo "FAIL: fallback never taken (weird)"; exit 1; }
echo "$FAST_A / $FAST_W"
echo "---"
echo "PREFLIGHT OK: affine == weird (cells $SHA_A), tables identical, both stride paths exercised"
