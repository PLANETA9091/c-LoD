# THE PARITY GATE verdict — runs #275/#276/#278/#281 (v6.16..v6.16.5)

**The user's spec, verbatim:** «сделай gallery в github ci — клиент с реальным voxy делает скрины
в тех же координатах и местах, потом клиент с нашими lods, потом pixel perfect кадров всех этих,
ориг и наше, и если не 100% такое же — CI красный, всё сохраняется в artifacts, и ты потом это
смотришь и делаешь так чтобы выглядело и функционировало так же само».

**DELIVERED — the pipeline exists, runs, gates and feeds:**

1. **THE SHARED MATRIX** (`scripts/bench/parity_vantages.txt`) — one file, both legs: rows 01-09 =
   the proven v6.8 voxy matrix, row 10 = the marker-ray paint-proof vantage (CX−75, 151.6, yaw 270).
   CX/CZ tokens substituted per leg; arithmetic evaluation (v6.16.1 — «CX-75» must BECOME −1005).
2. **THE VOXY LEG** — `run_voxy_shots.sh` walks the matrix (the 09_after_mesh lap re-shot kept).
3. **OUR LEG** — `run_ci_visual.sh` §5b PARITY TOUR: the session-1 client shoots the same rows into
   `shots/parity/` as `parity_<name>.png` (10 s settle per vantage, non-fatal by design).
4. **THE DIFFER** (`parity_diff.py`) — pairs BY NAME, pixel-exact % + near % (d≤8) + mean/max delta
   per pair, amplified diff heatmaps, voxy|ours|diff triptychs, `parity.md`/`parity.json`;
   **exit 1 while ANY pair is below 100%** — a missing pair is a FAIL, never a skip.
5. **THE GATE JOB** (`ci.yml` parity-gate, needs visual+voxy-reference) — downloads both artifacts,
   runs the differ, RED below 100%, uploads `parity-gallery`, publishes the feed.
6. **THE FEED** (`publish_parity_feed.sh`) — `gallery/parity-run-<N>/` on ci-verdicts
   (ci-verdict-bot identity, newest-5 prune, own `gallery/parity-index.md`; never touches the
   verdict feed's `latest.md` or the hand-built sheets). First feed commit: **e33758b** (#276).

## The four instrument bugs the gate itself caught (each fixed same-round)

| bug | evidence | fix |
|---|---|---|
| THE STDIN EATER: ffmpeg inside the matrix `while read` loop consumed the loop's stdin — one byte of every next row swallowed, every tour shot after the first lost a leading char (`10_marker_ray`→`0_marker_ray`) in BOTH legs | #275 report: mangled names, voxy 11 shots | `ffmpeg -nostdin` in both `shot()`s + tours read on fd 3 (v6.16.2) |
| THE GEOMETRY PARITY: the voxy window was 640×360 vs our 1280×720 — every reference frame letterboxed | #275 tri_01: black desktop border | `--width 1280 --height 720` (v6.16.2) |
| THE FEED 403: the gate job had no `permissions: contents: write` | #275 log: `fatal: ... 403` | permissions block (v6.16.2) |
| THE TIME-OF-DAY PARITY: the voxy leg locked `time set 6000` = NIGHT in the 26.3 scale (the v6.4.8 strobe lesson, never applied to the reference leg); then even `noon` drifted through `doDaylightCycle false` over the 6.6-minute leg | #276/#278 tri_09: voxy night forest + stars vs our noon sky | v6.16.3 `time set noon` + v6.16.5 per-shot pinning in BOTH legs (the clock does NOT hold — our own leg re-issues noon 8× per run for exactly this reason) |

Also: v6.16.4 — the gate now requires both needs non-skipped; on push it compared two nonexistent
artifacts and reddened the author's green push lane (run #279). Push runs are green again (#280/#282).

## The first honest baseline (#281 = 42404167317, sha 695f18a, all-nova noon)

10/10 pairs, 0 missing, **GATE: RED** — the numbers (exact % / near % (d≤8) / mean Δ):

| vantage | exact | near | mean Δ | note |
|---|---|---|---|---|
| 01 aerial west | 0.063 | 27.3 | 61.2 | voxy sky-empty here; ours slope+panels+dust |
| 02 aerial south | 0.034 | 21.0 | 99.7 | ours paints RAINBOW DUST COLUMNS from above — the transport is visible |
| 03 aerial north | 0.038 | 24.8 | 85.2 | |
| 04 aerial east | 0.037 | 16.5 | 94.8 | dust columns again |
| 05 oblique | 0.002 | 21.6 | 88.2 | ours: the huge red marker panel fills the frame |
| 06 ground west | 0.129 | 7.4 | 94.9 | dust columns from ground level |
| 07 far north | 0.274 | **70.9** | **4.5** | sky-only pair — PROOF the differ reads honest sameness |
| 08 back inside | 0.050 | 21.4 | 68.4 | |
| 09 after mesh | 0.063 | 24.0 | **74.6** | was 144.9 at night — the time fix halves the gap |
| 10 marker ray | 0.075 | 34.6 | 62.1 | **THE TARGET PAIR: voxy = real noon landscape (forest+lake), ours = sky+dust+panels** |

## What the gate says (the convergence queue, G8)

1. **OUR SIDE MUST PAINT** — the capture-commit flap still starves the heightfield (no LOD terrain
   at ANY vantage; the earlier forensics: the mountain rescan backlog re-dirties cells and the
   recent-edit priority scan caps at EDIT_HOLD_MAX_BATCH, filling with backlog before the marker's
   cells are reached). Until paint is alive, every pair is sky-vs-landscape.
2. **THE TRANSPORT MUST BE INVISIBLE** — voxy's frames have NO dust; ours carries rainbow columns
   and the ghost blobs (02/04/06/10). The v5.3.1 stealth gate works from the side; from ABOVE the
   columns are plain. Parity demands the dots never enter the USER's frame.
3. **THE DEBUG LENS IS OURS ALONE** — the dark-red strip + black panel ride every our-frame; voxy
   has none. Either the parity tour shoots the production (lens-free) pack, or the differ gains a
   documented lens-region exception.
4. **THEN THE LOOK** — palette/SSAO/fog/fade already ported (docs/voxy-visual-model.md); what
   remains is geometry (heightfield march vs block quads) and light (reconstructed vs live lightmap).

**Verdict:** THE GATE IS REAL — red exactly as ordered, measuring line-by-line, feeding artifacts
and the branch feed every run. Convergence is now a measurable number per vantage per run.

**Artifacts:** `parity-gallery` (per run) · feed: `gallery/parity-run-276/` (e33758b), `gallery/parity-run-278/`,
`gallery/parity-run-281/` · sheet: `gallery/sheet-H-паритет-гейт.png` (this branch).
