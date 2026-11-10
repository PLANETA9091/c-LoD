# PARITY GATE — c-LoD vs Voxy (pixel-perfect, per vantage)

- voxy shots: 13, ours shots: 13, paired: 13, missing (voxy shot with no ours pair): 0
- gate: 100% pixel-exact per pair; the job exits 1 while anything is below

| vantage | exact % | near % (d<=8) | mean d | max d | verdict |
|---|---|---|---|---|---|
| 00_summit_rise | 79.385 | 81.785 | 15.712 | 220 | FAIL |
| 01_aerial_west | 53.290 | 60.284 | 26.141 | 249 | FAIL |
| 02_aerial_south | 58.079 | 71.100 | 18.439 | 253 | FAIL |
| 03_aerial_north | 49.656 | 63.015 | 21.031 | 253 | FAIL |
| 04_aerial_east | 64.628 | 72.591 | 16.397 | 254 | FAIL |
| 05_oblique | 84.482 | 87.953 | 8.183 | 192 | FAIL |
| 06_ground_west | 50.235 | 66.217 | 16.856 | 249 | FAIL |
| 07_far_north | 85.963 | 91.889 | 7.942 | 248 | FAIL |
| 08_back_inside | 59.444 | 75.866 | 22.766 | 237 | FAIL |
| 09_after_mesh | 51.472 | 62.579 | 26.764 | 227 | FAIL |
| 10_marker_ray | 49.752 | 64.008 | 33.999 | 236 | FAIL |
| 11_topdown | 32.257 | 86.003 | 4.208 | 212 | FAIL |
| 12_far_north_re | 86.054 | 91.887 | 7.924 | 245 | FAIL |

**GATE: RED — parity below 100%**
