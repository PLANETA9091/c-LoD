> 📷 **NEW — `gallery/run255-gallery.md`:** the run #255 pixel gallery (3 annotated sheets, 32 cells, the ~50 m render cut visible as empty sky on BOTH legs).

# CURRENT STATE (2026-11-07, v6.12 round — the walk measured)

**The leg-2 dot death is NAMED AND MEASURED. Two independent bot-dispatched instrumented runs (#253 = 42355686504, #255 = 42358652148), one verdict.**

## The verdict

- **Packet shape exonerated** — the engine's exact per-dot packet (count=0 x120 grid) renders on every arm (692/715/765 on #253; 651/775/692 on #255).
- **Carrier exonerated as the leg-difference** — happy_villager renders RAW pack-free (458/483); packed legs read 0 identically on both = the eraser's design read.
- **Eraser ALIVE on leg-2** — the capture tail is not blind.
- **VERDICT [C] + THE NUMBER: the marker band is client-culled at ≈50 m, IDENTICALLY on both legs** (v6.12 walk: leg-1 250/44/29 alive at 30/40/50 m, dead at 60+; leg-2 245/54/35, dead at 60+). The stream's dots live at 75-105 m — 15-25 m beyond the cut. The #246 353 vs #253 0 FAR flip is the radius flapping run-to-run.
- **Leg-2 scene death persists** (0/12 probes) — separate class.

## The fix (G3, parameters measured)

1. ENGINE: decode band inside the cut (ZONE_FAR 112 → ≤50) or a non-particle carrier for the LOD band (the shader path the words themselves ride).
2. CONFIRM (one-off): view-distance 4→8 moves the cut to ≈120 m — the walk's 70/80 m steps should come alive.
3. BENCH: vantage at ~40 m puts the marker inside the cut.
4. QUEUED: CLOD_UNBUNDLED=1 (bundle wrapper) — after the band renders.

Full reads: `run-42355686504-verdict.md`, `run-42358652148-verdict-walk.md`. The bot can re-run the whole path (apply → push → dispatch → triage) end-to-end.
