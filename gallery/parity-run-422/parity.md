# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.423 | 84.485 | 17.956 | 205 | FAIL |
| 01_aerial_west | 60.687 | 75.858 | 19.236 | 248 | FAIL |
| 02_aerial_south | 49.903 | 71.703 | 26.949 | 253 | FAIL |
| 03_aerial_north | 54.078 | 75.094 | 15.749 | 253 | FAIL |
| 04_aerial_east | 52.071 | 65.261 | 34.672 | 255 | FAIL |
| 05_oblique | 89.834 | 97.985 | 0.960 | 169 | FAIL |
| 06_ground_west | 45.535 | 77.029 | 18.733 | 249 | FAIL |
| 07_far_north | 85.292 | 90.238 | 8.262 | 249 | FAIL |
| 08_back_inside | 55.824 | 73.020 | 26.347 | 238 | FAIL |
| 09_after_mesh | 45.836 | 55.426 | 38.314 | 237 | FAIL |
| 10_marker_ray | 40.632 | 54.853 | 38.256 | 238 | FAIL |
| 11_topdown | 25.994 | 66.414 | 8.945 | 216 | FAIL |
| 12_far_north_re | 85.299 | 90.235 | 8.296 | 246 | FAIL |

**GATE: RED — parity below 100%**
