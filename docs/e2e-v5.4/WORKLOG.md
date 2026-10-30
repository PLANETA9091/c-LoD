# e2e v5.4 — DELTA-ZONE scheduler (dirty phase + zone sweep replace the blind lap)

Date: 2026-10-30. Base: v5.3.3 cone-priority (48f01a0). This wave is the
TRAFFIC half of the stealth transport: the client's buffers are persistent,
so the server now re-streams ONLY what can decode right now or what changed.

## What changed

- `src/stream.rs`:
  - `tick_batch(budget, views: &[ViewHint])` — every player's camera is a
    view; views[0] drives the cone, ALL views define the decodable set.
  - Phase 0 (NEW, dirty delta): `set_cell` marks a cell dirty when a REAL
    edit happens (previous cell valid, encoded words differ — the initial
    scan fill is painting, not editing). Dirty cells stream FIRST (capped
    at half the budget), and only when SOME view can decode them; edits
    behind the back wait in the queue and ride the front of the batch the
    moment a view covers them.
  - Phase 2 (REPLACED): the legacy row-major lap now streams only DECODABLE
    cells (decode band + ANY view's frustum — the same geometry the
    per-player spawn gate uses), age-gated `ZONE_REFRESH_TICKS=600`
    (background self-heal); never-streamed cells are always eligible.
    Cells no player can decode were always culled at delivery — now they
    cost ZERO budget. An empty views slice keeps the legacy full lap
    (diagnostics).
  - New counters: `dirty_streams`, `zone_cells`, `dirty_pending`.
- `src/engine.rs`: views for all players; the rate-limited `stream:` log
  now prints EVERY 100 ticks even when the batch is empty (the idle
  collapse is a headline feature and must be visible).
- Tests: 4 new (`changed_cell_streams_first`, `unchanged_rewrites_stay_clean`,
  `zone_sweep_spends_budget_only_on_decodable_cells`,
  `dirty_cell_out_of_zone_waits_for_a_view`) -> **46/46**, clippy 0.

## Live facts (bench: Paper 26.3 + llvmpipe client, fresh boot)

- Boot gates 8/8 OK; pack finalized once (69998 bytes, sha1 adeb4d16… —
  byte-identical to v5.3.3: the scheduler is server-side only).
- **Idle collapse (the delta headline)**: while painting, batch = 1024
  (cone half-budget); on a painted, unchanged zone the batch collapses to
  **0** (`stream: … cone 22230 zone 1689 dirty 0 (q 0), batch 0`). The
  refresh wave returns every VIEW_REFRESH_TICKS=60 (cone) and
  ZONE_REFRESH_TICKS=600 (sweep).
- **Invisibility**: strict carrier-green scan over the sky band = **0 px**
  on every ON frame (north dwell, south turn t+2..t+15, night spawn view).
  The A/B OFF frame (`03_ab_off_raw_carrier.png`) shows what the dust_eater
  chain removes: the raw carrier field over the dome area.
- **Speed**: a NEVER-LOOKED direction (south) is fully painted by the first
  probe at **t+2 s** after a 90° turn (46 400 non-sky px in the south band,
  constant t+2..t+15 — stabilized within the first 2 s). The cone phase
  streams the new wedge near-first within ~9 ticks; the rest is decode
  latency. DH-class or better.
- **TPS: 20.0 / 20.0 / 20.0** under the full streaming load; 2 054 906
  spawns / 0 failed at the time of the check.
- Scan frontier: 23 937/65 536 cells in ~25 min (center-out ring sweep;
  gen-ticket bound — unchanged from v5.2.2).

## Shots (`shots/`)

- `01_center_north_dwell.png` — day, disk-center vantage (256,140,320):
  the dome curtain mid-accumulation; 0 green px.
- `02_south_turn_t2.png` — first probe t+2 s after the 90° turn to the
  never-looked south: already painted, 0 green px.
- `03_ab_off_raw_carrier.png` — posteffect removed: the RAW carrier dots
  over the dome area (what the dust_eater chain erases in ON frames).
- `04_night_spawn_clean.png` — night, spawn view: 0 green px.
- `05_east_turn_t8.png` — east turn t+8 s: dome curtain, vanilla chunk
  columns on the left, 0 green px.

## Bench notes (for the next operator)

- The bench player DIES on tp if the mode is not spectator first (fell
  from a high place -> death screen freezes cam0 at the last live
  rotation). Order: Respawn click (640,351) -> `gamemode spectator` ->
  `tp`. Judge cam0 health by the `stream:` log line.
- The disk-center vantage looks INTO the dome volume: the ray-march draws
  the curtain while the buffer accumulates; the v5.3.3 corner vantage
  (0,140,0 yaw 45) is blocked by vanilla hills at the moment (world seed
  area). Dome aesthetics from a fresh boot need the corner cells scanned
  (the frontier is gen-ticket bound).
- `time set day` does NOT hold (doDaylightCycle broken since v5.2.1):
  re-set before every day-lit shot series.
