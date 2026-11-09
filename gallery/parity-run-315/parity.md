# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 80.288 | 84.509 | 17.391 | 223 | FAIL |
| 01_aerial_west | 61.903 | 78.960 | 18.095 | 248 | FAIL |
| 02_aerial_south | 55.194 | 75.495 | 26.339 | 253 | FAIL |
| 03_aerial_north | 53.290 | 75.534 | 19.555 | 253 | FAIL |
| 04_aerial_east | 44.119 | 54.033 | 45.445 | 254 | FAIL |
| 05_oblique | 91.656 | 96.168 | 2.267 | 189 | FAIL |
| 06_ground_west | 53.095 | 80.526 | 13.195 | 249 | FAIL |
| 07_far_north | 85.391 | 90.250 | 8.127 | 248 | FAIL |
| 08_back_inside | 44.344 | 55.902 | 40.039 | 230 | FAIL |
| 09_after_mesh | 41.811 | 58.486 | 32.373 | 235 | FAIL |
| 10_marker_ray | 39.293 | 58.895 | 38.830 | 238 | FAIL |
| 11_topdown | 30.484 | 77.899 | 6.168 | 208 | FAIL |
| 12_far_north_re | 85.390 | 90.249 | 8.128 | 245 | FAIL |

**GATE: RED — parity below 100%**
