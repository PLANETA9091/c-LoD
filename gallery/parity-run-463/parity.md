# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 77.314 | 82.252 | 21.483 | 204 | FAIL |
| 01_aerial_west | 57.729 | 72.621 | 20.607 | 249 | FAIL |
| 02_aerial_south | 50.134 | 72.711 | 25.699 | 253 | FAIL |
| 03_aerial_north | 54.715 | 76.946 | 17.758 | 253 | FAIL |
| 04_aerial_east | 49.477 | 64.612 | 36.360 | 254 | FAIL |
| 05_oblique | 85.844 | 93.364 | 5.378 | 177 | FAIL |
| 06_ground_west | 33.942 | 68.879 | 23.053 | 250 | FAIL |
| 07_far_north | 85.269 | 90.236 | 8.282 | 249 | FAIL |
| 08_back_inside | 53.716 | 70.835 | 29.611 | 238 | FAIL |
| 09_after_mesh | 45.854 | 61.588 | 35.318 | 237 | FAIL |
| 10_marker_ray | 46.235 | 60.493 | 40.311 | 238 | FAIL |
| 11_topdown | 30.505 | 81.256 | 5.013 | 223 | FAIL |
| 12_far_north_re | 85.269 | 90.233 | 8.290 | 246 | FAIL |

**GATE: RED — parity below 100%**
