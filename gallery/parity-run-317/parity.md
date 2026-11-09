# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 76.613 | 79.841 | 22.875 | 189 | FAIL |
| 01_aerial_west | 60.646 | 76.456 | 18.234 | 249 | FAIL |
| 02_aerial_south | 52.410 | 72.319 | 30.402 | 253 | FAIL |
| 03_aerial_north | 57.105 | 79.143 | 15.072 | 253 | FAIL |
| 04_aerial_east | 43.664 | 53.079 | 47.754 | 248 | FAIL |
| 05_oblique | 92.378 | 97.491 | 1.984 | 169 | FAIL |
| 06_ground_west | 41.198 | 73.864 | 23.493 | 249 | FAIL |
| 07_far_north | 86.843 | 94.202 | 3.241 | 224 | FAIL |
| 08_back_inside | 59.579 | 63.568 | 49.156 | 239 | FAIL |
| 09_after_mesh | 56.154 | 71.563 | 22.138 | 238 | FAIL |
| 10_marker_ray | 49.166 | 66.792 | 31.064 | 236 | FAIL |
| 11_topdown | 33.404 | 86.861 | 3.984 | 223 | FAIL |
| 12_far_north_re | 85.460 | 90.249 | 8.121 | 245 | FAIL |

**GATE: RED — parity below 100%**
