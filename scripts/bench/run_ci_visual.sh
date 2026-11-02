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
# the marker column: 5x5 footprint, y 60..170 — the top surface of those
# columns becomes the marker no matter what the terrain height was (the
# height byte is surface_y + 64, so the cap paints a dome at world y=170).
FILL_Y0=60; FILL_Y1=170
# vantage: 75 blocks west of the marker, looking east (Bukkit yaw 270).
# THE GEOMETRY CONSTRAINTS (v6.4.2 — run 2026-10-31 stood at 100 m and the
# marker never decoded): the SERVER decode band requires BOTH plane probes
# (y=120 and y=80) inside [30, 110] 3D of the eye — from an eye at y=151.6
# the LO plane (dy=-71.6) caps the horizontal distance at ~83 blocks, so a
# 100 m vantage left every marker cell outside the band (cone 0 zone 0,
# spawned 0, wire 0p the whole run). The constraints chain:
#   - server decode band (LO plane): horizontal <= ~83 blocks;
#   - near gate: >= 30 blocks (CULL_RADIUS+2);
#   - vanilla purity: > 64 blocks (server view-distance 4) so the marker
#     zone stays pure-LOD territory;
#   - client particle cull: < 128 blocks (renderDistance 8).
# 75 blocks satisfies all four; 100 failed the first one.
VX=$(( CX - 75 )); VY=150; VYAW=270; VPITCH=5
mkdir -p "$SHOTS"
FAILURES=0
FOUND1=0; FOUND2=0
LENS_ZERO_NOTED=0   # v6.7.3: say "dead pipeline" ONCE, not 24 times
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

# v6.4.6: the shots are only evidence while the client is IN the game. The
# 2026-10-31 run b336f57 disproved both green legs: the client disconnected
# 2-19 s after the pack push (INFO client log does not say why — the
# renderer logger now runs at DEBUG), the screen fell back to the main
# menu, and the AUTUMN-PANORAMA red foliage fed the red detector 63680 px
# (threshold 150) — both legs "passed" on the menu. joins > lefts in
# server.log is the honest in-game bit; assert it around every shot.
left_count() {
  grep -c "Stepan left the game" /tmp/server.log 2>/dev/null || true
  }

online() {
  [ "$(join_count)" -gt "$(left_count)" ] 2>/dev/null
  }

assert_online() { # tag — fail fast once the client is out of the game
  if ! online; then
    fail "$tag: client left the game mid-test (joins=$(join_count) lefts=$(left_count)) — the shot is the main-menu panorama, not the LOD proof"
    return 1
  fi
  return 0
}

shot() { # name
  ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 \
    -i :99 -frames:v 1 "$SHOTS/$1.png" 2>>"$SHOTS/ffmpeg.err" \
    && note "shot $1" || { fail "screenshot $1 failed"; return 1; }
}

# v6.7 CAPTURE LENS forensics: crop the right strip (the lens panels) from a
# screenshot and count the CLIENT-DECODED marker colors in it — the client's
# belief, immune to in-world fog/angle. Signatures (debug_lens.fsh):
#   lens_red   ~ (102,0,0)   = color lattice idx 2 (RED_CONCRETE) COMMITTED
#   lens_black ~ (0,0,0)     = idx 0 committed (the lost-single-dot signature)
#   lens_water ~ (70,130,240)= idx 216 water committed
#   lens_dark  ~ (31,31,31)  = never-marked cells (no word ever committed)
lens_metrics() { # shot_name -> prints lens_red=N lens_black=N lens_water=N lens_dark=N lens_zero=B, saves crop
  # v6.7.3 ALL-ZERO CLASSIFIER (run 41396294510 lesson): lens_black over BOTH
  # panels (>= 90% each) means the capture chain never committed a single
  # cell this session — a dead pipeline, NOT the one-lost-dot signature
  # (that one is cell-LOCAL: marker cells black while neighbours decode).
  # lens_zero=1 lets the verdict say "probes measure a dead pipeline".
  python3 - "$SHOTS/$1.png" "$SHOTS/lens_$1.png" <<'PYEOF' 2>/dev/null || { echo "lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=?"; return 0; }
import sys
from PIL import Image
im = Image.open(sys.argv[1]).convert('RGB')
# Panel geometry (debug_lens.fsh, 1280x720): 320x320 px each on the right
# edge, GL y is bottom-up -> screenshot rows are flipped. Count ONLY inside
# the two panel rects — scene pixels in the surrounding strip must never
# pollute the signatures (a night sky is exactly lens_black-black).
PX0, PW = 952, 320
COLOR_ROWS = range(392, 712)   # GL y 8..328  (panel A: color)
HEIGHT_ROWS = range(64, 384)   # GL y 336..656 (panel B: height)
crop = im.crop((940, 50, 1280, 720))
crop.save(sys.argv[2])
px = im.load()
red = black = water = dark = 0
black_c = black_h = 0
for y in list(COLOR_ROWS) + list(HEIGHT_ROWS):
    in_color = y in COLOR_ROWS
    for x in range(PX0 + 1, PX0 + PW - 1):
        r, g, b = px[x, y]
        if 85 <= r <= 120 and g <= 25 and b <= 25:
            red += 1
        elif r <= 8 and g <= 8 and b <= 8:
            black += 1
            if in_color:
                black_c += 1
            else:
                black_h += 1
        elif 55 <= r <= 90 and 110 <= g <= 150 and 220 <= b <= 255:
            water += 1
        elif 24 <= r <= 40 and 24 <= g <= 40 and 24 <= b <= 40:
            dark += 1
PANEL = 320 * (PW - 2)  # rows x inner cols per panel
zero = 1 if (black_c >= 0.9 * PANEL and black_h >= 0.9 * PANEL) else 0
print(f"lens_red={red} lens_black={black} lens_water={water} lens_dark={dark} lens_zero={zero}")
PYEOF
}

paint_px() { # a.png b.png -> changed pixel count, ALWAYS a single line
  # v6.4.2 FIX: visual_diff.py asserts against --min-change and exits 1 when
  # below it; with the 2000000000 sentinel the assertion always fired, and
  # under `set -o pipefail` the `|| echo 999999` appended a SECOND line after
  # the real count — DRIFT became "0\n999999" and `[ "$DRIFT" -le 1500 ]`
  # died on the multiline string ("no stable baseline after 3 tries
  # (drift=0\n999999)"). Capture, take the last count, tolerate the exit.
  local out
  out=$(python3 "$BENCH_DIR/visual_diff.py" "$1" "$2" --min-change 2000000000 2>/dev/null \
        | grep -o 'paint px = [0-9]*' | grep -o '[0-9]*$' | tail -1) || true
  echo "${out:-999999}"
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
  # rc was captured by the EXIT trap BEFORE stop_stack ran
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
# v6.8.3: the GL stack version rides the verdict — a runner image update
# (Mesa/llvmpipe bump) is a prime suspect for a client that renders
# nothing, and the death window (v6.6.2 live -> v6.7.2 dead) contains
# NO repo change that touches the production pack.
GLINFO=$(xvfb-run -a glxinfo -B 2>/dev/null | grep -aiE "opengl version|llvmpipe|mesa " | head -4 | tr '\n' ';' || true)
sum "- GL: ${GLINFO:-unavailable (glxinfo/xvfb-run missing)}"
# v6.5.2: arm the /clod-cells forensics window on the marker center — the
# run can then compare the SERVER's belief about the marker cells (what it
# streams) with what the client painted. v6.7: also arm the CAPTURE LENS —
# the client's committed decode state rides every probe screenshot as two
# panels on the right edge (color lattice + height bytes), so a lost dot
# (the v6.6.2 fragility) is VISIBLE as lens_black instead of guessed.
export CLOD_EXTRA_OPTS="debug_cell_x=${CX};debug_cell_z=${CZ};debug_lens=true"
CLOD_CENTER="$CENTER" bash "$BENCH_DIR/boot_server.sh" || {
  echo "FAIL: boot/gates"; tail -40 /tmp/server.log; exit 1; }
sum "- boot + gates OK ($(date -u +%H:%M:%S))"
wait_log "sky-grid stream armed" /tmp/server.log 60 || {
  echo "FAIL: sky-grid never armed"; tail -40 /tmp/server.log; exit 1; }
sum "- sky-grid armed"

# ---- 2. client #1: join, accept pack ----------------------------------------
bash "$BENCH_DIR/run_client.sh" || fail "run_client.sh exited nonzero"
wait_log "Stepan joined the game" /tmp/server.log 240 || {
  echo "FAIL: client never joined"; tail -30 /tmp/client.log; exit 1; }
note "client joined (leg 1)"
sum "- joined session 1 ($(date -u +%H:%M:%S))"
# v6.4.2 FIX: the relog guard baseline must be captured AFTER the join it
# guards — captured before, it was 0 and the guard read "0 -> 1" (the join
# itself) and failed every run ("join count changed mid-test").
JOIN1=$(join_count)
sleep 5
accept_pack_prompt leg1 60
sleep 15

# ---- 3. deterministic world state + stream liveness -------------------------
# v6.4.8: 'time set 6000' put the 26.3 world in NIGHT (different tick scale —
# the v6.4.7 shots are a starfield), the dimmed carrier dots fail the
# capture strobe (pc.g > 0.40) and the decode goes blind. 'noon' is the
# vanilla literal — daytime in ANY version. Re-issued before every shot
# batch below; 20-min days drift slowly enough that a batch stays bright.
srv "gamerule doDaylightCycle false"
srv "time set noon"
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

# ---- 3b. paint-settle note (the wait itself runs after the first tp, §4) -----
# v6.4.2 FIX (run 41237303358): a fresh client receives the WHOLE disk over
# its first minutes; the baseline tries fired mid-wave and drifted ~118k px
# per 12s (the A/B would lie — "no stable baseline after 3 tries"), even
# though the wire went quiet by tick 700. The settle loop lives in §4,
# after the first tp to the vantage (at spawn the wire is quiet because
# nothing decodes — waiting there would prove nothing).

# ---- 4. baseline: tp to the vantage FIRST, then wait for the paint wave -----
# The wave starts when the camera enters the disk — the settle wait below is
# meaningless at spawn (the wire is quiet there because nothing decodes).
srv "tp Stepan $VX $VY $CZ $VYAW $VPITCH"
# v6.4.4: wire-quiet alone settled too early — the scan keeps dribbling late
# cells under the WIRE_QUIET threshold while the screen still drifts >1500 px
# between shots (run 41237303358: both legs passed, the baseline gate fired
# mid-wave). The scan's own live counters are the honest clock: hold the
# baseline until /clod-pack.info reports cells=X/Y fully painted.
SCAN_DONE=0; XY=""
for _ in $(seq 1 210); do
  XY=$(curl -s http://127.0.0.1:8199/clod-pack.info \
       | grep -o 'cells=[0-9]*/[0-9]*' | head -1)
  X="${XY#cells=}"; X="${X%/*}"; Y="${XY#*/}"
  if [ -n "$X" ] && [ -n "$Y" ] && [ "$Y" -gt 0 ] 2>/dev/null \
     && [ "$X" -ge "$Y" ] 2>/dev/null; then
    SCAN_DONE=1
    break
  fi
  sleep 2
done
note "scan complete=$SCAN_DONE (cells=${XY:-?})"
sum "- scan complete=$SCAN_DONE (cells=${XY:-?})"
LAST_WIRE=-1
SETTLED=0
for _ in $(seq 1 21); do
  W=$(grep -oE 'wire bundle [0-9]+p' /tmp/server.log 2>/dev/null | tail -1 | grep -o '[0-9]*' | tail -1)
  W="${W:-0}"
  if [ "$LAST_WIRE" -ge 0 ] && [ $((W - LAST_WIRE)) -lt "${WIRE_QUIET:-2000}" ] 2>/dev/null; then
    SETTLED=1
    break
  fi
  LAST_WIRE="$W"
  sleep 20
done
note "paint settled=$SETTLED (wire ${LAST_WIRE}p)"
sum "- paint settled=$SETTLED (wire ${LAST_WIRE}p)"

# ---- 4a. stable baseline (the scan keeps painting late cells; retry) ---------
# v6.4.4: 3 tries 12 s apart still fired mid-wave once the scan tail ran long;
# now 6 tries with a 10 s drain between them (worst case ~2 min extra).
DRIFT=999999
for t in 1 2 3 4 5 6; do
  assert_online "baseline try $t" || exit 1
  srv "time set noon"  # v6.4.8: keep the strobe bright across the tries
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
  sleep 10  # v6.4.4: let the tail of the wave drain between tries
done
[ "$DRIFT" -le 1500 ] 2>/dev/null \
  || fail "no stable baseline after 6 tries (drift=$DRIFT) — the A/B would lie"

# ---- 4b. PARTICLE RENDER PROBE (v6.4.9, separates two failure domains) ------
# Run 41256576023: the client accepted 446k level_particles packets but the
# frame had TWO greenish pixels. Day fog washes the villager green into the
# sky blue (g/b crosses over), so the capture strobe cannot tell — but FIRST
# we must know whether the 26.3/llvmpipe particle renderer draws these
# particles AT ALL. Summon two bursts from the console (no stream involved):
#   close — 20 blocks in front of the camera, well inside any fog start;
#   far   — exactly at the marker zone, where the real carriers land.
# v6.5.1: the carrier color moved from villager green to END_ROD white. The
# dust eraser's color net now fires on ANY green-excess dot (v6.5
# fog-invariant arm — run 41293241318 proved it by erasing the probe's own
# happy_villager column down to 6 px), so a green probe can no longer see
# the renderer behind the eraser. White sparks are invisible to both eraser
# arms and still answer the only question this probe asks: does the
# renderer draw console particles here at all?
note "particle render probe (console bursts)"
BURST_CLOSE_X=$((VX + 20))
srv "particle minecraft:end_rod $BURST_CLOSE_X 150 $CZ 0.2 0.2 0.2 0 400 force"
srv "particle minecraft:end_rod $((CX - 1)) 150 $CZ 0.2 0.2 0.2 0 400 force"
sleep 1
shot 09_burst_close || exit 1
BC=$(python3 - <<'PYEOF'
from PIL import Image
im = Image.open(f"{__import__('os').environ.get('SHOTS','/home/runner/mc-test/shots/ci-visual')}/09_burst_close.png").convert('RGB')
px = im.load(); n = 0
for y in range(180, 540):
    for x in range(320, 960):
        r, g, b = (c / 255.0 for c in px[x, y])
        if min(r, g, b) > 0.75 and max(r, g, b) - min(r, g, b) < 0.15:
            n += 1
print(n)
PYEOF
)
echo "- burst_close white px: ${BC:-?} (renderer alive if > 50)" | tee -a "$SUMMARY"
srv "particle minecraft:end_rod $((CX - 2)) 150 $CZ 1 1 1 0 1200 force"
sleep 1
shot 09b_burst_far || exit 1
BF=$(python3 - <<'PYEOF'
from PIL import Image
im = Image.open(f"{__import__('os').environ.get('SHOTS','/home/runner/mc-test/shots/ci-visual')}/09b_burst_far.png").convert('RGB')
px = im.load(); n = 0
for y in range(180, 540):
    for x in range(320, 960):
        r, g, b = (c / 255.0 for c in px[x, y])
        if min(r, g, b) > 0.75 and max(r, g, b) - min(r, g, b) < 0.15:
            n += 1
print(n)
PYEOF
)
echo "- burst_far white px: ${BF:-?} (renderer alive in the marker zone if > 20)" | tee -a "$SUMMARY"

# v6.7.3 RENDERER PREFLIGHT (run 41396294510 lesson): the burst gates were
# advisory text only — a dead client renderer burned the whole 12-minute
# edit/probe/relog schedule and the verdict then blamed the TRANSPORT for
# what was a dead pipeline (close=2, paint=114, lens all-zero). The close
# burst is the renderer's pulse: no close dots -> every downstream number
# is noise. Gate on CLOSE only (a weak FAR burst can still be fog/eraser
# in the marker zone — the probes decide that). One settle+retry to be
# fair to a slow llvmpipe first frame, then abort with forensics.
if ! [ "${BC:-0}" -gt 50 ] 2>/dev/null; then
  note "renderer pulse weak (close=${BC:-?}) — 20 s settle, one retry"
  sleep 20
  srv "particle minecraft:end_rod $BURST_CLOSE_X 150 $CZ 0.2 0.2 0.2 0 400 force"
  sleep 1
  shot 09c_burst_close_retry || exit 1
  BC2=$(python3 - <<'PYEOF'
from PIL import Image
im = Image.open(f"{__import__('os').environ.get('SHOTS','/home/runner/mc-test/shots/ci-visual')}/09c_burst_close_retry.png").convert('RGB')
px = im.load(); n = 0
for y in range(180, 540):
    for x in range(320, 960):
        r, g, b = (c / 255.0 for c in px[x, y])
        if min(r, g, b) > 0.75 and max(r, g, b) - min(r, g, b) < 0.15:
            n += 1
print(n)
PYEOF
)
  echo "- burst_close retry white px: ${BC2:-?} (preflight passes if > 50)" | tee -a "$SUMMARY"
  if ! [ "${BC2:-0}" -gt 50 ] 2>/dev/null; then
    # v6.8.2 PREFLIGHT DISAMBIGUATION (the 3x RENDERER DEAD lesson — runs
    # 41413233374 / 41426906229 / 41437201051): a weak burst alone is TWO
    # very different states — (a) the client truly cannot render particles
    # (the capture stayed empty the WHOLE session: lens_zero=1, the v6.7.2
    # signature), or (b) the steady-state refresh stream has saturated the
    # vanilla pool and the fresh end_rod spawns were rejected (the v6.5.3
    # measured mechanism — the renderer is ALIVE, the dome proves it; the
    # v6.7.3 gate would have aborted HEALTHY saturated runs forever). The
    # lens decides: the panels are drawn by the chain itself every frame,
    # so the burst screenshot carries the capture's belief for free.
    LZ=$(lens_metrics "09c_burst_close_retry" | grep -o 'lens_zero=[01]' | cut -d= -f2)
    echo "- preflight lens_zero=${LZ:-?} (1 = capture NEVER committed = no particle EVER rendered)" | tee -a "$SUMMARY"
    if [ "${LZ:-0}" = "1" ]; then
      fail "RENDERER DEAD: no particles at all (close=${BC:-?}, retry=${BC2:-?}) AND the capture never committed (lens_zero=1) — the client has not rendered a single carrier since boot; run INVALID, the transport verdict is N/A"
      { echo ""
        echo "### client log tail (renderer death forensics)"
        echo '```'
        tail -40 /tmp/client.log 2>/dev/null || true
        echo '```'
        echo "### client log forensics (pipeline/particle/atlas lines — v6.8.2)"
        echo '```'
        grep -av "level_particles" /tmp/client.log 2>/dev/null \
          | grep -iaE "pipeline|compil|shader|particle|atlas|stitch|exception|error|disconnect|shutdown|reload|LWJGL|OpenGL|Mesa|llvmpipe" \
          | tail -50 || true
        echo '```'
      } >> "$SUMMARY"
      cp "$SUMMARY" "$SHOTS/summary.md" 2>/dev/null || true
      exit 1
    fi
    note "burst weak (close=${BC:-?}, retry=${BC2:-?}) but the lens shows committed capture (lens_zero=0) — pool saturation, not a dead renderer; the run continues (v6.8.2 disambiguation)"
    BC=$BC2
  fi
fi

# ---- 5. THE LIVE EDIT (no relog!) + probe loop -------------------------------
note "placing $MARKER column at $CX,$CZ (y $FILL_Y0..$FILL_Y1)"
# v6.4.2 FIX (CI visual run 2026-10-31): the marker zone sits 100 blocks from
# the vantage while the server view-distance is 4 (64 blocks) — the fill hit
# "System chat: That position is not loaded", placed NOTHING, and every probe
# then failed on principle. Pin the marker chunks with forceload first.
srv "forceload add $((CX - 48)) $((CZ - 48)) $((CX + 48)) $((CZ + 48))"
sleep 3
FILL_LOG=$(grep -c "That position is not loaded" /tmp/server.log 2>/dev/null || true)
srv "fill $((CX - 2)) $FILL_Y0 $((CZ - 2)) $((CX + 2)) $FILL_Y1 $((CZ + 2)) minecraft:$MARKER"
sleep 3
FILL_LOG2=$(grep -c "That position is not loaded" /tmp/server.log 2>/dev/null || true)
if [ "${FILL_LOG2:-0}" -gt "${FILL_LOG:-0}" ] 2>/dev/null; then
  fail "the fill hit an unloaded chunk even after forceload — cannot place the marker"
else
  note "fill landed (no 'position not loaded' after forceload)"
fi
grep -iE "Changed [0-9]+ block|Filled" /tmp/server.log | tail -1 \
  || note "fill summary not in log (probe loop decides)"
sum "- fill issued: 5x$((FILL_Y1 - FILL_Y0 + 1))x5 $MARKER at $CX,$CZ (zone forceloaded ±48)"
# v6.5.2 forensics: the SERVER's belief about the marker cells right after
# the fill — heights/valid/words before any probe claims anything.
sleep 3
CELLS_DUMP=$(curl -s "http://127.0.0.1:8199/clod-cells" || true)
{ echo ""
  echo "### /clod-cells AFTER FILL (server belief)"
  echo '```'
  printf '%s\n' "$CELLS_DUMP" | head -18
  echo '```'
} >> "$SUMMARY"
echo "cells dump captured (after fill): $(printf '%s\n' "$CELLS_DUMP" | grep -c '^[0-9-]' || true) rows"
# v6.6.1 forensics: the scheduler's state RIGHT AFTER the fill — did the
# edit window arm (ewin > 0), how many cells went out via the dirty phase,
# how deep is the queue (the v6.6 run: q~12k boot backlog outvoted the
# edit and the window never armed). The line repeats after probe0 so the
# release/drain is visible on both ends.
sleep 4
echo "### stream diag after fill:" >> "$SUMMARY"
grep "stream: tick" /tmp/server.log | tail -2 >> "$SUMMARY" || true
srv "time set noon"  # v6.4.8: the probe loop must run in daylight
PROBE_LOG="$SHOTS/probe_counts.txt"; : > "$PROBE_LOG"
for k in $(seq 0 23); do
  assert_online "edit probe $k" || exit 1
  shot "12_after_edit_probe$k" || exit 1
  LM=$(lens_metrics "12_after_edit_probe$k")
  case "$LM" in *lens_zero=1*)
    if [ "$LENS_ZERO_NOTED" = "0" ]; then
      LENS_ZERO_NOTED=1
      sum "- LENS ALL-ZERO: both capture buffers read empty — the capture chain never committed this session; these probes measure a dead pipeline, not the transport (dot-loss verdict N/A)"
    fi ;;
  esac
  python3 "$BENCH_DIR/visual_diff.py" \
       "$SHOTS/10_before_edit.png" "$SHOTS/12_after_edit_probe$k.png" \
       --probe red --min-probe "$MIN_PROBE" --min-change "$MIN_CHANGE" \
       --out "$SHOTS/diff_live_edit.png" > /tmp/probe_out.txt 2>&1
  PC=$(grep -o 'probe\[red\] px = [0-9]*' /tmp/probe_out.txt | grep -o '[0-9]*$')
  PS=$(grep -o 'paint px = [0-9]*' /tmp/probe_out.txt | grep -o '[0-9]*$')
  echo "probe$k red=${PC:-?} paint=${PS:-?} $LM" >> "$PROBE_LOG"
  sum "- probe $k: red=${PC:-?} paint=${PS:-?} $LM (need red>=$MIN_PROBE paint>=$MIN_CHANGE)"
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
# v6.6.1 forensics (second beat): the scheduler state after the probe loop —
# did the hold/release window run during the probes (ewin, dirty_streams).
echo "### stream diag after probes:" >> "$SUMMARY"
grep "stream: tick" /tmp/server.log | tail -3 >> "$SUMMARY" || true
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
# v6.4.6: both 2026-10-31 sessions died 2-19 s after the pack push — the
# rejoin leg must not probe a menu, say it loudly and stop here.
assert_online "after leg-2 pack accept" || exit 1
sleep 15
assert_online "15 s after leg-2 pack accept" || exit 1
srv "time set noon"  # v6.4.8: was 'time set 6000' — night in the 26.3 scale
srv "gamemode spectator Stepan"
srv "tp Stepan $VX $VY $CZ $VYAW $VPITCH"
sleep 30   # fresh session: the dust must re-deliver the persisted disk
for k in $(seq 0 11); do
  assert_online "relog probe $k" || exit 1
  shot "13_after_relog_probe$k" || exit 1
  LM=$(lens_metrics "13_after_relog_probe$k")
  case "$LM" in *lens_zero=1*)
    if [ "$LENS_ZERO_NOTED" = "0" ]; then
      LENS_ZERO_NOTED=1
      sum "- LENS ALL-ZERO: both capture buffers read empty — the capture chain never committed this session; these probes measure a dead pipeline, not the transport (dot-loss verdict N/A)"
    fi ;;
  esac
  # leg 2 asserts the MARKER COLOR on screen (probe only): scene drift between
  # sessions is allowed to move non-marker pixels
  python3 "$BENCH_DIR/visual_diff.py" \
       "$SHOTS/10_before_edit.png" "$SHOTS/13_after_relog_probe$k.png" \
       --probe red --min-probe "$MIN_PROBE" --min-change 0 \
       --out "$SHOTS/diff_after_relog.png" > /tmp/probe_out.txt 2>&1
  PC=$(grep -o 'probe\[red\] px = [0-9]*' /tmp/probe_out.txt | grep -o '[0-9]*$')
  echo "relog_probe$k red=${PC:-?} $LM" >> "$PROBE_LOG"
  sum "- relog probe $k: red=${PC:-?} $LM (need >= $MIN_PROBE)"
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
# v6.8.2 CLIENT-LOG FORENSICS (always): the artifact endpoint is API-gated
# and the raw tail is packet spam — the verdict feed carries the client's
# own pipeline/particle/atlas words instead, healthy or not (an EMPTY
# forensics section is itself evidence: nothing complained).
{
  echo ""
  echo "### client log forensics (pipeline/particle/atlas — v6.8.2)"
  echo '```'
  grep -av "level_particles" /tmp/client.log 2>/dev/null \
    | grep -iaE "pipeline|compil|shader|particle|atlas|stitch|exception|error|disconnect|shutdown|reload|LWJGL|OpenGL|Mesa|llvmpipe" \
    | tail -50 || true
  echo '```'
} >> "$SUMMARY"
echo "=== server stream counters ==="
grep -E "stream|cells|laps|edits" /tmp/server.log | tail -8 || true
# v6.5.2 forensics: the server's belief AFTER the probe loop — if the
# marker cells read height=170/valid here while the screen stayed sky,
# the failure is on the client decode side, not the stream.
CELLS_DUMP2=$(curl -s "http://127.0.0.1:8199/clod-cells" || true)
{ echo ""
  echo "### /clod-cells AFTER PROBES (server belief)"
  echo '```'
  printf '%s\n' "$CELLS_DUMP2" | head -18
  echo '```'
} >> "$SUMMARY"
# v6.5.1: the edit-channel killer (run 41293241318) fired AFTER the boot
# gates — "listener registered" was green while every event dispatch threw
# IncompatibleClassChangeError inside ClodEdits.pushBlock (2405 times). The
# verdict sees the FULL session log, so the linkage/dispatch errors are
# gated here, not only in the boot gates.
LINK_ERR=$(grep -cE "NoSuchMethodError|NoClassDefFoundError|IncompatibleClassChangeError" /tmp/server.log || true)
DISPATCH_ERR=$(grep -c "Could not pass event" /tmp/server.log || true)
sum "- edit channel: $LINK_ERR linkage errors, $DISPATCH_ERR failed event dispatches (both must be 0)"
if [ "${LINK_ERR:-0}" -gt 0 ] || [ "${DISPATCH_ERR:-0}" -gt 0 ]; then
  fail "the edit channel threw $LINK_ERR linkage / $DISPATCH_ERR dispatch errors — the event push cannot be trusted this run"
fi
if [ "$FAILURES" = "0" ] && [ "$FOUND1" = "1" ] && [ "$FOUND2" = "1" ]; then
  echo "CI VISUAL: ALL LEGS PASS"
  # v6.6.2: the evidence rides the artifact (GITHUB_STEP_SUMMARY is not
  # downloadable via the API) — server belief, diag dumps, verdicts.
  cp "$SUMMARY" "$SHOTS/summary.md" 2>/dev/null || true
  exit 0
fi
echo "CI VISUAL: FAILED (failures=$FAILURES leg1=$FOUND1 leg2=$FOUND2)"
cp "$SUMMARY" "$SHOTS/summary.md" 2>/dev/null || true
exit 1
