# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.145 | 84.185 | 18.280 | 209 | FAIL |
| 01_aerial_west | 59.658 | 74.443 | 19.883 | 250 | FAIL |
| 02_aerial_south | 49.995 | 71.531 | 27.041 | 253 | FAIL |
| 03_aerial_north | 53.511 | 74.203 | 16.060 | 249 | FAIL |
| 04_aerial_east | 52.686 | 66.332 | 33.142 | 254 | FAIL |
| 05_oblique | 88.278 | 97.178 | 1.491 | 169 | FAIL |
| 06_ground_west | 27.664 | 61.413 | 27.582 | 249 | FAIL |
| 07_far_north | 85.292 | 90.273 | 8.272 | 249 | FAIL |
| 08_back_inside | 57.499 | 74.763 | 24.284 | 227 | FAIL |
| 09_after_mesh | 50.243 | 61.090 | 32.869 | 237 | FAIL |
| 10_marker_ray | 46.846 | 60.903 | 33.316 | 238 | FAIL |
| 11_topdown | 28.020 | 71.467 | 7.841 | 215 | FAIL |
| 12_far_north_re | 85.292 | 90.268 | 8.289 | 246 | FAIL |

**GATE: RED — parity below 100%**
