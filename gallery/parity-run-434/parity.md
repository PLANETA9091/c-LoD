# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 78.834 | 84.107 | 18.533 | 209 | FAIL |
| 01_aerial_west | 58.381 | 72.249 | 20.756 | 251 | FAIL |
| 02_aerial_south | 50.320 | 71.973 | 21.955 | 253 | FAIL |
| 03_aerial_north | 53.907 | 74.445 | 15.688 | 253 | FAIL |
| 04_aerial_east | 51.183 | 64.360 | 30.872 | 254 | FAIL |
| 05_oblique | 85.319 | 95.659 | 3.023 | 169 | FAIL |
| 06_ground_west | 24.656 | 58.327 | 26.860 | 249 | FAIL |
| 07_far_north | 85.466 | 90.258 | 8.108 | 248 | FAIL |
| 08_back_inside | 56.325 | 73.126 | 24.758 | 237 | FAIL |
| 09_after_mesh | 47.866 | 57.304 | 36.464 | 234 | FAIL |
| 10_marker_ray | 40.965 | 54.913 | 39.734 | 241 | FAIL |
| 11_topdown | 25.767 | 65.180 | 9.431 | 215 | FAIL |
| 12_far_north_re | 85.317 | 90.245 | 8.267 | 246 | FAIL |

**GATE: RED — parity below 100%**
