# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 12, ours shots: 12, paired: 12, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 01_aerial_west | 68.467 | 87.266 | 10.959 | 234 | FAIL |
| 02_aerial_south | 70.574 | 91.941 | 6.965 | 253 | FAIL |
| 03_aerial_north | 62.053 | 86.804 | 10.570 | 253 | FAIL |
| 04_aerial_east | 75.980 | 89.545 | 7.188 | 248 | FAIL |
| 05_oblique | 94.824 | 99.762 | 0.144 | 18 | FAIL |
| 06_ground_west | 63.148 | 88.362 | 8.052 | 255 | FAIL |
| 07_far_north | 91.583 | 91.583 | 12.458 | 249 | FAIL |
| 08_back_inside | 69.378 | 80.719 | 25.565 | 238 | FAIL |
| 09_after_mesh | 64.011 | 82.735 | 17.324 | 234 | FAIL |
| 10_marker_ray | 75.956 | 85.490 | 14.852 | 240 | FAIL |
| 11_topdown | 33.523 | 87.493 | 4.284 | 223 | FAIL |
| 12_far_north_re | 85.179 | 85.366 | 20.309 | 249 | FAIL |

**GATE: RED — parity below 100%**
