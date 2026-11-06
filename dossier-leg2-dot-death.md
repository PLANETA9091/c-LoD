# DOSSIER: the leg-2 dot death — the full stage map of the LOD transport, measured facts pinned per stage, every remaining suspect bound to its discriminating cell

Date: 2026-11-07. Sources read LIVE on master d4ff522 (the code that built #248/#250): `src/stream.rs`, `src/jni.rs`, `assets/glsl/{capture,dust_mask,lod_terrain,col_scan,rot_solve,clod_blit}.fsh`, `scripts/bench/run_ci_visual.sh` (+ the v6.11 chain), plus the published forensics (#246-#251, run-42063218460/42076419845 artifacts on this feed). Goal: one page that maps EVERY stage from the server's spawn decision to the client's displayed pixel — what kills it, how we know, and which instrument cell discriminates the remainder. Feeds G1/G3 of `roadmap.md`.

## 0. The anomaly, stated once

Leg-2 (the relog client) receives the stream continuously (+189..378 gate-passed carrier dots per 100-tick window, fail 0, ~5 min, both #248/#250) and renders NONE of it: 12 relog frames byte-frozen, `strobe_hits=0`, `mask_hits=0`, while the SAME client renders console `end_rod` bursts (NEAR 495/479, FAR 383/324 changed px) and its capture stage-eyes decode correct (rot/cam/az/el all read). Leg-1 decodes and paints in the same runs (red=1117/1183).

## 1. The stage map

**S0 — server spawn gate** (`stream.rs`):
- `spawn_visible` (149-168): a dot spawns to a player only if (1) `sky_visible` — even the frustum TOP edge clears the horizon (`SKY_PITCH_LIMIT_DEG = 47.0`, line 122; Bukkit pitch synced every tick), (2) 3D eye distance inside the decode band `(CULL_RADIUS=28, ZONE_FAR=112]` (92/102/163), (3) inside the frustum cone `cos_half = fov/2 + margin` (125-128, 167).
- `push_cell_spawns` (821-839): per cell per lap — `word_spawns(hword, …, LAYER_Y[1]=120)` + `word_spawns(cword, …, LAYER_Y[0]=80)`; `LAYER_Y = [80.0, 120.0]` (71). `word_spawns` (734) expands one 16-bit word into 16 sub-particle positions (4 slots × 2-bit X quantum), geometry the client shaders MUST mirror exactly.
- Measured: the #248/#250 wire counters count dots AFTER this gate — leg-2 got 189-378/window continuously → **S0 EXONERATED by counters** (the gate passed the dots through on the relog session too). The gate-diag blindness note (stale doc constants; gate log only fires on tick%100) stands as an observability gap, not a suspect.

**S1 — wire** (`jni.rs`):
- `build_particle_packet` (1623-1648): one packet per dot — `happy_villager`, `overrideLimiter=true`, `alwaysShow=true`, delta 0/0/0, speed 0, **count=0** ("exactly one particle at the point, zero drift").
- `bundle_send` (1700+): the tick's batch rides ONE `ClientboundBundlePacket` — single vanilla send per player per tick.
- Measured: leg-2's counters increment on packet BUILD/SEND success, fail 0; the client logs no decode errors; the #250 v6.10.1 re-arm streamed 21 cells of dots around the probe window. → **delivery EXONERATED by counters**; the two client logs confirm pack-flow both legs. What the counters CANNOT see: whether the CLIENT accepts bundle-wrapped packets and the count=0 branch (S2).

**S2 — client spawn (the packet-shape axis — UNMEASURED until v6.11):**
- `count=0` is a distinct client branch: exactly one particle at the exact position. Every harness burst is one packet with `count=1200` (client-side expansion) — a DIFFERENT branch. `overrideLimiter=true` bypasses the particle-setting clip (and v6.10.0 additionally pins `particles:2` in `run_client.sh`, removing the MINIMAL censor — forced bursts survive ~19% under MINIMAL via the double `nextInt(10)`).
- Discriminating cells (v6.11): control/leg-1/leg-2 `end_rod stream-shape` (count=0 ×120, eraser-immune) + the carrier twins. **This axis is the dossier's primary open suspect**: it is the ONE client-side path the stream uses that no published burst cell exercised.

**S3 — client render (the type axis):**
- The carrier quad is `happy_villager`; `end_rod` renders in EVERY probe ever fired (543/530/407/495/479/383/324), so the renderer, the world, and the vantage work on leg-2. The documented carrier FLAP (TESTS.md v6.9.14: marker red 1631/27/0/93/2961 across #234-#245, BOTH legs) is the type's own instability.
- Discriminating cells: control villager NEAR (type renderability raw), leg-1 villager NEAR (healthy twin), leg-2 villager NEAR (the carrier on the relog client), and the v6.11 carrier stream-shape twins (type × per-dot shape).

**S4 — capture/decode (pack-only):** (`capture.fsh`)
- Camera channel from sky (`rot_solve`), confidence gate `rotP.b < 0.25` → pass-through (244-256); mode 2 = the Y=100 edit-reset plane (232-243); data modes probe the cell's 16 sub-positions at the plane (Y=120 HI / Y=80 LO, 258-265) with the v5.3.1 5-tap cross strobe on un-fogged color + the v6.5 fog-invariant ring arm (175-195); fresh word rides `.b` for the eraser (280-286).
- Measured on leg-2: stage-eyes decode (rot=decoded, cam/az/el correct) — the channel and gates LIVE; `strobe_hits=0` — the strobe sees NO dots → consistent with death at S2/S3 (nothing on screen to strobe), NOT with a blind decoder.

**S5 — eraser (pack-only):** (`dust_mask.fsh`)
- Mask = projection of the fresh words (DOT_WORLD=1.3, discs 1.5-48 px) **+ the color net**: EVERY pixel passing the carrier-hue test (absolute `g>0.45, g>r+.08, g>b+.08` OR relative `(g-b)-ring>0.06`) gets a disc — **3 px in-band (`tHi/tLo ∈ [24,128]`), 2 px OUT-of-band** (185-220). `lod_terrain.fsh` repaints masked pixels from their surroundings.
- Consequence (published as the v6.11 §4 correction): the net reaches EVERYWHERE; no vantage is eraser-free for carrier hue. NEAR carrier cells on packed legs measure the eraser; the carrier's renderability anchor is the control arm; twins carry the leg comparison.
- Measured on leg-2: `mask_hits=0` — the eraser reports NOTHING to hide → also consistent with pre-display death.

**S6 — display:** the eye never sees dots by design; the DISPLAY is `lod_terrain.fsh`'s paint from the decoded words. Leg-1 paints (red=1117/1183); leg-2's paint is moot while S2/S3 starve the decoder.

## 2. The suspect table (post-v6.11 — every suspect bound to its cells)

| # | Suspect | Class | Discriminating cells | Status |
|---|---|---|---|---|
| A | **count=0 per-dot packet path dead on relog** | client spawn branch | leg-2 end_rod stream-shape (≈0 vs burst-alive) + control/leg-1 twins | OPEN — v6.11 primary |
| B | bundle wrapper swallows/truncates on relog | engine-only wire | stream-shape alive + villager/eraser alive + stream still dead → B by elimination | OPEN — engine-only |
| C | carrier type flap (villager quad) | client render type | control villager, leg-1/leg-2 villager NEAR, carrier stream twins | OPEN (documented flap) |
| D | in-band eraser blind on leg-2 | capture tail | v6.10.2f eraser test (@Y120 in-band) | OPEN |
| E | trickle cadence/pool (120/s trickle vs bursts) | client accounting | relog probes (live stream) + carrier stream twins | OPEN (weak) |
| F | plane/depth (reversed-Z at Y=80/120) | client render geometry | end_rod@120 plane control, dust@120/80 cross | OPEN — measured next run |
| G | pack-presence interaction | session | cached-pack wipe control (v6.10.2c) + leg-1 probes | OPEN |
| H | worldless relog client (#246/#247 class) | harness/env | FIXED by v6.10.0 (particles:2 pin + baseline-changed counting + scene ruler); relog `scene=` now guards it | CLOSED (fixed) |
| I | transport (gate/wire) | server | #248/#250 counters | CLOSED (exonerated) |
| J | enabled-set/arm race | session state | 124432c audit (healed clean, scene dead anyway) | CLOSED |

## 3. The G3 fix menu (pre-positioned, keyed to the reading)

- **A positive** (count=0 dead): batch the dots — `count>0` packets (e.g., group a cell's 16 sub-dots into packets of 16 at the cell center with a ±sub/2 delta — the decode still reads exact positions? NO — expansion randomizes positions; instead bundle per-cell MICRO-bursts only if the decode tolerates it) or, safest, per-dot packets inside the existing bundle but with a client-side diag FIRST (log the received-packet count client-side once). Design note: batching must preserve the word geometry the capture strobes — any expansion breaks the 2-bit X quantum; the real fix if A reads dead is a client diag + upstream report, not a geometry-breaking batch.
- **B positive** (bundle suspect): engine diagnostic mode `CLOD_UNBUNDLED=1` — send the tick's packets individually (v5-era path) for one run; if the stream paints, the bundle path on 26.3 is the killer and the fix is a chunked/bounded bundle or a fallback.
- **C positive** (type): carrier stability work (frame-locked spawn cadence) or migration — the migration candidate is measured by the dust cells (v6.10.2e) and the green-strobe design (v6.9.14); dust renders 0-2 px per the engine's own live verification, so a real migration needs a NEW stable type, not a return to dust.
- **D positive** (eraser blind): capture-tail repair — the eraser is cosmetic-stealth; a blind eraser on leg-2 would actually make dots VISIBLE, so a blind eraser + still-dead display points back to S2/S3. (D matters for the stealth contract, not for the death.)
- **F positive** (planes): move `LAYER_Y` or per-plane carrier — the cross cells give the numbers.

## 4. What would falsify this dossier

- The relog probes catching a LIVE stream on the G0 run (any non-zero changed/red in-band) → the anomaly is cadence/timing (E), not a hard death; re-time the probes against the re-arm window.
- end_rod stream-shape alive on ALL THREE arms AND the live stream still dead with the eraser test alive → B (bundle) is the only survivor → the engine diag mode decides it.
- control end_rod stream-shape ≈ 0 → the shape axis is build-wide dead and NONE of the leg-2 matrix matters until the carrier shape changes (the P0 row).
