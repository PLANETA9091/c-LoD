#!/usr/bin/env bash
# v6.59 THE CASCADE probe — the CI lane for the six coarse rings (cells
# 16 m * 4^k, coverage to ±2.1 Mm). Boots the module with cascade=true,
# joins a real vanilla client at a rank-0-safe vantage (y=140: the
# eye-100 rank rule keeps every scaffold word on the v5.1 planes 80/120 —
# the LIVING SECTION minefield stays clear), polls /clod-pack.info for
# the ring coverage (the builder fills the rings by native chunk
# sampling), and shoots the money shots: the aerial panorama (rings
# behind the disk edge) and the far-horizon view, plus zoom crops (the
# owner reads the frames with his own eyes). GATE: ring idx 0 coverage
# >= R0_FLOOR and ring idx 1 >= R1_FLOOR by the timeout, else exit 1.
set -uo pipefail
BENCH_DIR="$(cd "$(dirname "$0")" && pwd)"
SHOTS="${SHOTS:-/home/runner/mc-test/shots/cascade}"
mkdir -p "$SHOTS"
NOTES="$SHOTS/probe.log"
: > "$NOTES"
note() { echo "$(date -u +%H:%M:%S) $*" | tee -a "$NOTES"; }
CENTER="${CLOD_CENTER:--930,796}"
CX="${CENTER%,*}"
CZ="${CENTER#*,}"

note "cascade probe: center $CENTER, options cascade=true"
export CLOD_EXTRA_OPTS="cascade=true;cascade_lap_ticks=300;cascade_guard_ticks=8;cascade_budget=1400;cascade_gen_budget=64;carrier_sink=-0.03"
CLOD_CENTER="$CENTER" bash "$BENCH_DIR/boot_server.sh" || note "boot_server rc=$? (continuing)"
srv() { echo "$1" > /tmp/server.in; }
srv "gamerule advance_time false"
srv "time set noon"

bash "$BENCH_DIR/run_client.sh" || note "run_client rc=$? (continuing)"
sleep 25

# v6.59.3 (run #503's own frame): the client sat on the REQUIRED-pack prompt
# the whole session — the cascade pack's SHA is new, the cached acceptance is
# for the old one, and the probe never clicked Proceed (the visual lanes
# carry accept_pack_prompt; the server-side gate measured the BUILDER, the
# capture never ran). Ported verbatim from run_ci_visual.sh.
accept_pack_prompt() { # tag loops
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
    print('')
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
        print('561 447')
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
accept_pack_prompt cascade 30

# The fill vantage: y=140 — rank 0 planes (80/120) for every slot (the
# eye-100 rule bottoms out at tier 0), the PROVEN transport from y=140.
srv "gamemode spectator Stepan"
srv "tp Stepan $CX 140 $CZ 0 8"
sleep 10
# The client must be IN-WORLD (the prompt screen or a menu means the capture
# never runs and the march paints nothing): detect the yellow prompt again;
# if it re-rendered (a second push), click it once more.
accept_pack_prompt cascade-2 15

info() { curl -fsS "http://127.0.0.1:8199/clod-pack.info" 2>/dev/null || echo ""; }
cov() { # ring idx -> pct
  info | grep -oE "r$1=[0-9.]+" | head -1 | cut -d= -f2 || echo "0"
}

# ---- the fill loop ----------------------------------------------------------
R0_FLOOR="${R0_FLOOR:-90}"
R1_FLOOR="${R1_FLOOR:-25}"
# 12 min: ring idx 0's annulus admits ~3.5 min (the cursor exhausts), ring
# idx 1's chunks ride the same stream + another ~4 min — run #500 showed
# r0=92.8% at 5 min with r1 just starting.
DEADLINE=$(( $(date +%s) + 12 * 60 ))
best_r0=0
best_r1=0
while [ "$(date +%s)" -lt "$DEADLINE" ]; do
  r0=$(cov 0)
  r1=$(cov 1)
  best_r0=$(python3 -c "print(max($best_r0, $r0))")
  best_r1=$(python3 -c "print(max($best_r1, $r1))")
  note "coverage r0=${r0}% r1=${r1}% (best $best_r0/$best_r1)"
  ok=$(python3 -c "print(1 if $r0 >= $R0_FLOOR and $r1 >= $R1_FLOOR else 0)")
  [ "$ok" = "1" ] && break
  # Run #504's frames: the required-pack prompt RE-RENDERS after every pack
  # apply (the same reason the visual lanes accept before every probe) —
  # one click at session start leaves the prompt up for the whole fill.
  # Accept inside the loop so the world stays visible for the capture.
  accept_pack_prompt cascade-loop 2
  sleep 30
done
accept_pack_prompt cascade-final 5
info > "$SHOTS/pack-info.txt"

# ---- the shots --------------------------------------------------------------
shot() { # name
  ffmpeg -nostdin -y -loglevel error -f x11grab -video_size 1280x720 -framerate 2 \
    -i :99 -frames:v 1 "$SHOTS/$1.png" 2>>"$SHOTS/ffmpeg.err" \
    && note "shot $1" || note "screenshot $1 failed"
}
accept_pack_prompt cascade-shot 5
shot cascade_aerial
# v6.59.7: decode the march's cascade atlas heartbeat strip (GL row 2 =
# PNG y h-3, x 200..204): ring-1/2/3 height slices (marked fraction, max
# byte, min byte), the rot channel (az/el/conf), the ring-1 color slice.
HB=$(SHOTS="$SHOTS" python3 << 'PYD'
import os
from PIL import Image
im = Image.open(os.environ['SHOTS'] + '/cascade_aerial.png').convert('RGB')
w, h = im.size
y = h - 3
out = []
xs = list(range(200, 205)) + [205, 206]
names = ['r1H', 'r2H', 'rot', 'r1C', 'r3H', 'rwin', 'census']
for k, x in enumerate(xs):
    r, g, b = im.getpixel((x, y))
    if k == 2:
        out.append('rot az=%.2f el=%.2f fovf=%.2f' % (r, g, b))
    elif k == 5:
        out.append('rwin w=%d conf=%.2f hits=%d' % (int(r*16+0.5), g, b * 255))
    elif k == 6:
        dx = (g * 32 - 16) * 2.0
        dz = (b * 32 - 16) * 2.0
        out.append('census max(g-b)=%.3f dx=%.0fm dz=%.0fm' % (r / 255 / 4, dx, dz))
    else:
        out.append('%s mark=%.1f%% max=%d min=%d' % (names[k], r*100, int(g*255+0.5), int(b*255+0.5)))
print('HB: ' + ' | '.join(out))
PYD
)
note "${HB:-HB decode failed}"
# the far-horizon view: look south across the rings (yaw 0 = +Z)
srv "tp Stepan $CX 150 $CZ 0 0"
sleep 8
accept_pack_prompt cascade-shot2 5
shot cascade_horizon
# zoom crops (the owner reads the frames with his own eyes)
python3 - <<PYEOF || true
from PIL import Image
import os
for n in ("cascade_aerial", "cascade_horizon"):
    p = os.path.join("$SHOTS", n + ".png")
    if not os.path.exists(p):
        continue
    im = Image.open(p).convert("RGB")
    w, h = im.size
    im.crop((0, int(h * 0.30), w, int(h * 0.80))).resize((w, h)).save(
        os.path.join("$SHOTS", n + "_zoom.png"))
    print("zoom", n)
PYEOF

note "GATE: best r0=$best_r0 (floor $R0_FLOOR), r1=$best_r1 (floor $R1_FLOOR)"
# Run #500: the probe finished but the step hung to the 45-min ceiling —
# the boot_server JVM and the client hold the step's stdout/stderr. Kill
# everything BEFORE the gate verdict.
pkill -f "paper-26.3.jar" 2>/dev/null || true
pkill -f "dev.dist.launcher.Main" 2>/dev/null || true
sleep 3
pkill -f "Minecraft" 2>/dev/null || true
true
awk -v a="$best_r0" -v b="$best_r1" -v fa="$R0_FLOOR" -v fb="$R1_FLOOR" \
  'BEGIN { exit !(a >= fa && b >= fb) }' \
  && { note "CASCADE GATE: GREEN"; exit 0; } \
  || { note "CASCADE GATE: RED"; exit 1; }
