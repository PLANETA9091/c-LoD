# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 12, ours shots: 12, paired: 12, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 01_aerial_west | 68.670 | 87.999 | 10.822 | 234 | FAIL |
| 02_aerial_south | 70.441 | 91.497 | 7.075 | 253 | FAIL |
| 03_aerial_north | 61.907 | 86.454 | 10.687 | 253 | FAIL |
| 04_aerial_east | 75.460 | 88.690 | 7.392 | 248 | FAIL |
| 05_oblique | 94.824 | 99.762 | 0.144 | 18 | FAIL |
| 06_ground_west | 63.529 | 89.754 | 7.679 | 255 | FAIL |
| 07_far_north | 97.873 | 97.873 | 3.074 | 245 | FAIL |
| 08_back_inside | 71.610 | 89.112 | 10.691 | 255 | FAIL |
| 09_after_mesh | 64.267 | 83.435 | 17.126 | 234 | FAIL |
| 10_marker_ray | 75.780 | 85.232 | 14.967 | 240 | FAIL |
| 11_topdown | 33.629 | 87.943 | 4.019 | 223 | FAIL |
| 12_far_north_re | 85.582 | 87.275 | 12.642 | 248 | FAIL |

**GATE: RED — parity below 100%**
