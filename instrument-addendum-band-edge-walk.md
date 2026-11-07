# instrument addendum v6.12 — the BAND-EDGE WALK (graded 30-80 m end_rod bursts)

Applies AFTER the v6.11 chain (i.e. on top of `instrument-chain-full-v6.11.patch`, or on the already-instrumented branch `instrument/v6.11` @ 5b0b4f1). Zero rust, `bash -n` green, +46 lines, ~+45 s runtime.

## Why (from run 42355686504, VERDICT [C])

The first instrumented run exonerated the packet shape (count=0 per-dot grid rendered 692/715/765 on control/leg-1/leg-2), the carrier type (raw 458; packed zeros identical on both legs = the eraser's design read) and the eraser (alive on leg-2) — and measured the surviving mechanism: **a VANILLA end_rod burst at the marker zone read 0 on the HEALTHY leg while the same burst at 20 m read 524.** Nothing renders beyond the client's loaded chunk radius (view-distance 4 = 64 m); the stream's dots live at 75-105 m. The remaining unknown is the EXACT cut radius (and whether leg-2's cut differs — the run-to-run flap of #246's 353 vs #253's 0 needs a per-run number).

## The cells

`stream_burst`-free, minimal: six `/particle end_rod` bursts at **30/40/50/60/70/80 m** along the marker azimuth (VX+D, Y=150, CZ), one type (eraser-immune white), only the distance varies. Each step diffs against the previous shot (the ~3 s cadence leaves the previous step's dots dead — a dying neighbour inflates `total`, never `white`).

- leg-1 walk: after the v6.11 stream cells, before the live edit (baseline `11_baseline_settle` chain); shots `15_bandwalk_leg1_<D>m`; `sum` lines `- leg-1 band walk <D> m: changed-white N / total M vs the previous step (…)`.
- leg-2 walk: after the leg-2 stream cells, before the relog probes (baseline `14_leg2_baseline` chain); shots `15_bandwalk_leg2_<D>m`; the same `sum` lines plus `probe_counts.txt` colon lines.

## Decision rows (triage.py v6.11.1 reads them automatically)

| reading | meaning | action |
|---|---|---|
| largest alive step < 75 m (both legs, same cut) | the dots' band is client-culled — the shared radius explains the death; the flap (#246 353 vs #253 0) rides this radius | ENGINE fix: `ZONE_FAR`/dot geometry inside the measured cut (or re-plan the carrier geometry) |
| largest alive step < 75 m on leg-2 only, leg-1 cut ≥ 75 | a RELOG-SPECIFIC radius shift | the scene-dead class and the band death share a mechanism — client-side relog state is the target |
| largest alive step ≥ 75 m | the band is INSIDE the render radius — the cull does NOT explain the death | escalate to [B]: engine diagnostic mode `CLOD_UNBUNDLED=1` (the ClientboundBundlePacket wrapper) |
| all steps 0 including 30 m | a worldless/renderer-dead client, not a radius | re-run; the burst_close gate should have caught it |

## Apply

```bash
git apply instrument-addendum-band-edge-walk.patch   # after the v6.11 chain
# or the one-command path:
git apply instrument-chain-full-v6.12.patch          # = 35eb53b + full chain + this addendum
python3 triage.py <run summary.md>                    # the verdict, including the walk's cut
```

Verified: cumulative `instrument-chain-full-v6.12.patch` on a clean 35eb53b worktree — `git apply --check` green, `bash -n` green, sha1 `af25c1fb` byte-identical to the step-by-step chain (v6.11 cumulative + this addendum).
