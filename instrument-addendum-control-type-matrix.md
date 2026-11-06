# Instrument addendum v6.10.2d: the CONTROL TYPE MATRIX — one green dust burst on the pack-free client

Date: 2026-11-07. Companion to `run-248-250-packflow-bytecode-note.md` (pack content exonerated, suspect space reshaped). The control arm has only ever fired end_rod; dust on the pack-free client is unmeasured — and it is the cheapest discriminator left.

## The patch (+28 lines, `instrument-addendum-control-type-matrix.patch`)

Applies AFTER the full chain: base (`instrument-proposal-leg1-dust-type-matrix.patch`, bfbed5c) → `instrument-addendum-endrod-planes.patch` (8b16c22) → `instrument-addendum-cached-pack-control.patch` (0184784) → this. Apply-chain verified on 35eb53b, `bash -n` green on the full stack, zero rust. Runs only in the CONTROL branch (CLOD_CONTROL=1 — it runs in CI every visual run: both #248 and #250 shipped `visual-control-evidence`).

Insertion: after the end_rod close burst verdict, before the summary copy/exit —

- sleep 4 (let the end_rod px die; dots die in 3-5 s),
- `particle minecraft:dust{color:[0.0,1.0,0.0],scale:1.0} $((VX+20)) 150 $CZ 0.2 0.2 0.2 0 400 force` — the SAME NEAR spot that rendered 641 end_rod px in #250,
- `shot 09b_control_greendust`,
- `changed_class 09_burst_close 09b_control_greendust` → changed-green (successive baselines: a dying end_rod can only inflate total, never green),
- the verdict line names the outcome in the summary.

## Decision rows (reads on EVERY control arm, no relog needed)

| Observation | Verdict | Next |
|---|---|---|
| changed-green > 20 | **CONTROL DUST RENDERS** — dust renders WITHOUT the pack; the break needs the pack PRESENT (capture-chain × dust-pass interaction) or relog-session state | the leg-1 dust probe (base patch) splits those two |
| changed-green ~0 (with end_rod 641-class alive) | **CONTROL DUST DEAD** — dust does not render on this build/environment at all | P0: migrate the carrier type; the whole bit-stream is dust |

Combined with the main arm's cells this completes a full type × pack × session matrix in ONE run: (leg-1 dust) × (leg-2 dust/end_rod/villager/plane) × (control dust/end_rod).
