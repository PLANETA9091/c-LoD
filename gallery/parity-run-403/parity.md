# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.472 | 84.619 | 17.867 | 210 | FAIL |
| 01_aerial_west | 62.200 | 79.151 | 18.420 | 249 | FAIL |
| 02_aerial_south | 50.970 | 73.338 | 21.400 | 253 | FAIL |
| 03_aerial_north | 53.789 | 75.388 | 15.680 | 253 | FAIL |
| 04_aerial_east | 52.611 | 66.453 | 29.121 | 254 | FAIL |
| 05_oblique | 92.162 | 98.966 | 0.374 | 169 | FAIL |
| 06_ground_west | 51.111 | 80.118 | 17.165 | 249 | FAIL |
| 07_far_north | 85.533 | 91.669 | 7.879 | 248 | FAIL |
| 08_back_inside | 58.839 | 75.941 | 23.494 | 238 | FAIL |
| 09_after_mesh | 48.565 | 57.945 | 35.263 | 233 | FAIL |
| 10_marker_ray | 40.142 | 54.389 | 39.585 | 238 | FAIL |
| 11_topdown | 26.577 | 67.514 | 8.741 | 216 | FAIL |
| 12_far_north_re | 85.537 | 91.662 | 7.938 | 246 | FAIL |

**GATE: RED — parity below 100%**
