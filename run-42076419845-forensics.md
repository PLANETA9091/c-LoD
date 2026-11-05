# c-LoD verdict: run #250 (workflow_dispatch, fix/leg2-arm-race @ 6b0b599, v6.10.1)

**Correction to the #248 forensic addendum: the arm race was REAL but is NOT the root cause. #250 is the controlled experiment that kills it — and proves the leg-2 death lives deeper, inside the capture path, silent, with the chains fully armed.**

## What #250 ran

v6.10.1 (branch fix/leg2-arm-race): the bounded post-reload chain re-arm (engine.rs, push+100t/+200t), the harness arm-race control (tier flip when the first leg-2 scene probe reads 0), and the dust burst spread fix (delta 1 1 1). All code gates green in CI on the branch (94 tests, fmt, clippy -D warnings); the full visual e2e ran via workflow_dispatch on the branch (the e2e jobs are master-only on push).

## The result

- **LEG 1 PASS** (marker rendered, honest numbers) — the engine patch does not disturb the leg-1 bootstrap, as designed.
- **The engine re-arm fired exactly as designed** (server.log): `chain re-arm terrain (remove -> true add -> true) post-reload` at 17:49:35 (push+100t) and 17:49:45 (push+200t), both chains. No client warnings at all this run — **the #248 join-time race did not even happen** (the effect adds resolved cleanly after the reload).
- **The harness control fired** (`scene dead at first probe (0 px)` → tier flip out→Veil at 17:52:05, `effect terrain remove -> true`; back→Full at 17:52:08, `effect terrain add -> true`) — the same primitive that has carried leg-1's terrain arm in every successful run.
- **The scene stayed dead anyway**: baseline scene=0, all 12 relog probes red=0 scene=0, `FAIL: leg-2 painted no scene in any probe`.
- Dust probe with the spread fix: still changed=0 (see below).

## What this proves

1. **The arm race is not the root cause.** In #250 the chains armed early (post-reload, no race), were re-armed twice by remove+add, and rebuilt once more by the tier flip — five clean arm events, all returning true, over 5+ minutes — and the client never painted a world. The #248 client warnings were a real transient (and the re-arm instrumentation is worth keeping for determinism), but the death survives them.
2. **The leg-2 class is the capture path itself.** The debug lens panels (part of the same pack) render on the relog client — the pack is applied and post chains run — while the LOD terrain layer produces nothing. The death is silent: client.log is clean (no post-effect warnings, no shader errors).
3. The dust probe reads 0 changed px even with delta 1 1 1 — the same coordinates render 375 px with end_rod, so dust either fails to render (atlas/quad path) or the burst lands outside the ruler's expectations. The carrier question stays open; it is also moot until the scene lives.

## The prime suspect (found while tracing the transport)

The v5.3.1 spawn gate (engine.rs, `spawn_gate` path): dots spawn to a player **only when `sky_visible = pitch < SKY_PITCH_LIMIT_DEG (47°)`**, inside the 3D decode band **[CULL_RADIUS=28, ZONE_FAR=112]** from the eye, and inside the view frustum cone. The bench vantage is `pitch 5` (gate passes), but the sky planes at Y=260/272 sit 108–120+ m from the eye — **at or beyond ZONE_FAR**, so which dots flow at all is decided by these per-player, per-tick gates. The gate breakdown counters (`n_near / n_far / n_cone` + blind) are already computed per player per tick in the spawn loop — **and are not printed in the stream diag.**

## Next instruments (cheap, decisive)

1. **Surface the gate counters in the diag** (`gate n=X f=Y c=Z b=W` per player): if leg 2's counters read zero while leg 1's flow, the server stops spawning dots to the relog client (per-session server state — cursor/wire cache tied to the session); if both flow, the dots arrive and the client-side decode is the break.
2. **The sky-plane probe**: one extra leg-2 shot (and leg-1 control) with the camera pitched up (`pitch -60`) so the LO plane enters the frame; count dust-lattice rows changed-vs-baseline. Lattice visible on leg 1 and empty on leg 2 ⇒ delivery/render of the bit-stream is dead on relog clients; visible on both ⇒ the decode math is the suspect.
3. Keep everything v6.10.1 added: the re-arm makes arming deterministic regardless of runner speed, the flip is the standing control, the dust spread fix stands.

## Bottom line

LEG 1 PASS / leg-2 scene-dead survives its strongest counter-hypothesis yet. The renderer-dead class is now boxed into the capture/transport path with the chains exonerated (armed, re-armed, rebuilt — and silent). The gate counters and the sky-plane probe will split "the server never sends the bit-stream to a relog session" from "the client cannot commit the decode" in one run.
