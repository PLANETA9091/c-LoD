#!/usr/bin/env python3
"""PARITY GATE (v6.16) — c-LoD vs Voxy, pixel-perfect, per the user's spec:

    «сделай gallery в github ci — клиент с реальным voxy делает скрины в тех
    же координатах и местах, потом клиент с нашими lods, потом pixel perfect
    кадров всех этих, ориг и наше, и если не 100% такое же — CI красный»

Inputs:
  --voxy DIR   the voxy-reference leg's shot dir (01_aerial_west.png ...)
  --ours DIR   the c-LoD visual leg's parity dir   (parity_01_aerial_west.png
               ... — also accepted: a parity/ subdir scanned recursively)
  --out DIR    report dir: parity.md, parity.json, per-pair diff heatmaps and
               voxy|ours|diff triptychs (downscaled) for every pair

Verdict discipline:
  - a pair PASSES only at 100% pixel-exact match (every channel of every
    pixel identical);
  - a missing pair (either side failed to shoot the vantage) FAILS — a
    silent skip would fake the gate;
  - the script exits 1 (the CI job goes red) while ANY pair is below 100%.

Diagnostics per pair (the convergence work reads these):
  - exact %, near % (max |delta| <= 8 per channel — "same look, off by a
    rounding"), mean/max channel delta;
  - heatmap: amplified per-pixel max-channel delta (x4, clamped), so a
    palette/silhouette disagreement is readable at a glance;
  - triptych: voxy | ours | diff, each 640x360, one stacked strip.
"""
import argparse
import json
import os
import sys

import numpy as np
from PIL import Image

NEAR = 8          # per-channel delta still counted as "near match"
TRI_W, TRI_H = 640, 360


def find_shots(root):
    """filename (no ext) -> abs path, scanned recursively, first wins."""
    out = {}
    for dirpath, _dirs, files in os.walk(root):
        for f in files:
            if f.lower().endswith(".png"):
                out.setdefault(f[:-4], os.path.join(dirpath, f))
    return out


def pair_map(voxy, ours):
    """name -> (voxy_path, ours_path). ours side accepts both
    parity_<name>.png and bare <name>.png (the tour already prefixes)."""
    v = find_shots(voxy)
    o = find_shots(ours)
    pairs = {}
    for name in sorted(v):
        cand = o.get(f"parity_{name}") or o.get(name)
        if cand:
            pairs[name] = (v[name], cand)
    return pairs, v, o


def analyze(a_path, b_path):
    a = Image.open(a_path).convert("RGB")
    b = Image.open(b_path).convert("RGB")
    if a.size != b.size:
        b = b.resize(a.size)
    pa = np.asarray(a, dtype=np.int16)      # h,w,3
    pb = np.asarray(b, dtype=np.int16)
    d = np.abs(pa - pb)                      # per-channel abs delta
    m = d.max(axis=2)                        # per-pixel max-channel delta
    n = m.size
    exact_pct = 100.0 * float((m == 0).sum()) / n
    near_pct = 100.0 * float((m <= NEAR).sum()) / n
    mean_d = float(d.mean())
    dmax = int(m.max())
    return exact_pct, near_pct, mean_d, dmax, m, a, b


def heatmap(m):
    """amplified per-pixel delta, grayscale x4 clamped; zero-diff stays
    black, a 1-unit rounding spark stays visible at 4."""
    g = (m.astype(np.uint32) * 4).clip(0, 255).astype(np.uint8)
    return Image.fromarray(g)


def triptych(a, b, hm):
    """voxy | ours | diff at TRI scale, one stacked strip."""
    strip = Image.new("RGB", (TRI_W * 3 + 16, TRI_H + 8), (24, 28, 36))
    for k, im in enumerate((a, b, hm.convert("RGB"))):
        strip.paste(im.resize((TRI_W, TRI_H)), (k * (TRI_W + 8) + 4, 4))
    return strip


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--voxy", required=True)
    ap.add_argument("--ours", required=True)
    ap.add_argument("--out", default="parity-out")
    args = ap.parse_args()
    os.makedirs(args.out, exist_ok=True)

    pairs, vshots, oshots = pair_map(args.voxy, args.ours)
    missing = sorted(set(vshots) - set(pairs))
    orphan_ours = sorted(n for n in oshots if not (
        n.startswith("parity_") and n[7:] in vshots or n in vshots))

    rows = []
    results = {}
    for name in sorted(pairs):
        vp, op = pairs[name]
        try:
            exact_pct, near_pct, mean_d, dmax, m, a, b = analyze(vp, op)
        except Exception as e:  # corrupt shot = FAIL, never a skip
            rows.append((name, None, f"ERROR {e}"))
            results[name] = {"error": str(e)}
            continue
        heatmap(m).save(os.path.join(args.out, f"diff_{name}.png"))
        triptych(a, b, heatmap(m)).save(os.path.join(args.out, f"tri_{name}.png"))
        ok = exact_pct >= 100.0
        results[name] = {
            "exact_pct": round(exact_pct, 4),
            "near_pct": round(near_pct, 4),
            "mean_delta": round(mean_d, 3),
            "max_delta": dmax,
            "px_total": int(m.size),
            "pass": ok,
        }
        rows.append((name, exact_pct, near_pct, mean_d, dmax, ok))

    # ---- report ------------------------------------------------------------
    lines = [
        "# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)",
        "",
        f"- voxy shots: {len(vshots)}, ours shots: {len(oshots)}, "
        f"paired: {len(pairs)}, missing (voxy shot with no ours pair): "
        f"{len(missing)}",
        "- gate: 100% pixel-exact per pair; the job exits 1 while anything "
        "is below",
        "",
        "| vantage | exact % | near % (d<=8) | mean d | max d | verdict |",
        "|---|---|---|---|---|---|",
    ]
    for r in rows:
        if r[1] is None:
            lines.append(f"| {r[0]} | — | — | — | — | ERROR ({r[2]}) |")
            continue
        name, ex, nr, md, mx, ok = r
        lines.append(
            f"| {name} | {ex:.3f} | {nr:.3f} | {md:.3f} | {mx} | "
            f"{'PASS' if ok else 'FAIL'} |")
    for mm in missing:
        lines.append(f"| {mm} | — | — | — | — | FAIL (no ours shot) |")
    if orphan_ours:
        lines.append("")
        lines.append("- ours shots with no voxy pair (informational): "
                     + ", ".join(orphan_ours[:12]))
    ok_all = bool(rows) and all(
        r[5] for r in rows if r[1] is not None) and not missing
    lines += ["", "**GATE: "
              + ("GREEN — 100% parity" if ok_all else "RED — parity below 100%")
              + "**"]
    report = "\n".join(lines) + "\n"
    with open(os.path.join(args.out, "parity.md"), "w") as f:
        f.write(report)
    with open(os.path.join(args.out, "parity.json"), "w") as f:
        json.dump({"pairs": results, "missing": missing, "pass": ok_all},
                  f, indent=1)
    print(report)
    sys.exit(0 if ok_all else 1)


if __name__ == "__main__":
    main()
