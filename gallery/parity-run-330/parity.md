# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 75.193 | 78.726 | 30.866 | 231 | FAIL |
| 01_aerial_west | 58.091 | 69.692 | 20.462 | 251 | FAIL |
| 02_aerial_south | 48.586 | 65.533 | 38.121 | 255 | FAIL |
| 03_aerial_north | 54.488 | 72.738 | 17.620 | 253 | FAIL |
| 04_aerial_east | 42.498 | 49.913 | 58.007 | 255 | FAIL |
| 05_oblique | 92.665 | 97.773 | 1.772 | 210 | FAIL |
| 06_ground_west | 48.664 | 69.794 | 24.093 | 249 | FAIL |
| 07_far_north | 86.768 | 94.241 | 3.247 | 224 | FAIL |
| 08_back_inside | 61.231 | 65.051 | 50.503 | 239 | FAIL |
| 09_after_mesh | 59.188 | 70.633 | 19.626 | 238 | FAIL |
| 10_marker_ray | 49.010 | 65.768 | 36.205 | 236 | FAIL |
| 11_topdown | 27.403 | 66.360 | 9.231 | 224 | FAIL |
| 12_far_north_re | 85.391 | 90.248 | 8.125 | 246 | FAIL |

**GATE: RED — parity below 100%**
