#!/usr/bin/env python3
"""triage.py — c-LoD visual forensics verdict automator (v6.11.1).

Reads a CI run's probe log (probes.txt as published on the ci-verdicts feed,
or the run summary.md — v6.11.1 parses BOTH line formats: the summary.md
'changed-white N / total M' style and the probe_counts.txt 'changed-white: N
total: M' colon style, via per-cell anchor lists), extracts every instrument
cell, evaluates the published decision rows (v6.10.0 renderer gate ->
v6.10.2f carrier/eraser -> v6.11 stream-shape -> v6.11.1 band cull) and
prints the verdict + the next action. Missing cells degrade gracefully
(pre-chain runs report exactly which cells are absent).

v6.11.1 additions (from run 42355686504, the first instrumented run):
  * VERDICT [C] — the BAND CULL: a VANILLA end_rod burst dead at the marker
    band (75-105 m) on the HEALTHY leg (l1_far <= 20 with l1_near alive)
    means nothing renders beyond the client's loaded chunk radius
    (view-distance 4 = 64 m) — the stream's dots die before any engine-side
    mechanism matters; supersedes [B] for that run (bundle question becomes
    second-order until the band renders).
  * l1_near_endrod / l1_far_endrod cells (the leg-1 burst_close/burst_far
    lines).
  * fixed the l2_scene0 parse (the anchor window's own 'leg-2' digits used
    to win; now the count after the colon is read).

Usage:
  python3 triage.py <probe_log.txt|summary.md>
  python3 triage.py --selftest
"""
import re
import sys

# cell -> (anchor substring, [regex candidates with up to 2 capture groups], names)
# Regex candidates are tried in order: the first match wins. The second
# candidate keeps pre-chain artifacts (#248/#250 era) parseable.
CELLS = {
    # anchor lists: [summary.md format, probe_counts.txt format] — first hit wins
    "l2_near_endrod":   (["leg-2 renderer probe NEAR"], [r"white px:\s*(\d+)\s+raw\s*/\s*(\d+)\s+changed", r"white px:\s*(\d+)\s+changed:\s*(\d+)"], ("raw", "changed")),
    "l2_far_endrod":    (["leg-2 renderer probe FAR"], [r"white px:\s*(\d+)\s+raw\s*/\s*(\d+)\s+changed", r"white px:\s*(\d+)\s+changed:\s*(\d+)"], ("raw", "changed")),
    "l2_greendust":     (["leg-2 carrier probe green dust (80 m): changed-green px", "leg-2 green dust changed-green px"], [r"(\d+)\s*/\s*total changed\s*(\d+)", r"changed-green px:\s*(\d+)\s+total changed:\s*(\d+)"], ("green", "total")),
    "l2_whitedust":     (["leg-2 type matrix white dust NEAR (20 m): changed-white", "leg-2 white dust NEAR changed-white"], [r"(\d+)\s*/\s*total\s*(\d+)", r"changed-white:\s*(\d+)\s+total:\s*(\d+)"], ("white", "total")),
    "l2_villager":      (["leg-2 type matrix happy_villager NEAR (20 m): changed-green", "leg-2 happy_villager NEAR changed-green"], [r"(\d+)\s*/\s*total\s*(\d+)", r"changed-green:\s*(\d+)\s+total:\s*(\d+)"], ("green", "total")),
    "l2_endrod_120":    (["leg-2 plane control end_rod at Y=120", "leg-2 end_rod@Y120 changed-white"], [r"changed-white\s*(\d+)\s*/\s*total\s*(\d+)", r"changed-white:\s*(\d+)\s+total:\s*(\d+)"], ("white", "total")),
    "l2_dust_120":      (["leg-2 plane x type cross: white dust at Y=120", "leg-2 white dust@Y120 changed-white"], [r"changed-white\s*(\d+)\s*/\s*total\s*(\d+)", r"changed-white:\s*(\d+)\s+total:\s*(\d+)"], ("white", "total")),
    "l2_dust_80":       (["leg-2 cword plane: white dust at Y=80", "leg-2 white dust@Y80 changed-white"], [r"changed-white\s*(\d+)\s*/\s*total\s*(\d+)", r"changed-white:\s*(\d+)\s+total:\s*(\d+)"], ("white", "total")),
    "l2_eraser":        (["leg-2 ERASER TEST: CARRIER (happy_villager) at Y=120 in-band", "leg-2 carrier@Y120 in-band changed-green"], [r"changed-green\s*(\d+)\s*/\s*total\s*(\d+)", r"changed-green:\s*(\d+)\s+total:\s*(\d+)"], ("green", "total")),
    "l2_endrod_stream": (["leg-2 STREAM-SHAPE end_rod", "leg-2 end_rod stream-shape changed-white"], [r"changed-white\s*(\d+)\s*/\s*total\s*(\d+)", r"changed-white:\s*(\d+)\s+total:\s*(\d+)"], ("white", "total")),
    "l2_villager_stream": (["leg-2 STREAM-SHAPE carrier", "leg-2 villager stream-shape changed-green"], [r"changed-green\s*(\d+)\s*/\s*total\s*(\d+)", r"changed-green:\s*(\d+)\s+total:\s*(\d+)"], ("green", "total")),
    "l1_near_endrod":   (["burst_close white px"], [r"(\d+)"], ("white",)),
    "l1_far_endrod":    (["burst_far white px"], [r"(\d+)"], ("white",)),
    "l1_greendust":     (["leg-1 green dust (80 m, leg-2 geometry) changed-green"], [r"(\d+)\s*/\s*total\s*(\d+)"], ("green", "total")),
    "l1_whitedust":     (["leg-1 white dust (20 m NEAR, proven-render spot) changed-white"], [r"(\d+)\s*/\s*total\s*(\d+)"], ("white", "total")),
    "l1_villager":      (["leg-1 CARRIER (happy_villager) 20 m NEAR: changed-green"], [r"(\d+)\s*/\s*total\s*(\d+)"], ("green", "total")),
    "l1_endrod_stream": (["leg-1 STREAM-SHAPE end_rod"], [r"changed-white\s*(\d+)\s*/\s*total\s*(\d+)"], ("white", "total")),
    "l1_villager_stream": (["leg-1 STREAM-SHAPE carrier"], [r"changed-green\s*(\d+)\s*/\s*total\s*(\d+)"], ("green", "total")),
    "ctl_dust":         (["control type matrix: green dust NEAR (20 m) changed-green"], [r"(\d+)\s*/\s*total\s*(\d+)"], ("green", "total")),
    "ctl_villager":     (["control CARRIER (happy_villager, pack-free"], [r"changed-green\s*(\d+)\s*/\s*total\s*(\d+)"], ("green", "total")),
    "ctl_endrod_stream": (["control STREAM-SHAPE end_rod"], [r"changed-white\s*(\d+)\s*/\s*total\s*(\d+)"], ("white", "total")),
    "ctl_villager_stream": ("control STREAM-SHAPE carrier", [r"changed-green\s*(\d+)\s*/\s*total\s*(\d+)"], ("green", "total")),
    "l2_scene0":        (["leg-2 baseline scene px (x<940):", "leg-2 baseline scene px:"], [r":\s*(\d+)"], ("scene",)),
    "flag_cached_wipe": ("leg-2 CACHED-PACK CONTROL: removed the server-pack cache", None, None),
    "flag_cached_noop": ("leg-2 cached-pack control: cache dir absent", None, None),
    "flag_scene_dead":  ("LEG-2 SCENE DEAD", None, None),
    "flag_lens_zero":   ("LENS ALL-ZERO", None, None),
    "flag_leg2_pass":   ("**LEG 2 PASS**", None, None),
    "flag_leg2_fail":   ("**FAIL:**", None, None),
}

ALIVE = 20  # the chain's convention: > 20 changed px = rendered


def parse(text):
    cells = {}
    for name, (anchor, rxs, groups) in CELLS.items():
        anchors = anchor if isinstance(anchor, (list, tuple)) else [anchor]
        i = -1
        for a in anchors:
            i = text.find(a)
            if i >= 0:
                break
        if i < 0:
            continue
        if rxs is None:
            cells[name] = "FLAG"
            continue
        window = text[i:i + 400]
        for rx in rxs:
            m = re.search(rx, window)
            if m:
                cells[name] = tuple(int(g) for g in m.groups())
                break
    # v6.12 band-edge walk: l1_band_cut / l2_band_cut = (largest distance in
    # metres whose step still rendered (> ALIVE changed-white), 0 if none).
    # Read from both line formats: 'leg-N band walk D m: changed-white N /'
    # (summary.md) and 'leg-N band walk D m changed-white: N total:' (probe
    # log).
    for leg in ("1", "2"):
        best = 0
        for m in re.finditer(rf"leg-{leg} band walk (\d+) m: changed-white:?\s*(\d+)", text):
            if int(m.group(2)) > ALIVE:
                best = max(best, int(m.group(1)))
        cells[f"l{leg}_band_cut"] = (best,)
    return cells


def fmt(v):
    if v == "FLAG":
        return "flag"
    if isinstance(v, tuple):
        return "/".join(str(x) for x in v)
    return "?"


def evaluate(cells, text=""):
    out = []
    add = out.append

    def g(name, idx):
        v = cells.get(name)
        return v[idx] if isinstance(v, tuple) and len(v) > idx else None

    if "l2_near_endrod" not in cells:
        add("VERDICT: not a leg-2 visual run (no renderer probe line) — nothing to triage")
        return out
    near = g("l2_near_endrod", 1)
    add(f"renderer sanity: leg-2 NEAR end_rod changed={near} ({'ALIVE' if near and near > ALIVE else 'DEAD'})")
    if cells.get("l2_scene0") and cells["l2_scene0"][0] == 0 or "flag_scene_dead" in cells:
        add("WARNING: LEG-2 SCENE DEAD — the relog client paints no world; all leg-2 reads score a worldless client (v6.10.0 class)")
    if "flag_lens_zero" in cells:
        add("WARNING: capture buffers all-zero — the capture chain never committed this session")

    chain_cells = ["l2_greendust", "l2_whitedust", "l2_villager", "l1_greendust",
                   "l1_whitedust", "ctl_dust", "ctl_villager"]
    missing = [c for c in chain_cells if c not in cells]
    if missing:
        v611_cells = ["l2_endrod_stream", "l2_whitedust", "l2_eraser"]
        if all(c not in cells for c in v611_cells):
            add(f"VERDICT: PRE-CHAIN RUN — instrument cells absent ({len(missing)}: {', '.join(missing[:4])}…). "
                "Apply instrument-chain-full-v6.11.patch (one git apply onto fix/leg2-arm-race @ 35eb53b) "
                "and re-run.")
        else:
            add(f"VERDICT: INCOMPLETE LOG — {len(missing)} chain cells absent from THIS log "
                f"({', '.join(missing[:4])}…). The leg-1 and control cells live in the run's summary.md "
                "files (leg summary + the control artifact's summary) — feed triage the COMBINED summary; "
                "a probe_counts.txt alone carries the leg-2 colon cells only.")
        return out

    # ---- type axis -------------------------------------------------------
    cv = g("ctl_villager", 0)
    add(f"control carrier NEAR: green={cv} -> {'carrier renders RAW (pack-free)' if cv and cv > ALIVE else 'CARRIER TYPE DEAD on this build — P0 (migration cannot reuse dust: the engine verified dust degenerate on 26.3 backends)'}")
    # ---- stream-shape axis (v6.11) ---------------------------------------
    rows = []
    for arm, es, vs in (("control", "ctl_endrod_stream", "ctl_villager_stream"),
                        ("leg-1", "l1_endrod_stream", "l1_villager_stream"),
                        ("leg-2", "l2_endrod_stream", "l2_villager_stream")):
        if es in cells:
            w = g(es, 0)
            rows.append(f"{arm}: end_rod stream-shape changed-white={w} ({'ALIVE' if w and w > ALIVE else 'dead'})"
                        + (f", carrier stream green={g(vs, 0)}" if vs in cells else ""))
    if rows:
        add("stream-shape axis (count=0 per-dot packets): " + "; ".join(rows))
    l2s = g("l2_endrod_stream", 0)
    l1s = g("l1_endrod_stream", 0)
    cts = g("ctl_endrod_stream", 0)
    # ---- band cull (v6.11.1, run 42355686504) -----------------------------
    # A VANILLA /particle end_rod burst dead at the marker band on the HEALTHY
    # leg (while the NEAR burst renders) means the marker zone sits outside
    # the client's loaded chunk radius — nothing renders there, vanilla or
    # engine, so the stream's dots die before any engine-side mechanism
    # matters. Supersedes [B] for that run.
    l1n = g("l1_near_endrod", 0)
    l1f = g("l1_far_endrod", 0)
    band_cull = (l1f is not None and l1f <= ALIVE and l1n is not None and l1n > ALIVE)
    if band_cull:
        add(f"VERDICT [C]: THE MARKER BAND IS CULLED CLIENT-SIDE — a VANILLA end_rod burst at the marker zone "
            f"(75-105 m) read {l1f} on the HEALTHY leg while the NEAR burst read {l1n}: nothing renders beyond "
            "the client's loaded chunk radius (view-distance 4 = 64 m) and the stream's dots live exactly there. "
            "Per-dot shape, carrier type and the eraser are all exonerated at NEAR — the dots die before any "
            "engine-side mechanism matters. Next: v6.12 band-edge walk (graded 30-80 m bursts) measures the exact "
            "cut radius; the engine fix is the decode band (ZONE_FAR) inside the measured cut, or a one-off "
            "view-distance=8 diagnostic arm to confirm the cull. [B] (bundle wrapper) is second-order until the "
            "band renders.")
    for leg in ("1", "2"):
        cut = cells.get(f"l{leg}_band_cut", (None,))[0]
        if cut is not None and (f"l{leg}_band_cut" in cells):
            steps = [m for m in re.finditer(rf"leg-{leg} band walk (\d+) m", text)]
            if steps:
                verdict = ("the stream's dots (75-105 m) sit OUTSIDE this radius — client-culled before any "
                           "engine path matters; the engine fix is the decode band (ZONE_FAR) inside the measured cut"
                           if cut < 75 else
                           "the dots' band (75-105 m) starts INSIDE the render radius — the band cull does NOT "
                           "explain the leg death; escalate to [B] (CLOD_UNBUNDLED=1)")
                add(f"band walk leg-{leg}: effective render cut ≈ {cut} m (view-distance 4 loads 64 m) — {verdict}")
    if l2s is not None:
        if l2s <= ALIVE and near and near > ALIVE:
            add("VERDICT [A]: THE count=0 single-dot packet path is DEAD on the relog client while bursts render — "
                "the stream's per-dot packets die BY SHAPE. P0 fix: client-side per-dot diag, then batch dots "
                "(batching MUST NOT expand positions — the decode strobes the exact 2-bit-quantum geometry).")
        elif l2s > ALIVE:
            l1v = g("l1_villager_stream", 0)
            l2v = g("l2_villager_stream", 0)
            er = g("l2_eraser", 0)
            if (l1v is not None and l2v is not None and abs(l1v - l2v) <= ALIVE
                    and er is not None and er <= ALIVE):
                if band_cull:
                    add("note: the [B] branch conditions also hold on this run (shape alive, carrier twins equal, "
                        "eraser alive) — the CLOD_UNBUNDLED=1 diagnostic stays queued BEHIND the band fix: the "
                        "bundle wrapper cannot matter while the dots' own band is client-culled.")
                else:
                    add("VERDICT [B]: packet shape, carrier type (twins equal) and the in-band eraser all ALIVE — "
                        "the remaining engine-only divergence is the ClientboundBundlePacket wrapper "
                        "(no /particle equivalent). Next: engine diagnostic mode CLOD_UNBUNDLED=1 (send the tick's "
                        "packets individually for one run).")
            else:
                add("VERDICT: shape exonerated on leg-2 — split the rest by the type/eraser cells above "
                    "(twins differ → carrier-specific; eraser > 20 → capture tail blind).")
    # ---- plane/type cross -------------------------------------------------
    e120 = g("l2_endrod_120", 0)
    d120 = g("l2_dust_120", 0)
    d80 = g("l2_dust_80", 0)
    if e120 is not None:
        add(f"plane control end_rod@Y120: white={e120} ({'planes render' if e120 > ALIVE else 'plane/depth class alive'})")
    if d120 is not None and e120 is not None and e120 > ALIVE:
        if d120 > ALIVE:
            add("dust@Y120 renders at the exact engine geometry — dust is NOT plane-sensitive on leg-2 (migration-candidate data)")
        elif d80 is not None and d80 > ALIVE:
            add("dust@Y120 ~0 but dust@Y80 renders — inconsistent; re-read the cross rows")
        else:
            add("dust ~0 at BOTH engine planes with end_rod@120 alive → the type × plane interaction is real for dust "
                "(dead as a carrier regardless — dust stays degenerate per the engine's own live verification)")
    # ---- eraser -----------------------------------------------------------
    er = g("l2_eraser", 0)
    if er is not None:
        add(f"eraser test (in-band carrier@Y120): green={er} -> {'eraser ALIVE on leg-2 (dots die pre-display if the stream still shows nothing)' if er <= ALIVE else 'CAPTURE TAIL BLIND on leg-2'}")
    # ---- legacy dust readings ---------------------------------------------
    l1g = g("l1_greendust", 0)
    gd = g("l2_greendust", 0)
    if l1g is not None and gd is not None:
        add(f"dust cells (migration candidate only — dust is the documented-dead type): leg-1 green={l1g}, leg-2 green={gd}; "
            + ("a stable green carrier EXISTS (v6.9.14 migration unblocks)" if l1g > 100 and gd > 100
               else "dust stays ~0 — the v5.0→v5.1 finding repeats, no dust migration"))
    if "flag_cached_wipe" in cells:
        add("note: the cached-pack wipe control armed this run (leg-2 re-downloads the pack)")
    if "flag_leg2_pass" in cells:
        add("note: the base harness verdict is LEG-2 PASS (marker persisted) — the cells above refine WHY leg-1 differs")
    return out


FIXTURE_PRECHAIN = """probe0 red=1117 paint=15093 lens_red=0 rot=decoded strobe_hits=0 mask_hits=0
- leg-2 renderer probe NEAR (20 m) white px: 479 raw / 479 changed-vs-baseline (> 20 = the client renders particles)
- leg-2 renderer probe FAR (80 m, marker zone) white px: 321 raw / 318 changed-vs-baseline (> 20 = no distance cut)
- leg-2 green dust changed-green px: 0 total changed: 0
"""

FIXTURE_A = FIXTURE_PRECHAIN + """
- leg-2 baseline scene px (x<940): 3182 (0 = the relog client paints no world at all)
- leg-2 carrier probe green dust (80 m): changed-green px 0 / total changed 2 vs baseline (> 100 green = a stable green carrier exists)
- leg-1 green dust (80 m, leg-2 geometry) changed-green 0 / total 4 vs predust
- leg-1 white dust (20 m NEAR, proven-render spot) changed-white 0 / total 6 vs green-dust shot
- leg-1 CARRIER (happy_villager) 20 m NEAR: changed-green 5 / total 210 vs white-dust shot
- leg-1 STREAM-SHAPE end_rod (count=0 x120 NEAR — the engine's exact per-dot packet): changed-white 743 / total 900 vs the villager NEAR shot
- leg-1 STREAM-SHAPE carrier (happy_villager count=0 x120 NEAR): changed-green 4 / total 305 vs the end_rod stream shot
- control type matrix: green dust NEAR (20 m) changed-green 0 / total 8 vs the end_rod shot
- control CARRIER (happy_villager, pack-free — no capture shaders) NEAR: changed-green 612 / total 700 vs the control dust shot
- control STREAM-SHAPE end_rod (count=0 x120 NEAR — the engine's exact per-dot packet, no expansion): changed-white 801 / total 850 vs the control villager shot
- control STREAM-SHAPE carrier (happy_villager count=0 x120, pack-free): changed-green 588 / total 640 vs the end_rod stream shot
- leg-2 type matrix white dust NEAR (20 m): changed-white 0 / total 2 vs green-dust shot
- leg-2 type matrix happy_villager NEAR (20 m): changed-green 4 / total 190 vs white-dust shot
- leg-2 plane control end_rod at Y=120 (80 m, the hword plane): changed-white 350 / total 400 vs villager shot
- leg-2 plane x type cross: white dust at Y=120 (80 m, the hword plane — the end_rod@Y120 burst with ONLY the type swapped): changed-white 0 / total 3 vs the end_rod@Y120 shot
- leg-2 cword plane: white dust at Y=80 (80 m, the cword plane, same burst geometry): changed-white 0 / total 2 vs the dust@Y120 shot
- leg-2 ERASER TEST: CARRIER (happy_villager) at Y=120 in-band (75 m, the hword plane): changed-green 2 / total 40 vs the dust@Y80 shot
- leg-2 STREAM-SHAPE end_rod (count=0 x120 NEAR — the engine's exact per-dot packet, eraser-immune): changed-white 3 / total 9 vs the villager@Y120 shot
- leg-2 STREAM-SHAPE carrier (happy_villager count=0 x120 NEAR): changed-green 0 / total 4 vs the end_rod stream shot
"""

FIXTURE_B = FIXTURE_A.replace(
    "- leg-2 STREAM-SHAPE end_rod (count=0 x120 NEAR — the engine's exact per-dot packet, eraser-immune): changed-white 3 / total 9",
    "- leg-2 STREAM-SHAPE end_rod (count=0 x120 NEAR — the engine's exact per-dot packet, eraser-immune): changed-white 704 / total 880")

# v6.11.1 — the real run 42355686504 shape: streams alive everywhere, but a
# VANILLA burst at the marker band is dead on the HEALTHY leg → VERDICT [C].
FIXTURE_C = (FIXTURE_B
             .replace("- leg-2 renderer probe FAR (80 m, marker zone) white px: 321 raw / 318 changed-vs-baseline",
                      "- leg-2 renderer probe FAR (80 m, marker zone) white px: 0 raw / 0 changed-vs-baseline")
             + """
- burst_close white px: 524 (renderer alive if > 50)
- burst_far white px: 0 (renderer alive in the marker zone if > 20)
- leg-2 end_rod@Y120 changed-white: 0 total: 0
- leg-2 white dust@Y120 changed-white: 0 total: 0
""")


def selftest():
    ok = True

    def check(name, text, needle):
        nonlocal ok
        got = "\n".join(evaluate(parse(text), text))
        if needle not in got:
            ok = False
            print(f"SELFTEST FAIL [{name}]: missing '{needle}'\n{got}")
        else:
            print(f"selftest ok [{name}]")

    check("prechain", FIXTURE_PRECHAIN, "PRE-CHAIN RUN")
    check("shape-dead-A", FIXTURE_A, "VERDICT [A]")
    check("bundle-B", FIXTURE_B, "VERDICT [B]")
    check("band-cull-C", FIXTURE_C, "VERDICT [C]")
    # v6.12: the band-edge walk names the cut — a walk dead at 60 m keeps [C]
    # with the cut reported; a walk alive to 80 m escalates to [B].
    FIXTURE_WALK = FIXTURE_C + """
- leg-1 band walk 30 m: changed-white 410 / total 430 vs the previous step (> 20 = renders)
- leg-1 band walk 40 m: changed-white 388 / total 402 vs the previous step (> 20 = renders)
- leg-1 band walk 50 m: changed-white 351 / total 370 vs the previous step (> 20 = renders)
- leg-1 band walk 60 m: changed-white 22 / total 61 vs the previous step (> 20 = renders)
- leg-1 band walk 70 m: changed-white 0 / total 8 vs the previous step (> 20 = renders)
- leg-1 band walk 80 m: changed-white 0 / total 4 vs the previous step (> 20 = renders)
"""
    check("band-walk-cut", FIXTURE_WALK, "band walk leg-1: effective render cut ≈ 60 m")
    # the v6.11.1 colon format must parse standalone (probe_counts.txt only)
    FIXTURE_COLON = """- leg-2 renderer probe NEAR white px: 562 raw / 562 changed-vs-baseline
- leg-2 renderer probe FAR white px: 0 raw / 0 changed-vs-baseline
- leg-2 white dust NEAR changed-white: 19 total: 126
- leg-2 happy_villager NEAR changed-green: 0 total: 126
- leg-2 end_rod@Y120 changed-white: 0 total: 0
- leg-2 white dust@Y120 changed-white: 0 total: 0
- leg-2 white dust@Y80 changed-white: 0 total: 0
- leg-2 carrier@Y120 in-band changed-green: 0 total: 0
- leg-2 end_rod stream-shape changed-white: 765 total: 765
- leg-2 villager stream-shape changed-green: 0 total: 765
- burst_close white px: 524 (renderer alive if > 50)
- burst_far white px: 0 (renderer alive in the marker zone if > 20)
- leg-2 baseline scene px: 0
"""
    gc = parse(FIXTURE_COLON)
    colon_expect = {
        "l2_near_endrod": (562, 562), "l2_far_endrod": (0, 0),
        "l2_whitedust": (19, 126), "l2_villager": (0, 126),
        "l2_endrod_120": (0, 0), "l2_dust_120": (0, 0), "l2_dust_80": (0, 0),
        "l2_eraser": (0, 0), "l2_endrod_stream": (765, 765),
        "l2_villager_stream": (0, 765), "l1_near_endrod": (524,),
        "l1_far_endrod": (0,), "l2_scene0": (0,),
    }
    bad = {k: (gc.get(k), v) for k, v in colon_expect.items() if gc.get(k) != v}
    if bad:
        ok = False
        print(f"SELFTEST FAIL [colon-format]: {bad}")
    else:
        print("selftest ok [colon-format]")
    print("SELFTEST:", "GREEN" if ok else "RED")
    return 0 if ok else 1


def main():
    if len(sys.argv) > 1 and sys.argv[1] == "--selftest":
        sys.exit(selftest())
    if len(sys.argv) != 2:
        print(__doc__)
        sys.exit(2)
    text = open(sys.argv[1], encoding="utf-8", errors="replace").read()
    cells = parse(text)
    print("== cells ==")
    for k in CELLS:
        if k in cells:
            print(f"  {k:22s} {fmt(cells[k])}")
    print("== verdict ==")
    for line in evaluate(cells, text):
        print(line)


if __name__ == "__main__":
    main()
