#!/usr/bin/env bash
# Client e2e phase: launch vanilla client, auto-join, screenshot matrix.
# Requires: server already booted and pack finalized (boot_server.sh).
# v4.3 matrix: scan center (-930,796), atlas 1024 cell 4, seed 4819696040575474753.
set -uo pipefail
SHOTS="$HOME/mc-test/shots"
mkdir -p "$SHOTS"

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
# 1280x720 Xvfb: Yes button center ~562,385)
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

shot() { # name yaw pitch x y z
  srv "tp Stepan $4 $5 $6 $2 $3"
  sleep 3
  ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 -i :99 -frames:v 1 "$SHOTS/$1.png"
  echo "shot $1 (yaw=$2 pitch=$3 at $4,$5,$6)"
}

# 4. day matrix at the scan center — the "user view" from the bench sheet first
shot 01_user_day    -856.27 15.40  -929.67 173.38 796.37
shot 02_day_north   180 15  -930 173 796
shot 03_day_east    270 15  -930 173 796
shot 04_day_south     0 15  -930 173 796
shot 05_day_west     90 15  -930 173 796
shot 06_day_aerial  263 35  -930 220 796
shot 07_day_horizon 263  5  -930 200 796
shot 08_day_return -856.27 15.40  -929.67 173.38 796.37

# 5. grid sweep 5x5 over the atlas window (aerial, covers +-440 around center)
for gx in -1370 -1150 -930 -710 -490; do
  for gz in 376 586 796 1006 1216; do
    shot "g_${gx}_${gz}" 263 22 "$gx" 210 "$gz"
  done
done

echo "=== client log pack markers ==="
rg -N -i "pack|post" /tmp/client.log | tail -12 || true
pkill -f "net.minecraft.client.main.Main" 2>/dev/null || true
echo "E2E CLIENT PHASE DONE"
