# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 68.308 | 75.036 | 30.631 | 241 | FAIL |
| 01_aerial_west | 54.216 | 65.405 | 28.650 | 251 | FAIL |
| 02_aerial_south | 47.218 | 66.245 | 29.534 | 253 | FAIL |
| 03_aerial_north | 51.360 | 71.643 | 19.620 | 253 | FAIL |
| 04_aerial_east | 53.806 | 69.059 | 25.366 | 254 | FAIL |
| 05_oblique | 76.866 | 90.987 | 7.760 | 248 | FAIL |
| 06_ground_west | 39.008 | 65.621 | 19.590 | 250 | FAIL |
| 07_far_north | 85.269 | 90.234 | 8.295 | 249 | FAIL |
| 08_back_inside | 44.224 | 64.558 | 35.445 | 227 | FAIL |
| 09_after_mesh | 32.708 | 46.394 | 51.773 | 236 | FAIL |
| 10_marker_ray | 38.848 | 55.466 | 40.072 | 253 | FAIL |
| 11_topdown | 22.923 | 66.362 | 8.662 | 216 | FAIL |
| 12_far_north_re | 85.269 | 90.237 | 8.278 | 246 | FAIL |

**GATE: RED — parity below 100%**
