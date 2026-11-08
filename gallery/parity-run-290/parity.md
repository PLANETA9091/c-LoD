# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 12, ours shots: 12, paired: 12, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 01_aerial_west | 68.477 | 87.486 | 10.906 | 234 | FAIL |
| 02_aerial_south | 70.609 | 92.099 | 6.936 | 253 | FAIL |
| 03_aerial_north | 61.907 | 86.489 | 10.644 | 253 | FAIL |
| 04_aerial_east | 75.789 | 89.022 | 7.349 | 248 | FAIL |
| 05_oblique | 94.824 | 99.762 | 0.144 | 18 | FAIL |
| 06_ground_west | 63.327 | 89.534 | 7.715 | 255 | FAIL |
| 07_far_north | 92.312 | 92.312 | 11.519 | 248 | FAIL |
| 08_back_inside | 71.243 | 88.236 | 10.617 | 255 | FAIL |
| 09_after_mesh | 64.030 | 82.948 | 17.269 | 234 | FAIL |
| 10_marker_ray | 75.832 | 85.366 | 14.939 | 240 | FAIL |
| 11_topdown | 34.016 | 89.324 | 3.659 | 223 | FAIL |
| 12_far_north_re | 85.615 | 87.781 | 11.667 | 248 | FAIL |

**GATE: RED — parity below 100%**
