# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 75.340 | 78.873 | 23.485 | 210 | FAIL |
| 01_aerial_west | 59.934 | 71.232 | 21.268 | 248 | FAIL |
| 02_aerial_south | 49.321 | 66.719 | 32.174 | 253 | FAIL |
| 03_aerial_north | 55.425 | 73.671 | 17.128 | 253 | FAIL |
| 04_aerial_east | 43.970 | 53.434 | 48.674 | 254 | FAIL |
| 05_oblique | 94.089 | 99.084 | 0.282 | 163 | FAIL |
| 06_ground_west | 38.460 | 67.343 | 24.603 | 249 | FAIL |
| 07_far_north | 85.749 | 91.529 | 7.994 | 248 | FAIL |
| 08_back_inside | 57.849 | 61.990 | 49.084 | 239 | FAIL |
| 09_after_mesh | 55.550 | 66.831 | 25.210 | 233 | FAIL |
| 10_marker_ray | 70.946 | 81.416 | 15.913 | 236 | FAIL |
| 11_topdown | 27.598 | 67.004 | 9.070 | 217 | FAIL |
| 12_far_north_re | 85.717 | 91.536 | 7.990 | 243 | FAIL |

**GATE: RED — parity below 100%**
