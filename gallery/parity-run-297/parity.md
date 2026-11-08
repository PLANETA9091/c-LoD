# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 12, ours shots: 12, paired: 12, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 01_aerial_west | 68.419 | 87.358 | 11.034 | 238 | FAIL |
| 02_aerial_south | 70.431 | 91.545 | 7.287 | 253 | FAIL |
| 03_aerial_north | 62.096 | 86.966 | 10.509 | 253 | FAIL |
| 04_aerial_east | 76.131 | 89.930 | 7.092 | 248 | FAIL |
| 05_oblique | 94.824 | 99.762 | 0.144 | 18 | FAIL |
| 06_ground_west | 63.065 | 88.228 | 8.142 | 255 | FAIL |
| 07_far_north | 92.313 | 92.313 | 11.506 | 248 | FAIL |
| 08_back_inside | 71.193 | 88.226 | 11.521 | 238 | FAIL |
| 09_after_mesh | 64.002 | 82.978 | 17.381 | 238 | FAIL |
| 10_marker_ray | 75.830 | 85.245 | 14.883 | 240 | FAIL |
| 11_topdown | 33.539 | 87.578 | 4.173 | 223 | FAIL |
| 12_far_north_re | 85.564 | 86.944 | 15.516 | 249 | FAIL |

**GATE: RED — parity below 100%**
