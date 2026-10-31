#!/usr/bin/env bash
# v6.4 CI VISUAL TEST (layer 8): the live-edit proof, rendered by a real
# vanilla 26.3 client under Xvfb + llvmpipe on the GitHub runner.
#
# What it proves (the v6.2/v6.3 guarantee, ON SCREEN):
#   leg 1 — a block placed while a client is watching reaches the LOD paint
#           WITHOUT any relog: the baseline shot vs after-edit shots differ
#           and carry RED_CONCRETE pixels ~200 blocks away, where the vanilla
#           client has NO chunks (server view-distance 4 = 64 blocks) — so
#           only the dust->post chain can have painted them;
#   leg 2 — after a hard kick + rejoin the marker STILL paints: the streamed
#           disk state survives the session (persistence/determinism).
#
# Requires (the CI job assembles all of this):
#   - dist root at $MC_TEST_DIST (Paper + launcher + runtime + module),
#   - client runtime at ~/mc-test/client (setup_client.py ran),
#   - the sRGB shim at ~/mc-test/shim/libGL.so.1,
#   - $CLOD_JAVA pointing at a JDK 25 java binary.
# Env knobs: CLOD_CENTER ("x,z", default -930,796), MIN_PROBE (150),
#            MIN_CHANGE (300), SHOTS (~/mc-test/shots/ci-visual).
#
# Probe color math (scripts/bench/visual_diff.py): fog at 200 m mixes the
# LOD color with the noon sky, so the on-screen probe uses r >= 110 and
# r-g >= 30 — fogged RED_CONCRETE (142,33,33) clears that at any fog
# fraction <= 0.5 while fogged grass/dirt/terracotta never does.
set -uo pipefail
BENCH_DIR="$(cd "$(dirname "$0")" && pwd)"
JAVA="${CLOD_JAVA:?set CLOD_JAVA to the JDK 25 java binary}"
D="${MC_TEST_DIST:-$HOME/mc-test/dist}"
SHOTS="${SHOTS:-$HOME/mc-test/shots/ci-visual}"
CENTER="${CLOD_CENTER:--930,796}"
CX="${CENTER%,*}"; CZ="${CENTER#*,}"
MARKER="red_concrete"
MIN_PROBE="${MIN_PROBE:-150}"
MIN_CHANGE="${MIN_CHANGE:-300}"
# the marker column: 5x5 footprint, y 100..200 — the top surface of those
# columns becomes the marker no matter what the terrain height was
FILL_Y0=100; FILL_Y1=200
# vantage: 100 blocks west of the marker, looking east (Bukkit yaw 270).
# THE GEOMETRY CONSTRAINTS (v6.4.1 fix — the first run stood at 200 m and
# the marker never painted): the vanilla client culls particles beyond
# ~renderDistance*16 blocks (options.txt renderDistance:8 -> 128 blocks),
# so the dust carrying the marker bytes must spawn WITHIN that radius;
# the server view-distance 4 (64 blocks) still keeps the vantage outside
# the vanilla chunk radius — the marker zone stays pure-LOD territory.
VX=$(( CX - 100 )); VY=150; VYAW=270; VPITCH=5
mkdir -p "$SHOTS"
FAILURES=0
FOUND1=0; FOUND2=0
# self-carrying diagnostics: the runner's step log is auth-walled for the
# anonymous forensics this job exists for, but the STEP SUMMARY renders
# publicly — dump the whole decision timeline there, always.
SUMMARY="${GITHUB_STEP_SUMMARY:-/tmp/step_summary.md}"
touch "$SUMMARY"
sum() { echo "$*" >> "$SUMMARY"; }

fail() { echo "FAIL: $*"; FAILURES=$((FAILURES + 1)); sum "- **FAIL:** $*"; }
note() { echo "--- $*"; }

srv() { echo "$1" > /tmp/server.in; }

wait_log() { # pattern file timeout_loops
  for _ in $(seq 1 "$3"); do
    grep -q "$1" "$2" 2>/dev/null && return 0
    sleep 2
  done
  return 1
}

join_count() { # grep -c prints 0 itself when nothing matches; never append
  grep -c "Stepan joined the game" /tmp/server.log 2>/dev/null || true
  }

shot() { # name
  ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 \
    -i :99 -frames:v 1 "$SHOTS/$1.png" 2>>"$SHOTS/ffmpeg.err" \
    && note "shot $1" || { fail "screenshot $1 failed"; return 1; }
}

paint_px() { # a.png b.png -> changed pixel count (2>/dev/null: assertion FAILs are noise here)
  python3 "$BENCH_DIR/visual_diff.py" "$1" "$2" --min-change 2000000000 2>/dev/null \
    | grep -o 'paint px = [0-9]*' | grep -o '[0-9]*$' || echo 999999
}

accept_pack_prompt() { # tag loops — wait for the yellow prompt, click Proceed
  local tag="$1" loops="$2"
  for _ in $(seq 1 "$loops"); do
    ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 \
      -i :99 -frames:v 1 /tmp/prompt_probe.png 2>/dev/null
    if python3 -c "
from PIL import Image
im = Image.open('/tmp/prompt_probe.png').convert('RGB')
cnt = sum(1 for y in range(330, 365) for x in range(380, 900)
          if (lambda p: p[0] > 220 and p[1] > 220 and p[2] < 120)(im.getpixel((x, y))))
print(1 if cnt > 30 else 0)
" | grep -q "^1$"; then
      python3 "$BENCH_DIR/click.py" 561 385
      note "$tag: pack prompt clicked"
      return 0
    fi
    sleep 2
  done
  note "$tag: no pack prompt rendered in time (cached pack auto-accept?)"
  return 0
}

stop_stack() {
  pkill -f "net.minecraft.client.main.Main" 2>/dev/null || true
  pkill Xvfb 2>/dev/null || true
  for pid in $(pgrep -f 'paper-26.3' 2>/dev/null); do kill -TERM "$pid" 2>/dev/null || true; done
  for _ in $(seq 1 30); do
    pgrep -f 'paper-26.3' >/dev/null 2>&1 || break
    sleep 1
  done
  pkill -9 -f 'paper-26.3' 2>/dev/null || true
  pkill -9 -f 'launcher.jar' 2>/dev/null || true
}

# runs on EVERY exit path (set -u failures, early exits, verdicts): the
# anonymous-visible decision timeline — this is the whole point of layer 8
# forensics: a red X with no logs is worthless.
dump_summary() {
  local rc=$?
  sum ""
  sum "### boot / stream"
  sum '```'
  grep -E "pack finalized|sky-grid stream armed|http gate|G[0-9]+ ok|gates" /tmp/server.log 2>/dev/null | head -8 >> "$SUMMARY" || true
  grep -oE "cells=[0-9]+/[0-9]+" /tmp/server.log 2>/dev/null | tail -1 >> "$SUMMARY" || true
  grep -E "edits \+[0-9]+p/[0-9]+c/[0-9]+e" /tmp/server.log 2>/dev/null | tail -2 >> "$SUMMARY" || true
  sum '```'
  sum "### sessions"
  sum '```'
  grep -E "Stepan (joined|left)|Accepted|rejected" /tmp/server.log 2>/dev/null | head -10 >> "$SUMMARY" || true
  sum '```'
  sum "### client log tail"
  sum '```'
  tail -30 /tmp/client.log 2>/dev/null >> "$SUMMARY" || echo "(no client log)" >> "$SUMMARY"
  sum '```'
  sum "### server log tail"
  sum '```'
  tail -25 /tmp/server.log 2>/dev/null >> "$SUMMARY" || true
  sum '```'
  sum "### shots"
  sum '```'
  ls -la "$SHOTS" 2>/dev/null | tail -25 >> "$SUMMARY" || true
  sum '```'
  sum ""
  sum "**exit=$rc** failures=$FAILURES leg1=$FOUND1 leg2=$FOUND2"
}
trap 'rc=$?; stop_stack; dump_summary' EXIT

# ---- 0. sanity -------------------------------------------------------------
for f in "$D/launcher.jar" "$HOME/mc-test/shim/libGL.so.1" \
         "$HOME/mc-test/client/cp.txt" "$JAVA"; do
  [ -e "$f" ] || { echo "FAIL: missing $f"; exit 2; }
done

# ---- 1. boot the stack (fifo console, gates) --------------------------------
sum "### c-LoD visual run ($CENTER, marker=$MARKER, vantage $VX,$VY yaw $VYAW)"
CLOD_CENTER="$CENTER" bash "$BENCH_DIR/boot_server.sh" || {
  echo "FAIL: boot/gates"; tail -40 /tmp/server.log; exit 1; }
sum "- boot + gates OK ($(date -u +%H:%M:%S))"
wait_log "sky-grid stream armed" /tmp/server.log 60 || {
  echo "FAIL: sky-grid never armed"; tail -40 /tmp/server.log; exit 1; }
sum "- sky-grid armed"

# ---- 2. client #1: join, accept pack ----------------------------------------
JOIN1=$(join_count)
bash "$BENCH_DIR/run_client.sh" || fail "run_client.sh exited nonzero"
wait_log "Stepan joined the game" /tmp/server.log 240 || {
  echo "FAIL: client never joined"; tail -30 /tmp/client.log; exit 1; }
note "client joined (leg 1)"
sum "- joined session 1 ($(date -u +%H:%M:%S))"
sleep 5
accept_pack_prompt leg1 60
sleep 15

# ---- 3. deterministic world state + stream liveness -------------------------
srv "gamerule doDaylightCycle false"
srv "time set 6000"
srv "weather clear"
srv "gamemode spectator Stepan"
CELLS=0
for _ in $(seq 1 300); do
  CELLS=$(curl -s http://127.0.0.1:8199/clod-pack.info \
          | grep -o 'cells=[0-9]*' | head -1 | cut -d= -f2)
  CELLS="${CELLS:-0}"
  [ "$CELLS" -ge 2000 ] 2>/dev/null && break
  sleep 1
done
[ "$CELLS" -ge 2000 ] 2>/dev/null || { echo "FAIL: scan never populated (cells=$CELLS)"; exit 1; }
note "stream live: cells=$CELLS (the fill below re-streams its columns via the event push, so full scan completion is not required)"
sum "- stream live: cells=$CELLS"

# ---- 4. stable baseline (the scan keeps painting late cells; retry) ---------
DRIFT=999999
for t in 1 2 3; do
  srv "tp Stepan $VX $VY $CZ $VYAW $VPITCH"
  sleep 6
  shot 10_before_edit || exit 1
  sleep 6
  shot 11_baseline_settle || true
  [ -f "$SHOTS/11_baseline_settle.png" ] || { fail "settle shot missing"; exit 1; }
  DRIFT=$(paint_px "$SHOTS/10_before_edit.png" "$SHOTS/11_baseline_settle.png")
  note "baseline stability try $t: drift=$DRIFT px"
  sum "- baseline try $t: drift=$DRIFT px"
  [ "$DRIFT" -le 1500 ] 2>/dev/null && break
done
[ "$DRIFT" -le 1500 ] 2>/dev/null \
  || fail "no stable baseline after 3 tries (drift=$DRIFT) — the A/B would lie"

# ---- 5. THE LIVE EDIT (no relog!) + probe loop -------------------------------
note "placing $MARKER column at $CX,$CZ (y $FILL_Y0..$FILL_Y1)"
srv "fill $((CX - 2)) $FILL_Y0 $((CZ - 2)) $((CX + 2)) $FILL_Y1 $((CZ + 2)) minecraft:$MARKER"
sleep 2
grep -E "Changed [0-9]+ blocks" /tmp/server.log | tail -1 || note "fill summary not in log (probe loop decides)"
sum "- fill issued: 5x5x$((FILL_Y1 - FILL_Y0 + 1)) $MARKER at $CX,$CZ"
PROBE_LOG="$SHOTS/probe_counts.txt"; : > "$PROBE_LOG"
for k in $(seq 0 23); do
  shot "12_after_edit_probe$k" || exit 1
  python3 "$BENCH_DIR/visual_diff.py" \
       "$SHOTS/10_before_edit.png" "$SHOTS/12_after_edit_probe$k.png" \
       --probe red --min-probe "$MIN_PROBE" --min-change "$MIN_CHANGE" \
       --out "$SHOTS/diff_live_edit.png" > /tmp/probe_out.txt 2>&1
  PC=$(grep -o 'probe\[red\] px = [0-9]*' /tmp/probe_out.txt | grep -o '[0-9]*$')
  PS=$(grep -o 'paint px = [0-9]*' /tmp/probe_out.txt | grep -o '[0-9]*$')
  echo "probe$k red=${PC:-?} paint=${PS:-?}" >> "$PROBE_LOG"
  sum "- probe $k: red=${PC:-?} paint=${PS:-?} (need red>=$MIN_PROBE paint>=$MIN_CHANGE)"
  if grep -q "VISUAL OK" /tmp/probe_out.txt; then
    FOUND1=1
    note "marker VISIBLE on screen after edit (probe $k, ~$((k * 5 + 5)) s)"
    break
  fi
  sleep 5
done
NOW_JOIN=$(join_count)
[ "$NOW_JOIN" = "$JOIN1" ] \
  || fail "join count changed mid-test ($JOIN1 -> $NOW_JOIN): the proof must hold WITHOUT a relog"
[ "$FOUND1" = "1" ] && [ "$NOW_JOIN" = "$JOIN1" ] \
  && echo "LEG 1 PASS: the edit reached the LOD paint with zero relogs" \
  && sum "- **LEG 1 PASS** (zero relogs)"
grep -E "edits \+[0-9]+p/[0-9]+c/[0-9]+e" /tmp/server.log | tail -2 || true

# ---- 6. leg 2: kick + rejoin, the marker must persist ------------------------
srv "kick Stepan"
wait_log "Stepan left the game" /tmp/server.log 30 \
  || pkill -f "net.minecraft.client.main.Main" 2>/dev/null || true
sleep 3
JOIN2=$(join_count)
bash "$BENCH_DIR/run_client.sh" || fail "run_client.sh (rejoin) exited nonzero"
REJOINED=0
for _ in $(seq 1 240); do
  [ "$(join_count)" -gt "$JOIN2" ] 2>/dev/null && { REJOINED=1; break; }
  sleep 2
done
[ "$REJOINED" = "1" ] || { echo "FAIL: rejoin never happened"; tail -30 /tmp/client.log; exit 1; }
note "client rejoined (leg 2)"
sum "- joined session 2 ($(date -u +%H:%M:%S))"
sleep 5
accept_pack_prompt leg2 30
sleep 15
srv "time set 6000"
srv "gamemode spectator Stepan"
srv "tp Stepan $VX $VY $CZ $VYAW $VPITCH"
sleep 30   # fresh session: the dust must re-deliver the persisted disk
for k in $(seq 0 11); do
  shot "13_after_relog_probe$k" || exit 1
  # leg 2 asserts the MARKER COLOR on screen (probe only): scene drift between
  # sessions is allowed to move non-marker pixels
  python3 "$BENCH_DIR/visual_diff.py" \
       "$SHOTS/10_before_edit.png" "$SHOTS/13_after_relog_probe$k.png" \
       --probe red --min-probe "$MIN_PROBE" --min-change 0 \
       --out "$SHOTS/diff_after_relog.png" > /tmp/probe_out.txt 2>&1
  PC=$(grep -o 'probe\[red\] px = [0-9]*' /tmp/probe_out.txt | grep -o '[0-9]*$')
  echo "relog_probe$k red=${PC:-?}" >> "$PROBE_LOG"
  sum "- relog probe $k: red=${PC:-?} (need >= $MIN_PROBE)"
  if grep -q "VISUAL OK" /tmp/probe_out.txt; then
    FOUND2=1
    note "marker STILL visible after relog (probe $k)"
    break
  fi
  sleep 5
done
[ "$FOUND2" = "1" ] && echo "LEG 2 PASS: the streamed marker survives the session change" \
  && sum "- **LEG 2 PASS** (marker persisted over relog)" \
  || fail "marker lost after relog — stream state is not persistent/deterministic"

# ---- 7. verdict ---------------------------------------------------------------
echo "=== client log pack markers ==="
grep -iE "pack|post|reload" /tmp/client.log | tail -8 || true
echo "=== server stream counters ==="
grep -E "stream|cells|laps|edits" /tmp/server.log | tail -8 || true
if [ "$FAILURES" = "0" ] && [ "$FOUND1" = "1" ] && [ "$FOUND2" = "1" ]; then
  echo "CI VISUAL: ALL LEGS PASS"
  exit 0
fi
echo "CI VISUAL: FAILED (failures=$FAILURES leg1=$FOUND1 leg2=$FOUND2)"
exit 1
