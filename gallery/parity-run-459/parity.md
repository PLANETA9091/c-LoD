# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 76.924 | 81.779 | 21.259 | 204 | FAIL |
| 01_aerial_west | 57.670 | 71.741 | 22.129 | 251 | FAIL |
| 02_aerial_south | 48.581 | 69.862 | 28.323 | 253 | FAIL |
| 03_aerial_north | 53.976 | 74.924 | 15.896 | 253 | FAIL |
| 04_aerial_east | 48.758 | 61.702 | 39.142 | 254 | FAIL |
| 05_oblique | 87.797 | 96.034 | 3.187 | 169 | FAIL |
| 06_ground_west | 35.913 | 69.924 | 24.134 | 249 | FAIL |
| 07_far_north | 85.317 | 90.243 | 8.274 | 249 | FAIL |
| 08_back_inside | 53.105 | 69.283 | 29.115 | 227 | FAIL |
| 09_after_mesh | 42.798 | 54.483 | 38.212 | 236 | FAIL |
| 10_marker_ray | 37.348 | 53.555 | 43.937 | 238 | FAIL |
| 11_topdown | 28.172 | 72.543 | 7.436 | 215 | FAIL |
| 12_far_north_re | 85.317 | 90.245 | 8.271 | 246 | FAIL |

**GATE: RED — parity below 100%**
