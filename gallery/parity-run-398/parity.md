# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.157 | 84.262 | 18.389 | 204 | FAIL |
| 01_aerial_west | 58.211 | 72.069 | 20.497 | 254 | FAIL |
| 02_aerial_south | 49.668 | 71.124 | 26.696 | 253 | FAIL |
| 03_aerial_north | 53.387 | 73.803 | 16.193 | 253 | FAIL |
| 04_aerial_east | 51.850 | 64.963 | 35.258 | 254 | FAIL |
| 05_oblique | 86.318 | 95.821 | 2.882 | 169 | FAIL |
| 06_ground_west | 48.695 | 80.010 | 13.611 | 249 | FAIL |
| 07_far_north | 85.297 | 90.234 | 8.285 | 249 | FAIL |
| 08_back_inside | 57.092 | 73.598 | 24.728 | 238 | FAIL |
| 09_after_mesh | 50.467 | 59.697 | 35.172 | 233 | FAIL |
| 10_marker_ray | 40.257 | 53.953 | 39.539 | 238 | FAIL |
| 11_topdown | 27.605 | 70.595 | 8.487 | 216 | FAIL |
| 12_far_north_re | 85.297 | 90.234 | 8.288 | 246 | FAIL |

**GATE: RED — parity below 100%**
