#!/usr/bin/env python3
"""Visual assertions for the c-LoD CI visual test (layer 8).

Compares two screenshots and enforces expectations:

  - paint metric: pixels whose ANY channel moved more than THRESH (the
    paint_diff.py v5.2.1 value; night/dark-sky noise stays below it, so the
    test always locks `time set 6000` before shooting);
  - marker probe: the number of pixels matching the placed block's LOD
    color family. `red` = RED_CONCRETE (MapColor 142,33,33). The probe is
    fog-aware: at 200 m the atmosphere pass mixes the LOD color with the
    noon sky, and fogged red clears r >= 110 AND r-g >= 30 at any fog
    fraction <= 0.5 while fogged grass/dirt/terracotta (r-g <= 32 at the
    source) never does.

Exit code 0 = every expectation held; 1 = a FAIL line was printed; 2 = usage.
Always writes the composite (base + changes highlighted red) when --out is
given, so a failed job uploads the evidence.

Usage:
  visual_diff.py BEFORE.png AFTER.png [--probe red|white] [--min-probe N]
                 [--min-change N] [--out composite.png]
"""
import sys

from PIL import Image, ImageChops

THRESH = 15


def load(p):
    return Image.open(p).convert("RGB")


def is_red(c):
    r, g, b = c
    return r >= 110 and (r - g) >= 30


def is_white(c):
    r, g, b = c
    return min(r, g, b) >= 185 and (max(r, g, b) - min(r, g, b)) <= 30


PROBES = {"red": is_red, "white": is_white}


def main():
    args = sys.argv[1:]
    if len(args) < 2:
        print(__doc__)
        return 2
    before, after = load(args[0]), load(args[1])
    if after.size != before.size:
        print(f"FAIL size mismatch: {before.size} vs {after.size}")
        return 1

    opts = {"--probe": None, "--min-probe": 150, "--min-change": 200, "--out": None}
    i = 2
    while i < len(args):
        if args[i] in opts:
            opts[args[i]] = args[i + 1] if args[i] != "--probe" else args[i + 1]
            i += 2
        else:
            print(f"unknown arg {args[i]}")
            return 2
    probe_fn = PROBES[opts["--probe"]] if opts["--probe"] else None
    min_probe = int(opts["--min-probe"])
    min_change = int(opts["--min-change"])

    diff = ImageChops.difference(before, after).convert("L")
    px = diff.load()
    w, h = diff.size
    changed = sum(1 for y in range(h) for x in range(w) if px[x, y] > THRESH)

    pa = after.load()
    probed = sum(1 for y in range(h) for x in range(w) if probe_fn(pa[x, y])) if probe_fn else -1

    # coarse sector grid (8x5) of the paint density — where the change landed
    cols, rows = 8, 5
    cw, ch = w // cols, h // rows
    grid = [[0] * cols for _ in range(rows)]
    for y in range(h):
        gy = min(y // ch, rows - 1)
        for x in range(w):
            if px[x, y] > THRESH:
                grid[gy][x // cw] += 1

    print(f"paint px = {changed} ({100.0 * changed / (w * h):.2f}%)")
    for row in grid:
        print(" ".join(f"{v:6d}" for v in row))
    if probe_fn:
        print(f"probe[{opts['--probe']}] px = {probed}")

    if opts["--out"]:
        comp = before.copy()
        cpx = comp.load()
        dpx = ImageChops.difference(before, after).load()
        for y in range(h):
            for x in range(w):
                r, g, b = dpx[x, y]
                if (r + g + b) / 3 > THRESH:
                    cpx[x, y] = (255, 30, 30)
        comp.save(opts["--out"])
        print(f"composite -> {opts['--out']}")

    rc = 0
    if changed < min_change:
        print(f"FAIL paint {changed} < min-change {min_change}")
        rc = 1
    if probe_fn and probed < min_probe:
        print(f"FAIL probe[{opts['--probe']}] {probed} < min-probe {min_probe}")
        rc = 1
    if rc == 0:
        print("VISUAL OK")
    return rc


if __name__ == "__main__":
    sys.exit(main())
