# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.200 | 84.123 | 18.475 | 210 | FAIL |
| 01_aerial_west | 58.854 | 72.202 | 20.544 | 251 | FAIL |
| 02_aerial_south | 49.403 | 70.230 | 27.224 | 253 | FAIL |
| 03_aerial_north | 53.699 | 74.348 | 16.054 | 253 | FAIL |
| 04_aerial_east | 51.953 | 64.867 | 34.561 | 254 | FAIL |
| 05_oblique | 86.520 | 96.525 | 2.321 | 169 | FAIL |
| 06_ground_west | 27.192 | 61.159 | 26.189 | 249 | FAIL |
| 07_far_north | 85.481 | 90.280 | 8.100 | 248 | FAIL |
| 08_back_inside | 56.491 | 72.825 | 24.637 | 238 | FAIL |
| 09_after_mesh | 47.815 | 56.828 | 36.549 | 233 | FAIL |
| 10_marker_ray | 40.226 | 54.464 | 38.777 | 236 | FAIL |
| 11_topdown | 26.223 | 66.787 | 8.978 | 223 | FAIL |
| 12_far_north_re | 85.481 | 90.280 | 8.106 | 245 | FAIL |

**GATE: RED — parity below 100%**
