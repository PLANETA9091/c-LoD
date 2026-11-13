# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.213 | 84.017 | 18.409 | 209 | FAIL |
| 01_aerial_west | 60.162 | 75.544 | 19.363 | 248 | FAIL |
| 02_aerial_south | 51.191 | 73.435 | 21.301 | 253 | FAIL |
| 03_aerial_north | 54.215 | 75.358 | 15.463 | 253 | FAIL |
| 04_aerial_east | 51.597 | 64.962 | 31.108 | 254 | FAIL |
| 05_oblique | 88.714 | 96.774 | 2.134 | 169 | FAIL |
| 06_ground_west | 29.542 | 64.328 | 26.474 | 249 | FAIL |
| 07_far_north | 85.278 | 90.243 | 8.280 | 249 | FAIL |
| 08_back_inside | 57.494 | 74.712 | 24.335 | 238 | FAIL |
| 09_after_mesh | 46.603 | 56.729 | 35.683 | 237 | FAIL |
| 10_marker_ray | 39.498 | 54.339 | 39.536 | 238 | FAIL |
| 11_topdown | 27.256 | 69.434 | 8.210 | 222 | FAIL |
| 12_far_north_re | 85.278 | 90.240 | 8.283 | 246 | FAIL |

**GATE: RED — parity below 100%**
