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
#
# v6.15.1 REVERT NOTE (feat/visible-terrain, run #267 = 42388554905): the
# first attempt of this branch dropped the eye to y=120 to widen the band
# (LO dy=-41.6 -> cap ~102 m) and put a ground plane in frame. RESULT: the
# capture committed ZERO cells on BOTH legs (whole-disk heightfield empty,
# paint=0 on every probe, lap counter crawling 1 lap/1800 ticks, ewin
# armed and closed, rbeat 0, wire trickling 20 p/tick while the color net
# still saw dots — mask_hits=30) although decodable_by_any() passes the
# same cells from y=120 by the book. Something client-side (projection or
# strobe) does not survive the lower eye — documented as the round's open
# finding G7a; the eye returns to the PROVEN y=150 and the range is
# resized to the 83 m band instead:
#   - the range sits at 66..82 m horizontal (inside the band, beyond the
#     64 m vanilla chunk edge — pure-LOD territory);
#   - tops cap at y<=190 (height byte = surface_y+64 must stay <= 255);
#   - the COLOR words ride the LO plane (y=80), which is off-screen from
#     y=151.6 (dy=-71.6 projects below -40 deg) — the range paints through
#     the march's fallback palette (ColLow grass-green 0.35,0.52,0.28 ->
#     ColHigh stone-brown by height), the same mechanism that painted the
#     v6.14 marker column dark. Heights (HI plane, dy=-31.6) decode
#     on-screen — the terraces are real data.
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
  # -nostdin (v6.16.2): ffmpeg eats stdin — inside the parity tour's
  # matrix read-loop it mangled every shot name after the first (#275).
  ffmpeg -nostdin -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 \
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

# v6.14 LOD PAINT PROOF (the answer to «это какой-то красный столб, а не лоды»):
# the march paints the marker-column silhouette into the sky region at 75 m —
# pixels NO vanilla path can produce there (chunk edge is 64 m at server
# view-distance 4, so no chunk geometry exists behind those pixels; dust
# particles are white/green; the lens panels sit at x >= 940). Measured on
# the #255 baseline (640-space column rgb ~[51,81,71], sky ~[190,200,230]):
# the silhouette is the ONLY dark content in the column's screen band.
# The ruler scans the column's screen band (x [560,760], y [180,700],
# 1280-space) with r+g+b < 330 and counts X-COLUMNS whose dark run reaches
# 250 px — the silhouette is ~470 px tall there, while every UI band on the
# same frames (the chat-verification toast ~150 px, the chat lines ~60 px)
# stays far below the threshold. Pack leg healthy ~ 40-70 columns
# (run 42358652148 re-measures 52); ZONE_FAR=48 regression run 42364516378
# and the pack-free control: 0.
lod_silhouette() { # shot_name -> prints "lod_silhouette_cols=N"
  python3 - "$SHOTS/$1.png" <<'PYEOF' 2>/dev/null || { echo "lod_silhouette_cols=?"; return 0; }
import sys
import numpy as np
from PIL import Image
im = Image.open(sys.argv[1]).convert('RGB')
a = np.array(im).astype(int)
band = a[180:700, 560:760, :]
dark = (band.sum(axis=2) < 330)
cols = dark.sum(axis=0)
print(f"lod_silhouette_cols={int((cols >= 250).sum())}")
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
# v6.18.5 REVERT: the session-1 pack must NOT carry the un-fog engagement
# mask — run #290 proved the mask's overlays SATURATE THE TRANSPORT PROBES
# (probe 0 red=129083 = the mask's red overlay counted as marker red;
# scene=128875 = the overlays; the LEG-1/LEG-2 "PASS"es of #290 are
# threshold artifacts, not a transport recovery). The engagement evidence
# lives on the §7b mask8 client only (the parity world, vd 8).
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
  # v6.14: the CONTROL half of the paint proof — the same ruler on the same
  # vantage with NO pack and NO module: the silhouette must read ~0. Any
  # dark column here would mean a vanilla/environment source and would
  # invalidate the differential (the pack arm's paint must exceed it).
  CLODPX=$(lod_silhouette 10_before_edit)
  ct "control lod_silhouette=${CLODPX:-?} cols"
  sum "- control lod_silhouette_cols: ${CLODPX:-?} (must be 0 — no pack, no march; the differential vs the pack arm IS the paint proof)"
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

# ---- 3a2. THE TERRAIN RANGE (v6.15.1, feat/visible-terrain) ------------------
# THE USER ASK: every gallery round so far rendered ONE free-standing marker
# column in empty sky ("это какой-то красный столб, а не лоды"). The world
# around CENTER with seed clodbench2026 is flat plains — the heightfield has
# no relief to march, so the honest frame IS a lone column. This section
# gives the disk real far-terrain: two terraced mountains 66..82 m east of
# the eye, i.e. BEYOND the vanilla chunk edge (64 m) and INSIDE the decode
# band (~83 m from the y=151.6 eye) — pure-LOD territory the vanilla
# renderer can never touch. Built with the marker's own vanilla
# forceload+fill mechanism, forceloaded for the whole leg; the scanner
# streams their heights like any edit and the clod:lod_terrain march paints
# an actual LANDSCAPE (green-to-brown by the fallback palette, heights from
# streamed data).
#
# Composition from the eye at (CX-75, 151.6), yaw 270 (east), pitch 5:
#   - P1 «северная» center (CX-4, CZ-20): terraces 13/9/5, tops 150/175/185
#   - P2 «южная»    center (CX-2, CZ+16): terraces 13/9/5, tops 155/180/190
#   - grass_block tops on T1/T2, stone caps on T3 (the fallback palette
#     renders low = green, high = brown-gray regardless — the color words
#     ride the off-screen LO plane from this eye);
#   - the red marker (placed later, §5) stays free-standing IN FRONT of the
#     range in the sky gap between the peaks: the red calibration stick.
# Geometry audit (worst cells vs the band, eye y=151.62):
#   - farthest footprint corner: P2 (CX+4,CZ+22) -> dx=79, dz=22,
#     3D to LO plane sqrt(79^2+22^2+71.6^2)=105.5 <= 110 OK
#     (horizontal 82.0 <= 83.5 = sqrt(110^2-71.6^2) OK);
#   - nearest footprint edge: dx=66 >= 30 OK; all < 128 (client cull) OK;
#   - height byte = surface_y+64: max 190+64=254 <= 255 OK;
#   - every fill <= 32768 blocks OK; peaks never touch the marker 5x5 OK;
#   - silhouette ruler intact: the marker's dark column reads against the
#     sky gap between P1 and P2.
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
  note "mountain $lb placed at $mx,$mz (tops $h1/$h2/$h3)"
}
MTN_NOT_LOADED=0
for spec in "P1 $((CX - 4)) $((CZ - 20)) 150 175 185" \
            "P2 $((CX - 2)) $((CZ + 16)) 155 180 190"; do
  read -r MLB MMX MMZ MH1 MH2 MH3 <<< "$spec"
  build_mountain "$MMX" "$MMZ" "$MH1" "$MH2" "$MH3" "$MLB"
done
sleep 3
MTN_NOT_LOADED=$(grep -c "That position is not loaded" /tmp/server.log 2>/dev/null || true)
if [ "${MTN_NOT_LOADED:-0}" -gt 0 ]; then
  fail "a range fill hit an unloaded chunk after forceload ($MTN_NOT_LOADED hits)"
else
  note "terrain range landed (no 'position not loaded' after forceload)"
fi
grep -iE "Changed [0-9]+ block|Filled" /tmp/server.log | tail -1 \
  || note "range fill summary not in log"
sum "- terrain range: 2 terraced mountains at (CX-4,CZ-20)/(CX-2,CZ+16), tops 185/190, grass terraces + stone caps, 66..82 m from the eye (inside the 83.5 m band, chunk edge 64 m)"
srv "time set noon"

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

# ---- 4a2. LOD PAINT PROOF on the settled baseline (v6.14) --------------------
# The marker column (5x111x5 red_concrete, y 60..170) stands 75 m east of the
# eye — 11 m BEYOND the vanilla chunk edge (64 m). Its silhouette on this
# baseline is drawn by the clod:lod_terrain march out of the PERSISTENT
# decoded heightfield, not by any vanilla renderer. This is the metric the
# gallery rounds kept missing: the far frames were captioned «пустое небо»
# while the LOD paint stood in them (run 42358652148 baseline/far, ~40k px).
LODPX=$(lod_silhouette 10_before_edit)
note "lod paint baseline: ${LODPX:-?} cols"
sum "- ${LODPX:-?} (LOD paint ON SCREEN at 75 m if > 20 — the march-drawn marker column; vanilla cannot render there, chunk edge 64 m; #255 re-measures 49, the ZONE_FAR=48 regression #256 and the control read 0)"

# ---- 4a3. TERRAIN-GREEN RULER (v6.15, feat/visible-terrain) ------------------
# The v6.14 silhouette ruler keys on the marker's DARK column. The v6.15
# range paints BRIGHT: grass slopes (fogged green) and stone caps. This
# ruler counts terrain-green pixels in the sky region (x < 940 — UI-immune,
# the lens panels live at x >= 940): green-dominant, not sky-blue, not the
# marker red. On the v6.14 baselines this number is ~0 (flat plains — the
# only paint was the dark column); with the range streamed and marched it
# must be in the tens of thousands. The CONTROL arm reads ~0 on the same
# world because the control client has no pack: no march, no paint.
TERRAIN_PX=$(python3 - <<'PYEOF'
from PIL import Image
import os
im = Image.open(f"{os.environ.get('SHOTS','/home/runner/mc-test/shots/ci-visual')}/10_before_edit.png").convert('RGB')
px = im.load(); n = 0
for y in range(0, 720):
    for x in range(0, 940):
        r, g, b = (c / 255.0 for c in px[x, y])
        # grass green (fog-faded): green dominant over both red and blue
        if g > r + 0.06 and g > b + 0.06 and g > 0.22:
            n += 1
print(n)
PYEOF
)
note "terrain-green baseline: ${TERRAIN_PX:-?} px"
sum "- ${TERRAIN_PX:-?} terrain-green px in the sky region (LOD MOUNTAINS on screen if > 5000; the range paints ColLow-green on low terraces via the fallback palette; flat-world baselines and the pack-free control read ~0)"

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
srv "particle minecraft:end_rod $((CX - 2)) 150 $CZ 1 1 1 0 60 force"
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

# ---- 4b2. THE CUT WALK (v6.15.2, feat/visible-terrain — the G4 instrument) ---
# G4's flap: the client's far particle render comes and goes ACROSS RUNS on
# identical geometry (#246 alive at 80 m, #255 cut ~50 m, #264 alive at 75 m
# = 376 px, #267 dead at 75 m at the y=120 eye, #269 dead at 75 m at the
# PROVEN y=150 eye with the close burst alive at 561 px) — and the whole
# decode proof rides on it: no rendered dot, no capture, no paint. This
# walk measures the cut INSIDE the run: three console end_rod bursts on the
# marker ray (20 m already fired as burst_close, 75 m as burst_far), short
# 60-tick lifetimes so consecutive shots never overlap:
#   40 m -> 09w40, 60 m -> 09w60, 100 m -> 09w100
# each counted white in the central window (x 500..780 — azimuth-aligned
# with the marker ray; the marker column is dark and the range paint is
# green/brown, neither can fake white). The measured cut = the largest
# distance clearing 20 px; it lands in the summary next to the rest — every
# future run names its own cut, and a decode failure with cut >= 83 m
# convicts the capture (G7a) while cut < 83 m convicts the flap (G4).
WALK_CUT="none"
# the close burst (400 ticks, 20 s) must be dead before the first walk
# shot — its dots sit inside the walk window (x 500..780, y 300..540);
# the far burst now expires in 3 s (60 ticks) for the same reason.
sleep 25
for wspec in "40 $((VX + 40)) 09w40" "60 $((VX + 60)) 09w60" "100 $((VX + 100)) 09w100"; do
  read -r WD WX WSHOT <<< "$wspec"
  srv "particle minecraft:end_rod $WX 150 $CZ 0.2 0.2 0.2 0 60 force"
  sleep 1
  shot "$WSHOT" || exit 1
  WC=$(python3 - "$SHOTS/$WSHOT.png" <<'PYEOF'
import sys
from PIL import Image
im = Image.open(sys.argv[1]).convert('RGB')
px = im.load(); n = 0
for y in range(300, 540):
    for x in range(500, 780):
        r, g, b = (c / 255.0 for c in px[x, y])
        if min(r, g, b) > 0.75 and max(r, g, b) - min(r, g, b) < 0.15:
            n += 1
print(n)
PYEOF
)
  echo "- cut walk ${WD} m: ${WC:-?} white px (> 20 = rendered)" | tee -a "$SUMMARY"
  if [ "${WC:-0}" -gt 20 ] 2>/dev/null; then
    WALK_CUT=$WD
  fi
  sleep 5
done
sum "- measured far-render cut: ${WALK_CUT} m (G4 flap instrument; the decode proof needs >= 83 m from the y=151.6 eye)"

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
  LP=$(lod_silhouette "12_after_edit_probe$k")
  echo "probe$k red=${PC:-?} paint=${PS:-?} ${LP} $LM" >> "$PROBE_LOG"
  sum "- probe $k: red=${PC:-?} paint=${PS:-?} ${LP} $LM (need red>=$MIN_PROBE paint>=$MIN_CHANGE lod>20)"
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

# ---- 5b. PARITY DEBUG TOUR (v6.16, v6.17 demoted to evidence lane) ----------
# The v6.16 parity tour shot the matrix from the SESSION-1 client: the DEBUG
# pack (the capture-lens panels paint the right ~27% of every frame — run
# #281's triptychs), on the view-distance-4 server (the vanilla world ends
# at 64 m while the voxy reference leg legitimately renders to 128 m) and
# inside a client carrying the whole visual-CI session state. v6.17 keeps
# the tour as EXTRA EVIDENCE but moves the GATE's input to the dedicated
# parity-world session (§7b): same view-distance as voxy, PRODUCTION pack
# (no lens, no panels), fresh client. Non-fatal by design: a missing or
# failed tour shot shows up as a FAIL pair in the gate report, it never
# aborts the transport proof. Runs on the session-1 client (before the
# leg-2 kick) with a 10 s settle per vantage — the rot/camera channels
# re-arm on the tp, the heightfield is persistent, the march paints from
# what has already decoded.
if [ "${CLOD_CONTROL:-0}" != "1" ]; then
  PDIR="$SHOTS/parity_dbg"; mkdir -p "$PDIR"
  PARITY="$BENCH_DIR/parity_vantages.txt"
  PTOT=$(grep -cv -e '^$' -e '^#' "$PARITY" 2>/dev/null || echo 0)
  sum "- parity debug tour: $PTOT vantages (session-1 client, DEBUG pack — evidence only, the gate pairs the §7b world session)"
  ct "parity debug tour start"
  PN=0
  while read -r PNAME PYAW PPITCH PX PY PZ <&3; do
    case "$PNAME" in ''|\#*) continue;; esac
    # v6.16.1: the row tokens go through ARITHMETIC evaluation — "CX-75"
    # must become -1005, not the literal string "-930-75" (a tp arg the
    # command parser rejects: the shot would silently capture the previous
    # vantage). Decimal y values never enter $(()) — they pass through.
    PX=$(( ${PX//CX/$CX} )); PZ=$(( ${PZ//CZ/$CZ} ))
    srv "time set noon"  # v6.16.5: the clock drifts (see run #278) — pin
                         # the light per vantage like the voxy leg now does
    srv "tp Stepan $PX $PY $PZ $PYAW $PPITCH"
    sleep 10
    PN=$((PN + 1))
    shot "parity_dbg/$PNAME" || true
    ct "parity debug shot $PNAME ($PN/$PTOT)"
  done 3< "$PARITY"
  sum "- parity debug tour done: $PN shots in parity_dbg/ (the §7b world session feeds the gate)"
  # v6.18.5: the §5b mask forensics block is GONE — the session-1 pack no
  # longer carries the mask (the #290 probe pollution); the engagement
  # evidence = the §7b mask8 client below.
fi

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
srv "particle minecraft:dust{color:[0.0,1.0,0.0],scale:1.0} $((CX - 1)) 150 $CZ 0 0 0 0 600 force"
sleep 1
shot 14_leg2_burst_greendust || exit 1
GDL=$(changed_class 14_leg2_baseline 14_leg2_burst_greendust)
GD=$(echo "$GDL" | grep -o 'green=[0-9]*' | cut -d= -f2)
GDT=$(echo "$GDL" | grep -o 'total=[0-9]*' | cut -d= -f2)
sum "- leg-2 carrier probe green dust (80 m): changed-green px ${GD:-?} / total changed ${GDT:-?} vs baseline (> 100 green = a stable green carrier exists; total > 0 with green ~0 = the dust renders but fog washes it — a ruler problem, not a carrier problem)"
echo "- leg-2 green dust changed-green px: ${GD:-?} total changed: ${GDT:-?}" >> "$PROBE_LOG"
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

# ---- 7b. THE PARITY WORLD (v6.17): the session the gate deserves -------------
# Run #281's triptychs split the parity diff into classes we were MEASURING
# but not COMPARING: our lens panels (the debug pack paints the right ~27%
# of every frame), our data horizon (view-distance 4 = vanilla ends at 64 m;
# the voxy leg's server ships 8 = 128 m and its frames contain terrain ours
# never receives), and our session state. This block rebuilds the world the
# way the voxy leg sees it and shoots the SAME matrix from a DEDICATED
# client:
#   - view-distance 8 in server.properties (the voxy leg's server value);
#   - the PRODUCTION pack (CLOD_EXTRA_OPTS without debug_lens — packgen
#     ships no lens pass, no ClodLens UBO, no panels; test
#     debug_lens_pack_ships_lens_production_does_not);
#   - the SAME built world: the mountains + marker were placed in session 1
#     and live in the same level dir; the voxy leg builds the identical
#     structures (run_voxy_shots.sh §3b), so both worlds = seed + structures;
#   - the same shared matrix, the same spectator pose, the same noon pin.
# Non-fatal by design: the transport verdict above is already computed; a
# parity-world failure degrades to missing shots (FAIL pairs in the gate),
# never to a red transport run.
if [ "${CLOD_CONTROL:-0}" != "1" ]; then
  ct "parity world: rebuilding the session"
  DIST="${MC_TEST_DIST:-$HOME/mc-test/dist}"
  PROP="$DIST/server.properties"
  if [ -f "$PROP" ]; then
    sed -i 's/^view-distance=.*/view-distance=8/' "$PROP"
    grep -qE '^view-distance=' "$PROP" || echo 'view-distance=8' >> "$PROP"
    sum "- parity world: view-distance=8 (the voxy leg's server value)"
  else
    note "parity world: server.properties not found at $PROP — session runs on vd 4"
  fi
  # The production pack: no lens pass, no panels. The debug cell window
  # stays — the march's center hint is harmless without the lens.
  # v6.18 UN-FOG: the parity session arms the un-fog pass (the run #285
  # residual class — vanilla fog on the far forest vs the crisp voxy mesh)
  # and drops the atmosphere veil: it would re-tint the un-fogged band by
  # 2.6-6.4% while the voxy leg ships no veil.
  # v6.19 G8d MESH PAINT: the march depth-tests the vanilla band and paints
  # the unified march look wherever the streamed surface stands in front —
  # the run #292 representation-gap verdict's v7-class lever (rows 01-10).
  # v6.19.5 THE PARITY-SESSION WITNESS: #298 split the suspect list — the
  # mask8 client (debug_unfog chain) bound PaintParams (probe x307=(13,254))
  # and painted 78624 magenta-ish px, while THIS session's client downloaded
  # the armed pack (sha 99f0ca64, cache-miss proven) and its frames stayed
  # 99.2% identical to pre-G8d #293. The unfog-chain march either never ran
  # the vanilla path or ran it blind. Until the split is named, the parity
  # session carries its own probe (mesh_debug): the gate is red regardless,
  # one diagnostic round of probe-contaminated frames is the cheapest way
  # to see the armed chain's own engagement map + delivered uniform bytes.
  # v6.19.7 THE UNFOG A/B VERDICT (run #301): the unfog pass is CLEARED —
  # the parity boot with unfog=false (the plain chain) stayed dead: the
  # west-band heartbeat (0,0), magenta=0, while the mask8 lane in the SAME
  # run decoded the west band FULLY (x309h=235 = a live ~172 height, x310c
  # alpha=254) and painted 90128 px. The post-tour stream counters finally
  # exposed the real class: the parity server's LAST diag (at the 07
  # vantage — OUTSIDE the disk, z=-220 vs the disk edge CZ-256 — cone 0 is
  # CORRECT there) reads wire bundle 0p/0b cache 0/24576 — the session-3
  # process never recorded a single delivered particle, and the production
  # pack has NEVER painted in CI (the §7b session is the only plain-chain
  # lane; leg-1/§5b run the debug chain, mask8 the debug_unfog chain —
  # all alive). v6.19.8 SPLITS WORLD-vs-CHAIN: the parity boot now runs
  # THE DEBUG CHAIN (debug_lens=true — the windows paint the client's
  # capture targets on screen, the v6.7.3 all-zero classifier names a dead
  # pipeline per panel) + the mesh probes. Alive windows on the parity
  # world => the plain chain's pack is the killer (the last pass
  # clod_blit-vs-debug_lens is then the only structural difference left);
  # dark windows => the §7b session's STREAM starves regardless of chain
  # and the tour-window server counters (the preserved log) name it.
  # v6.19.10 THE HONEST GATE ROUND: #303 validated the whole pipeline on the
  # parity world (stream max cone 53872 / zone 5092 — was 0/0; the capture
  # heartbeat x309h=196, alpha 254 — decoded; the mesh engagement 297136 px
  # — the vanilla path painting) with the full observability stack. The
  # stack comes OFF for the honest measurement: the production parity
  # config is debug_lens=false (no panels), unfog=true (the #292 no-op
  # stays — #301 cleared it of the capture murder), mesh_paint=true (G8d
  # itself), NO mesh_debug (the magenta tint and the probe pixels are
  # evidence tools, not the render), atmosphere=false (the veil would
  # re-tint the band 2.6-6.4% vs the voxy leg that ships none). The table
  # this round is the project's FIRST honest march-vs-mesh parity row set:
  # near rows = the unified march look against the voxy mesh (the G8d
  # representation-gap lever's real number), far rows = the honest band.
  # CLOD_CENTER stays explicit — the v6.19.9 root cause never returns.
  export CLOD_EXTRA_OPTS="debug_cell_x=${CX};debug_cell_z=${CZ};debug_lens=false;unfog=true;mesh_paint=true;atmosphere=false"
  # v6.19.9 THE ROOT CAUSE (#302's maxima nailed it): parity stream max
  # cone 0, max zone 0 — the scheduler never sent ONE cell the whole
  # session — and the reason is the v6.4.2 regression class verbatim: THIS
  # boot dropped CLOD_CENTER, so boot_server.sh fell back to its 256,320
  # default and the disk armed 1186 blocks from the tour's (-930,796);
  # the player stood outside every decode band the whole tour, the wire
  # delivered nothing, the client's capture starved, the march painted
  # nothing — every parity table since v6.17 paired voxy against an EMPTY
  # frame. The mask8 lane always passed CLOD_CENTER explicitly — that is
  # why it was the only live §7b evidence all along. THE FIX: the same
  # explicit center the other two boots pass. The lens crop dir also gets
  # created (lens_metrics saves the crop under lens_<name>/ and died on
  # the missing directory — the #302 lens decode printed ?).
  CLOD_CENTER="$CENTER" bash "$BENCH_DIR/boot_server.sh" \
    || note "parity world: boot_server.sh rc=$? (continuing — the gate degrades to FAIL pairs)"
  mkdir -p "$SHOTS/lens_parity"
  srv "gamerule doDaylightCycle false"
  srv "time set noon"
  # Fresh client: the session-1/leg-2 processes hold the X display — kill
  # them first (their sessions died with the old server anyway).
  pkill -f "net.minecraft.client.main.Main" 2>/dev/null || true
  sleep 3
  bash "$BENCH_DIR/run_client.sh" || note "parity world: run_client.sh rc=$?"
  PW=$(join_count)
  for _ in $(seq 1 240); do
    [ "$(join_count)" -gt "$PW" ] 2>/dev/null && break
    sleep 2
  done
  wait_log "Stepan joined the game" /tmp/server.log 240 \
    || note "parity world: join never seen (gate degrades to FAIL pairs)"
  sum "- parity world: joined session 3 ($(date -u +%H:%M:%S))"
  sleep 5
  accept_pack_prompt parityworld 30 || note "parity world: pack prompt never accepted"
  sleep 10
  srv "gamemode spectator Stepan"
  # v6.17: the fresh server rescans the disk (the level is the same, the
  # Streamer is new) — the march paints from the STREAMED buffers, so give
  # the scan the same budget the voxy leg gives its meshing (MESH_WAIT 240)
  # and both legs render a COMPLETE world before the matrix.
  PCELLS=0
  for _ in $(seq 1 120); do
    PCELLS=$(curl -s http://127.0.0.1:8199/clod-pack.info \
             | grep -o 'cells=[0-9]*' | head -1 | cut -d= -f2)
    PCELLS="${PCELLS:-0}"
    [ "$PCELLS" -ge 65000 ] 2>/dev/null && break
    sleep 2
  done
  sum "- parity world: scan populated (cells=$PCELLS; the march paints from these buffers)"
  # THE PARITY WORLD TOUR — the same matrix, now from the production-pack
  # client on the voxy-identical world. settle 10 s like §5b.
  mkdir -p "$SHOTS/parity"
  PARITY="$BENCH_DIR/parity_vantages.txt"
  PTOT=$(grep -cv -e '^$' -e '^#' "$PARITY" 2>/dev/null || echo 0)
  sum "- parity world tour: $PTOT vantages (production pack, vd 8, shared structures)"
  PW_N=0
  while read -r PNAME PYAW PPITCH PX PY PZ <&3; do
    case "$PNAME" in ''|\#*) continue;; esac
    PX=$(( ${PX//CX/$CX} )); PZ=$(( ${PZ//CZ/$CZ} ))
    srv "time set noon"
    srv "tp Stepan $PX $PY $PZ $PYAW $PPITCH"
    sleep 10
    shot "parity/$PNAME" || true
    PW_N=$((PW_N + 1))
    ct "parity world shot $PNAME ($PW_N/$PTOT)"
  done 3< "$PARITY"
  sum "- parity world tour done: $PW_N shots in parity/ (the gate pairs voxy/<name>.png vs parity/<name>.png)"
  # v6.19.7: the parity session's OWN stream counters — the leg-1 section
  # greps the shared /tmp/server.log at ITS point, nobody ever dumped the
  # session-3 counters (the wire/cone numbers the starving-capture question
  # asks first). v6.19.8: ALSO the per-metric MAXIMA over the whole session
  # (the post-tour diag sits at the outside-disk 07 vantage where cone 0 is
  # correct — the tour-window values are the evidence) + the log itself
  # rides the artifact before the mask8 boot overwrites it.
  { echo "### parity world stream counters (session 3, tail + maxima)";
    grep -E "\[c-LoD\] stream: tick" /tmp/server.log | tail -2 || true;
    for m in cone zone "wire bundle"; do
      V=$(grep -oE "${m} [0-9]+" /tmp/server.log | awk '{print $2}' | sort -n | tail -1);
      echo "- parity stream max ${m}: ${V:-0}";
    done; } >> "$SUMMARY"
  cp /tmp/server.log "$SHOTS/parity/server.log" 2>/dev/null || true
  # v6.19.8: the parity session runs the DEBUG chain — the lens windows
  # paint the client's capture targets; decode them with the established
  # v6.7 forensics (the all-zero classifier names a dead pipeline per
  # panel) straight into the summary.
  LM=$(mktemp); lens_metrics parity/01_aerial_west > "$LM" 2>/dev/null || true
  { echo "### parity world capture lens (debug chain on the §7b world)";
    cat "$LM"; } >> "$SUMMARY"
  rm -f "$LM"
  # v6.19.5: the parity session carries the mesh probe (mesh_debug=true) —
  # decode it here, the same way the mask8 forensics decodes its own block.
  PWPROBE=$(mktemp)
  python3 - "$SHOTS/parity" > "$PWPROBE" 2>/dev/null <<'PYEOF' || true
import sys, os
from PIL import Image
p = os.path.join(sys.argv[1], "01_aerial_west.png")
if os.path.exists(p):
    im = Image.open(p).convert("RGB"); px = im.load(); w, h = im.size
    for yy in (2, h - 3):
        m1 = px[306, yy]; m2 = px[307, yy]; m3 = px[308, yy]
        m4 = px[309, yy]; m5 = px[310, yy]
        print(f"- parity mesh probe @png_y={yy}: x306=({m1[0]},{m1[1]},{m1[2]}) x307=({m2[0]},{m2[1]},{m2[2]}) x308=({m3[0]},{m3[1]},{m3[2]}) x309h=({m4[0]},{m4[1]},{m4[2]}) x310c=({m5[0]},{m5[1]},{m5[2]})")
    mag = 0; tot = 0
    for y in range(0, h, 4):
        for x in range(0, w, 4):
            c = px[x, y]; tot += 1
            if c[0] > c[1] + 60 and c[2] > c[1] + 60:
                mag += 1
    print(f"- parity mesh engagement: magenta-ish={mag*16} of {tot*16} sampled px")
PYEOF
  while IFS= read -r l; do sum "$l"; done < "$PWPROBE"
  rm -f "$PWPROBE"
  # v6.19.2: preserve the parity client's log BEFORE the mask8 boot overwrites
  # it — the pack SHA the gate frames actually rendered is the first thing a
  # frozen-parity question asks (the #295 mesh-paint freeze had no witness).
  cp /tmp/client.log "$SHOTS/parity/client.log" 2>/dev/null || true
  srv "time set noon"
  # v6.18.4 THE MASK ON THE PARITY WORLD: the §5b masks ride the session-1
  # client (view-distance 4 — no vanilla geometry beyond 64 m, so the masks
  # photograph the DUST carrier, not the forest; run #289's mask01 red band
  # = the saturated carrier dots). The engagement map the gate needs must
  # come from the PARITY WORLD itself (vd 8, the mountains + forest in
  # view): a DEDICATED boot — server (the mask pack: the module builds the
  # debug_unfog chain; run #290 proved the mask8 client silently kept the
  # PRODUCTION pack when only the client was recycled — the state block
  # pixels read as plain sky, the pack never reached the client) + fresh
  # client, three diagnostic vantages into parity_dbg8/.
  # Non-fatal by design (evidence, never the gate's input).
  # v6.19.2 MESH DEBUG: the mask8 lane doubles as the G8d engagement probe —
  # mesh_paint=true arms the vanilla path, mesh_debug=true compiles the
  # probe (#define: shader-live marker + the delivered PaintParams bytes +
  # a 35% magenta lift on every vanilla-path pixel). The #295 freeze
  # (parity table identical pre/post G8d) had no witness — this boot is it.
  export CLOD_EXTRA_OPTS="debug_cell_x=${CX};debug_cell_z=${CZ};debug_lens=false;unfog=true;unfog_debug=true;mesh_paint=true;mesh_debug=true"
  CLOD_CENTER="$CENTER" bash "$BENCH_DIR/boot_server.sh" \
    || note "mask8: boot_server.sh rc=$? (the mask evidence degrades, the gate unaffected)"
  pkill -f "net.minecraft.client.main.Main" 2>/dev/null || true
  sleep 3
  bash "$BENCH_DIR/run_client.sh" || note "mask8: run_client.sh rc=$?"
  MW=$(join_count)
  for _ in $(seq 1 120); do
    [ "$(join_count)" -gt "$MW" ] 2>/dev/null && break
    sleep 2
  done
  sleep 5
  accept_pack_prompt mask8 30 || note "mask8: pack prompt never accepted"
  sleep 5
  srv "gamemode spectator Stepan"
  mkdir -p "$SHOTS/parity_dbg8"
  for row in "01m 90 15 CX 140 CZ" "07m 180 5 CX 160 -220" "10m 270 5 CX-75 151.6 CZ"; do
    set -- $row
    MN="$1"; MYAW="$2"; MPITCH="$3"; MX="$4"; MY="$5"; MZ="$6"
    MX=$(( ${MX//CX/$CX} )); MZ=$(( ${MZ//CZ/$CZ} ))
    srv "time set noon"
    srv "tp Stepan $MX $MY $MZ $MYAW $MPITCH"
    sleep 10
    shot "parity_dbg8/$MN" || true
  done
  sum "- mask8: the un-fog engagement masks shot on the PARITY WORLD (vd 8) — parity_dbg8/01m|07m|10m"
  cp /tmp/client.log "$SHOTS/parity_dbg8/client.log" 2>/dev/null || true
  # v6.19.2: preserve the mask8 client's log — the pack SHA + any post-chain
  # config warnings (the uniform-binding suspect) live only here.
  # v6.18.5: the mask forensics + the fog-state decode go through `sum` —
  # the #290 python printed to stdout and never reached the summary.
  M8FORENSICS=$(mktemp)
  python3 - "$SHOTS/parity_dbg8" > "$M8FORENSICS" 2>/dev/null <<'PYEOF' || true
import sys, os
from PIL import Image
GREEN = (60, 220, 80); RED = (250, 60, 50); BLUE = (70, 100, 250)
def close(px, ref, tol=90):
    return all(abs(int(a) - b) < tol for a, b in zip(px[:3], ref))
for name in ("01m", "07m", "10m"):
    p = os.path.join(sys.argv[1], f"{name}.png")
    if not os.path.exists(p):
        continue
    im = Image.open(p).convert("RGB"); px = im.load(); w, h = im.size
    g = r = b = dim = 0
    for y in range(0, h, 4):
        for x in range(0, w, 4):
            c = px[x, y]
            if close(c, GREEN): g += 1
            elif close(c, RED): r += 1
            elif close(c, BLUE): b += 1
            elif sum(c) < 330 and abs(c[0]-c[1]) < 24 and abs(c[1]-c[2]) < 24: dim += 1
    print(f"- mask8 {name}: green={g*16} red={r*16} blue={b*16} dim={dim*16} (px, sampled /16)")
st = os.path.join(sys.argv[1], "01m.png")
if os.path.exists(st):
    im = Image.open(st).convert("RGB"); px = im.load()
    w, h = im.size
    # v6.19.2 MESH DEBUG probe decode: the march writes three markers at
    # GL y=2 and GL y=h-3 (the PNG sees y=h-3 and y=2 — the GL y-flip).
    #   x=306 WHITE = the probe shader is live (no uniform involved)
    #   x=307 r/g = the delivered PaintParams.x/.y (bytes of v*254)
    #   x=308 r = 255 iff the shader is live but PaintParams.y < 0.5
    #         g = the rot-channel confidence byte
    for yy in (2, h - 3):
        m1 = px[306, yy]; m2 = px[307, yy]; m3 = px[308, yy]
        m4 = px[309, yy]; m5 = px[310, yy]
        print(f"- mesh probe @png_y={yy}: x306=({m1[0]},{m1[1]},{m1[2]}) x307=({m2[0]},{m2[1]},{m2[2]}) x308=({m3[0]},{m3[1]},{m3[2]}) x309h=({m4[0]},{m4[1]},{m4[2]}) x310c=({m5[0]},{m5[1]},{m5[2]})")
    # The magenta engagement map: every vanilla-path pixel carries a 35%
    # lift toward (255,0,255) — r and b rise above g together.
    mag = 0; tot = 0
    for y in range(0, h, 4):
        for x in range(0, w, 4):
            c = px[x, y]; tot += 1
            if c[0] > c[1] + 60 and c[2] > c[1] + 60:
                mag += 1
    print(f"- mesh engagement: magenta-ish={mag*16} of {tot*16} sampled px (the vanilla-path paint coverage)")
st = os.path.join(sys.argv[1], "01m.png")
if os.path.exists(st):
    im = Image.open(st).convert("RGB"); px = im.load()
    w, h = im.size
    # GL gl_FragCoord.y grows UPWARD, the PNG grows DOWNWARD: the block
    # written at GL y=2 lands at PNG y=h-3. Read both ends, take the one
    # that differs from the local sky (the #292 probe read the wrong end
    # and 'decoded' the sky gradient).
    for yy in (2, h - 3):
        a = px[300, yy]; c = px[301, yy]
        marker = yy == 2 and False or (a[0] > 240 and a[1] < 40)
        print(f"- mask8 fog state @png_y={yy}: raw (300)=({a[0]},{a[1]},{a[2]}) (301)=({c[0]},{c[1]},{c[2]})")
        if not (abs(int(a[0]) - 151) < 12 and abs(int(a[1]) - 184) < 12):
            print(f"- mask8 fog state decoded: envStart={a[0]*8} envEnd={a[1]*8} rdEnd={a[2]*16} fogColor=({c[0]},{c[1]},{c[2]})")
PYEOF
  while IFS= read -r l; do sum "$l"; done < "$M8FORENSICS"
  rm -f "$M8FORENSICS"
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
