# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 67.941 | 77.036 | 26.017 | 241 | FAIL |
| 01_aerial_west | 47.850 | 60.040 | 34.025 | 249 | FAIL |
| 02_aerial_south | 39.708 | 56.464 | 36.337 | 255 | FAIL |
| 03_aerial_north | 46.855 | 63.022 | 26.417 | 255 | FAIL |
| 04_aerial_east | 50.852 | 64.501 | 24.294 | 255 | FAIL |
| 05_oblique | 73.674 | 88.562 | 10.089 | 222 | FAIL |
| 06_ground_west | 41.380 | 64.505 | 16.938 | 251 | FAIL |
| 07_far_north | 85.394 | 90.259 | 8.109 | 248 | FAIL |
| 08_back_inside | 42.435 | 64.396 | 33.925 | 237 | FAIL |
| 09_after_mesh | 31.943 | 47.465 | 48.587 | 233 | FAIL |
| 10_marker_ray | 40.888 | 59.084 | 34.979 | 240 | FAIL |
| 11_topdown | 22.800 | 68.151 | 7.992 | 223 | FAIL |
| 12_far_north_re | 85.394 | 90.261 | 8.101 | 245 | FAIL |

**GATE: RED — parity below 100%**
