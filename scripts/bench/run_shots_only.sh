#!/usr/bin/env bash
# Shot-only phase: client joins (pack auto-applies), waits for cells, matrix.
set -uo pipefail
SHOTS="$HOME/mc-test/shots"
CX=-1186; CZ=542
srv() { echo "$1" > /tmp/server.in; }
wait_for() {
  for i in $(seq 1 "$3"); do grep -q "$1" "$2" 2>/dev/null && return 0; sleep 1; done
  return 1
}

wait_for "pack finalized" /tmp/server.log 5 || exit 2
bash /home/z/my-project/c-LoD/scripts/bench/run_client.sh
wait_for "Stepan joined the game" /tmp/server.log 420 || { echo "no join"; exit 3; }
srv "gamemode spectator Stepan"
sleep 2
srv "tp Stepan $CX 100 $CZ 90 15"
echo "joined; waiting for the pack reload + cells"
wait_for "Reloading ResourceManager: vanilla, vanilla, server/" /tmp/client.log 90
sleep 8

for i in $(seq 1 60); do
  CELLS=$(curl -s --max-time 3 http://127.0.0.1:8199/clod-pack.info | rg -o "cells=([0-9]+)/([0-9]+)" -r '$1' 2>/dev/null || echo 0)
  [ "${CELLS:-0}" -ge 1500 ] 2>/dev/null && break
  sleep 1
done
echo "cells=${CELLS:-0} (streaming populated)"

shot() {
  srv "time set 6000"
  sleep 1
  srv "tp Stepan $4 $5 $6 $2 $3"
  sleep 3
  ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 -i :99 -frames:v 1 "$SHOTS/$1.png"
  echo "shot $1"
}

shot 01_aerial_west   90 15 "$CX" 130 "$CZ"
shot 02_aerial_south   0 15 "$CX" 130 "$CZ"
shot 03_aerial_north 180 15 "$CX" 130 "$CZ"
shot 04_aerial_east  270 15 "$CX" 130 "$CZ"
shot 05_oblique      135 25 "$CX" 150 "$CZ"
shot 06_ground       90  10 "$CX" 110 "$CZ"
sleep 35
shot 07_after_lap    90 15 "$CX" 130 "$CZ"
echo "=== server stream counters ==="
rg -N "stream: tick" /tmp/server.log | tail -3 || true
pkill -f "net.minecraft.client.main.Main" 2>/dev/null || true
echo "SHOTS DONE"
