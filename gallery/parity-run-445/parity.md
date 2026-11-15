# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.531 | 84.483 | 18.123 | 204 | FAIL |
| 01_aerial_west | 60.083 | 75.173 | 19.622 | 250 | FAIL |
| 02_aerial_south | 50.451 | 71.961 | 26.883 | 253 | FAIL |
| 03_aerial_north | 53.832 | 75.510 | 15.408 | 253 | FAIL |
| 04_aerial_east | 51.862 | 65.150 | 35.379 | 254 | FAIL |
| 05_oblique | 89.881 | 97.485 | 1.458 | 169 | FAIL |
| 06_ground_west | 29.870 | 64.285 | 25.764 | 249 | FAIL |
| 07_far_north | 85.278 | 90.245 | 8.271 | 249 | FAIL |
| 08_back_inside | 56.447 | 72.530 | 25.298 | 239 | FAIL |
| 09_after_mesh | 47.602 | 57.453 | 36.404 | 232 | FAIL |
| 10_marker_ray | 47.047 | 59.356 | 33.014 | 236 | FAIL |
| 11_topdown | 26.200 | 65.758 | 9.307 | 216 | FAIL |
| 12_far_north_re | 85.278 | 90.240 | 8.285 | 246 | FAIL |

**GATE: RED — parity below 100%**
