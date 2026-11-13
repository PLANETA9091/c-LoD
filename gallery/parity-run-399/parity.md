# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.132 | 84.144 | 18.657 | 209 | FAIL |
| 01_aerial_west | 59.851 | 74.180 | 19.664 | 248 | FAIL |
| 02_aerial_south | 50.965 | 73.544 | 22.689 | 253 | FAIL |
| 03_aerial_north | 54.884 | 77.557 | 15.030 | 253 | FAIL |
| 04_aerial_east | 51.479 | 65.579 | 33.257 | 254 | FAIL |
| 05_oblique | 89.276 | 97.417 | 1.302 | 171 | FAIL |
| 06_ground_west | 39.628 | 72.043 | 22.692 | 248 | FAIL |
| 07_far_north | 85.576 | 90.413 | 8.062 | 248 | FAIL |
| 08_back_inside | 57.511 | 73.961 | 24.054 | 237 | FAIL |
| 09_after_mesh | 49.629 | 58.981 | 34.406 | 236 | FAIL |
| 10_marker_ray | 40.253 | 55.405 | 38.466 | 238 | FAIL |
| 11_topdown | 27.317 | 69.663 | 8.430 | 215 | FAIL |
| 12_far_north_re | 85.438 | 90.383 | 8.246 | 246 | FAIL |

**GATE: RED — parity below 100%**
