#!/usr/bin/env bash
# Client phase v2: join -> accept pack RETRY loop -> verify post effects ->
# screenshot matrix. Client stays alive the whole time.
set -uo pipefail
SHOTS="$HOME/mc-test/shots"
S=/home/z/my-project/c-LoD/scripts/bench
mkdir -p "$SHOTS"

srv() { echo "$1" > /tmp/server.in; }

bash "$S/run_client.sh"

# wait for join
for i in $(seq 1 420); do
  grep -q "Stepan joined the game" /tmp/server.log 2>/dev/null && break
  sleep 1
done
grep -q "Stepan joined the game" /tmp/server.log || { echo "client never joined"; tail -20 /tmp/client.log; exit 3; }
echo "client joined"
sleep 6

# accept the pack: the push arrives after join, so retry clicks until the
# client actually reloads with the server pack (no clod post-effect warning)
accepted=0
for try in 1 2 3 4 5 6; do
  python3 "$S/click.py" 562 385 || true
  sleep 14
  if ! grep -q "Requested post effect does not exist: clod:lod_terrain" /tmp/client.log \
     && grep -qE "Reloading ResourceManager: vanilla, " /tmp/client.log; then
    accepted=1
    break
  fi
  echo "pack not yet accepted (try $try)"
done
[ "$accepted" = 1 ] || { echo "PACK NEVER ACCEPTED"; tail -30 /tmp/client.log; exit 4; }
echo "PACK ACCEPTED"

ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 -i :99 -frames:v 1 "$SHOTS/00_pack_applied.png"

# world state: lock NOON + clear weather + spectator
srv "gamerule doDaylightCycle false"
srv "time set 6000"
srv "weather clear"
srv "gamemode spectator Stepan"
sleep 2

shot() { # name yaw pitch x y z
  srv "tp Stepan $4 $5 $6 $2 $3"
  sleep 3
  ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 -i :99 -frames:v 1 "$SHOTS/$1.png"
  echo "shot $1"
}

shot 01_user_day    -856.27 15.40  -929.67 173.38 796.37
shot 02_day_north   180 15  -930 173 796
shot 03_day_east    270 15  -930 173 796
shot 04_day_south     0 15  -930 173 796
shot 05_day_west     90 15  -930 173 796
shot 06_day_aerial  263 35  -930 220 796
shot 07_day_horizon 263  5  -930 200 796
shot 08_day_return -856.27 15.40  -929.67 173.38 796.37

# grid sweep 5x5 over the atlas window
for gx in -1370 -1150 -930 -710 -490; do
  for gz in 376 586 796 1006 1216; do
    shot "g_${gx}_${gz}" 263 22 "$gx" 210 "$gz"
  done
done

echo "=== client log pack markers ==="
rg -N -i "post effect|Reloading|pack" /tmp/client.log | tail -8 || true
echo "CLIENT PHASE v2 DONE"
