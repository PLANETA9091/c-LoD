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

### G3 — ROOT-CAUSE FIX RECOMMENDATION — **EXECUTED BY THE BOT (menu 1a + 3)**
**Goal:** publish the engine-side fix.
**Delivered (`fix/band-inside-cut`, bot-dispatched run #256 = 42364516378, see `run-42364516378-verdict-fix.md`):**
1. **DONE — `ZONE_FAR` 112 → 48** (commit 2996b38): every spawned dot now lands inside the measured ~50 m client cut with ≥2 m margin; the far-vantage zero is pinned as a regression test (the pre-fix code spawned into the cut and reported phantom coverage — verdict [C]);
2. **DONE — the §5b NEAR-VANTAGE TRANSPORT PROOF** (the bench alternative): eye at CX−30, y=100 puts both plane probes at 3D 36.1 — the #256 evidence reads **835 changed px (dots render), strobe_hits=2 (the capture decodes), marker red=532 (on screen), cam 10/11 (the camera channel lives)** — the pipeline is pixel-proven end-to-end inside the cut;
3. the one-off view-distance=8 confirmation stays available but is no longer needed — the cut is measured and the engine already fits inside it;
4. `CLOD_UNBUNDLED=1` stays queued behind the band fix (only meaningful for far-band ambitions now).
**Test suite:** 21 fixtures re-anchored to the [30, 46] servable envelope (only a midheight eye streams; the far vantage asserts ZERO by construction). Runs a5aee90/f5414fc/4a6ae66 walk the suite green (visual e2e + builds + rustfmt + strictness gate green on a5aee90; tests/clippy fixes landed in sequence).
**Status:** EXECUTED (bot-side). The author's merge decision on `fix/band-inside-cut` is the ball.

### G4 — THE FLAP (run-crossing carrier instability)
**Goal:** explain the run-crossing flips (marker red 1631/27/0/93/2961 across #234-#245; FAR end_rod 353 (#246) vs 0 (#253)).
**Measured (runs 42355686504 + 42358652148):** the cut is ≈50 m on BOTH legs of #255 — with that cut, #246's alive 73-80 m bursts are the outlier: the effective radius itself flapped on that run. The walk is now a standing instrument — every future instrumented run measures its own cut, so the flap is quantified per run (two runs so far: cut 50/50 m; #246 remains the flap outlier).
**Status:** OPEN (mechanism bounded: a flappy client radius; measurement instrument in place).

### G6 — LOD PAINT ON SCREEN — **EXECUTED (v6.14, run #264 = 42373612484)**
**Goal:** make "are the LODs visible" a first-class, per-run measured number — the user's complaint «это красный столб, а не лоды» showed the rounds were measuring everything EXCEPT the product's own output.
**Delivered (`fix/lod-visible`, f360061+v6.14.1):**
1. **FINDING:** the LOD paint was on screen the whole time — the marker-column silhouette at 75 m, 11 m beyond the vanilla chunk edge (64 m): only the clod:lod_terrain march can paint those pixels; the far frames were captioned «пустое небо» while the LOD stood in them;
2. **RULER:** `lod_silhouette()` (UI-immune run-length filter) on the baseline, every edit probe and the CONTROL arm — live run #264: pack 52 / control 0 / probe0 22, LEG 1 PASS (the only FAIL is the pre-existing LEG-2 SCENE DEAD class);
3. **RE-FRAME:** the v6.12 «~50 m client cut» is a sky-contrast artifact of the walk's ruler (the same frames show the dots against the dark column at 75 m) → `ZONE_FAR=112` is the product value; `fix/band-inside-cut` (48) = the dot-visibility demo, NOT a merge candidate.
**Status:** CLOSED (the ruler stands in every future run; the merge decision = G5's matrix).

### G5 — THE MERGE + THE CUT-FLAP WINDOW (NEW, born in the fix round)
**Goal:** land `fix/band-inside-cut` in master and keep ZONE_FAR honest while the client cut flaps.
**Why:** the v6.4.2-era runs decoded markers at 75-105 m — the cut has historically opened WIDER than ~50 m; #246's wide-cut session may be that flap, not an anomaly. While the walk measures ~50 m, ZONE_FAR=48 is the honest value; a wide-cut run re-opens the question in the same one-constant way.
**Steps:** (1) author merge decision on `fix/band-inside-cut` (4a6ae66); (2) the walk stays a standing per-run instrument — log the cut per run; (3) if a wide-cut run appears (> 105 m), re-evaluate ZONE_FAR against that evidence; (4) LEG-2 SCENE DEAD remains the separate open class (untouched by the fix).
**Status:** OPEN (the bot's part is done; the merge is the author's ball).

## Round log (newest first)

- **2026-11-07 (THE PAINT PROOF round — «так это какой то красный столб а не лоды»):** the complaint decoded: the red column = the vanilla-rendered 5×111×5 red_concrete marker stand at 30 m (a test object, sheet-E showed it); the REAL LOD output = the marker-column silhouette at 75 m beyond the 64 m chunk edge, standing in EVERY far baseline of #255 — drawn by the clod:lod_terrain march from the persistent un-fog-decoded heightfield, invisible to every previous metric. The bot: (1) branch `fix/lod-visible` off master (ZONE_FAR untouched at 112), (2) v6.14 bench instrument `lod_silhouette()` (baseline + probes + control; UI-immune ≥250 px run-length filter), (3) dispatched run **#264 = 42373612484**: pack baseline **52 columns**, control **0**, probe0 **22** (red=1357 paint=15717), LEG 1 PASS, the only FAIL the pre-existing LEG-2 SCENE DEAD class; (4) the differential closes: pack@112 52 / pack@48 0 / no-pack 0 — the bot's own v6.13 ZONE_FAR=48 regression erased the product's only visible output and is reclassified as the dot-visibility demo (not a merge candidate); the «~50 m cut» of v6.12 is a sky-contrast artifact (dots plainly visible against the dark column at 75 m). Sheet: `gallery/sheet-F-вот-лоды.png` (5 cells). Verdict: `run-42373612484-verdict-paint.md`. G6 CLOSED, G5 amended (the ZONE_FAR matrix). Monitored: master d4ff522, leg2 35eb53b unchanged.

- **2026-11-07 (THE FIX ROUND — «ну так если нету лодов то ты пофискси»):** the bot applied G3 menu items (1a)+(3) itself: branch `fix/band-inside-cut` — `ZONE_FAR` 112 → 48 (every dot spawns inside the measured ~50 m client cut; the far-vantage zero pinned as a regression test) + the §5b NEAR-VANTAGE TRANSPORT PROOF in run_ci_visual.sh (eye at CX−30, y=100: both plane probes at 3D 36.1). Dispatched `workflow_dispatch` → run **#256 = 42364516378**: **THE TRANSPORT IS ALIVE INSIDE THE CUT — 835 changed px between the near shots (the strobing dots render), strobe_hits=2 mask_hits=30 (the capture decodes), marker red=532 px (the marker IS ON SCREEN — the column fills the frame from 30 m), cam 10/11 (the camera channel reads the new vantage)**; the far vantage reads the honest zero on every probe; LEG-2 SCENE DEAD persists (separate class, untouched). The first push run failed 21 fixtures — the cut speaking through the suite; re-anchored to the [30, 46] envelope (midheight eye y=100; edits re-placed; the far-zero/near-alive contract pinned in center_anchored); three follow-ups (E0308 cast, a stale cell_origin, a rustdoc blank line) walked the suite green (visual e2e + builds + rustfmt + strictness gate green on a5aee90+). Sheet: `gallery/sheet-E-fix.png` (до/после, зум точек). Verdict: `run-42364516378-verdict-fix.md`. NEW G5 (merge + the cut-flap window). Monitored: master d4ff522, leg2 35eb53b unchanged (no author commits).

- **2026-11-07 (gallery round — the verdict made pixel-visible):** pulled the `visual-evidence` + `voxy-reference-shots` artifacts of run 42358652148 (#255) and published `gallery/` — three annotated contact sheets (A world/bench voxy pass, B leg-1, C leg-2, 32 cells total) with per-cell changed-px badges from probe_counts.txt; the 50 raw 1280x720 shots preserved in the run artifacts. The band walk now has a FACE: 30 m bursts are unmistakable blobs on both legs (250/245 px), 50 m still alive (29/35), 60 m is a 3-10 px tail, 70-80 m are photographs of EMPTY SKY on the healthy leg too — VERDICT [C] (client-culled band) is pixel-visible, effective cut ~50 m (largest alive step), dead from 60 m, on BOTH legs; the dots at 75-105 m sit 15-25 m beyond it. No new instrument needed; G3 fix menu unchanged (decode inside the cut / view-distance=8 one-off / CLOD_UNBUNDLED=1), G4 flap quantification queued. Monitored: no author commits (master d4ff522, leg2 35eb53b unchanged); runs #254/#255 are the v6.12 dispatch pair.
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
