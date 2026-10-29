#!/usr/bin/env bash
# v5.1.1 CLEAN A/B: clouds disabled client-side (renderClouds:false) so the
# terrain-chain toggle is the ONLY variable between the paired shots.
# Sequence per vantage: shot ON -> /posteffect remove -> shot OFF -> add back.
set -uo pipefail
S=/home/z/my-project/c-LoD/scripts/bench
SHOTS="$HOME/mc-test/shots"
mkdir -p "$SHOTS"

srv() { echo "$1" > /tmp/server.in; }
wait_for() { for i in $(seq 1 "$3"); do grep -q "$1" "$2" 2>/dev/null && return 0; sleep 1; done; return 1; }

wait_for "pack finalized (ONE-TIME, static)" /tmp/server.log 5 || { echo "server not finalized"; exit 2; }
ps aux | grep "[p]aper-26.3" > /dev/null || { echo "server process dead"; exit 2; }

bash "$S/run_client.sh"
wait_for "Stepan joined the game" /tmp/server.log 420 || { echo "client never joined"; tail -20 /tmp/client.log; exit 3; }
echo "client joined"
sleep 5

# pack prompt (blind click fallback: the pack is pushed exactly once and the
# Proceed button sits at the same spot every boot)
python3 "$S/click.py" 561 385 || true
sleep 12
rg -N "Reloading ResourceManager" /tmp/client.log | tail -2 || true

srv "time set 6000"
srv "weather clear"
srv "gamemode spectator Stepan"
sleep 2

shot() { # name yaw pitch x y z
  srv "tp Stepan $4 $5 $6 $2 $3"
  sleep 3
  ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 -i :99 -frames:v 1 "$SHOTS/$1.png"
  echo "shot $1 (yaw=$2 pitch=$3 at $4,$5,$6)"
}

# CAPTURE DWELL: high camera looking down, both directions, heightfield fills
shot c10_dwell_east 270 30 0 150 64
sleep 22
shot c10_dwell_west 90 30 0 150 64
sleep 22

# --- A/B at the low vantage (30,78,130), looking east at the coast ---
shot c11_low_on 270 -8 30 78 130
srv "posteffect remove Stepan clod:lod_terrain"
sleep 5
shot c12_low_off 270 -8 30 78 130
srv "posteffect add Stepan clod:lod_terrain"
sleep 5

# --- A/B at the far vantage (30,78,530), looking back north ---
shot c13_far_on 180 -8 30 78 530
sleep 20   # chunk unload window behind the vantage
srv "posteffect remove Stepan clod:lod_terrain"
sleep 5
shot c14_far_off 180 -8 30 78 530
srv "posteffect add Stepan clod:lod_terrain"
sleep 5

echo "=== posteffect state ==="
srv "posteffect list Stepan"; sleep 2
rg -N "post effect" /tmp/server.log | tail -6 || true
pkill -f "net.minecraft.client.main.Main" 2>/dev/null || true
echo "CLEAN A/B DONE"
