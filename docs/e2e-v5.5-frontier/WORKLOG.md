# e2e v5.5 — GEN-FRONTIER sprint (the scan is the last slow half of the loop)

Date: 2026-10-31. Base: v5.4 DELTA-ZONE (59fbdc7). v5.4 made the STREAM
DH-class (never-looked direction painted by t+2 s); the remaining backlog
item was the SCAN frontier: server-side sampling of the 512x512 disk was
"gen-ticket bound" (~16 cells/s, full disk ≈ an hour). This wave attacks it.

## Root-cause hunt (live, leg diag + A/B/C)

- The frontier stalled visibly: cells froze mid-log with TPS 20 — the ring
  walk is fine; the bottleneck chain is FORCED-ticket generation throughput
  vs Paper's auto chunk-system sizing on a 2-CPU host (auto = 1 gen worker:
  availableProcessors - 2, min 1) + a wish window (96) smaller than the
  frontier ring's chunk set (~100 chunks at r≈44 cells).
- Added `wish {n} forced {n}` to the rate-limited `stream:` log line (now
  the frontier health is visible in every 100-tick diagnostic).

## What changed

- `src/config.rs`: two new knobs — `gen_budget` (NEW gen requests admitted
  per tick, default 16, was a fixed 4) and `gen_wish` (in-flight FORCED
  window, default 512, was a fixed 96), clamped 1..256 / 32..4096. Tests:
  `frontier_knobs_parse_and_clamp`.
- `src/engine.rs`: `wish_admit` takes the window as a parameter (test
  `wish_admit_takes_the_v55_window_from_cfg` covers 32 and 512); poll
  sampling cap 128 -> 256 cells/tick; the legacy constants moved into the
  test module as fixtures; diagnostics above.
- `scripts/bench/patch_paper_cfg.py` (NEW): line-level patcher for Paper's
  `config/paper-global.yml` chunk-system `worker-threads` / `io-threads`
  (env-gated, idempotent, no-op until Paper writes the file on first boot).
- `scripts/bench/boot_server.sh`: defaults `PAPER_WORKERS=4 PAPER_IO=2`;
  the port-8199 wait-loop (a surviving Paper JVM with rewritten argv held
  the port and silently killed the pack HTTP bind -> pack-less session,
  found live in the first verification boot); a HARD HTTP gate (curl the
  pack after "pack finalized", boot fails loudly when not served).
- `scripts/bench/run_frontier_leg.sh` (NEW): one A/B leg = fresh world +
  fixed seed + boot + client join + measured window + scan_rate.py + tps.
- `scripts/bench/scan_rate.py` (NEW): parses `stream:` lines -> per-window
  cells/s, overall rate, best window, frontier %.
- `scripts/bench/setup_server.sh`: fixed `level-seed=clodbench2026` (fresh
  worlds must be genetically identical across A/B legs).

## Measured (2-CPU sandbox, fresh world, seed clodbench2026, 180 s legs)

| leg | chunk-system | cells @180s | overall | best window | TPS 1m/5m |
|-----|--------------|-------------|---------|-------------|-----------|
| A   | auto (1 worker) | 5 998 (9.1%)  | 35.3 cells/s | 276 cells/s | 20.0/19.9 |
| B   | workers=4 io=2  | 13 669 (20.9%) | 72.5 cells/s | 755 cells/s | 20.0/19.4 |
| C   | workers=6 io=4  | 14 441 (22.0%) | 80.2 cells/s | 896 cells/s | 20.0/18.7 |

- B is the shipped default (C buys +11% rate for a deeper early TPS dip;
  on real multi-core hosts the window knobs scale further).
- Warm-world boots sample saved chunks instantly (49 235 cells in seconds)
  — only the never-generated remainder pays gen cost.
- The 2-CPU wall is real: best-window bursts hit 755-896 cells/s (12-14
  chunks/s) then decay as the frontier crosses fresh-generation territory
  while the llvmpipe client eats a core. On unshared cores the curve should
  extend; the knobs (gen_budget/gen_wish, worker-threads) are the levers.

## No-regression verification (final build, defaults 4/2)

Full v5.3.2 stealth protocol against the v5.5 lib (`shots/`):

- Top-down worst case (Y=280, pitch 90): OFF 0 / ON 0 carrier px.
- Mid-band level view at the disk center (inside the spawn cone): OFF 196
  raw carrier px -> ON **0 px, resid 0.0000%**; dome diff 9 997 px (1.08%)
  = the LOD itself painting (the eraser is selective, not a blur).
- MOTION: 90 s spin + hop tour — v06/v07/v08/v09 frames all **0 green px**;
  final A/B after all motion OFF 320 -> ON **0, resid 0.0000%**.
- Pack discipline: sha1 `adeb4d16…`, 69 998 bytes — byte-identical to
  v5.3.3/v5.4 (the frontier is server-side only; the pack never changed).
- TPS 20.0 steady under the full stream; gate counters advance (cone 10/1
  during hops, sky-gate true).
- Tests: 48/48 (46 + 2 new), clippy -D warnings clean, cargo fmt clean.

## Bench notes (for the next operator)

- If `boot_server.sh` reports `http gate: pack served OK`, the pack IS
  downloadable; a boot without the gate message must not be trusted for
  client-side verification.
- The pack-prompt probe (yellow Proceed detection) still races llvmpipe —
  if `pack prompt visible=0`, the client may still have accepted the pack
  (check the client log for the reload listener + the server for
  "effect terrain add"); clicking blind at (561,385) is the fallback.
- scan_rate.py works on any server.log slice; run it before/after config
  changes for an instant frontier health read.
