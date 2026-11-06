# Instrument addendum v6.10.2e: the PLANE × TYPE CROSS — dust at the stream's OWN planes (Y=120 hword, Y=80 cword)

Date: 2026-11-07. Companion to v6.10.2b–d. The matrix still has one unmeasured interaction cell: the type matrix fires dust at **Y=150 only**, and the plane control (v6.10.2b) fires **end_rod at Y=120 only**. Nobody has ever fired **dust at the stream's own planes** — while the engine's actual dots ARE dust at `LAYER_Y=[80.0, 120.0]` (stream.rs:71: hword rides Y=120, cword Y=80; edit-reset Y=100 midway), the planes are in-band and in-cone from the bench vantage on both legs (the stale-constants note), and the whole stream is dead on leg-2. If the verdict on the rest of the matrix comes back "dust renders", it will have been measured at a Y the engine never uses.

## The patch (+36 lines, `instrument-addendum-plane-type-cross.patch`)

Applies AFTER the full 4-patch chain: base (bfbed5c) → endrod-planes (8b16c22) → cached-pack control (0184784) → control type matrix (1a0c871) → **this**. Apply-chain verified on 35eb53b, `bash -n` green on the full stack, zero rust.

Insertion: right after the end_rod@Y120 verdict line, before the relog probe loop —

- `sleep 2` + white dust at **the EXACT end_rod@Y120 geometry** (`$((CX - 1)) 120 $CZ`, delta `1 1 1`, count `1200`, `force`) — a TYPE-ONLY swap against the plane control,
- `shot 14_leg2_burst_whitedust_plane`, `changed_class 14_leg2_burst_endrod_plane → whitedust_plane` (successive baseline; a dying end_rod only inflates `total`, never white; white is eraser-immune),
- `sleep 2` + the same burst at **Y=80 — the cword plane** (the second engine plane, deepest below-horizon geometry),
- `shot 14_leg2_burst_whitedust_cword`, baseline = the dust@Y120 shot (dust dies in ~1.5 s, the +3 s cadence leaves it fully dead),
- both verdict lines name the outcome in the summary.

## Decision rows (the type × plane interaction, read on the same run)

| end_rod@120 (8b16c22) | dust@120 | dust@80 | Verdict | Next |
|---|---|---|---|---|
| > 20 | > 20 | > 20 | **PLANES FINE FOR DUST** — the client renders dust at the exact engine geometry; the dead stream points at the ENGINE side (spawn/packet/dot content), not the client renderer | engine-side diag: what is different about the stream's own Spawn packets |
| > 20 | > 20 | ~ 0 | **Y=80 SPECIFICALLY LOSES** (deepest below-horizon) | engine: drop or move the cword plane |
| > 20 | ~ 0 | ~ 0 | **TYPE × PLANE INTERACTION** if dust@150 NEAR renders (dust is plane-sensitive) — else the dust type itself is dead on leg-2 | engine: move `LAYER_Y` or a per-plane carrier |
| ~ 0 | ~ 0 | ~ 0 | **THE WHOLE PLANE DEPTH LOSES on leg-2** (the reversed-Z class, the stream doc's own caveat) | the carrier must change altitude AND type |

Combined with the rest of the chain this completes the full type × plane × pack × session matrix in ONE run: (leg-1 dust@150) × (leg-2 dust/end_rod/villager @150, end_rod@120, dust@120, dust@80) × (control dust/end_rod, pack-free).

## The zero-friction path: the consolidated chain

The stack is now 5 patches deep — **`instrument-chain-full-v6.10.2e.patch` replaces the whole stack with ONE apply** (+213 lines vs vanilla `35eb53b`, real git index lines, verified byte-identical to the 5-step chain, `bash -n` green):

```
git apply instrument-chain-full-v6.10.2e.patch   # onto fix/leg2-arm-race @ 35eb53b
bash -n scripts/bench/run_ci_visual.sh           # sanity, should be silent
```

Cost of the full stack: +213 script lines, zero rust, ~+25 s runtime, purely additive (existing verdicts unchanged; the v6.7.3 preflight still gates every probe).
