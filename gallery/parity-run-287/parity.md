# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 11, ours shots: 11, paired: 11, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 01_aerial_west | 68.346 | 87.601 | 10.885 | 230 | FAIL |
| 02_aerial_south | 70.530 | 92.114 | 7.080 | 253 | FAIL |
| 03_aerial_north | 61.968 | 86.528 | 10.649 | 253 | FAIL |
| 04_aerial_east | 76.032 | 89.703 | 7.113 | 248 | FAIL |
| 05_oblique | 94.824 | 99.762 | 0.144 | 18 | FAIL |
| 06_ground_west | 63.583 | 90.133 | 7.696 | 253 | FAIL |
| 07_far_north | 99.472 | 99.472 | 0.806 | 245 | FAIL |
| 08_back_inside | 69.044 | 78.989 | 26.318 | 238 | FAIL |
| 09_after_mesh | 64.096 | 83.099 | 17.244 | 236 | FAIL |
| 10_marker_ray | 76.040 | 85.562 | 14.803 | 240 | FAIL |
| 11_topdown | 34.702 | 91.406 | 3.069 | 223 | FAIL |

**GATE: RED — parity below 100%**
