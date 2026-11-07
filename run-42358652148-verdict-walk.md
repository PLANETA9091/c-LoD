# VERDICT — run 42358652148 (#255): the BAND-EDGE WALK measured the cut — 50 m, IDENTICAL on both legs

- **Run:** 42358652148 (#255), branch `instrument/v6.11` @ 5e36f36 = the v6.11 chain + the v6.12 band-edge walk (+46 lines). Bot-dispatched (`workflow_dispatch`), ~17 min, build green, the same LEG-2 SCENE DEAD fail verdict (pre-existing class).
- **This is a full REPLICATION of run 42355686504 (#253) plus the walk measurement.** Two independent instrumented runs, one verdict.

## The walk (the round's headline numbers)

| distance | leg-1 changed-white | leg-2 changed-white | renders? |
|---|---|---|---|
| 30 m | 250 | 245 | YES both |
| 40 m | 44 | 54 | YES both (weak) |
| 50 m | 29 | 35 | YES both (weak) |
| **60 m** | **3** | **10** | **NO both** |
| 70 m | 0 | 0 | no |
| 80 m | 0 | 0 | no |

**The client's effective particle render cut ≈ 50 m — the same on the healthy leg and the relog client** (view-distance 4 loads 64 m; the vanilla particle distance fade eats the last ~10-15 m). triage.py v6.11.1 prints it automatically:
```
band walk leg-1: effective render cut ≈ 50 m — the stream's dots (75-105 m) sit OUTSIDE this radius
band walk leg-2: effective render cut ≈ 50 m — identical
```

## What the walk settles

1. **[C] CONFIRMED with the number:** the engine's dots (75-105 m) sit 15-25 m BEYOND a ~50 m cut — client-culled on EVERY run, vanilla-particle mechanism or not. The engine-side suspect queue ([B] bundle, pool, cadence) stays second-order.
2. **The cut is NOT relog-specific:** leg-1 and leg-2 die at the same radius. The leg-2 band death is the SHARED client cull; the relog client still renders particles to the same 50 m (NEAR 545 alive this run).
3. **The #246→#253 FAR flap (353 → 0) is now bounded:** with a 50 m cut, #246's alive 73-80 m bursts are the outlier — the effective radius itself flapped on that run (G4's measurement target; repeat walks on future runs quantify it).
4. **Everything else replicated:** streams alive (control 651 / leg-1 775 / leg-2 692), carrier raw 483 (control) vs packed-twin zeros, eraser alive, dust dead as documented, leg-2 scene dead 0/12, LEG 1 PASS.

## The finalized fix menu (G3 — the numbers are in)

1. **ENGINE (product fix, keeps pure-LOD):** the live-edit decode channel CANNOT be vanilla particles in the LOD band — the cut (≈50 m, flappy up to ~64 m) is a client constant the engine does not control at view-distance 4. Options: (a) move the decode band inside the cut (`ZONE_FAR` 112 → ≤50, and the dot geometry with it), or (b) re-plan the carrier to a mechanism that survives unloaded chunks (the same shader path the words themselves ride).
2. **CONFIRMATION DIAGNOSTIC (one-off):** view-distance 4 → 8 should move the cut to ≈120 m — the walk's 70/80 m steps would come alive; run once to confirm, then revert.
3. **BENCH ALTERNATIVE (harness, 2 lines):** vantage at ~40-45 m from the marker puts the marker cells inside the cut — the decode channel works and LEG 1's paint proof measures the real path.
4. **QUEUED:** [B] `CLOD_UNBUNDLED=1` — only after the band renders.

## Bottom line

The three-round arc is closed: the type matrix and the stream-shape cells exonerated the packet path (shape, carrier, eraser), and the walk measured the ONE mechanism left standing — **a ~50 m client particle cut that the stream's 75-105 m dots never survive.** The engine fix is now a design decision with measured parameters, not a search.
