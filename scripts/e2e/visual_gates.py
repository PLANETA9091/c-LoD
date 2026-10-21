#!/usr/bin/env python3
"""Visual + client-log gates for the c-LoD E2E screenshot matrix.

Runs in CI (e2e-visual job) after the bench client phase, and locally against
any shots dir. Dependency-free: PNG decoding via zlib/struct only.

The client runs at 640x360 centered on a 1280x720 Xvfb screen; only the
window rect is analyzed (the root window renders black).

Image gates (V):
  V1  every expected shot exists, decodes as PNG, is 1280x720
  V2  frames are not uniform/black inside the window (frozen or dead
      pipeline => flat frame)
  V3  the dark LOD band is present at the horizon: distant LOD fragments
      render dark blue-gray against the bright sky/veil in every direction
  V4  the sky band is sky-like at locked noon (if the (0,0) camera-channel
      pixel breaks, the post-chain loses the view vector and the sky
      collapses into a wrong flat band)
  V5  the sub-horizon veil plane is alive (not black) — the posteffect
      actually drew the far-world plane below the horizon

Client-log gates (C):
  C1  the server resource pack was accepted/applied
  C2  the clod post effect was never reported missing
  C3  no shader compile/link errors
  C4  no GLX visual failure (sRGB shim broken)

Exit 0 only when every gate passes. Usage:
  visual_gates.py <shots-dir> <client-log> [more-log-files...]
"""
import multiprocessing
import re
import struct
import sys
import zlib
from pathlib import Path

W, H = 1280, 720
# the MC window is 640x360 centered on the 1280x720 Xvfb screen; the root
# window around it renders black and must be excluded from every statistic
WIN = (324, 184, 956, 536)  # x0, y0, x1, y1 (4px inset)

# shots that must show the LOD world (everything except the two UI shots)
UI_SHOTS = {"00_join_state", "00_pack_applied"}
# shots taken OUTSIDE the atlas window: the LOD must NOT paint there (the
# edge fade releases before the border) — V3's expectation is inverted
OUT_SHOTS = {"07_veil_outside"}
# aerial pitch (-30deg) puts the horizon line outside the fixed band — V2/V5
# cover that frame, V3 skips it
V3_SKIP = {"03_noon_aerial"}

# V3 band: a thin strip AROUND THE HORIZON LINE (pitch-0 fov-70 puts el=0 at
# 0.50 of the window). Vanilla-only frames render pure sky/fog there; LOD
# hill and tree tops poke ABOVE the horizon line — terrain-tone pixels in
# this strip are direct evidence of the LOD silhouette.
HOR = (0.42, 0.505)
# sky band scanned for sky-likeness at noon
SKY = (0.05, 0.30)
# veil band (below horizon) scanned for liveness
VEIL = (0.62, 0.95)

MIN_STD_UI = 4.0        # V2: UI screens must not be flat
MIN_STD_WORLD = 5.0     # V2: world frames must not be flat
MIN_DARK = 0.008         # V3: terrain-tone fragments in the horizon band
# v4.3.1: the LOD is TEXTURED and lit by the vanilla lightmap — bright grass
# lands at luma 120-180, well above the old flat-palette darkness cutoff of
# 140. The horizon-band discriminator is now "terrain tone vs sky": mid and
# dark tones (luma < 185) count, sky/fog-blue (~200+) does not.
MAX_DARK_OUT = 0.50     # V3 inverted: an out-of-window frame must stay mostly clean
MIN_SKY = 0.50          # V4: measured 89-100% on v4.1.0
MIN_VEIL_LUMA = 80.0    # V5: measured 156-201 on v4.1.0


def read_png(path):
    """Minimal PNG reader: 8-bit gray/RGB/RGBA, non-interlaced."""
    data = Path(path).read_bytes()
    if data[:8] != b"\x89PNG\r\n\x1a\n":
        raise ValueError("not a PNG")
    pos, idat, ihdr = 8, b"", None
    while pos + 8 <= len(data):
        ln = struct.unpack(">I", data[pos:pos + 4])[0]
        typ = data[pos + 4:pos + 8]
        chunk = data[pos + 8:pos + 8 + ln]
        pos += 12 + ln
        if typ == b"IHDR":
            w, h, depth, ctype, comp, filt, inter = struct.unpack(">IIBBBBB", chunk)
            if depth != 8 or inter != 0 or ctype not in (0, 2, 6) or comp != 0 or filt != 0:
                raise ValueError(f"unsupported PNG: depth={depth} ctype={ctype} inter={inter}")
            ihdr = (w, h, ctype)
        elif typ == b"IDAT":
            idat += chunk
        elif typ == b"IEND":
            break
    if ihdr is None:
        raise ValueError("no IHDR")
    w, h, ctype = ihdr
    ch = {0: 1, 2: 3, 6: 4}[ctype]
    raw = zlib.decompress(idat)
    stride = w * ch
    out = bytearray(h * stride)
    prev = bytearray(stride)
    p = 0
    for y in range(h):
        ft = raw[p]
        p += 1
        line = bytearray(raw[p:p + stride])
        p += stride
        if ft == 1:
            for i in range(ch, stride):
                line[i] = (line[i] + line[i - ch]) & 0xFF
        elif ft == 2:
            for i in range(stride):
                line[i] = (line[i] + prev[i]) & 0xFF
        elif ft == 3:
            for i in range(stride):
                a = line[i - ch] if i >= ch else 0
                line[i] = (line[i] + ((a + prev[i]) >> 1)) & 0xFF
        elif ft == 4:
            for i in range(stride):
                a = line[i - ch] if i >= ch else 0
                b = prev[i]
                c = prev[i - ch] if i >= ch else 0
                pa = abs(b - c)
                pb = abs(a - c)
                pc = abs(a + b - 2 * c)
                pr = a if pa <= pb and pa <= pc else (b if pb <= pc else c)
                line[i] = (line[i] + pr) & 0xFF
        out[y * stride:(y + 1) * stride] = line
        prev = line
    return w, h, ch, bytes(out)


def luma_at(px, o, ch):
    if ch == 1:
        return px[o]
    return (px[o] * 54 + px[o + 1] * 183 + px[o + 2] * 19) >> 8


def is_sky(r, g, b):
    """Sky-like at locked noon: bright, clearly blue-dominant."""
    return b > 110 and b > r + 15 and g > 85


def analyze(name, path):
    """Return (ok, report-lines) for one shot."""
    problems = []
    try:
        w, h, ch, px = read_png(path)
    except Exception as e:  # noqa: BLE001
        return False, [f"  FAIL V1 {name}: unreadable PNG ({e})"]
    if (w, h) != (W, H):
        return False, [f"  FAIL V1 {name}: {w}x{h}, expected {W}x{H}"]
    lines = [f"  OK   V1 {name}: PNG {w}x{h}"]

    wx0, wy0, wx1, wy1 = WIN
    step = 2  # sample every 2nd pixel — plenty for statistics
    is_ui = name in UI_SHOTS

    lum = []
    for y in range(wy0, wy1, step):
        row = y * W * ch
        for x in range(wx0, wx1, step):
            lum.append(luma_at(px, row + x * ch, ch))
    n = len(lum)
    mean = sum(lum) / n
    std = (sum((v - mean) ** 2 for v in lum) / n) ** 0.5
    min_std = MIN_STD_UI if is_ui else MIN_STD_WORLD
    if std <= min_std:
        problems.append(f"  FAIL V2 {name}: flat frame (luma stddev {std:.1f} <= {min_std}) — pipeline dead?")
    else:
        lines.append(f"  OK   V2 {name}: luma stddev {std:.1f} > {min_std} (frame has content)")
    if is_ui:
        return not problems, lines + problems

    # V3: dark LOD band at the horizon
    dark = tot = 0
    for y in range(wy0 + int((wy1 - wy0) * HOR[0]), wy0 + int((wy1 - wy0) * HOR[1]), step):
        row = y * W * ch
        for x in range(wx0, wx1, step):
            tot += 1
            if luma_at(px, row + x * ch, ch) < 185:
                dark += 1
    darkf = dark / max(tot, 1)
    if name in V3_SKIP:
        lines.append(f"  OK   V3 {name}: skipped (aerial pitch — horizon outside the band)")
    elif name in OUT_SHOTS:
        if darkf > MAX_DARK_OUT:
            problems.append(f"  FAIL V3 {name}: horizon band dark-LOD fraction {darkf:.1%} > {MAX_DARK_OUT:.0%} — LOD leaked outside the atlas window")
        else:
            lines.append(f"  OK   V3 {name}: horizon band dark-LOD {darkf:.1%} <= {MAX_DARK_OUT:.0%} (outside window stays clean)")
    elif darkf < MIN_DARK:
        problems.append(f"  FAIL V3 {name}: horizon band dark-LOD fraction {darkf:.1%} < {MIN_DARK:.0%} — LODs not rendered?")
    else:
        lines.append(f"  OK   V3 {name}: horizon band dark-LOD {darkf:.1%} >= {MIN_DARK:.0%}")

    # V4: sky band sky-like at noon (camera channel holds)
    ssky = stot = 0
    for y in range(wy0 + int((wy1 - wy0) * SKY[0]), wy0 + int((wy1 - wy0) * SKY[1]), step):
        row = y * W * ch
        for x in range(wx0, wx1, step):
            o = row + x * ch
            stot += 1
            if ch >= 3 and is_sky(px[o], px[o + 1], px[o + 2]):
                ssky += 1
    skyf = ssky / max(stot, 1)
    if skyf < MIN_SKY:
        problems.append(f"  FAIL V4 {name}: sky band only {skyf:.1%} sky-like — noon camera channel broken?")
    else:
        lines.append(f"  OK   V4 {name}: sky band {skyf:.1%} sky-like (camera channel holds at noon)")

    # V5: sub-horizon veil plane alive (not black)
    vl = vt = 0
    for y in range(wy0 + int((wy1 - wy0) * VEIL[0]), wy0 + int((wy1 - wy0) * VEIL[1]), step):
        row = y * W * ch
        for x in range(wx0, wx1, step):
            vl += luma_at(px, row + x * ch, ch)
            vt += 1
    veil = vl / max(vt, 1)
    if veil < MIN_VEIL_LUMA:
        problems.append(f"  FAIL V5 {name}: veil plane luma {veil:.0f} < {MIN_VEIL_LUMA:.0f} — posteffect plane dead?")
    else:
        lines.append(f"  OK   V5 {name}: veil plane luma {veil:.0f} >= {MIN_VEIL_LUMA:.0f} (posteffect alive)")
    return not problems, lines + problems


def one(args):
    name, path = args
    return name, *analyze(name, path)


CLIENT_GATES = [
    ("C1 pack applied",
     re.compile(r"applying server resource pack|loaded server resource pack|reload.*clod"
                r"|Reloading ResourceManager:[^\n]*server/", re.I),
     "must match", "client never applied the server resource pack"),
]


def log_gates(paths):
    try:
        text = "\n".join(Path(p).read_text(encoding="utf-8", errors="replace") for p in paths) + "\n"
    except FileNotFoundError as e:
        return False, [f"  FAIL C* log missing: {e}"]
    lines_list = text.splitlines()

    # The pack reload marker: the server pack joins the resource stack
    # (26.3 logs the reload source list with server/<...>/<pack-uuid>).
    c1_rx = CLIENT_GATES[0][1]
    pack_idx = max((i for i, ln in enumerate(lines_list) if c1_rx.search(ln)), default=-1)

    clod_rx = re.compile(r"Requested post effect does not exist: clod", re.I)
    late_clod = [i for i, ln in enumerate(lines_list) if clod_rx.search(ln) and i > pack_idx]
    any_clod = [i for i, ln in enumerate(lines_list) if clod_rx.search(ln)]

    def hit(rx):
        return rx.search(text) is not None

    c3_rx = re.compile(r"couldn't compile|failed to compile|couldn't link|failed to link|"
                       r"shader.*error|error.*shader", re.I)
    c4_rx = re.compile(r"Couldn't find matching GLX visual", re.I)

    lines, ok = [], True

    def mark(name, good, hint):
        nonlocal ok
        if good:
            lines.append(f"  OK   {name}")
        else:
            ok = False
            lines.append(f"  FAIL {name} — {hint}")

    # C2: the boot race warns 'post effect does not exist' while the world
    # renders before the pack's reload finishes — only warnings AFTER the
    # pack reload marker mean the effect is really missing. (v4.3.1 CI:
    # the race fired at 16:40:55, the pack reload landed at 16:41:05, the
    # chain compiled and drew — the whole-run regex was a false FAIL.)
    mark("C1 pack applied", pack_idx >= 0, "client never applied the server resource pack")
    if pack_idx < 0:
        mark("C2 post effect exists", not any_clod,
             "clod:lod_terrain post effect missing — pack incomplete")
    else:
        mark("C2 post effect exists", not late_clod,
             f"clod post effect missing after the pack reload (lines {late_clod[:3]})")
    mark("C3 shaders compiled", not hit(c3_rx), "GLSL compile/link error in the client")
    mark("C4 GLX visual ok", not hit(c4_rx), "sRGB shim failed — check libclodgl build")
    return ok, lines


def main():
    if len(sys.argv) < 3:
        print("usage: visual_gates.py <shots-dir> <client-log> [more-logs...]", file=sys.stderr)
        return 2
    shots = Path(sys.argv[1])
    logs = sys.argv[2:]
    all_names = sorted(UI_SHOTS | {"01_noon_ground", "02_noon_high", "03_noon_aerial",
                                   "04_yaw_south", "05_yaw_north", "06_yaw_east",
                                   "07_veil_outside", "08_back_inside"})
    jobs = [(n, str(shots / f"{n}.png")) for n in all_names]
    with multiprocessing.Pool(min(4, len(jobs))) as pool:
        results = pool.map(one, jobs)
    ok = True
    for name, good, lines in results:
        print("\n".join(lines))
        ok = good and ok
    print("--- client log gates ---")
    lok, llines = log_gates(logs)
    print("\n".join(llines))
    print()
    if ok and lok:
        print("VISUAL: ALL GATES PASSED")
        return 0
    print("VISUAL: GATES FAILED")
    return 1


if __name__ == "__main__":
    sys.exit(main())
