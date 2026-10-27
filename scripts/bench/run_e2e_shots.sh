#!/usr/bin/env bash
# Client e2e phase: launch vanilla client, auto-join, screenshot matrix.
# Requires: server already booted and pack finalized (boot_server.sh).
# v5.0.1: shader-only pack (ONE-TIME, static marker); disk is FIXED at
# cfg center (bench: -1186,542, radius 256); all LOD data rides dust
# particles — the client phase also waits for the scan/stream to populate.
set -uo pipefail
SHOTS="$HOME/mc-test/shots"
mkdir -p "$SHOTS"

CX=-1186; CZ=542      # bench disk center (boot_server.sh CLOD_OPTS)
EDGE=$(( 256 ))       # disk radius in blocks

srv() { echo "$1" > /tmp/server.in; }

wait_for() { # pattern, file, timeout_s
  for i in $(seq 1 "$3"); do
    grep -q "$1" "$2" 2>/dev/null && return 0
    sleep 1
  done
  return 1
}

# 1. server sanity
wait_for "pack finalized (ONE-TIME, static)" /tmp/server.log 5 || { echo "server not finalized"; exit 2; }
ps aux | grep "[p]aper-26.3" > /dev/null || { echo "server process dead"; exit 2; }
curl -s -o /tmp/pack_check.zip -w "pack HTTP %{http_code} %{size_download}B\n" http://127.0.0.1:8199/clod-pack.zip

# 2. client
bash /home/z/my-project/c-LoD/scripts/bench/run_client.sh
wait_for "Stepan joined the game" /tmp/server.log 420 || { echo "client never joined"; tail -20 /tmp/client.log; exit 3; }
echo "client joined"
sleep 5
ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 -i :99 -frames:v 1 "$SHOTS/00_join_state.png"
echo "join-state shot taken (pack prompt expected)"

# the server pack prompt is on screen — click Yes (window 640x360 centered on
# 1280x720 Xvfb: Yes button spans x 489..635, y 376..394 -> center ~562,385)
python3 /home/z/my-project/c-LoD/scripts/bench/click.py 562 385
sleep 12
ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 -i :99 -frames:v 1 "$SHOTS/00_pack_applied.png"
echo "pack-accept clicked"

# 3. world state: lock NOON (the v4.0-killer scenario) + clear weather
srv "gamerule doDaylightCycle false"
srv "time set 6000"
srv "weather clear"
srv "gamemode spectator Stepan"
sleep 2

# 3b. THE STREAM: wait for arming, then for the scan to populate the disk.
#      All LOD bytes arrive as dust particles; shots before this show empty sky.
wait_for "sky-grid stream armed" /tmp/server.log 120 \
  && echo "stream armed" || echo "WARN: stream not armed in 120s"
STREAM_OK=0
for i in $(seq 1 300); do
  CELLS=$(curl -s http://127.0.0.1:8199/clod-pack.info | rg -o "cells=([0-9]+)/([0-9]+)" -r '$1' || echo 0)
  [ "${CELLS:-0}" -ge 2000 ] 2>/dev/null && { STREAM_OK=1; break; }
  sleep 1
done
echo "scan populated: cells=${CELLS:-0} (ok=$STREAM_OK)"
curl -s http://127.0.0.1:8199/clod-pack.info | head -4

# 4. LOD-heavy shot matrix (v5: fixed disk at CX,CZ). Aerial y=140 decodes
#    the low particle layer (80) against terrain below; oblique/ground vary
#    the vantage. Outside-disk shot must show NO LOD (pure vanilla + veil).
shot() { # name yaw pitch x y z
  srv "tp Stepan $4 $5 $6 $2 $3"
  sleep 3
  ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 -i :99 -frames:v 1 "$SHOTS/$1.png"
  echo "shot $1 (yaw=$2 pitch=$3 at $4,$5,$6)"
}

shot 01_aerial_west   90 15   "$CX" 140 "$CZ"
shot 02_aerial_south   0 15   "$CX" 140 "$CZ"
shot 03_aerial_north 180 15   "$CX" 140 "$CZ"
shot 04_aerial_east  270 15   "$CX" 140 "$CZ"
shot 05_oblique      135 30   "$CX" 170 "$CZ"
shot 06_ground_west   90  0   "$CX" 100 "$CZ"
shot 07_veil_outside  90 10     0 140   0
shot 08_back_inside   90  0   "$CX" 120 "$CZ"

# 5. STREAMING PROOF: wait one more partial lap (~30 s) and re-shoot — the
#    LOD must GAIN detail purely from newly decoded particles.
sleep 30
shot 09_after_lap    90 15   "$CX" 140 "$CZ"

echo "=== client log pack markers ==="
rg -N -i "pack|post" /tmp/client.log | tail -12 || true
echo "=== server stream counters ==="
rg -N "stream|cells|laps" /tmp/server.log | tail -8 || true
pkill -f "net.minecraft.client.main.Main" 2>/dev/null || true
echo "E2E CLIENT PHASE DONE"
