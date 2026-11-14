# PARITY RUN 438 (run 43648488499, sha e01eee9) — gate: False

# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.173 | 84.228 | 18.295 | 209 | FAIL |
| 01_aerial_west | 62.042 | 78.528 | 18.546 | 248 | FAIL |
| 02_aerial_south | 49.611 | 71.309 | 26.919 | 253 | FAIL |
| 03_aerial_north | 54.541 | 75.471 | 15.599 | 253 | FAIL |
| 04_aerial_east | 51.963 | 65.896 | 34.923 | 254 | FAIL |
| 05_oblique | 91.740 | 98.962 | 0.379 | 169 | FAIL |
| 06_ground_west | 53.257 | 82.822 | 13.896 | 249 | FAIL |
| 07_far_north | 85.333 | 90.309 | 8.268 | 249 | FAIL |
| 08_back_inside | 58.170 | 74.882 | 23.933 | 238 | FAIL |
| 09_after_mesh | 48.922 | 58.619 | 36.392 | 236 | FAIL |
| 10_marker_ray | 40.348 | 53.863 | 39.972 | 238 | FAIL |
| 11_topdown | 26.217 | 65.039 | 9.381 | 215 | FAIL |
| 12_far_north_re | 85.333 | 90.309 | 8.272 | 246 | FAIL |

**GATE: RED — parity below 100%**

## voxy | ours | diff (triptychs)

- ![tri_00_summit_rise.jpg](parity-run-438/tri_00_summit_rise.jpg)
- ![tri_01_aerial_west.jpg](parity-run-438/tri_01_aerial_west.jpg)
- ![tri_02_aerial_south.jpg](parity-run-438/tri_02_aerial_south.jpg)
- ![tri_03_aerial_north.jpg](parity-run-438/tri_03_aerial_north.jpg)
- ![tri_04_aerial_east.jpg](parity-run-438/tri_04_aerial_east.jpg)
- ![tri_05_oblique.jpg](parity-run-438/tri_05_oblique.jpg)
- ![tri_06_ground_west.jpg](parity-run-438/tri_06_ground_west.jpg)
- ![tri_07_far_north.jpg](parity-run-438/tri_07_far_north.jpg)
- ![tri_08_back_inside.jpg](parity-run-438/tri_08_back_inside.jpg)
- ![tri_09_after_mesh.jpg](parity-run-438/tri_09_after_mesh.jpg)
- ![tri_10_marker_ray.jpg](parity-run-438/tri_10_marker_ray.jpg)
- ![tri_11_topdown.jpg](parity-run-438/tri_11_topdown.jpg)
- ![tri_12_far_north_re.jpg](parity-run-438/tri_12_far_north_re.jpg)
- time audit: [time-audit.md](parity-run-438/time-audit.md) — the §7b console channel; the #332 night-matrix class made self-diagnosing (v6.25.5)
- view probe: [view_probe.md](parity-run-438/view_probe.md) — per-position q/settle state, the v6.26.6 settle-by-laps + dip-probe channel
