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
# Env knobs: CLOD_CENTER ("x,z", default -930,796), MIN_PROBE (60),
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
MIN_PROBE="${MIN_PROBE:-60}"
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

# v6.8.6 CONTROL LEGIBILITY: the control arm's GITHUB_STEP_SUMMARY only
# lands in $SHOTS/summary.md at the END of the leg — run 41472301678 died
# somewhere in 8.5 minutes of untraced work and the feed could only say
# "no summary". Every control stage now names itself into control-steps.log
# (no-op on the main visual leg), and the whole leg tees into
# control-leg.log below — the publisher carries both.
ct() { [ "${CLOD_CONTROL:-0}" = "1" ] && echo "$(date -u +%FT%TZ) control pulse: $1" >> "$HOME/mc-test/control-steps.log" || true; }
if [ "${CLOD_CONTROL:-0}" = "1" ]; then
  exec > >(tee -a "$HOME/mc-test/control-leg.log") 2>&1
  echo "$(date -u +%FT%TZ) control pulse: leg start (stdout/stderr teed to control-leg.log)" >> "$HOME/mc-test/control-steps.log"
fi

srv() { echo "$1" > /tmp/server.in; }

# v6.11 THE STREAM-SHAPE BURST: fire the stream's EXACT packet shape —
# /particle with count=0 emits ONE ClientboundLevelParticlesPacket with
# count=0, delta 0/0/0, speed 0, force — byte-identical to what the engine
# builds PER DOT (jni.rs build_particle_packet: "count=0 -> exactly one
# particle at the point, zero drift"; the whole bit-stream rides these
# single-particle packets). Every burst cell in this script instead sends
# ONE packet with count=1200 (server/client-side expansion) — the packet
# SHAPE axis was unmeasured. A 12x10 grid of single-packet dots, 1 block
# apart around the given spot, makes the per-dot path measurable: 120
# dots x multi-px each >> the >20 threshold, no expansion involved.
stream_burst() { # type_id x y z [n]
  local t="$1" x="$2" y="$3" z="$4" n="${5:-120}" i ox oz
  for i in $(seq 0 $((n - 1))); do
    ox=$(( i % 12 )); oz=$(( i / 12 ))
    srv "particle $t $((x + ox)) $y $((z + oz)) 0 0 0 0 0 force"
  done
}

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

assert_online() { # $1 = tag — fail fast once the client is out of the game
  # v6.9.3 FIX: the function read $tag (never set — run #232 died on
  # "line 111: tag: unbound variable" the FIRST time the assert actually
  # fired, hiding the real reason the client left) under set -u.
  local tag="$1"
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
lens_metrics() { # shot_name -> prints lens_* + stage eyes, saves crop
  # v6.7.3 ALL-ZERO CLASSIFIER (run 41396294510 lesson): lens_black over BOTH
  # panels (>= 90% each) means the capture chain never committed a single
  # cell this session — a dead pipeline, NOT the one-lost-dot signature
  # (that one is cell-LOCAL: marker cells black while neighbours decode).
  # lens_zero=1 lets the verdict say "probes measure a dead pipeline".
  # v6.9.0: the panels are LOCATED via their green frames (0,153,0) instead
  # of hardcoded rows — the 640x360-window era measured the letterbox
  # (PX0=952 rows 392..712 were outside the 640x360 window at x=320,y=176),
  # so every lens_zero since v6.7.3 was a letterbox artifact. The stage-eye
  # strips (debug_lens.fsh v6.9.0) are decoded relative to the color panel's
  # top frame edge: rot (green/magenta/yellow), scan (blue value = row hits),
  # carriers (yellow value = mask grid hits).
  python3 - "$SHOTS/$1.png" "$SHOTS/lens_$1.png" <<'PYEOF' 2>/dev/null || { echo "lens_red=? lens_black=? lens_water=? lens_dark=? lens_zero=? rot=? scan=? carriers=?"; return 0; }
import sys
import numpy as np
from PIL import Image
im = Image.open(sys.argv[1]).convert('RGB')
a = np.array(im).astype(int)
crop = im.crop((940, 50, 1280, 720))
crop.save(sys.argv[2])
H, W = a.shape[:2]
# locate green frames: (0,153,0) +- tolerance
gmask = (abs(a[:, :, 0]) < 30) & (a[:, :, 1] > 130) & (a[:, :, 1] < 175) & (abs(a[:, :, 2]) < 30)
ys, xs = np.where(gmask)
if len(xs) < 100:
    print("lens_red=0 lens_black=0 lens_water=0 lens_dark=0 lens_zero=0 rot=none scan=0 carriers=0")
    raise SystemExit
# cluster rows into two frames (upper = height panel B, lower = color panel A)
mid = (ys.min() + ys.max()) // 2
def bbox(m):
    yy, xx = ys[m], xs[m]
    return int(xx.min()), int(yy.min()), int(xx.max()), int(yy.max())
up = bbox(ys <= mid)
lo = bbox(ys > mid)
x0, yA0, x1, yA1 = lo  # color panel outer rect (frame included)
PW = x1 - x0 + 1
color_rows = range(yA0 + 2, yA1 - 1)
height_rows = range(up[1] + 2, up[3] - 1)
red = black = water = dark = 0
black_c = black_h = 0
dark_c = dark_h = 0
px = im.load()
for y in list(color_rows) + list(height_rows):
    in_color = y in color_rows
    # v6.9.4: skip the stage-eyes strip band (its black/green pixels are
    # service marks, not cell signatures)
    if in_color and yA0 + 1 <= y <= yA0 + 11:
        continue
    for x in range(x0 + 1, x0 + PW - 1):
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
        elif in_color and 24 <= r <= 40 and 24 <= g <= 40 and 24 <= b <= 40:
            # color panel never-marked: (0.12,0.12,0.12) gray
            dark += 1
            dark_c += 1
        elif (not in_color) and 50 <= r <= 80 and g <= 15 and b <= 15:
            # height panel never-marked: (0.25,0.02,0.02) dark red
            dark += 1
            dark_h += 1
PANEL = 320 * (PW - 2)
# v6.9.0: "never committed" = the NEVER-MARKED colors on both panels —
# color panel gray-31, height panel dark-red (64,5,5); the old pure-black
# rule only fired because the wrong rect measured the letterbox (pure
# black); committed idx-0 cells are the black signature.
zero = 1 if (dark_c >= 0.98 * PANEL and dark_h >= 0.98 * PANEL) else 0
# ---- stage eyes v2 (v6.9.2): four 24x8 bucket-printer strips sit at GL y
# [ya+panel-10, ya+panel-2) = the rows just BELOW the color panel's top
# frame edge in screenshot space. GL row r = screenshot row (sy0 + 7 - r).
# Strips: CAM[2,26) rows: cam.x / cam.z cell-space buckets (24 each over the
# 256-cell atlas); ROT[28,52) rows: az bucket (24 over 2pi), el bucket (8
# over pi, drawn as 3-px-wide groups); HITS[54,78) brightness = strobe-fired
# cell count; MASK[80,104) brightness = eraser-mask mark count.
sy0 = yA0 + 2
LIT = (0, 220, 0)
def strip(xa, xb):
    return a[sy0:sy0 + 8, x0 + xa:x0 + xb]
def lit_col(reg, gl_row):
    # returns the lit bucket column (strip-local) or None
    ys = 7 - gl_row
    row = reg[ys]
    for i in range(reg.shape[1]):
        r, g, b = row[i]
        if abs(r - LIT[0]) < 25 and abs(g - LIT[1]) < 25 and abs(b - LIT[2]) < 25:
            return i
    return None
cam_reg = strip(2, 26)
rot_reg = strip(28, 52)
hits_reg = strip(54, 78)
mask_reg = strip(80, 104)
def bucket_val(reg, gl_row, mode):
    # mode 'direct': 24 buckets over 24 columns -> bucket = lit column
    # mode 'el':     8 buckets drawn as 3-px groups -> bucket = column // 3
    li = lit_col(reg, gl_row)
    if li is None:
        return "?"
    if mode == 'el':
        return li // 3
    return li
# atlas geometry from the harness constants (imported via env): the buckets
# are decoded to WORLD coords here by the shell wrapper (it knows origin).
cxb = bucket_val(cam_reg, 0, 'direct')
czb = bucket_val(cam_reg, 1, 'direct')
azb = bucket_val(rot_reg, 0, 'direct')
elb = bucket_val(rot_reg, 1, 'el')
# never-decoded rot prints a full yellow row 0
rr = rot_reg[7, :, 0]; rg = rot_reg[7, :, 1]
rot_state = "never" if (rr > 240).mean() > 0.8 and (rg > 240).mean() > 0.8 else "decoded"
hits = int(round(hits_reg[:, :, 0].max() / 8.0))
mask = int(round(mask_reg[:, :, 0].max() / 8.0))
print(f"lens_red={red} lens_black={black} lens_water={water} lens_dark={dark} lens_zero={zero} rot={rot_state} cam_x_b={cxb} cam_z_b={czb} az_b={azb} el_b={elb} strobe_hits={hits} mask_hits={mask}")
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
  # v6.9.0 WINDOW FIX FALLOUT: with the client window at 1280x720 the prompt
  # renders FULL-SIZE — the yellow warning sits at y~288..335 and the Proceed
  # button (the LEFT one) at x~185..630, y~420..475. The old fixed click
  # (561,385) landed ABOVE the button → the pack was never accepted and the
  # client sat on the prompt all session (run #229: the "5405 white px
  # renderer alive" was the prompt's own white text!). The click is now
  # DYNAMIC: locate the Proceed button by its white label inside the button
  # band (y 390..560, x 100..700) and click the label's center; the yellow
  # detector only gates WHEN to click.
  local tag="$1" loops="$2"
  for _ in $(seq 1 "$loops"); do
    ffmpeg -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 \
      -i :99 -frames:v 1 /tmp/prompt_probe.png 2>/dev/null
    XY=$(python3 -c "
from PIL import Image
im = Image.open('/tmp/prompt_probe.png').convert('RGB')
px = im.load()
warn = sum(1 for y in range(280, 370) for x in range(200, 1100)
           if (lambda p: p[0] > 220 and p[1] > 220 and p[2] < 120)(px[x, y]))
if warn <= 30:
    print('')  # no prompt yet
else:
    xs, ys = [], []
    for y in range(410, 480):
        for x in range(120, 700):
            p = px[x, y]
            if p[0] > 235 and p[1] > 235 and p[2] > 235:
                xs.append(x); ys.append(y)
    if xs:
        print(f'{(min(xs)+max(xs))//2} {(min(ys)+max(ys))//2}')
    else:
        print('561 447')  # documented Proceed center at 1280x720
")
    if [ -n "$XY" ]; then
      python3 "$BENCH_DIR/click.py" $XY
      note "$tag: pack prompt clicked at ($XY)"
      return 0
    fi
    sleep 2
  done
  note "$tag: no pack prompt rendered in time (cached pack auto-accept?)"
  return 0
}

# v6.8.4: the white-px counter as a function — the control arm needs the
# SAME burst counter as the pack arm (a different counter would make the
# differential verdict a comparison of rulers, not of frames).
white_count() { # shot_name -> white px in the center window
  python3 - "$1" <<'PYEOF'
import sys, os
from PIL import Image
im = Image.open(os.environ.get('SHOTS', '/home/runner/mc-test/shots/ci-visual') + '/' + sys.argv[1] + '.png').convert('RGB')
px = im.load(); n = 0
for y in range(180, 540):
    for x in range(320, 960):
        r, g, b = (c / 255.0 for c in px[x, y])
        if min(r, g, b) > 0.75 and max(r, g, b) - min(r, g, b) < 0.15:
            n += 1
print(n)
PYEOF
}

# v6.10.0: the changed-pixel classifier — counts pixels that DIFFER from a
# baseline shot (sum of channel deltas > 8) inside the scene region
# (x < 940; the lens-panel column x >= 952 burned the v6.9.14 green ruler:
# its static (0,153,0) frame scored "366 green px" in EVERY frame of
# #246/#247 while the dust burst changed exactly 0 pixels in both runs).
# Classes mirror the existing rulers in int space: white = end_rod burst,
# green = dust carrier, red = marker paint. total > 0 with all classes ~0
# means something rendered but the fog washed it — a ruler problem, not a
# carrier problem.
changed_class() { # baseline_shot target_shot -> "white=N green=N red=N total=N"
  python3 - "$1" "$2" <<'PYEOF'
import sys, os
import numpy as np
from PIL import Image
root = os.environ.get('SHOTS', '/home/runner/mc-test/shots/ci-visual')
a = np.asarray(Image.open(f"{root}/{sys.argv[1]}.png").convert('RGB'), dtype=np.int16)
b = np.asarray(Image.open(f"{root}/{sys.argv[2]}.png").convert('RGB'), dtype=np.int16)
d = np.abs(a.astype(int) - b.astype(int)).sum(axis=2) > 8
d[:, 940:] = False
r, g, bl = b[..., 0], b[..., 1], b[..., 2]
white = d & (r > 191) & (g > 191) & (bl > 191) & \
        (np.maximum(np.maximum(r, g), bl) - np.minimum(np.minimum(r, g), bl) < 38)
green = d & ((g - bl) > 30) & (g > r + 25) & (g > 115)
red   = d & (r >= 90) & ((r - g) >= 30)
print(f"white={int(white.sum())} green={int(green.sum())} red={int(red.sum())} total={int(d.sum())}")
PYEOF
}

# v6.10.0 THE SCENE RULER: non-sky pixels in the scene region (x < 940).
# Runs #241/#246/#247's pixel forensics caught the relog leg painting NO
# scene at all — every 13_after_relog frame is pure sky + the 204800-px
# lens-panel column (x >= 952), zero scene pixels — while LEG 1's frames
# from the same vantage carry the LOD-painted terrain strip (is_red
# ~1200-2100 px, pre-pin #241 included). The relog probes have been
# scoring red=0 on a client that sees NOTHING, not on one that fails to
# decode: the fresh-session capture chain never commits (the renderer-dead
# class of #227/#228, alive in leg 2), and the pack suppresses vanilla
# terrain, so no LOD paint == no world at all. Both legs run spectator
# (no HUD), so any scene px is world.
scene_count() { # shot_name -> non-sky px at x < 940
  python3 - "$1" <<'PYEOF'
import sys, os
import numpy as np
from PIL import Image
root = os.environ.get('SHOTS', '/home/runner/mc-test/shots/ci-visual')
a = np.asarray(Image.open(f"{root}/{sys.argv[1]}.png").convert('RGB'), dtype=np.int16)
r, g, b = a[..., 0], a[..., 1], a[..., 2]
sky = ((b > r) & (b >= g)) | ((r > 220) & (g > 220) & (b > 220))
ns = ~sky
ns[:, 940:] = False
print(int(ns.sum()))
PYEOF
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
  # v6.8.4: the control arm boots plain Paper — the CRUSSTY launcher is not
  # part of that path
  if [ "${CLOD_CONTROL:-0}" = "1" ] && [ "$f" = "$D/launcher.jar" ]; then continue; fi
  [ -e "$f" ] || { echo "FAIL: missing $f"; exit 2; }
done
ct "sanity ok (boot_server next)"

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
ct "boot ok"
# v6.8.4: the control server has no c-LoD module — no sky-grid line exists;
# boot_server.sh already gated the vanilla boot on Paper's own 'Done ('
if [ "${CLOD_CONTROL:-0}" = "1" ]; then
  sum "- control boot: plain Paper up (no module, no pack — boot gated on 'Done (')"
else
  wait_log "sky-grid stream armed" /tmp/server.log 60 || {
    echo "FAIL: sky-grid never armed"; tail -40 /tmp/server.log; exit 1; }
  sum "- sky-grid armed"
fi

# ---- 2. client #1: join, accept pack ----------------------------------------
bash "$BENCH_DIR/run_client.sh" || fail "run_client.sh exited nonzero"
ct "run_client launch issued"
wait_log "Stepan joined the game" /tmp/server.log 240 || {
  echo "FAIL: client never joined"; tail -30 /tmp/client.log; exit 1; }
note "client joined (leg 1)"
sum "- joined session 1 ($(date -u +%H:%M:%S))"
ct "client joined"
# v6.4.2 FIX: the relog guard baseline must be captured AFTER the join it
# guards — captured before, it was 0 and the guard read "0 -> 1" (the join
# itself) and failed every run ("join count changed mid-test").
JOIN1=$(join_count)

# ---- 2b. v6.8.4 CONTROL PATH (CLOD_CONTROL=1): the differential arm ---------
# The frozen-frame discovery (v6.8.3 screenshots, run 41464836708): the pack
# client's frame freezes mid-boot with LIVE lens panels — the suspects are
# the pack's post-chains vs the runner environment itself. This arm runs the
# SAME client against the PLAIN Paper server (boot_server.sh CLOD_CONTROL=1,
# no module/pack) and answers ONE question with the SAME console bursts:
# does the vanilla renderer draw here at all?
if [ "${CLOD_CONTROL:-0}" = "1" ]; then
  sum "- mode: CONTROL (vanilla server, no pack — differential arm)"
  ct "control branch entered"
  srv "gamerule doDaylightCycle false"
  srv "time set noon"
  srv "gamemode spectator Stepan"
  srv "tp Stepan $VX $VY $CZ $VYAW $VPITCH"
  sleep 3
  shot 10_before_edit || exit 1
  sleep 12
  shot 11_baseline_settle || exit 1
  DRIFT=$(paint_px "$SHOTS/10_before_edit.png" "$SHOTS/11_baseline_settle.png")
  echo "- baseline try 1: drift=${DRIFT:-?} px" | tee -a "$SUMMARY"
  ct "baseline drift measured (${DRIFT:-?} px)"
  if [ "${DRIFT:-9999}" -le 3 ] 2>/dev/null; then
    sum "- frame motion: drift=0 px on a PACK-FREE client — the frame freezes WITHOUT the pack too (environment suspect)"
  else
    sum "- frame motion: drift=${DRIFT} px — the pack-free client repaints (the renderer loop is alive)"
  fi
  note "control: particle render probe (console bursts)"
  ct "console bursts fire"
  srv "particle minecraft:end_rod $((VX + 20)) 150 $CZ 0.2 0.2 0.2 0 400 force"
  srv "particle minecraft:end_rod $((CX - 1)) 150 $CZ 0.2 0.2 0.2 0 400 force"
  sleep 1
  shot 09_burst_close || exit 1
  BC=$(white_count 09_burst_close)
  echo "- burst_close white px: ${BC:-?} (control passes if > 50)" | tee -a "$SUMMARY"
  if [ "${BC:-0}" -gt 50 ]; then
    sum "- **CONTROL PASS:** the pack-free vanilla client DRAWS the console bursts (close=${BC}) — the environment renders particles fine; the frozen frame on the pack runs points at the c-LoD pack chain"
  else
    sum "- **CONTROL FAIL:** the pack-free vanilla client draws NOTHING (close=${BC:-0}) — the environment kills the render by itself; the pack is exonerated"
  fi
  # v6.10.2d THE CONTROL TYPE MATRIX: the control arm has only ever fired
  # end_rod — DUST on the pack-free client is UNMEASURED, and the #248/#250
  # bytecode+log mining (instrument note 2026-11-07) exonerated the pack
  # CONTENT (leg-2's own client.log: 'Returning cached file since actual
  # hash matches requested' — the cache-hit returns the byte-identical file
  # at the identical path, and the pack is shader-only: no textures at all).
  # One green dust burst at the SAME NEAR spot, counted as changed px vs the
  # end_rod shot (successive baselines: the dying end_rod can only inflate
  # total, never green), splits the two remaining suspects IN THIS ARM:
  #   dust renders here (>20 changed-green) -> dust renders WITHOUT the
  #       pack: the break needs the pack PRESENT (the capture chains
  #       interacting with the dust pass) or the relog-session state —
  #       the leg-1 dust probe (base patch) splits those two;
  #   dust ~0 here -> dust does not render on this build/environment AT
  #       ALL (P0: migrate the carrier type; the whole bit-stream is dust).
  sleep 4
  srv "particle minecraft:dust{color:[0.0,1.0,0.0],scale:1.0} $((VX + 20)) 150 $CZ 0.2 0.2 0.2 0 400 force"
  sleep 1
  shot 09b_control_greendust || exit 1
  CDM=$(changed_class 09_burst_close 09b_control_greendust)
  CDG=$(echo "$CDM" | grep -o 'green=[0-9]*' | cut -d= -f2)
  CDT=$(echo "$CDM" | grep -o 'total=[0-9]*' | cut -d= -f2)
  sum "- control type matrix: green dust NEAR (20 m) changed-green ${CDG:-?} / total ${CDT:-?} vs the end_rod shot (> 20 = dust renders without the pack — the break needs the pack present or relog state; ~0 = dust unrenderable on the build, P0)"
  if [ "${CDG:-0}" -gt 20 ]; then
    sum "- **CONTROL DUST RENDERS** (changed-green=${CDG}): the break requires the pack's PRESENCE (capture-chain x dust-pass interaction) or relog-session state — the leg-1 dust probe names which"
  else
    sum "- **CONTROL DUST DEAD** (changed-green=${CDG:-0}): dust does not render on this build/environment at all — P0: migrate the carrier type"
  fi
  # v6.10.2f THE CONTROL CARRIER CELL (correction note 2026-11-07): the control
  # client is PACK-FREE — no capture/eraser shaders exist at all, so the
  # carrier type renders RAW here. The stream's carrier is happy_villager
  # (jni.rs: it REPLACED the v5.0 dust carrier — dust is documented degenerate
  # on the 26.3 software backends), so the control's decisive type is THIS,
  # not dust: control villager > 20 = the carrier renders without the pack;
  # ~0 = the carrier type is broken on this build/environment itself (P0 —
  # migrate the carrier; the flap note TESTS.md v6.9.14 already documents a
  # run-crossing flap for it).
  sleep 4
  srv "particle minecraft:happy_villager $((VX + 20)) 150 $CZ 0.2 0.2 0.2 0 400 force"
  sleep 1
  shot 09c_control_villager || exit 1
  CVL=$(changed_class 09b_control_greendust 09c_control_villager)
  CVG=$(echo "$CVL" | grep -o 'green=[0-9]*' | cut -d= -f2)
  CVT=$(echo "$CVL" | grep -o 'total=[0-9]*' | cut -d= -f2)
  sum "- control CARRIER (happy_villager, pack-free — no capture shaders) NEAR: changed-green ${CVG:-?} / total ${CVT:-?} vs the control dust shot (> 20 = the carrier renders raw; ~0 = the carrier type is broken on this build — P0)"
  # v6.11 THE STREAM-SHAPE CELLS, control arm (dossier 2026-11-07): the
  # pack-free client has NO capture shaders and NO eraser — the raw
  # renderability of the engine's PER-DOT packet path. end_rod first (the
  # known-alive type), then the carrier on the same shape:
  #   end_rod stream-shape > 20 (control burst alive)  -> the count=0
  #       single-dot path renders on this build — with the burst cells the
  #       whole packet-shape axis is exonerated;
  #   end_rod stream-shape ~ 0 (control burst alive)   -> THE count=0
  #       single-dot path is broken on this build (P0 — the stream dies by
  #       SHAPE on every client; the engine must batch dots into count>0
  #       packets — no carrier migration can help);
  #   carrier stream-shape ~ 0 with end_rod stream alive -> the count=0
  #       CARRIER path specifically fails (type/pool at the packet level).
  sleep 4
  stream_burst minecraft:end_rod $((VX + 20)) 150 $CZ
  sleep 1
  shot 09d_control_endrod_stream || exit 1
  CES=$(changed_class 09c_control_villager 09d_control_endrod_stream)
  CEW=$(echo "$CES" | grep -o 'white=[0-9]*' | cut -d= -f2)
  CET=$(echo "$CES" | grep -o 'total=[0-9]*' | cut -d= -f2)
  sum "- control STREAM-SHAPE end_rod (count=0 x120 NEAR — the engine's exact per-dot packet, no expansion): changed-white ${CEW:-?} / total ${CET:-?} vs the control villager shot (> 20 with the burst alive = the per-dot packet path renders, shape exonerated; ~0 with the burst alive = the count=0 path is broken on this build — P0: batch the dots)"
  sleep 4
  stream_burst minecraft:happy_villager $((VX + 20)) 150 $CZ
  sleep 1
  shot 09e_control_villager_stream || exit 1
  CVS=$(changed_class 09d_control_endrod_stream 09e_control_villager_stream)
  CVSG=$(echo "$CVS" | grep -o 'green=[0-9]*' | cut -d= -f2)
  CVST=$(echo "$CVS" | grep -o 'total=[0-9]*' | cut -d= -f2)
  sum "- control STREAM-SHAPE carrier (happy_villager count=0 x120, pack-free): changed-green ${CVSG:-?} / total ${CVST:-?} vs the end_rod stream shot (> 20 = the per-dot CARRIER path renders raw; ~0 with end_rod stream alive = the per-dot carrier path specifically fails — type/pool at the packet level)"
  cp "$SUMMARY" "$SHOTS/summary.md" 2>/dev/null || true
  exit 0
fi

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
# v6.9.0 PROMPT GUARD (run #229 lesson): the prompt screen's own white text
# (5405 px) passes the "renderer alive" gate while the client never accepted
# the pack — the run then burns 12 minutes measuring a menu. If the pack
# prompt is still on screen HERE, fail immediately with the real cause.
if python3 -c "
from PIL import Image
import os
im = Image.open(f\"{os.environ.get('SHOTS','/home/runner/mc-test/shots/ci-visual')}/09_burst_close.png\").convert('RGB')
px = im.load()
warn = sum(1 for y in range(280, 370) for x in range(200, 1100)
           if (lambda p: p[0] > 220 and p[1] > 220 and p[2] < 120)(px[x, y]))
print(1 if warn > 30 else 0)" | grep -q "^1$"; then
  fail "PACK NEVER ACCEPTED: the resource-pack prompt is still on screen at the burst probe — the click missed the Proceed button (or the prompt re-appeared); every downstream number is a menu measurement; run INVALID"
  cp "$SUMMARY" "$SHOTS/summary.md" 2>/dev/null || true
  exit 1
fi
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
    # v6.9.2 stage eyes ride the summary on the DEAD path: the bucket printers
    # name the blind stage. Bucket -> world: origin = center - disk/2, cell 2
    # (the harness constants), 24 buckets over 256 cells.
    EYES=$(lens_metrics "09c_burst_close_retry" | grep -o 'rot=[a-z]* cam_x_b=[0-9?]* cam_z_b=[0-9?]* az_b=[0-9?]* el_b=[0-9?]* strobe_hits=[0-9]* mask_hits=[0-9]*')
    echo "- preflight lens_zero=${LZ:-?} (1 = capture NEVER committed = no particle EVER rendered)" | tee -a "$SUMMARY"
    echo "- preflight stage eyes: ${EYES:-?}" | tee -a "$SUMMARY"
    if [ "${LZ:-0}" = "1" ]; then
      # expected: cam bucket ~8 (vantage 90.5 cells from origin), az ~0 (east),
      # el ~3 ((-5deg+90)/180*8 = 3.8)
      REASON="all eyes live but zero commits — the capture merge path itself"
      case "$EYES" in
        *rot=never*) REASON="camera payload never decoded (rot dead — no sky rows survive)" ;;
        *cam_x_b=?)  REASON="CAM strip unreadable (the lens strips did not render — chain partial?)" ;;
        *cam_x_b=0*|*cam_x_b=1*|*cam_x_b=2*|*cam_x_b=3*|*cam_x_b=4*|*cam_x_b=5*|*cam_x_b=6*|*cam_x_b=23*)
          REASON="GLOBALS CAMERA X BUCKET WRONG (expected ~8 = the vantage): the post-chain Globals camera is not the player camera — the capture projects from the wrong eye" ;;
        *az_b=1[0-9]*|*az_b=2[0-3]*|*az_b=6*|*az_b=7*|*az_b=8*|*az_b=9*)
          REASON="ROT AZIMUTH BUCKET WRONG (expected ~0 = east): the decoded az disagrees with the true camera direction" ;;
        *strobe_hits=0*)
          case "$EYES" in
            *az_b=0*|*az_b=23*) REASON="camera buckets correct but the strobe NEVER fires — the projection sample positions miss the dots (fov/aspect/precision beyond bucket resolution)" ;;
            *) REASON="az bucket suspicious AND no strobe hits" ;;
          esac ;;
        *mask_hits=0*) REASON="strobe fires but the eraser mask is empty — the dust_mask pass reads stale/dead inputs" ;;
      esac
      fail "RENDERER DEAD: no particles at all (close=${BC:-?}, retry=${BC2:-?}) AND the capture never committed (lens_zero=1) — stage eyes say: ${REASON}; run INVALID, the transport verdict is N/A"
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

# ---- 4c. THE LEG-1 DUST PROBE (v6.10.2; the feed's decisive instrument) ------
# The bit-stream is 100% dust (word_spawns -> Spawn at the LAYER_Y planes) and
# at IDENTICAL coords/delta/force on leg-2 dust reads 0 changed px twice
# (#248 stacked delta 0 0 0 inconclusive, #250 fair delta 1 1 1) while end_rod
# at the same coords renders 318-324 (#250 FAR, twice). But the harness fires
# dust ONLY on leg-2 — the missing cell is the SAME burst on leg 1, where this
# client demonstrably renders (end_rod 543/530/407 raw, marker red 1183/1117,
# chains painted).
# The leg-1 answer splits the two remaining root-cause candidates in ONE run:
#   leg-1 green dust > 0   -> dust renders on a live client; the leg-2 zero is
#                             session-specific -> the fresh-session capture/
#                             decode path (the dust-vs-end_rod asymmetry rides
#                             the relog, e.g. the reload-time particles.png
#                             atlas rebuild vs the dust quad blit);
#   leg-1 green = 0 + white = 0 (total ~0) -> dust does not render on this
#                             client/build AT ALL (26.3/llvmpipe) -> the
#                             carrier type itself is the root cause; the
#                             stream must leave dust (P0 engine finding).
# A dedicated pre-burst baseline (09d) keeps the changed count free of the
# <=1500 px paint-wave drift the 10/11 pair tolerates; the white-dust burst
# at the proven NEAR spot (end_rod 543 raw there) splits color-specific from
# type-specific when green reads 0 (the eraser's fog-invariant arm fires on
# green excess even on leg 1 — total/white are the signal there). Successive
# baselines (09d->09e->09f) isolate each burst: a dying neighbour burst can
# only inflate `total` (its pixels revert to sky = unclassified delta), never
# the counted class.
note "leg-1 dust probe (carrier-type discriminator)"
shot 09d_leg1_predust || exit 1
srv "particle minecraft:dust{color:[0.0,1.0,0.0],scale:1.0} $((CX - 1)) 150 $CZ 1 1 1 0 600 force"
sleep 1
shot 09e_leg1_greendust_far || exit 1
L1GL=$(changed_class 09d_leg1_predust 09e_leg1_greendust_far)
L1GD=$(echo "$L1GL" | grep -o 'green=[0-9]*' | cut -d= -f2)
L1GT=$(echo "$L1GL" | grep -o 'total=[0-9]*' | cut -d= -f2)
sum "- leg-1 green dust (80 m, leg-2 geometry) changed-green ${L1GD:-?} / total ${L1GT:-?} vs predust (> 100 green = dust renders on a live client — the leg-2 zero is session-specific; green 0 with total ~0 = dust never renders here, the carrier type IS the root cause)"
sleep 2
srv "particle minecraft:dust{color:[1.0,1.0,1.0],scale:1.0} $((VX + 20)) 150 $CZ 0.2 0.2 0.2 0 400 force"
sleep 1
shot 09f_leg1_whitedust_near || exit 1
L1WL=$(changed_class 09e_leg1_greendust_far 09f_leg1_whitedust_near)
L1WC=$(echo "$L1WL" | grep -o 'white=[0-9]*' | cut -d= -f2)
L1WT=$(echo "$L1WL" | grep -o 'total=[0-9]*' | cut -d= -f2)
sum "- leg-1 white dust (20 m NEAR, proven-render spot) changed-white ${L1WC:-?} / total ${L1WT:-?} vs green-dust shot (> 20 = the dust TYPE renders; green 0 + white > 0 = color-specific break, green 0 + white 0 = dust-type break)"
# v6.10.2f THE CARRIER REFERENCE (correction note 2026-11-07): the bit-stream's
# carrier is happy_villager — NOT minecraft:dust (stream.rs: the villager-sparkle
# carrier has one hue; jni.rs: happy_villager REPLACED the v5.0 dust carrier —
# dust quads render with degenerate UVs on every 26.3 software backend, verified
# live on GL llvmpipe AND Vulkan lavapipe). Every dust cell in this run measures
# a DOCUMENTED-DEAD type; this cell measures the ACTUAL carrier on the client
# that demonstrably works — the reference twin for the leg-2 villager cell.
# 20 m NEAR is OUTSIDE the decode band [CULL_RADIUS=28 .. ZONE_FAR=112], so the
# eraser's color net cannot eat it (dust_mask.fsh repaints carrier-hue pixels
# in-band only) — the count is a true renderability datapoint.
sleep 2
srv "particle minecraft:happy_villager $((VX + 20)) 150 $CZ 0.2 0.2 0.2 0 400 force"
sleep 1
shot 09g_leg1_villager_near || exit 1
L1VL=$(changed_class 09f_leg1_whitedust_near 09g_leg1_villager_near)
L1VG=$(echo "$L1VL" | grep -o 'green=[0-9]*' | cut -d= -f2)
L1VT=$(echo "$L1VL" | grep -o 'total=[0-9]*' | cut -d= -f2)
sum "- leg-1 CARRIER (happy_villager) 20 m NEAR: changed-green ${L1VG:-?} / total ${L1VT:-?} vs white-dust shot (> 20 = the carrier renders on the healthy client — the leg-2 villager twin's reference; ~0 with end_rod alive = the carrier type itself is dead on BOTH legs — the flap at its extreme, carrier must change)"
# v6.11 leg-1 STREAM-SHAPE cells (dossier 2026-11-07) — the healthy-client
# reference twins for the leg-2 per-dot path. end_rod first: white is
# IMMUNE to the eraser's color net, so this is the clean shape read on a
# packed client. Then the carrier on the same shape: NOTE the color net
# (dust_mask.fsh v5.3.2b) erases carrier hue EVERYWHERE — its 2px
# out-of-band tier fires on ANY carrier-hue pixel, band only sets the disc
# radius (3px in-band / 2px out) — so villager cells on packed legs are
# eraser-overlaid and the leg-1 vs leg-2 TWIN COMPARISON is the signal:
#   leg-1 end_rod stream ~ 0 (leg-1 bursts alive) -> the count=0 path is
#       dead on BOTH legs — a 26.3-client single-particle bug class, P0:
#       the engine must batch dots regardless of the leg-2 anomaly;
#   leg-1 end_rod stream > 20 -> healthy reference established; leg-2's
#       twin cell now reads cleanly against it.
sleep 2
stream_burst minecraft:end_rod $((VX + 20)) 150 $CZ
sleep 1
shot 09h_leg1_endrod_stream || exit 1
L1ESL=$(changed_class 09g_leg1_villager_near 09h_leg1_endrod_stream)
L1ESW=$(echo "$L1ESL" | grep -o 'white=[0-9]*' | cut -d= -f2)
L1EST=$(echo "$L1ESL" | grep -o 'total=[0-9]*' | cut -d= -f2)
sum "- leg-1 STREAM-SHAPE end_rod (count=0 x120 NEAR — the engine's exact per-dot packet): changed-white ${L1ESW:-?} / total ${L1EST:-?} vs the villager NEAR shot (> 20 = the per-dot path renders on the healthy client — the leg-2 twin's reference; ~0 with the bursts alive = the count=0 path is dead on BOTH legs — P0: batch the dots)"
sleep 2
stream_burst minecraft:happy_villager $((VX + 20)) 150 $CZ
sleep 1
shot 09i_leg1_villager_stream || exit 1
L1VSL=$(changed_class 09h_leg1_endrod_stream 09i_leg1_villager_stream)
L1VSG=$(echo "$L1VSL" | grep -o 'green=[0-9]*' | cut -d= -f2)
L1VST=$(echo "$L1VSL" | grep -o 'total=[0-9]*' | cut -d= -f2)
sum "- leg-1 STREAM-SHAPE carrier (happy_villager count=0 x120 NEAR): changed-green ${L1VSG:-?} / total ${L1VST:-?} vs the end_rod stream shot (the healthy twin for the leg-2 per-dot carrier read: ~ 0 green = the 2px color net erases the per-dot carrier hue on a HEALTHY packed client — the expected design read; > 20 = the net misses the per-dot geometry — dot-size/pool class on the healthy leg)"

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
# v6.10.2c THE CACHED-PACK CONTROL (evidence arm, leg 1): leg 1 is the
# FIRST-DOWNLOAD arm (fresh runner, empty server-pack cache, yellow prompt,
# Proceed click). Record the pack flow lines the client actually logged so
# the leg-2 half of the differential below reads against the real leg-1
# flow, not an assumed one.
echo "- leg-1 pack flow (client.log):" >> "$PROBE_LOG"
grep -iE "download|resource.?pack|applying" /tmp/client.log 2>/dev/null | tail -8 >> "$PROBE_LOG" \
  || echo "  (no pack lines)" >> "$PROBE_LOG"
for k in $(seq 0 23); do
  assert_online "edit probe $k" || exit 1
  # v6.9.8: re-arm noon EVERY probe — the day/night boundary can cross
  # mid-loop (~5 min at 26.3's day length) and the marker's rendered hue
  # swings with the lightmap (run #238 measured dusk purple).
  srv "time set noon"
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
# v6.10.2c THE CACHED-PACK CONTROL (the decisive A/B on the log-mining
# note's leading suspect): leg-1 = first download + prompt + Proceed
# (measured); leg-2 = relog into the SAME client dir ($HOME/mc-test/client),
# the pack is already in the client's server-pack cache ('downloads', the
# DownloadQueue cacheDir in 26.3) and loads via the cached path with no
# prompt (the accept_pack_prompt leg2 note: 'cached pack auto-accept?') —
# the ONE measured client-side difference between the rendering leg and
# the dead leg. This control removes the cache while the client is DOWN,
# forcing leg 2 to re-download from the plugin HTTP server (engine.rs
# pack_url: /clod-pack.zip) and take the SAME load path leg 1 took. One
# variable flips, the type matrix already on board reads the answer:
#   leg-2 particles RENDER (white NEAR > 20, dust > 0) -> the cached-pack
#       load path IS the mechanism; the fix re-resolves the pack/particles
#       on a cached apply (or wipes + re-pushes per session);
#   leg-2 stays dead (~0, matrix unchanged vs #248/#250) -> the cache is
#       exonerated; the break survives an identical load flow and the
#       next variable up the ladder is the persistent gameDir state
#       itself (a fresh gameDir for leg 2).
PACK_CACHE="$HOME/mc-test/client/downloads"
if [ -d "$PACK_CACHE" ]; then
  NPACK=$(find "$PACK_CACHE" -type f 2>/dev/null | wc -l)
  echo "- leg-2 cached-pack control: cache inventory BEFORE removal ($NPACK files):" >> "$PROBE_LOG"
  find "$PACK_CACHE" -type f -printf '  %s %p\n' 2>/dev/null | head -8 >> "$PROBE_LOG"
  rm -rf "$PACK_CACHE"
  sum "- leg-2 CACHED-PACK CONTROL: removed the server-pack cache ($NPACK files) — leg 2 re-downloads the pack (fresh load path, same as leg 1)"
  echo "- leg-2 cached-pack control: cache removed ($NPACK files)" >> "$PROBE_LOG"
else
  sum "- leg-2 cached-pack control: cache dir absent at relog (leg 2 already loads fresh — control is a no-op this run)"
  echo "- leg-2 cached-pack control: cache dir absent" >> "$PROBE_LOG"
fi
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
# v6.10.2c (evidence arm, leg 2): after the cache wipe the client must
# RE-download before the prompt/applied state — these lines prove whether
# it did. The client.log grep runs late enough that download+apply have
# finished (or the run already failed above); the download-queue event
# log (DownloadQueue's log.json under 'downloads') records the fetch
# itself with URL + result when it exists.
echo "- leg-2 pack flow (client.log):" >> "$PROBE_LOG"
grep -iE "download|resource.?pack|applying" /tmp/client.log 2>/dev/null | tail -8 >> "$PROBE_LOG" \
  || echo "  (no pack lines)" >> "$PROBE_LOG"
DLQ=$(find "$HOME/mc-test/client/downloads" -name 'log.json' 2>/dev/null | head -1)
if [ -n "${DLQ:-}" ]; then
  echo "- leg-2 download-queue log.json ($DLQ):" >> "$PROBE_LOG"
  tail -c 1200 "$DLQ" >> "$PROBE_LOG"; echo >> "$PROBE_LOG"
fi
srv "time set noon"  # v6.4.8: was 'time set 6000' — night in the 26.3 scale
srv "gamemode spectator Stepan"
srv "tp Stepan $VX $VY $CZ $VYAW $VPITCH"
sleep 30   # fresh session: the dust must re-deliver the persisted disk
# v6.10.1 THE ARM-RACE CONTROL: run #248's forensics named the leg-2
# scene-dead ROOT CAUSE — the join-time posteffect add loses the pack
# push/reload race on a relog-into-vantage (the client resolves the clod
# chains 4-5 s BEFORE the pack lands, drops the failed adds, and 26.3
# never re-requests them on the post-reload rebuild). One deterministic
# tier flip re-arms the chains on the now pack-bearing client — the same
# remove+add the leg-1 vantage tp has been riding all along — so leg 2
# measures the TRANSPORT (its purpose), not the arm race. The engine's
# bounded re-arm (push+100t/+200t, v6.10.1) normally covers this already;
# this flip is the independent control: after it the scene MUST be alive.
# (tp target: CZ+460 leaves the 512-block disk for both the min-corner and
# the center-corner reading of the 256x256-cell origin — Veil radius.)
shot 13_leg2_scene_probe || true
SC0=$(scene_count 13_leg2_scene_probe)
if [ "${SC0:-0}" = "0" ]; then
  sum "- leg-2 scene dead at first probe (0 px): the arm race hit the relog — tier flip to re-arm the chains (out -> Veil, back -> Full)"
  echo "- leg-2 arm-race control: scene 0 at first probe, flipping tier" >> "$PROBE_LOG"
  srv "tp Stepan $VX $VY $((CZ + 460)) $VYAW $VPITCH"  # outside the disk -> Veil (terrain chain removed)
  sleep 3
  srv "tp Stepan $VX $VY $CZ $VYAW $VPITCH"            # back inside -> Full (chain re-added on the pack-bearing client)
  sleep 25   # chain rebuild + one full sky-grid lap (~12.8 s) of decode
  sum "- leg-2 tier flip done: chains re-armed post-reload; baseline follows"
fi
# v6.10.0: the leg-2 BASELINE shot — everything the leg-2 rulers count is
# CHANGED pixels against this frame, so the lens-panel column and the sky
# gradient cannot masquerade as burst/marker content (the v6.9.14 green
# ruler burned on exactly that).
shot 14_leg2_baseline || exit 1
L2_SCENE0=$(scene_count 14_leg2_baseline)
sum "- leg-2 baseline scene px (x<940): ${L2_SCENE0:-?} (0 = the relog client paints no world at all)"
echo "- leg-2 baseline scene px: ${L2_SCENE0:-?}" >> "$PROBE_LOG"
# v6.9.12/v6.9.13 LEG-2 RENDERER PROBES: run #243's forensics caught the
# leg-2 client rendering NO particles at all (189-point re-arm bursts
# arrive every 10 s; a shot taken 2 s into a burst shows ZERO non-sky
# pixels) while LEG 1 rendered the same geometry; #244's probe split the
# mystery: the console burst 20 blocks from the vantage RENDERS (543
# white px) while the re-arm's dots at 75-105 m do not. The distance is
# now the prime suspect (a vanilla particle render cut, e.g. the loaded
# client chunks at server view-distance*16 = 64 blocks). Two bursts, one
# ruler: NEAR (20 blocks, inside any cut) and FAR (80 blocks, at the
# marker zone — where the carriers land). NEAR alive + FAR dead names
# the distance cut; both alive convicts the re-arm dots themselves.
# v6.10.0: #247 broke the TYPE hypothesis — end_rod FAR flapped
# 407 -> 354 -> 0 across #245/#246/#247 while NEAR stayed alive, so the
# flap is not happy_villager-specific. And every burst since #243 ran
# under the particles:0 pin (MINIMAL): non-forced particles never spawn
# and forced bursts survive ~19% of attempts — the pin was the censor.
# Both legs now run particles:2 (ALL); raw white counts stay for series
# continuity, changed-vs-baseline counts ride next to them.
srv "particle minecraft:end_rod $((VX + 20)) 150 $CZ 0.2 0.2 0.2 0 400 force"
sleep 1
shot 14_leg2_burst_near || exit 1
L2B=$(white_count 14_leg2_burst_near)
L2BC=$(changed_class 14_leg2_baseline 14_leg2_burst_near | grep -o 'white=[0-9]*' | cut -d= -f2)
sum "- leg-2 renderer probe NEAR (20 m) white px: ${L2B:-?} raw / ${L2BC:-?} changed-vs-baseline (> 20 = the client renders particles)"
echo "- leg-2 renderer probe NEAR white px: ${L2B:-?} changed: ${L2BC:-?}" >> "$PROBE_LOG"
srv "particle minecraft:end_rod $((CX - 1)) 150 $CZ 1 1 1 0 1200 force"
sleep 1
shot 14_leg2_burst_far || exit 1
L2F=$(white_count 14_leg2_burst_far)
L2FC=$(changed_class 14_leg2_baseline 14_leg2_burst_far | grep -o 'white=[0-9]*' | cut -d= -f2)
sum "- leg-2 renderer probe FAR (80 m, marker zone) white px: ${L2F:-?} raw / ${L2FC:-?} changed-vs-baseline (> 20 = no distance cut; ~0 with NEAR alive = particles beyond the client chunk radius do not render)"
echo "- leg-2 renderer probe FAR white px: ${L2F:-?} changed: ${L2FC:-?}" >> "$PROBE_LOG"
# v6.9.14 THE CARRIER-TYPE PROBE: the transport's carrier is
# happy_villager (green); its render FLAPS across runs (the marker column
# measured 1631 / 27 / 0 / 93 / 2961 red px on identical geometry — runs
# #234-#245) while end_rod rendered in every single probe (543/530/407).
# A GREEN DUST burst at the same spot answers whether a stable green
# carrier exists: dust{color:[0,1,0]} renders a green cloud the capture's
# green-hue strobe can decode — if it proves stable across runs, the
# carrier migrates to it and the flap dies with happy_villager.
# v6.10.0: #246's in-band green ruler was a FALSE POSITIVE — it counted
# the static (0,153,0) lens-panel frame at x=952..959 (366 px in EVERY
# frame of #246/#247, pixel-identical to no-burst frames) while the dust
# burst changed exactly 0 pixels in both runs. The ruler now counts
# green-excess among pixels CHANGED vs the leg-2 baseline, x < 940.
# v6.10.1: the burst geometry was the SECOND reason it read 0 — delta
# 0 0 0 speed 0 stacks all 600 particles at ONE world point; at 80 m a
# single stacked billboard is subpixel, under the >8-delta threshold
# (the end_rod FAR burst that renders 375 px spreads over delta 1 1 1;
# #248 server.log proves the dust command parses and executes). Spread
# like FAR so the carrier question is answerable at all.
srv "particle minecraft:dust{color:[0.0,1.0,0.0],scale:1.0} $((CX - 1)) 150 $CZ 1 1 1 0 600 force"
sleep 1
shot 14_leg2_burst_greendust || exit 1
GDL=$(changed_class 14_leg2_baseline 14_leg2_burst_greendust)
GD=$(echo "$GDL" | grep -o 'green=[0-9]*' | cut -d= -f2)
GDT=$(echo "$GDL" | grep -o 'total=[0-9]*' | cut -d= -f2)
sum "- leg-2 carrier probe green dust (80 m): changed-green px ${GD:-?} / total changed ${GDT:-?} vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)"
echo "- leg-2 green dust changed-green px: ${GD:-?} total changed: ${GDT:-?}" >> "$PROBE_LOG"
# v6.10.2 THE LEG-2 TYPE MATRIX (the feed's follow-up cell, fired in-run):
# every leg-2 dust datapoint so far was FAR (80 m) — two questions remain
# that the end_rod-vs-dust pair cannot separate on leg 2:
#   white dust at the NEAR coords (20 m) — end_rod proves the spot renders
#   (495/479 changed) and fog is negligible there, so a zero here convicts
#   the dust TYPE (or its color path), not distance/fog;
#   happy_villager at the NEAR coords — the second non-dust type (the old
#   carrier): a render here with dust 0 narrows the break to dust; a zero
#   here TOO says the relog client renders end_rod and nothing else.
# Successive baselines (greendust -> whitedust -> villager) isolate each
# burst: a dying neighbour can only inflate `total`, never the counted
# class; the eraser (if the tier-flip re-armed the chains) can eat green —
# total is the signal for the villager reading.
sleep 2
srv "particle minecraft:dust{color:[1.0,1.0,1.0],scale:1.0} $((VX + 20)) 150 $CZ 0.2 0.2 0.2 0 400 force"
sleep 1
shot 14_leg2_burst_whitedust || exit 1
L2WDL=$(changed_class 14_leg2_burst_greendust 14_leg2_burst_whitedust)
L2WDC=$(echo "$L2WDL" | grep -o 'white=[0-9]*' | cut -d= -f2)
L2WDT=$(echo "$L2WDL" | grep -o 'total=[0-9]*' | cut -d= -f2)
sum "- leg-2 type matrix white dust NEAR (20 m): changed-white ${L2WDC:-?} / total ${L2WDT:-?} vs green-dust shot (> 20 = dust renders on the relog client NEAR — the FAR zero is distance/fog; 0 with the end_rod NEAR burst alive (495/479) = the dust TYPE is unrendered on leg 2)"
echo "- leg-2 white dust NEAR changed-white: ${L2WDC:-?} total: ${L2WDT:-?}" >> "$PROBE_LOG"
sleep 2
srv "particle minecraft:happy_villager $((VX + 20)) 150 $CZ 0.2 0.2 0.2 0 400 force"
sleep 1
shot 14_leg2_burst_villager || exit 1
L2HVL=$(changed_class 14_leg2_burst_whitedust 14_leg2_burst_villager)
L2HVG=$(echo "$L2HVL" | grep -o 'green=[0-9]*' | cut -d= -f2)
L2HVT=$(echo "$L2HVL" | grep -o 'total=[0-9]*' | cut -d= -f2)
sum "- leg-2 type matrix happy_villager NEAR (20 m): changed-green ${L2HVG:-?} / total ${L2HVT:-?} vs white-dust shot (the old carrier on the relog client; total is the signal — the eraser can eat green)"
echo "- leg-2 happy_villager NEAR changed-green: ${L2HVG:-?} total: ${L2HVT:-?}" >> "$PROBE_LOG"
# v6.10.2b THE PLANE CONTROL (log-mining addendum, 2026-11-06): the wire
# counters in #248/#250 proved the stream dots ARRIVE at the leg-2 client
# continuously (+189-378 particles per 100-tick window, fail 0, gate-passed)
# and none of them render — but end_rod has only ever been fired at Y=150
# (eye level). This burst fires end_rod at the stream's OWN plane (Y=120,
# the hword plane, marker zone, delta 1 1 1 — the exact geometry of the dots
# the client refuses to render):
#   renders (> 20) -> the planes are fine; the break is dust-TYPE-only —
#                      the carrier must change type, not altitude;
#   ~0             -> the break rides the plane/depth position TOO (the
#                      reversed-Z class from the stream doc's own caveat:
#                      below-horizon particles lose to the sky) — the
#                      carrier must change type AND possibly in Y.
sleep 2
srv "particle minecraft:end_rod $((CX - 1)) 120 $CZ 1 1 1 0 1200 force"
sleep 1
shot 14_leg2_burst_endrod_plane || exit 1
EPL=$(changed_class 14_leg2_burst_villager 14_leg2_burst_endrod_plane)
EPC=$(echo "$EPL" | grep -o 'white=[0-9]*' | cut -d= -f2)
EPT=$(echo "$EPL" | grep -o 'total=[0-9]*' | cut -d= -f2)
sum "- leg-2 plane control end_rod at Y=120 (80 m, the hword plane): changed-white ${EPC:-?} / total ${EPT:-?} vs villager shot (> 20 = the planes render — the break is dust-type-only; ~0 with end_rod@150 alive (495/479) = the break rides the plane/depth too)"
echo "- leg-2 end_rod@Y120 changed-white: ${EPC:-?} total: ${EPT:-?}" >> "$PROBE_LOG"
# v6.10.2e THE PLANE x TYPE CROSS (feed addendum, 2026-11-07): the type matrix
# fires dust at Y=150 ONLY, and the plane control fires END_ROD at Y=120 ONLY —
# the interaction cell (dust AT the stream planes) is unmeasured, while the
# engine's actual dots are dust at LAYER_Y=[80,120] (stream.rs:71: hword rides
# Y=120, cword Y=80) and the whole stream is dead on leg-2. Two bursts mirror
# the end_rod@Y120 burst EXACTLY (same 80 m spot, same delta 1 1 1, count
# 1200, force) with only the type swapped, plus the second engine plane:
#   dust@120 renders (with end_rod@120 > 20) -> the client renders dust at the
#       exact engine geometry: the dead stream points at the ENGINE side
#       (spawn/packet/dot content), not the client renderer;
#   dust@120 ~0 with end_rod@120 > 20 and dust@150 NEAR rendering -> dust is
#       PLANE-sensitive (the type x plane interaction) -> move LAYER_Y or the
#       per-plane carrier;
#   dust@120 AND dust@80 ~0 (end_rod@150 alive) -> the whole plane depth loses
#       on leg-2 (the reversed-Z class) -> the carrier must change altitude
#       AND type. White is immune to the eraser; successive baselines keep a
#       dying neighbour in `total` only (dust dies in ~1.5 s, the +3 s cadence
#       leaves it fully dead before the next baseline).
sleep 2
srv "particle minecraft:dust{color:[1.0,1.0,1.0],scale:1.0} $((CX - 1)) 120 $CZ 1 1 1 0 1200 force"
sleep 1
shot 14_leg2_burst_whitedust_plane || exit 1
L2WPL=$(changed_class 14_leg2_burst_endrod_plane 14_leg2_burst_whitedust_plane)
L2WPC=$(echo "$L2WPL" | grep -o 'white=[0-9]*' | cut -d= -f2)
L2WPT=$(echo "$L2WPL" | grep -o 'total=[0-9]*' | cut -d= -f2)
sum "- leg-2 plane x type cross: white dust at Y=120 (80 m, the hword plane — the end_rod@Y120 burst with ONLY the type swapped): changed-white ${L2WPC:-?} / total ${L2WPT:-?} vs the end_rod@Y120 shot (> 20 with end_rod@120 alive = dust renders at the exact engine geometry — the dead stream points at the engine spawn/packet path, not the client renderer; ~0 = the type x plane interaction, move LAYER_Y or the per-plane carrier)"
echo "- leg-2 white dust@Y120 changed-white: ${L2WPC:-?} total: ${L2WPT:-?}" >> "$PROBE_LOG"
sleep 2
srv "particle minecraft:dust{color:[1.0,1.0,1.0],scale:1.0} $((CX - 1)) 80 $CZ 1 1 1 0 1200 force"
sleep 1
shot 14_leg2_burst_whitedust_cword || exit 1
L2WCL=$(changed_class 14_leg2_burst_whitedust_plane 14_leg2_burst_whitedust_cword)
L2WCC=$(echo "$L2WCL" | grep -o 'white=[0-9]*' | cut -d= -f2)
L2WCT=$(echo "$L2WCL" | grep -o 'total=[0-9]*' | cut -d= -f2)
sum "- leg-2 cword plane: white dust at Y=80 (80 m, the cword plane, same burst geometry): changed-white ${L2WCC:-?} / total ${L2WCT:-?} vs the dust@Y120 shot (> 20 = the cword plane renders dust too — planes fully exonerated for dust; ~0 with dust@120 > 20 = the Y=80 plane specifically loses (deepest below-horizon) — drop or move it; both ~0 = the whole plane depth loses on leg-2, reversed-Z class)"
echo "- leg-2 white dust@Y80 changed-white: ${L2WCC:-?} total: ${L2WCT:-?}" >> "$PROBE_LOG"
# v6.10.2f THE ERASER TEST (correction note 2026-11-07): the stream's carrier
# is happy_villager (NOT dust), the dots ride the planes Y=120 (hword) and
# Y=80 (cword), and BOTH the spot (75 m) and the planes sit INSIDE the decode
# band [CULL_RADIUS=28 .. ZONE_FAR=112] where dust_mask.fsh's color net
# REPAINTS every carrier-hue pixel from its surroundings — the stealth
# eraser. On a HEALTHY capture chain this burst renders AND gets erased:
# changed-green ~ 0 BY DESIGN. So this cell does NOT measure renderability —
# it measures the ERASER TAIL of the capture pipeline on the relog client:
#   ~0 (with the NEAR villager cell alive on leg 2)  -> the eraser is ALIVE
#       on leg 2: the stream dots die BEFORE display (render-path/pool),
#       never reach capture (mask_hits=0 consistent) — dot-geometry/pool;
#   > 20 -> the eraser is BLIND on leg 2: the capture tail is down, and any
#       dot that rendered would be VISIBLE — their total absence localizes
#       the death to the particle render path itself (with end_rod@120
#       alive, the plane/depth class is exonerated for the carrier too).
sleep 2
srv "particle minecraft:happy_villager $((CX - 1)) 120 $CZ 1 1 1 0 1200 force"
sleep 1
shot 14_leg2_burst_villager_plane || exit 1
L2VPL=$(changed_class 14_leg2_burst_whitedust_cword 14_leg2_burst_villager_plane)
L2VPG=$(echo "$L2VPL" | grep -o 'green=[0-9]*' | cut -d= -f2)
L2VPT=$(echo "$L2VPL" | grep -o 'total=[0-9]*' | cut -d= -f2)
sum "- leg-2 ERASER TEST: CARRIER (happy_villager) at Y=120 in-band (75 m, the hword plane): changed-green ${L2VPG:-?} / total ${L2VPT:-?} vs the dust@Y80 shot (~ 0 = the eraser is ALIVE on leg 2 — stream dots die pre-display; > 20 = the eraser is BLIND on leg 2 — the capture tail is down)"
echo "- leg-2 carrier@Y120 in-band changed-green: ${L2VPG:-?} total: ${L2VPT:-?}" >> "$PROBE_LOG"
# v6.11 THE STREAM-SHAPE CELLS on the relog leg (dossier 2026-11-07) — the
# decisive PACKET-SHAPE split. The stream's wire payload is ONE count=0
# packet PER DOT (jni.rs build_particle_packet: overrideLimiter=true,
# alwaysShow=true, delta 0/0/0, speed 0, count=0 — sent in a
# ClientboundBundlePacket per tick); every burst cell above is ONE packet
# with count=1200. end_rod first — white is immune to the color net, so
# this is the CLEAN per-dot path read on the relog client:
#   > 20 (NEAR end_rod burst 479/495 alive) -> the count=0 single-dot
#       path RENDERS on the relog client — packet shape exonerated; the
#       stream's death must ride the type (the villager flap), the
#       in-band eraser (the cell above), the BUNDLE wrapper (engine-only,
#       no /particle equivalent) or the trickle cadence — the published
#       cells split all but the bundle, which becomes the named suspect;
#   ~ 0 (burst alive) -> THE count=0 single-dot path is DEAD on the relog
#       client: the stream's per-dot packets die BY SHAPE — the engine
#       must batch (count>0 packets or per-dot bundles of bursts); this
#       is the P0 fix row and it does not exist in any client shader.
sleep 2
stream_burst minecraft:end_rod $((VX + 20)) 150 $CZ
sleep 1
shot 14_leg2_burst_endrod_stream || exit 1
L2ESL=$(changed_class 14_leg2_burst_villager_plane 14_leg2_burst_endrod_stream)
L2ESW=$(echo "$L2ESL" | grep -o 'white=[0-9]*' | cut -d= -f2)
L2EST=$(echo "$L2ESL" | grep -o 'total=[0-9]*' | cut -d= -f2)
sum "- leg-2 STREAM-SHAPE end_rod (count=0 x120 NEAR — the engine's exact per-dot packet, eraser-immune): changed-white ${L2ESW:-?} / total ${L2EST:-?} vs the villager@Y120 shot (> 20 with the NEAR burst alive = the per-dot path renders — shape exonerated, suspects narrow to type/bundle/cadence; ~0 with the burst alive = the count=0 path is DEAD on the relog client — the stream dies by packet shape, the engine must batch dots)"
echo "- leg-2 end_rod stream-shape changed-white: ${L2ESW:-?} total: ${L2EST:-?}" >> "$PROBE_LOG"
# carrier stream-shape — the per-dot CARRIER path (the color net overlays
# this cell; the leg-1 twin 09i is the reference):
sleep 2
stream_burst minecraft:happy_villager $((VX + 20)) 150 $CZ
sleep 1
shot 14_leg2_burst_villager_stream || exit 1
L2VSL=$(changed_class 14_leg2_burst_endrod_stream 14_leg2_burst_villager_stream)
L2VSG=$(echo "$L2VSL" | grep -o 'green=[0-9]*' | cut -d= -f2)
L2VST=$(echo "$L2VSL" | grep -o 'total=[0-9]*' | cut -d= -f2)
sum "- leg-2 STREAM-SHAPE carrier (happy_villager count=0 x120 NEAR): changed-green ${L2VSG:-?} / total ${L2VST:-?} vs the end_rod stream shot (read against the leg-1 twin: leg-2 ~ leg-1 = the per-dot carrier path behaves like the healthy leg — carrier exonerated at NEAR; leg-2 green > 20 with leg-1 ~ 0 = the eraser is blind on leg 2 at per-dot geometry; leg-2 ~ 0 with end_rod stream alive = the per-dot CARRIER path specifically dies — type/pool class)"
echo "- leg-2 villager stream-shape changed-green: ${L2VSG:-?} total: ${L2VST:-?}" >> "$PROBE_LOG"
L2_SCENE_ALIVE=0
for k in $(seq 0 11); do
  assert_online "relog probe $k" || exit 1
  srv "time set noon"  # v6.9.8: keep the light flat across the window
  shot "13_after_relog_probe$k" || exit 1
  SC=$(scene_count "13_after_relog_probe$k")
  if [ "${SC:-0}" -gt 0 ] 2>/dev/null; then L2_SCENE_ALIVE=1; fi
  if [ "$k" = "0" ] && [ "${SC:-0}" = "0" ]; then
    sum "- LEG-2 SCENE DEAD: the relog client paints ZERO scene pixels (x<940) — pure sky + lens panels; the fresh-session capture chain never commits (renderer-dead class, #227/#228); these probes measure a worldless client, not the transport"
  fi
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
  echo "relog_probe$k red=${PC:-?} scene=${SC:-?} $LM" >> "$PROBE_LOG"
  sum "- relog probe $k: red=${PC:-?} scene=${SC:-?} $LM (need red >= $MIN_PROBE; scene > 0 = the client paints a world at all)"
  if grep -q "VISUAL OK" /tmp/probe_out.txt; then
    FOUND2=1
    note "marker STILL visible after relog (probe $k)"
    break
  fi
  sleep 5
done
if [ "$FOUND2" = "1" ]; then
  echo "LEG 2 PASS: the streamed marker survives the session change"
  sum "- **LEG 2 PASS** (marker persisted over relog)"
elif [ "${L2_SCENE_ALIVE:-0}" = "0" ]; then
  fail "leg-2 painted no scene in any probe — the fresh-session capture chain never commits (renderer-dead class of #227/#228); the relog verdict is N/A until the chain arms on a fresh client"
else
  fail "marker lost after relog — stream state is not persistent/deterministic"
fi

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
