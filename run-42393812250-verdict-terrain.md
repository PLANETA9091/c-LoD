# THE TERRAIN RANGE verdict — runs #266-#271 (feat/visible-terrain)

Owner: ci-verdict-bot. Round: the user's chain «это не лоды всё равно а я же прошу о лоды» — the
product must show a LANDSCAPE, not one free-standing column. Branch `feat/visible-terrain`
(4aa6d12 → 971aa7d → 094bbf4) off `fix/lod-visible` 3033a41; three dispatch runs
(#267 = 42388309086, #269 = 42391969533, #271 = 42393812250), all three blocked by the same
pre-existing class. Every number below is from the run artifacts on this branch.

## WHAT IS NOW STANDING (survives the blocker)

1. **The terrain range is real and streamed.** Two terraced mountains (13/9/5 stone terraces,
   grass tops, tops y=185/190) built with the marker's own vanilla forceload+fill at
   (CX-4, CZ-20) and (CX-2, CZ+16) — 66..82 m from the eye: beyond the vanilla chunk edge
   (64 m), inside the LO-plane decode cap (83.5 m from y=151.6). `/clod-cells` after the fill
   and after the probes carries the mountain cells with heights 118/155 and grass rgb
   [134,183,131] — the scanner sees them, the edit push streamed them
   (edits +5650c/960e at tick 5900), `cells=65536/65536` complete.
2. **The fallback-palette finding (G7b).** The COLOR words ride the LO plane (y=80), which is
   OFF-SCREEN from the y=151.6 eye (dy=-71.6 projects below -40°) — ALL color at this vantage
   paints through the march's fallback palette: `mix(ColLow, ColHigh, hn)` with
   ColLow=(0.35,0.52,0.28) grass-green and ColHigh=(0.48,0.44,0.40) stone-brown by height.
   This is WHY the v6.14 marker column reads dark navy, and it means the range renders
   terrain-colored WITHOUT any color-word decode. The mountains do not need the LO plane.
3. **Two new standing instruments on the branch:**
   - `terrain-green px` — the green-dominant pixel ruler on the settled baseline (the
     landscape's first-class number; flat baselines and the pack-free control read ~0);
   - **THE CUT WALK (v6.15.2)** — the run measures its own far-render reach with console
     end_rod bursts at 40/60/100 m (plus the existing 20/75 m probes): short lifetimes,
     azimuth-aligned window, per-run number in the summary.
4. **The branch is merge-clean**: zero Rust changes, zero shader changes; the probe/paint/
   silhouette rulers and the LEG gates keep their meaning (10_before_edit is taken after the
   range settles, so the live-edit diff still measures the marker only). Builds/tests/clippy/
   strictness green on every push (#266/#268/#270).

## THE BLOCKER (measured, named, and now instrumented)

**The capture-commit flap** — the author's own v6.10.0 class «fresh-session capture chain
never commits» — ate all three runs on BOTH legs: heightfield empty, paint=0 in 24/24 probes,
mask_hits=0, `terrain-green=0`, while the SERVER side streamed the whole disk and the edit
pushes landed. Three data points in one round (#267, #269, #271) next to the working #264 —
the flap, not the branch, is the variable.

The cut walk's first live measurement (#271) turned the «cut» model upside down:

| burst distance | white px | state |
|---|---|---|
| 20 m (burst_close) | 577 | rendered |
| 40 m (09w40) | 77 | rendered |
| 60 m (09w60) | **2** | DEAD |
| 75 m (burst_far) | **356** | rendered |
| 100 m (09w100) | 0 | DEAD |

**60 m dead with 40 m and 75 m alive — the far-render «cut» is NOT monotonic**, so the G4
flap is not a clean radius either. And with dots plainly on screen at 75 m (356 px) the
capture still decoded nothing: **the renderer and the capture are two separate flapping
domains.** The run names its own numbers now — a future decode failure with the walk alive
convicts the capture; with the walk dead, the render flap.

## G7a correction

The first attempt (4aa6d12, run #267) dropped the eye to y=120 to widen the band to ~102 m;
its capture death was initially blamed on the lower eye (G7a as written then). #269/#271
reproduced the same death at the PROVEN y=150 — the eye height is exonerated; the branch
keeps the y=150 geometry and the comment block records the full history for the author.

## NEXT (the unblocker, G7c)

The chain needs a CAPTURE-RETRY: at the settled baseline, if `mask_hits=0` and the
heightfield lens reads empty, kick+rejoin and re-settle ONCE (bounded, 2 tries) before the
probes — the run self-heals from the flap instead of burning 12 minutes measuring a dead
pipeline. The range is already in place; the moment any run's capture commits, the march
paints two terraced mountains where vanilla cannot render — the sheet-G composition is
waiting for that frame.

Evidence: `gallery/sheet-G-горы-и-флап.png`; artifacts `visual-evidence` of runs
42388309086 / 42391969533 / 42393812250; branch `feat/visible-terrain` (094bbf4).
