# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 12, ours shots: 12, paired: 12, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 01_aerial_west | 64.957 | 79.409 | 12.224 | 234 | FAIL |
| 02_aerial_south | 68.754 | 88.075 | 8.385 | 253 | FAIL |
| 03_aerial_north | 54.034 | 70.550 | 19.344 | 253 | FAIL |
| 04_aerial_east | 74.748 | 84.242 | 8.663 | 249 | FAIL |
| 05_oblique | 78.612 | 80.334 | 13.805 | 128 | FAIL |
| 06_ground_west | 62.811 | 87.258 | 8.563 | 253 | FAIL |
| 07_far_north | 86.582 | 89.845 | 12.277 | 226 | FAIL |
| 08_back_inside | 69.218 | 79.998 | 22.882 | 238 | FAIL |
| 09_after_mesh | 60.073 | 69.432 | 23.291 | 233 | FAIL |
| 10_marker_ray | 73.525 | 81.614 | 14.685 | 247 | FAIL |
| 11_topdown | 33.704 | 88.063 | 3.912 | 223 | FAIL |
| 12_far_north_re | 85.630 | 90.415 | 8.059 | 245 | FAIL |

**GATE: RED — parity below 100%**
