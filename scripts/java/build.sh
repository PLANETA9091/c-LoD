#!/usr/bin/env bash
# Rebuild the embedded Java helper bytecode (src/seedgen.rs and src/edits.rs
# include_bytes!). The sources are JDK-ONLY — no Minecraft/Paper jars on the
# classpath: ClodSeedgen reaches every Minecraft type through MethodHandles
# resolved at runtime, ClodEdits references the stable org.bukkit.plugin/
# org.bukkit.event API shapes carried by the preflight mock tree (same
# names — the real classes take over at runtime). The .class files are
# committed alongside the sources; CI recompiles and diffs them, so re-run
# this whenever either source changes and commit the result.
set -euo pipefail
cd "$(dirname "$0")/../../java"
JAVAC="${JAVAC:-javac}"
"$JAVAC" --release 17 -Xlint:-rawtypes ClodSeedgen.java
echo "ClodSeedgen.class rebuilt ($(wc -c < ClodSeedgen.class) bytes)"
echo "ClodSeedgen\$GenModel.class emitted ($(wc -c < 'ClodSeedgen$GenModel.class') bytes)"
"$JAVAC" --release 17 -Xlint:-rawtypes \
    -sourcepath ../scripts/java/preflight/mock-src -implicit:none ClodEdits.java
echo "ClodEdits.class rebuilt ($(wc -c < ClodEdits.class) bytes)"
