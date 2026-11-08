# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 11, ours shots: 11, paired: 11, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 01_aerial_west | 68.469 | 87.109 | 11.015 | 234 | FAIL |
| 02_aerial_south | 70.478 | 91.641 | 7.180 | 253 | FAIL |
| 03_aerial_north | 61.857 | 86.390 | 10.706 | 253 | FAIL |
| 04_aerial_east | 75.764 | 89.290 | 7.227 | 248 | FAIL |
| 05_oblique | 94.824 | 99.762 | 0.144 | 18 | FAIL |
| 06_ground_west | 62.991 | 87.268 | 8.420 | 253 | FAIL |
| 07_far_north | 92.312 | 92.312 | 11.519 | 248 | FAIL |
| 08_back_inside | 71.434 | 88.538 | 10.900 | 238 | FAIL |
| 09_after_mesh | 64.054 | 82.619 | 17.370 | 236 | FAIL |
| 10_marker_ray | 75.893 | 85.387 | 14.859 | 240 | FAIL |
| 11_topdown | 33.225 | 86.466 | 4.347 | 222 | FAIL |

**GATE: RED — parity below 100%**
