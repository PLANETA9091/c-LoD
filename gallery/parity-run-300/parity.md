# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 12, ours shots: 12, paired: 12, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 01_aerial_west | 68.491 | 87.264 | 10.934 | 255 | FAIL |
| 02_aerial_south | 70.319 | 91.201 | 7.324 | 255 | FAIL |
| 03_aerial_north | 61.653 | 85.851 | 10.727 | 253 | FAIL |
| 04_aerial_east | 75.636 | 88.426 | 7.390 | 255 | FAIL |
| 05_oblique | 94.823 | 99.761 | 0.145 | 255 | FAIL |
| 06_ground_west | 63.481 | 89.321 | 7.835 | 255 | FAIL |
| 07_far_north | 91.891 | 91.891 | 12.175 | 255 | FAIL |
| 08_back_inside | 67.438 | 71.674 | 38.213 | 255 | FAIL |
| 09_after_mesh | 64.046 | 82.775 | 17.296 | 255 | FAIL |
| 10_marker_ray | 75.847 | 85.281 | 14.947 | 255 | FAIL |
| 11_topdown | 33.840 | 88.392 | 3.919 | 223 | FAIL |
| 12_far_north_re | 85.178 | 85.366 | 20.296 | 255 | FAIL |

**GATE: RED — parity below 100%**
