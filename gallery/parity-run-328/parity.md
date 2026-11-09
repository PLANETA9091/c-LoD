# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 76.042 | 79.550 | 22.464 | 210 | FAIL |
| 01_aerial_west | 58.331 | 69.760 | 20.444 | 248 | FAIL |
| 02_aerial_south | 49.310 | 66.666 | 31.595 | 253 | FAIL |
| 03_aerial_north | 54.447 | 72.324 | 17.311 | 249 | FAIL |
| 04_aerial_east | 46.013 | 55.250 | 45.918 | 254 | FAIL |
| 05_oblique | 93.102 | 97.892 | 1.007 | 169 | FAIL |
| 06_ground_west | 57.540 | 78.521 | 13.325 | 249 | FAIL |
| 07_far_north | 86.692 | 91.778 | 8.883 | 226 | FAIL |
| 08_back_inside | 61.627 | 66.833 | 47.124 | 239 | FAIL |
| 09_after_mesh | 59.712 | 70.764 | 21.947 | 238 | FAIL |
| 10_marker_ray | 71.049 | 81.433 | 15.932 | 236 | FAIL |
| 11_topdown | 27.452 | 66.324 | 9.285 | 223 | FAIL |
| 12_far_north_re | 85.465 | 90.260 | 8.111 | 245 | FAIL |

**GATE: RED — parity below 100%**
