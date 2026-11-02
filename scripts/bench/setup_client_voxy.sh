#!/usr/bin/env bash
# Add the Fabric layer (Fabric Loader 0.20.1 + Fabric API + Sodium 0.9.2 +
# Voxy 0.2.20-beta) onto the vanilla 26.3 client runtime built by
# setup_client.py. Idempotent: every step skips finished work, so it can sit
# between the client-runtime cache restore and the voxy-reference leg.
#
# Env:
#   VOXY_RD      section render distance, 32-block sections (default 3 — the
#                4 GB sandbox-safe value; CI sets 8)
#   VOXY_THREADS voxy mesh service threads (default 1; CI sets 2)
#   CLOD_JAVA    java binary for the installer (default ~/tools/jdk25/bin/java)
set -euo pipefail
C="$HOME/mc-test/client"
JAVA="${CLOD_JAVA:-$HOME/tools/jdk25/bin/java}"
BENCH="$(cd "$(dirname "$0")" && pwd)"

mkdir -p "$C/mods"

# 1. The installer needs the vanilla version JSON next to the jar
#    (setup_client.py copies only 26.3.jar).
if [ ! -f "$C/versions/26.3/26.3.json" ]; then
  cp "$BENCH/26.3.json" "$C/versions/26.3/26.3.json"
fi

# 2. Fabric loader profile (KnotClient; Knot runtime-remaps the game jar on
#    first boot and caches it under .fabric/).
if [ ! -f "$C/versions/fabric-loader-0.20.1-26.3/fabric-loader-0.20.1-26.3.json" ]; then
  curl -sL --max-time 60 -o /tmp/fabric-installer.jar \
    "https://maven.fabricmc.net/net/fabricmc/fabric-installer/1.1.0/fabric-installer-1.1.0.jar"
  "$JAVA" -jar /tmp/fabric-installer.jar client -dir "$C" -mcversion 26.3 -loader 0.20.1 -noprofile
fi

fetch_mod() { # url, dest — skip when already present and non-empty
  if [ ! -s "$2" ]; then
    curl -sL --max-time 120 -o "$2" "$1"
  fi
  test -s "$2"
}

# 3. Mods — pinned versions, all verified present for MC 26.3:
#    voxy requires sodium 0.9.2/0.9.3 (fabric-loader rejects the set otherwise).
fetch_mod "https://cdn.modrinth.com/data/P7dR8mSH/versions/EQKgpeKu/fabric-api-0.165.0%2B26.3.jar" \
          "$C/mods/fabric-api-0.165.0+26.3.jar"
fetch_mod "https://cdn.modrinth.com/data/AANobbMI/versions/bAZQdGpg/sodium-fabric-0.9.2%2Bmc26.3.jar" \
          "$C/mods/sodium-fabric-0.9.2+mc26.3.jar"
fetch_mod "https://cdn.modrinth.com/data/fxxUqruK/versions/iIrM8ZKQ/voxy-0.2.20-beta.jar" \
          "$C/mods/voxy-0.2.20-beta.jar"

# 4. Voxy config — pre-write BEFORE the first client boot so the render
#    distance and service threads are deterministic (the client otherwise
#    generates this file with defaults).
RD="${VOXY_RD:-3}"
THREADS="${VOXY_THREADS:-1}"
mkdir -p "$C/config"
cat > "$C/config/voxy-config.json" <<EOF
{
  "enabled": true,
  "enable_rendering": true,
  "ingest_enabled": true,
  "section_render_distance": ${RD}.0,
  "service_threads": ${THREADS},
  "sub_division_size": 64.0,
  "dont_use_sodium_builder_threads": false
}
EOF

echo "fabric layer ready: loader 0.20.1, api 0.165.0, sodium 0.9.2, voxy 0.2.20-beta (rd=${RD}, threads=${THREADS})"
