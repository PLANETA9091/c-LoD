#!/usr/bin/env python3
"""A/B pixel-diff for the LOD paint proof (v5.1.0 methodology).

Compares the terrain-chain OFF shot against the ON shot, counts changed
pixels and writes a composite: OFF as the base, differences highlighted in
red. Also prints a coarse 40x22 ASCII heat grid of the diff density so the
paint zone location is obvious at a glance.
"""
import sys
from PIL import Image, ImageChops

OFF = Image.open(sys.argv[1]).convert("RGB")
ON = Image.open(sys.argv[2]).convert("RGB")
OUT = sys.argv[3]

diff = ImageChops.difference(OFF, ON)
g = diff.convert("L")
px = g.load()
w, h = g.size
changed = sum(1 for y in range(h) for x in range(w) if px[x, y] > 12)

comp = OFF.copy()
cpx = comp.load()
dpx = diff.load()
for y in range(h):
    for x in range(w):
        r, gg, b = dpx[x, y]
        if (r + gg + b) / 3 > 12:
            cpx[x, y] = (255, 30, 30)
comp.save(OUT)

# coarse heat grid (gx x gy cells, density of changed px per cell)
GX, GY = 40, 22
cw, ch = w // GX, h // GY
print(f"size={w}x{h} changed={changed} ({100.0*changed/(w*h):.2f}%)")
for gy in range(GY):
    row = ""
    for gx in range(GX):
        n = sum(
            1 for yy in range(gy*ch, (gy+1)*ch) for xx in range(gx*cw, (gx+1)*cw)
            if px[xx, yy] > 12
        )
        row += "." if n == 0 else ("-" if n < 8 else ("+" if n < 40 else "#"))
    print(row)
