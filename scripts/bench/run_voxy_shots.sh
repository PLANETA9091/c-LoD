#!/usr/bin/env bash
# Voxy A/B phase B: the pyramid meshing continued since the flight phase
# (the client was left alive). Wait a bit more, then take the 4 shots.
set -uo pipefail
SHOTS="$HOME/mc-test/shots"
S="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
mkdir -p "$SHOTS"
srv() { echo "$1" > /tmp/server.in; }
grab() { ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 -i :99 -frames:v 1 "$1" 2>/dev/null; }

pgrep -f "net.fabricmc.loader.impl.launch.knot.KnotClient" >/dev/null || { echo "voxy client is gone — rerun the flight phase"; exit 3; }
echo "client alive, meshing settles"
sleep 45

srv "gamemode spectator Stepan"
srv "weather clear"
srv "gamerule doDaylightCycle false"
srv "time set 6000"
sleep 2

UX=-929.67; UY=173.38; UZ=796.37; UYAW=-856.27; UPITCH=15.40

vshot() { # name time
  srv "time set $2"
  sleep 2
  srv "tp Stepan $UX $UY $UZ $UYAW $UPITCH"
  sleep 7
  grab "$SHOTS/$1.png"
  echo "shot $1 (time=$2, user camera)"
}

vshot 20_voxy_day 6000
vshot 21_voxy_sunset 12610
vshot 22_voxy_night 18000
vshot 23_voxy_day_return 6000

echo "=== voxy client log tail ==="
tail -12 /tmp/voxy_client.log
pkill -9 -f "net.fabricmc.loader.impl.launch.knot.KnotClient" 2>/dev/null || true
pkill -9 -f "java.*KnotClient" 2>/dev/null || true
echo "VOXY AB PHASE DONE"
