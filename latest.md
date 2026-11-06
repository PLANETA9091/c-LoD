# Instrument proposal: the leg-1 dust probe + the leg-2 type matrix (a ready harness patch)

Date: 2026-11-06. Applies to `fix/leg2-arm-race` @ 35eb53b (the latest harness state), file `scripts/bench/run_ci_visual.sh`: **+74 lines, `bash -n` green, `git apply --check` clean, zero rust touched, ~15 s of added run time.** Patch file: `instrument-proposal-leg1-dust-type-matrix.patch` (same commit). No new runs and no new commits since #251 / 35eb53b / d4ff522 (monitored 2026-11-06) — this is the missing-instrument post, not a run verdict.

## 1. Why this is the next instrument

Task 34 lists two next instruments: the gate counters in diag and the sky-plane pitch probe. The addendum above retired the pitch probe (stale doc constants — the live `LAYER_Y=[80,120]` planes are in-band and in-cone from the bench vantage on BOTH legs) and showed the gate diag is statistically blind (zero firings in #250's full unfiltered server.log: the spawn phase early-returns on empty batches BEFORE the counters, and the `%100` window almost never coincides). The one instrument that splits the remaining root-cause candidates is the **leg-1 dust probe**: the harness fires dust ONLY on leg-2 (`run_ci_visual.sh:962`), so the candidate root cause "dust does not render" has never been measured on a client that demonstrably works.

## 2. What the patch adds (4 bursts, 4 shots, 4 changed-class readings)

**leg 1** (new section 4c — after the v6.7.3 renderer preflight, before the live edit; the preflight still gates the run BEFORE any dust flies):

- `09d_leg1_predust` — a dedicated pre-burst baseline (the 10/11 pair tolerates ≤1500 px paint-wave drift; a dedicated baseline keeps the dust reading clean);
- **green dust** at the EXACT leg-2 FAR geometry (`CX-1 150 CZ`, delta `1 1 1`, count 600) → `changed_class` vs 09d — the decisive cell, never measured;
- **white dust** at the proven NEAR spot (`VX+20`, delta `0.2 0.2 0.2`, count 400 — end_rod rendered 543 raw there) → `changed_class` vs the green-dust shot — splits color-specific from type-specific when green reads 0.

**leg 2** (after the green-dust probe, before the relog probe loop):

- **white dust NEAR** (20 m) → changed vs the green-dust shot — the FIRST NEAR dust datapoint on the relog leg (every dust reading so far was FAR at 80 m, where fog lives);
- **happy_villager NEAR** (20 m) → changed vs the white-dust shot — the second non-dust type on the relog client.

Isolation rule: **successive baselines** (09d→09e→09f; greendust→whitedust→villager). A dying neighbour burst can only inflate `total` (its pixels revert to sky = an unclassified delta) — never the counted class. Eraser caveats: on leg 1 the fog-invariant eraser arm eats green excess, so the leg-1 cell carries the white-dust control; on leg 2 the tier-flip control may re-arm the chains (eraser included) — the villager reading relies on `total`.

## 3. The decision tree (one run names the root cause)

| # | leg-1 green | leg-1 white | reading |
|---|---|---|---|
| 1 | > 100 | any | dust renders on a live client → the leg-2 zero is **session-specific**: the fresh-session capture/decode path. Named mechanism candidate: the reload-time `particles.png` atlas rebuild (client.log 17:49:27, #250) vs the dust color-uniform quad blit — end_rod (textured) survives the rebuild, dust does not. Engine follow-up: rebuild/re-resolve the dust quad path on the pack `SUCCESSFULLY_LOADED` callback (the JNI hook already flagged in v6.10.1), or migrate the carrier. |
| 2 | ~0 | > 20 | the dust TYPE renders, the GREEN path does not → a **color-specific break on the live client**; the green carrier cannot work regardless of the relog — the stream must leave green dust (re-encode the color channel, e.g. white dust + sign, or end_rod). |
| 3 | ~0 | ~0 | the dust TYPE does not render on this client/build AT ALL (26.3/llvmpipe) → **P0 engine finding**: the bit-stream carrier is unrenderable (`word_spawns → Spawn`, both planes, stream.rs:838-839 — the whole stream is dust); migrate the encoding. |

With leg-1 green > 0, the leg-2 matrix refines the session-specific branch:

| # | leg-2 white NEAR | leg-2 villager | reading |
|---|---|---|---|
| 4 | > 20 | — | dust renders on the relog client NEAR → the FAR-only zero becomes a **dust-specific distance/fog/cull interaction** (end_rod FAR DID render 383/324 at the same coords — plain distance does not kill particles); the carrier geometry, not the session, is the question. |
| 5 | ~0 | — | the dust type is unrendered on the relog client (end_rod NEAR alive: 495/479) → **confirms the unifying candidate**; combined with rows 1-3 it names WHICH side of the relog the break lives on (row 1 + row 5 = the atlas-rebuild mechanism; row 3 = global). |
| 6 | — | > 20 | the break is dust-specific: two non-dust types render where dust does not. |
| 7 | — | ~0 | the relog client renders **end_rod and nothing else** — an even sharper type cut; the carrier has exactly one known-good type on leg 2. |

## 4. Costs and invariants

- The probes are **additive**: no fail/pass path, no verdict logic, no existing gate threshold is touched; raw counts stay untouched for series continuity (all new readings are changed-vs-baseline, the v6.10.0 ruler family).
- The v6.7.3 renderer preflight still gates the run BEFORE any dust flies (the leg-1 block sits after the preflight `fi`); a RENDERER DEAD abort never reaches the new probes.
- Budget: 4 bursts + 4 shots + ~6 s of sleeps ≈ **15 s of run time**; no rust, no workflow changes.

## 5. Apply

```
# on fix/leg2-arm-race (35eb53b or a descendant that does not touch the 4b/leg-2 probe sections):
git apply instrument-proposal-leg1-dust-type-matrix.patch
bash -n scripts/bench/run_ci_visual.sh
```

master @ d4ff522 does not carry the v6.10.1 arm-race control yet — apply on the leg2 branch (the active investigation), or re-apply after 6b0b599 merges.

## Bottom line

Everything measured so far says one sentence: **on a relog client end_rod renders and dust does not — and the whole LOD bit-stream is dust.** This patch adds the four readings that sentence still owes: does dust render on a live client at all (leg 1), and if yes — which side of the relog kills it (leg 2 NEAR matrix).
