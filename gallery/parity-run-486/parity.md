# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 66.566 | 76.002 | 27.264 | 218 | FAIL |
| 01_aerial_west | 48.749 | 57.539 | 36.974 | 251 | FAIL |
| 02_aerial_south | 43.964 | 61.084 | 33.124 | 253 | FAIL |
| 03_aerial_north | 45.533 | 59.133 | 27.421 | 255 | FAIL |
| 04_aerial_east | 50.529 | 64.644 | 26.444 | 254 | FAIL |
| 05_oblique | 64.526 | 83.824 | 11.590 | 222 | FAIL |
| 06_ground_west | 45.209 | 58.824 | 21.654 | 251 | FAIL |
| 07_far_north | 85.486 | 90.396 | 8.226 | 249 | FAIL |
| 08_back_inside | 40.556 | 62.135 | 34.053 | 227 | FAIL |
| 09_after_mesh | 29.570 | 44.987 | 53.122 | 232 | FAIL |
| 10_marker_ray | 38.873 | 50.922 | 37.257 | 251 | FAIL |
| 11_topdown | 20.025 | 58.959 | 10.894 | 223 | FAIL |
| 12_far_north_re | 85.486 | 90.390 | 8.245 | 246 | FAIL |

**GATE: RED — parity below 100%**
