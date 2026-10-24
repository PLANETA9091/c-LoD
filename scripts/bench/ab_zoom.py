#!/usr/bin/env python3
"""A/B zoom comparison of two e2e evidence dirs (same camera shots).

Usage: ab_zoom.py <dirA> <dirB> <outDir> [shot1 shot2 ...]
Defaults to the standard 8-shot matrix found in dirB. Produces per shot:
  <outDir>/<shot>_ab.png     — full frames stacked (A top / B bottom)
  <outDir>/<shot>_z<x>.png   — x8 zoom crops (A left / B right)
and prints local-outlier stats (speckle metric) per crop for both sides.
"""
import sys, os
import numpy as np
from PIL import Image

CROPS = [  # (label, x0frac, y0frac, x1frac, y1frac) — horizon band + near ground
    ("z1_horizon_l", 0.05, 0.36, 0.35, 0.52),
    ("z2_horizon_c", 0.40, 0.36, 0.70, 0.52),
    ("z3_horizon_r", 0.65, 0.36, 0.95, 0.52),
    ("z4_near",      0.35, 0.55, 0.65, 0.80),
]

def outliers(arr, thr=40):
    """Fraction of pixels whose channel deviates >thr from the 3x3 median."""
    a = arr.astype(np.int16)
    pad = np.pad(a, ((1, 1), (1, 1), (0, 0)), mode="edge")
    stack = np.stack([pad[dy:dy + a.shape[0], dx:dx + a.shape[1]]
                      for dy in range(3) for dx in range(3)])
    med = np.median(stack, axis=0)
    dev = np.abs(a - med).max(axis=2)
    return float((dev > thr).mean())

def load(path):
    im = Image.open(path).convert("RGB")
    return np.asarray(im), im

def main():
    dirA, dirB, out = sys.argv[1], sys.argv[2], sys.argv[3]
    shots = sys.argv[4:]
    if not shots:
        shots = sorted(f for f in os.listdir(dirB)
                       if f.endswith(".png") and not f.startswith(("00_", "contact")))
    os.makedirs(out, exist_ok=True)
    for shot in shots:
        pa, pb = os.path.join(dirA, shot), os.path.join(dirB, shot)
        if not (os.path.exists(pa) and os.path.exists(pb)):
            print(f"[skip] {shot}: missing in one of the dirs")
            continue
        (a, ima), (b, imb) = load(pa), load(pb)
        h = min(a.shape[0], b.shape[0]); w = min(a.shape[1], b.shape[1])
        a, b = a[:h, :w], b[:h, :w]
        # full A/B stack
        canvas = Image.new("RGB", (w, h * 2 + 4), (20, 20, 20))
        canvas.paste(ima.crop((0, 0, w, h)), (0, 0))
        canvas.paste(imb.crop((0, 0, w, h)), (0, h + 4))
        canvas.save(os.path.join(out, f"{shot[:-4]}_ab.png"))
        # crops
        for label, x0, y0, x1, y1 in CROPS:
            X0, Y0, X1, Y1 = int(x0 * w), int(y0 * h), int(x1 * w), int(y1 * h)
            ca, cb = a[Y0:Y1, X0:X1], b[Y0:Y1, X0:X1]
            Z = 8
            caZ = Image.fromarray(ca).resize((ca.shape[1] * Z, ca.shape[0] * Z), Image.NEAREST)
            cbZ = Image.fromarray(cb).resize((cb.shape[1] * Z, cb.shape[0] * Z), Image.NEAREST)
            pair = Image.new("RGB", (caZ.width * 2 + 4, caZ.height), (20, 20, 20))
            pair.paste(caZ, (0, 0)); pair.paste(cbZ, (caZ.width + 4, 0))
            pair.save(os.path.join(out, f"{shot[:-4]}_{label}.png"))
            oa, ob = outliers(ca), outliers(cb)
            diff = float((np.abs(ca.astype(np.int16) - cb.astype(np.int16)).max(axis=2) > 8).mean())
            print(f"{shot[:-4]} {label}: outliers A={oa*100:.2f}% B={ob*100:.2f}%  |pxdiff>8|={diff*100:.1f}%")
        print(f"[ok] {shot}")

if __name__ == "__main__":
    main()
