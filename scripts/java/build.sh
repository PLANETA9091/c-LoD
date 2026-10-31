#!/usr/bin/env bash
# Rebuild the v5.8 seedgen helper bytecode embedded in the module
# (src/seedgen.rs include_bytes!). The source is JDK-ONLY — no Minecraft
# jars on the classpath; every Minecraft type is reached through
# MethodHandles resolved at runtime. The .class is committed alongside the
# source; CI recompiles and diffs it, so re-run this whenever
# java/ClodSeedgen.java changes and commit the result.
set -euo pipefail
cd "$(dirname "$0")/../../java"
JAVAC="${JAVAC:-javac}"
"$JAVAC" --release 17 -Xlint:-rawtypes ClodSeedgen.java
echo "ClodSeedgen.class rebuilt ($(wc -c < ClodSeedgen.class) bytes)"
