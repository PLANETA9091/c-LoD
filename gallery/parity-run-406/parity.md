# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.109 | 84.253 | 18.210 | 209 | FAIL |
| 01_aerial_west | 58.204 | 72.737 | 20.427 | 254 | FAIL |
| 02_aerial_south | 49.787 | 71.948 | 26.583 | 253 | FAIL |
| 03_aerial_north | 54.752 | 76.236 | 14.990 | 253 | FAIL |
| 04_aerial_east | 51.552 | 65.169 | 34.615 | 254 | FAIL |
| 05_oblique | 85.518 | 94.814 | 3.797 | 169 | FAIL |
| 06_ground_west | 33.882 | 67.766 | 25.083 | 249 | FAIL |
| 07_far_north | 85.537 | 91.663 | 7.937 | 249 | FAIL |
| 08_back_inside | 56.732 | 74.170 | 24.606 | 238 | FAIL |
| 09_after_mesh | 48.064 | 58.345 | 35.283 | 233 | FAIL |
| 10_marker_ray | 44.848 | 57.665 | 35.294 | 238 | FAIL |
| 11_topdown | 27.212 | 68.978 | 8.317 | 216 | FAIL |
| 12_far_north_re | 85.537 | 91.661 | 7.942 | 246 | FAIL |

**GATE: RED — parity below 100%**
