# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 10, ours shots: 10, paired: 10, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 01_aerial_west | 0.063 | 27.331 | 61.188 | 255 | FAIL |
| 02_aerial_south | 0.034 | 20.960 | 99.733 | 255 | FAIL |
| 03_aerial_north | 0.038 | 24.751 | 85.207 | 255 | FAIL |
| 04_aerial_east | 0.037 | 16.495 | 94.833 | 255 | FAIL |
| 05_oblique | 0.002 | 21.567 | 88.217 | 255 | FAIL |
| 06_ground_west | 0.129 | 7.356 | 94.936 | 255 | FAIL |
| 07_far_north | 0.274 | 70.896 | 4.529 | 25 | FAIL |
| 08_back_inside | 0.050 | 21.355 | 68.362 | 255 | FAIL |
| 09_after_mesh | 0.063 | 24.005 | 74.550 | 255 | FAIL |
| 10_marker_ray | 0.075 | 34.623 | 62.113 | 255 | FAIL |

**GATE: RED — parity below 100%**
