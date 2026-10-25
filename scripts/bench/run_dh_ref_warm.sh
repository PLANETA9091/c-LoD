#!/usr/bin/env bash
# DH reference WARM re-run: both worlds already carry the seed-flight LOD data
# (server DistantHorizons.db + client local gen from the first run_dh_ref.sh).
# Boot server, join client, straight to the reference camera, shot matrix.
# Fits one 580s tool call.
set -u
S="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
D="$HOME/mc-test/dh-dist"
SHOTS="$HOME/mc-test/shots"
JAVA="$HOME/tools/jdk25/bin/java"
mkdir -p "$SHOTS"

cleanup() {
  pkill -f "net.minecraft.client.main.Main" 2>/dev/null
  pkill -f KnotClient 2>/dev/null
  pkill -f fabric-server 2>/dev/null
  pkill -f KnotServer 2>/dev/null
  pkill Xvfb 2>/dev/null
  pkill -f "tail -f /tmp/fabcin" 2>/dev/null
  return 0
}
trap cleanup EXIT
cleanup; sleep 2

rm -f /tmp/dhserver.log /tmp/fabcin
mkfifo /tmp/fabcin
cd "$D"
tail -f /tmp/fabcin 2>/dev/null | "$JAVA" -Xmx600M -jar fabric-server.jar nogui > /tmp/dhserver.log 2>&1 &
for i in $(seq 1 90); do
  grep -qE 'Done \(' /tmp/dhserver.log 2>/dev/null && break
  sleep 2
done
grep -qE 'Done \(' /tmp/dhserver.log || { echo "FABRIC BOOT TIMEOUT"; tail -10 /tmp/dhserver.log; exit 1; }
echo "fabric DH server up (warm)"

f() { echo "$1" > /tmp/fabcin; }

CLOD_DH_PORT=25566 bash "$S/run_dh_client.sh" 2>&1 | tail -1
for i in $(seq 1 240); do grep -q "Stepan joined the game" /tmp/dhserver.log 2>/dev/null && break; sleep 1; done
grep -q "Stepan joined the game" /tmp/dhserver.log || { echo "dh client never joined"; tail -10 /tmp/dh_client.log; exit 3; }
echo "dh client joined"

f "gamemode spectator Stepan"
f "weather clear"
f "gamerule doDaylightCycle false"
f "time set 6000"

# let the DH client rebuild its GL pipeline + pull the warm LOD ring
# (server-side gen only: client generationMaxChunkRadius=0, so the band
#  appears as the server syncs — give it a full soak window)
# long DH client-gen soak: llvmpipe + 2 cores needs minutes for the
# near LOD ring even at gen radius 32 (observed: render thread pinned in
# BlazeVanillaFadeRenderer while the gen queue crawls)
sleep 300

shot() { # name time pitch y yaw
  f "time set $2"
  sleep 2
  f "tp Stepan -929.67 $4 796.37 $5 $3"
  sleep 8
  ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 -i :99 -frames:v 1 "$SHOTS/$1.png"
  echo "shot $1 (time=$2)"
}

shot 20_dh_day         6000  15.40 173.38 -856.27
shot 21_dh_yaw_south   6000  15.40 173.38 0
shot 22_dh_yaw_east    6000  15.40 173.38 90
shot 23_dh_yaw_north   6000  15.40 173.38 180
shot 24_dh_aerial      6000  15.40 240.00 -856.27

echo "=== dh client log: DH markers ==="
grep -icE "Distant Horizons 3" /tmp/dh_client.log || true
echo "DH REFERENCE COMPLETE $(date +%H:%M:%S)"
