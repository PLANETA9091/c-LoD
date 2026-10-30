#!/usr/bin/env bash
# One leg of the v5.5 frontier A/B: fresh world -> boot (optional paper
# thread patch) -> client join -> MEASURE_SECS of scan-rate sampling.
#
# Usage: run_frontier_leg.sh LABEL [PAPER_WORKERS] [PAPER_IO]
# Results appended to /tmp/frontier_ab.log (summary) and /tmp/leg_<LABEL>.txt
set -euo pipefail
LABEL="${1:?label}"
WORKERS="${2:-}"
IO="${3:-}"
MEASURE_SECS="${MEASURE_SECS:-720}"
B=/home/z/my-project/c-LoD/scripts/bench

pkill -f "net.minecraft.client.main.Main" 2>/dev/null || true
pkill -f launcher.jar 2>/dev/null || true
pkill -f paper-26.3 2>/dev/null || true
pkill Xvfb 2>/dev/null || true
sleep 2

# FAIR A/B: identical seed (server.properties level-seed), identical fresh
# world, identical client. The only delta is the paper chunk-system patch.
rm -rf ~/mc-test/dist/world ~/mc-test/dist/world_nether ~/mc-test/dist/world_the_end
echo "[$(date +%T)] leg $LABEL: fresh world, workers='${WORKERS:-auto}' io='${IO:-auto}'"

if [ -n "$WORKERS" ]; then
  PAPER_WORKERS="$WORKERS" PAPER_IO="$IO" bash "$B/boot_server.sh"
else
  bash "$B/boot_server.sh"
fi

# Client: fresh launch, auto-join.
setsid bash "$B/run_client.sh" > /tmp/client_boot.log 2>&1 < /dev/null &
disown

# Wait for the join (the scan is player-gated).
JOINED=0
for i in $(seq 1 90); do
  if grep -q "Stepan joined the game" /tmp/server.log 2>/dev/null; then JOINED=1; break; fi
  sleep 2
done
[ "$JOINED" = "1" ] || { echo "leg $LABEL: CLIENT NEVER JOINED"; exit 1; }
JOIN_T=$(grep "Stepan joined the game" /tmp/server.log | head -1 | awk '{print $1,$2}')
echo "[$(date +%T)] leg $LABEL: joined at $JOIN_T, measuring ${MEASURE_SECS}s"
sleep "$MEASURE_SECS"

{
  echo "=== leg $LABEL workers=${WORKERS:-auto} io=${IO:-auto} measure=${MEASURE_SECS}s ==="
  python3 "$B/scan_rate.py" /tmp/server.log | tail -8
  echo "--- tps ---"
  echo "tps" > /tmp/server.in
  sleep 2
  grep -A2 "TPS from last 1m" /tmp/server.log | tail -3
  echo "--- forced/wish counters ---"
  grep -oE "stream: tick [0-9]+ [0-9]+/[0-9]+ cells" /tmp/server.log | tail -3
} | tee "/tmp/leg_${LABEL}.txt" | tee -a /tmp/frontier_ab.log
echo "[$(date +%T)] leg $LABEL done"
