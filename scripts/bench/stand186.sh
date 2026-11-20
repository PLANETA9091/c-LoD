#!/bin/bash
# v6.59.10 THE FORCELoad VERDICT: window 11's world heights without any
# client. Paper 26.3 + forceload (256-chunk command limit — one full 16x16
# chunk probe inside window 11's x 1886..2994 z 3100..4108 square), then a
# graceful stop so Paper writes the overworld regions to the NEW layout
# (world/dimensions/minecraft/overworld/region), then parse_mca_heightmap.py
# prints the per-window min/max/mean and the verdict vs the atlas constant
# 186 (height byte 0xBA = 122 m under the ±64 code).
set -u
D="${MC_TEST_DIST:-$HOME/mc-test/dist}"
JAVA="${CLOD_JAVA:-$HOME/tools/jdk-25.0.5.1+2/bin/java}"
LOGL=/tmp/stand186.log
note() { echo "$(date +%H:%M:%S) $*" | tee -a "$LOGL"; }

mkdir -p "$D/versions"
cp -f /home/z/my-project/paper-26.3.jar "$D/versions/paper-26.3.jar"
cd "$D"
echo "eula=true" > eula.txt
rm -f /tmp/server.in /tmp/server.log
mkfifo /tmp/server.in 2>/dev/null || true
"$JAVA" -Xms256M -Xmx512M -XX:MaxMetaspaceSize=256M -XX:MaxDirectMemorySize=128M \
  -XX:+UseG1GC -jar versions/paper-26.3.jar --nogui \
  < <(tail -f /tmp/server.in) > /tmp/server.log 2>&1 &
SRV=$!
note "paper pid $SRV booting (jdk $("$JAVA" -version 2>&1 | head -1))"
for i in $(seq 1 90); do
  grep -q "Done (" /tmp/server.log && break
  grep -qE "Fatal|Exception in thread" /tmp/server.log && { note "BOOT FAILED:"; tail -20 /tmp/server.log | tee -a "$LOGL"; exit 1; }
  sleep 2
done
grep -q "Done (" /tmp/server.log || { note "boot timeout"; tail -30 /tmp/server.log | tee -a "$LOGL"; exit 1; }
note "boot done: $(grep -o 'Done ([^)]*)' /tmp/server.log | head -1)"

# 256 chunks (the command cap) inside window 11 — x 2200..2455, z 3500..3755.
echo "forceload add 2200 3500 2455 3755" > /tmp/server.in
sleep 2
note "forceload issued; waiting for the chunk generation to settle"
sleep 150
echo "forceload" > /tmp/server.in
sleep 3
note "forceload status: $(grep -c 'Marked' /tmp/server.log) marked lines"

# graceful stop — the only way regions reach disk (checked in #543..#550).
kill -TERM "$SRV"
for i in $(seq 1 40); do
  kill -0 "$SRV" 2>/dev/null || break
  sleep 1
done
kill -9 "$SRV" 2>/dev/null || true
sleep 2
note "stopped; regions:"
find world -name "*.mca" 2>/dev/null | head -8 | tee -a "$LOGL"

# The verdict: the region dir sits at the NEW layout on 26.3 — find it.
REG=$(find world -name "r.0.0.mca" 2>/dev/null | head -1)
[ -n "$REG" ] || REG=$(find world -name "r.*.mca" 2>/dev/null | head -1)
note "region sample: ${REG:-NONE}"
[ -n "$REG" ] && python3 /home/z/my-project/c-LoD/scripts/parse_mca_heightmap.py "$(dirname "$REG")" 2>&1 | tee -a "$LOGL"
note "VERDICT PASS DONE"
