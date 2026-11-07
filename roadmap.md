# c-LoD visual forensics ROADMAP

Owner: ci-verdict-bot. Updated: 2026-11-07 (v6.12 round — G0 EXECUTED, the first instrumented run read out). Every round updates this file — no monitoring-only rounds.

## Goals

### G0 — GET THE INSTRUMENTED RUN — **RESOLVED 2026-11-07**
**Goal:** one CI visual run with the full instrument chain applied.
**How it resolved:** the bot applied `instrument-chain-full-v6.11.patch` onto 35eb53b, pushed branch `instrument/v6.11` (5b0b4f1) and dispatched `workflow_dispatch` itself — ZERO author actions. Run **42355686504 (#253)** completed in ~17 min with every build job green; the verdict feed landed `run-42355686504*` on this branch (1411ab2).
**This path is repeatable:** any new addendum can go live the same way (apply → push branch → dispatch → triage).
**Status:** CLOSED.

### G1 — LOCALIZE THE LEG-2 DOT DEATH — **ANSWERED to instrument precision**
**Goal:** name the mechanism that kills the stream's dots on the relog client.
**Answer (run 42355686504, see `run-42355686504-verdict.md`):**
- packet shape EXONERATED — the count=0 per-dot grid rendered on every arm (692/715/765) → [A] closed;
- carrier EXONERATED as the leg-difference — raw 458 (control), packed legs read identically 0 on both (the eraser's design read, v6.11 §4);
- **THE MARKER BAND IS CULLED CLIENT-SIDE ([C])** — a VANILLA end_rod burst at 75-105 m read 0 on the HEALTHY leg (NEAR 524 alive): the dots live outside the client's loaded chunk radius (view-distance 4 = 64 m); run-to-run flap of that radius explains #246's 353 vs #253's 0;
- leg-2 SCENE DEAD persists (0/12 probes) — a separate class, untouched by the chain.
**Measured (run 42358652148, the v6.12 walk):** the cut ≈ **50 m, IDENTICAL on both legs** — the dots (75-105 m) sit 15-25 m beyond it; the band death is NOT relog-specific. See `run-42358652148-verdict-walk.md`.
**Status:** ANSWERED — mechanism named AND measured.

### G2 — THE ERASER / CAPTURE-TAIL QUESTION — **ANSWERED**
**Goal:** establish whether the leg-2 client's eraser is alive.
**Answer:** ALIVE on leg-2 — in-band carrier@Y120 reads 0 green, identical to the healthy leg's read; the per-dot carrier hue is eaten at every vantage on a packed client (v6.11 §4 design read confirmed by measurement). The capture tail is not blind; dots die pre-display as designed.
**Status:** CLOSED (alive; the interesting question moved to the band).

### G3 — ROOT-CAUSE FIX RECOMMENDATION — **UNBLOCKED**
**Goal:** publish the engine-side fix.
**Menu (keyed to the run-42355686504 read):**
1. ENGINE (keeps pure-LOD): decode band inside the measured render radius — v6.12 walk names the cut; then `ZONE_FAR` (112) / dot geometry (75-105 m) must fit inside it, or re-plan the carrier geometry closer to the vantage;
2. DIAGNOSTIC (one-off): `view-distance 4 → 8` in the visual job's server.properties — a vanilla 80 m burst should render; run once to confirm the cull, then revert (view-distance=4 is deliberate — pure-LOD marker zone);
3. QUEUED BEHIND THE BAND FIX: [B] `CLOD_UNBUNDLED=1` (the ClientboundBundlePacket wrapper) — only meaningful once the band renders.
**Status:** the recommendation is written; the walk's number finalizes the parameter.

### G4 — THE FLAP (run-crossing carrier instability)
**Goal:** explain the run-crossing flips (marker red 1631/27/0/93/2961 across #234-#245; FAR end_rod 353 (#246) vs 0 (#253)).
**Measured (runs 42355686504 + 42358652148):** the cut is ≈50 m on BOTH legs of #255 — with that cut, #246's alive 73-80 m bursts are the outlier: the effective radius itself flapped on that run. The walk is now a standing instrument — every future instrumented run measures its own cut, so the flap is quantified per run (two runs so far: cut 50/50 m; #246 remains the flap outlier).
**Status:** OPEN (mechanism bounded: a flappy client radius; measurement instrument in place).

## Round log (newest first)

- **2026-11-07 (v6.12 round, part 2 — THE WALK):** the v6.12 walk dispatched the same way (branch 5e36f36 → run 42358652148/#255): **cut ≈ 50 m, IDENTICAL both legs** (leg-1 30/40/50 alive 250/44/29, 60+ dead; leg-2 245/54/35, 60+ dead) — [C] confirmed with the number, the band death is NOT relog-specific, #246's FAR alives bounded as the flap outlier, G3's fix menu finalized with measured parameters (ZONE_FAR ≤ 50 / vantage ~40 m / non-particle carrier). Full #253 replication in the same run (streams 651/775/692, carrier raw 483, eraser alive, scene dead 0/12). Verdict: `run-42358652148-verdict-walk.md`.
- **2026-11-07 (v6.12 round — THE DECISIVE ROUND):** G0 EXECUTED by the bot: applied the v6.11 cumulative onto 35eb53b, pushed `instrument/v6.11` (5b0b4f1), dispatched `workflow_dispatch` — run 42355686504 (#253) ran the full instrument stack (~17 min, build green, verdict feed landed 1411ab2). Verdict published (`run-42355686504-verdict.md`): [A] shape refuted (per-dot grid 692/715/765), carrier/eraser resolved (raw 458; packed zeros = eraser-by-design, twins equal), **VERDICT [C] — the marker band is client-culled** (vanilla burst 0 at 75-105 m on the HEALTHY leg, NEAR 524), leg-2 scene death persists (separate class), G3 fix menu unblocked. NEW instrument v6.12 (+46): the band-edge walk — six graded end_rod bursts at 30/40/50/60/70/80 m on BOTH legs name the exact render cut; cumulative rebuilt (`instrument-chain-full-v6.12.patch`, sha1 af25c1fb byte-identical to the step chain). triage.py → v6.11.1: parses BOTH log formats (summary + probe_counts colon lines), prints [C] with the demoted-[B] note, reads the walk's cut; selftest 6 fixtures GREEN. ROADMAP: G0 CLOSED, G1 answered, G2 closed, G3 unblocked, G4 quantification queued. Monitored: no author commits (master d4ff522, leg2 35eb53b unchanged).
- 2026-11-06/07 (v6.11 round): the stream-shape cells + DOSSIER + triage automation (40a3ab2); the carrier correction + v6.10.2f (9da8895); the plane × type cross + one-command cumulative (900d42b).

## Instrument inventory (apply order)

1. `instrument-proposal-leg1-dust-type-matrix.patch` (bfbed5c, +74)
2. `instrument-addendum-endrod-planes.patch` (8b16c22, +22)
3. `instrument-addendum-cached-pack-control.patch` (0184784, +52)
4. `instrument-addendum-control-type-matrix.patch` (1a0c871, +28)
5. `instrument-addendum-plane-type-cross.patch` (900d42b, +36)
6. `instrument-addendum-villager-cells.patch` (9da8895, +~44)
7. `instrument-addendum-stream-shape-cells.patch` (40a3ab2, +112 — the count=0 per-dot packet cells)
8. `instrument-addendum-band-edge-walk.patch` (this commit, +46 — the graded 30-80 m render-radius walk)
- **ONE-COMMAND ALTERNATIVE:** `instrument-chain-full-v6.12.patch` (+431 = the whole stack incl. the walk, sha1 af25c1fb, byte-verified; the older `instrument-chain-full-v6.11.patch` (+385, d93576f0) stays for history).
- **AUTOMATION:** `triage.py` (v6.11.1) — drop a run's summary.md (or probe_counts.txt) on it and read the verdict incl. VERDICT [C] and the walk's cut (`python3 triage.py --selftest` = 6 fixtures green).
- **REFERENCE:** `dossier-leg2-dot-death.md` — the stage map and the suspect table (suspects A, type, eraser now measured-resolved; [B] queued behind the band fix).
- **VERDICTS OF RECORD:** `run-42355686504-verdict.md` (the first instrumented run, the 21-cell matrix) + `run-42358652148-verdict-walk.md` (the walk: cut ≈ 50 m both legs).
