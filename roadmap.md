# c-LoD visual forensics ROADMAP

Owner: ci-verdict-bot. Updated: 2026-11-07 (v6.11 round, commit after 9da8895). Every round updates this file — no monitoring-only rounds.

## Goals

### G0 — GET THE INSTRUMENTED RUN (blocking everything)
**Goal:** one CI visual run with the full instrument chain applied.
**Instrument:** `instrument-chain-full-v6.11.patch` — ONE `git apply` onto `fix/leg2-arm-race` @ 35eb53b (**+385 insertions**, verified byte-identical sha1 d93576f0 to the 7-patch step-by-step chain, `bash -n` green, zero rust, ~+65 s total runtime, purely additive).
**Action (author):** `git apply instrument-chain-full-v6.11.patch` → push → CI visual run. Then `python3 triage.py <probe log>` — the verdict is automatic.
**Action (bot):** watch runs/commits every round; keep the cumulative patch current with every addendum.
**Status:** OPEN — no new runs since #251 (2026-11-05), the chain is not applied yet.

### G1 — LOCALIZE THE LEG-2 DOT DEATH (the core question)
**Goal:** name the mechanism that kills the stream's dots on the relog client (dots arrive ~19-38/s for 5 min, zero render, zero capture, end_rod alive).
**The stage map lives in `dossier-leg2-dot-death.md`** — every pipeline stage (server gate → wire → client spawn → render → capture → eraser → display) with file:line, measured facts, and its discriminating cells; suspect table A-J; the G3 fix menu pre-positioned.
**PRIMARY OPEN SUSPECT (v6.11):** the packet-SHAPE axis — the stream sends ONE `count=0` packet PER DOT (jni.rs:1623-1648) and every burst cell sends ONE `count=1200` packet; the per-dot path was never measured. Cells: control/leg-1/leg-2 end_rod stream-shape (eraser-immune) + carrier twins.
**Reads from the G0 run (decision trees live in the linked notes; `triage.py` evaluates them automatically):**
- packet-shape cells (v6.11): the three end_rod stream-shape cells + the carrier stream twins;
- carrier cells (v6.10.2f): leg-1 villager NEAR / leg-2 villager NEAR (bfbed5c) / control villager NEAR / leg-2 in-band villager@Y=120 (eraser test);
- type × plane cells (v6.10.2e): dust@150/120/80 — the migration-candidate measurement;
- session × pack cells (bfbed5c + 0184784): leg-1 dust probes, leg-2 type matrix, the cached-pack wipe control;
- control cells (v6.10.2d): pack-free dust + villager.
**Status:** OPEN — all cells unmeasured until G0.

### G2 — THE ERASER / CAPTURE-TAIL QUESTION
**Goal:** establish whether the leg-2 client's eraser (dust_mask.fsh color net, in-band) is alive; an in-band carrier burst answers it directly on the G0 run.
**Refinement (v6.11 §4):** the color net has NO out-of-band exemption — its 2px tier erases carrier hue at EVERY vantage (dust_mask.fsh:185-220); NEAR carrier cells on packed legs measure the eraser, the carrier's renderability anchor is the control arm, and the leg-1↔leg-2 twin comparison is the valid leg-difference signal.
**Status:** OPEN — instrumented by v6.10.2f (leg-2 villager@Y=120 in-band) + the v6.11 carrier stream twins.

### G3 — ROOT-CAUSE FIX RECOMMENDATION
**Goal:** after G1/G2 read out, publish the engine-side fix. The menu is pre-positioned in `dossier-leg2-dot-death.md` §3, keyed to the reading: A (count=0 shape dead) → client per-dot diag, then batching that preserves the word geometry; B (bundle wrapper) → a CLOD_UNBUNDLED=1 diagnostic mode; C (type flap) → spawn-cadence stability or a NEW carrier (not dust); D (eraser blind) → capture-tail repair; F (planes) → move LAYER_Y / per-plane carrier.
**Status:** BLOCKED by G0/G1.

### G4 — THE FLAP (run-crossing carrier instability)
**Goal:** TESTS.md v6.9.14 documents the marker/transport flap (1631/27/0/93/2961 red px, #234-#245, BOTH legs). If G1 shows the flap is the leg-2 extreme, evaluate carrier stability options (frame-locked spawn cadence, or migration candidate from the dust cells' reading).
**Status:** OPEN — evidence exists; decision awaits G1.

## Round log (newest first)

- **2026-11-07 (this round, v6.11 + DOSSIER + AUTOMATION):** read the LIVE wire path (jni.rs build_particle_packet: ONE `count=0` packet PER DOT; bundle_send: ClientboundBundlePacket per tick) — the packet-SHAPE axis is unmeasured by every published burst cell; NEW v6.11 (+112 lines): six STREAM-SHAPE cells (control/leg-1/leg-2 × end_rod + carrier — a 12×10 grid of 120 single-packet count=0 dots via `/particle count=0`, byte-identical to the engine's per-dot packet); correction to the v6.10.2f §3 band reading: the color net's 2px tier erases carrier hue OUT-of-band too (dust_mask.fsh:185-220) — NEAR carrier cells measure the eraser, the control arm anchors renderability; DOSSIER published (dossier-leg2-dot-death.md): the full stage map S0-S6 with file:line, suspect table A-J, the G3 fix menu; AUTOMATION published (triage.py): evaluates every published decision row from a run's probe log, self-tested green (3 fixtures) + parses the real #248/#250 artifacts (pre-chain detection); cumulative rebuilt (+385, sha1-identical to the 7-patch chain). See instrument-addendum-stream-shape-cells.md, dossier-leg2-dot-death.md, triage.py.
- 2026-11-07: CORRECTION published — the bit-stream carrier is happy_villager, NOT minecraft:dust (stream.rs: "the villager-sparkle carrier has one hue"; jni.rs: "happy_villager replaces the v5.0 dust carrier: dust quads render with degenerate UVs on every 26.3 software backend — verified live on GL llvmpipe AND Vulkan lavapipe"). All dust cells re-interpreted (documented-dead type; the "P0 migrate" rows misfire); the decode band [CULL_RADIUS=28, ZONE_FAR=112] vs the probe spots classified (NEAR 20 m out-of-band = measurable; disk 75 m in-band = eraser-covered); NEW v6.10.2f (+~44 lines): three CARRIER cells (leg-1 NEAR, control pack-free, leg-2 in-band@Y=120 eraser test) + the cumulative rebuilt (+272, byte-identical to the 6-patch chain). See correction-carrier-is-villager.md.
- 2026-11-07: v6.10.2e — the plane × type cross (dust@Y=120/80) + the consolidated cumulative patch (one `git apply`). Commit 900d42b.
- 2026-11-06/07: v6.10.2c/v6.10.2d — the cached-pack wipe control + the control-arm type matrix; the packflow/bytecode evidence note (pack content exonerated). Commits 0184784, 1a0c871.
- 2026-11-06: bfbed5c — the leg-1 dust probe + leg-2 type matrix (base); 8b16c22 — the end_rod@Y=120 plane control; the wire-delivery log-mining note (transport exonerated by counters).
- 2026-11-05: #248/#250 forensics, the stale-constants correction (LAYER_Y=[80,120]), the independent audit.

## Instrument inventory (apply order)

1. `instrument-proposal-leg1-dust-type-matrix.patch` (bfbed5c, +74)
2. `instrument-addendum-endrod-planes.patch` (8b16c22, +22)
3. `instrument-addendum-cached-pack-control.patch` (0184784, +52)
4. `instrument-addendum-control-type-matrix.patch` (1a0c871, +28)
5. `instrument-addendum-plane-type-cross.patch` (900d42b, +36)
6. `instrument-addendum-villager-cells.patch` (9da8895, +~44)
7. `instrument-addendum-stream-shape-cells.patch` (this commit, +112 — the count=0 per-dot packet cells)
- **ONE-COMMAND ALTERNATIVE:** `instrument-chain-full-v6.11.patch` (+385 = the whole stack, sha1 d93576f0, byte-verified).
- **AUTOMATION:** `triage.py` — drop a run's probe log on it and read the verdict (`python3 triage.py --selftest` = green).
- **REFERENCE:** `dossier-leg2-dot-death.md` — the stage map and the suspect table behind every cell.
