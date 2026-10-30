#!/usr/bin/env bash
# SEEDGEN leg (v5.7): fresh world -> boot -> PLAYERLESS native fill measurement.
#
# The disk fills without any client: the pre-join arm fixes the origin and
# the seedgen workers generate never-generated chunks off the chunk system
# (no tickets, no TPS cost). This leg answers "how fast does the LOD disk
# fill on a cold world" — the seedgen headline metric.
#
# Usage: run_seedgen_leg.sh [MEASURE_SECS]    (default 180)
# Appends the report to /tmp/seedgen_legs.log.
set -euo pipefail
MEASURE_SECS="${1:-180}"
B=/home/z/my-project/c-LoD/scripts/bench

pkill -f "net.minecraft.client.main.Main" 2>/dev/null || true
pkill -f launcher.jar 2>/dev/null || true
pkill -f paper-26.3 2>/dev/null || true
pkill Xvfb 2>/dev/null || true
sleep 2

rm -rf ~/mc-test/dist/world ~/mc-test/dist/world_nether ~/mc-test/dist/world_the_end
echo "[$(date +%T)] seedgen leg: fresh world, playerless fill, ${MEASURE_SECS}s"
T0=$(date +%s)

bash "$B/boot_server.sh" || true
T1=$(date +%s)

sleep "$MEASURE_SECS"
T2=$(date +%s)

{
  echo "=== seedgen leg $(date +%F\ %T): boot ${T1}-${T0}s + measure ${MEASURE_SECS}s ==="
  python3 "$B/seedgen_rate.py" /tmp/server.log | tail -12
  echo "--- tps ---"
  echo "tps" > /tmp/server.in
  sleep 2
  grep -A2 "TPS from last 1m" /tmp/server.log | tail -3
  echo "--- gates ---"
  python3 /home/z/my-project/c-LoD/scripts/e2e/gates.py /tmp/server.log | tail -3
} | tee "/tmp/seedgen_leg_$(date +%H%M).txt" | tee -a /tmp/seedgen_legs.log
echo "[$(date +%T)] seedgen leg done"
