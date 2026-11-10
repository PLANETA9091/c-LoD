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

# 3b. THE SHARED STRUCTURES (v6.17): the c-LoD leg's world is the seed PLUS
#     two terraced mountains + the 5x111x5 red marker column (placed with
#     the vanilla forceload+fill mechanism — the v6.15 terrain range and the
#     v6.14 marker). v6.16's gate compared a BUILT world against a PRISTINE
#     one: even a perfect renderer could never pair the frames, because the
#     voxy world did not contain the structures ours renders (run #281,
#     10_marker_ray: voxy real landscape vs our sky+towers). The reference
#     leg now places the IDENTICAL structures at the IDENTICAL coords from
#     run_ci_visual.sh §3a2/§5 (P1 (CX-4,CZ-20) tops 150/175/185; P2
#     (CX-2,CZ+16) tops 155/180/190; marker CX±2 y60..170) BEFORE the
#     meshing wait, so voxy's WorldImporter imports and meshes them like
#     any other chunk. Not fatal on failure: a "position not loaded" hit
#     degrades that pair's parity honestly in the gate report.
build_mountain() { # cx cz top1 top2 top3 label
  local mx="$1" mz="$2" h1="$3" h2="$4" h3="$5" lb="$6"
  srv "forceload add $((mx - 12)) $((mz - 12)) $((mx + 12)) $((mz + 12))"
  sleep 2
  srv "fill $((mx - 6)) 55 $((mz - 6)) $((mx + 6)) $((h1 - 1)) $((mz + 6)) minecraft:stone"
  srv "fill $((mx - 6)) $h1 $((mz - 6)) $((mx + 6)) $h1 $((mz + 6)) minecraft:grass_block"
  srv "fill $((mx - 4)) $((h1 + 1)) $((mz - 4)) $((mx + 4)) $((h2 - 1)) $((mz + 4)) minecraft:stone"
  srv "fill $((mx - 4)) $h2 $((mz - 4)) $((mx + 4)) $h2 $((mz + 4)) minecraft:grass_block"
  srv "fill $((mx - 2)) $((h2 + 1)) $((mz - 2)) $((mx + 2)) $h3 $((mz + 2)) minecraft:stone"
  sleep 2
  echo "mountain $lb placed at $mx,$mz (tops $h1/$h2/$h3)"
}
for spec in "P1 $((CX - 4)) $((CZ - 20)) 150 175 185" \
            "P2 $((CX - 2)) $((CZ + 16)) 155 180 190"; do
  read -r MLB MMX MMZ MH1 MH2 MH3 <<< "$spec"
  build_mountain "$MMX" "$MMZ" "$MH1" "$MH2" "$MH3" "$MLB"
done
srv "forceload add $((CX - 48)) $((CZ - 48)) $((CX + 48)) $((CZ + 48))"
sleep 2
srv "fill $((CX - 2)) 60 $((CZ - 2)) $((CX + 2)) 170 $((CZ + 2)) minecraft:red_concrete"
sleep 2
STRUCT_NL=$(grep -c "That position is not loaded" /tmp/server.log 2>/dev/null || true)
t "shared structures built (not-loaded hits: $STRUCT_NL)"
echo "shared structures built (not-loaded hits: ${STRUCT_NL:-?})"
srv "time set noon"

# 4. let Voxy scan+mesh the loaded chunks (WorldImporter). The first boot also
#    pays the Knot runtime remap. Meshing budget scales with the runner.
echo "waiting for voxy meshing..."
t "meshing wait ${MESH_WAIT}s"
sleep "$MESH_WAIT"
t "meshing wait done"
grep -iE "voxy" /tmp/voxy_client.log | tail -6 || true

SHOT_N=0
shot() { # name yaw pitch x y z
  # v6.16.5: pin the light before EVERY shot — run #278 proved the 26.3
  # clock drifts through doDaylightCycle false (the leg set noon once at
  # 18:40:15 and by shot 09, 6.6 min later, the world was at night: the
  # parity matrix compared voxy night against our noon on the LATE rows).
  # Our own leg re-issues 'time set noon' before every phase for exactly
  # this reason (the v6.4.8 lesson); the reference leg now does too.
  srv "time set noon"
  srv "tp Stepan $4 $5 $6 $2 $3"
  # v6.18 SKY DETERMINISM: the SAME time-set->shot window on both legs —
  # run #285's 07_far_north missed the 100% gate by 3 px (+-1 in the sky
  # gradient) when the legs waited 10 s vs 3 s after the noon pin, and the
  # 26.3 clock drifts through doDaylightCycle false (v6.16.5), so the
  # fractional day differed. Same window on both legs = the same fractional
  # day = the same gradient. v6.25.5: the window is now 30 s on BOTH legs —
  # the race guard from the #325-#328 bimodal-flap census (a 10 s settle
  # races the paint conveyor: 06/08/09/10 flap between narrow modes).
  sleep "${PARITY_SETTLE:-30}"
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
