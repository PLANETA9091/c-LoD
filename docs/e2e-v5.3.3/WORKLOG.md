# e2e v5.3.3 — cone-priority scheduler on the stealth transport

Date: 2026-10-30. Base: v5.3.2 zero-residual (3504d62). This wave adds the
SPEED half: after a turn the visible zone refills in seconds, not half a lap.

## What changed

- `src/stream.rs`: `ViewHint(eye, look, now)` + two-phase `tick_batch`:
  - phase 1 (cone): cells whose BOTH plane probe points sit in the decode
    band `[CULL_RADIUS+2, ZONE_FAR-2]` AND inside the spawn-gate frustum
    (`cos_half_cone`) stream FIRST, near-first, age-gated
    (`VIEW_REFRESH_TICKS=60`); cache rebuilt on >8 blocks / >5 deg;
  - phase 2: the legacy row-major lap (other players, view=None, self-heal);
  - every cone-phase spawn passes the per-player gate geometry by
    construction — zero budget wasted on culled dots.
- `src/engine.rs`: first player's live camera (Bukkit yaw/pitch ->
  `look_dir`) drives the phase every engine tick.
- Tests: 3 new (`cone_phase_streams_first_and_in_band`,
  `cone_age_gate_holds_fresh_cells`, `cone_follows_the_turn`) -> **42/42**,
  clippy clean, GLSL 12/12.

## Live facts (bench: Paper 26.3 + llvmpipe client, fresh boot)

- Boot gates 8/8 OK; pack finalized once (69998 bytes, sha1 adeb4d16…).
- TPS: **20.0 / 20.0 / 20.0** under full streaming load.
- Spawns: 59 861+ / **0 failed**; cells scanned 10 985/65 536 (growing).
- **Invisibility**: strict carrier-green scan (`g>0.45, g>r+0.2, g>b+0.2`)
  = **0 pixels** on every frame (day north view, east turn, night with moon).
- **Speed (the new part)**: 90-degree turn at t=0 -> silhouette already
  painted at **t+3 s** (15 238 px above-horizon band), stable by t+8 s
  (15 420 px), t+20 s (15 500 px). The cone phase paints the visible band
  within ~2-3 s of any turn — DH-class subjective fill speed at the same
  2048 particles/tick budget (was: the row-major cursor met the new cone
  half a lap later).

## Shots (`shots/`)

- `01_north_LOD_day.png` — day, north view: LOD dome with trees, 0 green px.
- `02_east_turn_t3.png` — +3 s after a 90° turn: silhouette painted.
- `03_east_turn_t8_night.png` — night, moon: LOD hills + horizon mountains,
  0 green px.
- `04_east_turn_t20.png` — +20 s: stable.

## Bench notes (for the next operator)

- The bench player died twice from ocean spawn drowning; the respawn click
  on llvmpipe renders 640x360 UI centered in 1280x720 — Respawn button is at
  (640, 350), pack-confirm Proceed at (562, 385). `spawnpoint` + spectator
  mode make the stand stable.
- The gate log samples hit cone-idle windows often (the cone drains its
  budget in ~7 ticks, then sleeps `VIEW_REFRESH_TICKS=60`); judge cone
  health by the `spawned` counter and the pictures, not by one sample.
