#!/usr/bin/env bash
# Boot PLAIN Paper 26.3 (no CRUSSTY agent, no launcher, no c-LoD module) for
# the Voxy A/B: voxy is a client-side mod, the LOD server contributes nothing
# to its baseline — but the CRUSSTY stack costs ~300 MB of RSS that the 4 GB
# box needs for the voxy client (live-OOMs 2026-10-23: client+stack don't fit
# 4 GB together). Same world dir => same seed/terrain; stdin fifo for tp.
set -uo pipefail
D="$HOME/mc-test/dist"
JAVA="${CLOD_JAVA:-$HOME/mc-test/jdk25/bin/java}"

pkill -f launcher.jar 2>/dev/null || true
pkill -f paper-26.3 2>/dev/null || true
pkill -f "libcrussty" 2>/dev/null || true
sleep 1

rm -f /tmp/server.in /tmp/server.log
mkfifo /tmp/server.in
cd "$D"
tail -f /tmp/server.in 2>/tmp/tail.err | "$JAVA" -Xms256M -Xmx400M \
  -XX:MaxMetaspaceSize=192M -XX:+UseG1GC -jar "$D/versions/paper-26.3.jar" --nogui \
  > /tmp/server.log 2>&1 &
SRV=$!
exec 9>/tmp/server.in
echo "paper pid $SRV"

for i in $(seq 1 150); do
  if grep -q "Done (" /tmp/server.log 2>/dev/null; then break; fi
  if ! kill -0 $SRV 2>/dev/null; then echo "SERVER DIED"; tail -30 /tmp/server.log; exit 1; fi
  sleep 2
done
grep -q "Done (" /tmp/server.log || { echo "TIMEOUT waiting for paper"; tail -30 /tmp/server.log; exit 1; }
sleep 2
srv() { echo "$1" > /tmp/server.in; }
srv "gamerule doDaylightCycle false"
srv "difficulty peaceful"
echo "PAPER READY"
