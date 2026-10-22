#!/usr/bin/env bash
# Voxy A/B phase: same seeded bench server, real Voxy client, the user's
# reference camera. Produces 20_voxy_day / 21_voxy_sunset / 22_voxy_night.
# Run AFTER run_e2e_shots.sh (server finalized, daylight locked).
set -uo pipefail
SHOTS="$HOME/mc-test/shots"
S="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
mkdir -p "$SHOTS"

srv() { echo "$1" > /tmp/server.in; }
wait_for() { for i in $(seq 1 "$3"); do grep -q "$1" "$2" 2>/dev/null && return 0; sleep 1; done; return 1; }

bash "$S/run_voxy_client.sh"
wait_for "Stepan joined the game" /tmp/server.log 420 || { echo "voxy client never joined"; tail -30 /tmp/voxy_client.log; exit 3; }
echo "voxy client joined"
sleep 6

# The server pushes the c-LoD resource pack — DECLINE it (No button) so the
# posteffect LOD never renders on top of Voxy's own LODs (clean baseline).
python3 "$S/click.py" 718 385
sleep 5
ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 -i :99 -frames:v 1 "$SHOTS/20_voxy_join.png"

srv "gamemode spectator Stepan"
srv "weather clear"
srv "gamerule doDaylightCycle false"
srv "time set 6000"
sleep 2

# Grid flight around the reference camera so Voxy accumulates chunk data for
# its LOD pyramid over the same area our atlas window covers (±512 blocks).
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
rg -N -i "voxy|lod|error|warn" /tmp/voxy_client.log | tail -15 || true
pkill -f "net.fabricmc.loader.impl.launch.knot.KnotClient" 2>/dev/null || true
echo "VOXY AB PHASE DONE"
