# CORRECTION: the bit-stream carrier is happy_villager — every "dust" cell in the published matrix measures a type the engine already documented as degenerate (v6.10.2f)

Date: 2026-11-07. Found by reading the LIVE engine source on `master` (d4ff522 — the code that built #248/#250), prompted by the v6.10.2e plane work. This rewrites the interpretation layer of the whole published instrument chain. Nothing needs reverting: the patches stay additive; the decision trees get re-read as below, and v6.10.2f adds the missing CARRIER cells.

## 1. The engine source is explicit

- `src/stream.rs` (master, the `push_cell_spawns` block, lines ~836-839): `word_spawns(hword, ..., LAYER_Y[1], ...)` / `word_spawns(cword, ..., LAYER_Y[0], ...)` with the comment — *"v5.1: the HI word rides ONLY the high layer and the LO word ONLY the low layer — **the villager-sparkle carrier has one hue**, so the capture pass tells words apart by plane height."*
- `src/jni.rs` (master, `build_particle_packet`, lines ~1609-1626): *"Build ONE force **villager-sparkle** ClientboundLevelParticlesPacket … This IS the LOD transport payload (SKY-GRID BIT-STREAM). **happy_villager replaces the v5.0 dust carrier: dust quads render with degenerate UVs on every 26.3 software backend (verified live on GL llvmpipe AND Vulkan lavapipe — dust stays 1-2 px while explosions and town-sparkles render full-size).**"* The packet is `happy_villager`, `overrideLimiter=true`, `count=0`, zero drift — one dot at a point.
- `TESTS.md` v6.9.14 (CI #245 era): *"the carrier is happy_villager, and its render **FLAPS** across runs on identical geometry (the marker column measured 1631/27/0/93/2961 red px in #234-#245, **in LEG 1 as much as LEG 2**) while end_rod rendered in every single probe. A GREEN DUST burst answers whether a **stable green carrier exists** for the capture's green-hue strobe; if it proves stable, **the carrier migrates**."*

So the harness's `minecraft:dust{color:[0,1,0]}` probe (run_ci_visual.sh:933) is the **v6.9.14 carrier-MIGRATION candidate test** — and the "dust" naming in the wire notes bled from the `dust_mask.fsh` SHADER-PASS name (the eraser) into the analysis language. **The bit-stream is 100% happy_villager. "The whole bit-stream is dust" was wrong.**

## 2. What this does to the published matrix (bfbed5c → 900d42b)

| Published cell | Re-read under the true carrier |
|---|---|
| leg-1 green dust FAR (09e) + white dust NEAR (09f) | measures a **documented-degenerate type** → expected ~0/small on the HEALTHY leg too; **NOT a leg-2 discriminator** |
| leg-2 white dust NEAR (14_…whitedust) | same — expected ~0 regardless of the leg-2 anomaly |
| leg-2 green dust FAR (GD, run_ci_visual.sh:933) | the v6.9.14 migration candidate — 0 twice is the **known v5.0→v5.1 finding repeating**, not a leg-2 mystery |
| control green dust (09b, v6.10.2d) | same; "CONTROL DUST DEAD → P0" **misfires** — the carrier is already NOT dust |
| dust@Y120 / dust@Y80 (v6.10.2e) | same as above — retained as the migration-candidate × plane measurement (dust > 20 anywhere would make it MORE stable than documented = a real migration signal) |
| **leg-2 happy_villager NEAR (14_…villager, bfbed5c)** | **THE carrier cell**: 20 m NEAR is OUTSIDE the decode band → the eraser cannot eat it → a true carrier-renderability datapoint on the relog client |
| end_rod cells (all) | unchanged — the known-alive textured-sprite reference |

**The "P0: dust unrenderable → migrate the carrier" rows in the earlier decision trees must be ignored** — the migration already happened at v5.1 for exactly the reason those rows would re-discover.

## 3. The band geometry decides which villager probes are measurable

`dust_mask.fsh` (the eraser): *"the mask now merges BOTH capture rounds' fresh words … (2) a **COLOR NET marks every screen pixel inside the spawn decode band whose raw color carries the bright carrier hue** — a strobe miss cannot survive it. Outside the band no live dots exist by construction (stream.rs spawn gate)."* Constants (stream.rs): `CULL_RADIUS = 28.0`, `ZONE_FAR = 112.0`; the bench vantage sits 75 blocks from the disk center (`VX = CX - 75`).

- **NEAR probe spot (VX+20 = 20 m from the eye): OUTSIDE the band** → villager/end_rod probes render freely, the eraser never touches them.
- **The disk / marker zone (≈75 m): INSIDE the band [28, 112]** → any carrier-hue pixel there is repainted from its surroundings on a HEALTHY capture chain — the stealth eraser working as designed.
- The stream dots themselves spawn **only inside the band** (the v5.3 spawn gate) at `LAYER_Y = [80, 120]`.

Consequence: **an in-band villager burst is a PROBE OF THE ERASER, not of renderability** — on a healthy client it renders and gets erased (≈0 changed px BY DESIGN); if it reads > 20, the capture tail is blind.

## 4. The leg-2 anomaly, re-stated under the true carrier

The leg-2 client receives the stream (~19-38 carrier dots/s, fail 0) and: zero display change (12 frames byte-frozen), `mask_hits=0`, `strobe_hits=0`, while the SAME client renders console end_rod (NEAR 495/479, FAR 383/324) and its capture stage-eyes read correct. Under §1-3 the candidate set becomes:

1. **The carrier render path dies on the relog client** (the villager quad specifically — with end_rod alive in the same engine) — the flap at its extreme (TESTS.md documents 0-px marker states on BOTH legs);
2. **The dots die pre-display for dot-specific reasons** (count=0 single-particle packets vs bursts, pool accounting, dot geometry vs the packet path) — the render path itself is fine (the villager NEAR cell will show it);
3. The eraser/capture tail is NOT the hider (it reports `mask_hits=0` — it sees nothing to hide), but its aliveness is separately measurable (§5).

## 5. v6.10.2f: the three CARRIER cells (+~44 script lines, `instrument-addendum-villager-cells.patch`)

Applies AFTER the full 5-patch chain; apply-chain verified, `bash -n` green, zero rust.

| Cell | Where | Reads |
|---|---|---|
| **leg-1 villager NEAR** (09g, 20 m, out-of-band) | after the leg-1 white dust cell | the carrier on the HEALTHY client — the reference twin |
| **control villager NEAR** (09c, pack-free — NO capture shaders exist) | control arm, after the dust verdict | the carrier RAW — no pack, no eraser, nothing to hide it |
| **leg-2 villager @ Y=120 in-band** (14_…villager_plane, 75 m, the hword plane — the exact stream geometry) | after the dust@Y80 cell | the ERASER TEST: ≈0 = eraser alive on leg 2 (stream dots die pre-display); > 20 = the eraser is blind on leg 2 |

Decision rows (read together with the already-published 14_…villager NEAR cell):

- leg-2 NEAR villager > 20 **and** in-band villager ≈ 0 → eraser alive; the stream dots die pre-display → dot-geometry/pool class (§4.2); next: engine-side dot-path diag.
- leg-2 NEAR villager > 20 **and** in-band villager > 20 → eraser blind on leg 2 → the capture tail is down; any rendered dot would show — their total absence localizes death to the particle render path itself (§4.1/4.2 split by the leg-1 twin).
- leg-2 NEAR villager ≈ 0 with end_rod alive → the carrier TYPE is dead on the relog client (§4.1, the flap) → carrier must change (dust candidacy = the dust cells' reading).
- control villager ≈ 0 → the carrier is broken on this build/environment itself (P0) regardless of legs.
