# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 77.220 | 82.478 | 21.095 | 218 | FAIL |
| 01_aerial_west | 62.396 | 79.031 | 17.714 | 248 | FAIL |
| 02_aerial_south | 49.213 | 72.408 | 25.682 | 253 | FAIL |
| 03_aerial_north | 52.023 | 73.024 | 18.798 | 253 | FAIL |
| 04_aerial_east | 53.121 | 69.350 | 28.031 | 254 | FAIL |
| 05_oblique | 76.600 | 84.795 | 17.355 | 222 | FAIL |
| 06_ground_west | 33.361 | 67.410 | 23.069 | 249 | FAIL |
| 07_far_north | 85.238 | 90.296 | 8.268 | 249 | FAIL |
| 08_back_inside | 51.386 | 69.327 | 26.517 | 226 | FAIL |
| 09_after_mesh | 40.367 | 54.540 | 39.577 | 237 | FAIL |
| 10_marker_ray | 47.794 | 65.900 | 29.848 | 238 | FAIL |
| 11_topdown | 27.637 | 75.309 | 6.330 | 216 | FAIL |
| 12_far_north_re | 85.238 | 90.293 | 8.283 | 246 | FAIL |

**GATE: RED — parity below 100%**
