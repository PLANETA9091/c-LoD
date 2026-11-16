# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 77.324 | 82.518 | 20.944 | 209 | FAIL |
| 01_aerial_west | 60.383 | 74.354 | 19.170 | 250 | FAIL |
| 02_aerial_south | 48.541 | 71.234 | 25.229 | 253 | FAIL |
| 03_aerial_north | 50.045 | 70.749 | 20.410 | 254 | FAIL |
| 04_aerial_east | 52.982 | 68.436 | 26.718 | 254 | FAIL |
| 05_oblique | 75.880 | 85.363 | 18.473 | 222 | FAIL |
| 06_ground_west | 29.764 | 64.006 | 22.938 | 249 | FAIL |
| 07_far_north | 85.264 | 90.248 | 8.252 | 249 | FAIL |
| 08_back_inside | 49.544 | 66.294 | 31.554 | 227 | FAIL |
| 09_after_mesh | 39.366 | 53.188 | 40.392 | 237 | FAIL |
| 10_marker_ray | 51.503 | 69.062 | 27.074 | 250 | FAIL |
| 11_topdown | 27.132 | 74.200 | 6.582 | 216 | FAIL |
| 12_far_north_re | 85.278 | 90.243 | 8.280 | 246 | FAIL |

**GATE: RED — parity below 100%**
