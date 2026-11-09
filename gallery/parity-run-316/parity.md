# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 78.693 | 82.696 | 18.928 | 195 | FAIL |
| 01_aerial_west | 53.497 | 70.179 | 22.842 | 251 | FAIL |
| 02_aerial_south | 47.700 | 67.469 | 36.955 | 253 | FAIL |
| 03_aerial_north | 52.189 | 73.474 | 19.365 | 253 | FAIL |
| 04_aerial_east | 39.269 | 47.932 | 53.527 | 254 | FAIL |
| 05_oblique | 91.264 | 97.541 | 1.813 | 189 | FAIL |
| 06_ground_west | 38.077 | 74.155 | 16.135 | 251 | FAIL |
| 07_far_north | 86.267 | 89.645 | 12.420 | 226 | FAIL |
| 08_back_inside | 49.133 | 60.942 | 37.804 | 232 | FAIL |
| 09_after_mesh | 46.007 | 62.380 | 30.301 | 232 | FAIL |
| 10_marker_ray | 37.039 | 57.261 | 39.233 | 251 | FAIL |
| 11_topdown | 26.984 | 72.886 | 8.511 | 199 | FAIL |
| 12_far_north_re | 85.733 | 91.529 | 7.991 | 243 | FAIL |

**GATE: RED — parity below 100%**
