#!/usr/bin/env bash
# Voxy A/B phase 2: grid flight + day/sunset/night shots. REQUIRES phase 1
# (run_voxy_join.sh) to have completed: the Voxy client is in the world at
# the user camera with the c-LoD pack declined. Does NOT launch or kill the
# client (phase 1 owns the lifecycle up to join; cleanup happens here).
set -uo pipefail
SHOTS="$HOME/mc-test/shots"
S="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
mkdir -p "$SHOTS"

srv() { echo "$1" > /tmp/server.in; }

srv "gamemode spectator Stepan"
srv "weather clear"
srv "gamerule doDaylightCycle false"
srv "time set 6000"
sleep 2

# Grid flight around the reference camera so Voxy accumulates chunk data for
# its LOD pyramid over the same area our atlas window covers (±512 blocks;
# server atlas 512 => ±256 band, voxy RD 4 sections = 256 blocks — matched).
# 5x5 stops, spacing 192 blocks, camera height 200.
UX=-929.67; UY=173.38; UZ=796.37; UYAW=-856.27; UPITCH=15.40
i=0
for dz in -384 -192 0 192 384; do
  for dx in -384 -192 0 192 384; do
    i=$((i+1))
    srv "tp Stepan $(( ${UX%.*} + dx )) 200 $(( ${UZ%.*} + dz )) $UYAW $UPITCH"
    sleep 4
  done
done
echo "grid flight done ($i stops) — settling for Voxy meshing"
sleep 15

vshot() { # name time
  srv "time set $2"
  sleep 2
  srv "tp Stepan $UX $UY $UZ $UYAW $UPITCH"
  sleep 8
  ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 -i :99 -frames:v 1 "$SHOTS/$1.png"
  echo "shot $1 (time=$2, user camera)"
}

vshot 20_voxy_day 6000
vshot 21_voxy_sunset 12610
vshot 22_voxy_night 18000
vshot 23_voxy_day_return 6000

echo "=== voxy client log tail ==="
tail -30 /tmp/voxy_client.log
pkill -f "net.fabricmc.loader.impl.launch.knot.KnotClient" 2>/dev/null || true
echo "VOXY AB PHASE DONE"
