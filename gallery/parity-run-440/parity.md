# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- ours pair sources: . = 13
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.230 | 84.164 | 18.196 | 210 | FAIL |
| 01_aerial_west | 62.128 | 78.951 | 18.393 | 249 | FAIL |
| 02_aerial_south | 50.899 | 73.498 | 20.892 | 253 | FAIL |
| 03_aerial_north | 53.810 | 75.405 | 15.945 | 253 | FAIL |
| 04_aerial_east | 51.916 | 65.384 | 29.694 | 254 | FAIL |
| 05_oblique | 91.968 | 98.966 | 0.376 | 169 | FAIL |
| 06_ground_west | 32.442 | 67.211 | 26.015 | 249 | FAIL |
| 07_far_north | 85.406 | 90.267 | 8.134 | 248 | FAIL |
| 08_back_inside | 57.210 | 74.418 | 24.363 | 237 | FAIL |
| 09_after_mesh | 47.410 | 57.191 | 35.111 | 233 | FAIL |
| 10_marker_ray | 40.851 | 55.233 | 37.969 | 236 | FAIL |
| 11_topdown | 26.484 | 67.420 | 8.528 | 217 | FAIL |
| 12_far_north_re | 85.407 | 90.270 | 8.113 | 245 | FAIL |

**GATE: RED — parity below 100%**
