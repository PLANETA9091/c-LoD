#!/usr/bin/env python3
"""Deep-zoom a region of an e2e screenshot: <in> <out> <x0> <y0> <x1> <y1> [scale]"""
import sys
from PIL import Image

src, dst, x0, y0, x1, y1 = sys.argv[1], sys.argv[2], *map(float, sys.argv[3:7])
scale = int(sys.argv[7]) if len(sys.argv) > 7 else 6
im = Image.open(src)
w, h = im.size
box = (int(x0 * w), int(y0 * h), int(x1 * w), int(y1 * h))
crop = im.crop(box)
crop = crop.resize((crop.width * scale, crop.height * scale), Image.NEAREST)
crop.save(dst)
print(f"{dst}: {crop.size}")
