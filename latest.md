# c-LoD forensic addendum: run #250 — the spawn-gate suspicion rests on stale doc constants; the real discriminator is already measured (dust vs end_rod at identical coords on leg-2)

## 1. The prime suspect's arithmetic used stale constants

Task 34 and the #250 forensics locate the prime suspect via "the sky planes at Y=260/272 sit 108–120+ m from the eye — at or beyond ZONE_FAR". Y=260/272 exists ONLY in the stale module doc (stream.rs lines 8-9, predating the plane move). The live constants are `LAYER_Y: [f64; 2] = [80.0, 120.0]` (stream.rs:71 — hword rides Y=120, cword Y=80; edit-reset plane Y=100 midway, stream.rs:79).

Recomputed from the bench eye (Y=151.62, pitch 5, half-cone = ASSUMED_FOV 110/2 + margin 8 = 63°, stream.rs:112-126):

| plane | Δy from eye | in-band horiz (d≤112) | elevation at marker horiz 75 m | cone at pitch 5 (−5°±63°) |
|---|---|---|---|---|
| hword Y=120 | −31.62 m | ≤ 107.4 m | −22.9° | IN CONE |
| cword Y=80 | −71.62 m | ≤ 86.1 m | −43.7° | IN CONE |
| reset Y=100 | −51.62 m | ≤ 99.4 m | −34.5° | IN CONE |

The ENTIRE bit-stream is in-band and in-cone from the bench vantage — for both legs, identically. The stated spawn-gate suspicion ("sky planes at/beyond ZONE_FAR decide which dots flow") dissolves: there is no geometry reason for the gate to behave differently on leg-2 at the same vantage. The gate counters are still worth surfacing (per-session server state remains a live question) — but not because of plane geometry.

## 2. The gate diag exists since v5.3.1 and is statistically blind

`gate: sky {} near {} far {} cone {}` (engine.rs, since f396967 v5.3.1) prints only on spawn-phase ticks that ALSO hit `tick % 100 == 0` — but the spawn phase early-returns on empty batches (engine.rs:1281 `if batch.is_empty() { return; }` BEFORE the per-player gate loop), and in #250's full 15-minute server.log (uploaded unfiltered, ci.yml:643) the line fired ZERO times, while the `stream:` diag prints every 100 ticks unconditionally (stream_diag is called BEFORE the early return, engine.rs:1280). To surface the counters: (a) print the gate line on EVERY spawn-phase fire (drop the %100 guard — spawn fires are rare, rearm-beat cadence), or (b) aggregate per-diag-window counters into the `stream:` line.

## 3. The real discriminator is already in the #250 data

At IDENTICAL coords and spread (`$((CX - 1)) 150 $CZ 1 1 1 ... force`):

- `minecraft:end_rod` (count 1200) → **318–324 changed px** (renders);
- `minecraft:dust{color:[0,1,0],scale:1.0}` (count 600) → **0 changed px** — twice (#248: delta 0 0 0 stacked at one subpixel point, inconclusive; #250: delta 1 1 1, fair, same coords as FAR).

The bit-stream is 100% dust (`word_spawns → Spawn { y: plane_y }`, stream.rs:838-839, both layers). If dust does not RENDER on the relog client, the capture strobe reads sky at every sub-cell position, the decode keeps previous values (empty on a fresh session), and the chains paint nothing — exactly the byte-frozen scene-dead frame of every leg-2 since pre-pin #241.

Supporting existing data: the leg-2 baseline + 12 relog probes are byte-frozen (sha256-identical across #248/#250, audit marker above) even though the relog re-arm streams the last edit's 21 cells — dust dots at Y=80/120, in-band, in-cone, within the 128-block client particle cull — around the probe window. If ANY dust rendered on leg-2, the frozen frames would move. They do not.

**Unifying root-cause candidate: dust particles do not render on fresh-session (relog) clients; end_rod does.** The capture→decode→paint chain has no input by construction; the engine/harness chain work (v6.10.0/v6.10.1) was real but upstream of the break.

## 4. The missing cell: a leg-1 dust probe (never measured)

The harness fires dust only on leg-2 (run_ci_visual.sh:962). Fire the same burst on LEG-1 right after the FAR console burst and count changed vs the leg-1 baseline. Expected > 0 (leg-1's own paint argues dust+decode+chain all work there); this kills the alternative "ruler problem" reading of the leg-2 zero and confirms the per-session dust break. One run, ~4 lines.

Follow-up matrix on leg-2 (cheap, same run): dust at NEAR coords (20 m — distance dependence), white dust `color:[1,1,1]` (color/ruler artifact exclusion), happy_villager (second non-dust type — the type matrix). Mechanism candidates for a dust-specific break after the pack RELOAD: the particles.png-atlas rebuild at reload (client.log 17:49:27 "Created: 512x256x0 minecraft:textures/atlas/particles.png-atlas") hitting the dust quad path (color-uniform sprite blit) but not end_rod's; a 26.3 reversed-Z interaction specific to the dust render type (the stream doc's own caveat "26.3 reversed-Z hides particles against the sky" cannot be a blanket rule — end_rod against sky renders at the same coords).

## 5. Instrument note on the sky-plane pitch probe

With the real planes at Y=80/120, the pitch −60 shot is NOT needed for delivery-proof at the bench vantage — the stream is in-cone at pitch 5 for both legs. The pitch probe keeps only model-validation value. The decisive instrument is the leg-1 dust probe (#4).

## Bottom line

The gate/geometry suspect of Task 34 is retired (stale constants). The evidence now converges on one measurable sentence: **on a relog client, end_rod renders and dust does not — and the whole LOD bit-stream is dust.** One leg-1 dust probe (plus the optional leg-2 type matrix) splits the last alternatives and names the root cause.
