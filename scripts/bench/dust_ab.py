#!/usr/bin/env python3
"""v5.3.2 dust A/B metric: how much carrier reaches the eye.

Usage: dust_ab.py <ON.png> <OFF.png> [out.txt]

ON  = frame with the clod:lod_terrain chain (eraser active)
OFF = same camera, chain removed (raw frame, dust visible)

Metrics:
  changed      — pixels differing between OFF and ON (dust erase footprint)
  green_off    — OFF pixels passing the raw carrier hue test (stream alive)
  green_on     — ON pixels passing the same test (the residual class)
  resid_ratio  — green_on / max(green_off, 1)
The carrier hue test (v5.3.2): the villager sprite is CHROMA-green with a
very low red channel — full-strength texels measure r <= 79 while g >= 153
(g - r >= 60); natural foliage (98,127,25) and grass (102,153,64) both have
g - r < 55 and are excluded. Blended-over-sky cores (82,170,143) still pass.
"""
import sys
from PIL import Image


def load(p):
    return Image.open(p).convert("RGB")


def green_mask(im):
    w, h = im.size
    px = im.load()
    out = bytearray(w * h)
    for y in range(h):
        for x in range(w):
            r, g, b = px[x, y]
            if g > 115 and g > r + 60 and g > b + 25:
                out[y * w + x] = 1
    return out


def main():
    on, off = load(sys.argv[1]), load(sys.argv[2])
    if on.size != off.size:
        print("size mismatch", on.size, off.size)
        return 2
    w, h = on.size
    po, pff = on.load(), off.load()
    changed = green_on = green_off = 0
    mo, mf = green_mask(on), green_mask(off)
    for i in range(w * h):
        x, y = i % w, i // w
        a, b = po[x, y], pff[x, y]
        if abs(a[0] - b[0]) + abs(a[1] - b[1]) + abs(a[2] - b[2]) > 24:
            changed += 1
        green_on += mo[i]
        green_off += mf[i]
    ratio = green_on / max(green_off, 1)
    lines = [
        f"frame {w}x{h}",
        f"changed   {changed} ({100.0 * changed / (w * h):.4f}%)",
        f"green_off {green_off}",
        f"green_on  {green_on}",
        f"resid     {100.0 * ratio:.4f}%",
    ]
    text = "\n".join(lines)
    print(text)
    if len(sys.argv) > 3:
        with open(sys.argv[3], "w") as f:
            f.write(text + "\n")
    return 0


if __name__ == "__main__":
    sys.exit(main())
