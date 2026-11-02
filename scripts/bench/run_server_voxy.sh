#!/usr/bin/env bash
# Boot the PLAIN Paper 26.3 reference server (NO c-LoD module) for the Voxy
# visual-reference leg. The Voxy client must see a clean vanilla server: no
# resource-pack push, no dust particles, no LOD dome — only the real world at
# a PINNED SEED so the same terrain can be regenerated for the c-LoD leg.
# Usage: run_server_voxy.sh   — logs to /tmp/voxy_server.log, stdin fifo
set -euo pipefail
BENCH_DIR="$(cd "$(dirname "$0")" && pwd)"
D="${VOXY_SERVER_DIR:-$HOME/mc-test/voxy-server}"
JAVA="${CLOD_JAVA:-$HOME/tools/jdk25/bin/java}"
# Paper jar resolution: explicit env > bench copy > local dist > download.
PAPER_JAR="${CLOD_PAPER:-}"
if [ -z "$PAPER_JAR" ] && [ -f "$BENCH_DIR/26.3-server.jar" ]; then
  PAPER_JAR="$BENCH_DIR/26.3-server.jar"
fi
if [ -z "$PAPER_JAR" ] && [ -f "$HOME/mc-test/dist/versions/paper-26.3.jar" ]; then
  PAPER_JAR="$HOME/mc-test/dist/versions/paper-26.3.jar"
fi
SEED="${VOXY_SEED:-clodbench2026}"   # same seed as the c-LoD visual CI leg — the reference shots paint the same terrain

pkill -f "paper-26.3" 2>/dev/null || true
sleep 1

mkdir -p "$D"
cd "$D"
if [ -z "$PAPER_JAR" ]; then
  echo "fetching Paper 26.3 (no local jar found)"
  URL=$(curl -s https://fill.papermc.io/v3/projects/paper/versions/26.3/builds/latest \
    | python3 -c "import json,sys;print(json.load(sys.stdin)['downloads']['server:default']['url'])")
  curl -sL -o "$D/paper-26.3.jar" "$URL"
  PAPER_JAR="$D/paper-26.3.jar"
fi
cp -f "$PAPER_JAR" ./paper-26.3.jar
echo "eula=true" > eula.txt
# pinned seed + deterministic world; view distance 8 matches the client
cat > server.properties <<EOF
level-seed=$SEED
online-mode=false
view-distance=8
simulation-distance=4
spawn-protection=0
motd=voxy visual reference
white-list=false
max-players=5
enable-command-block=false
EOF

rm -f /tmp/server.in /tmp/voxy_server.log
mkfifo /tmp/server.in

# Console plumbing that survives the whole session: `tail -f` holds the fifo
# read side so the java stdin NEVER sees EOF (a plain `java < fifo` closes
# the console thread on the first writer exit — later `echo > fifo` commands
# would land in a dead pipe). `exec 3>` holds a writer open.
export MALLOC_ARENA_MAX=2
tail -f /tmp/server.in | nohup "$JAVA" -Xms160M -Xmx${VOXY_SERVER_XMX:-320M} -XX:MaxDirectMemorySize=64M -XX:+UseG1GC \
  -jar paper-26.3.jar nogui > /tmp/voxy_server.log 2>&1 &
SRV=$!
exec 3>/tmp/server.in
SRVPIPE=$!

for i in $(seq 1 240); do
  grep -q "Done (" /tmp/voxy_server.log 2>/dev/null && break
  if ! kill -0 "$SRV" 2>/dev/null; then
    echo "server died during boot:"; tail -20 /tmp/voxy_server.log; exit 1
  fi
  sleep 1
done
grep -q "Done (" /tmp/voxy_server.log || { echo "server never finished boot"; tail -20 /tmp/voxy_server.log; exit 1; }
echo "voxy reference server up (pid $SRV, seed $SEED)"
