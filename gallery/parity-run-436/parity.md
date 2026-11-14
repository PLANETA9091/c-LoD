# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.473 | 84.576 | 18.094 | 209 | FAIL |
| 01_aerial_west | 58.120 | 72.471 | 21.177 | 248 | FAIL |
| 02_aerial_south | 49.780 | 71.770 | 27.288 | 253 | FAIL |
| 03_aerial_north | 53.250 | 73.731 | 16.729 | 253 | FAIL |
| 04_aerial_east | 51.692 | 65.228 | 35.731 | 254 | FAIL |
| 05_oblique | 85.754 | 95.320 | 3.793 | 171 | FAIL |
| 06_ground_west | 26.100 | 60.664 | 27.465 | 248 | FAIL |
| 07_far_north | 85.787 | 91.602 | 7.871 | 248 | FAIL |
| 08_back_inside | 57.834 | 75.171 | 23.802 | 239 | FAIL |
| 09_after_mesh | 50.658 | 63.124 | 29.729 | 234 | FAIL |
| 10_marker_ray | 41.046 | 58.199 | 37.659 | 237 | FAIL |
| 11_topdown | 28.349 | 73.792 | 6.935 | 215 | FAIL |
| 12_far_north_re | 85.535 | 91.583 | 8.044 | 246 | FAIL |

**GATE: RED — parity below 100%**
