#!/usr/bin/env bash
# DH reference one-shot: Fabric server WITH the DH server mod (the only way
# a DH client produces distant LODs — without the server's level key the
# client never creates its DhLevel), same seed as the c-LoD bench, then the
# user's reference camera + shot matrix against real DH rendering.
# Fabric server: ~/mc-test/dh-dist (port 25566, DH 3.3.4 + fabric-api).
set -u
S="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
D="$HOME/mc-test/dh-dist"
SHOTS="$HOME/mc-test/shots"
JAVA="$HOME/tools/jdk25/bin/java"
mkdir -p "$SHOTS"

cleanup() {
  pkill -f "net.fabricmc.loader" 2>/dev/null
  pkill -f "net.minecraft.client.main.Main" 2>/dev/null
  pkill -f fabric-server 2>/dev/null
  pkill Xvfb 2>/dev/null
  pkill -f "tail -f /tmp/fabcin" 2>/dev/null
  return 0
}
trap cleanup EXIT
cleanup; sleep 2

# ---- fabric + DH server (warm boot), console via fifo ----
rm -f /tmp/dhserver.log /tmp/fabcin
mkfifo /tmp/fabcin
cd "$D"
tail -f /tmp/fabcin 2>/dev/null | "$JAVA" -Xmx1500M -jar fabric-server.jar nogui > /tmp/dhserver.log 2>&1 &
SRV=$!
for i in $(seq 1 150); do
  grep -qE 'Done \(' /tmp/dhserver.log 2>/dev/null && break
  kill -0 $SRV 2>/dev/null || { echo "FABRIC SERVER DIED"; tail -20 /tmp/dhserver.log; exit 1; }
  sleep 2
done
grep -qE 'Done \(' /tmp/dhserver.log || { echo "FABRIC BOOT TIMEOUT"; tail -20 /tmp/dhserver.log; exit 1; }
echo "fabric DH server up"

# ---- DH client (connects to 25566) ----
# Client DH config for a fair A/B: LOD render distance = our atlas half-extent
# (128 chunks = 2048 blocks), distant-generation enabled (default radius 0 =
# disabled), no move-pause, generation area pinned to the camera chunks.
TOML="$HOME/mc-test/client/config/DistantHorizons.toml"
if [ -f "$TOML" ]; then
  sed -i 's/^[[:space:]]*lodChunkRenderDistanceRadius = .*/\t\t\t\tlodChunkRenderDistanceRadius = 128/' "$TOML"
  sed -i 's/^[[:space:]]*generationMaxChunkRadius = .*/\t\tgenerationMaxChunkRadius = 128/' "$TOML"
  sed -i 's/^[[:space:]]*pauseIfMovingQuickly = .*/\t\tpauseIfMovingQuickly = false/' "$TOML"
  sed -i 's/^[[:space:]]*generationCenterChunkX = .*/\t\tgenerationCenterChunkX = -58/' "$TOML"
  sed -i 's/^[[:space:]]*generationCenterChunkZ = .*/\t\tgenerationCenterChunkZ = 49/' "$TOML"
  echo "DH client config tuned (LOD RD 128, gen on)"
fi

CLOD_DH_PORT=25566 bash "$S/run_dh_client.sh" 2>&1 | tail -1
for i in $(seq 1 240); do grep -q "Stepan joined the game" /tmp/dhserver.log 2>/dev/null && break; sleep 1; done
grep -q "Stepan joined the game" /tmp/dhserver.log || { echo "dh client never joined"; tail -20 /tmp/dh_client.log; exit 3; }
echo "dh client joined fabric server"
sleep 5
ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 -i :99 -frames:v 1 "$SHOTS/20_dh_join.png"

# ---- spectator + daylight lock + camera corridor seed flight ----
f() { echo "$1" > /tmp/fabcin; }
f "gamemode spectator Stepan"
f "weather clear"
f "gamerule doDaylightCycle false"
f "time set 6000"
sleep 2

# Short seed flight along the user's view corridor (server real-time LOD gen
# covers 256 chunks around the player — 9 stops seed the whole area fast).
UX=-929; UZ=796; UYAW=223; UPITCH=15
i=0
for t in 0 512 1024 1536; do
  for lat in -256 0 256; do
    i=$((i+1))
    FX=$(python3 -c "print(round($UX + 0.691*$t + 0.723*$lat))")
    FZ=$(python3 -c "print(round($UZ - 0.723*$t + 0.691*$lat))")
    f "tp Stepan $FX 200 $FZ $UYAW $UPITCH"
    sleep 3
  done
done
echo "seed flight done ($i stops) — settling for DH server-side LOD gen"
sleep 40

shot() { # name time pitch y
  f "time set $2"
  sleep 2
  f "tp Stepan -929.67 $4 796.37 -856.27 $3"
  sleep 8
  ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 -i :99 -frames:v 1 "$SHOTS/$1.png"
  echo "shot $1 (time=$2)"
}

shot 20_dh_day         6000  15.40 173.38
f "tp Stepan -929.67 173.38 796.37 0 15.40";     sleep 6
ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 -i :99 -frames:v 1 "$SHOTS/21_dh_yaw_south.png"
f "tp Stepan -929.67 173.38 796.37 90 15.40";    sleep 6
ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 -i :99 -frames:v 1 "$SHOTS/22_dh_yaw_east.png"
f "tp Stepan -929.67 173.38 796.37 180 15.40";   sleep 6
ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 -i :99 -frames:v 1 "$SHOTS/23_dh_yaw_north.png"
shot 24_dh_aerial      6000  15.40 240
shot 25_dh_sunset     12610  15.40 173.38
shot 26_dh_night      18000  15.40 173.38
shot 27_dh_day_return  6000  15.40 173.38

echo "=== dh client log: DH markers ==="
grep -iE "distant horizons|DHLevel|LOD|level key" /tmp/dh_client.log | tail -6
echo "DH REFERENCE COMPLETE $(date +%H:%M:%S)"
