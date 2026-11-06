# Instrument v6.11 — the STREAM-SHAPE cells: the engine's per-dot packet (count=0) has never been measured — the whole bit-stream rides a shape no burst cell tests

Date: 2026-11-07. Follows the v6.10.2f correction (the carrier is happy_villager). Read from the LIVE sources on master d4ff522: `src/jni.rs::build_particle_packet` (lines 1623-1648) and `src/stream.rs` (word_spawns 734, push_cell_spawns 821-839, spawn_visible 149-168). Companion document: `dossier-leg2-dot-death.md` — the full stage map of the transport with file:line and the suspect-to-cell mapping. New automation: `triage.py` — evaluates every published decision row from a run's probe log, self-tested against #248/#250 artifacts.

## 1. The packet-shape axis is unmeasured — and it is the stream's actual shape

Every burst cell published so far (leg-1 probes, leg-2 type matrix, plane control, plane × type cross, eraser test, control matrix) sends **ONE packet with count=1200** (or 400/600): `/particle <type> X Y Z <delta> 0 <count> force` — one `ClientboundLevelParticlesPacket` the client expands into N particles inside the delta box.

The stream does not do that. The engine builds **one packet PER DOT** — `jni.rs:1623-1648`:

```
(options=happy_villager, overrideLimiter=true, alwaysShow=true,
 x, y, z, xDist=0.0, yDist=0.0, zDist=0.0, maxSpeed=0.0, count=0
 -> "exactly one particle at the point, zero drift")
```

— and ships the tick's batch in ONE `ClientboundBundlePacket` (`jni.rs::bundle_send`, 1700+). `count=0` is a distinct client-side branch: spawn exactly one particle at the exact point. No burst cell exercises it.

**`/particle` with count=0 emits the byte-identical packet** (same constructor, same fields — Paper's `/particle` forwards through `ServerLevel.sendParticles`). So the harness CAN measure the stream's exact wire shape on all three arms. That is v6.11: `stream_burst()` fires a 12×10 grid of 120 single-packet dots (1 block apart, NEAR 20-31 m — out-of-band, on-screen, x<940), `+112 script lines`, applies after the full 6-patch chain, `bash -n` green, zero rust, ~+30 s runtime, purely additive.

## 2. The six cells (three arms × end_rod + carrier)

| Cell | Shot | Baseline | Reads |
|---|---|---|---|
| control end_rod stream-shape | `09d_control_endrod_stream` | 09c_control_villager | the count=0 path RAW (pack-free: no eraser exists) |
| control carrier stream-shape | `09e_control_villager_stream` | 09d_control_endrod_stream | the per-dot CARRIER path raw |
| leg-1 end_rod stream-shape | `09h_leg1_endrod_stream` | 09g_leg1_villager_near | healthy-client reference (white = eraser-immune) |
| leg-1 carrier stream-shape | `09i_leg1_villager_stream` | 09h_leg1_endrod_stream | healthy twin for the leg-2 carrier read |
| leg-2 end_rod stream-shape | `14_leg2_burst_endrod_stream` | 14_…villager_plane | **THE decisive cell** — the per-dot path on the relog client, eraser-immune |
| leg-2 carrier stream-shape | `14_leg2_burst_villager_stream` | 14_…endrod_stream | per-dot carrier on leg-2, read against the leg-1 twin |

Why end_rod first: the color net cannot eat white (its hue test needs `g > r+0.08`), so the end_rod stream-shape cell is the clean per-dot-path read on packed clients. The carrier cells are eraser-overlaid (see §4) and read as twins.

## 3. Decision rows

- **leg-2 end_rod stream-shape > 20** (NEAR burst 479/495 alive) → the count=0 single-dot path **renders** on the relog client → packet shape exonerated. The stream's death must ride: the carrier type (villager flap — split by the villager cells/twins), the in-band eraser (split by the v6.10.2f eraser test), the **bundle wrapper** (engine-only — `ClientboundBundlePacket` has no `/particle` equivalent, cannot be measured from the harness), or the trickle cadence (relog probes). If the villager and eraser cells come back alive-looking, the bundle wrapper is the last man standing → G3 recommendation: an unbundled-send diagnostic mode in the engine.
- **leg-2 end_rod stream-shape ≈ 0** (burst alive) → **THE count=0 single-dot path is DEAD on the relog client.** The stream's per-dot packets die BY SHAPE — nothing in any client shader is involved (the eraser never sees them: `mask_hits=0` consistent). G3 P0 fix: batch the dots (count>0 packets, or bundle N-dots-per-packet) — no carrier migration can help.
- **leg-1 / control end_rod stream-shape ≈ 0** (their bursts alive) → the count=0 path is dead on the build itself (both legs) — a 26.3-client single-particle bug class; same P0 batch fix, independent of the leg-2 anomaly.
- **carrier stream-shape twins** — leg-2 ≈ leg-1: the per-dot carrier path behaves like the healthy leg (carrier exonerated at NEAR). leg-2 green > 20 with leg-1 ≈ 0: the eraser is blind on leg-2 at per-dot geometry. leg-2 ≈ 0 with end_rod stream alive: the per-dot CARRIER path specifically dies — type/pool at the packet level.

## 4. A correction to the v6.10.2f §3 band classification (found writing this instrument)

The correction note classified the NEAR spot (20 m) as "OUTSIDE the band → the eraser never touches it". Strictly wrong: `dust_mask.fsh` v5.3.2b (lines 185-220) fires the color net at **EVERY** pixel passing the carrier-hue test — the band (`tHi/tLo ∈ [24,128]`, line 194) only sets the disc radius: **3 px in-band, 2 px OUT-of-band** ("OUT-OF-BAND carrier hue still gets a 2px disc" — the stale-dot cleanup). A NEAR carrier burst on a healthy packed client therefore renders AND gets net-erased (its hue-core pixels repaint from their own ring ≈ the pre-burst frame; the surviving signal is the sub-threshold rim + imperfect fill in `total`).

Consequences for the published rows (no patch changes needed — the readings just re-scope):
- **villager NEAR cells on packed legs measure the ERASER** (at per-dot-free burst geometry), not renderability;
- the renderability anchor for the carrier on this build is the **control arm** (pack-free) alone;
- the leg-1 ↔ leg-2 twin comparison (same eraser both legs) is the valid signal for carrier×leg differences;
- the v6.10.2f row "leg-2 NEAR villager ≈ 0 with end_rod alive → the carrier TYPE is dead" is only valid together with the control villager cell (~0 there = type dead build-wide; > 20 there + leg-2 ≈ 0 = leg-2-specific death, not type).

## 5. Cumulative rebuilt — ONE command for the decisive run

`instrument-chain-full-v6.11.patch` — **+385 insertions** vs vanilla `35eb53b` (raw git diff, real index lines), verified **byte-identical (sha1 d93576f0)** to the 7-patch step-by-step chain (6 published + this addendum) applied onto a clean worktree, `bash -n` green. The author runs:

```
git apply instrument-chain-full-v6.11.patch && bash -n scripts/bench/run_ci_visual.sh
```

Every cell is additive (no fail/pass path touched, no thresholds changed, no rust). One run then reads the ENTIRE matrix: session × type × plane × pack × packet-shape, with the eraser test and the per-dot path separated.
