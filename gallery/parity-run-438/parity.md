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
