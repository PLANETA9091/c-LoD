#!/usr/bin/env bash
# Boot the stack with the BIG-ATLAS product profile (backlog: "atlas 2048 on
# real machines" — the step towards the product default atlas 4096/cell 4).
# Variant of boot_server.sh; scan deadline 420s so boot+finalize fit one call.
set -euo pipefail
D="$HOME/mc-test/dist"
JAVA="${CLOD_JAVA:-$HOME/tools/jdk25/bin/java}"
export CLOD_OPTS="scan_mode=sync;scan_deadline_secs=420;tick_ms=100;atlas_size=2048;cell_size=4;stream_chunks=32;http_port=8199;center_x=-930;center_z=796"
# Heap: atlas 2048 (base 16MB + color/side strips 32MB each + tile-id) — 700M
# was fine for 1024; keep 900M headroom for the 4x texel scan working set.
export DIST_JAVA_OPTS="-Xms256M -Xmx${CLOD_SERVER_XMX:-900M} -XX:MaxMetaspaceSize=256M -XX:MaxDirectMemorySize=128M -XX:+UseG1GC"

pkill -f launcher.jar 2>/dev/null || true
pkill -f paper-26.3 2>/dev/null || true
sleep 1

rm -f /tmp/server.in /tmp/server.log
mkfifo /tmp/server.in
cd "$D"
tail -f /tmp/server.in 2>/tmp/tail.err | "$JAVA" -Xmx256M -jar launcher.jar > /tmp/server.log 2>&1 &
SRV=$!
exec 9>/tmp/server.in
echo "launcher pid $SRV"

for i in $(seq 1 270); do
  if grep -q "pack finalized (ONE-TIME, static)" /tmp/server.log 2>/dev/null; then break; fi
  if ! kill -0 $SRV 2>/dev/null; then echo "SERVER DIED"; tail -40 /tmp/server.log; exit 1; fi
  sleep 2
done
grep -q "pack finalized (ONE-TIME, static)" /tmp/server.log || { echo "TIMEOUT waiting for finalize"; tail -40 /tmp/server.log; exit 1; }
sleep 2
echo "=== gates ==="
python3 /home/z/my-project/c-LoD/scripts/e2e/gates.py /tmp/server.log
echo "=== server is up (stdin: /tmp/server.in, log: /tmp/server.log) ==="
grep -E "atlas|scan" /tmp/server.log | tail -5
