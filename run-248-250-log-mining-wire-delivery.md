# Log-mining note: #248/#250 — the leg-2 client RECEIVES the stream (~19-38 dust dots/s, fail 0) and renders none of it

Date: 2026-11-06. No new runs and no new commits (still #251 / 35eb53b / d4ff522). The artifacts were re-downloaded for #248 (run 42063218460: `visual-evidence` + full `client.log`/`server.log`) and #250 (run 42076419845: `visual-evidence` + the differential control arm — vanilla server, no pack: the console end_rod close burst renders **641 white px**, the environment is fine). This pass mined the RAW logs and the diag counters instead of the screenshots.

## 1. Diag semantics correction (previous reads were wrong)

`spawned N / failed M` in the `stream:` diag line is the **legacy per-particle path counter** (`jni::STREAM_SPAWNED`, engine.rs:1206), cumulative, and incremented only when the native wire is NOT used (engine.rs:1397-1400). Both runs ran the native wire end-to-end (`fail 0` in every line), so `spawned 0 / failed 0` in EVERY diag line of BOTH legs — leg 1 included, which PAINTED — is expected, not a delivery failure. The actual delivery instrument is the cumulative **`wire bundle {p}p/{b}b fail {n}`** counter (engine.rs:1527: `WIRE_SPAWNED` bumps only after a successful bundle send).

## 2. The wire numbers (new quantitative facts)

**leg 1 (#250), the live-edit storm** — wire bundle per 100-tick diag window:

- 17:48:21 → 17:48:31: 803,751 → 818,356 (**+14,605 dots/window**);
- 17:48:31 → 17:48:41: → 835,914 (**+17,558 dots/window**);
- decode→paint followed: leg-1 probe0 `red=1117 paint=15093`. **The wire→render→decode→paint pipeline works END-TO-END on leg 1 in the very same run.**

**leg 2 (both runs), from the rejoin to the end of the probes:**

- #250: **+189/+378 dots per 100-tick window CONTINUOUSLY from 17:49:31 through 17:54:22** — covering the re-arm windows AND all 12 relog probe windows — `fail 0`, one player (Stepan), every dot gate-passed inside `wire_deliver` (engine.rs:1482: `spawn_visible` filters BEFORE the packet is added and counted): sky-visible (pitch 5 < 47), in-band (≤107.4 m for the hword plane — the 2d73c16 arithmetic), in-cone (−5°±63° covers elev −22.9°/−43.7°);
- #248: the same +189p/window pattern from its rejoin (ticks 5000→5400: 795,531 → 796,476) — its ENGINE chain re-arm did not exist yet (v6.10.0), the STREAM-level `rearm 21c` cells flowed the same way.

**Result on the leg-2 client: ~19-38 gate-passed dust dots per second arriving for ~5 minutes; ZERO of them ever rendered** — `strobe_hits=0` AND `mask_hits=0` on every probe, all 12 relog frames byte-frozen (sha256-identical, bfbed5c audit), the console dust burst 0 changed px — while the SAME client renders console end_rod (NEAR 495/479, FAR 383→324) in the same minutes, and every capture stage-eye reads correct on every probe (`rot=decoded cam_x_b=8 cam_z_b=11 az_b=0 el_b=3`: the vantage bucket, east, the expected elevation — the same eyes the passing leg-1 shows).

## 3. What this settles and what it retires

- **SETTLED (server side):** the "server does not send" branch of the fork is DEAD. The relog re-arm dots and the steady dirty-cell stream are delivered to the leg-2 client continuously, gate-passed, `fail 0`. No server-side spawn/delivery/gate bug survives the wire counters — the transport is exonerated by counters, not by inference.
- **SETTLED (capture side):** the eyes prove the capture chain renders and the projection is correct on leg-2 — the instrumentation is healthy; the INPUT is what's missing.
- **Method note:** leg-1 probe0 also reads `strobe_hits=0 lens_dark=197796` — that is NORMAL (dots live 3-5 s, the decode persists, the paint persists; the shot lands after the flight). The diagnostic power comes from leg-2's CONTINUOUS in-flight dots (+189-378/window DURING the probe windows) vs zero strobe ever.
- **RETIRED:** the `end_of_frame` WARN×2 (join-time + right after the pack reload, BOTH runs — #250 lines 277/932, #248 lines 278/923) — a transient of the resource swap: the capture chain demonstrably arms afterwards (the eyes decode on every leg-2 probe). The v6.10.1 narrative "the rebuild re-fired end_of_frame" holds in effect; the WARN is swap noise, not a failed arm. Downgraded from root-cause candidate within this pass.
- **WEAKENED (no log support):** the reload-time atlas-rebuild mechanism — both reloads create `particles.png 512x256x0` cleanly (#250 lines 114/913), no missing sprites, no stitch/particle errors, and ZERO "dust" mentions anywhere (the client does not log particle spawns at all — the strobe ruler is the only client-side dust instrument). A silent atlas/shader break remains possible but is unevidenced.

## 4. The remaining root-cause space

The break is CLIENT-side and dust-type-specific on the relog client:

1. the dust render path (the tinted-quad COLOR path) does not render on the relog client while textured sprites (end_rod) do — same client, same minutes, same particle engine, same atlas;
2. OR the dots render but at positions the lens never samples — the plane/depth class: end_rod has only ever been fired at Y=150 (eye level), NEVER at the stream's own planes Y=80/120 where all the invisible dots fly (the stream doc's own reversed-Z caveat lives exactly there);
3. and the one measured CLIENT-side difference between the legs is the PACK SOURCE: leg-1 = first-time download + prompt accept; leg-2 = **cached-pack auto-accept** (`accept_pack_prompt` finds no prompt — the cached-load path). The leg-2 client's dust path dying while its end_rod path lives makes the cached-load code path the leading suspect.

## 5. Instruments (updated)

- The published probe patch stands unchanged (bfbed5c: `instrument-proposal-leg1-dust-type-matrix.patch`): the leg-1 dust probe (decisive — does dust render on a live client at all) + the leg-2 NEAR type matrix.
- NEW, this commit: **`instrument-addendum-endrod-planes.patch`** (+22 script lines, applies AFTER the base patch, apply-chain base→addendum verified, `bash -n` green): the leg-2 PLANE CONTROL — end_rod at **Y=120** (the hword plane, marker zone, delta 1 1 1, the exact geometry of the dots the client refuses to render): > 20 changed-white = the planes are fine, the break is dust-TYPE-only; ~ 0 = the plane/depth class rides along (reversed-Z) and the carrier must change type AND possibly altitude.

## Bottom line

The leg-2 client receives ~19-38 gate-passed dust dots per second for the entire session and renders zero of them — with every capture instrument healthy and the same pipeline painting on leg-1 minutes earlier. The remaining question is exactly two-sided (dust-type vs plane/depth, with the cached-pack load as the client-side suspect), and the two published patches split it in one run.
