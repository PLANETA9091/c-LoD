#!/usr/bin/env bash
# Voxy visual-reference leg: clean Paper server + Fabric client with Voxy.
# Produces the reference screenshot matrix the c-LoD marcher is compared
# against (same seed, same vantages, same noon lock — the run_e2e_shots.sh
# recipe, minus the pack and the dust transport).
set -uo pipefail
SHOTS="$HOME/mc-test/voxy-shots"
BENCH="$(cd "$(dirname "$0")" && pwd)"
mkdir -p "$SHOTS"

# v6.8.6 LEGIBILITY: run 41472301678 died in <=1 s between "voxy leg start"
# and the first log file — the leg's own stdout went ONLY to the API-gated
# step log, so the feed saw "voxy leg start" then NOTHING. The whole leg
# now tees into voxy-leg.log (the publisher carries its tail in the
# failure marker) and every stage appends a trace line to voxy-steps.log —
# the same lesson v6.8.3 and v6.8.5 each taught once already.
TRACE="$HOME/mc-test/voxy-steps.log"
LEGLOG="$HOME/mc-test/voxy-leg.log"
t() { echo "$(date -u +%FT%TZ) voxy leg: $1" >> "$TRACE"; }
exec > >(tee -a "$LEGLOG") 2>&1
t "start (full leg stdout/stderr teed to voxy-leg.log)"

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
if bash "$BENCH/run_server_voxy.sh"; then
  t "server up (Done ( in voxy_server.log)"
else
  rc=$?
  t "SERVER SCRIPT FAILED rc=${rc} — the server boot echoes + its voxy_server.log tail are in voxy-leg.log above"
  exit 2
fi

# 2. client (Fabric + Voxy) — first boot runtime-remaps the game jar (slow)
bash "$BENCH/run_client_voxy.sh"
t "client launch issued"
wait_for "Stepan joined the game" /tmp/voxy_server.log 600 || {
  t "CLIENT NEVER JOINED within 600s — voxy_client.log tail is in voxy-leg.log below"
  echo "voxy client never joined"; tail -30 /tmp/voxy_client.log; exit 3; }
t "client joined"
echo "voxy client joined"
sleep 5

# 3. world state: lock NOON + clear weather + spectator (same as the c-LoD leg)
srv "gamerule doDaylightCycle false"
srv "time set noon"  # v6.16.3: was 'time set 6000' — NIGHT in the 26.3 tick
# scale (the v6.4.8 strobe lesson, un-applied on the reference leg): the
# parity gate was comparing voxy NIGHT frames against c-LoD NOON frames
# (run #276's 09_after_mesh triptych: voxy dark forest + stars vs our noon
# sky — a time-of-day diff masquerading as a renderer diff). The parity
# matrix must run BOTH legs at the same fixed noon.
srv "weather clear"
srv "gamemode spectator Stepan"
sleep 2

# 4. let Voxy scan+mesh the loaded chunks (WorldImporter). The first boot also
#    pays the Knot runtime remap. Meshing budget scales with the runner.
echo "waiting for voxy meshing..."
t "meshing wait ${MESH_WAIT}s"
sleep "$MESH_WAIT"
t "meshing wait done"
grep -iE "voxy" /tmp/voxy_client.log | tail -6 || true

SHOT_N=0
shot() { # name yaw pitch x y z
  srv "tp Stepan $4 $5 $6 $2 $3"
  sleep 3
  # -nostdin (v6.16.2): ffmpeg eats stdin — inside the matrix read-loop it
  # swallowed one byte of every next row, mangling the shot names
  # (run #275: 10_marker_ray -> 0_marker_ray).
  ffmpeg -nostdin -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 -i :99 -frames:v 1 "$SHOTS/$1.png" \
    || t "ffmpeg FAILED on $1 (rc=$?) — continuing, the rest of the matrix still shoots"
  SHOT_N=$((SHOT_N + 1))
  t "shot $1 ok ($SHOT_N/$PARITY_N)"
  echo "shot $1 (yaw=$2 pitch=$3 at $4,$5,$6)"
}

# v6.16 PARITY MATRIX: the shot list is the SHARED parity_vantages.txt — the
# c-LoD visual leg shoots the SAME rows into shots/parity/ (parity_<name>.png)
# and the parity-gate CI job pairs them BY NAME and fails below 100% pixel
# match. The matrix lives in one file so the two legs can never drift apart.
PARITY="$BENCH/parity_vantages.txt"
PARITY_N=$(grep -cv -e '^$' -e '^#' "$PARITY")
voxy_tour() { # every row except 09_after_mesh (it re-shoots after the lap wait)
  while read -r PNAME PYAW PPITCH PX PY PZ <&3; do
    case "$PNAME" in ''|\#*) continue;; esac
    [ "$PNAME" = "09_after_mesh" ] && continue
    # v6.16.1: arithmetic evaluation — "CX-75" must BECOME -1005 (the tp
    # parser rejects the literal "-930-75" string the plain substitution
    # produced). Decimal y values never enter $(()) — they pass through.
    PX=$(( ${PX//CX/$CX} )); PZ=$(( ${PZ//CZ/$CZ} ))
    shot "$PNAME" "$PYAW" "$PPITCH" "$PX" "$PY" "$PZ"
  done 3< "$PARITY"
}

voxy_tour

# 5. one more wait + re-shot: voxy keeps meshing outward — detail must grow
t "lap wait ${LAP_WAIT}s"
sleep "$LAP_WAIT"
shot 09_after_mesh   90 15   "$CX" 140 "$CZ"

echo "=== voxy client markers ==="
grep -iE "voxy|ssao|gl" /tmp/voxy_client.log | tail -12 || true
pkill -f "KnotClient" 2>/dev/null || true
pkill -f "paper-26.3" 2>/dev/null || true
t "done (${SHOT_N} shots in $SHOTS)"
echo "VOXY REFERENCE PHASE DONE"
