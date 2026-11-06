# c-LoD visual forensics ROADMAP

Owner: ci-verdict-bot. Updated: 2026-11-07 (round of 2026-11-07, commit after 900d42b). Every round updates this file — no monitoring-only rounds.

## Goals

### G0 — GET THE INSTRUMENTED RUN (blocking everything)
**Goal:** one CI visual run with the full instrument chain applied.
**Instrument:** `instrument-chain-full-v6.10.2f.patch` — ONE `git apply` onto `fix/leg2-arm-race` @ 35eb53b (+272 lines, verified byte-identical to the 6-patch step-by-step chain, `bash -n` green, zero rust, ~+35 s runtime, purely additive).
**Action (author):** `git apply instrument-chain-full-v6.10.2f.patch` → push → CI visual run.
**Action (bot):** watch runs/commits every round; keep the cumulative patch current with every addendum.
**Status:** OPEN — no new runs since #251 (2026-11-05), the chain is not applied yet.

### G1 — LOCALIZE THE LEG-2 DOT DEATH (the core question)
**Goal:** name the mechanism that kills the stream's dots on the relog client (dots arrive ~19-38/s for 5 min, zero render, zero capture, end_rod alive).
**Reads from the G0 run (decision trees live in the linked notes):**
- carrier cells (v6.10.2f): leg-1 villager NEAR / leg-2 villager NEAR (bfbed5c) / control villager NEAR / leg-2 in-band villager@Y=120 (eraser test);
- type × plane cells (v6.10.2e): dust@150/120/80 — the migration-candidate measurement;
- session × pack cells (bfbed5c + 0184784): leg-1 dust probes, leg-2 type matrix, the cached-pack wipe control;
- control cells (v6.10.2d): pack-free dust + villager.
**Status:** OPEN — all cells unmeasured until G0.

### G2 — THE ERASER / CAPTURE-TAIL QUESTION
**Goal:** establish whether the leg-2 client's eraser (dust_mask.fsh color net, in-band) is alive; an in-band carrier burst answers it directly on the G0 run.
**Status:** OPEN — instrumented by v6.10.2f (leg-2 villager@Y=120 in-band).

### G3 — ROOT-CAUSE FIX RECOMMENDATION
**Goal:** after G1/G2 read out, publish the engine-side fix: per the tree — dot-path/pool diag (pre-display death), capture-tail repair (eraser blind), or carrier migration (type dead; the dust candidacy is measured by the v6.10.2e cells + the v6.9.14 design).
**Status:** BLOCKED by G0/G1.

### G4 — THE FLAP (run-crossing carrier instability)
**Goal:** TESTS.md v6.9.14 documents the marker/transport flap (1631/27/0/93/2961 red px, #234-#245, BOTH legs). If G1 shows the flap is the leg-2 extreme, evaluate carrier stability options (frame-locked spawn cadence, or migration candidate from the dust cells' reading).
**Status:** OPEN — evidence exists; decision awaits G1.

## Round log (newest first)

- **2026-11-07 (this round): CORRECTION published** — the bit-stream carrier is happy_villager, NOT minecraft:dust (stream.rs: "the villager-sparkle carrier has one hue"; jni.rs: "happy_villager replaces the v5.0 dust carrier: dust quads render with degenerate UVs on every 26.3 software backend — verified live on GL llvmpipe AND Vulkan lavapipe"). All dust cells re-interpreted (documented-dead type; the "P0 migrate" rows misfire); the decode band [CULL_RADIUS=28, ZONE_FAR=112] vs the probe spots classified (NEAR 20 m out-of-band = measurable; disk 75 m in-band = eraser-covered); NEW v6.10.2f (+~44 lines): three CARRIER cells (leg-1 NEAR, control pack-free, leg-2 in-band@Y=120 eraser test) + the cumulative rebuilt (+272, byte-identical to the 6-patch chain). See correction-carrier-is-villager.md.
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
6. `instrument-addendum-villager-cells.patch` (this commit, +~44)
- **ONE-COMMAND ALTERNATIVE:** `instrument-chain-full-v6.10.2f.patch` (+272 = the whole stack, byte-verified).
