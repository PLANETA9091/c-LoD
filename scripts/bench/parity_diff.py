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
    parity_<name>.png and bare <name>.png (the tour already prefixes).
    v6.26.5: also returns srcs (name -> ours path RELATIVE to the ours
    root). #349 proved why: the ci.yml fallback resolved a wide artifact
    root, parity_diff paired the 13 voxy shots against a 226-file soup of
    debug sessions and published a binary 100/0 table nobody could read.
    Provenance turns that class self-diagnosing from the table alone."""
    v = find_shots(voxy)
    o = find_shots(ours)
    pairs = {}
    srcs = {}
    for name in sorted(v):
        cand = o.get(f"parity_{name}") or o.get(name)
        if cand:
            pairs[name] = (v[name], cand)
            srcs[name] = os.path.relpath(cand, ours)
    return pairs, v, o, srcs


DIAG_BINS = [1, 2, 4, 8, 16, 32, 64]  # delta histogram edges (px share, %)

def diagnose(pa, pb, m):
    """v6.26.1 SHADING DIAGNOSTICS riding the feed (the palette/AO session
    reads these instead of re-analyzing gallery jpgs by hand):
    - delta_hist: px share (%) per max-channel-delta bucket — separates the
      rounding class (1-8) from real content misses (8+);
    - edge_vs_flat: hot pixels (d>8) split by the VOXY luminance gradient
      (grad<3 flat vs grad>=8 edge) with the SIGNED ours-voxy luminance
      delta per class — the #336 triptych forensics showed flat ground
      near-perfect (delta 0.31) while slopes run +2.46 (ours brighter =
      missing AO darkening on concave steps); this field pins it per
      vantage every run;
    - sky_share_hot: hot-pixel share inside the top 25% of the frame —
      the sky must stay clean (v6.25 mirror lesson); a rise here means
      the march leaks above the horizon again.
    """
    h, w = m.shape
    total = h * w
    hist = {}
    edges = DIAG_BINS + [1 << 30]
    for i, lo in enumerate(DIAG_BINS):
        hi = edges[i + 1]
        hist[f"{lo}-{hi if hi < (1 << 30) else 'inf'}"] = round(
            100.0 * float(((m >= lo) & (m < hi)).sum()) / total, 3)
    lum = (pa[..., 0] * 0.299 + pa[..., 1] * 0.587 + pa[..., 2] * 0.114)
    lum_o = (pb[..., 0] * 0.299 + pb[..., 1] * 0.587 + pb[..., 2] * 0.114)
    gy, gx = np.gradient(lum)
    grad = np.hypot(gx, gy)
    hot = m > 8
    flat = (grad < 3.0) & hot
    edge = (grad >= 8.0) & hot
    signed = lum_o - lum
    edge_vs_flat = {
        "hot_total": int(hot.sum()),
        "flat_px": int(flat.sum()),
        "flat_signed": round(float(signed[flat].mean()), 2) if flat.any() else None,
        "edge_px": int(edge.sum()),
        "edge_signed": round(float(signed[edge].mean()), 2) if edge.any() else None,
        "mid_px": int((hot & (grad >= 3.0) & (grad < 8.0)).sum()),
    }
    sky = m[: h // 4, :] > 8
    sky_share_hot = round(100.0 * float(sky.sum()) / max(int(hot.sum()), 1), 3)
    # v6.26.5 CAPTURE-QUALITY: ours pure-black share. The #349 ours frames
    # were letterboxed (mini-render inside a black canvas) — a capture
    # corruption that used to read as an ordinary FAIL with mean d ~110.
    # Named here, the table itself separates "renderer misses" from
    # "capture fell apart".
    # v6.26.6 NEAR-BAND SIGNATURE: the d in [1,8] mass is the single
    # biggest class on the 2026-11-10 tables (11_topdown carries 51.8 pp
    # of near-band = exact 32 vs near 85.5) — content is RIGHT, values
    # differ by 1-8 per channel. The SIGNED luminance delta of that band
    # per gradient class (flat/edge) discriminates the mechanism from the
    # feed alone: fog-mix tint rounding drifts flat+edge together with a
    # distance-scaled bias; albedo/level-blend rounding shows on edges
    # only; a global +1 quantization shows flat near zero.
    nb = (m > 0) & (m <= 8)
    nb_flat = nb & (grad < 3.0)
    nb_edge = nb & (grad >= 8.0)
    near_band = {
        "near_px": int(nb.sum()),
        "near_flat_signed": round(float(signed[nb_flat].mean()), 2) if nb_flat.any() else None,
        "near_edge_signed": round(float(signed[nb_edge].mean()), 2) if nb_edge.any() else None,
    }
    ours_black = round(100.0 * float((pb.max(axis=2) < 8).sum())
                       / (h * w), 2)
    return {"delta_hist": hist, "edge_vs_flat": edge_vs_flat,
            "sky_share_hot": sky_share_hot, "near_band": near_band,
            "ours_black_share": ours_black}

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
    return exact_pct, near_pct, mean_d, dmax, m, a, b, diagnose(pa, pb, m)


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

    pairs, vshots, oshots, srcs = pair_map(args.voxy, args.ours)
    missing = sorted(set(vshots) - set(pairs))

    # v6.26.5 PAIRING PROVENANCE: the soup-table guard. If the ours root
    # spans several capture dirs (parity/ + parity_dbg/ + ...) or carries
    # far more shots than voxy vantages, the pairing picked arbitrary
    # frames and the table is NOT a renderer verdict (#349 class).
    src_dirs = {}
    for name, rel in srcs.items():
        d = os.path.dirname(rel) or "."
        src_dirs[d] = src_dirs.get(d, 0) + 1
    src_summary = ", ".join(f"{d or '.'} = {n}" for d, n in sorted(src_dirs.items()))
    soup = len(oshots) > 2 * len(vshots) or len(src_dirs) > 1
    orphan_ours = sorted(n for n in oshots if not (
        n.startswith("parity_") and n[7:] in vshots or n in vshots))

    rows = []
    results = {}
    for name in sorted(pairs):
        vp, op = pairs[name]
        try:
            exact_pct, near_pct, mean_d, dmax, m, a, b, diag = analyze(vp, op)
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
            "ours_src": srcs.get(name, "?"),
            "diag": diag,
        }
        # v6.26.5: name the capture-corruption class right in the verdict
        cap = ""
        if diag["ours_black_share"] > 25.0:
            cap = f" (ours capture: black {diag['ours_black_share']:.0f}%)"
        rows.append((name, exact_pct, near_pct, mean_d, dmax, ok, cap))

    # ---- report ------------------------------------------------------------
    lines = [
        "# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)",
        "",
        f"- voxy shots: {len(vshots)}, ours shots: {len(oshots)}, "
        f"paired: {len(pairs)}, missing (voxy shot with no ours pair): "
        f"{len(missing)}",
        f"- ours pair sources: {src_summary or 'none'}",
        "- gate: 100% pixel-exact per pair; the job exits 1 while anything "
        "is below",
        "",
        *( [f"**PAIRING SUSPECT: ours root spans {len(src_dirs)} dirs / "
            f"{len(oshots)} shots for {len(vshots)} voxy vantages — pairs "
            "came from mixed capture sessions (the #349 soup class); the "
            "renderer reading of this table is INVALID, fix the ours-root "
            "resolution before reading percentages**"] if soup else [] ),
        "| vantage | exact % | near % (d<=8) | mean d | max d | verdict |",
        "|---|---|---|---|---|---|",
    ]
    for r in rows:
        if r[1] is None:
            lines.append(f"| {r[0]} | — | — | — | — | ERROR ({r[2]}) |")
            continue
        name, ex, nr, md, mx, ok = r[:6]
        cap = r[6] if len(r) > 6 else ""
        lines.append(
            f"| {name} | {ex:.3f} | {nr:.3f} | {md:.3f} | {mx} | "
            f"{'PASS' if ok else 'FAIL'}{cap} |")
    for mm in missing:
        lines.append(f"| {mm} | — | — | — | — | FAIL (no ours shot) |")
    if orphan_ours:
        lines.append("")
        lines.append("- ours shots with no voxy pair (informational): "
                     + ", ".join(orphan_ours[:12]))
    ok_all = bool(rows) and all(
        r[5] for r in rows if r[1] is not None) and not missing and not soup
    lines += ["", "**GATE: "
              + ("GREEN — 100% parity" if ok_all else "RED — parity below 100%")
              + (" (pairing suspect — see banner)" if soup and not ok_all else "")
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
