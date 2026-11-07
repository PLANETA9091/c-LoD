# Run 42364516378 (#256) — THE FIX ROUND: ZONE_FAR inside the measured cut, the transport comes ALIVE at the near vantage

**Branch:** `fix/band-inside-cut` (bot), head 2996b38 for the visual leg + cb8b55c→4a6ae66 test re-anchoring.
**Trigger:** the user's «ну так если нету лодов то ты пофискси» — the bot applied G3 menu items (1a)+(3) itself, dispatched `workflow_dispatch`, and read the artifacts. Zero author actions.

## What was applied (commit 2996b38)

1. **ENGINE — `ZONE_FAR` 112 → 48** (src/stream.rs): every dot the engine spawns now lands inside the measured ~50 m client particle cut with ≥2 m margin. The decode band [CULL_RADIUS+2, ZONE_FAR−2] = [30, 46] stays symmetric. The far cells no player can reach cost zero budget by construction; the pack-seeded static paint beyond the chunk radius is untouched (the LOD-purity proof keeps its territory).
2. **BENCH — the §5b NEAR-VANTAGE TRANSPORT PROOF** (run_ci_visual.sh): a new leg-1 pass tps to CX−30, y=100 — the plane midheight — where BOTH plane probes (y=80/120) sit at 3D 36.1 m, dead-center inside [30, 46] AND inside the measured cut. Two shots 8 s apart + lens metrics + the marker red probe, with the vanilla-chunk confound documented (at 30 m the fill blocks also render vanilla-side; the far leg keeps the purity proof).

## Measured (the visual leg of #256)

| probe | far vantage 75 m (leg 1, all 24 probes) | near vantage 30 m (§5b, NEW) |
| --- | --- | --- |
| changed px between shots | red=0 paint=0 — nothing streams (BY CONSTRUCTION, the [C] contract) | **835 px v0→v1 — the strobing stream dots RENDER** |
| capture lens | strobe_hits=0 mask_hits=0 (every probe) | **strobe_hits=2 mask_hits=30 — the capture DECODES** |
| camera channel | cam 8/11 az 0 el 3 (static) | **cam 10/11 az 0 el 4 — reads the new vantage** |
| marker red-family px | 0 (was 141 near-terrain family in #255) | **532 px — the marker IS ON SCREEN** |

The §5b shots (15_near_v0/v1.png) show the marker column FILLING the frame from 30 m, the white stream dots visible against the column top, and the LOD terrain strip at the bottom — the first on-screen proof of the full pipeline: spawn → render → decode → camera-channel → marker.

**The far vantage now reads a clean zero** — the honest contract the fix buys: the pre-fix code spawned dots the client culled on every run while the scheduler's counters reported phantom coverage (verdict [C]). The new `center_anchored_disk_streams_from_an_off_center_vantage` test pins this zero and the near-vantage alive.

## The test suite re-anchored (cb8b55c..4a6ae66)

The first push run failed 21 fixtures — the measured cut speaking through the suite, not noise: the two-plane protocol streams a cell only when BOTH plane probes sit in [30, 46] 3D, which with ZONE_FAR=48 means ONLY a midheight eye (y ~ 85..115) looking at cells 30..41 m. The fixtures moved into the envelope that physically works on this client: vhint's eye → y=100, the rearm family's settled closure → (cx−35, y=100), edit cells re-placed (66,100)→(66,83), (66,20)→(66,45), fresh_wave's strip → an exact band-wedge query (assert re-anchored from a count to the >EDIT_HOLD_MAX_BATCH wave signature), and the far-vantage-zero + near-vantage-alive contract pinned in center_anchored. Final state: 92 passed + 1 fixed (E0308 usize/u32, stale cell_origin assert) + the rustdoc blank line — see the a5aee90/f5414fc/4a6ae66 runs.

## What did NOT change

- **LEG-2 SCENE DEAD persists** (0/12 probes, the run's own FAIL verdict names the renderer-dead class of #227/#228) — a separate, pre-existing class; the band fix does not touch it.
- The band walk stays a standing instrument (every instrumented run re-measures its cut).
- `CLOD_UNBUNDLED=1` stays queued behind the band fix (G3 item 4).

## Consequences and the new goal

**G3 is EXECUTED (bot-side, menu items 1a + 3):** the engine no longer spawns into the cut, and the transport is pixel-proven alive end-to-end from a vantage the client can actually render.

**NEW G5 — THE MERGE + THE CUT-FLAP WINDOW:** the v6.4.2-era runs decoded markers at 75–105 m, which means the client cut has historically opened wider than ~50 m (the #246 flap outlier may be a wide-cut session, not an anomaly). While the cut stays ~50 m, ZONE_FAR=48 is the honest value; if the walk ever measures a wide-cut run again, ZONE_FAR can be raised in the same one-constant way. The author's merge decision on `fix/band-inside-cut` is the ball now.
