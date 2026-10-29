#!/usr/bin/env bash
# v5.3.2 ZERO-RESIDUAL e2e: the eye must never see the transport, from any
# vantage, including motion. Builds on the v5.3.1 stealth protocol:
#
#   A. top-down worst case (Y=280, pitch 90 = straight down): sky-blind
#      spawn gate means +0 spawns; both frames must be dot-free.
#   B. mid-band level view INSIDE the spawn cone (the residual class):
#      dwell until spawn waves hit the cone, then the ON/OFF pair +
#      dust_ab.py metric. Target: green_on == 0 (v5.3.1 had 19 px).
#   C. low 4-direction sweep (silhouette + eraser sanity).
#   D. MOTION: 90s camera spin during active streaming + quarter-point hop
#      tour; every mid-motion frame must be dot-free, counters must advance.
#
# Shots land in $SHOTS (default ~/mc-test/shots/v532). Foreground only
# (background processes die in this sandbox).
set -uo pipefail
S=/home/z/my-project/c-LoD/scripts/bench
SHOTS="${SHOTS:-$HOME/mc-test/shots/v532}"
# Phased run: PHASE=1 client+pack+scan wait, 2 = static A/B legs,
# 3 = sweep + motion tour, "all" = everything (foreground only — the
# sandbox kills background processes, and one Bash call caps at 10 min).
PHASE="${PHASE:-all}"
has() { [ "$PHASE" = "all" ] || [ "$PHASE" = "$1" ]; }
mkdir -p "$SHOTS"

srv() { echo "$1" > /tmp/server.in; }
wait_for() { for i in $(seq 1 "$3"); do grep -q "$1" "$2" 2>/dev/null && return 0; sleep 1; done; return 1; }
shot() { ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 \
          -framerate 2 -i :99 -frames:v 1 "$1" 2>/dev/null; echo "shot $(basename "$1")"; }
dwell() { local t=$1; local e=0; while [ "$e" -lt "$t" ]; do sleep 15; e=$((e+15)); srv "time set 6000"; done; }

# 1. server sanity
wait_for "pack finalized (ONE-TIME, static)" /tmp/server.log 5 || { echo "server not finalized"; exit 2; }
ps aux | grep "[p]aper-26.3" > /dev/null || { echo "server process dead"; exit 2; }
echo "=== /clod-pack.info (baseline) ==="
curl -s http://127.0.0.1:8199/clod-pack.info; echo

# 2. client + the ONE pack acceptance
if has 1; then
bash "$S/run_client.sh"
wait_for "Stepan joined the game" /tmp/server.log 420 || { echo "client never joined"; tail -20 /tmp/client.log; exit 3; }
echo "client joined"
sleep 5
PROMPT=0
for i in $(seq 1 60); do
  ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 -i :99 -frames:v 1 /tmp/prompt_probe.png 2>/dev/null
  if python3 -c "
from PIL import Image
im = Image.open('/tmp/prompt_probe.png').convert('RGB')
cnt = sum(1 for y in range(330, 365) for x in range(380, 900)
          if (lambda p: p[0] > 220 and p[1] > 220 and p[2] < 120)(im.getpixel((x, y))))
print(1 if cnt > 30 else 0)
" | grep -q "^1$"; then
    PROMPT=1; break
  fi
  sleep 2
done
echo "pack prompt visible=$PROMPT"
shot "$SHOTS/v00_prompt.png"
python3 "$S/click.py" 561 385   # Proceed
sleep 15
srv "gamerule doDaylightCycle false"
srv "time set 6000"
srv "weather clear"
srv "gamemode spectator Stepan"
sleep 2
srv "posteffect add Stepan clod:lod_terrain"
sleep 3
srv "posteffect list Stepan"
sleep 2
fi  # end phase 1

# 3. wait for the scan to populate the disk
if has 1; then
STREAM_OK=0; CELLS=0
for i in $(seq 1 240); do
  CELLS=$(rg -o "([0-9]+)/65536 cells" /tmp/server.log -r '$1' 2>/dev/null | tail -1 || echo 0)
  [ "${CELLS:-0}" -ge 2000 ] 2>/dev/null && { STREAM_OK=1; break; }
  sleep 1
done
echo "scan populated: cells=${CELLS:-0} (ok=$STREAM_OK)"
fi  # end phase 1 scan wait

# --- A. TOP-DOWN WORST CASE (sky-blind: spawn gate must give +0 spawns) ---
if has 2; then
echo "=== A. top-down Y=280 pitch 90 ==="
srv "tp Stepan 256 280 320 0 90"
dwell 30
shot "$SHOTS/v01_topdown_ON.png"
srv "posteffect remove Stepan clod:lod_terrain"
sleep 8
shot "$SHOTS/v02_topdown_OFF.png"
srv "posteffect add Stepan clod:lod_terrain"
sleep 5
python3 "$S/dust_ab.py" "$SHOTS/v01_topdown_ON.png" "$SHOTS/v02_topdown_OFF.png" "$SHOTS/metric_topdown.txt"
SPAWN_CULL=$(grep -c "culled" /tmp/server.log 2>/dev/null || echo 0)
echo "gate culled lines so far: $SPAWN_CULL"

# --- B. MID-BAND LEVEL VIEW (the residual class: spawns active in cone) ---
# NOTE: the scan is a ring sweep from the GRID CENTER (cell 128,128 = world
# (origin_x+256, origin_z+256) = (256,320) for CLOD_CENTER=0,64). The camera
# must sit inside the scanned circle, or every spawn is far-culled (the
# v5.3.1 live run had its camera there; the first v5.3.2 attempt sat 295
# blocks away and measured an empty transport).
echo "=== B. mid-band level view at the disk center (256,320) ==="
srv "tp Stepan 256 78 320 270 -8"
dwell 90
shot "$SHOTS/v03_level_ON.png"
srv "posteffect remove Stepan clod:lod_terrain"
sleep 8
shot "$SHOTS/v04_level_OFF.png"
srv "posteffect add Stepan clod:lod_terrain"
sleep 5
python3 "$S/dust_ab.py" "$SHOTS/v03_level_ON.png" "$SHOTS/v04_level_OFF.png" "$SHOTS/metric_level.txt"
fi  # end phase 2

# --- C. LOW 4-DIRECTION SWEEP ---
if has 3; then
echo "=== C. low sweep from (256,78,320) ==="
for yp in "270:-8:east" "0:-8:south" "90:-8:west" "180:-8:north"; do
  YAW="${yp%%:*}"; rest="${yp#*:}"; PITCH="${rest%%:*}"; DIR="${rest#*:}"
  srv "tp Stepan 256 78 320 $YAW $PITCH"
  sleep 10
  shot "$SHOTS/v05_low_$DIR.png"
done

# --- D. MOTION: spin + hop tour ---
echo "=== D. motion: 90s spin during active streaming ==="
srv "tp Stepan 256 78 320 270 -8"
sleep 5
python3 "$S/spin_camera.py" 90 140 0.12 &
SPIN_PID=$!
sleep 20
shot "$SHOTS/v06_spin1.png"
sleep 25
shot "$SHOTS/v07_spin2.png"
wait $SPIN_PID
sleep 8
shot "$SHOTS/v08_spin_end.png"

echo "=== D2. hop tour (quarter points, camera moving) ==="
srv "tp Stepan 200 95 280 225 20";  sleep 12; shot "$SHOTS/v09_hop_nw.png"
srv "tp Stepan 320 95 280 135 20";  sleep 12; shot "$SHOTS/v10_hop_ne.png"
srv "tp Stepan 320 95 380 45 20";   sleep 12; shot "$SHOTS/v11_hop_se.png"
srv "tp Stepan 200 95 380 315 20";  sleep 12; shot "$SHOTS/v12_hop_sw.png"

# final level A/B after all the motion (stream must still decode+erase)
srv "tp Stepan 256 78 320 270 -8"
dwell 60
shot "$SHOTS/v13_final_ON.png"
srv "posteffect remove Stepan clod:lod_terrain"
sleep 8
shot "$SHOTS/v14_final_OFF.png"
srv "posteffect add Stepan clod:lod_terrain"
sleep 5
python3 "$S/dust_ab.py" "$SHOTS/v13_final_ON.png" "$SHOTS/v14_final_OFF.png" "$SHOTS/metric_final.txt"

echo "=== /clod-pack.info (after) ==="
curl -s http://127.0.0.1:8199/clod-pack.info; echo
echo "=== gate stats ==="
grep -E "gate|culled" /tmp/server.log | tail -6 || true
echo "V532 E2E DONE"
fi  # end phase 3
