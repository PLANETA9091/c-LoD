# e2e v5.6 PRE-JOIN ARM + crash forensics closure (2026-10-30)

## Part 1 — CI un-broken after 4 days red (the tests story)

All 30+ GitHub Actions runs since 2026-10-28 were `failure` with **zero jobs**:
the v5-era `ci.yml` edit placed a python heredoc body at column 0 inside a
`run: |` block, which terminated the YAML block scalar — GitHub rejected the
workflow before any job was created. A second latent parse error (`name:` with
an unquoted `v5:` inside) and silently-truncated step names (`Boot #1 ...` —
` #` starts a YAML comment) were fixed in the same pass.

With the workflow alive again, the first real run (b7c9ec4) exposed two more
stale gates, both fixed:

1. **`sky-grid stream armed` could never fire on CI** — arming was
   player-anchored (`stream_step` early-returns with no players), and the CI
   server has nobody joining. -> **v5.6 PRE-JOIN ARM** (see below).
2. **Pack entry count 12 -> 13** — v5.3.2 added `dust_mask.fsh` (the
   zero-residual dust_eater pass) four days ago; the inline zip assertion was
   never updated (masked by the broken YAML). Now asserts 13 entries +
   `dust_mask.fsh` presence + the `clod:post/dust_mask` pass in the chain.

Result: **run 841d5a0 = full green** (rustfmt, clippy -D warnings, tests 48/48,
glslang, 3 OS release builds, packaging, e2e-live two real Paper boots, strict
gate) — first fully-green CI since 2026-10-27.

## Part 2 — v5.6 PRE-JOIN ARM (the code change)

- `arm_from_config`: with `center_x`/`center_z` configured the streamer arms
  the moment the world resolves — pure Rust state, zero JNI. Headless boots
  (CI, ops) get the armed disk and warm scan before anybody joins; the
  player-anchored path (no config center) is unchanged.
- `stream_scan` pre-join warm scan: with nobody online it still samples cells
  whose chunks are ALREADY loaded, but the async force frontier stays strictly
  player-gated (`allow_force=false` on idle — no tickets churned; the v5.0.4
  crash mitigation is preserved; unloaded cells just skip).
- Local gates: fmt clean, 48/48 tests, clippy -D warnings, release build.

## Part 3 — Live stand verification (v5.6 deployed)

- Boot 18:31:58: pack finalized + **stream armed at 18:31:58**; the player
  joins at 18:32:30 — arming happened 32 s BEFORE any player existed
  (`boot_order_evidence.txt`).
- Warm world: 54874/65536 cells already valid at tick 200 (20 s), 61676 by
  tick 4000; particles streamed from the first second (spawned 1M+ by tick
  4000, failed 0).
- Stealth intact on live frames: pitch -10 horizon shot =
  `03_pitch-10_lod_horizon_0carrier.png`, **0 green carrier pixels**
  (dust_eater + capture pipeline untouched by v5.6).
- TPS 20.0/20.0/19.4 under full stream.

## Part 4 — THE IDLE CRASH: caught, attributed, closed

The last backlog defect ("SIGSEGV pc=0 ~5-18 min after the last player left,
scan_cell suspected") is now explained by direct evidence:

- **Caught live**: `hs_err_pid16818_signal_storm.log` — the v5.5 server
  SIGSEGV'd at pc=0x0 at **18:25:52**, elapsed 2452 s, i.e. **18 m 39 s after
  the player left** (18:07:13) — the edge of the historical window. Thread:
  `Signal Dispatcher`, Java frame `jdk.internal.misc.Signal.dispatch(I)V`,
  call through a NULL pointer (`~StubRoutines::call_stub` -> RIP=0). The
  hs_err's own error reporter crashed twice while printing stacks (stacks in
  older dumps — including the "scan_cell" attribution — are unreliable).
- **Trigger**: this SIGSEGV landed while `boot_server.sh`'s old shutdown
  sequence was running: `pkill` (TERM) + a port-wait loop that fires
  `pkill -9` (KILL) 1 s later — a signal barrage into a mid-shutdown JVM.
- **Controlled experiments (v5.6)**:
  - single clean SIGTERM -> graceful exit 143, **no hs_err**;
  - simultaneous double TERM (launcher+paper) -> clean exit, **no hs_err**;
  - 18.5 min idle soak (player left 18:07:13, monitored to 18:25:47) -> no
    crash while nobody was killing anything.
- **Conclusion**: the "idle crash" was never idle — it was the session-teardown
  signal storm (historical sessions ended by killing the process group).
  Fixed by **shutdown discipline**: TERM once -> wait up to 30 s for a
  graceful exit -> KILL only as last resort. Applied to `boot_server.sh` and
  to both CI e2e-live stop blocks.
- The module itself needs no change: its threads never touch signals, and the
  crash is below the module (JVM signal machinery + agent teardown).

## Artifacts

- `hs_err_pid16818_signal_storm.log` — the caught crash (v5.5, TERM+KILL barrage)
- `boot_order_evidence.txt` — arming-before-join ordering + scan growth
- `01..03_*.png` — first frame, noon, pitch -10 horizon (0 carrier px)
