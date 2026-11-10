# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 75.009 | 78.431 | 23.805 | 210 | FAIL |
| 01_aerial_west | 59.784 | 70.934 | 21.613 | 248 | FAIL |
| 02_aerial_south | 48.364 | 65.631 | 33.326 | 253 | FAIL |
| 03_aerial_north | 54.569 | 72.262 | 18.275 | 253 | FAIL |
| 04_aerial_east | 42.389 | 51.537 | 50.739 | 254 | FAIL |
| 05_oblique | 94.138 | 98.962 | 0.358 | 169 | FAIL |
| 06_ground_west | 32.735 | 62.345 | 26.406 | 249 | FAIL |
| 07_far_north | 85.301 | 90.253 | 8.204 | 248 | FAIL |
| 08_back_inside | 60.559 | 77.221 | 21.328 | 238 | FAIL |
| 09_after_mesh | 54.130 | 65.527 | 27.165 | 237 | FAIL |
| 10_marker_ray | 51.807 | 65.778 | 32.094 | 238 | FAIL |
| 11_topdown | 32.112 | 85.570 | 4.286 | 216 | FAIL |
| 12_far_north_re | 85.298 | 90.234 | 8.282 | 246 | FAIL |

**GATE: RED — parity below 100%**
