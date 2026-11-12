# PARITY RUN 386 (run 43262992452, sha ffa57ff) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 1, paired: 1, missing (voxy shot with no ours pair): 12
- ours pair sources: . = 1
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 01_aerial_west | 21.544 | 33.492 | 33.205 | 255 | FAIL |
| 00_summit_rise | — | — | — | — | FAIL (no ours shot) |
| 02_aerial_south | — | — | — | — | FAIL (no ours shot) |
| 03_aerial_north | — | — | — | — | FAIL (no ours shot) |
| 04_aerial_east | — | — | — | — | FAIL (no ours shot) |
| 05_oblique | — | — | — | — | FAIL (no ours shot) |
| 06_ground_west | — | — | — | — | FAIL (no ours shot) |
| 07_far_north | — | — | — | — | FAIL (no ours shot) |
| 08_back_inside | — | — | — | — | FAIL (no ours shot) |
| 09_after_mesh | — | — | — | — | FAIL (no ours shot) |
| 10_marker_ray | — | — | — | — | FAIL (no ours shot) |
| 11_topdown | — | — | — | — | FAIL (no ours shot) |
| 12_far_north_re | — | — | — | — | FAIL (no ours shot) |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_01_aerial_west.jpg](parity-run-386/tri_01_aerial_west.jpg)
- time audit: [time-audit.md](parity-run-386/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-386/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
