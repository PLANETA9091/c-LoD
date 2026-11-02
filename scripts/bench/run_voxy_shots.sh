#!/usr/bin/env bash
# Voxy visual-reference leg: clean Paper server + Fabric client with Voxy.
# Produces the reference screenshot matrix the c-LoD marcher is compared
# against (same seed, same vantages, same noon lock — the run_e2e_shots.sh
# recipe, minus the pack and the dust transport).
set -uo pipefail
SHOTS="$HOME/mc-test/voxy-shots"
BENCH="$(cd "$(dirname "$0")" && pwd)"
mkdir -p "$SHOTS"

CX="${VOXY_VANTAGE_X:--930}"; CZ="${VOXY_VANTAGE_Z:-796}"   # the c-LoD visual leg's disk center — same terrain, same vantages
MESH_WAIT="${VOXY_MESH_WAIT:-240}"
LAP_WAIT="${VOXY_LAP_WAIT:-120}"

srv() { echo "$1" > /tmp/server.in; }

wait_for() { # pattern, file, timeout_s
  for i in $(seq 1 "$3"); do
    grep -q "$1" "$2" 2>/dev/null && return 0
    sleep 1
  done
  return 1
}

# 1. server
bash "$BENCH/run_server_voxy.sh" || exit 2

# 2. client (Fabric + Voxy) — first boot runtime-remaps the game jar (slow)
bash "$BENCH/run_client_voxy.sh"
wait_for "Stepan joined the game" /tmp/voxy_server.log 600 || {
  echo "voxy client never joined"; tail -30 /tmp/voxy_client.log; exit 3; }
echo "voxy client joined"
sleep 5

# 3. world state: lock NOON + clear weather + spectator (same as the c-LoD leg)
srv "gamerule doDaylightCycle false"
srv "time set 6000"
srv "weather clear"
srv "gamemode spectator Stepan"
sleep 2

# 4. let Voxy scan+mesh the loaded chunks (WorldImporter). The first boot also
#    pays the Knot runtime remap. Meshing budget scales with the runner.
echo "waiting for voxy meshing..."
sleep "$MESH_WAIT"
grep -iE "voxy" /tmp/voxy_client.log | tail -6 || true

shot() { # name yaw pitch x y z
  srv "tp Stepan $4 $5 $6 $2 $3"
  sleep 3
  ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 -i :99 -frames:v 1 "$SHOTS/$1.png"
  echo "shot $1 (yaw=$2 pitch=$3 at $4,$5,$6)"
}

shot 01_aerial_west   90 15   "$CX" 140 "$CZ"
shot 02_aerial_south   0 15   "$CX" 140 "$CZ"
shot 03_aerial_north 180 15   "$CX" 140 "$CZ"
shot 04_aerial_east  270 15   "$CX" 140 "$CZ"
shot 05_oblique      135 30   "$CX" 170 "$CZ"
shot 06_ground_west   90  0   "$CX" 100 "$CZ"
shot 07_far_north    180  5   "$CX" 160 "-220"
shot 08_back_inside   90  0   "$CX" 120 "$CZ"

# 5. one more wait + re-shot: voxy keeps meshing outward — detail must grow
sleep "$LAP_WAIT"
shot 09_after_mesh   90 15   "$CX" 140 "$CZ"

echo "=== voxy client markers ==="
grep -iE "voxy|ssao|gl" /tmp/voxy_client.log | tail -12 || true
pkill -f "KnotClient" 2>/dev/null || true
pkill -f "paper-26.3" 2>/dev/null || true
echo "VOXY REFERENCE PHASE DONE"
