# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 78.901 | 83.571 | 18.920 | 204 | FAIL |
| 01_aerial_west | 61.852 | 77.819 | 18.490 | 249 | FAIL |
| 02_aerial_south | 49.984 | 71.774 | 26.693 | 253 | FAIL |
| 03_aerial_north | 54.358 | 75.996 | 15.350 | 253 | FAIL |
| 04_aerial_east | 51.117 | 65.006 | 36.338 | 254 | FAIL |
| 05_oblique | 91.961 | 98.959 | 0.378 | 169 | FAIL |
| 06_ground_west | 33.601 | 67.957 | 24.847 | 249 | FAIL |
| 07_far_north | 85.750 | 91.522 | 8.011 | 248 | FAIL |
| 08_back_inside | 55.775 | 72.818 | 25.569 | 237 | FAIL |
| 09_after_mesh | 0.000 | 0.247 | 99.126 | 245 | FAIL |
| 10_marker_ray | 39.973 | 54.506 | 39.288 | 236 | FAIL |
| 11_topdown | 26.292 | 67.191 | 8.990 | 223 | FAIL |
| 12_far_north_re | 85.751 | 91.530 | 7.979 | 243 | FAIL |

**GATE: RED — parity below 100%**
