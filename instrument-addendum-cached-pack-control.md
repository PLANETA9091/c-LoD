# Instrument addendum v6.10.2c: the CACHED-PACK CONTROL — the decisive A/B on the leading suspect

Date: 2026-11-07. Feed commit 8b16c22 (log-mining note) named the **cached-pack load path** as the leading suspect: leg-1 = first download + prompt + Proceed click (measured, renders), leg-2 = relog into the SAME client dir with the pack already cached (auto-accept, dust = 0 in #248/#250). This addendum converts that suspicion into a one-variable A/B that runs inside the next visual run.

## The client-side fact (verified in the 26.3 client bytecode)

- `Minecraft.class` holds `resourcePackDirectory` + `downloads` — the server-pack cache dir is `<gameDir>/downloads` (26.x layout, `DownloadQueue` `cacheDir` + `download-queue/log.json` event log).
- The harness client dir is fixed: `run_client.sh` → `C="$HOME/mc-test/client"`, reused by BOTH legs, so the pack downloaded on leg-1 is still there on the leg-2 relog.
- The pack is served by the plugin's own HTTP server (`engine.rs: pack_url()` → `http://<ip>:<http_port>/clod-pack.zip`), so a cold client can always re-download it — removing the cache cannot break delivery.
- The harness's own `accept_pack_prompt` already anticipates the asymmetry: `no pack prompt rendered in time (cached pack auto-accept?)` — the prompt fires on leg-1, leg-2 usually loads silently from cache.

## The control (patch: `instrument-addendum-cached-pack-control.patch`, +52 lines)

Applies AFTER `instrument-proposal-leg1-dust-type-matrix.patch` (bfbed5c) and AFTER `instrument-addendum-endrod-planes.patch` (8b16c22). Apply-chain verified: base → endrod-planes → cached-pack-control, `bash -n` green on the full chain, `git apply --check` clean at every step, zero rust touched.

Three insertions in `scripts/bench/run_ci_visual.sh`:

1. **Leg-1 evidence arm** (right after `PROBE_LOG` init): records the pack-flow lines leg-1 actually logged (the first-download arm) into the probe log.
2. **THE CONTROL** (between the kick and the rejoin, client down): inventories `<client>/downloads` (file count + sizes into the probe log), then `rm -rf` it. Leg-2 now re-downloads from the plugin HTTP server and takes the SAME load path leg-1 took. If the dir is absent, the run says so honestly (control = no-op).
3. **Leg-2 evidence arm** (after the 15-s post-accept assert): records leg-2's pack-flow lines + the fresh `download-queue/log.json` — proof the re-download actually happened.

Single variable flips: warm-cache leg-2 (measured twice, dust = 0) → cold-cache leg-2. The leg-1 dust probe and the leg-2 type matrix from bfbed5c read the answer; the end_rod@Y=120 plane control from 8b16c22 rides alongside.

## Decision tree (reads on the SAME run)

| # | Observation | Verdict | Action |
|---|---|---|---|
| 1 | Leg-2 white-dust NEAR > 20 **with the cache wiped** (was ~0 in #248/#250) | **ROOT CAUSE: the cached-pack load path** — the cached apply breaks dust rendering | Fix on the carrier side: re-resolve the pack/particles on a cached apply (re-push per session), or wipe + re-push |
| 2 | Leg-2 matrix unchanged (~0 dust, end_rod alive) with the cache wiped | Cache **exonerated** — the break survives an identical load flow | Next variable up the ladder: the persistent gameDir state (fresh gameDir for leg 2) |
| 3 | Leg-1 green dust > 0 (bfbed5c row 1) AND leg-2 flips alive under the control | Both mechanisms real: session-specific break + cached load path | Fix covers both: fresh load per session |
| 4 | Leg-1 green ~0, white ~0 (bfbed5c row 3) regardless of the control | Dust type unrenderable on the build — P0 | Migrate the carrier type |

Rows 2–4 of the bfbed5c decision tree (color-specific vs type-specific) read as before — the control only sharpens rows 1/5.

## Costs

+52 script lines, zero rust, ~0 s runtime cost (the inventory+remove is sub-second; the re-download adds one prompt cycle the harness already handles). Additive only: no fail/pass path, no threshold, no raw-count series touched. The v6.7.3 preflight still gates the run before any dust flies.
