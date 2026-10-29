#!/usr/bin/env bash
# v5.3.2 residual burst: measure the eraser against ACTIVE spawn waves.
# 10 raw frames (chain removed) -> burst OFF; 10 erased frames (chain on)
# -> burst ON. Server gate log correlates spawn activity per frame.
set -uo pipefail
S=/home/z/my-project/c-LoD/scripts/bench
SHOTS="$HOME/mc-test/shots/v532"
mkdir -p "$SHOTS"
srv() { echo "$1" > /tmp/server.in; }
shot() { ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 \
          -framerate 2 -i :99 -frames:v 1 "$1" 2>/dev/null; }

# camera: mid-band level view inside the spawn cone (AT THE DISK CENTER —
# the scan is a ring sweep from the grid center (256,320); elsewhere every
# spawn is far-culled and the measurement is empty)
srv "gamemode spectator Stepan"
srv "tp Stepan 256 78 320 270 -8"
sleep 8
srv "time set 6000"

echo "=== burst OFF (chain removed, raw dust) ==="
srv "posteffect remove Stepan clod:lod_terrain"
sleep 6
for i in $(seq 0 9); do
  srv "time set 6000"
  shot "$SHOTS/b_off_$i.png"
  sleep 2
done
srv "posteffect add Stepan clod:lod_terrain"
sleep 6

echo "=== burst ON (chain active, eraser on) ==="
for i in $(seq 0 9); do
  srv "time set 6000"
  shot "$SHOTS/b_on_$i.png"
  sleep 2
done

echo "=== gate log (spawns during the bursts) ==="
grep -E "gate|spawn" /tmp/server.log | tail -8 || true
echo "BURST DONE"
