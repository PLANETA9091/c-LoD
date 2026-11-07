# Visual gallery — run #255 (42358652148, head 5e36f36), 2026-11-07

The first pixel-level presentation of the instrument stack's output. Three annotated
contact sheets built from the `visual-evidence` + `voxy-reference-shots` artifacts;
every tile carries its `changed`-px badge from `probe_counts.txt` (green = alive > 20,
grey = empty). The 50 raw 1280×720 shots live in the run artifacts; sheets are the
curated view.

## Sheets

| sheet | file | what it shows |
|---|---|---|
| A | `sheet-A-world.png` | the world & bench (voxy pass): aerial views, oblique, ground view of the marker funnel, inside the funnel, after-mesh state |
| B | `sheet-B-leg1.png` | LEG-1 (healthy client, no relog): 16 cells — type matrix at 20/80 m, both stream-shape grids, the 30→80 m band walk, the live-edit diff (red column = paint) |
| C | `sheet-C-leg2.png` | LEG-2 (relog client, pack cache wiped → re-downloaded): 16 cells — same matrix, the plane × type cross, the decisive stream-shape cell, the band walk, the pre-tier-flip dead scene |

## How the scene reads (any 1280×720 shot)

- **Centre column** — the LOD-painted stream column (dark navy) with the dot
  speckle inside; bursts land on it.
- **Right, top** — the dark-red reference panel (marker band, `red_concrete`
  fill 5×111×5 at -930,796).
- **Right, bottom** — the calibration chart (thin colour strip on dark ground).
- **B16/C16 diff tiles** — red column = the live edit actually painted (leg-1)
  and the relog delta (leg-2).

## The numbers the sheets carry (probe_counts.txt)

| cell | leg-1 | leg-2 | read |
|---|---|---|---|
| end_rod 20 m | 535 | 545 | renderer alive on BOTH legs |
| end_rod 80 m (marker zone) | 0 | 0 | nothing renders at the marker distance |
| green dust 80/20 m | 0 | 0 | dust type documented-dead (v6.10.2f) — not a leg discriminator |
| white dust 20 m | 11 | 14 | trace-only |
| happy_villager 20 m | 0 | 0 | eraser 2-px colour net, by design, BOTH legs |
| end_rod @Y120 / dust @Y120 / dust @Y80 | — | 0 / 0 / 0 | plane × type cross: no plane-specific loss measurable |
| STREAM-SHAPE end_rod count=0 ×120 | 775 | **692** | per-dot wire shape renders fine on BOTH legs — [A] stays refuted |
| STREAM-SHAPE villager ×120 | 0 | 0 | net eats the carrier hue, both legs |
| band walk 30 m | 250 | 245 | unmistakable blob |
| band walk 40 m | 44 | 54 | alive |
| band walk 50 m | 29 | 35 | alive |
| band walk 60 m | **3** | **10** | tail |
| band walk 70 m | 0 | 0 | EMPTY SKY |
| band walk 80 m | 0 | 0 | EMPTY SKY |

**The effective cut is ~50 m on BOTH legs** (largest alive step; dead from 60 m —
the 3-10 px tail at 60 m is the boundary blur). The stream's dots live at 75–105 m,
15–25 m beyond the cut: VERDICT [C] (the marker band is client-culled) is now
pixel-visible — C14 is a photograph of empty sky where the walk's burst should be.
(64 m is the nominal view-distance-4 chunk radius; the walk reads the EFFECTIVE
radius, which itself flaps run-to-run — see G4.)

## Consequences (G3 fix menu, unchanged)

1. decode the band inside the measured cut (move the payload under 60–65 m), or
2. one-off `view-distance=8` diagnostic run (does the cut follow the radius?),
3. `CLOD_UNBUNDLED=1` queued behind the band fix.

G4 (the run-crossing flap) stays queued; leg-2 scene death (0/12) remains a
separate class.
