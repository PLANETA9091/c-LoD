# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 77.364 | 82.485 | 19.980 | 209 | FAIL |
| 01_aerial_west | 55.396 | 70.118 | 23.236 | 251 | FAIL |
| 02_aerial_south | 47.909 | 69.745 | 29.272 | 253 | FAIL |
| 03_aerial_north | 54.331 | 76.004 | 15.966 | 253 | FAIL |
| 04_aerial_east | 46.844 | 61.149 | 40.623 | 254 | FAIL |
| 05_oblique | 84.283 | 93.481 | 5.977 | 172 | FAIL |
| 06_ground_west | 28.937 | 65.023 | 23.982 | 248 | FAIL |
| 07_far_north | 85.771 | 91.601 | 7.871 | 248 | FAIL |
| 08_back_inside | 52.495 | 70.168 | 28.719 | 237 | FAIL |
| 09_after_mesh | 41.506 | 51.785 | 42.960 | 236 | FAIL |
| 10_marker_ray | 40.834 | 54.845 | 41.240 | 238 | FAIL |
| 11_topdown | 27.197 | 69.553 | 8.380 | 216 | FAIL |
| 12_far_north_re | 85.535 | 91.580 | 8.062 | 246 | FAIL |

**GATE: RED — parity below 100%**
