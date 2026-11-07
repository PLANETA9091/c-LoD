# CURRENT STATE (2026-11-07, v6.12 round)

**The first instrumented run happened — run 42355686504 (#253), bot-dispatched, zero author actions — and the leg-2 dot death is NAMED.**

## The verdict (full read: `run-42355686504-verdict.md`)

- **Packet shape exonerated** — the engine's exact per-dot packet (count=0 x120 grid) rendered on every arm: control 692, leg-1 715, **leg-2 765** white px.
- **Carrier exonerated as the leg-difference** — happy_villager renders RAW pack-free (458 green px); on packed legs it reads 0 on BOTH legs identically — the eraser's design read (v6.11 §4), now measured.
- **Eraser ALIVE on leg-2** (in-band carrier 0 green, twin-equal) — the capture tail is not blind.
- **VERDICT [C]: the marker band is client-culled** — a VANILLA end_rod burst at 75-105 m read **0 on the HEALTHY leg** while the same burst at 20 m read 524. The stream's dots live exactly in that band (decode [28..112], dots at 75-105 m); nothing renders beyond the loaded chunk radius (view-distance 4 = 64 m). The #246→#253 FAR flip (353 → 0) is the same radius flapping run-to-run.
- **Leg-2 scene death persists** (0/12 probes, worldless relog client) — separate class, untouched by the instruments.

## The fix (G3)

1. ENGINE: decode band inside the measured render radius — the **v6.12 band-edge walk** (six graded end_rod bursts at 30/40/50/60/70/80 m, both legs) names the exact cut; `ZONE_FAR`/dot geometry must fit inside it.
2. DIAGNOSTIC (one-off): view-distance 4→8 confirms the cull, then revert (breaks the pure-LOD premise).
3. QUEUED: `CLOD_UNBUNDLED=1` (bundle wrapper) — only if the band renders and dots still die.

## Next

Run the walk: `git apply instrument-chain-full-v6.12.patch` → dispatch → `python3 triage.py <summary.md>` prints VERDICT [C]/[B] + the cut automatically. The bot can run this path end-to-end (as it did for #253).
