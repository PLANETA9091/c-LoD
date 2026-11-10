# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 87.314 | 89.809 | 6.009 | 210 | FAIL |
| 01_aerial_west | 55.436 | 65.098 | 24.400 | 249 | FAIL |
| 02_aerial_south | 56.469 | 73.036 | 15.929 | 249 | FAIL |
| 03_aerial_north | 51.864 | 65.685 | 21.685 | 249 | FAIL |
| 04_aerial_east | 61.771 | 68.195 | 18.984 | 250 | FAIL |
| 05_oblique | 86.373 | 91.022 | 7.614 | 190 | FAIL |
| 06_ground_west | 38.455 | 66.694 | 16.889 | 251 | FAIL |
| 07_far_north | 85.561 | 90.420 | 8.066 | 248 | FAIL |
| 08_back_inside | 60.389 | 76.263 | 21.290 | 237 | FAIL |
| 09_after_mesh | 52.879 | 62.997 | 28.402 | 233 | FAIL |
| 10_marker_ray | 51.358 | 64.526 | 33.592 | 236 | FAIL |
| 11_topdown | 31.652 | 83.410 | 4.850 | 217 | FAIL |
| 12_far_north_re | 85.561 | 90.424 | 8.052 | 245 | FAIL |

**GATE: RED — parity below 100%**
