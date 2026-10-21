#!/usr/bin/env bash
# Assemble the CRUSSTY dist root for the c-LoD bench (mirrors CI e2e job).
# Env overrides: CLOD_DIST, CLOD_REPO, CLOD_CRUSSTY, CLOD_PAPER_JAR, CLOD_JAVA
set -euo pipefail
D="${CLOD_DIST:-$HOME/mc-test/dist}"
REPO="$(cd "$(dirname "$0")/../.." && pwd)"
CRUSSTY="${CLOD_CRUSSTY:-$(dirname "$REPO")/analysis/crussty}"
PAPER="${CLOD_PAPER_JAR:-$HOME/mc-test/dl/paper-26.3.jar}"
JAVA="${CLOD_JAVA:-$HOME/mc-test/jdk25/bin/java}"

rm -rf "$D"
mkdir -p "$D/versions" "$D/modules/lod"
cp "$PAPER" "$D/versions/paper-26.3.jar"
cp "$REPO/target/release/liblod.so" "$D/modules/lod/"
cp "$REPO/module.json" "$D/modules/lod/"

export PATH="$HOME/.cargo/bin:$PATH"
if [ ! -f "$CRUSSTY/runtime/target/release/libcrussty_runtime.so" ]; then
  (cd "$CRUSSTY" && cargo build --release --manifest-path runtime/Cargo.toml)
fi
cp "$CRUSSTY/runtime/target/release/libcrussty_runtime.so" "$D/"

if [ ! -f "$D/launcher.jar" ]; then
  rm -rf /tmp/lout && mkdir -p /tmp/lout "$D"
  JBIN="$(dirname "$JAVA")"
  "$JBIN/javac" -d /tmp/lout "$CRUSSTY/launcher/src/main/java/dev/dist/launcher/Main.java"
  "$JBIN/jar" cfe "$D/launcher.jar" dev.dist.launcher.Main -C /tmp/lout .
fi

printf 'eula=true\n' > "$D/eula.txt"
{
  echo 'online-mode=false'
  echo 'view-distance=4'
  echo 'simulation-distance=4'
  echo 'server-port=25565'
  echo 'spawn-protection=0'
  echo 'white-list=false'
  echo 'level-seed=c-LoDCI'
  echo 'motd=c-LoD bench'
} > "$D/server.properties"

echo "dist root ready at $D"
