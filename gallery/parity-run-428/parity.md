# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.059 | 83.649 | 18.343 | 210 | FAIL |
| 01_aerial_west | 58.027 | 71.953 | 21.144 | 250 | FAIL |
| 02_aerial_south | 49.120 | 70.214 | 27.979 | 253 | FAIL |
| 03_aerial_north | 55.169 | 77.633 | 14.630 | 253 | FAIL |
| 04_aerial_east | 51.932 | 65.936 | 34.397 | 254 | FAIL |
| 05_oblique | 86.570 | 95.743 | 2.996 | 169 | FAIL |
| 06_ground_west | 34.099 | 68.354 | 24.627 | 247 | FAIL |
| 07_far_north | 85.467 | 90.261 | 8.097 | 248 | FAIL |
| 08_back_inside | 57.134 | 73.501 | 24.406 | 239 | FAIL |
| 09_after_mesh | 50.857 | 60.345 | 34.039 | 236 | FAIL |
| 10_marker_ray | 41.404 | 55.655 | 38.591 | 238 | FAIL |
| 11_topdown | 27.616 | 69.969 | 8.386 | 216 | FAIL |
| 12_far_north_re | 85.317 | 90.245 | 8.277 | 246 | FAIL |

**GATE: RED — parity below 100%**
