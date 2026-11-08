# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 12, ours shots: 12, paired: 12, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 01_aerial_west | 68.786 | 88.312 | 10.742 | 255 | FAIL |
| 02_aerial_south | 70.056 | 90.667 | 7.445 | 255 | FAIL |
| 03_aerial_north | 61.978 | 86.609 | 10.626 | 253 | FAIL |
| 04_aerial_east | 75.287 | 88.123 | 7.570 | 255 | FAIL |
| 05_oblique | 94.823 | 99.762 | 0.145 | 255 | FAIL |
| 06_ground_west | 63.793 | 90.790 | 7.581 | 255 | FAIL |
| 07_far_north | 92.089 | 92.089 | 11.580 | 255 | FAIL |
| 08_back_inside | 71.447 | 88.973 | 10.484 | 255 | FAIL |
| 09_after_mesh | 64.345 | 83.846 | 17.100 | 255 | FAIL |
| 10_marker_ray | 75.904 | 85.354 | 14.886 | 255 | FAIL |
| 11_topdown | 33.810 | 88.628 | 3.704 | 223 | FAIL |
| 12_far_north_re | 85.484 | 87.909 | 9.936 | 255 | FAIL |

**GATE: RED — parity below 100%**
