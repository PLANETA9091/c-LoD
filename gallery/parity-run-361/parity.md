# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 90.500 | 93.913 | 4.909 | 210 | FAIL |
| 01_aerial_west | 56.911 | 67.411 | 23.906 | 249 | FAIL |
| 02_aerial_south | 65.475 | 82.170 | 10.286 | 253 | FAIL |
| 03_aerial_north | 56.821 | 74.900 | 15.484 | 253 | FAIL |
| 04_aerial_east | 59.476 | 67.767 | 28.440 | 254 | FAIL |
| 05_oblique | 91.142 | 95.922 | 2.796 | 193 | FAIL |
| 06_ground_west | 42.340 | 62.910 | 26.613 | 249 | FAIL |
| 07_far_north | 85.477 | 90.319 | 8.094 | 248 | FAIL |
| 08_back_inside | 58.702 | 74.080 | 22.669 | 237 | FAIL |
| 09_after_mesh | 50.257 | 61.294 | 27.032 | 231 | FAIL |
| 10_marker_ray | 52.485 | 62.735 | 36.599 | 236 | FAIL |
| 11_topdown | 31.276 | 82.210 | 5.054 | 222 | FAIL |
| 12_far_north_re | 85.477 | 90.316 | 8.107 | 246 | FAIL |

**GATE: RED — parity below 100%**
