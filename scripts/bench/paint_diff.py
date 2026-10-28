#!/usr/bin/env python3
"""Paint metric for c-LoD e2e: how many screen pixels the LOD post-chain
changed between ON and OFF shots.

A pixel counts as painted when ANY channel differs by more than THRESH
(15 — the value the v5.2.1 sessions used; night paint on dark sky stays
below it, so always `time set 6000` before shots). Prints the count and a
coarse 8x5 sector map so coverage MOVEMENT is visible between legs.

Usage: paint_diff.py ON.png OFF.png [reveal.png]
"""
import sys
from PIL import Image

THRESH = 15


def load(p):
    return Image.open(p).convert("RGB")


def diff_map(a, b):
    w, h = a.size
    if b.size != a.size:
        b = b.resize(a.size)
    pa, pb = a.load(), b.load()
    total = 0
    cols, rows = 8, 5
    grid = [[0] * cols for _ in range(rows)]
    cw, ch = w // cols, h // rows
    for y in range(h):
        gy = min(y // ch, rows - 1)
        for x in range(w):
            ca, cb = pa[x, y], pb[x, y]
            if (abs(ca[0] - cb[0]) > THRESH or abs(ca[1] - cb[1]) > THRESH
                    or abs(ca[2] - cb[2]) > THRESH):
                total += 1
                grid[gy][min(x // cw, cols - 1)] += 1
    return total, grid


def main():
    if len(sys.argv) < 3:
        print(__doc__)
        sys.exit(1)
    a, b = load(sys.argv[1]), load(sys.argv[2])
    total, grid = diff_map(a, b)
    print(f"paint px = {total}")
    for row in grid:
        print(" ".join(f"{v:6d}" for v in row))
    if len(sys.argv) > 3:
        # reveal mode: painted pixels inside the reveal frame itself
        r = load(sys.argv[3])
        if r.size != a.size:
            r = r.resize(a.size)
        pr = r.load()
        pa = a.load()
        rev = sum(
            1 for y in range(a.size[1]) for x in range(a.size[0])
            if (lambda c, d: abs(c[0] - d[0]) > THRESH or abs(c[1] - d[1]) > THRESH
                or abs(c[2] - d[2]) > THRESH)(pr[x, y], pa[x, y])
        )
        print(f"reveal px vs ON = {rev}")


if __name__ == "__main__":
    main()
