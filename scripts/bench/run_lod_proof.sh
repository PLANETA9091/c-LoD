#!/usr/bin/env bash
# v5.1.1 LOD-silhouette proof: low-camera vantage + A/B chain toggle +
# persistent far view. Companion to run_e2e_shots.sh (which proves the
# stream/veil); this one targets the LOD paint itself, following the
# v5.1.0 geometry findings:
#   - LOD silhouette is visible ONLY with the camera BELOW the terrain
#     heights (silhouette projects onto sky pixels above the horizon);
#   - the capture heightfield fills from a HIGH camera dwell looking down
#     (dust projects against terrain, both words captured);
#   - 26.3 tp pitch is inverted: plus = down, minus = up.
set -uo pipefail
S=/home/z/my-project/c-LoD/scripts/bench
SHOTS="$HOME/mc-test/shots"
mkdir -p "$SHOTS"

srv() { echo "$1" > /tmp/server.in; }
wait_for() { for i in $(seq 1 "$3"); do grep -q "$1" "$2" 2>/dev/null && return 0; sleep 1; done; return 1; }

# 1. server sanity
wait_for "pack finalized (ONE-TIME, static)" /tmp/server.log 5 || { echo "server not finalized"; exit 2; }
ps aux | grep "[p]aper-26.3" > /dev/null || { echo "server process dead"; exit 2; }

# 2. client + pack prompt
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
ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 -i :99 -frames:v 1 "$SHOTS/p00_prompt.png"
python3 "$S/click.py" 561 385   # Proceed
sleep 15
ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 -i :99 -frames:v 1 "$SHOTS/p00_pack_applied.png"
rg -N "Reloading ResourceManager" /tmp/client.log | tail -2 || true

# 3. world state: lock NOON + clear weather + spectator
srv "time set 6000"
srv "weather clear"
srv "gamemode spectator Stepan"
sleep 2

# 4. wait for the server scan to populate the disk (log counter fallback)
STREAM_OK=0; CELLS=0
for i in $(seq 1 240); do
  CELLS=$(rg -o "([0-9]+)/65536 cells" /tmp/server.log -r '$1' 2>/dev/null | tail -1 || echo 0)
  [ "${CELLS:-0}" -ge 2000 ] 2>/dev/null && { STREAM_OK=1; break; }
  sleep 1
done
echo "scan populated: cells=${CELLS:-0} (ok=$STREAM_OK)"

shot() { # name yaw pitch x y z
  srv "tp Stepan $4 $5 $6 $2 $3"
  sleep 3
  ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 -i :99 -frames:v 1 "$SHOTS/$1.png"
  echo "shot $1 (yaw=$2 pitch=$3 at $4,$5,$6)"
}

# 5. CAPTURE DWELL: high camera looking down (+pitch = down) — dust on both
#    planes projects against terrain, heightfield words fill.
shot p10_dwell_east 270 30 0 150 64
sleep 20
shot p10_dwell_west 90 30 0 150 64
sleep 20

# 6. LOW VANTAGE matrix at the v5.1.0 point (30,78,130): camera below the
#    surrounding heights -> silhouettes paint on sky pixels. One yaw per
#    cardinal; pitch -8 = slightly UP (26.3 inverted).
shot p11_low_east  270 -8 30 78 130
shot p12_low_south   0 -8 30 78 130
shot p13_low_west   90 -8 30 78 130
shot p14_low_north 180 -8 30 78 130

# 7. A/B: toggle ONLY the terrain chain (Paper /posteffect), same camera.
srv "posteffect remove Stepan clod:lod_terrain"
sleep 4
shot p15_ab_terrain_off 270 -8 30 78 130
srv "posteffect add Stepan clod:lod_terrain"
sleep 4
shot p16_ab_terrain_on 270 -8 30 78 130
srv "posteffect list Stepan"; sleep 2

# 8. PERSISTENT memory: move ~400 blocks south (village chunks unload),
#    look back north at where the terrain was.
shot p17_far_south 180 -8 30 78 530
sleep 25   # distant chunk unload window
shot p18_far_view_back 180 -8 30 78 530
srv "posteffect remove Stepan clod:lod_terrain"
sleep 4
shot p19_far_ab_off 180 -8 30 78 530
srv "posteffect add Stepan clod:lod_terrain"
sleep 2

echo "=== client log post markers ==="
rg -N -i "post effect|posteffect" /tmp/client.log | tail -10 || true
echo "=== server stream counters ==="
rg -N "stream: tick" /tmp/server.log | tail -4 || true
pkill -f "net.minecraft.client.main.Main" 2>/dev/null || true
echo "LOD PROOF DONE"
