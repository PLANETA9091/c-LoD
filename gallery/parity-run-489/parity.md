# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 67.389 | 76.288 | 27.227 | 220 | FAIL |
| 01_aerial_west | 49.924 | 59.777 | 35.136 | 251 | FAIL |
| 02_aerial_south | 42.215 | 59.468 | 33.819 | 253 | FAIL |
| 03_aerial_north | 49.051 | 66.780 | 22.388 | 253 | FAIL |
| 04_aerial_east | 49.852 | 63.270 | 25.739 | 254 | FAIL |
| 05_oblique | 68.511 | 89.639 | 7.597 | 232 | FAIL |
| 06_ground_west | 45.271 | 62.289 | 19.755 | 247 | FAIL |
| 07_far_north | 85.481 | 90.276 | 8.112 | 248 | FAIL |
| 08_back_inside | 65.493 | 80.875 | 16.683 | 237 | FAIL |
| 09_after_mesh | 57.022 | 67.526 | 25.609 | 233 | FAIL |
| 10_marker_ray | 33.690 | 48.782 | 43.545 | 251 | FAIL |
| 11_topdown | 25.405 | 60.948 | 10.867 | 223 | FAIL |
| 12_far_north_re | 85.480 | 90.270 | 8.133 | 245 | FAIL |

**GATE: RED — parity below 100%**
