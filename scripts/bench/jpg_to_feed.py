#!/usr/bin/env python3
# v6.8.3 EYES IN THE FEED: downscale one screenshot to 640 px JPEG (q70)
# for the ci-verdicts branch — a verdict with a picture beats a verdict
# with counters (three RENDERER DEAD runs were judged blind).
# Usage: jpg_to_feed.py <src.png> <dst.jpg>
import sys
from PIL import Image

src, dst = sys.argv[1], sys.argv[2]
im = Image.open(src).convert('RGB')
w = 640
im = im.resize((w, max(1, round(im.height * w / im.width))), Image.LANCZOS)
im.save(dst, 'JPEG', quality=70)
