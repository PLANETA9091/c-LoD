# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 77.312 | 80.372 | 18.391 | 222 | FAIL |
| 01_aerial_west | 46.659 | 57.990 | 34.625 | 249 | FAIL |
| 02_aerial_south | 42.087 | 59.247 | 40.090 | 253 | FAIL |
| 03_aerial_north | 52.781 | 70.206 | 17.336 | 250 | FAIL |
| 04_aerial_east | 47.912 | 57.122 | 37.283 | 254 | FAIL |
| 05_oblique | 85.549 | 90.997 | 8.931 | 209 | FAIL |
| 06_ground_west | 34.668 | 63.703 | 21.051 | 249 | FAIL |
| 07_far_north | 85.461 | 90.252 | 8.111 | 249 | FAIL |
| 08_back_inside | 60.069 | 76.692 | 21.403 | 237 | FAIL |
| 09_after_mesh | 53.868 | 64.895 | 24.710 | 233 | FAIL |
| 10_marker_ray | 48.894 | 63.889 | 33.924 | 240 | FAIL |
| 11_topdown | 31.489 | 82.874 | 4.892 | 212 | FAIL |
| 12_far_north_re | 85.461 | 90.251 | 8.123 | 246 | FAIL |

**GATE: RED — parity below 100%**
