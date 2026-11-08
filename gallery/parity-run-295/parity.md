# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 12, ours shots: 12, paired: 12, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 01_aerial_west | 68.462 | 87.482 | 10.907 | 234 | FAIL |
| 02_aerial_south | 70.608 | 92.083 | 6.928 | 253 | FAIL |
| 03_aerial_north | 61.862 | 86.410 | 10.707 | 253 | FAIL |
| 04_aerial_east | 75.702 | 89.283 | 7.288 | 248 | FAIL |
| 05_oblique | 94.824 | 99.762 | 0.144 | 18 | FAIL |
| 06_ground_west | 63.414 | 89.621 | 7.699 | 255 | FAIL |
| 07_far_north | 96.980 | 96.981 | 3.470 | 247 | FAIL |
| 08_back_inside | 71.178 | 88.229 | 10.566 | 255 | FAIL |
| 09_after_mesh | 64.017 | 82.916 | 17.274 | 234 | FAIL |
| 10_marker_ray | 75.913 | 85.447 | 14.874 | 240 | FAIL |
| 11_topdown | 33.933 | 88.704 | 3.717 | 223 | FAIL |
| 12_far_north_re | 85.337 | 87.814 | 9.721 | 255 | FAIL |

**GATE: RED — parity below 100%**
