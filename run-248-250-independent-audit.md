# c-LoD independent audit: runs #248 (42063218460, d4ff522 v6.10.0) + #250 (42076419845, 6b0b599 v6.10.1)

**Independent re-verification of the #248 verdict and the #250 forensics, with own pixel rulers and raw log greps (not trusting probe_counts or the summary). The verdicts CONFIRM; one forensic claim is CORRECTED; one new hard fact emerges.**

## Confirmed by own rulers (PNG-decode at pixel level)

- **Leg 1 paints** (probe0 after fill): is_red = 1183 (#248) / 1117 (#250) — exact match with the recorded verdicts.
- **Leg 2 paints nothing**: is_red = 0 on the baseline and every checked relog probe of both runs.
- **The leg-2 frame set is byte-frozen**: among the 12 relog probes + baseline + greendust shot there is exactly ONE unique frame per run (sha256-identical); diff vs baseline = 0 px over the whole frame.
- **The leg-2 renderer is alive**: the burst shots DO move pixels — NEAR changed 495 (#248) / 479 (#250), FAR 383 / 324 — matching the harness counters. Particles render into the frame; only the LOD scene layer contributes nothing.
- **Dust probe still empty** even with the delta 1 1 1 spread fix: 0 changed px in both runs while the same coordinates give 375 px with end_rod. The dust instrument needs its own investigation (atlas/quad path); it is moot until the scene lives.

## NEW hard fact: cross-run byte identity

The frozen leg-2 frame of #248 (v6.10.0) and #250 (v6.10.1) is **the same image byte-for-byte** (sha256 2fe0cf5a…): engine changed, chains re-armed twice (remove+add, all true) and rebuilt once more by the harness tier flip between the shots — and not one pixel moved. The leg-2 world render is a deterministic constant; the frame is live (bursts perturb it) but the scene layer is absent, independent of chain/arm state.

## CORRECTION to the #250 forensics

"No client warnings at all this run — the #248 join-time race did not even happen" is **wrong**. client.log line 277-279: `[17:49:20] WARN Requested post effect does not exist: minecraft:end_of_frame / clod:lod_terrain / clod:atmosphere`. The join-time race DID happen in #250 as well. What changed in #250 is that the v6.10.1 re-arm healed it: 17:49:35 and 17:49:45 `chain re-arm terrain/atmosphere (remove -> true add -> true) post-reload` — with zero warnings after 17:49:35 (the re-added chains resolved against the now-loaded pack), plus the harness flip at 17:52:05→08 (Full→Veil→Full, terrain remove→true / add→true).

## Sharpened conclusion

The death is definitively NOT the enabled-set state: in #250 the join-time adds failed (WARN×3 at 17:49:20, same race as #248), the re-arm restored the set (all true, resolved cleanly), the flip rebuilt the chain again — 8+ clean arm operations over 2.5 minutes across two engine versions — and the scene stayed dead at a byte-constant frame. The chains are enabled, silent, and unproductive: the death sits in what the chains consume or how they execute on a fresh-session client. The capture/decode path (prime suspect: the v5.3.1 spawn gate — sky_visible pitch<47, decode band [28,112] vs sky planes at 108–120 m) stands, and the two decisive instruments remain: surface the per-player gate counters in the diag, and the leg-2 sky-plane pitch probe (pitch ≈ -60 shot, count lattice rows changed-vs-baseline, leg-1 control).

## #248 race timeline (verified to the second from client.log)

16:31:40 join → 16:31:41 WARN×3 (chains requested against vanilla resources) → 16:31:45 `Reloading ResourceManager: vanilla, vanilla, server/00000000/…` → 16:31:48 reload completes (atlases rebuilt) and the post-reload rebuild re-requests ONLY `minecraft:end_of_frame` (still WARNed at that instant); `clod:lod_terrain`/`clod:atmosphere` are never re-requested — they left the enabled set.
