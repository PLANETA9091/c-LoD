# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.316 | 84.394 | 18.125 | 210 | FAIL |
| 01_aerial_west | 60.225 | 74.821 | 19.608 | 250 | FAIL |
| 02_aerial_south | 51.022 | 73.262 | 20.860 | 253 | FAIL |
| 03_aerial_north | 53.427 | 74.088 | 16.228 | 253 | FAIL |
| 04_aerial_east | 51.775 | 65.077 | 29.439 | 254 | FAIL |
| 05_oblique | 90.056 | 98.189 | 1.041 | 169 | FAIL |
| 06_ground_west | 29.567 | 63.812 | 25.327 | 248 | FAIL |
| 07_far_north | 85.464 | 90.261 | 8.110 | 248 | FAIL |
| 08_back_inside | 56.059 | 72.140 | 25.019 | 239 | FAIL |
| 09_after_mesh | 49.802 | 59.929 | 34.195 | 236 | FAIL |
| 10_marker_ray | 41.350 | 55.847 | 38.292 | 238 | FAIL |
| 11_topdown | 27.701 | 70.343 | 8.267 | 216 | FAIL |
| 12_far_north_re | 85.317 | 90.243 | 8.284 | 246 | FAIL |

**GATE: RED — parity below 100%**
