# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 12, ours shots: 12, paired: 12, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 01_aerial_west | 68.719 | 88.169 | 10.748 | 234 | FAIL |
| 02_aerial_south | 70.610 | 92.088 | 7.030 | 253 | FAIL |
| 03_aerial_north | 62.104 | 86.905 | 10.474 | 253 | FAIL |
| 04_aerial_east | 75.987 | 89.523 | 7.091 | 248 | FAIL |
| 05_oblique | 94.824 | 99.762 | 0.144 | 18 | FAIL |
| 06_ground_west | 63.592 | 89.348 | 7.929 | 255 | FAIL |
| 07_far_north | 85.367 | 85.367 | 20.424 | 249 | FAIL |
| 08_back_inside | 71.462 | 88.313 | 13.525 | 238 | FAIL |
| 09_after_mesh | 64.250 | 83.579 | 17.119 | 234 | FAIL |
| 10_marker_ray | 75.913 | 85.445 | 14.901 | 240 | FAIL |
| 11_topdown | 34.080 | 89.316 | 3.542 | 223 | FAIL |
| 12_far_north_re | 85.548 | 86.769 | 15.473 | 248 | FAIL |

**GATE: RED — parity below 100%**
