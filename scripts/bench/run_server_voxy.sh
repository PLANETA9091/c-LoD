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
  # v6.8.6: run 41472301678's leg died in <=1 s — the prime suspect is this
  # bare `curl -s | python3` pipeline (no --fail, no retry, no error body:
  # any non-JSON API answer kills the leg under set -e BEFORE voxy_server.log
  # even exists, and the feed sees only "voxy leg start"). Now every step
  # names itself and carries the API's own words into voxy-leg.log.
  API_JSON=$(curl -sS --fail --retry 3 --retry-delay 2 --max-time 30 \
    -H 'User-Agent: c-LoD-ci/6.8.6 (github.com/PLANETA9091/c-LoD)' \
    https://fill.papermc.io/v3/projects/paper/versions/26.3/builds/latest) || {
    echo "FATAL: papermc API request failed (curl rc=$?) — response body (if any) printed above"; exit 1; }
  URL=$(echo "$API_JSON" | python3 -c "import json,sys;d=json.load(sys.stdin);print(d['downloads']['server:default']['url'])") || {
    echo "FATAL: papermc API response not parseable / missing server:default (first 300 bytes):"
    echo "$API_JSON" | head -c 300; echo; exit 1; }
  echo "api ok, downloading $(echo "$API_JSON" | python3 -c 'import json,sys;print(json.load(sys.stdin).get("name","paper build ?"))' 2>/dev/null || echo 'paper build ?')"
  curl -sS --fail --retry 3 --retry-delay 2 --max-time 300 -o "$D/paper-26.3.jar" "$URL" || {
    echo "FATAL: Paper jar download failed: $URL"; exit 1; }
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
